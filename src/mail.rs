//use lettre::transport::smtp::Error;
use rocket::fairing::{self, AdHoc};
use rocket::tokio::{task, time};
use rocket::{Build, Rocket};
use serde;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Mutex;
use std::time::Duration;

use model::Contact;

#[derive(serde::Deserialize)]
pub(crate) struct MailContact {
    header: String,
    to: String,
}

static SINGLETON: Mutex<MailyTask> = Mutex::new(MailyTask::new());

type TaskHandle = rocket::tokio::task::JoinHandle<()>;
struct MailyTask {
    value: Option<TaskHandle>,
    keep_alive: bool,
}

impl MailyTask {
    pub const fn new() -> Self {
        MailyTask {
            value: None,
            keep_alive: true,
        }
    }

    async fn launch() {
        let mut thread: std::sync::MutexGuard<'_, MailyTask> = SINGLETON.lock().unwrap();

        let forever = task::spawn(async {
            let mut interval = time::interval(Duration::from_millis(10000));
            while SINGLETON.lock().unwrap().keep_alive {
                interval.tick().await;
                send_pending_emails().await;
            }
        });
        thread.value = match &thread.value {
            Some(_) => Some(forever),
            None => Some(forever),
        }
    }
}

//TODO: Unified Logging
async fn send_pending_emails() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite:db/sqlx/db.sqlite")
        .await;
    println!("Started Pool");
    match pool {
        Err(_) => {
            println!("pool is error");
            return;
        }
        Ok(conn) => {
            let conn = conn.acquire().await;
            println!("Pool conn is acquired");
            match conn {
                Err(_) => {
                    println!("Pool conn is error");
                    return;
                }
                Ok(mut cconn) => {
                    let get_conn = &mut cconn;
                    let contact = Contact::find_raising_contact(get_conn).await;
                    println!("Tried to find contact");
                    match contact {
                        Err(_) => {
                            println!("Error finding contact");
                            return;
                        }
                        Ok(None) => {
                            println!("No Contact Now");
                            return;
                        }
                        Ok(Some(contact)) => {
                            println!("What contact");
                            let mark_conn = &mut cconn;
                            //let con_text = contact.to_string();
                            println!("{}", contact.to_mail_body().await);
                            let r = contact.mark_contact_raised(mark_conn).await;
                            match r {
                                Ok(_) => {
                                    println!("Marked sent");
                                    return;
                                }
                                Err(e) => {
                                    println!("{} Marking Complete", e.0);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn start_maily(rocket: Rocket<Build>) -> fairing::Result {
    MailyTask::launch().await;
    Ok(rocket)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Email Stage", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("Start Email", start_maily))
    })
}

//use lettre::transport::smtp::Error;
use rocket::fairing::{self, AdHoc};
use rocket::tokio::{task, time};
use rocket::{Build, Rocket};
use std::sync::Mutex;
use std::time::Duration;

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
            None => Some(forever)
        }
    }

}

async fn send_pending_emails() {
    eprintln!("Doing something...");
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

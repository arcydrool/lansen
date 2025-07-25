use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use rocket::tokio::{self};
use serde;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;

use model::Contact;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = Path::new("lansen.toml");
    println!("exists {}", m.exists());

    let config = configure().await;
    if config.is_err() {
        panic!("Lansen Maily Failed to Configure {}", config.unwrap_err());
    }
    let config = config.unwrap();
    send_pending_emails(config).await
}

async fn configure() -> Result<Configuration, figment::Error> {
    let figment = Figment::new()
        .merge(Toml::file("lansen.toml"))
        .merge(Env::prefixed("LANSEN_"));
    figment.extract()
}

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub(crate) struct Configuration {
    header: String,
    to: String,
}

//TODO: Unified Logging
async fn send_pending_emails(config: Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite:model/db/sqlx/db.sqlite")
        .await;
    println!("Started Pool");
    match pool {
        Err(_) => {
            println!("pool is error");
            return Ok(());
        }
        Ok(conn) => {
            let conn = conn.acquire().await;
            println!("Pool conn is acquired");
            match conn {
                Err(_) => {
                    println!("Pool conn is error");
                    return Ok(());
                }
                Ok(mut cconn) => {
                    let get_conn = &mut cconn;
                    let contact = Contact::find_raising_contact(get_conn).await;
                    println!("Tried to find contact");
                    match contact {
                        Err(_) => {
                            println!("Error finding contact");
                            return Ok(());
                        }
                        Ok(None) => {
                            println!("No Contact Now");
                            return Ok(());
                        }
                        Ok(Some(contact)) => {
                            println!("What contact");
                            let mark_conn = &mut cconn;
                            //let con_text = contact.to_string();
                            println!("to: {}\n{}\n{}", config.to, config.header,contact.to_mail_body().await);
                            let r = contact.mark_contact_raised(mark_conn).await;
                            match r {
                                Ok(_) => {
                                    println!("Marked sent");
                                    return Ok(());
                                }
                                Err(e) => {
                                    println!("{} Marking Complete", e.0);
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use rocket::{futures::future, tokio::{self}};
use serde;
use sqlx::{pool::PoolConnection, sqlite::SqlitePoolOptions, Sqlite};
use std::path::Path;
use lettre::Message;

use model::Contact;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = Path::new("lansen.toml");
    dbg!("exists {}", m.exists());

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
    let pending_contact: Result<Option<Contact>, Box<dyn std::error::Error>> = first_pending(pending_contact).await;

                    
                    dbg!("Tried to find contact");
                    match pending_contact {
                        Err(_) => {
                            dbg!("Error finding contact");
                            return Ok(());
                        }
                        Ok(None) => {
                            dbg!("No Contact Now");
                            return Ok(());
                        }
                        Ok(Some(contact)) => {
                            dbg!("What contact");
                            let mark_conn = &mut cconn;
                            //let con_text = contact.to_string();
                            //dbg!("to: {}\n{}\n{}", config.to, config.header,contact.to_mail_body().await);
                            let sent = send_email(&config,&contact).await;
                            let marked = contact.mark_contact_raised(mark_conn).await;
                            //handle two errors at once. Powerful.
                            match (sent, marked) {
                                (Ok(_),Ok(_)) => {
                                    dbg!("Marked sent");
                                    return Ok(());
                                }
                                (Err(e),_) => {
                                    dbg!("{} Sending Email", & *e);
                                    return Err(e);
                                }
                                (_,Err(e)) => {
                                    dbg!("{} Marking Complete", &e.0);
                                    return Err(Box::new(e.0));
                                }
                            }
                        }
                    }
                }
            

async fn send_email(config: &Configuration,contact: &Contact) -> Result<(), Box<dyn std::error::Error>> {
    let message = Message::builder();
    Ok(())
}



async fn first_pending<T> (pending_fn: &dyn Future<Output = model::Result<Option<T>>) -> Result<Option<T>, Box<dyn std::error::Error>>{
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite:model/db/sqlx/db.sqlite")
        .await?;
    dbg!("Started Pool");
    let conn = pool.acquire().await?;
     
                    let get_conn = &mut conn;
         pending_fn.await
}

async fn pending_contact(get_conn: PoolConnection<Sqlite>) ->  model::Result<Option<Contact>> {
 Contact::find_raising_contact(get_conn).await
}
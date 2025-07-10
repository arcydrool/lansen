use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;

use rocket_db_pools::{sqlx, Connection};

use crate::model::{Contact, Db};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(super) struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    name: String,
    company: String,
    email: String,
    tel: String,
    interests: Vec<String>,
    additional: String,
}

#[catch(default)]
pub(super) fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[post("/", data = "<post>")]
pub(super) async fn create(
    db: Connection<Db>,
    post: Json<Contact>,
) -> Result<Created<Json<Contact>>> {
    // NOTE: sqlx#2543, sqlx#1648 mean we can't use the pithier `fetch_one()`.
    let result = Contact::create(db, post).await;
    match result {
        Ok(contact) => Ok(Created::new("/").body(Json(contact))),
        Err(e) => Err(e),
    }
}

use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};
use crate::model::{MoldPost, Db};

use rocket_db_pools::{sqlx, Connection, Database};

use futures::{future::TryFutureExt, stream::TryStreamExt};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(super) struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    title: String,
    text: String,
}

#[post("/", data = "<post>")]
pub(super) async fn create(
    mut db: Connection<Db>,
    mut post: Json<MoldPost>,
) -> Result<Created<Json<MoldPost>>> {
     return MoldPost::create(db, post).await
}

#[get("/")]
pub(super) async fn list(mut db: Connection<Db>) -> Result<Json<Vec<i64>>> {
    return MoldPost::list(db).await
}

#[get("/<id>")]
pub(super) async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<MoldPost>> {
    return MoldPost::read(db, id).await
}


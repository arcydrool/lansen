use rocket::response::status::Created;
use rocket::serde::{json::Json};
use crate::model::{MoldPost, Db};

use rocket_db_pools::{sqlx, Connection};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[post("/", data = "<post>")]
pub(super) async fn create(
     db: Connection<Db>,
     post: Json<MoldPost>,
) -> Result<Created<Json<MoldPost>>> {
     return MoldPost::create(db, post).await
}

#[get("/")]
pub(super) async fn list( db: Connection<Db>) -> Result<Json<Vec<i64>>> {
    return MoldPost::list(db).await
}

#[get("/<id>")]
pub(super) async fn read( db: Connection<Db>, id: i64) -> Option<Json<MoldPost>> {
    return MoldPost::read(db, id).await
}


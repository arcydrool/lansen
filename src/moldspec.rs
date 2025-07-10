use rocket::response::status::Created;
use rocket::serde::{json::Json};
use crate::model::{QuotePost, Db};

use rocket_db_pools::{sqlx, Connection};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[post("/", data = "<post>")]
pub(super) async fn create(
     db: Connection<Db>,
     post: Json<QuotePost>,
) -> Result<Created<Json<QuotePost>>> {
     return QuotePost::create(db, post).await
}

#[get("/")]
pub(super) async fn list( db: Connection<Db>) -> Result<Json<Vec<i64>>> {
    return QuotePost::list(db).await
}

#[get("/<id>")]
pub(super) async fn read( db: Connection<Db>, id: i64) -> Option<Json<QuotePost>> {
    return QuotePost::read(db, id).await
}


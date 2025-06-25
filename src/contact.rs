use rocket::fairing::{self, AdHoc};
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket::{futures, Build, Rocket};

use rocket_db_pools::{sqlx, Connection, Database};

use futures::stream::TryStreamExt;

#[derive(Database)]
#[database("sqlx")]
pub(super) struct Db(sqlx::SqlitePool);

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
    mut db: Connection<Db>,
    mut post: Json<Post>,
) -> Result<Created<Json<Post>>> {
    // NOTE: sqlx#2543, sqlx#1648 mean we can't use the pithier `fetch_one()`.
    let interests: String = post.interests.join(",");
    let results = sqlx::query!(
        "INSERT INTO contacts (id, name, company, email, tel, interests, additional) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id",
        post.id,
        post.name,
        post.company,
        post.email,
        post.tel,
        interests,
        post.additional
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    post.id = Some(results.first().expect("returning results").id);
    Ok(Created::new("/").body(post))
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}

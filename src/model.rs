use futures::{future::TryFutureExt, stream::TryStreamExt};
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};
use rocket_db_pools::sqlx::{pool::PoolConnection, Sqlite};
use rocket_db_pools::{sqlx, Database};
use sqlx::Acquire;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct QuotePost {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    title: String,
    text: String,
    raising: bool,
    raised: bool,
}

impl QuotePost {
    pub(crate) async fn create(
        mut db: PoolConnection<Sqlite>,
        mut post: Json<QuotePost>,
    ) -> Result<Created<Json<QuotePost>>> {
        // NOTE: sqlx#2543, sqlx#1648 mean we can't use the pithier `fetch_one()`.
        let results = sqlx::query!(
            "INSERT INTO quote (title, text) VALUES (?, ?) RETURNING id",
            post.title,
            post.text
        )
        .fetch(db.acquire().await.unwrap())
        .try_collect::<Vec<_>>()
        .await?;

        post.id = Some(results.first().expect("returning results").id);
        return Ok(Created::new("/").body(post));
    }
    pub(crate) async fn list(mut db: PoolConnection<Sqlite>) -> Result<Json<Vec<i64>>> {
        let ids = sqlx::query!("SELECT id FROM quote where raising = 0 and raised = 0")
            .fetch(db.acquire().await.unwrap())
            .map_ok(|record| record.id)
            .try_collect::<Vec<_>>()
            .await?;

        Ok(Json(ids))
    }

    pub(crate) async fn read(mut db: PoolConnection<Sqlite>, id: i64) -> Option<Json<QuotePost>> {
        sqlx::query!(
            "SELECT id, title, text, raising, raised FROM quote WHERE id = ?",
            id
        )
        .fetch_one(db.acquire().await.unwrap())
        .map_ok(|r| {
            Json(QuotePost {
                id: Some(r.id),
                title: r.title,
                text: r.text,
                raising: r.raising,
                raised: r.raised,
            })
        })
        .await
        .ok()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Contact {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    name: String,
    company: String,
    email: String,
    tel: String,
    interests: Vec<String>,
    additional: String,
    raising: bool,
    raised: bool,
}

impl std::fmt::Display for Contact{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

impl Contact {
    pub(crate) async fn create(mut db: PoolConnection<Sqlite>, post: Json<Contact>) -> Result<Contact> {
        // NOTE: sqlx#2543, sqlx#1648 mean we can't use the pithier `fetch_one()`.
        let vinterests = post.interests.clone();
        let interests: String = post.interests.join(",");
        let query = sqlx::query!(
            "INSERT INTO contact (id, name, company, email, tel, interests, additional) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id", 
            post.id, post.name, post.company, post.email, post.tel, interests, post.additional);
        let results = query.fetch(db.acquire().await.unwrap()).try_collect::<Vec<_>>().await?;
        let results = results.first();

        match results {
            Some(rec) => Ok(Contact {
                id: Some(rec.id),
                name: post.name.to_string(),
                company: post.company.to_string(),
                email: post.email.to_string(),
                tel: post.tel.to_string(),
                interests: vinterests,
                additional: post.additional.to_string(),
                raising: false,
                raised: false,
            }),
            None => Err(rocket::response::Debug(sqlx::Error::RowNotFound)),
        }
    }

    
    pub(crate) async fn find(mut db: PoolConnection<Sqlite>, id: i64) -> Result<Contact> {
        let query = sqlx::query!(
            "
          select id, name, company, email, tel, interests, additional, raising, raised 
            from contact where id = ?",
            id
        );
        let record = query.fetch_one(db.acquire().await.unwrap()).await;
        match record {
            Ok(row) => {
                let interests = row.interests.split(",").map(|c| c.to_string()).collect();
                Ok(Contact {
                    id: Some(row.id),
                    name: row.name,
                    company: row.company,
                    email: row.email,
                    tel: row.tel,
                    interests: interests,
                    additional: row.additional,
                    raising: row.raising,
                    raised: row.raised,
                })
            }
            Err(e) => Err(rocket::response::Debug(e)),
        }
    }
    pub(crate) async fn find_raising_contact(mut db: PoolConnection<Sqlite>) -> Result<Option<Contact>> {
        let query = sqlx::query!(
            "
            update contact  
              set raising = true where raising = 0 and raised = 0 
              and not exists ( 
                select 1 from contact cc 
                where cc.id < id and raising = 0 and raised = 0 ) 
            returning id "
        );
        let result = query.fetch_optional(db.acquire().await.unwrap()).await;

        match result {
            Err(e) => Err(rocket::response::Debug(e)),
            Ok(some_rec) => match some_rec {
                None => Ok(None),
                Some(rec) => {
                    let contact = Contact::find(db, rec.id.unwrap_or_default()).await;
                    match contact {
                        Err(e) => Err(e),
                        Ok(contact) => Ok(Some(contact)),
                    }
                }
            },
        }
    }
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

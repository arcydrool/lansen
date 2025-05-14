#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

mod sqlx;

use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/sqlx", sqlx::list()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(sqlx::stage())
}

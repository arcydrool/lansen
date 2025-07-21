#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod model;
mod contact;
mod mail;
mod moldspec;

use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/index.html", moldspec::list()))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/moldspec",
            routes![
                moldspec::create,
                moldspec::list,
                moldspec::read
            ],
        )
        .register("/c", catchers![contact::default])
        .mount("/c", routes![contact::create])
        .mount(
            "/",
            FileServer::new(relative!("static"), rocket::fs::Options::None),
        ).attach(model::stage())
        .attach(mail::stage())
        .launch.await
}

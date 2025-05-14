#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod moldspec;

use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/moldspec", moldspec::list()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/",
            FileServer::new(relative!("static"), rocket::fs::Options::None),
        )
        .attach(moldspec::stage())
}

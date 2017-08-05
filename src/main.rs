#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate dotenv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod db;
mod collectors;

use rocket::Request;

#[error(400)]
fn error(_: &Request) -> &'static str {
    ""
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            collectors::wifi,
            collectors::sound,
            collectors::light,
            collectors::gps
        ])
        .catch(errors![error])
        .manage(db::init_pool())
        .launch();
}

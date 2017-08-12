#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate dotenv;

#[allow(unused_imports)]
#[macro_use]
extern crate env_logger;

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

extern crate egg_mode;
extern crate geo;

use std::env;

mod db;
mod endpoints;
mod cloud_readings;
mod server;
mod map;

fn main() {
    println!("Connecting to database...");
    let db_pool = db::init_pool();

    println!("Fetching parameters...");
    let parameters = {
        let conn = db_pool.get().expect("Failed to acquire a pooled database");
        db::models::Parameter::fetch(&*conn).expect("Failed to fetch parameters")
    };

    let args = env::args();
    match args.skip(1).next().as_ref().map(|x| x.as_ref()) {
        Some("twitter") => cloud_readings::twitter::subscribe_to_twitter(),
        Some("geojson") => map::geojson(&parameters),
        Some(arg) => println!("Unrecognized parameter: {}", arg),
        None => server::run()
    }
}

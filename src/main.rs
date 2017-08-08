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

extern crate futures;
extern crate tokio_core;
extern crate twitter_stream;

use std::env;

mod db;
mod collectors;
mod cloud_readings;
mod server;


fn main() {
    let args = env::args();

    match args.skip(1).next().as_ref().map(|x| x.as_ref()) {
        Some("twitter") => cloud_readings::subscribe_to_twitter(),
        Some(arg) => println!("Unrecognized parameter: {}", arg),
        None => server::run()
    }
}

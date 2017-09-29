#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_jwt;
extern crate chrono;
extern crate geo;
extern crate soundlines_core;
extern crate soundlines_simlib;

#[allow(unused_imports)]
#[macro_use]
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod db_guard;
mod endpoints;
mod server;
mod rocket_extensions;
mod user;

fn main() {
    server::run()
}

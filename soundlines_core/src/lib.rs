#![feature(custom_attribute)]
extern crate chrono;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate geo;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
pub extern crate postgis;

extern crate serde;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate byteorder;

extern crate rand;

pub mod db;
pub mod map;

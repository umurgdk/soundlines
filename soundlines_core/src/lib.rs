#![feature(custom_attribute)]

extern crate chrono;
extern crate dotenv;
extern crate geo;
pub extern crate r2d2;
pub extern crate r2d2_postgres;
extern crate postgres;
extern crate byteorder;
extern crate rand;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub extern crate postgis;

pub mod db;

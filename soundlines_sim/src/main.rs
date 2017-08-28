extern crate cgmath;
extern crate mint;
extern crate geo;
extern crate three;
extern crate rand;

#[macro_use]
extern crate ndarray;
extern crate noise;
extern crate chrono;
extern crate rayon;

extern crate soundlines_core;

mod helpers;
mod constants;
mod context;
mod simulation;

mod sim_geo;
mod sim_entity;
mod sim_dna;
mod sim_seed;

use std::env;
use soundlines_core::db;

fn main() {
    let program = env::args().skip(1).next().unwrap_or("simulation".to_string());
    let connection_pool = db::init_pool();

    match program.as_ref() {
        "simulation" => simulation::run(connection_pool),
        "visualization" => println!("To be done!"), // visualization::run(),
        cmd => println!("Unsupported comand: {}", cmd)
    }
}

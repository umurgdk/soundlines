#![feature(custom_attribute)]
extern crate cgmath;
extern crate geo;
extern crate rand;
extern crate chrono;
extern crate chrono_tz;
extern crate rayon;
extern crate cron;
extern crate job_scheduler;
extern crate noise;

#[macro_use] extern crate serde_json;
#[macro_use] extern crate clap;

extern crate soundlines_core;

mod helpers;
mod constants;
mod context;

mod simulation;
mod snapshot;
mod randomizer;

mod genworld;
mod gencells;

mod sim_geo;
mod sim_entity;
mod sim_dna;
mod sim_seed;

use std::process;
use std::error::Error;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use clap::AppSettings;

use soundlines_core::db;

fn main() {
    let app = App::new("Soundlines Simulation")
        .version("0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)

        .subcommand(SubCommand::with_name("gencells")
                    .about("Creates initial cells")
                    .arg(Arg::with_name("cell_size")
                         .help("Sets the cell size")
                         .takes_value(true)
                         .default_value("50.0")
                         .require_equals(true)))

        .subcommand(SubCommand::with_name("genworld")
                    .about("Randomly generates seeds")

                    .arg(Arg::with_name("clear")
                         .short("c")
                         .help("deletes previously created entities and seeds")))

        .subcommand(SubCommand::with_name("randomize")
                    .about("Randomize entities settings (prefabs)"))

        .subcommand(SubCommand::with_name("snapshot")
                    .about("Starts taking snapshots of the world (entities, seeds, cells) in intervals")

                    .arg(Arg::with_name("directory")
                         .long("directory")
                         .help("Output directory where the snapshot json files going to be written")
                         .require_equals(true)
                         .default_value("snapshots"))

                    .arg(Arg::with_name("interval")
                         .long("interval")
                         .help("Any integer in units of minutes")
                         .require_equals(true)
                         .default_value("60")))

        .subcommand(SubCommand::with_name("simulate")
                    .about("Starts the simulation")

                    .arg(Arg::with_name("time_scale")
                         .long("time_scale")
                         .help("Any integer > 0 to determine how fast the simulation going to run")
                         .require_equals(true)
                         .default_value("1.0"))

                    .arg(Arg::with_name("seed_max_age")
                         .long("seed_max_age")
                         .help("Any float > 0 to determine how long a seed can survive without being bloomed")
                         .require_equals(true)
                         .default_value("250.0")));


    let matches = app.get_matches();
    let connection_pool = db::init_pool();

    let mut context = context::SimContext::default();

    let res: Result<_, _> = match matches.subcommand() {
        ("simulate", Some(options)) => {
            context.time_scale = value_t_or_exit!(options.value_of("time_scale"), f32).floor();
            context.seed_max_age = value_t_or_exit!(options.value_of("seed_max_age"), f32).floor();
            simulation::run(connection_pool, context)
        },

        ("randomize", _) =>
            randomizer::run(connection_pool),

        ("snapshot", Some(options)) =>
            snapshot::run(value_t_or_exit!(options.value_of("interval"), u32),
                          value_t_or_exit!(options.value_of("directory"), String)),

        ("genworld", Some(options)) => 
            genworld::run(connection_pool,  options.is_present("clear")),

        ("gencells", Some(options)) =>
            gencells::run(value_t_or_exit!(options.value_of("cell_size"), f64)),

        _ => unreachable!()
    };
                    
    if let Err(err) = res {
        println!("ERROR: {} :: {}", err.description(), err.cause().map(Error::description).unwrap_or(""));
        process::exit(1);
    }
}

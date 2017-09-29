use std::thread;
use std::error::Error;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::process;

use serde_json;

use cron::Schedule;
use job_scheduler::Job;
use job_scheduler::JobScheduler;

use chrono::prelude::*;
use chrono_tz::Japan;

use soundlines_core::db::models::*;
use soundlines_core::db::extensions::*;
use soundlines_core::db::init_connection;

pub fn run(interval: u32, snapshots_dir: String) -> Result<(), Box<Error>> {
    let mut sched = JobScheduler::new();

    // Every hour
    let schedule: Schedule = format!("0 0/{} * * * * *", interval).parse()?;

    println!("Upcoming times to run snapshotter");
    for time in schedule.upcoming(Japan).take(3) {
        println!("* {}", time);
    }

    println!();

    sched.add(Job::new(schedule, || {
        if let Err(err) = snapshot(&snapshots_dir) {
            on_error(&err);
            process::exit(1);
        }
    }));

    // Run forever
    loop {
        sched.tick();
        thread::sleep(Duration::from_millis(500));
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn snapshot(output_directory: &str) -> Result<(), Box<Error>> {
    let now = Utc::now().with_timezone(&Japan);
    println!("Taking snapshot for {}", now);

    let conn = init_connection();

    // Taking snapshot of entities
    let (entities, seeds, cells, weather, users) = (
        conn.all::<Entity>()?.into_iter().map(|e| e.into_json()).collect::<Vec<_>>(),
        conn.all::<Seed>()?.into_iter().map(|s| s.into_json()).collect::<Vec<_>>(),
        conn.all::<Cell>()?.into_iter().map(|c| c.into_json()).collect::<Vec<_>>(),
        conn.first::<Weather>()?,
        User::get_all_locations(&conn, None)?
    );

    let json = json!({
        "entities": entities,
        "seeds": seeds,
        "cells": cells,
        "users": users,
        "weather": weather
    });

    let json_output = serde_json::to_string(&json)?;

    let formatted_date = now.format("%Y_%m_%d_%H_%M_%S");
    let file_path = format!("{}/{}.json", output_directory, formatted_date);
    let mut file = File::create(&file_path)?;
    file.write_all(json_output.as_bytes())?;

    println!("Snapshot created ");

    Ok(())
}

fn on_error(err: &Box<Error>) {
    eprintln!("ERROR :: {} :: {}", err.description(), if let Some(err) = err.cause() { err.description() } else { "" } );
    process::exit(1);
}


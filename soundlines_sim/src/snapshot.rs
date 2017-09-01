use std::thread;
use std::sync::Arc;
use std::error::Error;
use std::time::Instant;
use std::time::Duration;

use job_scheduler::Job;
use job_scheduler::JobScheduler;

use chrono::prelude::*;
use chrono_tz::Japan;
use cron::Schedule;

use soundlines_core::db::models::*;
use soundlines_core::db::extensions::*;
use soundlines_core::db::init_pool;
use soundlines_core::db::PooledConnection;

pub fn run(interval: u32) -> Result<(), Box<Error>> {
    let mut sched = JobScheduler::new();

    // Every hour
    let schedule: Schedule = format!("0 0/{} * * * * *", interval).parse()?;

    println!("Upcoming times to run snapshotter");
    for time in schedule.upcoming(Japan).take(3) {
        println!("* {}", time);
    }

    println!("");

    sched.add(Job::new(schedule, snapshot));

    // Run forever
    loop {
        sched.tick();
        thread::sleep(Duration::from_millis(500));
    }

    unreachable!()
}

fn snapshot() {
    let now = Utc::now().with_timezone(&Japan);
    println!("Taking snapshot for {}", now);

    let pool = init_pool();

    // Taking snapshot of entities
    let conn = pool.get().expect("Failed to get a pooled connection");
    let entities_thread = thread::spawn(move || snapshot_entities(conn));
}

fn snapshot_entities(conn: PooledConnection) {
    println!("Snapshotting entities...");
}

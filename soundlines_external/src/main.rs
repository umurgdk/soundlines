#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
extern crate log;
extern crate env_logger;
extern crate egg_mode;
extern crate cron;
extern crate job_scheduler;
extern crate chrono;
extern crate chrono_tz;
extern crate serde_json;
extern crate soundlines_core;
extern crate forecast_io;

use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

use chrono::prelude::*;
use chrono_tz::Japan;

use cron::Schedule;
use job_scheduler::Job;
use job_scheduler::JobScheduler;

use soundlines_core::db::init_pool;

mod config;
mod twitter;
mod weather;
mod errors;

use self::errors::*;

fn run() -> errors::Result<()> {
	env_logger::init();

    let pool = init_pool();
    let mut job_scheduler = JobScheduler::new();

    let config = match config::from_json_file("external.json") {
        Ok(config) => config,
        Err(errors::Error(errors::ErrorKind::Io(_), _)) => {
            println!("There is no configuration file. Creating external.json...");
            config::create("external.json")?
        },
        Err(err) => return Err(err)
    };

    let tweet_minutes: Schedule = format!("0 0/{} * * * * *", config.minutes).parse()
        .chain_err(|| "Failed to parse cron string!")?;

	let weather_minutes: Schedule = format!("0 0/{} * * * * *", config.minutes).parse()
		.chain_err(|| "Failed to parse cron string!")?;

    println!("Now: {}", Utc::now().with_timezone(&Japan));
    println!("Last tweet id: {}", config.last_tweet_id.unwrap_or(0));
    println!("Upcoming times to fetch external data...");
    for time in tweet_minutes.upcoming(Japan).take(3) {
        println!("* {}", time);
    }

    let config = Arc::new(Mutex::new(config));

    let conn = pool.get()?;
    let mut twitter_runner = twitter::TweetRunner::new(conn, config.clone());

    job_scheduler.add(Job::new(tweet_minutes, move || {
        if let Err(err) = twitter_runner.run() {
            handle_error(&err);
        }
    }));

	let conn = pool.get()?;
	let mut weather_runner = weather::WeatherRunner::new(conn);

	weather_runner.run()?;

	job_scheduler.add(Job::new(weather_minutes, move || {
		if let Err(err) = weather_runner.run() {
			handle_error(&err);
		}
	}));

    let half_minute = Duration::from_secs(30);
    loop {
        job_scheduler.tick();
        thread::sleep(half_minute);
    }
}

quick_main!(run);

fn handle_error(e: &self::errors::ErrorKind) {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "{}", e).expect(errmsg);
    ::std::process::exit(1);
}

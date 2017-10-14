#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
extern crate serde_json;
extern crate soundlines_core;

use clap::App;
use clap::Arg;
use clap::SubCommand;

use soundlines_core::db;
use soundlines_core::errors;
use soundlines_core::models;
use soundlines_core::generators;

quick_main!(run);

fn run() -> errors::Result<()> {
	let matches = App::new("soundlines_cli")
		.version("0.1")
		.author("Umur Gedik <umurgdk@gmail.com>")
		.about("Management application for soundlines project")
		.subcommand(
			SubCommand::with_name("generate")
				.about("Generator commands")
				.arg(
					Arg::with_name("json-only")
						.help("Instead of generating things in the database, it will simply return a json")
						.long("json-only")
						.global(true))
				.subcommand(
					SubCommand::with_name("cells")
						.about("Generates cell's for the builtin region polygon"))
				.subcommand(
					SubCommand::with_name("seeds")
						.about("Generate seeds in the world")
						.arg(
							Arg::with_name("count")
								.short("c")
								.takes_value(true)
								.help("Number of seeds to generate"))))
		.get_matches();

	match matches.subcommand() {
		("generate", Some(g_matches)) => {
			match g_matches.subcommand() {
				("cells", Some(c_matches)) => generate_cells(c_matches.is_present("json-only")),
				("seeds", Some(s_matches)) => generate_seeds(s_matches.is_present("json-only"), value_t_or_exit!(s_matches.value_of("count"), u32)),
				_ => print_usage_and_exit(&matches)
			}
		},

		_ => print_usage_and_exit(&matches)
	}
}

fn print_usage_and_exit(matches: &clap::ArgMatches) -> errors::Result<()> {
	println!("{}", matches.usage());
	::std::process::exit(1);
	Ok(())
}

fn generate_cells(json_only: bool) -> errors::Result<()> {
	let cells = generators::cells::generate();

	if json_only {
		let json = serde_json::to_string_pretty(&cells)?;
		println!("{}", json);
		return Ok(());
	}

	let conn = db::connect();
	let cells = generators::cells::generate();
	db::cells::insert_batch(&conn, &cells)?;

	Ok(())
}

fn generate_seeds(json_only: bool, count: u32) -> errors::Result<()> {
	let conn = db::connect();
	let cells = db::cells::all_vec(&conn)?;
	let settings = db::plant_setting::all_vec(&conn)?;
	let seeds = generators::seeds::generate_batch(&cells, &settings, count);

	if json_only {
		let json = serde_json::to_string_pretty(&seeds)?;
		println!("{}", json);
		return Ok(());
	}

	db::seeds::batch_insert(&conn, &seeds)?;

	Ok(())
}
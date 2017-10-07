use std::env;
use dotenv::dotenv;
use postgres::TlsMode;

pub use postgres::Connection;

pub mod cells;
pub mod entities;
pub mod seeds;
pub mod weather;

/// Connects to database with `env::var(DATABASE_URL)`
///
/// ### Panics
///
/// On failed database connection attempt this function panics
pub fn connect() -> Connection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	Connection::connect(database_url, TlsMode::None).expect("Connection attempt to postgres db failed")
}

/// Start listening for notifications in given channel name
pub fn listen(conn: &Connection, channel: &str) -> ::errors::Result<()> {
	conn.batch_execute(&format!("listen {}", channel)).map_err(::errors::Error::from)
}
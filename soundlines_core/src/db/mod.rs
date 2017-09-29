use std::env;

use r2d2;
use r2d2_postgres::PostgresConnectionManager as ConnectionManager;
use r2d2_postgres::TlsMode;
use postgres;
use postgres::TlsMode as PTlsMode;
use dotenv::dotenv;

pub mod models;
pub mod extensions;

pub use self::extensions::*;

pub type Error = postgres::Error;
pub type Result<T> = postgres::Result<T>;
pub type Connection = postgres::Connection;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub fn init_connection() -> Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::connect(database_url, PTlsMode::None).expect("failed to connect database")
}

pub fn init_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::new(database_url, TlsMode::None).expect("failed to connect database");

    r2d2::Pool::new(config, manager).expect("Failed to create database pool")
}

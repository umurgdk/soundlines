use std::env;

use r2d2;
use r2d2_postgres::PostgresConnectionManager as ConnectionManager;
use r2d2_postgres::TlsMode;
use postgres;
use dotenv::dotenv;

pub mod models;
pub mod guard;
pub mod helpers;
pub mod extensions;

pub use self::helpers::*;
pub use self::guard::DbConn;
pub use self::extensions::*;

pub type Result<T> = postgres::Result<T>;
pub type Connection = postgres::Connection;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub fn init_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::new(database_url, TlsMode::None).expect("failed to connect database");

    r2d2::Pool::new(config, manager).expect("Failed to create database pool")
}

use std::env;

use diesel::pg::PgConnection;

use r2d2;
use r2d2_diesel::ConnectionManager;

use dotenv::dotenv;

pub mod models;
pub mod schema;
pub mod guard;
pub mod helpers;

pub use self::helpers::*;
pub use self::guard::DbConn;

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::new(config, manager).expect("Failed to connect database")
}

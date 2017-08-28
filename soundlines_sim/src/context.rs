use soundlines_core::db::PooledConnection;
use soundlines_core::db::extensions::*;

pub struct SimContext {
    pub db_conn: PooledConnection
}

impl SimContext {
    pub fn new(db_conn: PooledConnection) -> Self {
        Self { db_conn }
    }
}

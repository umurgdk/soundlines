use soundlines_core::db::PooledConnection;
use soundlines_core::db::extensions::*;

#[derive(Debug)]
pub struct SimContext {
    pub time_scale: f32,
    pub max_seed_age: f32
}

impl Default for SimContext {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            max_seed_age: 250.0
        }
    }
}

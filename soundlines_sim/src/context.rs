#[derive(Debug)]
pub struct SimContext {
    pub time_scale: f32,
    pub seed_max_age: f32
}

impl Default for SimContext {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            seed_max_age: 250.0
        }
    }
}

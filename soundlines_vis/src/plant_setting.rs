use three::Geometry;

#[derive(Debug, Clone)]
pub struct PlantSetting {
    pub growth_limit: f32,
    pub life_expectancy: f32,
    pub wifi_sensitivity: f32,
    pub light_sensitivity: f32,
    pub sound_sensitivity: f32,
    pub neighbor_tolerance: f32,
    pub birth_proba: f32,
    pub bloom_proba: f32,    //for seed
    pub mating_freq: f32,
	pub geometry: Geometry
}

impl PlantSetting {
    pub fn setting1() -> PlantSetting {
        PlantSetting {
            growth_limit: 300.0,
            life_expectancy: 250.0,
            wifi_sensitivity: 0.0,
            light_sensitivity: 0.0,
            sound_sensitivity: 0.0,
            neighbor_tolerance: 2.0,
            birth_proba: 1.0,
            bloom_proba: 1.0,    //for seed
            mating_freq: 30.0,
	        geometry: Geometry::new_box(0.2, 10.0, 0.2)
        }
    }
}

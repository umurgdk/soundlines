use chrono::prelude::*;

pub type Point = [f64; 2];
pub type Polygon = Vec<[f64; 2]>;

pub fn default_user_id() -> i32 { 0 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
	#[serde(default)]
	pub id: i32,
	#[serde(skip_serializing)]
	pub point: Point,
	pub prefab: String,
	pub cell_id: i32,
	#[serde(skip_serializing)]
	pub setting: PlantSetting,
	pub dna: Dna,
	pub fitness: f32,
	pub life_expectancy: f32,
	pub nickname: String,
	pub age: f32,
	pub size: f32,
	pub start_mating_at: f32,
	pub last_seed_at: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDraft {
	pub point: Point,
	pub setting: PlantSetting,
	pub dna: Dna
}

pub fn make_entities_from_drafts(drafts: Vec<EntityDraft>, cell_ids: &[Option<i32>]) -> Vec<Entity> {
	use rand;

	drafts.into_iter().enumerate().filter_map(|(i, EntityDraft { point, setting, dna })| {
		if cell_ids[i].is_none() {
			return None;
		}

		let id = -1;
		let prefab = setting.prefab.clone();
		let cell_id = cell_ids[i].unwrap();
		let fitness = dna.fitness;
		let life_expectancy = dna.life_expectancy;
		let size = dna.size;
		let nickname = format!("entity-{}", rand::random::<u32>());

		Some(Entity {
			id, point, prefab, cell_id, setting, dna,
			fitness, size, life_expectancy, nickname,
			age: 0.0, start_mating_at: 0.0, last_seed_at: 0.0,
		})
	}).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
	pub id: i32,
	pub cell_id: i32,
	pub dna: Dna,
	#[serde(skip_serializing)]
	pub setting: PlantSetting,
	#[serde(skip_serializing)]
	pub point: Point,
	pub created_at: DateTime<Utc>,
	pub age: f32,
	pub prefab: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedDraft {
	pub dna: Dna,
	pub setting: PlantSetting,
	pub point: Point
}

pub fn make_seeds_from_drafts(drafts: Vec<SeedDraft>, cell_ids: &[Option<i32>]) -> Vec<Seed> {
	drafts.into_iter().enumerate().filter_map(|(i, SeedDraft { point, setting, dna })| {
		if cell_ids[i].is_none() {
			return None;
		}

		let prefab = setting.prefab.clone();
		let cell_id = cell_ids[i].unwrap();

		Some(Seed { id: -1, point, prefab, cell_id, setting, dna, created_at: Utc::now(), age: 0.0 })
	}).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dna {
	pub size: f32,
	pub fitness: f32,
	pub life_expectancy: f32,
	pub growth_rate: f32,
	pub aging_rate: f32,
	pub mutation_rate: f32,
	pub stress_rate: f32,
	pub healthy_rate: f32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
	#[serde(default)]
	pub id: i32,
	pub geom: Polygon,

	pub wifi: f32,
	pub wifi_total: f32,
	pub wifi_count: f32,

	pub light: f32,
	pub light_total: f32,
	pub light_count: f32,

	pub sound: f32,
	pub sound_total: f32,
	pub sound_count: f32,

	pub sns: i32,
	pub visit: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpsReading {
	#[serde(default)]
	pub id: i32,
	pub user_id: i32,
	pub created_at: DateTime<Utc>,
	#[serde(skip)]
	pub point: Point
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LightReading {
	#[serde(default)]
	pub id: i32,
	#[serde(default="default_user_id")]
	pub user_id: i32,
	#[serde(default = "Utc::now")]
	pub created_at: DateTime<Utc>,
	pub level: f32,
	#[serde(skip)]
	pub point: Point
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundReading {
	#[serde(default)]
	pub id: i32,
	#[serde(default="default_user_id")]
	pub user_id: i32,
	#[serde(default = "Utc::now")]
	pub created_at: DateTime<Utc>,
	pub level: f32,
	#[serde(skip)]
	pub point: Point
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WifiReading {
	pub id: Option<i32>,
	pub user_id: i32,
	pub created_at: DateTime<Utc>,
	pub ssid: String,
	pub level: f32,
	pub frequency: f32,
	pub point: Point
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
	#[serde(skip)]
	pub id: i32,
	pub temperature: f64,
	pub precip: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantSetting {
	#[serde(default)]
	pub id: i32,
	pub name: String,
	pub prefab: String,
	pub growth_limit: f32,
	pub life_expectancy: f32,
	pub wifi_sensitivity: f32,
	pub light_sensitivity: f32,
	pub sound_sensitivity: f32,
	pub neighbor_tolerance: f32,
	pub birth_proba: f32,
	pub bloom_proba: f32,
	pub mating_freq: f32,
	pub mating_duration: f32,
	pub fruit_duration: f32,
	pub mating_distance: f32,
	pub crowd_distance: f32
}
use soundlines_core::db::models::Entity as DbEntity;
use soundlines_core::db::models::Dna as DbDna;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
	pub id: i32,
	pub point: Point,
	pub prefab: String,
	pub cell_id: i32,
	pub setting_id: i32,
	pub dna: Dna,
	pub fitness: f32,
	pub life_expectancy: f32,
	pub nickname: String,
	pub age: f32,
	pub size: f32,
	pub start_mating_at: f32,
	pub last_seed_at: f32
}

pub fn entity_from_db(entity: DbEntity, dnas: &HashMap<i32, Dna>) -> Entity {
	let DbEntity { id, point, prefab, cell_id, setting_id, dna_id, fitness, life_expectancy, nickname, age, size, start_mating_at, last_seed_at} = entity;
	Entity { id, point, prefab, cell_id, setting_id, dna: dnas[dna_id].clone(), fitness, life_expectancy, nickname, age, size, start_mating_at, last_seed_at }
}

#[derive(Serialize, Deserialize, Clone)]
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

pub fn dna_from_db(dna: DbDna) -> Dna {
	let DbDna { id: _id, size, fitness, life_expectancy, growth_rate, aging_rate, mutation_rate, stress_rate, healthy_rate } = dna;
	Dna { size, fitness, life_expectancy, growth_rate, aging_rate, mutation_rate, stress_rate, healthy_rate }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Seed {

}
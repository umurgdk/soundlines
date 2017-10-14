use chrono::prelude::*;

use helpers;
use models::Seed;
use models::Cell;
use models::PlantSetting;
use geom;

pub fn generate(cell: &Cell, setting: &PlantSetting) -> Seed {
	let point = geom::cell::random_inner_point(&cell.geom);
	let dna = ::generators::dnas::generate(&setting);

	Seed {
		id: -1,
		dna,
		age: 0.0,
		cell_id: cell.id,
		created_at: Utc::now(),
		point,
		prefab: setting.prefab.clone(),
		setting: setting.clone()
	}
}

pub fn generate_batch(cells: &[Cell], settings: &[PlantSetting], count: u32) -> Vec<Seed> {
	(0..count).map(|_| {
		let cell = helpers::random_choice(cells);
		let setting = helpers::random_choice(settings);
		generate(cell, setting)
	}).collect::<Vec<_>>()
}
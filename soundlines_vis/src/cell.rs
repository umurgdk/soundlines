use cgmath::Deg;
use cgmath::Vector3;
use cgmath::Rotation3;
use cgmath::Quaternion;
use cgmath::prelude::*;

use three::Mesh;
use three::Group;
use three::Factory;
use three::Material;
use three::Geometry;

use std::rc::Weak;

use dna::Dna;
use helpers::*;
use seed::Seed;
use constants::*;
use plant::Plant;
use plant_setting::PlantSetting;

pub struct Cell {
	pub x: i32,
	pub y: i32,

	pub wifi: f32,
	pub light: f32,
	pub sound: f32,

	pub geotag: i32,
	pub visits: i32,

	pub plants: Vec<Plant>,
	pub seeds: Vec<Seed>,

	pub location: Vector3<f32>,
	pub mesh: Mesh
}

impl Cell {
	pub fn new(factory: &mut Factory, group: &mut Group, plant_settings: &Vec<PlantSetting>, x: usize, y: usize, location: Vector3<f32>) -> Cell {
		let material = Material::MeshBasic { color: 0x00ff00, map: None, wireframe: true };
		let geometry = Geometry::new_plane(CELL_SIZE, CELL_SIZE);
		let mut mesh = factory.mesh(geometry, material.clone());
		mesh.set_orientation(Quaternion::from_angle_x(Deg(-90.0)));

		let location_arr: [f32; 3] = location.into();
		mesh.set_position(location_arr);

		let mut cell = Cell {
			x: x as i32,
			y: y as i32,
			wifi: 0.0,
			light: 0.0,
			sound: 0.0,
			geotag: 0,
			visits: 0,
			plants: vec![],
			seeds: vec![],
			location,
			mesh
		};

		group.add(&cell.mesh);

		if random(0.0, 1.0) >= 0.1 {
			return cell;
		}

		for i in 0..2 {
			let x = random(0.0, CELL_SIZE);
			let z = random(0.0, CELL_SIZE);
			let y = 0.0;

			let plant_location = cell.location + Vector3::new(x, y, z);

			let settings_len = plant_settings.len() as f32;
			let plant_setting_index = random(0.0, settings_len).floor() as usize;

			let setting = plant_settings[plant_setting_index].clone();

			let size = random(setting.growth_limit * 0.08, setting.growth_limit * 0.25);
			let fitness = random(100.0, 115.0);
			let life_expectancy = random(setting.life_expectancy * 0.8, setting.life_expectancy * 1.1);
			let growth_rate = random(0.0007, 0.004);
			let aging_rate = random(0.005, 0.01);
			let mutation_rate = random(0.01, 0.1);
			let stress_rate = random(0.0003, 0.001);

			let dna = Dna {
				size,
				life_expectancy,
				fitness,
				growth_rate,
				aging_rate,
				mutation_rate,
				stress_rate,
				healthy_rate: Dna::healthy_rate(stress_rate)
			};

			println!("Add plant at {:?}", plant_location);

			let plant =
				Plant::new(factory,
	               group,
	               setting,
	               dna,
	               cell.x,
	               cell.y,
	               plant_location);

			cell.plants.push(plant);
		}

		cell
	}

	pub fn update(&mut self, grp: &mut Group) {
		use std::mem;

		let v: Vec<Plant> = vec![];
		let mut plants: Vec<Plant> = mem::replace(&mut self.plants, v);

		plants = plants.into_iter()
			.map(|mut p| {
				p.update(grp, self.wifi, self.sound, self.light);
				p
			})
			.filter(|p| !p.is_dead())
			.collect::<Vec<_>>();

		self.plants = plants;
	}
}

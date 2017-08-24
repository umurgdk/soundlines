use std::collections::HashMap;

use three;
use three::Mesh;
use three::Factory;
use three::Group;
use three::Geometry;
use three::Material;

use cgmath::Rotation3;
use cgmath::Quaternion;
use cgmath::Vector3;
use cgmath::Deg;

use ndarray::prelude::*;

use constants::*;
use plant_setting::PlantSetting;
use cell::Cell;
use helpers::*;
use seed::Seed;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PlantType {
	Normal
}

pub struct World {
	pub width: usize,
	pub height: usize,
	pub plant_settings: Vec<PlantSetting>,
	pub cells: Array2<Cell>,
	pub neighbors: Array2<u32>,
	pub group: Group
}

impl World {
	pub fn new(factory: &mut Factory, width: usize, height: usize) -> Self {
		let mut plant_settings = vec![
			PlantSetting::setting1()
		];

		let mut group = factory.group();
		let cells = Self::generate_cells(factory, &mut group, &plant_settings, width, height);

		let neighbors = Array2::zeros((width, height));

		World {
			width,
			height,
			plant_settings,
			cells,
			neighbors,
			group
		}
	}

	pub fn update(&mut self) {
		for cell in self.cells.iter_mut() {
			cell.update(&mut self.group);
		}

//		let mut neighbors = self.neighbors.view_mut();
//		neighbors.fill(0);
//
//		neighbors += self.cells.slice(s![0..-2, 0..-2]).map(|c| c.plants.len());
//		neighbors += self.cells.slice(s![0..-2, 1..-1]).map(|c| c.plants.len());
//		neighbors += self.cells.slice(s![0..-2, 2..  ]).map(|c| c.plants.len());
//
//		neighbors += self.cells.slice(s![1..-1, 0..-2]).map(|c| c.plants.len());
//		neighbors += self.cells.slice(s![1..-1, 2..  ]).map(|c| c.plants.len());
//
//		neighbors += self.cells.slice(s![2..  , 0..-2]).map(|c| c.plants.len());
//		neighbors += self.cells.slice(s![2..  , 1..-1]).map(|c| c.plants.len());
//		neighbors += self.cells.slice(s![2..  , 2..  ]).map(|c| c.plants.len());
//
//		let mut zv = self.cells.slice_mut(s![1..-1, 1..-1]);
//		zv.zip_mut_with(&neighbors, |cell, &n| {
//
//		});
	}

	fn generate_cells(factory: &mut Factory, group: &mut Group, settings: &Vec<PlantSetting>, width: usize, height: usize) -> Array2<Cell> {
		let scale1 = 0.7;
		let scale2 = 0.25;
		let scale3 = 0.4;

		let offset1 = random(0.0, 0.3);
		let offset2 = random(0.0, 0.3);
		let offset3 = random(0.0, 0.3);

		let mut cells = Vec::with_capacity(width * height);

		for row in 0..height {
			for col in 0..width {
				let wifi = noise(col as f32 * scale1 + offset1, row as f32 * scale1 + offset1);
				let light = noise(col as f32 * scale2 + offset2, row as f32 * scale2 + offset2);
				let sound = noise(col as f32 * scale3 + offset3, row as f32 * scale3 + offset3);

				let x = col as f32 * CELL_SIZE + col as f32 * CELL_GAP;
				let z = row as f32 * CELL_SIZE + row as f32 * CELL_GAP;
				let location = Vector3::new(x, 0.0, z);

				let mut cell =
					Cell::new(factory, group, settings, col, row, location);

				cells.push(cell);
			}
		}


		Array2::from_shape_vec((height, width), cells)
			.expect("Failed to crate array cells")
	}
}

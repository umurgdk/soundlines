use ::models::Entity;
use ::helpers::*;
use ::models::Cell;

impl Entity {
	const WIFI_LOW: f32 = -50.0;
	const WIFI_HIGH: f32 = 50.0;

	// TODO: Add mappings for light and sound too

	pub fn is_dead(&self) -> bool {
		self.fitness <= 0.0 || self.age > self.dna.life_expectancy
	}

	pub fn is_waiting_fruit(&self) -> bool {
		let since_last_seed = self.age - self.last_seed_at;
		since_last_seed < self.setting.fruit_duration
	}

	pub fn is_mating(&self) -> bool {
		let since_last_mate = self.age - self.start_mating_at;
		since_last_mate < self.setting.mating_duration && !self.is_waiting_fruit()
	}

	pub fn is_overcrowded(&self, neighbor_count: i32) -> bool {
		neighbor_count >= self.setting.neighbor_tolerance as i32
	}

	pub fn should_start_mating(&self) -> bool {
		let right_moment = (self.age % self.setting.mating_freq).floor() == 0.0;
		let have_chance = random(0.0, 1.05) < self.setting.birth_proba;

		right_moment && have_chance && !self.is_waiting_fruit()
	}

	fn calculate_sensitivity(&self, cell: &Cell) -> f32 {
		let cell_wifi = clamp_map(cell.wifi, Self::WIFI_LOW, Self::WIFI_HIGH, 0.0, 1.0);

		if cell_wifi > 0.5 {
			let wifi =
				(map(self.setting.wifi_sensitivity, -1.0, 1.0, 0.2, 5.0) *
					map(cell_wifi, 0.0, 1.0, 0.5, 1.0)) / 2.6;

			let light =
				(map(self.setting.light_sensitivity, -1.0, 1.0, 0.2, 5.0) *
					map(cell.light, 0.0, 1.0, 0.5, 1.0) ) / 2.6;

			let sound =
				(map(self.setting.sound_sensitivity, -1.0, 1.0, 0.2, 5.0) *
					map(cell.sound, 0.0, 1.0, 0.5, 1.0) ) / 2.6;

			wifi * light * sound
		} else {
			let wifi =
				(map(self.setting.wifi_sensitivity , -1.0, 1.0, 0.2, 5.0) *
					map(cell_wifi, 0.0, 1.0, 0.5, 1.0) ) / 1.3  ;

			let light =
				(map(self.setting.light_sensitivity, -1.0, 1.0, 0.2, 5.0) *
					map(cell.light, 0.0, 1.0, 0.5, 1.0) ) / 1.3;

			let sound =
				(map(self.setting.sound_sensitivity, -1.0, 1.0, 0.2, 5.0) *
					map(cell.sound, 0.0, 1.0, 0.5, 1.0) ) / 1.3 ;

			wifi * light * sound
		}
	}

	pub fn update_by_neighbors(&mut self, cell: &Cell, neighbor_count: i32) {
		let sensitivity = self.calculate_sensitivity(cell).abs();

		self.age += self.dna.aging_rate / sensitivity;

		// plant only grows when it is fit
		if self.fitness > 30.0 && self.size < self.setting.growth_limit {
			self.size += sensitivity;
		}

		// stressed when overcrowded
		if self.is_overcrowded(neighbor_count) {
			self.fitness -= (self.dna.stress_rate / sensitivity) * neighbor_count as f32 * 4.0;
		}

		if !self.is_mating() && self.should_start_mating() {
			self.start_mating();
		}
	}

	pub fn start_mating(&mut self) {
		self.start_mating_at = self.age;
	}

	pub fn mate(&mut self) {
		self.last_seed_at = self.age;
	}
}
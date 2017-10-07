use ::models::Seed;
use ::helpers::*;

const MAX_AGE: f32 = 250.0;

impl Seed {
	pub fn is_dead(&self) -> bool {
		self.age >= MAX_AGE
	}

	pub fn update(&mut self) {
		self.age += self.dna.growth_rate * 4.0;
	}

	pub fn should_bloom(&self) -> bool {
		let is_right_moment = (self.age % (self.setting.mating_freq / 2.0)).floor() == 0.0;
		let have_chance = random(0.0, 1.05) < self.setting.bloom_proba;

		is_right_moment && have_chance
	}
}
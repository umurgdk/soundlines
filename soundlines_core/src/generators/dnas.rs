use models::Dna;
use models::PlantSetting;

use helpers::random;

pub fn generate(setting: &PlantSetting) -> Dna {
	let stress_rate = random(0.0003, 0.001);

	Dna {
		size: random(setting.growth_limit * 0.08, setting.growth_limit * 0.25),
		fitness: random(100.0, 115.0),
		life_expectancy: random(setting.life_expectancy * 0.8, setting.life_expectancy * 1.1),
		growth_rate: random(0.0007, 0.004),
		aging_rate: random(0.005, 0.01),
		mutation_rate: random(0.01, 0.1),
		stress_rate,
		healthy_rate: (1.0 - stress_rate) / 50.0
	}
}
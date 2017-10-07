use ::models::Dna;
use ::helpers::*;

const GENETIC_BIAS: f32 = 0.65;

impl Dna {
	pub fn reproduce(&self, partner: &Dna) -> Dna {
		let child_fitness = 100.0;
		let mut child_size = 0.0;
		let mut child_life_expectancy = 0.0;
		let mut child_growth_rate = 0.0;
		let mut child_aging_rate = 0.0;
		let mut child_stress_rate = 0.0;
		let mut child_mutation_rate = 0.0;

		if self.size > partner.size {
			child_size = self.size * GENETIC_BIAS + partner.size * (1.0 - GENETIC_BIAS);
		} else if self.size <= partner.size {
			child_size = partner.size * GENETIC_BIAS + self.size * (1.0 - GENETIC_BIAS);
		}

		//growthrate
		if self.growth_rate > partner.growth_rate {
			child_growth_rate = self.growth_rate * GENETIC_BIAS + partner.growth_rate * (1.0 - GENETIC_BIAS);
		} else if self.growth_rate <= partner.fitness {
			child_growth_rate = partner.growth_rate * GENETIC_BIAS + self.growth_rate * (1.0 - GENETIC_BIAS);
		}

		//life_expectancy
		if self.life_expectancy > partner.life_expectancy {
			child_life_expectancy = self.life_expectancy * GENETIC_BIAS + partner.life_expectancy * (1.0 - GENETIC_BIAS);
		} else if self.life_expectancy <= partner.life_expectancy {
			child_life_expectancy = partner.life_expectancy * GENETIC_BIAS + self.life_expectancy * (1.0 - GENETIC_BIAS);
		}

		// agingrate
		if self.aging_rate < partner.aging_rate {
			child_aging_rate = self.aging_rate * GENETIC_BIAS + partner.aging_rate * (1.0 - GENETIC_BIAS) ;
		} else if self.aging_rate >= partner.aging_rate {
			child_aging_rate = partner.aging_rate * GENETIC_BIAS + self.aging_rate * (1.0 - GENETIC_BIAS);

		}

		//stressrate
		if self.stress_rate < partner.stress_rate {
			child_stress_rate = self.stress_rate * GENETIC_BIAS + partner.stress_rate * (1.0 - GENETIC_BIAS) ;
		} else if self.stress_rate >= partner.stress_rate {
			child_stress_rate = partner.stress_rate * GENETIC_BIAS + self.stress_rate * (1.0 - GENETIC_BIAS);
		}

		//mutationrate
		if self.mutation_rate < partner.mutation_rate {
			child_mutation_rate = self.mutation_rate * GENETIC_BIAS + partner.mutation_rate * (1.0 - GENETIC_BIAS) ;
		} else if self.mutation_rate >= partner.mutation_rate {
			child_mutation_rate = partner.mutation_rate * GENETIC_BIAS + self.mutation_rate * (1.0 - GENETIC_BIAS);
		}

		let mut child = Dna {
			size: child_size,
			life_expectancy: child_life_expectancy,
			fitness: child_fitness,
			growth_rate: child_growth_rate,
			aging_rate: child_aging_rate,
			mutation_rate: child_mutation_rate,
			stress_rate: child_stress_rate,
			healthy_rate: (1.0 - child_stress_rate) / 50.0
		};

		child.mutate();
		child
	}

	pub fn mutate(&mut self) {
		if random(0.0, 1.0) < self.mutation_rate {
			self.size *= random(0.9,1.1);
		}

		if random(0.0, 1.0) < self.mutation_rate {
			self.fitness *= random(0.9,1.1);
		}

		if random(0.0, 1.0) < self.mutation_rate {
			self.growth_rate *= random(0.9,1.1);
		}

		if random(0.0, 1.0) < self.mutation_rate {
			self.aging_rate *= random(0.9,1.1);
		}

		if random(0.0, 1.0) < self.mutation_rate {
			self.stress_rate *= random(0.9,1.1);
		}
	}
}
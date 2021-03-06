use std::ops::Deref;
use std::ops::DerefMut;

use soundlines_core::db::models::Dna;
use soundlines_core::db::models::PlantSetting;

use helpers::*;
use constants::*;

#[derive(Debug)]
pub struct SimDna<'s> {
    pub dna: Dna,
    pub setting: &'s PlantSetting
}

impl<'s> Deref for SimDna<'s> {
    type Target = Dna;

    fn deref(&self) -> &Self::Target {
        &self.dna
    }
}

impl<'s> DerefMut for SimDna<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dna
    }
}

impl<'s> SimDna<'s> {
    pub fn random_from_setting(setting: &'s PlantSetting) -> Self {
        let stress_rate = random(0.0003, 0.001);
        let dna = Dna {
            id: -1,
            setting_id: setting.id.expect("setting doesn't have an id"),
            size: random(setting.growth_limit * 0.08, setting.growth_limit * 0.25),
            fitness: random(100.0, 115.0),
            life_expectancy: random(setting.life_expectancy * 0.8, setting.life_expectancy * 1.1),
            growth_rate: random(0.0007, 0.004),
            aging_rate: random(0.005, 0.01),
            mutation_rate: random(0.01, 0.1),
            stress_rate,
            healthy_rate: (1.0 - stress_rate) / 50.0
        };

        Self::from_dna(dna, setting)
    }

    pub fn from_dna(dna: Dna, setting: &'s PlantSetting) -> Self {
        Self {
            dna,
            setting
        }
    }

    pub fn reproduce(&self, partner: &SimDna) -> SimDna {
 		let child_fitness = 100.0;
        let mut child_size = 0.0;
 		let mut child_life_expectancy = 0.0;
 		let mut child_growth_rate = 0.0;
 		let mut child_aging_rate = 0.0;
 		let mut child_stress_rate = 0.0;
 		let mut child_mutation_rate = 0.0;

 		if self.dna.size > partner.dna.size {
    		child_size = self.dna.size * GENETIC_BIAS + partner.dna.size * (1.0 - GENETIC_BIAS);
 		} else if self.dna.size <= partner.dna.size {
   			child_size = partner.dna.size * GENETIC_BIAS + self.dna.size * (1.0 - GENETIC_BIAS);
 		}

 		//growthrate
 		if self.dna.growth_rate > partner.dna.growth_rate {
    		child_growth_rate = self.dna.growth_rate * GENETIC_BIAS + partner.dna.growth_rate * (1.0 - GENETIC_BIAS);
 		} else if self.dna.growth_rate <= partner.dna.fitness {
            child_growth_rate = partner.dna.growth_rate * GENETIC_BIAS + self.dna.growth_rate * (1.0 - GENETIC_BIAS);
 		}

  		//life_expectancy
  		if self.dna.life_expectancy > partner.dna.life_expectancy {
    		child_life_expectancy = self.dna.life_expectancy * GENETIC_BIAS + partner.dna.life_expectancy * (1.0 - GENETIC_BIAS);
 		} else if self.dna.life_expectancy <= partner.dna.life_expectancy {
            child_life_expectancy = partner.dna.life_expectancy * GENETIC_BIAS + self.dna.life_expectancy * (1.0 - GENETIC_BIAS);
 		}

 		// agingrate
 		if self.dna.aging_rate < partner.dna.aging_rate {
    		child_aging_rate = self.dna.aging_rate * GENETIC_BIAS + partner.dna.aging_rate * (1.0 - GENETIC_BIAS) ;
 		} else if self.dna.aging_rate >= partner.dna.aging_rate {
            child_aging_rate = partner.dna.aging_rate * GENETIC_BIAS + self.dna.aging_rate * (1.0 - GENETIC_BIAS);

 		}

  		//stressrate
  		if self.dna.stress_rate < partner.dna.stress_rate {
     		child_stress_rate = self.dna.stress_rate * GENETIC_BIAS + partner.dna.stress_rate * (1.0 - GENETIC_BIAS) ;
 		} else if self.dna.stress_rate >= partner.dna.stress_rate {
            child_stress_rate = partner.dna.stress_rate * GENETIC_BIAS + self.dna.stress_rate * (1.0 - GENETIC_BIAS);
 		}

 		//mutationrate
  		if self.dna.mutation_rate < partner.dna.mutation_rate {
    		child_mutation_rate = self.dna.mutation_rate * GENETIC_BIAS + partner.dna.mutation_rate * (1.0 - GENETIC_BIAS) ;
 		} else if self.dna.mutation_rate >= partner.dna.mutation_rate {
            child_mutation_rate = partner.dna.mutation_rate * GENETIC_BIAS + self.dna.mutation_rate * (1.0 - GENETIC_BIAS);
 		}

 		let child = Dna {
 		    id: -1,
 		    setting_id: self.dna.setting_id,
            size: child_size,
            life_expectancy: child_life_expectancy,
            fitness: child_fitness,
            growth_rate: child_growth_rate,
            aging_rate: child_aging_rate,
            mutation_rate: child_mutation_rate,
            stress_rate: child_stress_rate,
            healthy_rate: (1.0 - child_stress_rate) / 50.0
 		};

        let mut sim_child = SimDna::from_dna(child, self.setting);
 		sim_child.mutate();
 		sim_child
    }

    pub fn mutate(&mut self) {
        if random(0.0, 1.0) < self.dna.mutation_rate {
     		self.dna.size *= random(0.9,1.1);
   		}

  		if random(0.0, 1.0) < self.dna.mutation_rate {
     		self.dna.fitness *= random(0.9,1.1);
   		}

  		if random(0.0, 1.0) < self.dna.mutation_rate {
     		self.dna.growth_rate *= random(0.9,1.1);
   		}

  		if random(0.0, 1.0) < self.dna.mutation_rate {
     		self.dna.aging_rate *= random(0.9,1.1);
   		}

  		if random(0.0, 1.0) < self.dna.mutation_rate {
     		self.dna.stress_rate *= random(0.9,1.1);
   		}
    }
}

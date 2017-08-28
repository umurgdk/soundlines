use std::ops::Deref;
use std::ops::DerefMut;

use soundlines_core::db::models::Dna;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::models::Seed;

use helpers::*;
use constants::*;
use sim_dna::SimDna;

pub struct SimSeed<'d, 's: 'd> {
    pub seed: Seed,
    pub dna: &'d SimDna<'s>,
    pub setting: &'s PlantSetting,
}

impl<'d, 's> Deref for SimSeed<'d, 's> {
    type Target = Seed;

    fn deref(&self) -> &Self::Target {
        &self.seed
    }
}

impl<'d, 's> DerefMut for SimSeed<'d, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seed
    }
}

impl<'d, 's> SimSeed<'d, 's> {
    pub fn from_seed(seed: Seed, dna: &'d SimDna<'s>, setting: &'s PlantSetting) -> Self {
        Self { seed, dna, setting  }
    }

    pub fn is_dead(&self) -> bool {
        self.seed.age >= 250.0
    }

    pub fn update(&mut self) {
        self.seed.age += self.dna.growth_rate * 4.0;
    }

    pub fn should_bloom(&self) -> bool {
        let mating_freq_seconds = self.setting.mating_freq.0.as_secs() as f64;
        let since_created_seconds = since(self.seed.created_at).as_secs() as f64;

        let is_right_moment = since_created_seconds % (mating_freq_seconds / 2.0) < 1.0;
        let have_chance = random(0.0, 1.0) < self.setting.bloom_proba;

        is_right_moment && have_chance
    }
}

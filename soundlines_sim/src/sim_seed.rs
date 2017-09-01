use std::ops::Deref;
use std::ops::DerefMut;

use geo::Point;

use soundlines_core::db::models::Dna;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::models::Seed;

use helpers::*;
use constants::*;
use sim_dna::SimDna;
use context::SimContext;

pub struct SimSeed<'d, 's: 'd, 'c> {
    pub seed: Seed,
    pub dna: &'d SimDna<'s>,
    pub setting: &'s PlantSetting,
    pub ctx: &'c SimContext
}

impl<'d, 's: 'd, 'c> Deref for SimSeed<'d, 's, 'c> {
    type Target = Seed;

    fn deref(&self) -> &Self::Target {
        &self.seed
    }
}

impl<'d, 's: 'd, 'c> DerefMut for SimSeed<'d, 's, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seed
    }
}

impl<'d, 's: 'd, 'c> SimSeed<'d, 's, 'c> {
    pub fn from_seed(seed: Seed, dna: &'d SimDna<'s>, setting: &'s PlantSetting, ctx: &'c SimContext) -> Self {
        Self { seed, dna, setting, ctx }
    }

    pub fn is_dead(&self) -> bool {
        self.seed.age >= self.ctx.seed_max_age
    }

    pub fn update(&mut self) {
        self.seed.age += self.dna.growth_rate * 4.0;
    }

    pub fn should_bloom(&self) -> bool {
        let is_right_moment =  (self.seed.age % (self.setting.mating_freq / 2.0)).floor() == 0.0;
        let have_chance = random(0.0, 1.05) < self.setting.bloom_proba;

        is_right_moment && have_chance
    }
}

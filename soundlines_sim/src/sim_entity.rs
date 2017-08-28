use std::ops::Deref;
use std::ops::DerefMut;

use chrono::prelude::*;

use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::Dna;

use helpers::*;
use sim_dna::SimDna;

pub struct SimEntity<'s: 'd, 'd> {
    pub entity: Entity,
    pub setting: &'s PlantSetting,
    pub dna: &'d SimDna<'s>
}

impl<'s: 'd, 'd> Deref for SimEntity<'s, 'd> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl<'s: 'd, 'd> DerefMut for SimEntity<'s, 'd> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

impl<'a: 'd, 'd> SimEntity<'a, 'd> {
    pub fn from_entity(entity: Entity, setting: &'a PlantSetting, dna: &'d SimDna<'a>) -> Self {
        Self {
            entity,
            setting,
            dna,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.fitness <= 0.0 || self.entity.age > self.dna.life_expectancy
    }

    pub fn is_waiting_fruit(&self) -> bool {
        let since_last_seed = since(self.entity.last_seed_at);
        since_last_seed < self.setting.fruit_duration.0
    }

    pub fn is_mating(&self) -> bool {
        let since_last_mate = since(self.entity.start_mating_at);
        since_last_mate < self.setting.mating_duration.0 && !self.is_waiting_fruit()
    }

    pub fn is_overcrowded(&self, neighbor_count: i32) -> bool {
        neighbor_count >= self.setting.neighbor_tolerance as i32
    }

    pub fn should_start_mating(&self) -> bool {
        let since_last_mate = since(self.entity.start_mating_at);

        let mating_freq_seconds = self.setting.mating_freq.0.as_secs() as f64;
        let since_last_seconds = since_last_mate.as_secs() as f64;

        let modulus = since_last_seconds % mating_freq_seconds;

        let right_moment = modulus < 1.0;
        let have_chance = random(0.0, 1.0) < self.setting.birth_proba;

        right_moment && have_chance && !self.is_waiting_fruit()
    }

    fn calculate_sensitivity(&self, cell: &Cell) -> f32 {
        if cell.wifi > 0.5 {
       		let wifi =  
       			(map(self.setting.wifi_sensitivity, -1.0, 1.0, 0.2, 5.0) *
       		     map(cell.wifi, 0.0, 1.0, 0.5, 1.0)) / 2.6;

        	let light = 
        		(map(self.setting.light_sensitivity, -1.0, 1.0, 0.2, 5.0) *
        		 map(cell.light, 0.0, 1.0, 0.5, 1.0) ) / 2.6;

        	let sound = 
        		(map(self.setting.sound_sensitivity, -1.0, 1.0, 0.2, 5.0) *
        		 map(cell.sound, 0.0, 1.0, 0.5, 1.0) ) / 2.6;

        	wifi * light * sound
     	} else if cell.wifi <= 0.5  {
       		let wifi = 
       			(map(self.setting.wifi_sensitivity , -1.0, 1.0, 5.0, 0.2) *
       			 map(cell.wifi, 0.0, 1.0, 0.5, 1.0) ) / 1.3  ;

       		let light = 
       			(map(self.setting.light_sensitivity, -1.0, 1.0, 5.0, 0.2) *
       			 map(cell.light, 0.0, 1.0, 0.5, 1.0) ) / 1.3;

       		let sound = 
       			(map(self.setting.sound_sensitivity, -1.0, 1.0, 5.0, 0.2) *
       			 map(cell.sound, 0.0, 1.0, 0.5, 1.0) ) / 1.3 ;

        	wifi * light * sound
     	} else {
     	    unreachable!();
     	}
    }

    pub fn update_by_neighbors(&mut self, cell: &Cell, neighbor_count: i32) {
        let sensitivity = self.calculate_sensitivity(cell);

  		self.entity.age += self.dna.aging_rate / sensitivity;

  		// plant only grows when it is fit
  		if self.entity.fitness > 30.0 && self.entity.size < self.setting.growth_limit {
     		self.entity.size += self.dna.growth_rate * sensitivity;
  		}

  		// stressed when overcrowded 
  		if self.is_overcrowded(neighbor_count) { 
     		self.entity.fitness -= (self.dna.stress_rate / sensitivity) * neighbor_count as f32 * 4.0;
  		}
    }

    pub fn start_mating(&mut self) {
        self.entity.start_mating_at = Utc::now();
    }

    pub fn mate(&mut self) {
        self.entity.last_seed_at = Utc::now();
    }
}

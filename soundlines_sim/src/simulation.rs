use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;

use geo::Point;
use geo::Polygon;
use geo::haversine_distance::HaversineDistance;

use rand;
use rand::Rand;
use cgmath::Vector2;
use cgmath::InnerSpace;
use rayon::prelude::*;

use soundlines_core::db::Pool;
use soundlines_core::db::extensions::*;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Seed;

use helpers::*;
use context::SimContext;
use sim_entity::SimEntity;
use sim_dna::SimDna;
use sim_seed::SimSeed;
use sim_geo::get_seed_location;

pub fn run(connection_pool: Pool) {
    let connection_pool = Arc::new(connection_pool);
    let ctx = SimContext::new(connection_pool.get().expect("Failed to get connection from pool"));

    let cells = ctx.db_conn.all::<Cell>().expect("Failed to fetch cells")
        .into_iter()
        .map(|c| (c.id, c))
        .collect::<HashMap<_, _>>();

    let plant_settings = ctx.db_conn.all::<PlantSetting>().expect("Failed to fetch plant settings")
        .into_iter()
        .map(|s| (s.id.unwrap(), s))
        .collect::<HashMap<_, _>>();

    let dnas = ctx.db_conn.all::<Dna>().expect("Failed to fetch dnas")
        .into_iter()
        .map(|d| {
            let setting = &plant_settings[&d.setting_id];
            (d.id, SimDna::from_dna(d, setting))
        })
        .collect::<HashMap<_, _>>();

    let mut entities = ctx.db_conn.all::<Entity>().expect("Failed to fetch entities")
        .into_iter()
        .map(|e| {
            let setting = &plant_settings[&e.setting_id];
            let dna = &dnas[&e.dna_id];

            (e.id, SimEntity::from_entity(e, setting, dna))
        })
        .collect::<HashMap<_, SimEntity>>();

    let mut seeds = ctx.db_conn.all::<Seed>().expect("Failed to fetch seeds")
        .into_iter()
        .map(|s| {
            let dna = &dnas[&s.dna_id];
            let setting = &plant_settings[&dna.setting_id];

            // seed from db so it has id
            (s.id.unwrap(), SimSeed::from_seed(s, dna, setting))
        })
        .collect::<HashMap<_,_>>();

    let entity_locations = entities
        .iter()
        .map(|(id, e)| (*id, Point::new(e.point.x, e.point.y)))
        .collect::<HashMap<_, _>>();

    loop {
        let mut possible_mate_candidates: Vec<(i32, Vec<i32>)> = vec![];
        for (id, entity) in entities.iter_mut() {
            let setting = plant_settings.get(&entity.setting_id)
                .expect(&format!("Plant setting with id: {} not found", entity.setting_id));

            let entity_location = Point::new(entity.point.x, entity.point.y);

            let overcrowd_distance = setting.overcrowd_distance;
            let neighbors_count = entity_locations
                .iter()
                .filter(|&(_, &location)| entity_location.haversine_distance(&location) < overcrowd_distance as f64)
                .count();

            {
                let cell = &cells[&entity.cell_id];
                entity.update_by_neighbors(cell, neighbors_count as i32);
            }

            if entity.should_start_mating() {
                entity.start_mating();
            }

            if !entity.is_mating() {
                continue;
            }

            let mating_distance = setting.mating_distance as f64;
            let mate_candidate_ids = entity_locations
                .iter()
                .filter(|&(_, &location)| entity_location.haversine_distance(&location) < mating_distance)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();

            possible_mate_candidates.push((*id, mate_candidate_ids));
        }

        let mut new_seeds = vec![];

        let mut rng = rand::thread_rng();
        for &(entity_id, ref mating_candiadates) in possible_mate_candidates.iter() {
            let matched_entity_id = random_choice_with(mating_candiadates, |candidate_id| {
                entities[&candidate_id].is_mating()
            });

            if let Some(matched_id) = matched_entity_id {
                entities.get_mut(&entity_id).unwrap().mate();
                entities.get_mut(matched_id).unwrap().mate();

                let entity1 = &entities[&entity_id];
                let entity2 = &entities[matched_id];

                let wind = Vector2::<f64>::rand(&mut rng).normalize_to(25.0);
                let seed_location = get_seed_location(entity1, entity2, wind);
                let child_dna = entity1.dna.reproduce(entity2.dna);

                new_seeds.push((child_dna, entity1.setting, seed_location));
            }
        }

        // update seeds
        for (_, seed) in seeds.iter_mut() {
            seed.update();
        }

        use rayon::iter::Zip;

        // create seeds
        new_seeds
            .into_par_iter()
            .for_each_with(connection_pool.clone(), |pool, (dna, setting, loc)| {
                let conn = pool.get().expect("Failed to get connection in parallel seed creation");
                let cell = Cell::find_containing(&*conn, &loc).expect("Failed to fetch cell containing a location");

                // TODO: handle seed thrown outside of the grid
                if let Some(cell) = cell {
                    let seed = Seed::new(dna.id, into_core_point(&loc), cell.id);
                    conn.insert(&seed).expect("Failed to write new seed to db");
                }
            });

        // destroy dead seeds
        let (dead_seeds, other_seeds): (Vec<_>, Vec<_>) = seeds.into_iter().partition(|&(_, ref s)| s.is_dead());
        dead_seeds.into_par_iter()
            .for_each_with(connection_pool.clone(), |pool, (id, _)| {
                let conn = pool.get().expect("Failed to get connection in parallel seed destroying");
                conn.delete::<Seed>(id).expect("Failed to delete dead seed");
            });

        // create new entities for bloomed seeds
        let (blooming_seeds, other_seeds): (Vec<_>, Vec<_>) = other_seeds.into_iter().partition(|&(_, ref s)| s.should_bloom());
        blooming_seeds.into_par_iter()
            .for_each_with(connection_pool.clone(), |pool, (id, s)| {
                let conn = pool.get().expect("Failed to get connection in parallel seed blooming");
                conn.delete::<Seed>(id).expect("Failed to delete bloomed seed");

                let entity = Entity::new(s.seed.point, s.seed.cell_id, s.setting, s.dna);
                conn.insert(&entity).expect("Failed to insert new entity for blooming seed");
            });

        // Update rest of the seeds
        seeds = other_seeds.into_iter().collect::<HashMap<_, _>>();
        seeds.par_iter()
            .for_each_with(connection_pool.clone(), |pool, (&id, ref s)| {
                let conn = pool.get().expect("Failed to get connection in parallel seed updating");
                conn.update(id, &s.seed).expect("Failed to update seed");
            });

        let (dead_entities, other_entities) = entities.into_iter().partition(|&(_, ref e)| e.is_dead());
        entities = other_entities;
        // Delete dead entities
        dead_entities.into_par_iter()
            .for_each_with(connection_pool.clone(), |pool, (id, _)| {
                let conn = pool.get().expect("Failed to get connection in parallel destroying dead entities");
                conn.delete::<Entity>(id).expect("Failed to delete entity");
            });

        // Update rest of the entities
        entities.par_iter()
            .for_each_with(connection_pool.clone(), |pool, (&id, ref e)| {
                let conn = pool.get().expect("Failed to get connection in parallel updating entities");
                conn.update(id, &e.entity).expect("Failed to update entity");
            });

        thread::sleep(Duration::from_secs(1));
    }
}

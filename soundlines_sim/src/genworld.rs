use std::error::Error;
use std::collections::HashMap;

use soundlines_core::db::Pool;
use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::Seed;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::extensions::*;

use sim_seed;
use sim_seed::SeedDraft;
use helpers::*;

pub fn run(connection_pool: Pool, clear: bool) -> Result<(), Box<Error>> {
    let conn = connection_pool.get()?;
    let cells = conn.all::<Cell>()?;
    let plant_settings = conn.all::<PlantSetting>()?;
    let prefabs = plant_settings
        .iter()
        .map(|s| (s.id.unwrap(), &s.prefab))
        .collect::<HashMap<_, _>>();

    if clear {
        conn.delete_all::<Dna>()?;
        conn.delete_all::<Entity>()?;
        conn.delete_all::<Seed>()?;
    }

    let mut dnas_to_create = Vec::with_capacity(cells.len() * 2);
    let mut seed_locations = Vec::with_capacity(cells.len() * 2);
    let mut cell_ids = Vec::with_capacity(cells.len() * 2);

    for cell in cells.into_iter() {
        if random(0.0, 1.0) >= 0.1 {
            continue;
        }

        for _ in 0..10 {
            let SeedDraft { location, dna, .. } = sim_seed::generate(&plant_settings, &cell);
            seed_locations.push(location);
            dnas_to_create.push(dna);
            cell_ids.push(cell.id);
        }
    }

    let dnas = conn.insert_batch_return(&dnas_to_create, true)?;

    let seeds_to_insert: Vec<Seed> =
        dnas.into_iter()
            .enumerate()
            .map(|(i, dna)| {
                let prefab = prefabs[&dna.setting_id].to_owned();
                Seed::new(dna.id, seed_locations[i].clone(), cell_ids[i], dna.setting_id, prefab)
            })
            .collect();

    let seeds_len = seeds_to_insert.len();
    conn.insert_batch(&seeds_to_insert)?;

    println!("{} seeds spread into world...", seeds_len);
    Ok(())
}

use std::error::Error;

use soundlines_core::db::Pool;
use soundlines_core::db::models::*;
use soundlines_core::db::extensions::*;

use helpers::*;

pub fn run(connection_pool: Pool) -> Result<(), Box<Error>> {
    let db = connection_pool.get()?;

    let plant_settings = db.all::<PlantSetting>()?;

    let entities = db.all::<Entity>()?;
    let mut entity_ids = Vec::with_capacity(entities.len());

    let entities = entities
        .into_iter()
        .map(|mut e| {
            let setting = random_choice(&plant_settings);
            e.setting_id = setting.id.expect("expected setting to have an id");
            e.prefab = setting.prefab.clone();

            entity_ids.push(e.id);
            e
        })
        .collect::<Vec<_>>();

    println!("Updating entities...");
    db.update_batch(&entity_ids, &entities)?;

    println!("{} entities updated successfully", entities.len());

    Ok(())
}

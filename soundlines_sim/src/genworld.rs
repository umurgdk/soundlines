use std::error::Error;
use std::collections::HashMap;

use soundlines_core::db::Pool;
use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Cell;
use soundlines_core::db::models::Seed;
use soundlines_core::db::models::Entity;
use soundlines_core::db::models::Parameter;
use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::extensions::*;

use geo::Point as GPoint;
use geo::Polygon as GPolygon;
use soundlines_core::postgis::ewkb::Point;

use sim_dna::SimDna;
use sim_entity::SimEntity;

use helpers::*;
use context::SimContext;

pub fn run(connection_pool: Pool, clear: bool) -> Result<(), Box<Error>> {
    let ctx = SimContext {
        time_scale: 4.0,
        ..SimContext::default()
    };

    let conn = connection_pool.get()?;

    let parameters = conn.first::<Parameter>()?
        .ok_or("No paramters set on db")?;

    let cell_size = parameters.cell_size as f64;

    let cells = conn.all::<Cell>()?;
    let plant_settings = conn.all::<PlantSetting>()?;

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

        let (location1, location2) = get_random_locations(&cell, cell_size);
        seed_locations.push(location1);
        seed_locations.push(location2);

        let setting = random_choice(&plant_settings);
        let dna = SimDna::random_from_setting(setting);
        dnas_to_create.push(dna.dna);

        let dna = SimDna::random_from_setting(setting);
        dnas_to_create.push(dna.dna);

        cell_ids.push(cell.id);
        cell_ids.push(cell.id);
    }

    let dnas = conn.insert_batch_return(&dnas_to_create, true)?;

    let seeds_to_insert: Vec<Seed> = 
        dnas.into_iter()
            .enumerate()
            .map(|(i, dna)| {
                let setting_id = dna.setting_id;
                Seed::new(dna.id, seed_locations[i].clone(), cell_ids[i], setting_id)
            })
            .collect();

    let seeds_len = seeds_to_insert.len();
    conn.insert_batch(&seeds_to_insert)?;

    println!("{} seeds spread into world...", seeds_len);
    Ok(())
}

fn get_random_locations(cell: &Cell, cell_size: f64) -> (Point, Point) {
    use rand::Rand;
    use cgmath::Deg;
    use geo::centroid::Centroid;
    use geo::boundingbox::BoundingBox;
    use geo::haversine_destination::HaversineDestination;

    let points: Vec<_> = cell.geom.rings[0].points.iter().map(into_geo_point).collect();
    let polygon = GPolygon::new(points.into(), vec![]);
    let bbox = polygon.bbox().expect("Bounding box of the cell");

    let mut location1 = GPoint::new(bbox.xmin, bbox.ymin);
    location1 = location1.haversine_destination(90.0, random(0.0, cell_size));
    location1 = location1.haversine_destination(0.0, random(0.0, cell_size));

    let mut location2 = GPoint::new(bbox.xmin, bbox.ymin);
    location2 = location2.haversine_destination(90.0, random(0.0, cell_size));
    location2 = location2.haversine_destination(0.0, random(0.0, cell_size));

    (
        Point::new(location1.x(), location1.y(), Some(4326)), 
        Point::new(location2.x(), location2.y(), Some(4326))
    )
}

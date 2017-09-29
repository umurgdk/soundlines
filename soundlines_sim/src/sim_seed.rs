use std::ops::Deref;
use std::ops::DerefMut;

use soundlines_core::db::models::PlantSetting;
use soundlines_core::db::models::Seed;
use soundlines_core::db::models::Dna;
use soundlines_core::db::models::Cell;
use soundlines_core::postgis::ewkb::Point;
use geo::Polygon as GPolygon;
use geo::Point as GPoint;

use helpers::*;
use sim_dna::SimDna;
use context::SimContext;

pub struct SeedDraft {
    pub dna: Dna,
    pub location: Point,
    pub cell_id: i32
}

pub fn generate(plant_settings: &[PlantSetting], cell: &Cell) -> SeedDraft {
    let location = get_random_location(cell, 50.0);
    let setting = random_choice(plant_settings);
    let dna = SimDna::random_from_setting(setting);

    SeedDraft { dna: dna.dna, location, cell_id: cell.id }
}

fn get_random_location(cell: &Cell, cell_size: f64) -> Point {
    use geo::boundingbox::BoundingBox;
    use geo::haversine_destination::HaversineDestination;

    let points: Vec<_> = cell.geom.rings[0].points.iter().map(into_geo_point).collect();
    let polygon = GPolygon::new(points.into(), vec![]);
    let bbox = polygon.bbox().expect("Bounding box of the cell");

    let mut location1 = GPoint::new(bbox.xmin, bbox.ymin);
    location1 = location1.haversine_destination(90.0, random(0.0, cell_size));
    location1 = location1.haversine_destination(0.0, random(0.0, cell_size));

    Point::new(location1.x(), location1.y(), Some(4326))
}

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

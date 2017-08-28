use std::f64;
use cgmath::Rad;
use cgmath::Deg;
use cgmath::Vector2;
use cgmath::InnerSpace;

use geo::Point;
use geo::LineString;
use geo::centroid::Centroid;
use geo::haversine_distance::HaversineDistance;
use geo::haversine_destination::HaversineDestination;

use sim_dna::SimDna;
use sim_seed::SimSeed;
use sim_entity::SimEntity;

pub fn get_seed_location(from: &SimEntity, to: &SimEntity, wind: Vector2<f64>) -> Point<f64> {
    let from_location = Point::new(from.entity.point.x, from.entity.point.y);
    let to_location = Point::new(to.entity.point.x, to.entity.point.y);
    let line_string = LineString(vec![from_location, to_location]);

    let center = line_string.centroid().unwrap();
    let magnitude = wind.magnitude();
    let wind = wind.normalize();
    let angle: Deg<f64> = Rad(f64::consts::FRAC_PI_2 - f64::atan(wind.y / wind.x)).into();

    center.haversine_destination(angle.0, magnitude * 0.5)
}

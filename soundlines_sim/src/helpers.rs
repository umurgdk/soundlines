use std::time::Duration;

use rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

use chrono::prelude::*;

use noise::Perlin;
use noise::NoiseModule;

use geo::Point as GPoint;
use soundlines_core::postgis::ewkb::Point;
use soundlines_core::postgis::ewkb::Polygon;

pub fn random<T>(min: T, max: T) -> T 
    where T: PartialEq + PartialOrd + rand::distributions::range::SampleRange
{
    if min == max {
	    return min;
    }

    let mut rng = rand::thread_rng();
    Range::new(min, max).ind_sample(&mut rng)
}

pub fn random_choice<T>(elems: &[T]) -> &T {
    let mut rng = rand::thread_rng();

    let range = Range::new(0, elems.len());
    &elems[range.ind_sample(&mut rng)]
}

pub fn random_choice_with<T, F>(elems: &[T], predicate: F) -> Option<&T> 
    where F: Fn(&T) -> bool
{
    use std::collections::HashSet;

    let mut rng = rand::thread_rng();

    let range = Range::new(0, elems.len());
    let mut tried_indexes = HashSet::new();

    while tried_indexes.len() < elems.len() {
        let index = range.ind_sample(&mut rng);
        tried_indexes.insert(index);

        if predicate(&elems[index]) {
            return Some(&elems[index]);
        }
    }

    None
}

pub fn noise(x: f32, y: f32) -> f32 {
    let perlin = Perlin::new();
    perlin.get([x, y])
}

pub fn map(val: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (val - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn since(time: DateTime<Utc>) -> Duration {
    let now = Utc::now();
    now.signed_duration_since(time).to_std()
        .expect("Failed to convert chrono::Duration to std::time::Duration")
}

pub fn into_core_point(point: &GPoint<f64>) -> Point {
    Point::new(point.x(), point.y(), Some(4326))
}

pub fn into_geo_point(point: &Point) -> GPoint<f64> {
    GPoint::new(point.x, point.y)
}

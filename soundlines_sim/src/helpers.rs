use rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

use geo::Point as GPoint;
use soundlines_core::postgis::ewkb::Point;

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

pub fn clamp(val: f32, lower: f32, upper: f32) -> f32 {
    val.max(lower).min(upper)
}

pub fn clamp_map(val: f32, lower: f32, upper: f32, map_lower: f32, map_upper: f32) -> f32 {
    map(clamp(val, lower, upper), lower, upper, map_lower, map_upper)
}

pub fn map(val: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (val - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn into_core_point(point: &GPoint<f64>) -> Point {
    Point::new(point.x(), point.y(), Some(4326))
}

pub fn into_geo_point(point: &Point) -> GPoint<f64> {
    GPoint::new(point.x, point.y)
}

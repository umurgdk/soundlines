use std::time::Duration;
use std::time::Instant;
use rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

pub struct Timer {
	start: Instant,
	name: String
}

impl Timer {
	pub fn new<S: Into<String>>(name: S) -> Timer {
		Timer { start: Instant::now(), name: name.into() }
	}

	pub fn finish(self) -> (String, u64) {
		let since_start = Instant::now().duration_since(self.start);
		let mut millis = since_start.as_secs() * 1000;
		millis += since_start.subsec_nanos() as u64 / 1000000u64;

		(self.name, millis)
	}

	pub fn since(&self) -> Duration {
		Instant::now().duration_since(self.start)
	}

	pub fn reset(&mut self) {
		self.start = Instant::now();
	}
}

pub fn new_timer<S: Into<String>>(name: S) -> Timer {
	Timer::new(name)
}

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

	if elems.len() == 0 {
		return None;
	}

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

pub fn get_seed_location(from: &::models::Entity, to: &::models::Entity) -> [f64;2] {
	use geo::*;
	use geo::algorithm::haversine_destination::HaversineDestination;
	use geo::algorithm::centroid::Centroid;
	use cgmath::Vector2;
	use cgmath::Deg;
	use cgmath::Rad;
	use cgmath::prelude::*;
	use rand::Rand;

	let mut rng = rand::thread_rng();
	let wind = Vector2::<f64>::rand(&mut rng).normalize_to(30.0);

	// TODO: Check lat, lng order
	let from_location = Point::new(from.point[0], from.point[1]);
	let to_location = Point::new(to.point[0], to.point[1]);
	let line_string = LineString(vec![from_location, to_location]);

	let center = line_string.centroid().unwrap();
	let magnitude = wind.magnitude();
	let wind = wind.normalize();
	let angle: Deg<f64> = Rad(::std::f64::consts::FRAC_PI_2 - f64::atan(wind.y / wind.x.max(0.01))).into();

	let target = center.haversine_destination(angle.0, magnitude);
	[target.x(), target.y()]
}

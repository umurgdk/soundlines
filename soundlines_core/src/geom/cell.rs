use rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;
use geo::haversine_distance::HaversineDistance;
use geo::haversine_destination::HaversineDestination;
use geo::Point as GPoint;
use cgmath;
use cgmath::InnerSpace;

use models::Point;
use models::Polygon;

pub fn width(polygon: &Polygon) -> f64 {
	let top_left = GPoint::new(polygon[0][0], polygon[0][1]);
	let top_right = GPoint::new(polygon[3][0], polygon[3][1]);

	top_right.haversine_distance(&top_left).abs()
}

pub fn height(polygon: &Polygon) -> f64 {
	let top_left = GPoint::new(polygon[0][0], polygon[0][1]);
	let bottom_left = GPoint::new(polygon[1][0], polygon[1][1]);

	top_left.haversine_distance(&bottom_left).abs()
}

pub fn random_inner_point(polygon: &Polygon) -> Point {
	let top_left = GPoint::new(polygon[0][0], polygon[0][1]);
	let width = width(polygon);
	let height = height(polygon);

	let width_range = Range::new(0.0, width);
	let height_range = Range::new(0.0, height);
	let mut rng = rand::thread_rng();

	let rel_x = width_range.ind_sample(&mut rng);
	let rel_y = -height_range.ind_sample(&mut rng);
	let rel_vec = cgmath::Vector2::new(rel_x, rel_y);

	let magnitude = rel_vec.magnitude();
	let rel_vec = rel_vec.normalize();
	let angle: cgmath::Deg<f64> = cgmath::Rad(::std::f64::consts::FRAC_PI_2 - f64::atan(rel_vec.y / rel_vec.x)).into();

	let point = top_left.haversine_destination(angle.0, magnitude);
	[point.x(), point.y()]
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn test_random_inner_point() {}
}
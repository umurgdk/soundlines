use spade::PointN;
use spade::BoundingRect;
use spade::SpatialObject;
use models;

impl SpatialObject for models::Seed {
	type Point = [f64; 2];

	fn mbr(&self) -> BoundingRect<Self::Point> {
		BoundingRect::from_point(self.point)
	}

	fn distance2(&self, point: &Self::Point) -> <Self::Point as PointN>::Scalar {
		((point[0] - self.point[0]).powi(2) + (point[1] - self.point[1]).powi(2)).sqrt()
	}
}
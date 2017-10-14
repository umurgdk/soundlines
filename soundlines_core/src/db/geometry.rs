use models::Polygon;

pub fn encode_polygon(polygon: &Polygon) -> String {
	let points = polygon.iter().map(|p| format!("{} {}", p[0], p[1])).collect::<Vec<_>>().join(",");
	format!("POLYGON(({}))", points)
}
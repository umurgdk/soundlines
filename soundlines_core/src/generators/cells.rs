use geo::Point as GPoint;
use geo::Polygon as GPolygon;
use geo::haversine_destination::HaversineDestination;
use geo::distance::Distance;
use geo::translate::Translate;
use geo::intersects::Intersects;
use geo::boundingbox::BoundingBox;

use models::Cell;

const CELL_SIZE: f64 = 50.0;

pub fn generate() -> Vec<Cell> {
	generate_cell_polygons(CELL_SIZE).into_iter().map(|p| {
		let points = p.exterior.into_iter().map(|p| [p.x(), p.y()]).collect::<Vec<_>>();

		Cell {
			id: 0,
			geom: points,

			wifi: 0.0,
			wifi_total: 0.0,
			wifi_count: 0.0,

			light: 0.0,
			light_total: 0.0,
			light_count: 0.0,

			sound: 0.0,
			sound_total: 0.0,
			sound_count: 0.0,

			sns: 0,
			visit: 0
		}
	}).collect()
}

fn generate_cell_polygons(cell_size: f64) -> Vec<GPolygon<f64>> {
	let polygon = GPolygon::new(POINTS.to_vec().into(), vec![]);
	let bbox = polygon.bbox().expect("Bounding couldn't calculated!");

	let tl = GPoint::new(bbox.xmin, bbox.ymax);
	let tr = GPoint::new(bbox.xmax, bbox.ymax);
	let bl = GPoint::new(bbox.xmin, bbox.ymin);

	let cell_tl = tl.clone();
	let cell_tr = cell_tl.haversine_destination(90.0, cell_size as f64);
	let cell_br = cell_tr.haversine_destination(180.0, cell_size as f64);
	let cell_bl = cell_tl.haversine_destination(180.0, cell_size as f64);

	let cell_width = cell_tl.distance(&cell_tr);
	let cell_height = cell_tl.distance(&cell_bl);

	let cell_tmp = GPolygon::new(vec![cell_tl, cell_bl, cell_br, cell_tr, cell_tl].into(), vec![]);

	let horizontal_length = tl.distance(&tr);
	let vertical_length = tl.distance(&bl);

	let num_h_cells = f64::ceil(horizontal_length / cell_width);
	let num_v_cells = f64::ceil(vertical_length / cell_height);

	let mut cells = vec![];
	for row in 0..num_v_cells as i32 {
		for col in 0..num_h_cells as i32 {
			let cell = cell_tmp.clone();
			let target_point = cell_tl
				.haversine_destination(90.0, col as f64 * cell_size as f64)
				.haversine_destination(180.0, row as f64 * cell_size as f64);

			let offset = target_point - cell_tl;
			let cell = cell.translate(offset.x(), offset.y());

			if polygon.intersects(&cell) {
				cells.push(cell);
			}
		}
	}

	cells
}

const POINTS: [(f64, f64); 25] = [
	(126.99238300323485, 37.577371838120854),
	(126.99144959449767, 37.57644501294559),
	(126.9913637638092, 37.57567973575725),
	(126.99362754821776, 37.57200629577624),
	(126.99376702308655, 37.5704246199809),
	(126.99531197547913, 37.570450131147325),
	(126.99549436569214, 37.5665468210747),
	(127.00414180755614, 37.56657233356933),
	(127.00975298881532, 37.565390245474774),
	(127.01042890548705, 37.565934518582225),
	(127.01181292533875, 37.566274687254975),
	(127.01262831687927, 37.56696352405947),
	(127.01249957084656, 37.567813931081034),
	(127.01003193855284, 37.568740863675565),
	(127.00977444648743, 37.57062870906774),
	(127.01058983802794, 37.5715045850486),
	(127.00900197029114, 37.57306923106738),
	(127.0076823234558, 37.57300120366314),
	(127.00883030891418, 37.571249476602766),
	(126.99640631675719, 37.57127498748666),
	(126.99637413024901, 37.572014799318765),
	(126.9967818260193, 37.572771610715066),
	(126.99553728103638, 37.57636848558067),
	(126.99291944503784, 37.57729531170845),
	(126.99238300323485, 37.577371838120854),
];


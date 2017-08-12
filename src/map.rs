use serde_json;
use geo::Point;
use geo::Polygon;
use geo::distance::Distance;
use geo::contains::Contains;
use geo::translate::Translate;
use geo::boundingbox::BoundingBox;
use geo::haversine_destination::HaversineDestination;

use db::models::Parameter;
use db::models::Cell;

pub fn generate_cells(params: &Parameter) -> Vec<Cell> {
    generate_cell_polygons(params).into_iter().map(|p| {
        Cell {
            id: None,
            geom: p
        }
    }).collect()
}

pub fn generate_cell_polygons(params: &Parameter) -> Vec<Polygon<f64>> {
    let polygon = Polygon::new(POINTS.to_vec().into(), vec![]);
    let bbox = polygon.bbox().expect("Bounding couldn't calculated!");

    let tl = Point::new(bbox.xmin, bbox.ymax);
    let tr = Point::new(bbox.xmax, bbox.ymax);
    let bl = Point::new(bbox.xmin, bbox.ymin);

    let cell_tl = tl.clone();
    let cell_tr = cell_tl.haversine_destination(90.0, params.cell_size as f64);
    let cell_br = cell_tr.haversine_destination(180.0, params.cell_size as f64);
    let cell_bl = cell_tl.haversine_destination(180.0, params.cell_size as f64);

    let cell_width = cell_tl.distance(&cell_tr);
    let cell_height = cell_tl.distance(&cell_bl);

    let cell_tmp = Polygon::new(vec![cell_tl, cell_tr, cell_br, cell_bl].into(), vec![]);

    let horizontal_length = tl.distance(&tr);
    let vertical_length = tl.distance(&bl);

    let num_h_cells = f64::ceil(horizontal_length / cell_width);
    let num_v_cells = f64::ceil(vertical_length / cell_height);

    let mut cells = vec![];
    for row in 0..num_v_cells as i32 {
        for col in 0..num_h_cells as i32 {
            let cell = cell_tmp.clone();
            let target_point = cell_tl
                .haversine_destination(90.0, col as f64 * params.cell_size as f64)
                .haversine_destination(180.0, row as f64 * params.cell_size as f64);

            let offset = target_point - cell_tl;
            let cell = cell.translate(offset.x(), offset.y());

            // TODO: Use intersects after bug is fixed  if polygon.intersects(&cell) {
            // https://github.com/georust/rust-geo/issues/140
            let intersects = cell.exterior.clone().into_iter().any(|p| polygon.contains(&p));
            if intersects {
                cells.push(cell);
            }
        }
    }

    cells
}

pub fn geojson(params: &Parameter) {
    let cells = generate_cell_polygons(params).into_iter().map(|cell| {
        let mut points = cell.exterior.into_iter().map(|p| [p.x(), p.y()]).collect::<Vec<_>>();
        let first_point = points[0].clone();
        points.push(first_point);

        json!({
            "type": "Feature",
            "properties": {
                "stroke": "#555555",
                "stroke-width": 2,
                "stroke-opacity": 1,
                "fill": "#ff5353",
                "fill-opacity": 0.5
            },
            "geometry": {
                "type": "Polygon",
                "coordinates": [points]
            }
        })
    }).collect::<Vec<_>>();

    println!("Numbmer of cells: {}", cells.len());

    let json = serde_json::to_string_pretty(&cells).expect("JSON Serialization failed!");
    println!("{}", json);
}

pub const POINTS: [(f64, f64); 26] = [
    ( 126.99294090270998, 37.5772698028868 ),
    ( 126.99240982532503, 37.57736758665555 ),
    ( 126.99149787425996, 37.5764237553521 ),
    ( 126.99141204357149, 37.57567548419531 ),
    ( 126.99364364147186, 37.57203605817081 ),
    ( 126.99377775192261, 37.57054367201618 ),
    ( 126.99527978897093, 37.57054792387107 ),
    ( 126.99573040008544, 37.56655107315775 ),
    ( 127.00274169445039, 37.56686572662952 ),
    ( 127.00982272624971, 37.56547103626546 ),
    ( 127.01042890548705, 37.5659387707002 ),
    ( 127.01180756092073, 37.56629594774544 ),
    ( 127.01261758804321, 37.56694226375953 ),
    ( 127.01248347759248, 37.56781818309174 ),
    ( 127.009693980217,   37.568872674254486 ),
    ( 127.00952231884003, 37.570369345756944 ),
    ( 127.010595202446,   37.57149182964705 ),
    ( 127.00900733470918, 37.573056475933804 ),
    ( 127.00767695903781, 37.57298844851792 ),
    ( 127.00883567333221, 37.571253728417325 ),
    ( 126.99647605419162, 37.57070949417681 ),
    ( 126.99636876583098, 37.57202755463073 ),
    ( 126.99677646160124, 37.572771610715066 ),
    ( 126.99616760015486, 37.574859179812385 ),
    ( 126.99551582336427, 37.57638974318991 ),
    ( 126.99294090270998, 37.5772698028868 )
];

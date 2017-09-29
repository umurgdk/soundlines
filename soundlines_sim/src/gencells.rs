use std::error::Error;
use std::iter::FromIterator;

use geo::Point as GPoint;
use geo::Polygon as GPolygon;
use geo::distance::Distance;
use geo::translate::Translate;
use geo::intersects::Intersects;
use geo::boundingbox::BoundingBox;
use geo::haversine_destination::HaversineDestination;

use soundlines_core::postgis::ewkb::LineString;
use soundlines_core::postgis::ewkb::Polygon;
use soundlines_core::postgis::ewkb::Point;

use soundlines_core::db::init_connection;
use soundlines_core::db::models::Cell;
use soundlines_core::db::extensions::*;

pub fn run(cell_size: f64) -> Result<(), Box<Error>> {
    let conn = init_connection();
    let cells = generate_cells(cell_size);

    conn.insert_batch(&cells)?;
    Ok(())
}

fn generate_cells(cell_size: f64) -> Vec<Cell> {
    generate_cell_polygons(cell_size).into_iter().map(|p| {
        let points = p.exterior.clone().into_iter().map(|p| {
            Point::new(p.x(), p.y(), Some(4326))
        }).collect::<Vec<_>>();

        let mut strings = vec![LineString::from_iter(points)];
        strings[0].srid = Some(4326);
        let mut polygon = Polygon::from_iter(strings);
        polygon.srid = Some(4326);

        Cell {
            id: 0,
            geom: polygon,

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

const POINTS: [(f64, f64); 26] = [
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


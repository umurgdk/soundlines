use std::collections::HashMap;

use postgis::ewkb::Point;
use postgis::ewkb::Polygon;
use geo::Point as GPoint;

use postgres::rows::Row;
use postgres::types::ToSql;
use serde_json::Value;

use db::Result;
use db::Connection;
use db::extensions::*;
use db::models::Entity;
use db::models::Seed;

#[derive(Clone, Debug)]
pub struct Cell {
    pub id: i32,
    pub geom: Polygon,
  	pub wifi: f32,
  	pub light: f32,
  	pub sound: f32,
  	pub sns: i32,
  	pub visit: i32
}

pub struct CellNeighbours {
    pub cells: HashMap<i32, Cell>,
    pub seeds: Vec<Seed>,
    pub entities: Vec<Entity>,
    pub current_cell_id: i32
}

const NEIGHBOR_WITH_ENTITIES_QUERY: &'static str = r#"
select
       id        as c_id,
       geom      as c_geom,
       wifi      as c_wifi,
       light     as c_light,
       sound     as c_sound,
       sns       as c_sns,
       visit     as c_visit,
       st_contains(geom, st_setsrid(st_point($1, $2),4326)) as c_current
from cells
where st_dwithin(geom::geography, ST_SetSRID(ST_Point($1, $2), 4326)::geography, $3);
"#;

impl Cell {
    pub fn find_containing(conn: &Connection, point: &GPoint<f64>) -> Result<Option<Cell>> {
        let point = Point::new(point.x(), point.y(), Some(4326));
        conn.query("select * from cells where ST_Contains(geom, $1) LIMIT 1", &[&point])
            .map(|rows| rows.try_get(0).map(Cell::from_sql_row))
    }

    pub fn find_containing_core(conn: &Connection, point: &Point) -> Result<Option<Cell>> {
        conn.query("select * from cells where ST_Contains(geom, $1) LIMIT 1", &[&point])
            .map(|rows| rows.try_get(0).map(Cell::from_sql_row))
    }

    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id as i64,
            "points": self.geom.rings[0].points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()
        })
    }

    pub fn into_json(self) -> Value {
        self.to_json()
    }

    pub fn find_neighbors(conn: &Connection, location: &Point, within: f64) -> Result<CellNeighbours> {
        let mut cells = HashMap::new();
        let mut entities = vec![];
        let mut seeds = vec![];
        let mut current_cell_id = -1;

        let cell_rows = conn.query(NEIGHBOR_WITH_ENTITIES_QUERY, &[&location.x, &location.y, &within])?;

        for row in cell_rows.into_iter() {
            let cell_id: i32 = row.get::<_, i32>("c_id");
            if !cells.contains_key(&cell_id) {
                cells.insert(cell_id, Cell {
                    id: cell_id,
                    geom: row.get("c_geom"),
                    wifi: row.get("c_wifi"),
                    light: row.get("c_light"),
                    sound: row.get("c_sound"),
                    sns: row.get("c_sns"),
                    visit: row.get("c_visit"),
                });
            }

            if row.get("c_current") {
                current_cell_id = cell_id;
            }

            let mut cell_entities = conn.filter::<Entity>(&["cell_id"], &[&cell_id])?;
            entities.append(&mut cell_entities);

            let mut cell_seeds = conn.filter::<Seed>(&["cell_id"], &[&cell_id])?;
            seeds.append(&mut cell_seeds);
        }

        Ok(CellNeighbours {
            cells,
            entities,
            seeds,
            current_cell_id
        })
    }
}

impl SqlType for Cell {
    fn table_name() -> &'static str { "cells" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: row.get("id"),
            geom: row.get("geom"),
  	        wifi: row.get("wifi"),
  	        light: row.get("light"),
  	        sound: row.get("sound"),
  	        sns: row.get("sns"),
  	        visit: row.get("visit")
        }
    }

    fn insert_fields() -> Vec<&'static str> { 
        vec![
            "geom",
  	        "wifi",
  	        "light",
  	        "sound",
  	        "sns",
  	        "visit"
        ] 
    }
    
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![
            &self.geom,
  	        &self.wifi,
  	        &self.light,
  	        &self.sound,
  	        &self.sns,
  	        &self.visit
        ]
    }
}

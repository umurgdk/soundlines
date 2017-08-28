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
    pub entities: Vec<Entity>,
    pub current_cell_id: i32
}

const NEIGHBOR_WITH_ENTITIES_QUERY: &'static str = r#"
select
       cell.id        as c_id,
       cell.geom      as c_geom,
       cell.wifi      as c_wifi,
       cell.light     as c_light,
       cell.sound     as c_sound,
       cell.sns       as c_sns,
       cell.visit     as c_visit,
       st_contains(cell.geom, st_setsrid(st_point($1, $2),4326)) as c_current,

       entity.id              as e_id,
       entity.cell_id         as e_cell_id,
       entity.point           as e_point,
       entity.prefab          as e_prefab,
       entity.setting_id      as e_setting_id,
       entity.dna_id          as e_dna_id,
       entity.fitness         as e_fitness,
       entity.age             as e_age,
       entity.size            as e_size,
       entity.life_expectancy as e_life_expectancy,
       entity.nickname        as e_nickname,
       entity.mating          as e_mating,
       entity.start_mating_at as e_start_mating_at,
       entity.last_seed_at    as e_laast_seed_at

from
    entities entity

join
    cells cell on entity.cell_id = cell.id

where
    st_dwithin(cell.geom::geography, ST_SetSRID(ST_Point($1, $2), 4326)::geography, $3);
"#;

impl Cell {
    pub fn find_containing(conn: &Connection, point: &GPoint<f64>) -> Result<Option<Cell>> {
        let point = Point::new(point.x(), point.y(), Some(4326));
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
        let mut current_cell_id = -1;

        let rows = conn.query(NEIGHBOR_WITH_ENTITIES_QUERY, &[&location.x, &location.y, &within])?;

        for row in rows.into_iter() {
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

            entities.push(Entity {
                id: row.get("e_id"),
                point: row.get("e_point"),
                prefab: row.get("e_prefab"),
                cell_id: row.get("e_cell_id"),
                setting_id: row.get("e_setting_id"),
                dna_id: row.get("e_dna_id"),
                fitness: row.get("e_fitness"),
                age: row.get("e_age"),
                size: row.get("e_size"),
                life_expectancy: row.get("e_life_expectancy"),
                nickname: row.get("e_nickname"),
                mating: row.get("e_mating"),
                start_mating_at: row.get("e_start_mating_at"),
                last_seed_at: row.get("e_laast_seed_at")
            });
        }

        Ok(CellNeighbours {
            cells,
            entities,
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

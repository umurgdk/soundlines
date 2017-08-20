use postgis::ewkb::Point;
use postgis::ewkb::Polygon;
use postgres::rows::Row;
use postgres::types::ToSql;
use serde_json::Value;

use std::collections::HashMap;

use db::Result;
use db::Connection;
use db::extensions::*;
use db::models::Entity;

#[derive(Clone)]
pub struct Cell {
    pub id: i32,
    pub geom: Polygon
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
       st_contains(cell.geom, st_setsrid(st_point($1, $2),4326)) as c_current,

       entity.id      as e_id,
       entity.cell_id as e_cell_id,
       entity.point   as e_point,
       entity.prefab  as e_prefab

from
    entities entity

join
    cells cell on entity.cell_id = cell.id

where
    st_dwithin(cell.geom::geography, ST_SetSRID(ST_Point($1, $2), 4326)::geography, $3);
"#;

impl Cell {
    #[allow(unused)]
    pub fn find_containing(conn: &Connection, point: &Point) -> Result<Option<Cell>> {
        conn.query("select id, geom from cells where ST_Contains(geom, $1) LIMIT 1", &[&point])
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
                    geom: row.get("c_geom")
                });
            }

            if row.get("c_current") {
                current_cell_id = cell_id;
            }

            entities.push(Entity {
                id: row.get("e_id"),
                point: row.get("e_point"),
                prefab: row.get("e_prefab"),
                cell_id: row.get("e_cell_id")
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
            geom: row.get("geom")
        }
    }

    fn insert_fields() -> Vec<&'static str> { vec!["geom"] }
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![&self.geom]
    }
}

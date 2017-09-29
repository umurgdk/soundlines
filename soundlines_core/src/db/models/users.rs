use postgres::rows::Row;
use postgres::types::ToSql;
use chrono::prelude::*;

use db::Result;
use db::Connection;
use db::extensions::*;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime<Utc>
}

impl SqlType for User {
    fn table_name() -> &'static str { "users" }

    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self { id: row.get("id"), created_at: row.get("created_at") }
    }

    fn insert_fields() -> Vec<&'static str> { vec!["created_at"] }
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> { vec![&self.created_at] }
}

#[derive(Serialize, Deserialize)]
pub struct UserLocation {
    pub id: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub cell_id: i32
}

impl User {
    pub fn get_all_locations(conn: &Connection, except: Option<i32>) -> Result<Vec<UserLocation>> {
        use chrono::Duration;
        use postgis::ewkb::Point;

        let except_id = except.unwrap_or(-1);

        let three_seconds_before = Utc::now() - Duration::seconds(30) - Duration::milliseconds(10);

		let query = r#"
		    select gps_readings.user_id, gps_readings.point, cells.id as cell_id from gps_readings
		    inner join cells on st_contains(cells.geom, gps_readings.point)
		    where created_at >= $1 and user_id != $2"#;

        println!(r#"select gps_readings.user_id, gps_readings.point, cells.id as cell_id from gps_readings
		    inner join cells on st_contains(cells.geom, gps_readings.point)
		    where created_at >= {} and user_id != {}"#, three_seconds_before, except_id);

        conn.query(query, &[&three_seconds_before, &except_id])
            .map(|rows| {
                rows.into_iter().map(|row| {
                    let point: Point = row.get("point");
                    UserLocation {
                        id: row.get("user_id"),
                        latitude: point.y,
                        longitude: point.x,
                        cell_id: row.get("cell_id")
                    }
                }).collect()
            })
    }
}

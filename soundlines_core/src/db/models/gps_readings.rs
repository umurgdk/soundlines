use chrono::prelude::*;
use postgis::ewkb::Point;
use postgres::rows::Row;
use postgres::types::ToSql;

use db::Result;
use db::Connection;
use db::extensions::*;
use db::models::default_user_id;

#[derive(Clone)]
pub struct GpsReading {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub point: Point
}

impl GpsReading {
    pub fn get_time_range(conn: &Connection, since: &DateTime<Utc>, until: &DateTime<Utc>) -> Result<Vec<GpsReading>> {
        conn.query("select * from gps_readings where created_at >= $1 and created_at < $2", &[&since, &until])
            .map(|rows| rows.into_iter().map(GpsReading::from_sql_row).collect::<Vec<_>>())
    }

    pub fn get_time_range_count(conn: &Connection, since: &DateTime<Utc>, until: &DateTime<Utc>) -> Result<i32> {
        conn.query("select COUNT(*) from gps_readings where created_at >= $1 and created_at < $2", &[&since, &until])
            .map(|rows| rows.try_get(0).map(|r| r.get::<_, i32>("count")).unwrap_or(0))
    }

    pub fn last_gps_readings_by_user(conn: &Connection) -> Result<Vec<GpsReading>> {
        conn.query(r#"
            select *
            from gps_readings
            where id in (select distinct max(id) from gps_readings group by user_id)
            order by created_at desc;
        "#, &[])
            .map(|rows| rows.into_iter().map(GpsReading::from_sql_row).collect::<Vec<_>>())
    }
}

impl SqlType for GpsReading {
    fn table_name() -> &'static str { "gps_readings" }
    fn from_sql_row<'a>(row: Row<'a>) -> Self {
        Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            point: row.get("point")
        }
    }

    fn insert_fields() -> Vec<&'static str> {
        vec!["user_id", "created_at", "point"]
    }

    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![ &self.user_id, &self.created_at, &self.point ]
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GpsReadingJson {
    pub id: Option<i32>,
    #[serde(default="default_user_id")]
    pub user_id: i32,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64
}

impl GpsReadingJson {
    pub fn into_gps_reading(self, id: i32) -> GpsReading {
        GpsReading {
            id: self.id.unwrap_or(id),
            user_id: self.user_id,
            created_at: self.created_at,
            point: Point::new(self.longitude, self.latitude, Some(4326))
        }
    }
}

impl From<GpsReading> for GpsReadingJson {
    fn from(reading: GpsReading)-> Self {
        Self {
            id: Some(reading.id),
            user_id: reading.user_id,
            created_at: reading.created_at,
            latitude: reading.point.y,
            longitude: reading.point.x
        }
    }
}

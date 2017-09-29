use postgres::rows::Row;
use postgres::types::ToSql;

use db::Result as DbResult;
use db::Connection;
use db::extensions::*;

#[derive(Debug, Clone, Serialize)]
pub struct Weather {
	#[serde(skip)]
	pub id: i32,
	pub temperature: f64,
	pub precip: Option<String>
}

impl Weather {
	pub fn get(conn: &Connection) -> DbResult<Option<Weather>> {
		conn.first::<Weather>()
	}
}

impl SqlType for Weather {
	fn table_name() -> &'static str { "weather" }
	fn from_sql_row<'a>(row: Row<'a>) -> Self {
		Self {
			id: row.get("id"),
			temperature: row.get("temperature"),
			precip: row.get("precip")
		}
	}

	fn insert_fields() -> Vec<&'static str> {
		vec![
			"temperature",
			"precip"
		]
	}

	fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql> {
		vec![
			&self.temperature,
			&self.precip
		]
	}
}
use db::Connection;
use models::PlantSetting;

/// Get all plant settings
pub fn all(conn: &Connection) -> ::errors::Result<::postgres::rows::Rows> {
	let query = r#"
		select
			(
				data ||
				jsonb_build_object(
					'id', id
				)
			)::text
		from settings_json;
	"#;

	conn.query(&query, &[]).map_err(|e| e.into())
}

pub fn all_vec(conn: &Connection) -> ::errors::Result<Vec<PlantSetting>> {
	use fallible_iterator;
	use fallible_iterator::FallibleIterator;

	let rows = all(conn)?;
	let settings_iterator = rows.into_iter().map(|r| ::serde_json::from_str::<PlantSetting>(&r.get::<_, String>(0)));
	let settings = fallible_iterator::convert(settings_iterator).collect()?;
	Ok(settings)
}
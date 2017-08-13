use super::Result;
use super::Connection;
use super::models::Parameter;

impl Parameter {
    pub fn fetch(conn: &Connection) -> Result<Parameter> {
        let rows = conn.query("SELECT * FROM parameters where id=0", &[])?;
        let row = rows.get(0);

        Ok(Parameter{
            id: Some(row.get("id")),
            cell_size: row.get("cell_size")
        })
    }
}

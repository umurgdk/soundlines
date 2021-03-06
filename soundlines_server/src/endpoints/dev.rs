use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::path::Path;

use rocket::response::NamedFile;
use rocket_contrib::Json;

use db_guard::*;

use soundlines_core::db::Result as DbResult;
use soundlines_core::db::models::*;
use soundlines_core::db::extensions::*;

#[get("/version")]
pub fn get_version() -> Result<String, io::Error> {
    let mut content = String::new();
    let mut file = match File::open("resources/version") {
        Ok(file) => file,
        _        => return Ok(content)
    };

    file.read_to_string(&mut content)?;
    Ok(content)
}

#[put("/version", data = "<content>")]
pub fn update_version(content: String) -> Result<String, io::Error> {
    let mut file = File::create("resources/version")?;
    file.write_all(content.as_bytes())?;

    Ok(content)
}

#[get("/settings")]
pub fn get_settings(conn: DbConn) -> DbResult<Json<Vec<PlantSetting>>> {
    let settings = conn.all::<PlantSetting>()?;
    Ok(Json(settings))
}

#[put("/settings/<setting_id>", data="<setting>")]
pub fn update_setting(conn: DbConn, setting_id: i32, setting: Json<PlantSetting>) -> DbResult<()> {
    let setting = setting.into_inner();
    let _ = conn.update(setting_id, &setting)?;

    Ok(())
}

#[get("/snapshot/<time>")]
pub fn get_snapshot(time: String) -> io::Result<NamedFile> {
    let path = Path::new("snapshots").join(time).with_extension("json");
    NamedFile::open(&path)
}
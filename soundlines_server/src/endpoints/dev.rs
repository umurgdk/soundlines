use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

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

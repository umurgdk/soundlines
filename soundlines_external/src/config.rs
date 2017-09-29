use std::fs;
use std::path::Path;
use std::io::Write;
use serde_json;

use errors::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub last_tweet_id: Option<u64>,
    pub minutes: i32,
    #[serde(default, skip)]
    path: String,
}

fn default<P: AsRef<Path>>(path: P) -> Config {
    Config {
        last_tweet_id: None,
        path: path.as_ref().to_str().unwrap().to_string(),
        minutes: 1
    }
}

pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path_string = path.as_ref().to_str().ok_or(Error::from("Failed to convert path to str"))?.to_string();
    let file = fs::File::open(&path)?;
    let mut config: Config = serde_json::from_reader(file)?;
    config.path = path_string;

    Ok(config)
}

pub fn create<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config = default(path);
    save(&config)?;

    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(&config.path)?;
    file.write(json.as_bytes())?;

    Ok(())
}

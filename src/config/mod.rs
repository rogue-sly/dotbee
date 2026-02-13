pub mod conflict;
pub mod icons;

use self::icons::IconStyle;
pub use conflict::ConflictAction;
use indexmap::IndexMap;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Config {
    pub settings: Settings,
    pub global: Option<Global>,
    pub profiles: Option<IndexMap<String, Profile>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Settings {
    pub on_conflict: Option<ConflictAction>,
    pub icon_style: Option<IconStyle>,
    pub auto_detect_profile: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Global {
    pub links: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    pub links: IndexMap<String, String>,
}

impl Config {
    pub fn load(path: Option<String>) -> Result<Config, Box<dyn Error>> {
        let path_str = path.unwrap_or_else(|| "dotsy.toml".to_string());
        let config_path = Path::new(&path_str);

        if !config_path.exists() {
            // If no config, return a default empty config.
            return Ok(Config::default());
        }

        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

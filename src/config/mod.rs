use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub variables: Option<HashMap<String, String>>,
    pub hooks: Option<Hooks>,
    pub global: Option<Global>,
    pub profiles: Option<HashMap<String, Profile>>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub atomic_links: bool,
    pub auto_detect_profile: bool,
    pub on_conflict: String,
}

#[derive(Debug, Deserialize)]
pub struct Hooks {
    pub pre: Option<HashMap<String, String>>,
    pub post: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Global {
    pub links: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub links: HashMap<String, String>,
}

impl Config {
    pub fn load_from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

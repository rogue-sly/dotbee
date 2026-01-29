pub mod hooks;
pub mod icons;

use hooks::Hooks;
use indexmap::IndexMap;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub hooks: Option<Hooks>,
    pub global: Option<Global>,
    pub profiles: Option<IndexMap<String, Profile>>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub on_conflict: String,
    pub icon_style: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Global {
    pub links: IndexMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub links: IndexMap<String, String>,
}

impl Config {
    /// my stupid brain be like
    /// "should I rename `path` parameter to `path_string`?"
    /// wait a minute, I could just do lsp hover!
    /// wait no that's clear enough...
    /// hmm no...
    /// man screw this shit
    pub fn load(path: Option<String>) -> Result<Config, Box<dyn Error>> {
        let path = path.unwrap_or("dotsy.toml".to_string());

        let config_path = Path::new(&path);
        if !config_path.exists() {
            return Err(format!("dotsy.toml not found. Run 'dotsy init' first.").into());
        }

        let content = fs::read_to_string(config_path).unwrap();
        let config = toml::from_str(&content).unwrap();
        Ok(config)
    }
}

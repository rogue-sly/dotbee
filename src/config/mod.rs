use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub hooks: Option<Hooks>,
    pub global: Option<Global>,
    pub profiles: Option<HashMap<String, Profile>>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub on_conflict: String,
    pub icon_style: Option<String>,
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

pub struct Icons {
    pub check: String,
    pub cross: String,
    pub link: String,
    pub unlink: String,
    pub warning: String,
    pub info: String,
    pub error: String,
}

impl Icons {
    pub fn new(style: &str) -> Self {
        match style.to_lowercase().as_str() {
            "emoji" => Self {
                check: "✅ ".to_string(),
                cross: "❌ ".to_string(),
                link: "🔗 ".to_string(),
                unlink: "💔 ".to_string(),
                warning: "⚠️ ".to_string(),
                info: "ℹ️ ".to_string(),
                error: "🚫 ".to_string(),
            },
            "nerdfont" => Self {
                check: " ".to_string(),
                cross: " ".to_string(),
                link: " ".to_string(),
                unlink: " ".to_string(),
                warning: " ".to_string(),
                info: " ".to_string(),
                error: " ".to_string(),
            },
            // default
            "text" | _ => Self {
                check: "DONE ".to_string(),
                cross: "FAIL ".to_string(),
                link: "LINK ".to_string(),
                unlink: "MISS ".to_string(),
                warning: "WARN ".to_string(),
                info: "INFO ".to_string(),
                error: "ERROR ".to_string(),
            },
        }
    }
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct State {
    pub active_profile: Option<String>,
}

impl State {
    fn get_path() -> PathBuf {
        // Use XDG_STATE_HOME (~/.local/state) or fallback
        let mut path =
            dirs::state_dir().unwrap_or_else(|| dirs::home_dir().expect("Could not determine home directory").join(".local/state"));
        path.push("dotsy");
        path.push("state.toml");
        path
    }

    pub fn load() -> io::Result<Self> {
        let path = Self::get_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        // If parsing fails (e.g. empty file or corrupt), return default (empty state)
        // or we could error out. For resilience, default might be better but logging error is good.
        let state: State = toml::from_str(&content).unwrap_or_default();
        Ok(state)
    }

    pub fn save(&self) -> io::Result<()> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }

    pub fn set_active_profile(&mut self, profile: String) -> io::Result<()> {
        self.active_profile = Some(profile);
        self.save()
    }

    pub fn clear_active_profile(&mut self) -> io::Result<()> {
        self.active_profile = None;
        self.save()
    }
}

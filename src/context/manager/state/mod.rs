use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Link {
    pub source: String,
    pub target: String,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct State {
    active_profile: Option<String>,
    dotfiles_path: Option<PathBuf>,
    links: Vec<Link>,
}

impl State {
    fn get_path() -> PathBuf {
        let mut path = cfg_select! {
            any(target_os = "linux") => dirs::state_dir().expect("Couldn't determine state directory"),
            _ => dirs::data_dir().expect("Couldn't determine data directory"),
        };

        path.push("dotbee");
        path.push("state.json");
        path
    }

    fn load() -> io::Result<Self> {
        let path = Self::get_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        let state: State = serde_json::from_str(&content).unwrap_or_default();
        Ok(state)
    }

    fn save(&self) -> io::Result<()> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }
}

pub struct StateManager {
    state: State,
}

impl StateManager {
    pub fn load() -> anyhow::Result<Self, anyhow::Error> {
        let state = State::load()?;
        Ok(Self { state })
    }

    pub fn get_active_profile(&self) -> Option<&str> {
        self.state.active_profile.as_deref()
    }

    pub fn set_active_profile(&mut self, profile: String) -> anyhow::Result<(), anyhow::Error> {
        self.state.active_profile = Some(profile);
        self.state.save()?;
        Ok(())
    }

    pub fn get_dotfiles_path(&self) -> Option<&Path> {
        self.state.dotfiles_path.as_deref()
    }

    pub fn set_dotfiles_path(&mut self, path: Option<PathBuf>) -> anyhow::Result<(), anyhow::Error> {
        self.state.dotfiles_path = path;
        self.state.save()?;
        Ok(())
    }

    pub fn get_links(&self) -> &[Link] {
        &self.state.links
    }

    pub fn add_link(&mut self, source: String, target: String, is_dir: bool) -> anyhow::Result<(), anyhow::Error> {
        let link = Link { source, target, is_dir };
        if !self.state.links.contains(&link) {
            self.state.links.push(link);
        }
        self.state.save()?;
        Ok(())
    }

    pub fn remove_links<F>(&mut self, predicate: F) -> anyhow::Result<usize, anyhow::Error>
    where
        F: Fn(&Link) -> bool,
    {
        let before = self.state.links.len();
        self.state.links.retain(|l| !predicate(l));
        let removed = before - self.state.links.len();
        if removed > 0 {
            self.state.save()?;
        }
        Ok(removed)
    }

    pub fn clear(&mut self) -> anyhow::Result<(), anyhow::Error> {
        self.state = State::default();
        self.state.save()?;
        Ok(())
    }
}

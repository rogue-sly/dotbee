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
pub struct State {
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

    fn save(&self) -> io::Result<()> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }

    pub fn load() -> io::Result<Self> {
        let path = Self::get_path();
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(e),
        };
        let state: State = serde_json::from_str(&content).unwrap_or_default();
        Ok(state)
    }

    pub fn get_active_profile(&self) -> Option<&str> {
        self.active_profile.as_deref()
    }

    pub fn set_active_profile(&mut self, profile: String) -> anyhow::Result<(), anyhow::Error> {
        self.active_profile = Some(profile);
        self.save()?;
        Ok(())
    }

    pub fn get_dotfiles_path(&self) -> Option<&Path> {
        self.dotfiles_path.as_deref()
    }

    pub fn set_dotfiles_path(&mut self, path: Option<PathBuf>) -> anyhow::Result<(), anyhow::Error> {
        self.dotfiles_path = path;
        self.save()?;
        Ok(())
    }

    pub fn get_links(&self) -> &[Link] {
        &self.links
    }

    pub fn add_link(&mut self, source: String, target: String, is_dir: bool) -> anyhow::Result<(), anyhow::Error> {
        let link = Link { source, target, is_dir };
        if !self.links.contains(&link) {
            self.links.push(link);
        }
        self.save()?;
        Ok(())
    }

    pub fn remove_links<F>(&mut self, predicate: F) -> anyhow::Result<usize, anyhow::Error>
    where
        F: Fn(&Link) -> bool,
    {
        let before = self.links.len();
        self.links.retain(|l| !predicate(l));
        let removed = before - self.links.len();
        if removed > 0 {
            self.save()?;
        }
        Ok(removed)
    }

    pub fn clear(&mut self) -> anyhow::Result<(), anyhow::Error> {
        *self = State::default();
        self.save()?;
        Ok(())
    }
}

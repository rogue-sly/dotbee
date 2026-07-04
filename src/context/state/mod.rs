use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
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
    fn get_path() -> Result<PathBuf> {
        let base = cfg_select! {
            any(target_os = "linux") => dirs::state_dir(),
            _ => dirs::data_dir()
        };

        let mut path = base.context("Couldn't determine state directory")?;
        path.push("dotbee");
        path.push("state.json");
        Ok(path)
    }

    fn save(&self) -> Result<()> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("Failed to create state directory {:?}", parent))?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize state to json")?;
        fs::write(&path, &content).with_context(|| format!("Failed to write state file at {:?}", path))?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path().context("Failed to determine state file path")?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).with_context(|| format!("Failed to read state file at {:?}", path))?;
        if content.trim().is_empty() {
            return Ok(Self::default());
        }

        let state: State = serde_json::from_str(&content).with_context(|| format!("Failed to parse state file at {:?}", path))?;
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

    pub fn get_dotfiles_root(&self) -> Result<PathBuf> {
        match self.get_dotfiles_path() {
            Some(p) if p.exists() => Ok(p.to_path_buf()),
            Some(p) => bail!("Stored dotfiles path {:?} no longer exists. Run 'dotbee init' to set a new one.", p),
            None => std::env::current_dir().context("Failed to get current directory"),
        }
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

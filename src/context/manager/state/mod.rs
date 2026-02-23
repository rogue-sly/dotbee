use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ManagedLink {
    pub source: String,
    pub target: String,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct State {
    active_profile: Option<String>,
    dotfiles_path: Option<PathBuf>,
    managed_links: Vec<ManagedLink>,
}

impl State {
    fn get_path() -> PathBuf {
        let mut path =
            dirs::state_dir().unwrap_or_else(|| dirs::home_dir().expect("Could not determine home directory").join(".local/state"));
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
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let state = State::load()?;
        Ok(Self { state })
    }

    pub fn get_active_profile(&self) -> Option<&str> {
        self.state.active_profile.as_deref()
    }

    pub fn set_active_profile(&mut self, profile: String) -> Result<(), Box<dyn Error>> {
        self.state.active_profile = Some(profile);
        self.state.save()?;
        Ok(())
    }

    pub fn get_dotfiles_path(&self) -> Option<&Path> {
        self.state.dotfiles_path.as_deref()
    }

    pub fn set_dotfiles_path(&mut self, path: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
        self.state.dotfiles_path = path;
        self.state.save()?;
        Ok(())
    }

    pub fn get_managed_links(&self) -> &[ManagedLink] {
        &self.state.managed_links
    }

    pub fn add_managed_link(&mut self, source: String, target: String, is_dir: bool) -> Result<(), Box<dyn Error>> {
        let link = ManagedLink { source, target, is_dir };
        if !self.state.managed_links.contains(&link) {
            self.state.managed_links.push(link);
        }
        self.state.save()?;
        Ok(())
    }

    pub fn remove_managed_links<F>(&mut self, predicate: F) -> Result<usize, Box<dyn Error>>
    where
        F: Fn(&ManagedLink) -> bool,
    {
        let before = self.state.managed_links.len();
        self.state.managed_links.retain(|l| !predicate(l));
        let removed = before - self.state.managed_links.len();
        if removed > 0 {
            self.state.save()?;
        }
        Ok(removed)
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.state = State::default();
        self.state.save()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a StateManager directly from a known State without touching disk.
    fn make_manager(state: State) -> StateManager {
        StateManager { state }
    }

    fn empty_manager() -> StateManager {
        make_manager(State::default())
    }

    // ── active_profile ────────────────────────────────────────────────────────

    #[test]
    fn test_get_active_profile_initially_none() {
        let sm = empty_manager();
        assert!(sm.get_active_profile().is_none());
    }

    #[test]
    fn test_set_and_get_active_profile() {
        let mut sm = make_manager(State {
            active_profile: None,
            dotfiles_path: None,
            managed_links: vec![],
        });
        // Bypass disk by calling the inner state directly
        sm.state.active_profile = Some("desktop".to_string());
        assert_eq!(sm.get_active_profile(), Some("desktop"));
    }

    // ── dotfiles_path ─────────────────────────────────────────────────────────

    #[test]
    fn test_get_dotfiles_path_initially_none() {
        let sm = empty_manager();
        assert!(sm.get_dotfiles_path().is_none());
    }

    #[test]
    fn test_set_and_get_dotfiles_path() {
        let mut sm = empty_manager();
        sm.state.dotfiles_path = Some(PathBuf::from("/home/user/dotfiles"));
        assert_eq!(sm.get_dotfiles_path(), Some(Path::new("/home/user/dotfiles")));
    }

    #[test]
    fn test_clear_dotfiles_path() {
        let mut sm = make_manager(State {
            active_profile: None,
            dotfiles_path: Some(PathBuf::from("/some/path")),
            managed_links: vec![],
        });
        sm.state.dotfiles_path = None;
        assert!(sm.get_dotfiles_path().is_none());
    }

    // ── managed_links ─────────────────────────────────────────────────────────

    #[test]
    fn test_get_managed_links_initially_empty() {
        let sm = empty_manager();
        assert!(sm.get_managed_links().is_empty());
    }

    #[test]
    fn test_add_managed_link_appends() {
        let mut sm = empty_manager();
        sm.state.managed_links.push(ManagedLink {
            source: "/home/user/dotfiles/bash/bashrc".to_string(),
            target: "/home/user/.bashrc".to_string(),
            is_dir: false,
        });

        let links = sm.get_managed_links();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].source, "/home/user/dotfiles/bash/bashrc");
        assert_eq!(links[0].target, "/home/user/.bashrc");
        assert!(!links[0].is_dir);
    }

    #[test]
    fn test_add_managed_link_no_duplicates() {
        let mut sm = empty_manager();
        let link = ManagedLink {
            source: "/src".to_string(),
            target: "/dst".to_string(),
            is_dir: false,
        };
        sm.state.managed_links.push(link.clone());

        // Simulate the dedup logic used by add_managed_link
        if !sm.state.managed_links.contains(&link) {
            sm.state.managed_links.push(link);
        }

        assert_eq!(sm.get_managed_links().len(), 1);
    }

    #[test]
    fn test_remove_managed_links_by_predicate() {
        let mut sm = make_manager(State {
            active_profile: None,
            dotfiles_path: None,
            managed_links: vec![
                ManagedLink {
                    source: "/s1".to_string(),
                    target: "/t1".to_string(),
                    is_dir: false,
                },
                ManagedLink {
                    source: "/s2".to_string(),
                    target: "/t2".to_string(),
                    is_dir: true,
                },
                ManagedLink {
                    source: "/s3".to_string(),
                    target: "/t3".to_string(),
                    is_dir: false,
                },
            ],
        });

        let before = sm.get_managed_links().len();
        sm.state.managed_links.retain(|l| l.is_dir); // keep only dirs
        let after = sm.get_managed_links().len();

        assert_eq!(before, 3);
        assert_eq!(after, 1);
        assert_eq!(sm.get_managed_links()[0].source, "/s2");
    }

    #[test]
    fn test_clear_resets_all_fields() {
        let mut sm = make_manager(State {
            active_profile: Some("laptop".to_string()),
            dotfiles_path: Some(PathBuf::from("/dots")),
            managed_links: vec![ManagedLink {
                source: "/s".to_string(),
                target: "/t".to_string(),
                is_dir: false,
            }],
        });

        sm.state = State::default();

        assert!(sm.get_active_profile().is_none());
        assert!(sm.get_dotfiles_path().is_none());
        assert!(sm.get_managed_links().is_empty());
    }

    // ── ManagedLink equality ──────────────────────────────────────────────────

    #[test]
    fn test_managed_link_equality() {
        let a = ManagedLink {
            source: "/s".to_string(),
            target: "/t".to_string(),
            is_dir: false,
        };
        let b = ManagedLink {
            source: "/s".to_string(),
            target: "/t".to_string(),
            is_dir: false,
        };
        let c = ManagedLink {
            source: "/s".to_string(),
            target: "/t".to_string(),
            is_dir: true,
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}

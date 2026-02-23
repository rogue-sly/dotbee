pub mod conflict;
pub mod icons;

use self::icons::IconStyle;
pub use conflict::ConflictAction;
use indexmap::IndexMap;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
struct Config {
    settings: Settings,
    global: Option<Global>,
    profiles: Option<IndexMap<String, Profile>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Settings {
    #[serde(default, deserialize_with = "conflict::deserialize_conflict_action")]
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

pub struct ConfigManager {
    config: Config,
    config_path: Option<PathBuf>,
}

impl ConfigManager {
    pub fn load(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        let path_str = path.unwrap_or_else(|| "dotbee.toml".to_string());
        let config_path = Path::new(&path_str);

        if !config_path.exists() {
            return Ok(Self {
                config: Config::default(),
                config_path: None,
            });
        }

        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        let config_path = Some(fs::canonicalize(config_path)?);

        Ok(Self { config, config_path })
    }

    pub fn get_profile(&self, name: &str) -> Result<&Profile, Box<dyn Error>> {
        let profiles = self.config.profiles.as_ref().ok_or("No profiles defined in configuration.")?;
        profiles
            .get(name)
            .ok_or_else(|| format!("Profile '{}' not found in configuration.", name).into())
    }

    pub fn list_profiles(&self) -> Vec<&str> {
        self.config
            .profiles
            .as_ref()
            .map(|p| p.keys().map(|k| k.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn has_profiles(&self) -> bool {
        self.config.profiles.as_ref().map(|p| !p.is_empty()).unwrap_or(false)
    }

    pub fn get_global_links(&self) -> Option<&IndexMap<String, String>> {
        self.config.global.as_ref().map(|g| &g.links)
    }

    pub fn get_settings(&self) -> &Settings {
        &self.config.settings
    }

    pub fn get_config_path(&self) -> Option<&Path> {
        self.config_path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_config(dir: &std::path::Path, content: &str) -> PathBuf {
        let path = dir.join("dotbee.toml");
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_load_missing_file_returns_default() {
        let result = ConfigManager::load(Some("/nonexistent/dotbee.toml".to_string()));
        assert!(result.is_ok());
        let cm = result.unwrap();
        assert!(!cm.has_profiles());
        assert!(cm.list_profiles().is_empty());
        assert!(cm.get_global_links().is_none());
        assert!(cm.get_config_path().is_none());
    }

    #[test]
    fn test_load_valid_config() {
        let dir = tempdir().unwrap();
        let toml = r#"
[settings]
icon_style = "text"

[global.links]
"~/.bashrc" = "bash/bashrc"

[profiles.desktop.links]
"~/.config/i3/config" = "linux/i3_config"

[profiles.server.links]
"~/.tmux.conf" = "server/tmux.conf"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        assert!(cm.has_profiles());
        assert_eq!(cm.list_profiles().len(), 2);
        assert!(cm.list_profiles().contains(&"desktop"));
        assert!(cm.list_profiles().contains(&"server"));
    }

    #[test]
    fn test_get_profile_found() {
        let dir = tempdir().unwrap();
        let toml = r#"
[profiles.laptop.links]
"~/.vimrc" = "vim/vimrc"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        let profile = cm.get_profile("laptop");
        assert!(profile.is_ok());
        assert!(profile.unwrap().links.contains_key("~/.vimrc"));
    }

    #[test]
    fn test_get_profile_not_found() {
        let dir = tempdir().unwrap();
        let toml = r#"
[profiles.desktop.links]
"~/.vimrc" = "vim/vimrc"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        let result = cm.get_profile("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_get_profile_no_profiles_section() {
        let dir = tempdir().unwrap();
        let toml = r#"
[settings]
icon_style = "text"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        let result = cm.get_profile("anything");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No profiles"));
    }

    #[test]
    fn test_get_global_links() {
        let dir = tempdir().unwrap();
        let toml = r#"
[global.links]
"~/.bashrc" = "bash/bashrc"
"~/.gitconfig" = "git/gitconfig"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        let links = cm.get_global_links();
        assert!(links.is_some());
        let links = links.unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links.get("~/.bashrc").unwrap(), "bash/bashrc");
    }

    #[test]
    fn test_get_global_links_absent() {
        let dir = tempdir().unwrap();
        let toml = r#"
[profiles.desktop.links]
"~/.vimrc" = "vim/vimrc"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        assert!(cm.get_global_links().is_none());
    }

    #[test]
    fn test_get_settings_defaults() {
        let dir = tempdir().unwrap();
        let toml = "";
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        let settings = cm.get_settings();
        assert!(settings.on_conflict.is_none());
        assert!(settings.icon_style.is_none());
        assert!(settings.auto_detect_profile.is_none());
    }

    #[test]
    fn test_get_config_path_is_set_when_file_exists() {
        let dir = tempdir().unwrap();
        let path = write_config(dir.path(), "");
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        assert!(cm.get_config_path().is_some());
    }

    #[test]
    fn test_list_profiles_order_preserved() {
        let dir = tempdir().unwrap();
        let toml = r#"
[profiles.alpha.links]
"~/.a" = "a"

[profiles.beta.links]
"~/.b" = "b"

[profiles.gamma.links]
"~/.c" = "c"
"#;
        let path = write_config(dir.path(), toml);
        let cm = ConfigManager::load(Some(path.to_str().unwrap().to_string())).unwrap();

        let profiles = cm.list_profiles();
        assert_eq!(profiles, vec!["alpha", "beta", "gamma"]);
    }
}

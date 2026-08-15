pub mod conflict;

use anyhow::anyhow;
pub use conflict::ConflictAction;
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::message;

#[derive(Debug, Deserialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    #[serde(default, deserialize_with = "conflict::deserialize_conflict_action")]
    pub on_conflict: Option<ConflictAction>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { on_conflict: None }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Global {
    pub links: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub links: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    settings: Settings,
    global: Option<Global>,
    profiles: Option<IndexMap<String, Profile>>,

    #[serde(skip)]
    config_path: Option<PathBuf>,
}

impl Config {
    pub fn load(path: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
        let path_str = path.unwrap_or_else(|| "dotbee.toml".to_string());
        let config_path = Path::new(&path_str);

        let content = match fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self {
                    settings: Settings::default(),
                    global: None,
                    profiles: None,
                    config_path: None,
                });
            }
            Err(e) => return Err(e.into()),
        };
        let parsed: Config = toml::from_str(&content)?;
        let config_path = Some(fs::canonicalize(config_path)?);

        let mut config = Self {
            settings: parsed.settings,
            global: parsed.global,
            profiles: parsed.profiles,
            config_path,
        };
        config.normalize();
        config.validate().map_err(|errors| {
            anyhow!(
                "dotbee.toml configuration error: {} error(s) found\n{}",
                errors.len(),
                errors
                    .iter()
                    .enumerate()
                    .map(|(i, e)| format!("  {}. {}", i + 1, e))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

        Ok(config)
    }

    pub fn get_profile(&self, name: &str) -> anyhow::Result<&Profile, anyhow::Error> {
        let profiles = self.profiles.as_ref().ok_or(anyhow!("No profiles defined in configuration."))?;
        profiles.get(name).ok_or(anyhow!("Profile '{}' not found in configuration.", name))
    }

    pub fn list_profiles(&self) -> Vec<&str> {
        self.profiles
            .as_ref()
            .map(|p| p.keys().map(|k| k.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn has_profiles(&self) -> bool {
        self.profiles.as_ref().map(|p| !p.is_empty()).unwrap_or(false)
    }

    pub fn get_global_links(&self) -> Option<&IndexMap<String, String>> {
        self.global.as_ref().map(|g| &g.links)
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_config_path(&self) -> Option<&Path> {
        self.config_path.as_deref()
    }

    /// used for validating dotbee.toml config
    /// logs warnings and infos
    /// puts dotbee at a halt if it returns a Vec which contains the errors
    fn validate(&self) -> Result<(), Vec<String>> {
        // no config file, nothing to validate
        let Some(config_path) = &self.config_path else {
            return Ok(());
        };

        let dotfiles_root = config_path.parent().unwrap_or(Path::new("."));

        // collect errors to show them all
        let mut errors: Vec<String> = vec![];

        {
            let has_profiles = self.profiles.as_ref().is_some_and(|p| !p.is_empty());
            // check if profiles are empty
            if !has_profiles {
                message::warning("No profiles defined in configuration.");
            }
        }

        // check for empty global links
        if self.global.as_ref().is_some_and(|g| g.links.is_empty()) {
            message::info("Global links section is empty. Consider adding shared links here.");
        }

        // build flat list of all sources for checks
        let link_sources: Vec<(&str, &IndexMap<String, String>)> = {
            let mut sources: Vec<(&str, &IndexMap<String, String>)> = Vec::new();

            if let Some(g) = &self.global {
                sources.push(("global", &g.links));
            }

            if let Some(profiles) = &self.profiles {
                for (name, profile) in profiles {
                    // check for empty profile names
                    if name.is_empty() {
                        errors.push("Profile name is empty.".to_string());
                    }
                    sources.push((name.as_str(), &profile.links))
                }
            }

            sources
        };

        let global_links = link_sources
            .iter()
            .find(|(profile, _)| *profile == "global")
            .map(|(_, links)| *links);

        for (section, links) in link_sources {
            for (destination, source) in links {
                // check for empty strings
                if destination.is_empty() {
                    errors.push(format!("[{}]: link destination is empty.", section));
                }
                if source.is_empty() {
                    errors.push(format!("[{}]: link source is empty for destination '{}'.", section, destination));
                }

                // check for absolute or tilde paths (likely dest/source swapped)
                if source.starts_with('/') || source.starts_with('~') {
                    errors.push(format!(
                        "[{}]: source path '{}' looks like a destination path (starts with / or ~). Swap your destination and source.",
                        section, source
                    ));
                }

                // check if a profile overrides a global link
                if section != "global"
                    && let Some(global) = global_links
                    && global.contains_key(destination)
                {
                    errors.push(format!(
                        "'{}' in section '{}' overrides an existing global link.",
                        destination, section
                    ));
                }

                // check source path existance
                if !source.is_empty() && !source.starts_with('/') && !source.starts_with('~') {
                    let source_path = dotfiles_root.join(source);
                    if !source_path.exists() {
                        errors.push(format!(
                            "Source path '{}' not found (expected at {}).",
                            source,
                            source_path.display()
                        ));
                    }
                }
            }
        }

        // should hint to dotbee whether it should continue to the next step
        // or just put it at a halt
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn normalize(&mut self) {
        if let Some(global) = &mut self.global {
            for source in global.links.values_mut() {
                let trimmed = source.trim_start_matches("./");
                if trimmed.len() < source.len() {
                    *source = trimmed.to_string();
                }
            }
        }

        if let Some(profiles) = &mut self.profiles {
            for profile in profiles.values_mut() {
                for source in profile.links.values_mut() {
                    let trimmed = source.trim_start_matches("./");
                    if trimmed.len() < source.len() {
                        *source = trimmed.to_string();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::fs;
    use tempfile::TempDir;

    /// create a fake dir
    fn setup(toml: &str, sources: &[&str]) -> (TempDir, String) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("dotbee.toml");
        fs::write(&path, toml).unwrap();
        for src in sources {
            let src_path = dir.path().join(src);
            if let Some(parent) = src_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(src_path, "").unwrap();
        }
        (dir, path.to_string_lossy().to_string())
    }

    /// expecting an error to occur, if not then return error string :p
    fn error_msg(toml: &str, sources: &[&str]) -> String {
        let (_dir, path) = setup(toml, sources);
        match Config::load(Some(path)) {
            Ok(_) => panic!("expected error, got Ok"),
            Err(e) => e.to_string(),
        }
    }

    #[test]
    fn empty_profile_name() {
        let err = error_msg(
            r#"[profiles.""]
            links = { "~/.config/test" = "test" }
            "#,
            &[],
        );
        assert!(err.contains("Profile name is empty"), "got: {err}");
    }

    #[test]
    fn empty_destination() {
        let err = error_msg(
            r#"[global.links]
            "" = "foo"
            "#,
            &[],
        );
        assert!(err.contains("link destination is empty"), "got: {err}");
    }

    #[test]
    fn empty_source() {
        let err = error_msg(
            r#"[global.links]
            "~/.config/test" = ""
            "#,
            &[],
        );
        assert!(err.contains("link source is empty for destination '~/.config/test'"), "got: {err}");
    }

    #[test]
    fn swapped_dest_absolute() {
        let err = error_msg(
            r#"[profiles.p.links]
            "~/.config/test" = "/etc/passwd"
            "#,
            &[],
        );
        assert!(err.contains("looks like a destination path"), "got: {err}");
        assert!(err.contains("/etc/passwd"), "got: {err}");
    }

    #[test]
    fn swapped_dest_tilde() {
        let err = error_msg(
            r#"[profiles.p.links]
            "something" = "~/path"
            "#,
            &[],
        );
        assert!(err.contains("looks like a destination path"), "got: {err}");
        assert!(err.contains("~/path"), "got: {err}");
    }

    #[test]
    fn profile_overrides_global() {
        let err = error_msg(
            r#"[global.links]
            "~/.config/foo" = "global_foo"

            [profiles.p.links]
            "~/.config/foo" = "local_foo"
            "#,
            &["global_foo", "local_foo"],
        );
        assert!(err.contains("overrides an existing global link"), "got: {err}");
        assert!(err.contains("'p'"), "got: {err}");
    }

    #[test]
    fn missing_source_file() {
        let err = error_msg(
            r#"[profiles.p.links]
            "~/.config/test" = "nonexistent"
            "#,
            &[],
        );
        assert!(err.contains("Source path"), "got: {err}");
        assert!(err.contains("not found"), "got: {err}");
    }

    #[test]
    fn valid_global_links() {
        let (_dir, path) = setup(
            r#"[global.links]
            "~/.config/a" = "a"
            "#,
            &["a"],
        );
        let config = Config::load(Some(path)).unwrap();
        let links = config.get_global_links().unwrap();
        assert_eq!(links.get("~/.config/a"), Some(&"a".to_string()));
        assert_eq!(links.len(), 1);
        assert!(!config.has_profiles());
        assert!(config.get_config_path().is_some());
    }

    #[test]
    fn valid_profile_links() {
        let (_dir, path) = setup(
            r#"[profiles.p.links]
            "~/.config/b" = "b"
            "#,
            &["b"],
        );
        let config = Config::load(Some(path)).unwrap();
        let profile = config.get_profile("p").unwrap();
        assert_eq!(profile.links.get("~/.config/b"), Some(&"b".to_string()));
        assert_eq!(config.list_profiles(), vec!["p"]);
        assert!(config.has_profiles());
    }

    #[test]
    fn global_and_profile() {
        let (_dir, path) = setup(
            r#"[global.links]
            "~/.config/a" = "a"

            [profiles.p.links]
            "~/.config/b" = "b"
            "#,
            &["a", "b"],
        );
        let config = Config::load(Some(path)).unwrap();
        let global = config.get_global_links().unwrap();
        assert_eq!(global.get("~/.config/a"), Some(&"a".to_string()));
        let profile = config.get_profile("p").unwrap();
        assert_eq!(profile.links.get("~/.config/b"), Some(&"b".to_string()));
        assert_eq!(config.list_profiles(), vec!["p"]);
    }

    #[test]
    fn no_config_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nonexistent.toml");
        let config = Config::load(Some(path.to_string_lossy().to_string())).unwrap();
        assert!(config.get_config_path().is_none());
        assert!(!config.has_profiles());
    }

    #[test]
    fn empty_config() {
        let (_dir, path) = setup("[settings]", &[]);
        let config = Config::load(Some(path)).unwrap();
        assert!(!config.has_profiles());
        assert!(config.get_global_links().is_none());
        let settings = config.get_settings();
        assert!(settings.on_conflict.is_none());
    }

    #[test]
    fn normalization() {
        let (_dir, path) = setup(
            r#"[global.links]
            "~/.config/a" = "./a"

            [profiles.p.links]
            "~/.config/b" = "./b"
            "#,
            &["./a", "./b"],
        );
        let config = Config::load(Some(path)).unwrap();
        let global = config.get_global_links().unwrap();
        let profile = config.get_profile("p").unwrap();
        assert_eq!(
            global.get("~/.config/a"),
            Some(&"a".to_string()),
            "expected ./ prefix stripped, got {:?}",
            global.get("~/.config/a")
        );
        assert_eq!(
            profile.links.get("~/.config/b"),
            Some(&"b".to_string()),
            "expected ./ prefix stripped, got {:?}",
            profile.links.get("~/.config/b")
        );
    }
}

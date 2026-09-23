pub mod conflict;
pub mod hook;
pub mod vars;

use anyhow::anyhow;
pub use conflict::ConflictAction;
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::context::config::hook::Hook;
use crate::utils::message;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    #[serde(default, deserialize_with = "conflict::deserialize_conflict_action")]
    pub on_conflict: Option<ConflictAction>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub src: String,
    pub dst: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub links: IndexMap<String, Link>,
    pub pre_hook: Option<Hook>,
    pub post_hook: Option<Hook>,
}

/// reserved profile name for links applied regardless of the active profile
pub const GLOBAL_PROFILE: &str = "global";

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    settings: Settings,
    vars: IndexMap<String, String>,
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
                return Ok(Self::default());
            }
            Err(e) => return Err(e.into()),
        };
        let parsed: Config = toml::from_str(&content)?;
        let config_path = Some(fs::canonicalize(config_path)?);

        let mut config = Self {
            settings: parsed.settings,
            vars: parsed.vars,
            profiles: parsed.profiles,
            config_path,
        };

        let mut errors: Vec<String> = vec![];
        if let Err(e) = config.normalize() {
            errors.extend(e);
        }
        if let Err(e) = config.validate() {
            errors.extend(e);
        }

        if !errors.is_empty() {
            return Err(anyhow!(
                "dotbee.toml configuration error: {} error(s) found\n{}",
                errors.len(),
                errors
                    .iter()
                    .enumerate()
                    .map(|(i, e)| format!("  {}. {}", i + 1, e))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }

        Ok(config)
    }

    pub fn get_profile(&self, name: &str) -> anyhow::Result<&Profile, anyhow::Error> {
        let profiles = self
            .profiles
            .as_ref()
            .ok_or(anyhow!("No profiles defined in configuration."))?;
        profiles
            .get(name)
            .ok_or(anyhow!("Profile '{}' not found in configuration.", name))
    }

    pub fn list_profiles(&self) -> Vec<&str> {
        self.profiles
            .as_ref()
            .map(|p| {
                p.keys()
                    .map(|k| k.as_str())
                    .filter(|k| *k != GLOBAL_PROFILE)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn has_profiles(&self) -> bool {
        self.profiles
            .as_ref()
            .map(|p| p.keys().any(|k| k != GLOBAL_PROFILE))
            .unwrap_or(false)
    }

    pub fn get_global_links(&self) -> Option<&IndexMap<String, Link>> {
        self.profiles
            .as_ref()?
            .get(GLOBAL_PROFILE)
            .map(|p| &p.links)
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_path(&self) -> Option<&Path> {
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
            let has_profiles = self
                .profiles
                .as_ref()
                .is_some_and(|p| p.keys().any(|k| k != GLOBAL_PROFILE));
            // check if profiles are empty
            if !has_profiles {
                message::warning("No profiles defined in configuration.");
            }
        }

        // check for empty global links
        if self
            .profiles
            .as_ref()
            .and_then(|p| p.get(GLOBAL_PROFILE))
            .is_some_and(|g| g.links.is_empty())
        {
            message::info("Global links section is empty. Consider adding shared links here.");
        }

        // build flat list of all sources for checks
        let link_sources: Vec<(&str, &IndexMap<String, Link>)> = {
            let mut sources: Vec<(&str, &IndexMap<String, Link>)> = Vec::new();

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
            for (name, link) in links {
                if name.is_empty() {
                    errors.push(format!("[{}]: link name is empty.", section));
                }
                if link.src.is_empty() {
                    errors.push(format!("[{}]: link '{}' has an empty src.", section, name));
                }
                if link.dst.is_empty() {
                    errors.push(format!("[{}]: link '{}' has an empty dst.", section, name));
                }

                // src lives inside the dotfiles repo, so it must be relative
                if link.src.starts_with('/') || link.src.starts_with('~') {
                    errors.push(format!(
                "[{}]: link '{}' has src '{}' which looks like a destination path (starts with / or ~). src should be relative to the dotfiles root.",
                section, name, link.src
            ));
                }

                // dst is where the symlink lands, so it must be absolute (or ~)
                if !link.dst.is_empty() && !link.dst.starts_with('/') && !link.dst.starts_with('~')
                {
                    errors.push(format!(
                        "[{}]: link '{}' has dst '{}' which is not absolute. dst should start with / or ~.",
                        section, name, link.dst
                    ));
                }

                // profile overrides global now compares by dst value, not map key
                if section != "global"
                    && let Some(global) = global_links
                    && global.values().any(|g| g.dst == link.dst)
                {
                    errors.push(format!(
                        "'{}' in section '{}' overrides an existing global link (dst '{}').",
                        name, section, link.dst
                    ));
                }

                // source path existence
                if !link.src.is_empty() && !link.src.starts_with('/') && !link.src.starts_with('~')
                {
                    let source_path = dotfiles_root.join(&link.src);
                    if !source_path.exists() {
                        errors.push(format!(
                            "Source path '{}' not found (expected at {}).",
                            link.src,
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

    fn normalize(&mut self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = vec![];

        // validate variable names
        for name in self.vars.keys() {
            if name.is_empty() {
                errors.push("[vars]: variable name is empty.".to_string());
            } else if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                errors.push(format!(
                    "[vars]: variable name '{}' is invalid. Only alphanumeric characters and '_' are allowed.",
                    name
                ));
            }
        }

        // interpolate variables into link src/dst
        if let Some(profiles) = &mut self.profiles {
            for (profile_name, profile) in profiles {
                for (link_name, link) in profile.links.iter_mut() {
                    let src_context = format!("[{}]: link '{}' src", profile_name, link_name);
                    match vars::interpolate(&link.src, &self.vars, &src_context) {
                        Ok(src) => link.src = src,
                        Err(e) => errors.extend(e),
                    }

                    let dst_context = format!("[{}]: link '{}' dst", profile_name, link_name);
                    match vars::interpolate(&link.dst, &self.vars, &dst_context) {
                        Ok(dst) => link.dst = dst,
                        Err(e) => errors.extend(e),
                    }

                    link.src = link.src.trim_start_matches("./").to_string();
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

use crate::config::Profile;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum DestinationStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink,
    NonExistent,
}

pub fn expand_path(path_str: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if path_str.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            if path_str == "~" {
                return Ok(home);
            }
            // safely strip prefix
            return Ok(home.join(path_str.trim_start_matches("~/")));
        }
    }
    Ok(PathBuf::from(path_str))
}

pub fn is_profile_active(profile: &Profile, cwd: &Path) -> bool {
    if profile.links.is_empty() {
        return false;
    }

    for (target_str, source_str) in &profile.links {
        let target_path = match expand_path(target_str) {
            Ok(p) => p,
            Err(_) => return false,
        };
        let source_path = cwd.join(source_str);

        if !target_path.is_symlink() {
            return false;
        }

        match fs::read_link(&target_path) {
            Ok(p) => {
                if p != source_path {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    true
}

pub fn get_destination_status(source: &Path, destination: &Path) -> Result<DestinationStatus, Box<dyn std::error::Error>> {
    if !destination.exists() && !destination.is_symlink() {
        return Ok(DestinationStatus::NonExistent);
    }

    let target = match fs::read_link(destination) {
        Ok(v) => v,
        Err(_) => return Ok(DestinationStatus::ConflictingFileOrDir),
    };

    match (destination.is_symlink(), target == source) {
        (true, true) => Ok(DestinationStatus::AlreadyLinked),
        (true, false) => Ok(DestinationStatus::ConflictingSymlink),
        _ => Ok(DestinationStatus::ConflictingFileOrDir),
    }
}

pub fn unlink_profile_links(links: &std::collections::HashMap<String, String>, cwd: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for (target_str, source_str) in links {
        let target_path = expand_path(target_str)?;
        let source_path = cwd.join(source_str);

        if target_path.is_symlink() {
            let actual_target = fs::read_link(&target_path)?;
            if actual_target == source_path {
                fs::remove_file(&target_path)?;
                println!("  {} Unlinked {}", " ".red(), target_str);
            }
        }
    }
    Ok(())
}

pub fn symlink_with_parents(source: &Path, destination: &PathBuf) -> std::io::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    std::os::unix::fs::symlink(source, destination)
}

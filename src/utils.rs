use crate::context::Context;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum DestinationStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink,
    NonExistent,
}

/// expands tilde to $HOME; otherwise, returns the same path
pub fn expand_tilde(path_str: &str) -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        if path_str == "~" {
            return home;
        }

        if let Some(stripped) = path_str.strip_prefix("~/") {
            return home.join(stripped);
        }
    }
    PathBuf::from(path_str)
}

pub fn get_destination_status(source: &Path, destination: &Path) -> DestinationStatus {
    if !destination.exists() && !destination.is_symlink() {
        return DestinationStatus::NonExistent;
    }

    let Ok(target) = fs::read_link(destination) else {
        return DestinationStatus::ConflictingFileOrDir;
    };

    match (destination.is_symlink(), target == source) {
        (true, true) => DestinationStatus::AlreadyLinked,
        (true, false) => DestinationStatus::ConflictingSymlink,
        _ => DestinationStatus::ConflictingFileOrDir,
    }
}

pub fn symlink_with_parents(source: &Path, destination: &PathBuf, context: &Context) -> std::io::Result<()> {
    if context.dry_run {
        return Ok(());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    std::os::unix::fs::symlink(source, destination)
}

pub fn get_hostname() -> String {
    use nix::unistd::gethostname;
    let hostname = gethostname().expect("Couldn't get hostname");
    hostname.into_string().expect("Failed to parse hostname")
}

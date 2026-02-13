use context::message::Message;
use indexmap::IndexMap;
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

pub fn unlink_profile_links(links: &IndexMap<String, String>, dry_run: bool, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;

    for (target_str, source_str) in links {
        let target_path = expand_tilde(target_str);
        let source_path = cwd.join(source_str);

        if target_path.is_symlink() && fs::read_link(&target_path)? == source_path {
            if dry_run {
                message.delete(&format!("Would unlink {} (dry run)", target_str));
            } else {
                fs::remove_file(&target_path)?;
                message.delete(&format!("Unlinked {}", target_str));
            }
        }
    }
    Ok(())
}

pub fn symlink_with_parents(source: &Path, destination: &PathBuf, dry_run: bool) -> std::io::Result<()> {
    if dry_run {
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
    let hostname_string = hostname.into_string().expect("Failed to parse hostname");
    hostname_string
}


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

pub fn expand_path(path_str: &str) -> PathBuf {
    if path_str.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            if path_str == "~" {
                return home;
            }
            // safely strip prefix
            return home.join(path_str.trim_start_matches("~/"));
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

pub fn unlink_profile_links(
    links: &IndexMap<String, String>,
    cwd: &Path,
    dry_run: bool,
    message: &Message,
) -> Result<(), Box<dyn std::error::Error>> {
    for (target_str, source_str) in links {
        let target_path = expand_path(target_str);
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
    let hostname_string = hostname.into_string().expect("failed to convert from OsString to String");
    hostname_string
}

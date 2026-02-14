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
    match dirs::home_dir() {
        // simply return home
        Some(home) if path_str == "~" => home,
        // slice it after ~/ (same as strip_prefix()) then join it with dirs::home_dir()
        Some(home) if path_str.starts_with("~/") => home.join(&path_str[2..]),
        // do nothing
        _ => PathBuf::from(path_str),
    }
}

pub fn get_destination_status(source: &Path, destination: &Path) -> DestinationStatus {
    let metadata = match fs::symlink_metadata(destination) {
        Ok(meta) => meta,
        Err(_) => return DestinationStatus::NonExistent,
    };

    if !metadata.is_symlink() {
        return DestinationStatus::ConflictingFileOrDir;
    }

    match fs::read_link(destination) {
        Ok(target) if target == source => DestinationStatus::AlreadyLinked,
        _ => DestinationStatus::ConflictingSymlink,
    }
}

pub fn symlink_with_parents(source: &Path, destination: &Path, context: &Context) -> std::io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_expand_tilde() {
        if let Some(home) = dirs::home_dir() {
            assert_eq!(expand_tilde("~"), home);
            assert_eq!(expand_tilde("~/test"), home.join("test"));
        }
        assert_eq!(expand_tilde("/etc/hosts"), PathBuf::from("/etc/hosts"));
    }

    #[test]
    fn test_destination_status() -> std::io::Result<()> {
        let dir = tempdir()?;
        let source = dir.path().join("source");
        let dest = dir.path().join("dest");

        // Non-existent
        assert_eq!(get_destination_status(&source, &dest), DestinationStatus::NonExistent);

        // File/Dir (not symlink)
        fs::write(&dest, "content")?;
        assert_eq!(get_destination_status(&source, &dest), DestinationStatus::ConflictingFileOrDir);

        // Correct symlink
        fs::remove_file(&dest)?;
        std::os::unix::fs::symlink(&source, &dest)?;
        assert_eq!(get_destination_status(&source, &dest), DestinationStatus::AlreadyLinked);

        // Conflicting symlink
        let other_source = dir.path().join("other_source");
        fs::remove_file(&dest)?;
        std::os::unix::fs::symlink(&other_source, &dest)?;
        assert_eq!(get_destination_status(&source, &dest), DestinationStatus::ConflictingSymlink);

        Ok(())
    }
}

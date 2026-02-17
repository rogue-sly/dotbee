use crate::context::Context;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the status of a potential symlink destination.
#[derive(Debug, PartialEq)]
pub enum DestinationStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink,
    NonExistent,
}

/// Expands a tilde (`~`) at the start of a path to the user's home directory.
///
/// # Arguments
/// * `path_str` - A string slice representing the path to expand.
///
/// # Returns
/// A `PathBuf` with the tilde expanded, or the original path if no tilde was present.
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

/// Determines the status of a `destination` path relative to a `source` path
/// for symlink operations. This is crucial for safely managing dotfiles,
/// allowing the application to identify existing links, conflicts, or
/// non-existent paths.
///
/// # Arguments
/// * `source` - The path to the original file or directory that the symlink
///   *should* point to.
/// * `destination` - The location where the symlink *is* or *would be* created.
///
/// # Returns
/// A `DestinationStatus` enum indicating the current state of the `destination` path:
/// * `AlreadyLinked`: The `destination` is a symlink correctly pointing to `source`.
/// * `ConflictingFileOrDir`: The `destination` exists but is a regular file or directory.
/// * `ConflictingSymlink`: The `destination` is a symlink, but it either points
///   to a different path than `source` or is broken.
/// * `NonExistent`: The `destination` path does not exist.
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

/// Creates a symbolic link from `source` to `destination`, ensuring all parent
/// directories of `destination` exist beforehand.
///
/// This function acts like `mkdir -p` followed by `ln -s`.
///
/// # Arguments
/// * `source` - The path to the original file or directory that the symlink will point to.
/// * `destination` - The path where the symbolic link will be created.
/// * `context` - The application context, used to check for `dry_run` mode.
///
/// # Errors
/// This function will return an `std::io::Error` if:
/// * It's not in `dry_run` mode and fails to create parent directories.
/// * It's not in `dry_run` mode and fails to create the symbolic link.
pub fn symlink(source: &Path, destination: &Path, context: &Context) -> std::io::Result<()> {
    if context.dry_run {
        return Ok(());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    std::os::unix::fs::symlink(source, destination)
}

/// Retrieves the system's hostname.
///
/// # Panics
/// This function will panic if it fails to get the hostname from the system or
/// if the hostname cannot be parsed into a valid string.
pub fn get_hostname() -> String {
    use nix::unistd::gethostname;
    let hostname = gethostname().expect("Couldn't get hostname");
    hostname.into_string().expect("Failed to parse hostname")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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

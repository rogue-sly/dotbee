use std::{fs, path::Path};

#[derive(Debug, PartialEq)]
pub enum SymlinkStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink,
    NonExistent,
}

#[derive(Default)]
pub struct Symlink;

impl Symlink {
    pub fn new() -> Self {
        Self
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
    /// A `SymlinkStatus` enum indicating the current state of the `destination` path:
    /// * `AlreadyLinked`: The `destination` is a symlink correctly pointing to `source`.
    /// * `ConflictingFileOrDir`: The `destination` exists but is a regular file or directory.
    /// * `ConflictingSymlink`: The `destination` is a symlink, but it either points
    ///   to a different path than `source` or is broken.
    /// * `NonExistent`: The `destination` path does not exist.
    pub fn check(&self, source: &Path, destination: &Path) -> SymlinkStatus {
        match fs::read_link(destination) {
            Ok(target) if target == source => SymlinkStatus::AlreadyLinked,
            Ok(_) => SymlinkStatus::ConflictingSymlink,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => SymlinkStatus::NonExistent,
            Err(_) => SymlinkStatus::ConflictingFileOrDir,
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
    ///
    /// # Errors
    /// This function will return an error if:
    /// * fails to create parent directories.
    /// * fails to create the symbolic link.
    pub fn create(&self, source: &Path, destination: &Path) -> anyhow::Result<(), anyhow::Error> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        std::os::unix::fs::symlink(source, destination)?;
        Ok(())
    }
}

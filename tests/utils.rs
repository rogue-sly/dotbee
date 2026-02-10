use std::{fs, path::PathBuf};
use dotsy::utils::{DestinationStatus, expand_path, get_destination_status};
use tempfile::tempdir;

#[test]
fn test_expand_path_no_tilde() {
    assert_eq!(expand_path("foo/bar"), PathBuf::from("foo/bar"));
    assert_eq!(expand_path("/abs/path"), PathBuf::from("/abs/path"));
}

#[test]
fn test_expand_path_with_tilde() {
    if let Some(home) = dirs::home_dir() {
        assert_eq!(expand_path("~"), home);
        assert_eq!(expand_path("~/foo"), home.join("foo"));
    }
}

#[test]
fn test_get_destination_status() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let source = dir.path().join("source");
    let destination = dir.path().join("dest");

    // NonExistent
    assert_eq!(get_destination_status(&source, &destination), DestinationStatus::NonExistent);

    // ConflictingFileOrDir
    fs::write(&destination, "content")?;
    assert_eq!(
        get_destination_status(&source, &destination),
        DestinationStatus::ConflictingFileOrDir
    );
    fs::remove_file(&destination)?;

    // AlreadyLinked
    std::os::unix::fs::symlink(&source, &destination)?;
    assert_eq!(get_destination_status(&source, &destination), DestinationStatus::AlreadyLinked);

    // ConflictingSymlink
    let other_source = dir.path().join("other_source");
    fs::remove_file(&destination)?;
    std::os::unix::fs::symlink(&other_source, &destination)?;
    assert_eq!(get_destination_status(&source, &destination), DestinationStatus::ConflictingSymlink);

    Ok(())
}

use std::fs;

use tempfile::TempDir;

use dotbee::context::config::Config;

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

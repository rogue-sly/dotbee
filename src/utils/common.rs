use anyhow::{Context, Result};
use nix::unistd::gethostname;
use std::path::PathBuf;

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

/// Retrieves the system's hostname.
pub fn get_hostname() -> Result<String> {
    let hostname = gethostname().context("Couldn't get hostname")?;
    let hostname = hostname.to_str().context("Failed to parse hostname")?.to_string();
    Ok(hostname)
}

use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn get_config<'a>() -> Result<&'a Path, Box<dyn std::error::Error>> {
    let config_path = Path::new("dotsy.toml");
    return match config_path.exists() {
        true => Ok(config_path),
        false => Err(format!(
            "{} {}",
            " ".red(),
            "dotsy.toml already exists in the current directory."
        )
        .into()),
    };
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

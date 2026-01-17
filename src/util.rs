use colored::Colorize;
use std::path::Path;

pub fn get_config<'a>() -> Result<&'a Path, Box<dyn std::error::Error>> {
    let config_path = Path::new("dotsy.toml");
    return match config_path.exists() {
        true => Ok(config_path),
        false => Err(format!("{} {}", " ".red(), "dotsy.toml already exists in the current directory.").into()),
    };
}

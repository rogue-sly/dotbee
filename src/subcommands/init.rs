use crate::config::Icons;
use colored::Colorize;
use std::error::Error;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../config/dotsy.toml");

pub fn run(config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let path_string = config_path.unwrap_or("dotsy.toml".to_string());
    let config_path = Path::new(&path_string);
    let icons = Icons::new("nerdfonts");

    if config_path.exists() {
        println!(
            "{} {}",
            icons.cross.red(),
            format!("{} already exists in the current directory.", path_string).red()
        );
        return Ok(());
    }

    if dry_run {
        println!("{} Would initialize {} (dry run)", icons.check.green(), path_string);
        return Ok(());
    }

    fs::write(config_path, DEFAULT_CONFIG)?;

    println!("{} {}", icons.check.green(), format!("Successfully initialized {}", path_string).green());
    println!(
        "Edit the file to configure your dotfiles, then run {} to apply.",
        format!("dotsy switch <profile>").yellow()
    );

    Ok(())
}

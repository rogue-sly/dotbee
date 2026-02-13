use colored::Colorize;
use crate::context::Context;
use std::error::Error;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../config/dotsy.toml");

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let path_string = context.config_path.clone().unwrap_or("dotsy.toml".to_string());
    let config_path = Path::new(&path_string);
    let message = &context.message;

    if config_path.exists() {
        message.error(&format!("{} already exists in the current directory.", path_string));
        return Ok(());
    }

    if context.dry_run {
        message.success(&format!("Would initialize {} (dry run)", path_string));
        return Ok(());
    }

    fs::write(config_path, DEFAULT_CONFIG)?;

    message.success(&format!("Successfully initialized {}", path_string));
    println!(
        "Edit the file to configure your dotfiles, then run {} to apply.",
        "dotsy switch <profile>".yellow()
    );

    Ok(())
}

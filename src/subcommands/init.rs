use crate::context::Context;
use colored::Colorize;
use std::error::Error;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../config/dotsy.toml");

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
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

    // Update state to remember this dotfiles directory
    if let Ok(abs_config_path) = fs::canonicalize(config_path)
        && let Some(parent) = abs_config_path.parent()
    {
        context.state.dotfiles_path = Some(parent.to_path_buf());
        context.state.save()?;
    }

    message.success(&format!("Successfully initialized {}", path_string));
    println!(
        "Edit the file to configure your dotfiles, then run {} to apply.",
        "dotsy switch <profile>".yellow()
    );

    Ok(())
}

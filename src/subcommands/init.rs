use crate::context::Context;
use colored::Colorize;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG: &str = include_str!("../context/manager/config/dotbee.toml");

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let path_string = context
        .manager
        .config
        .get_config_path()
        .map(|p| p.to_path_buf())
        .unwrap_or(PathBuf::from("dotbee.toml"));
    let config_path = Path::new(&path_string);
    let message = &context.message;

    if config_path.exists() {
        message.error(&format!(
            "{} already exists in the current directory.",
            path_string.to_string_lossy()
        ));
        return Ok(());
    }

    if context.dry_run {
        message.success(&format!("Would initialize {} (dry run)", path_string.to_string_lossy()));
        return Ok(());
    }

    fs::write(config_path, DEFAULT_CONFIG)?;

    // Update state to remember this dotfiles directory
    if let Some(parent) = fs::canonicalize(config_path)
        .ok()
        .and_then(|abs_config_path| abs_config_path.parent().map(|dotfiles_path| dotfiles_path.to_path_buf()))
    {
        context.manager.state.set_dotfiles_path(Some(parent))?;
    }

    message.success(&format!("Successfully initialized {}", path_string.to_string_lossy()));
    println!(
        "Edit the file to configure your dotfiles, then run {} to apply.",
        "dotbee switch <profile>".yellow()
    );

    Ok(())
}

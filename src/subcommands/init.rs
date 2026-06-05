use crate::context::Context;
use crate::utils::message;
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG: &str = include_str!("../context/config/dotbee.toml");

pub fn run(context: &mut Context) -> anyhow::Result<(), anyhow::Error> {
    let path_string = context
        .config
        .get_config_path()
        .map(|p| p.to_path_buf())
        .unwrap_or(PathBuf::from("dotbee.toml"));
    let config_path = Path::new(&path_string);

    if context.dry_run {
        if config_path.exists() {
            message::error(&format!(
                "{} already exists in the current directory.",
                path_string.to_string_lossy()
            ));
        } else {
            message::success(&format!("Would initialize {} (dry run)", path_string.to_string_lossy()));
        }
        return Ok(());
    }

    // check then create
    match fs::File::create_new(config_path) {
        Ok(mut file) => {
            file.write_all(DEFAULT_CONFIG.as_bytes())?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            message::error(&format!(
                "{} already exists in the current directory.",
                path_string.to_string_lossy()
            ));
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    }

    // update state to remember this dotfiles directory
    if let Some(parent) = fs::canonicalize(config_path)
        .ok()
        .and_then(|abs_config_path| abs_config_path.parent().map(|dotfiles_path| dotfiles_path.to_path_buf()))
    {
        context.state.set_dotfiles_path(Some(parent))?;
    }

    message::success(&format!("Successfully initialized {}", path_string.to_string_lossy()));
    println!(
        "Edit the file to configure your dotfiles, then run {} to apply.",
        "dotbee switch <profile>".yellow()
    );

    Ok(())
}

use crate::context::Context;
use crate::utils::message;
use anyhow::{Result, anyhow, bail};
use std::env;
use std::path::Path;
use std::process::Command;

pub fn run(context: &Context) -> Result<()> {
    let config_path = context
        .config
        .get_path()
        .ok_or_else(|| anyhow!("Could not find dotbee.toml. Run 'dotbee init' to create one."))?;

    let editor = resolve_editor();

    if context.dry_run {
        message::info(&format!("Would open {} with '{}'", config_path.display(), editor));
        return Ok(());
    }

    open_editor(config_path, &editor)
}

fn resolve_editor() -> String {
    env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            message::warning("$EDITOR is not set, falling back to 'vi'.");
            "vi".to_string()
        })
}

fn open_editor(path: &Path, editor: &str) -> Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("{editor} \"$1\""))
        .arg("sh")
        .arg(path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("Editor '{}' exited with status {:?}", editor, status.code());
    }
}

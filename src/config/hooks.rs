use colored::Colorize;
use indexmap::IndexMap;
use serde::Deserialize;
use std::error::Error;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Hooks {
    pub pre: Option<IndexMap<String, String>>,
    pub post: Option<IndexMap<String, String>>,
}

pub fn execute_hook(hooks: &IndexMap<String, String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    for (name, command) in hooks {
        if dry_run {
            println!("  Would run {}: {} (dry run)", name.cyan(), command);
        } else {
            println!("  Running {}: {}", name.cyan(), command);
            let status = Command::new("sh").arg("-c").arg(command).status().unwrap();
            if !status.success() {
                eprintln!("{} Hook '{}' failed with status {}", "Warning:".yellow(), name, status);
            }
        }
    }
    Ok(())
}

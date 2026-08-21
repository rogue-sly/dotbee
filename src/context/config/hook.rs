use anyhow::Result;
use nix::libc;
use serde::Deserialize;
use std::{path::PathBuf, process::Command};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Hook {
    File { path: PathBuf },
    Inline(String),
}

impl Hook {
    pub fn execute(&self) -> Result<()> {
        match self {
            Hook::File { path } => match Command::new(path).status() {
                Ok(_status) => {}
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: The script at {:?} is not executable. Try 'chmod +x {:?}'", path, path);
                }
                Err(e) if e.raw_os_error() == Some(libc::ENOEXEC) => {
                    eprintln!(
                        "Error: Exec format error. Your script at '{:?}' is missing a shebang (e.g., #!/bin/sh) at the top of the file.",
                        path
                    );
                }
                Err(e) => eprintln!("An unexpected error occurred: {}", e),
            },
            Hook::Inline(code) => {
                Command::new("sh").arg("-c").arg(code).status()?;
            }
        }
        Ok(())
    }
}

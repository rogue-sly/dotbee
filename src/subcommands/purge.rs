use crate::utils::common::expand_tilde;
use crate::{context::Context, utils::message};
use colored::Colorize;
use std::fs;

pub fn run(context: &mut Context) -> anyhow::Result<(), anyhow::Error> {
    let links = context.manager.state.get_links();

    if links.is_empty() {
        message::info("No managed links found to purge.");
        if !context.dry_run {
            context.manager.state.clear()?;
            message::success("State cleared.");
        }
        return Ok(());
    }

    if context.dry_run {
        println!("{}", "Purge Plan (Dry Run):".bold().yellow());
    } else {
        println!("{}", "Executing Purge...".bold().red());
    }

    for link in links {
        let target_path = expand_tilde(&link.target);

        if !target_path.exists() && !target_path.is_symlink() {
            if context.dry_run {
                message::warning(&format!("{} is already missing from disk.", link.target));
            } else {
                message::warning(&format!("Cleaning up stale state for missing link: {}", link.target));
            }
            continue;
        }

        if !target_path.is_symlink() {
            if context.dry_run {
                message::error(&format!("SKIPPING {}: not a symlink.", link.target));
            } else {
                message::error(&format!("Aborting removal of {}: path is a real file/directory.", link.target));
            }
            continue;
        }

        if context.dry_run {
            message::delete(&format!("Would remove {}", link.target));
        } else {
            match fs::remove_file(&target_path) {
                Ok(_) => message::delete(&format!("Removed {}", link.target)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    message::warning(&format!("Target '{}' disappeared during execution.", link.target));
                }
                Err(e) => message::error(&format!("Failed to remove {}: {}", link.target, e)),
            }
        }
    }

    if !context.dry_run {
        context.manager.state.clear()?;
        message::success("Purge complete.");
    }

    Ok(())
}

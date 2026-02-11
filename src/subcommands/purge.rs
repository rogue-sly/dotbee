use colored::Colorize;
use context::Context;
use std::{error::Error, fs, io};
use utils::expand_path;

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;

    if context.dry_run {
        println!("{}", "Purging all managed links from state (dry run)...".bold().red());
    } else {
        println!("{}", "Purging all managed links from state...".bold().red());
    }

    let mut links_to_unlink = Vec::new();
    if !context.dry_run {
        // Clone links from state before clearing it in memory
        links_to_unlink = context.state.managed_links.clone();
        context.state.managed_links.clear(); // Clear state in memory
    }

    for link in &links_to_unlink {
        let target_path = expand_path(&link.target); // Resolve target path from state

        if target_path.is_symlink() {
            // Optionally, one could check if target_path.read_link() == expand_path(&link.source)
            // For a robust purge, we assume anything recorded in state as a symlink should be removed.

            if context.dry_run {
                msg.delete(&format!("Would unlink {} (dry run)", link.target));
            } else {
                // Attempt to remove the symlink
                match fs::remove_file(&target_path) {
                    Ok(_) => msg.delete(&format!("Unlinked {}", link.target)),
                    Err(e) => {
                        // Handle errors: target might be gone, or permissions issues
                        if e.kind() == io::ErrorKind::NotFound {
                            // Target was not found but was in state. Log a warning.
                            msg.warning(&format!("Target '{}' not found but was in state.", link.target));
                        } else {
                            // Other errors like permission denied
                            msg.error(&format!("Failed to unlink {}: {}", link.target, e));
                        }
                    }
                }
            }
        } else {
            // Path in state is not a symlink or does not exist. Log a warning.
            if !context.dry_run {
                msg.warning(&format!("Path '{}' in state is not a symlink or does not exist.", link.target));
            }
        }
    }

    if context.dry_run {
        msg.success("Purge dry run complete.");
    } else {
        // Save the cleared managed_links list
        context.state.save()?;
        // Also clear active profile as per original behavior
        context.state.clear_active_profile()?;
        msg.success("Purge complete.");
    }

    Ok(())
}

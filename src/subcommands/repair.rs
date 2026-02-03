use colored::Colorize;
use dotsy::context::Context;
use dotsy::message::Message;
use dotsy::utils::{DestinationStatus, expand_path, find_active_profile, get_destination_status, symlink_with_parents};
use indexmap::IndexMap;
use std::error::Error;
use std::path::Path;

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let message = &context.message;

    if context.dry_run {
        println!("{}", "Repairing symlinks (dry run)...".bold().blue());
    } else {
        println!("{}", "Repairing symlinks...".bold().blue());
    }

    if let Some(global) = &context.config.global {
        println!("Checking global links...");
        repair_links(&global.links, &cwd, context.dry_run, message)?;
    }

    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = find_active_profile(profiles, context.state.active_profile.as_ref(), &cwd) {
            if let Some(profile) = profiles.get(active_name) {
                message.info(&format!("Checking active profile '{}'...", active_name.green()));
                repair_links(&profile.links, &cwd, context.dry_run, message)?;
            } else {
                message.info(&format!("Active profile '{}' not found in config.", active_name));
            }
        } else {
            message.info("No active profile detected. Only global links were checked.");
        }
    }

    message.success("Repair complete.");
    Ok(())
}

fn repair_links(links: &IndexMap<String, String>, cwd: &Path, dry_run: bool, message: &Message) -> Result<(), Box<dyn Error>> {
    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            message.unlink(&format!("Source missing: {}", source_path.display()));
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {}
            DestinationStatus::NonExistent => {
                if dry_run {
                    message.success(&format!("Would link {} -> {} (dry run)", source_str, target_str));
                } else {
                    message.success(&format!("Linking {} -> {}", source_str, target_str));
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                }
            }
            DestinationStatus::ConflictingSymlink => {
                if dry_run {
                    message.success(&format!("Would relink {} -> {} (dry run)", source_str, target_str));
                } else {
                    message.success(&format!("Relinking {} -> {}", source_str, target_str));
                    if target_path.exists() || target_path.is_symlink() {
                        std::fs::remove_file(&target_path)?;
                    }
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                }
            }
            DestinationStatus::ConflictingFileOrDir => {
                message.error(&format!(
                    "Conflict at {} (File/Dir exists). Manual intervention required.",
                    target_str
                ));
            }
        }
    }
    Ok(())
}

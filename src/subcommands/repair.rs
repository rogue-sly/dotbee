use crate::config::icons::Icons;
use crate::context::Context;
use crate::utils::{DestinationStatus, expand_path, find_active_profile, get_destination_status, symlink_with_parents};
use colored::Colorize;
use indexmap::IndexMap;
use std::error::Error;
use std::path::Path; // Added this import

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let icons = &context.icons;

    if context.dry_run {
        println!("{}", "Repairing symlinks (dry run)...".bold().blue());
    } else {
        println!("{}", "Repairing symlinks...".bold().blue());
    }

    if let Some(global) = &context.config.global {
        println!("Checking global links...");
        repair_links(&global.links, &cwd, context.dry_run, icons)?;
    }

    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = find_active_profile(profiles, context.state.active_profile.as_ref(), &cwd) {
            if let Some(profile) = profiles.get(active_name) {
                println!("Checking active profile '{}'...", active_name.green());
                repair_links(&profile.links, &cwd, context.dry_run, icons)?;
            } else {
                println!("Active profile '{}' not found in config.", active_name);
            }
        } else {
            println!("No active profile detected. Only global links were checked.");
        }
    }

    println!("{}", "Repair complete.".green());
    Ok(())
}

fn repair_links(links: &IndexMap<String, String>, cwd: &Path, dry_run: bool, icons: &Icons) -> Result<(), Box<dyn Error>> {
    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            println!("  {} Source missing: {}", icons.unlink.yellow(), source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {}
            DestinationStatus::NonExistent => {
                if dry_run {
                    println!("  {} Would link {} -> {} (dry run)", icons.success.green(), source_str, target_str);
                } else {
                    println!("  {} Linking {} -> {}", icons.success.green(), source_str, target_str);
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                }
            }
            DestinationStatus::ConflictingSymlink => {
                if dry_run {
                    println!(
                        "  {} Would relink {} -> {} (dry run)",
                        icons.success.green(),
                        source_str,
                        target_str
                    );
                } else {
                    println!("  {} Relinking {} -> {}", icons.success.green(), source_str, target_str);
                    if target_path.exists() || target_path.is_symlink() {
                        std::fs::remove_file(&target_path)?;
                    }
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                }
            }
            DestinationStatus::ConflictingFileOrDir => {
                println!(
                    "  {} Conflict at {} (File/Dir exists). Manual intervention required.",
                    icons.error.red(),
                    target_str
                );
            }
        }
    }
    Ok(())
}

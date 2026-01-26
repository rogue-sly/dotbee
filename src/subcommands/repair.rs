use crate::config::Config;
use crate::config::Icons;
use crate::util::{expand_path, get_destination_status, is_profile_active, symlink_with_parents, DestinationStatus};
use colored::Colorize;
use std::error::Error;
use std::path::Path;

pub fn run(config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let cwd = std::env::current_dir()?;
    let icon_style = config.settings.icon_style.as_deref().unwrap_or("nerdfonts");
    let icons = Icons::new(icon_style);

    if dry_run {
        println!("{}", "Repairing symlinks (dry run)...".bold().blue());
    } else {
        println!("{}", "Repairing symlinks...".bold().blue());
    }

    if let Some(global) = &config.global {
        println!("Checking global links...");
        repair_links(&global.links, &cwd, dry_run, &icons)?;
    }

    if let Some(profiles) = &config.profiles {
        let mut active_found = false;
        for (name, profile) in profiles {
            if is_profile_active(profile, &cwd) {
                println!("Checking profile '{}'...", name.green());
                repair_links(&profile.links, &cwd, dry_run, &icons)?;
                active_found = true;
            }
        }
        if !active_found {
            println!("No active profile detected. Only global links were checked.");
        }
    }

    println!("{}", "Repair complete.".green());
    Ok(())
}

fn repair_links(
    links: &std::collections::HashMap<String, String>,
    cwd: &Path,
    dry_run: bool,
    icons: &Icons,
) -> Result<(), Box<dyn Error>> {
    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            println!("  {} Source missing: {}", icons.unlink.yellow(), source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {
                // All good, do nothing
            }
            DestinationStatus::NonExistent => {
                if dry_run {
                    println!("  {} Would link {} -> {} (dry run)", icons.check.green(), source_str, target_str);
                } else {
                    println!("  {} Linking {} -> {}", icons.check.green(), source_str, target_str);
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                }
            }
            DestinationStatus::ConflictingSymlink => {
                if dry_run {
                    println!("  {} Would relink {} -> {} (dry run)", icons.check.green(), source_str, target_str);
                } else {
                    println!("  {} Relinking {} -> {}", icons.check.green(), source_str, target_str);
                    if target_path.exists() || target_path.is_symlink() {
                        std::fs::remove_file(&target_path)?;
                    }
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                }
            }
            DestinationStatus::ConflictingFileOrDir => {
                println!(
                    "  {} Conflict at {} (File/Dir exists). Manual intervention required.",
                    icons.cross.red(),
                    target_str
                );
            }
        }
    }
    Ok(())
}

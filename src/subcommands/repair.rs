use crate::config::Config;
use crate::config::icons::Icons;
use crate::state::State;
use crate::utils::{DestinationStatus, expand_path, get_destination_status, resolve_active_profile, symlink_with_parents};
use colored::Colorize;
use indexmap::IndexMap;
use std::error::Error;
use std::path::Path;

pub fn run(config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let state = State::load()?;
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
        if let Some(active_name) = resolve_active_profile(profiles, state.active_profile.as_ref(), &cwd) {
            if let Some(profile) = profiles.get(active_name) {
                // If found via state, say "from state", else "inferred"?
                // resolve_active_profile hides where it came from.
                // But generally "Checking active profile 'name'..." is sufficient.
                println!("Checking active profile '{}'...", active_name.green());
                repair_links(&profile.links, &cwd, dry_run, &icons)?;
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

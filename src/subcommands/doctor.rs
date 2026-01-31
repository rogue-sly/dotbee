use crate::config::Config;
use crate::config::icons::Icons;
use crate::state::State;
use crate::utils::{DestinationStatus, expand_path, find_active_profile, get_destination_status, is_profile_active};
use colored::Colorize;
use indexmap::IndexMap;
use std::error::Error;
use std::path::Path;

pub fn run(config_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let state = State::load()?;
    let cwd = std::env::current_dir()?;
    let icon_style = config.settings.icon_style.unwrap_or_default();
    let icons = Icons::new(icon_style);

    println!("{}", "Dotsy Doctor Report".bold().underline());
    println!();

    if let Some(global) = &config.global {
        println!("{}", "Global Links:".blue().bold());
        check_links(&global.links, &cwd, &icons)?;
        println!();
    }

    if let Some(profiles) = &config.profiles {
        if let Some(active_name) = find_active_profile(profiles, state.active_profile.as_ref(), &cwd) {
            let is_state_backed = state.active_profile.as_ref() == Some(active_name);
            let source_label = if is_state_backed { "State" } else { "Inferred" };

            println!("Active Profile ({}): {}", source_label, active_name.cyan().bold());

            if let Some(profile) = profiles.get(active_name) {
                if is_profile_active(profile, &cwd) {
                    println!("  Status: {}", "Healthy".green());
                } else {
                    println!("  Status: {}", "Broken / Partially Applied".yellow());
                }
                println!();
                check_links(&profile.links, &cwd, &icons)?;
            } else {
                println!("  Status: Profile '{}' not found in config!", active_name.red());
            }
        } else {
            println!("No active profile detected.");
        }
    } else {
        println!("No profiles defined.");
    }

    Ok(())
}

fn check_links(links: &IndexMap<String, String>, cwd: &Path, icons: &Icons) -> Result<(), Box<dyn Error>> {
    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (target_str, source_str) in sorted_links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            println!("  {} {} (Source missing: {})", icons.error.red(), source_str, source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {
                println!("  {} {} -> {}", icons.success.green(), source_str, target_str);
            }
            DestinationStatus::ConflictingSymlink => {
                println!("  {} {} (Symlink points to wrong target)", icons.warning.yellow(), target_str);
            }
            DestinationStatus::ConflictingFileOrDir => {
                println!("  {} {} (Conflict: File/Dir exists)", icons.error.red(), target_str);
            }
            DestinationStatus::NonExistent => {
                println!("  {} {} (Not linked)", icons.info.dimmed(), source_str);
            }
        }
    }
    Ok(())
}

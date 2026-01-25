use crate::config::Config;
use crate::util::{expand_path, get_destination_status, is_profile_active, DestinationStatus};
use colored::Colorize;
use std::error::Error;
use std::path::Path;

pub fn run(config_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let cwd = std::env::current_dir()?;

    println!("{}", "Dotsy Doctor Report".bold().underline());
    println!();

    if let Some(global) = &config.global {
        println!("{}", "Global Links:".blue().bold());
        check_links(&global.links, &cwd)?;
        println!();
    }

    if let Some(profiles) = &config.profiles {
        let mut active_found = false;
        for (name, profile) in profiles {
            if is_profile_active(profile, &cwd) {
                println!("Active Profile: {}", name.green().bold());
                check_links(&profile.links, &cwd)?;
                println!();
                active_found = true;
            }
        }

        if !active_found {
            println!("No fully active profile detected.");
        }
    } else {
        println!("No profiles defined.");
    }

    Ok(())
}

fn check_links(links: &std::collections::HashMap<String, String>, cwd: &Path) -> Result<(), Box<dyn Error>> {
    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (target_str, source_str) in sorted_links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            println!("  {} {} (Source missing: {})", "❌".red(), source_str, source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {
                println!("  {} {} -> {}", "✅".green(), source_str, target_str);
            }
            DestinationStatus::ConflictingSymlink => {
                println!("  {} {} (Symlink points to wrong target)", "⚠️ ".yellow(), target_str);
            }
            DestinationStatus::ConflictingFileOrDir => {
                println!("  {} {} (Conflict: File/Dir exists)", "🚫".red(), target_str);
            }
            DestinationStatus::NonExistent => {
                println!("  {} {} (Not linked)", "⚪".dimmed(), source_str);
            }
        }
    }
    Ok(())
}

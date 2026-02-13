use colored::Colorize;
use context::Context;
use indexmap::IndexMap;
use std::error::Error;
use utils::{expand_tilde, get_destination_status, DestinationStatus};

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let message = &context.message;

    println!("{}", "Dotsy Doctor Report\n".bold().underline());

    // Check global symlinks
    if let Some(global) = &context.config.global {
        println!("{}", "Global Links:".bold());
        check_links(&global.links, context)?;
    }

    // Check current profile symlinks
    let profiles = match &context.config.profiles {
        Some(profiles) => profiles,
        None => {
            message.info("No profiles defined in dotsy.toml.");
            return Ok(());
        }
    };

    let active_profile = match &context.state.active_profile {
        Some(active_profile) => active_profile,
        None => {
            message.info("No active profile detected.");
            return Ok(());
        }
    };

    match profiles.get(active_profile) {
        Some(profile) => {
            println!("{} ({}){}", "Active Profile".bold(), active_profile.cyan().bold(), ":".bold());
            check_links(&profile.links, context)?
        }
        None => {
            message.error(&format!("Status: Profile '{}' not found in config!", active_profile.red()));
            std::process::exit(1)
        }
    }

    Ok(())
}

fn check_links(links: &IndexMap<String, String>, context: &Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let msg = &context.message;

    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (target_str, source_str) in sorted_links {
        let source_path = cwd.join(source_str);
        let target_path = expand_tilde(target_str);

        if !source_path.exists() {
            msg.error(&format!("{} (Source missing: {})", source_str, source_path.display()));
            continue;
        }

        let status = get_destination_status(&source_path, &target_path);

        match status {
            DestinationStatus::AlreadyLinked => {
                msg.success(&format!("{} -> {}", source_str, target_str));
            }
            DestinationStatus::ConflictingSymlink => {
                msg.warning(&format!("{} (Symlink points to wrong target)", target_str));
            }
            DestinationStatus::ConflictingFileOrDir => {
                msg.error(&format!("{} (Conflict: File/Dir exists)", target_str));
            }
            DestinationStatus::NonExistent => {
                msg.warning(&format!("{} (Not linked)", source_str));
            }
        }
    }
    println!();

    Ok(())
}

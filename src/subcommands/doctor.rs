use colored::Colorize;
use dotsy::context::Context;
use dotsy::utils::{DestinationStatus, expand_path, find_active_profile, get_destination_status, is_profile_active};
use indexmap::IndexMap;
use std::error::Error;
use std::path::Path;

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let message = &context.message;

    println!("{}", "Dotsy Doctor Report".bold().underline());
    println!();

    if let Some(global) = &context.config.global {
        println!("{}", "Global Links:".blue().bold());
        check_links(&global.links, &cwd, context)?;
        println!();
    }

    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = find_active_profile(profiles, context.state.active_profile.as_ref(), &cwd) {
            let is_state_backed = context.state.active_profile.as_ref() == Some(active_name);
            let source_label = if is_state_backed { "State" } else { "Inferred" };

            message.info(&format!("Active Profile ({}): {}", source_label, active_name.cyan().bold()));

            if let Some(profile) = profiles.get(active_name) {
                if is_profile_active(profile, &cwd) {
                    message.success("Status: Healthy");
                } else {
                    message.warning("Status: Broken / Partially Applied");
                }
                println!();
                check_links(&profile.links, &cwd, &context)?;
            } else {
                message.error(&format!("Status: Profile '{}' not found in config!", active_name.red()));
            }
        } else {
            message.info("No active profile detected.");
        }
    } else {
        message.info("No profiles defined.");
    }

    Ok(())
}

fn check_links(links: &IndexMap<String, String>, cwd: &Path, context: &Context) -> Result<(), Box<dyn Error>> {
    let message = &context.message;

    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (target_str, source_str) in sorted_links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            message.error(&format!("{} (Source missing: {})", source_str, source_path.display()));
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {
                message.success(&format!("{} -> {}", source_str, target_str));
            }
            DestinationStatus::ConflictingSymlink => {
                message.warning(&format!("{} (Symlink points to wrong target)", target_str));
            }
            DestinationStatus::ConflictingFileOrDir => {
                message.error(&format!("{} (Conflict: File/Dir exists)", target_str));
            }
            DestinationStatus::NonExistent => {
                message.info(&format!("{} (Not linked)", source_str));
            }
        }
    }
    Ok(())
}

use crate::context::Context;
use crate::utils::{find_active_profile, unlink_profile_links};
use colored::Colorize;
use std::error::Error;

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let icons = &context.icons;

    if context.dry_run {
        println!("{}", "Purging active links (dry run)...".bold().red());
    } else {
        println!("{}", "Purging active links...".bold().red());
    }

    if let Some(global) = &context.config.global {
        println!("Unlinking global links...");
        unlink_profile_links(&global.links, &cwd, context.dry_run, icons)?;
    }

    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = find_active_profile(profiles, context.state.active_profile.as_ref(), &cwd) {
            if let Some(profile) = profiles.get(active_name) {
                println!("Unlinking active profile '{}'...", active_name.yellow());
                unlink_profile_links(&profile.links, &cwd, context.dry_run, icons)?;
            } else {
                println!("Active profile '{}' not found in config. Skipping.", active_name);
            }
        } else {
            // If resolve returns None, nothing to purge.
        }
    }

    if context.dry_run {
        println!("{}", "Purge dry run complete.".green());
    } else {
        context.state.clear_active_profile()?;
        println!("{}", "Purge complete.".green());
    }

    Ok(())
}

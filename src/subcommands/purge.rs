use crate::config::Config;
use crate::config::icons::Icons;
use crate::state::State;
use crate::util::{resolve_active_profile, unlink_profile_links};
use colored::Colorize;
use std::error::Error;

pub fn run(config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let mut state = State::load()?;
    let cwd = std::env::current_dir()?;
    let icon_style = config.settings.icon_style.as_deref().unwrap_or("nerdfonts");
    let icons = Icons::new(icon_style);

    if dry_run {
        println!("{}", "Purging active links (dry run)...".bold().red());
    } else {
        println!("{}", "Purging active links...".bold().red());
    }

    if let Some(global) = &config.global {
        println!("Unlinking global links...");
        unlink_profile_links(&global.links, &cwd, dry_run, &icons)?;
    }

    if let Some(profiles) = &config.profiles {
        if let Some(active_name) = resolve_active_profile(profiles, state.active_profile.as_ref(), &cwd) {
            if let Some(profile) = profiles.get(active_name) {
                println!("Unlinking active profile '{}'...", active_name.yellow());
                unlink_profile_links(&profile.links, &cwd, dry_run, &icons)?;
            } else {
                println!("Active profile '{}' not found in config. Skipping.", active_name);
            }
        } else {
            // If resolve returns None, nothing to purge.
        }
    }

    if dry_run {
        println!("{}", "Purge dry run complete.".green());
    } else {
        state.clear_active_profile()?;
        println!("{}", "Purge complete.".green());
    }

    Ok(())
}

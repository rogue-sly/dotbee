use crate::config::Config;
use crate::config::Icons;
use crate::util::{is_profile_active, unlink_profile_links};
use colored::Colorize;
use std::error::Error;

pub fn run(config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
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
        for (name, profile) in profiles {
            if is_profile_active(profile, &cwd) {
                println!("Unlinking profile '{}'...", name.yellow());
                unlink_profile_links(&profile.links, &cwd, dry_run, &icons)?;
            }
        }
    }

    if dry_run {
        println!("{}", "Purge dry run complete.".green());
    } else {
        println!("{}", "Purge complete.".green());
    }

    Ok(())
}

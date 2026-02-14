use colored::Colorize;
use crate::context::Context;
use indexmap::IndexMap;
use std::error::Error;
use crate::utils::{expand_tilde, get_destination_status, DestinationStatus};

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let message = &context.message;

    println!("{}", "Dotsy Doctor Report\n".bold().underline());

    let mut config_links = indexmap::IndexMap::new();

    // Check global symlinks
    if let Some(global) = &context.config.global {
        println!("{}", "Global Links:".bold());
        for (k, v) in &global.links {
            config_links.insert(k.clone(), v.clone());
        }
        check_links(&global.links, context)?;
    }

    // Check current profile symlinks
    let profiles = match &context.config.profiles {
        Some(profiles) => profiles,
        None => {
            message.info("No profiles defined in dotsy.toml.");
            check_ghost_links(&config_links, context)?;
            return Ok(());
        }
    };

    let active_profile = match &context.state.active_profile {
        Some(active_profile) => active_profile,
        None => {
            message.info("No active profile detected.");
            check_ghost_links(&config_links, context)?;
            return Ok(());
        }
    };

    match profiles.get(active_profile) {
        Some(profile) => {
            println!("{} ({}){}", "Active Profile".bold(), active_profile.cyan().bold(), ":".bold());
            for (k, v) in &profile.links {
                config_links.insert(k.clone(), v.clone());
            }
            check_links(&profile.links, context)?
        }
        None => {
            message.error(&format!("Status: Profile '{}' not found in config!", active_profile.red()));
            check_ghost_links(&config_links, context)?;
            std::process::exit(1)
        }
    }

    check_ghost_links(&config_links, context)?;

    Ok(())
}

fn check_ghost_links(config_links: &IndexMap<String, String>, context: &Context) -> Result<(), Box<dyn Error>> {
    let mut ghosts = Vec::new();
    for managed in &context.state.managed_links {
        if !config_links.contains_key(&managed.target) {
            ghosts.push(managed);
        }
    }

    if !ghosts.is_empty() {
        println!("{}", "Ghost Links (in state but not in current config):".bold().yellow());
        for ghost in ghosts {
            context.message.warning(&format!(
                "{} (formerly linked to {})",
                ghost.target, ghost.source
            ));
        }
        println!("{}", "\nRun 'dotsy switch' to clean up ghost links.".italic().dimmed());
    }

    Ok(())
}

fn check_links(links: &IndexMap<String, String>, context: &Context) -> Result<(), Box<dyn Error>> {
    let dotfiles_root = context
        .state
        .dotfiles_path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));
    let msg = &context.message;

    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (target_str, source_str) in sorted_links {
        let source_path = dotfiles_root.join(source_str);
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

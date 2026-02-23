use crate::context::Context;
use crate::context::manager::symlink::SymlinkStatus;
use crate::utils::expand_tilde;
use colored::Colorize;
use indexmap::IndexMap;
use std::error::Error;

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let message = &context.message;

    println!("{}", "Dotbee Doctor Report\n".bold().underline());

    let mut config_links: IndexMap<String, String> = indexmap::IndexMap::new();

    // Check global symlinks
    if let Some(global_links) = context.manager.config.get_global_links() {
        println!("{}", "Global Links:".bold());
        for (k, v) in global_links {
            config_links.insert(k.clone(), v.clone());
        }
        check_links(global_links, context)?;
    }

    // Check current profile symlinks
    let active_profile = match context.manager.state.get_active_profile() {
        Some(p) => p.to_string(),
        None => {
            message.info("No active profile detected.");
            check_ghost_links(&config_links, context)?;
            return Ok(());
        }
    };

    if !context.manager.config.has_profiles() {
        message.info("No profiles defined in dotbee.toml.");
        check_ghost_links(&config_links, context)?;
        return Ok(());
    }

    match context.manager.config.get_profile(&active_profile) {
        Ok(profile) => {
            println!("{} ({}){}", "Active Profile".bold(), active_profile.cyan().bold(), ":".bold());
            for (k, v) in &profile.links {
                config_links.insert(k.clone(), v.clone());
            }
            check_links(&profile.links, context)?
        }
        Err(_) => {
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
    for managed in context.manager.state.get_managed_links() {
        if !config_links.contains_key(&managed.target) {
            ghosts.push(managed);
        }
    }

    if !ghosts.is_empty() {
        println!("{}", "Ghost Links (in state but not in current config):".bold().yellow());
        for ghost in ghosts {
            context
                .message
                .warning(&format!("{} (formerly linked to {})", ghost.target, ghost.source));
        }
        println!("{}", "\nRun 'dotbee switch' to clean up ghost links.".italic().dimmed());
    }

    Ok(())
}

fn check_links(links: &IndexMap<String, String>, context: &Context) -> Result<(), Box<dyn Error>> {
    let dotfiles_root = context
        .manager
        .state
        .get_dotfiles_path()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));
    let message = &context.message;

    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (target_str, source_str) in sorted_links {
        let source_path = dotfiles_root.join(source_str);
        let target_path = expand_tilde(target_str);

        if !source_path.exists() {
            message.error(&format!("{} (Source missing: {})", source_str, source_path.display()));
            continue;
        }

        let status = context.manager.symlink.check(&source_path, &target_path);

        match status {
            SymlinkStatus::AlreadyLinked => {
                message.success(&format!("{} -> {}", source_str, target_str));
            }
            SymlinkStatus::ConflictingSymlink => {
                message.warning(&format!("{} (Symlink points to wrong target)", target_str));
            }
            SymlinkStatus::ConflictingFileOrDir => {
                message.error(&format!("{} (Conflict: File/Dir exists)", target_str));
            }
            SymlinkStatus::NonExistent => {
                message.warning(&format!("{} (Not linked)", source_str));
            }
        }
    }
    println!();

    Ok(())
}

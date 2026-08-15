use crate::context::config::Link;
use crate::context::symlink::SymlinkStatus;
use crate::utils::common::expand_tilde;
use crate::utils::message;
use anyhow::bail;
use colored::Colorize;
use indexmap::IndexMap;

pub fn run(context: &crate::context::Context) -> anyhow::Result<(), anyhow::Error> {
    println!("{}", "Dotbee Doctor Report\n".bold().underline());

    let mut config_links: IndexMap<String, Link> = indexmap::IndexMap::new();

    // check global symlinks
    if let Some(global_links) = context.config.get_global_links() {
        println!("{}", "Global Links:".bold());
        for (name, link) in global_links {
            config_links.insert(name.clone(), link.clone());
        }
        check_links(global_links, context)?;
    }

    // check current profile symlinks
    let active_profile = match context.state.get_active_profile() {
        Some(p) => p.to_string(),
        None => {
            message::info("No active profile detected.");
            check_ghost_links(&config_links, context)?;
            return Ok(());
        }
    };

    if !context.config.has_profiles() {
        message::info("No profiles defined in dotbee.toml.");
        check_ghost_links(&config_links, context)?;
        return Ok(());
    }

    match context.config.get_profile(&active_profile) {
        Ok(profile) => {
            println!("{} ({}){}", "Active Profile".bold(), active_profile.cyan().bold(), ":".bold());
            for (name, link) in &profile.links {
                config_links.insert(name.clone(), link.clone());
            }
            check_links(&profile.links, context)?
        }
        Err(_) => {
            message::error(&format!("Status: Profile '{}' not found in config!", active_profile.red()));
            check_ghost_links(&config_links, context)?;
            bail!(
                "Profile '{}' not found. Update your config or run 'dotbee sync --profile <profile>' to select a different profile.",
                active_profile
            )
        }
    }

    check_ghost_links(&config_links, context)?;

    Ok(())
}

fn check_ghost_links(config_links: &IndexMap<String, Link>, context: &crate::context::Context) -> anyhow::Result<(), anyhow::Error> {
    let mut ghosts = Vec::new();
    for link in context.state.get_links() {
        if !config_links.values().any(|l| l.dst == link.target) {
            ghosts.push(link);
        }
    }

    if !ghosts.is_empty() {
        println!("{}", "Ghost Links (in state but not in current config):".bold().yellow());
        for ghost in ghosts {
            message::warning(&format!("{} (formerly linked to {})", ghost.target, ghost.source));
        }
        println!("{}", "\nRun 'dotbee sync' to clean up ghost links.".italic().dimmed());
    }

    Ok(())
}

fn check_links(links: &IndexMap<String, Link>, context: &crate::context::Context) -> anyhow::Result<(), anyhow::Error> {
    let dotfiles_root = context.state.get_dotfiles_root()?;

    let mut sorted_links: Vec<_> = links.iter().collect();
    sorted_links.sort_by_key(|(k, _)| k.as_str());

    for (_name, link) in sorted_links {
        let source_path = dotfiles_root.join(&link.src);
        let target_path = expand_tilde(&link.dst);

        if !source_path.exists() {
            message::error(&format!("{} (Source missing: {})", link.src, source_path.display()));
            continue;
        }

        let status = context.symlink.check(&source_path, &target_path);

        match status {
            SymlinkStatus::AlreadyLinked => {
                message::success(&format!("{} -> {}", link.src, link.dst));
            }
            SymlinkStatus::ConflictingSymlink => {
                message::warning(&format!("{} (Symlink points to wrong target)", link.dst));
            }
            SymlinkStatus::ConflictingFileOrDir => {
                message::error(&format!("{} (Conflict: File/Dir exists)", link.dst));
            }
            SymlinkStatus::NonExistent => {
                message::warning(&format!("{} (Not linked)", link.src));
            }
        }
    }
    println!();

    Ok(())
}

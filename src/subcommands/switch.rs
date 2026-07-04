use crate::{
    context::{config::ConflictAction, symlink::SymlinkStatus},
    utils::{
        common::{expand_tilde, get_hostname},
        message,
    },
};

use anyhow::{Context, bail};
use colored::Colorize;
use indexmap::IndexMap;
use std::{
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
};

pub enum ConflictKind {
    Symlink,
    FileOrDir,
}

impl Display for ConflictKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConflictKind::Symlink => "Symlink",
                ConflictKind::FileOrDir => "File/Dir",
            }
        )
    }
}

pub fn run(profile_name: Option<String>, context: &mut crate::context::Context) -> anyhow::Result<(), anyhow::Error> {
    let target_profile = match profile_name {
        Some(name) => name,
        None => {
            if !context.config.get_settings().auto_detect_profile.unwrap_or_default() {
                bail!("No profile specified and auto_detect_profile is disabled.");
            }

            let hostname = get_hostname().context("Failed to auto-detect profile from hostname")?;
            message::info(&format!(
                "No profile specified. Auto-detecting profile from hostname: '{}'",
                hostname
            ));

            hostname
        }
    };

    let dotfiles_root = context.state.get_dotfiles_root()?;
    let dry_run = context.dry_run;

    // build desired links map from config
    let mut desired_links: IndexMap<String, String> = IndexMap::new();

    if let Some(global_links) = context.config.get_global_links() {
        for (k, v) in global_links {
            desired_links.insert(k.clone(), v.clone());
        }
    }

    let profile = context.config.get_profile(&target_profile)?;
    for (k, v) in &profile.links {
        desired_links.insert(k.clone(), v.clone());
    }

    // step 1: remove ghost links (links in state but not in desired config)
    // collect state links upfront to avoid borrow conflicts with remove_links
    let state_links: Vec<(String, String, bool)> = context
        .state
        .get_links()
        .iter()
        .map(|l| (l.target.clone(), l.source.clone(), l.is_dir))
        .collect();

    for (target, source, _is_dir) in &state_links {
        if !desired_links.contains_key(target) {
            let target_path = expand_tilde(target);
            let source_path = dotfiles_root.join(source);

            if target_path.is_symlink() && fs::read_link(&target_path)? == source_path {
                if dry_run {
                    message::delete(&format!("Would remove ghost link (missing from config): {}", target));
                } else {
                    fs::remove_file(&target_path)?;
                    message::delete(&format!("Removed ghost link: {}", target));
                    context.state.remove_links(|l| l.target == *target)?;
                }
            }
        }
    }

    // print header for dry run
    if dry_run {
        println!(
            "{} {} {}",
            "Switching to profile".yellow(),
            target_profile.bold().cyan(),
            "(dry run)".yellow()
        );
    }

    // step 2: process desired links
    for (target_str, source_str) in &desired_links {
        let source_path = dotfiles_root.join(source_str);
        let target_path = expand_tilde(target_str);

        if !source_path.exists() {
            message::error(&format!("Source missing: {}", source_str));
            continue;
        }

        let status = context.symlink.check(&source_path, &target_path);
        let is_dir = source_path.is_dir();

        match status {
            SymlinkStatus::AlreadyLinked => {
                message::success(&format!("{} -> {} (already linked)", source_str, target_str));
                if !dry_run {
                    context.state.add_link(source_str.clone(), target_str.clone(), is_dir)?;
                }
            }
            SymlinkStatus::NonExistent => {
                if dry_run {
                    message::link(&format!("Would link {} -> {}", source_str, target_str));
                } else {
                    context.symlink.create(&source_path, &target_path)?;
                    message::link(&format!("{} -> {}", source_str, target_str));
                    context.state.add_link(source_str.clone(), target_str.clone(), is_dir)?;
                }
            }
            SymlinkStatus::ConflictingSymlink | SymlinkStatus::ConflictingFileOrDir => {
                let kind = match status {
                    SymlinkStatus::ConflictingSymlink => ConflictKind::Symlink,
                    _ => ConflictKind::FileOrDir,
                };

                if dry_run {
                    message::warning(&format!("Conflict at {}: {} exists. Strategy will be applied.", target_str, kind));
                    message::info(&format!("  Source: {}", source_str));
                } else {
                    let strategy = &context.config.get_settings().on_conflict;
                    let action = match strategy {
                        None => {
                            message::error(&format!("Conflict: {} -> {} ({})", source_str, target_str, kind));
                            ConflictAction::prompt(&kind)?
                        }
                        Some(a) => a.clone(),
                    };

                    handle_conflict(&action, &source_path, &target_path, source_str, context)?;

                    if action == ConflictAction::Overwrite || action == ConflictAction::Adopt {
                        let is_dir = source_path.is_dir();
                        context.state.add_link(source_str.clone(), target_str.clone(), is_dir)?;
                    }
                }
            }
        }
    }

    if !dry_run {
        context.state.set_active_profile(target_profile.clone())?;
        message::success(&format!("Switched to profile '{}'", target_profile));
    }

    Ok(())
}

fn handle_conflict(
    action: &ConflictAction,
    source: &Path,
    destination: &PathBuf,
    rel_source: &str,
    context: &crate::context::Context,
) -> anyhow::Result<(), anyhow::Error> {
    let dotfiles_root = context.state.get_dotfiles_root()?;

    match action {
        ConflictAction::Skip => println!("  Skipped {}", destination.display()),
        ConflictAction::Abort => bail!("Operation aborted by user."),
        ConflictAction::Overwrite => {
            if let Err(e) = fs::remove_file(destination)
                && e.kind() == std::io::ErrorKind::IsADirectory
            {
                fs::remove_dir_all(destination).with_context(|| format!("Failed to remove directory at {:?}", destination))?;
            }
            context.symlink.create(source, destination)?;
            println!("  Overwrite: {} → {}", source.display(), destination.display());
        }
        ConflictAction::Adopt => {
            let adopt_target = dotfiles_root.join(rel_source);
            if let Some(parent) = adopt_target.parent() {
                fs::create_dir_all(parent).with_context(|| format!("Failed to create directory at {:?}", parent))?;
            }

            if let Err(e) = fs::remove_file(&adopt_target)
                && e.kind() == std::io::ErrorKind::IsADirectory
            {
                fs::remove_dir_all(&adopt_target).with_context(|| format!("Failed to remove directory at {:?}", adopt_target))?;
            }

            fs::rename(destination, &adopt_target).with_context(|| format!("Failed to move {:?} to {:?}", destination, adopt_target))?;
            context.symlink.create(source, destination)?;
            println!("  Adopted: {} → {}", source.display(), destination.display());
        }
    }

    Ok(())
}

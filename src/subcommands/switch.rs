use colored::Colorize;
use crate::config::ConflictAction;
use crate::context::Context;
use indexmap::IndexMap;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::utils::{DestinationStatus, expand_tilde, get_destination_status, get_hostname, symlink_with_parents};

/// Actions for the switch command.
pub enum SwitchAction {
    UnlinkGhost {
        target_display: String,
        target_path: PathBuf,
    },
    LinkNew {
        source_display: String,
        target_display: String,
        source_path: PathBuf,
        target_path: PathBuf,
        is_dir: bool,
    },
    UpdateState {
        source_display: String,
        target_display: String,
        is_dir: bool,
    },
    Conflict {
        source_display: String,
        target_display: String,
        source_path: PathBuf,
        target_path: PathBuf,
        kind: String, // "Symlink" or "File/Dir"
    },
    SourceMissing {
        source_display: String,
        _source_path: PathBuf,
    },
}

pub fn run(profile_name: Option<String>, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let target_profile = match profile_name {
        Some(name) => name,
        None => {
            if !context.config.settings.auto_detect_profile.unwrap_or(false) {
                return Err("No profile specified and auto_detect_profile is disabled.".into());
            }

            let hostname = get_hostname();
            context.message.info(&format!(
                "No profile specified. Auto-detecting profile from hostname: '{}'",
                hostname
            ));

            hostname
        }
    };

    // 1. GENERATE THE PLAN
    let plan = generate_plan(&target_profile, context)?;

    // 2. DISPATCH
    if context.dry_run {
        execute_dry(&plan, &target_profile, context);
    } else {
        execute(plan, &target_profile, context)?;
    }

    Ok(())
}

fn generate_plan(target_profile: &str, context: &Context) -> Result<Vec<SwitchAction>, Box<dyn Error>> {
    let mut plan = Vec::new();
    let dotfiles_root = context
        .state
        .dotfiles_path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    // 1. Resolve all links that SHOULD exist in the target configuration
    let mut desired_links = indexmap::IndexMap::new();

    if let Some(global) = &context.config.global {
        for (k, v) in &global.links {
            desired_links.insert(k.clone(), v.clone());
        }
    }

    if let Some(profiles) = &context.config.profiles {
        if let Some(profile) = profiles.get(target_profile) {
            for (k, v) in &profile.links {
                desired_links.insert(k.clone(), v.clone());
            }
        } else {
            return Err(format!("Profile '{}' not found in configuration.", target_profile).into());
        }
    } else {
        return Err("No profiles defined in config.".into());
    }

    // 2. Phase A: Identify Ghost Links (links in state but not in desired config)
    for managed in &context.state.managed_links {
        if !desired_links.contains_key(&managed.target) {
            let target_path = expand_tilde(&managed.target);
            let source_path = dotfiles_root.join(&managed.source);

            // Safety check: Only unlink if it actually points to our source
            if target_path.is_symlink() && fs::read_link(&target_path)? == source_path {
                plan.push(SwitchAction::UnlinkGhost {
                    target_display: managed.target.clone(),
                    target_path,
                });
            }
        }
    }

    // 3. Phase B: Process desired links
    for (target_str, source_str) in desired_links {
        let source_path = dotfiles_root.join(&source_str);
        let target_path = expand_tilde(&target_str);

        if !source_path.exists() {
            plan.push(SwitchAction::SourceMissing {
                source_display: source_str.clone(),
                _source_path: source_path,
            });
            continue;
        }

        let status = get_destination_status(&source_path, &target_path);
        let is_dir = source_path.is_dir();

        match status {
            DestinationStatus::AlreadyLinked => {
                plan.push(SwitchAction::UpdateState {
                    source_display: source_str.clone(),
                    target_display: target_str.clone(),
                    is_dir,
                });
            }
            DestinationStatus::NonExistent => {
                plan.push(SwitchAction::LinkNew {
                    source_display: source_str.clone(),
                    target_display: target_str.clone(),
                    source_path,
                    target_path,
                    is_dir,
                });
            }
            DestinationStatus::ConflictingSymlink | DestinationStatus::ConflictingFileOrDir => {
                let kind = if status == DestinationStatus::ConflictingSymlink {
                    "Symlink"
                } else {
                    "File/Dir"
                };

                plan.push(SwitchAction::Conflict {
                    source_display: source_str.clone(),
                    target_display: target_str.clone(),
                    source_path,
                    target_path,
                    kind: kind.to_string(),
                });
            }
        }
    }

    Ok(plan)
}

fn execute_dry(plan: &[SwitchAction], target_profile: &str, context: &Context) {
    let msg = &context.message;
    println!(
        "{} {} {}",
        "Switching to profile".yellow(),
        target_profile.bold().cyan(),
        "(dry run)".yellow()
    );

    for action in plan {
        match action {
            SwitchAction::UnlinkGhost { target_display, .. } => {
                msg.delete(&format!("Would unlink ghost (missing from config): {}", target_display));
            }
            SwitchAction::LinkNew {
                source_display,
                target_display,
                ..
            } => {
                msg.link(&format!("Would link {} -> {}", source_display, target_display));
            }
            SwitchAction::UpdateState {
                source_display,
                target_display,
                ..
            } => {
                msg.success(&format!("{} -> {} (already linked)", source_display, target_display));
            }
            SwitchAction::Conflict {
                source_display,
                target_display,
                kind,
                ..
            } => {
                msg.warning(&format!(
                    "Conflict at {}: {} exists. Strategy will be applied.",
                    target_display, kind
                ));
                msg.info(&format!("  Source: {}", source_display));
            }
            SwitchAction::SourceMissing { source_display, .. } => {
                msg.error(&format!("Source missing: {}", source_display));
            }
        }
    }
}

fn execute(plan: Vec<SwitchAction>, target_profile: &str, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;
    let strategy = context.config.settings.on_conflict.clone();

    for action in plan {
        match action {
            SwitchAction::UnlinkGhost {
                target_display,
                target_path,
            } => {
                fs::remove_file(&target_path)?;
                msg.delete(&format!("Unlinked ghost: {}", target_display));
                context.state.managed_links.retain(|l| l.target != target_display);
            }
            SwitchAction::LinkNew {
                source_display,
                target_display,
                source_path,
                target_path,
                is_dir,
            } => {
                symlink_with_parents(&source_path, &target_path, context)?;
                msg.link(&format!("{} -> {}", source_display, target_display));
                context.state.add_managed_link(source_display, target_display, is_dir);
            }
            SwitchAction::UpdateState {
                source_display,
                target_display,
                is_dir,
            } => {
                msg.success(&format!("{} -> {} (already linked)", source_display, target_display));
                context.state.add_managed_link(source_display, target_display, is_dir);
            }
            SwitchAction::Conflict {
                source_display,
                target_display,
                source_path,
                target_path,
                kind,
            } => {
                let action = match &strategy {
                    Some(ConflictAction::Ask) | None => {
                        msg.error(&format!("Conflict: {} -> {} ({})", source_display, target_display, kind));
                        ConflictAction::prompt(&kind).unwrap()
                    }
                    Some(a) => a.clone(),
                };

                handle_conflict(action.clone(), &source_path, &target_path, &source_display, context)?;

                if action == ConflictAction::Overwrite || action == ConflictAction::Adopt {
                    let is_dir = source_path.is_dir();
                    context.state.add_managed_link(source_display, target_display, is_dir);
                }
            }
            SwitchAction::SourceMissing { source_display, .. } => {
                msg.error(&format!("Source missing: {}", source_display));
            }
        }
    }

    context.state.set_active_profile(target_profile.to_string())?;
    msg.success(&format!("Switched to profile '{}'", target_profile));

    Ok(())
}

fn handle_conflict(
    action: ConflictAction,
    source: &Path,
    destination: &PathBuf,
    rel_source: &str,
    context: &Context,
) -> Result<(), Box<dyn Error>> {
    let dotfiles_root = context
        .state
        .dotfiles_path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    match action {
        ConflictAction::Skip => println!("  Skipped {}", destination.display()),
        ConflictAction::Abort => return Err("Operation aborted by user.".into()),
        ConflictAction::Overwrite => {
            if destination.is_symlink() || destination.is_file() || destination.is_dir() {
                #[cfg(not(target_os = "android"))]
                trash::delete(destination)?;
                #[cfg(target_os = "android")]
                if destination.is_dir() {
                    fs::remove_dir_all(destination).unwrap();
                } else {
                    fs::remove_file(destination).unwrap();
                }
            }
            symlink_with_parents(source, destination, context).unwrap();
            println!("  Overwrite: {} → {}", source.display(), destination.display());
        }
        ConflictAction::Adopt => {
            let adopt_target = dotfiles_root.join(rel_source);
            if let Some(parent) = adopt_target.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            if adopt_target.exists() {
                #[cfg(not(target_os = "android"))]
                trash::delete(&adopt_target).unwrap();
                #[cfg(target_os = "android")]
                if adopt_target.is_dir() {
                    fs::remove_dir_all(&adopt_target).unwrap();
                } else {
                    fs::remove_file(&adopt_target).unwrap();
                }
            }
            fs::rename(destination, &adopt_target).unwrap();
            symlink_with_parents(source, destination, context).unwrap();
            println!("  Adopted: {} → {}", source.display(), destination.display());
        }
        ConflictAction::Ask => {
            panic!("'Ask' action should have been resolved before handling conflict")
        }
    }

    Ok(())
}

pub fn remove_profile_links(links: &IndexMap<String, String>, context: &Context) -> Result<(), Box<dyn Error>> {
    let dotfiles_root = context
        .state
        .dotfiles_path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    for (target_str, source_str) in links {
        let target_path = expand_tilde(target_str);
        let source_path = dotfiles_root.join(source_str);

        if target_path.is_symlink() && fs::read_link(&target_path)? == source_path {
            if context.dry_run {
                context.message.delete(&format!("Would unlink {} (dry run)", target_str));
            } else {
                fs::remove_file(&target_path)?;
                context.message.delete(&format!("Unlinked {}", target_str));
            }
        }
    }
    Ok(())
}

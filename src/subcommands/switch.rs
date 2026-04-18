use crate::context::{
    Context,
    manager::{config::ConflictAction, symlink::SymlinkStatus},
};
use colored::Colorize;
use indexmap::IndexMap;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::utils::{expand_tilde, get_hostname};

/// Actions for the switch command.
/// These are possible list of actions that
/// the switch command might do.
pub enum Action {
    RemoveGhostLink {
        target_display: String,
        target_path: PathBuf,
    },
    CreateNewLink {
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
            if !context.manager.config.get_settings().auto_detect_profile.unwrap_or(false) {
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

fn generate_plan(target_profile: &str, context: &Context) -> Result<Vec<Action>, Box<dyn Error>> {
    let mut plan = Vec::new();
    let dotfiles_root = context
        .manager
        .state
        .get_dotfiles_path()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    // 1. Resolve all links that SHOULD exist in the target configuration
    let mut desired_links: IndexMap<String, String> = indexmap::IndexMap::new();

    if let Some(global_links) = context.manager.config.get_global_links() {
        for (k, v) in global_links {
            desired_links.insert(k.clone(), v.clone());
        }
    }

    let profile = context.manager.config.get_profile(target_profile)?;
    for (k, v) in &profile.links {
        desired_links.insert(k.clone(), v.clone());
    }

    // 2. Phase A: Identify Ghost Links (links in state but not in desired config)
    for link in context.manager.state.get_links() {
        if !desired_links.contains_key(&link.target) {
            let target_path = expand_tilde(&link.target);
            let source_path = dotfiles_root.join(&link.source);

            // Safety check: Only remove if it actually points to our source
            if target_path.is_symlink() && fs::read_link(&target_path)? == source_path {
                plan.push(Action::RemoveGhostLink {
                    target_display: link.target.clone(),
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
            plan.push(Action::SourceMissing {
                source_display: source_str.clone(),
                _source_path: source_path,
            });
            continue;
        }

        let status = context.manager.symlink.check(&source_path, &target_path);
        let is_dir = source_path.is_dir();

        match status {
            SymlinkStatus::AlreadyLinked => {
                plan.push(Action::UpdateState {
                    source_display: source_str.clone(),
                    target_display: target_str.clone(),
                    is_dir,
                });
            }
            SymlinkStatus::NonExistent => {
                plan.push(Action::CreateNewLink {
                    source_display: source_str.clone(),
                    target_display: target_str.clone(),
                    source_path,
                    target_path,
                    is_dir,
                });
            }
            SymlinkStatus::ConflictingSymlink | SymlinkStatus::ConflictingFileOrDir => {
                let kind = if status == SymlinkStatus::ConflictingSymlink {
                    "Symlink"
                } else {
                    "File/Dir"
                };

                plan.push(Action::Conflict {
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

fn execute_dry(plan: &[Action], target_profile: &str, context: &Context) {
    let msg = &context.message;
    println!(
        "{} {} {}",
        "Switching to profile".yellow(),
        target_profile.bold().cyan(),
        "(dry run)".yellow()
    );

    for action in plan {
        match action {
            Action::RemoveGhostLink { target_display, .. } => {
                msg.delete(&format!("Would remove ghost link (missing from config): {}", target_display));
            }
            Action::CreateNewLink {
                source_display,
                target_display,
                ..
            } => {
                msg.link(&format!("Would link {} -> {}", source_display, target_display));
            }
            Action::UpdateState {
                source_display,
                target_display,
                ..
            } => {
                msg.success(&format!("{} -> {} (already linked)", source_display, target_display));
            }
            Action::Conflict {
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
            Action::SourceMissing { source_display, .. } => {
                msg.error(&format!("Source missing: {}", source_display));
            }
        }
    }
}

fn execute(plan: Vec<Action>, target_profile: &str, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;
    let strategy = &context.manager.config.get_settings().on_conflict;

    for action in plan {
        match action {
            Action::RemoveGhostLink {
                target_display,
                target_path,
            } => {
                fs::remove_file(&target_path)?;
                msg.delete(&format!("Removed ghost link: {}", target_display));
                context.manager.state.remove_links(|l| l.target == target_display)?;
            }
            Action::CreateNewLink {
                source_display,
                target_display,
                source_path,
                target_path,
                is_dir,
            } => {
                context.manager.symlink.create(&source_path, &target_path)?;
                msg.link(&format!("{} -> {}", source_display, target_display));
                context.manager.state.add_link(source_display, target_display, is_dir)?;
            }
            Action::UpdateState {
                source_display,
                target_display,
                is_dir,
            } => {
                msg.success(&format!("{} -> {} (already linked)", source_display, target_display));
                context.manager.state.add_link(source_display, target_display, is_dir)?;
            }
            Action::Conflict {
                source_display,
                target_display,
                source_path,
                target_path,
                kind,
            } => {
                let action = match strategy {
                    None => {
                        msg.error(&format!("Conflict: {} -> {} ({})", source_display, target_display, kind));
                        ConflictAction::prompt(&kind).unwrap()
                    }
                    Some(a) => a.clone(),
                };

                handle_conflict(&action, &source_path, &target_path, &source_display, context)?;

                if action == ConflictAction::Overwrite || action == ConflictAction::Adopt {
                    let is_dir = source_path.is_dir();
                    context.manager.state.add_link(source_display, target_display, is_dir)?;
                }
            }
            Action::SourceMissing { source_display, .. } => {
                msg.error(&format!("Source missing: {}", source_display));
            }
        }
    }

    context.manager.state.set_active_profile(target_profile.to_string())?;
    msg.success(&format!("Switched to profile '{}'", target_profile));

    Ok(())
}

fn handle_conflict(
    action: &ConflictAction,
    source: &Path,
    destination: &PathBuf,
    rel_source: &str,
    context: &Context,
) -> Result<(), Box<dyn Error>> {
    let dotfiles_root = context
        .manager
        .state
        .get_dotfiles_path()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    match action {
        ConflictAction::Skip => println!("  Skipped {}", destination.display()),
        ConflictAction::Abort => return Err("Operation aborted by user.".into()),
        ConflictAction::Overwrite => {
            if destination.is_dir() {
                fs::remove_dir_all(destination).unwrap();
            } else {
                fs::remove_file(destination).unwrap();
            }
            context.manager.symlink.create(source, destination)?;
            println!("  Overwrite: {} → {}", source.display(), destination.display());
        }
        ConflictAction::Adopt => {
            let adopt_target = dotfiles_root.join(rel_source);
            if let Some(parent) = adopt_target.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            if adopt_target.exists() {
                if adopt_target.is_dir() {
                    fs::remove_dir_all(&adopt_target).unwrap();
                } else {
                    fs::remove_file(&adopt_target).unwrap();
                }
            }

            fs::rename(destination, &adopt_target).unwrap();
            context.manager.symlink.create(source, destination)?;
            println!("  Adopted: {} → {}", source.display(), destination.display());
        }
    }

    Ok(())
}

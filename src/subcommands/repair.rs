use crate::context::Context;
use crate::utils::{DestinationStatus, expand_tilde, get_destination_status, symlink};
use colored::Colorize;
use std::error::Error;
use std::path::PathBuf;

/// Actions that the repair command can take.
pub enum Action {
    Link {
        source_display: String,
        target_display: String,
        source_path: PathBuf,
        target_path: PathBuf,
        is_dir: bool,
    },
    Relink {
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
    NotifyConflict {
        target_display: String,
    },
    NotifySourceMissing {
        source_display: String,
        _source_path: PathBuf,
    },
}

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    // 1. GENERATE THE PLAN
    let plan = generate_plan(context)?;

    if plan.is_empty() {
        context.message.success("No repairs needed. All symlinks are healthy.");
        return Ok(());
    }

    // 2. DISPATCH
    if context.dry_run {
        execute_dry_run(&plan, context);
    } else {
        execute(plan, context)?;
    }

    Ok(())
}

fn generate_plan(context: &Context) -> Result<Vec<Action>, Box<dyn Error>> {
    let mut plan = Vec::new();
    let dotfiles_root = context
        .state
        .dotfiles_path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    // Helper to process a set of links
    let process_links = |links: &indexmap::IndexMap<String, String>, plan: &mut Vec<Action>| {
        for (target_str, source_str) in links {
            let source_path = dotfiles_root.join(source_str);
            let target_path = expand_tilde(target_str);

            if !source_path.exists() {
                plan.push(Action::NotifySourceMissing {
                    source_display: source_str.clone(),
                    _source_path: source_path,
                });
                continue;
            }

            let status = get_destination_status(&source_path, &target_path);
            let is_dir = source_path.is_dir();

            match status {
                DestinationStatus::AlreadyLinked => {
                    // Check if it's in state. If not, we should update state.
                    let in_state = context.state.managed_links.iter().any(|l| l.target == *target_str);
                    if !in_state {
                        plan.push(Action::UpdateState {
                            source_display: source_str.clone(),
                            target_display: target_str.clone(),
                            is_dir,
                        });
                    }
                }
                DestinationStatus::NonExistent => {
                    plan.push(Action::Link {
                        source_display: source_str.clone(),
                        target_display: target_str.clone(),
                        source_path,
                        target_path,
                        is_dir,
                    });
                }
                DestinationStatus::ConflictingSymlink => {
                    plan.push(Action::Relink {
                        source_display: source_str.clone(),
                        target_display: target_str.clone(),
                        source_path,
                        target_path,
                        is_dir,
                    });
                }
                DestinationStatus::ConflictingFileOrDir => {
                    plan.push(Action::NotifyConflict {
                        target_display: target_str.clone(),
                    });
                }
            }
        }
    };

    // Global links
    if let Some(global) = &context.config.global {
        process_links(&global.links, &mut plan);
    }

    // Active profile links
    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = &context.state.active_profile {
            if let Some(profile) = profiles.get(active_name) {
                process_links(&profile.links, &mut plan);
            }
        }
    }

    Ok(plan)
}

fn execute_dry_run(plan: &[Action], context: &Context) {
    let msg = &context.message;
    println!("{}", "Repair Plan (Dry Run):".bold().blue());

    for action in plan {
        match action {
            Action::Link {
                source_display,
                target_display,
                ..
            } => {
                msg.success(&format!("Would link {} -> {} (dry run)", source_display, target_display));
            }
            Action::Relink {
                source_display,
                target_display,
                ..
            } => {
                msg.success(&format!("Would relink {} -> {} (dry run)", source_display, target_display));
            }
            Action::UpdateState {
                source_display,
                target_display,
                ..
            } => {
                msg.info(&format!("Would add to state: {} -> {} (dry run)", source_display, target_display));
            }
            Action::NotifyConflict { target_display } => {
                msg.error(&format!(
                    "Conflict at {}: File/Dir exists. Manual intervention required.",
                    target_display
                ));
            }
            Action::NotifySourceMissing { source_display, .. } => {
                msg.unlink(&format!("Source missing: {}", source_display));
            }
        }
    }
}

fn execute(plan: Vec<Action>, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;
    println!("{}", "Executing Repair...".bold().blue());

    for action in plan {
        match action {
            Action::Link {
                source_display,
                target_display,
                source_path,
                target_path,
                is_dir,
            } => {
                msg.success(&format!("Linking {} -> {}", source_display, target_display));
                symlink(&source_path, &target_path, context)?;
                context.state.add_managed_link(source_display, target_display, is_dir);
            }
            Action::Relink {
                source_display,
                target_display,
                source_path,
                target_path,
                is_dir,
            } => {
                msg.success(&format!("Relinking {} -> {}", source_display, target_display));
                if target_path.exists() || target_path.is_symlink() {
                    std::fs::remove_file(&target_path)?;
                }
                symlink(&source_path, &target_path, context)?;
                context.state.add_managed_link(source_display, target_display, is_dir);
            }
            Action::UpdateState {
                source_display,
                target_display,
                is_dir,
            } => {
                msg.info(&format!("Updating state for: {} -> {}", source_display, target_display));
                context.state.add_managed_link(source_display, target_display, is_dir);
            }
            Action::NotifyConflict { target_display } => {
                msg.error(&format!(
                    "Conflict at {}: File/Dir exists. Manual intervention required.",
                    target_display
                ));
            }
            Action::NotifySourceMissing { source_display, .. } => {
                msg.unlink(&format!("Source missing: {}", source_display));
            }
        }
    }

    context.state.save()?;
    msg.success("Repair complete.");
    Ok(())
}

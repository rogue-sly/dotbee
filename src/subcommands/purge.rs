use colored::Colorize;
use crate::context::Context;
use crate::state::ManagedLink;
use std::{error::Error, fs, io, path::PathBuf};
use crate::utils::expand_tilde;

/// The types of actions our purge command can take.
pub enum PurgeAction {
    Delete {
        target_display: String,
        path: PathBuf,
        link_state: ManagedLink,
    },
    NotifyMissing {
        target_display: String,
        link_state: ManagedLink,
    },
    NotifyNotASymlink {
        target_display: String,
        _path: PathBuf,
    },
}

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;
    // 1. GENERATE THE PLAN
    // This always runs, fixing the previous dry-run bug.
    let plan = generate_plan(context);

    if plan.is_empty() {
        msg.info("No managed links found to purge.");
        return Ok(());
    }

    // 2. DISPATCH
    if context.dry_run {
        execute_dry(&plan, context);
    } else {
        execute(plan, context)?;
    }

    Ok(())
}

fn generate_plan(context: &Context) -> Vec<PurgeAction> {
    let mut plan: Vec<PurgeAction> = vec![];

    for link in &context.state.managed_links {
        let target_path = expand_tilde(&link.target);

        // Check if the path exists or is a broken symlink
        if !target_path.exists() && !target_path.is_symlink() {
            plan.push(PurgeAction::NotifyMissing {
                target_display: link.target.clone(),
                link_state: link.clone(),
            });
            continue;
        }

        // Safety check: Is it actually a symlink?
        if !target_path.is_symlink() {
            plan.push(PurgeAction::NotifyNotASymlink {
                target_display: link.target.clone(),
                _path: target_path,
            });
            continue;
        }

        plan.push(PurgeAction::Delete {
            target_display: link.target.clone(),
            path: target_path,
            link_state: link.clone(),
        });
    }

    plan
}

fn execute(plan: Vec<PurgeAction>, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;
    println!("{}", "Executing Purge...".bold().red());

    for action in plan {
        match action {
            PurgeAction::Delete {
                path,
                target_display,
                link_state,
            } => match fs::remove_file(&path) {
                Ok(_) => {
                    msg.delete(&format!("Removed {}", target_display));
                    context.state.managed_links.retain(|l| l != &link_state);
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        msg.warning(&format!("Target '{}' disappeared during execution.", target_display));
                        context.state.managed_links.retain(|l| l != &link_state);
                    } else {
                        msg.error(&format!("Failed to remove {}: {}", target_display, e));
                    }
                }
            },
            PurgeAction::NotifyMissing {
                target_display,
                link_state,
            } => {
                msg.warning(&format!("Cleaning up stale state for missing link: {}", target_display));
                context.state.managed_links.retain(|l| l != &link_state);
            }
            PurgeAction::NotifyNotASymlink { target_display, .. } => {
                msg.error(&format!("Aborting removal of {}: path is a real file/directory.", target_display));
            }
        }
    }

    context.state.save()?;
    context.state.clear_active_profile()?; // This also calls save()
    msg.success("Purge complete.");
    Ok(())
}

fn execute_dry(plan: &[PurgeAction], context: &Context) {
    let msg = &context.message;
    println!("{}", "Purge Plan (Dry Run):".bold().yellow());

    for action in plan {
        match action {
            PurgeAction::Delete { target_display, .. } => {
                msg.delete(&format!("Would remove {}", target_display));
            }
            PurgeAction::NotifyMissing { target_display, .. } => {
                msg.warning(&format!("{} is already missing from disk.", target_display));
            }
            PurgeAction::NotifyNotASymlink { target_display, .. } => {
                msg.error(&format!("SKIPPING {}: not a symlink.", target_display));
            }
        }
    }
}

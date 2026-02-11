use colored::Colorize;
use context::Context;
use indexmap::IndexMap;
use std::error::Error;
use utils::{expand_path, get_destination_status, symlink_with_parents, DestinationStatus};

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    if context.dry_run {
        println!("{}", "Repairing symlinks (dry run)...".bold().blue());
    } else {
        println!("{}", "Repairing symlinks...".bold().blue());
    }

    if let Some(global) = &context.config.global {
        println!("Checking global links...");
        let links = global.links.clone();
        repair_links(&links, context)?;
    }

    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = context.state.active_profile.clone() {
            if let Some(profile) = profiles.get(&active_name) {
                let name_yellow = active_name.green();
                context.message.info(&format!("Checking active profile '{}'...", name_yellow));
                let links = profile.links.clone();
                repair_links(&links, context)?;
            } else {
                context.message.info(&format!("Active profile '{}' not found in config.", active_name));
            }
        } else {
            context.message.info("No active profile detected. Only global links were checked.");
        }
    }

    if !context.dry_run {
        context.state.save()?;
    }

    context.message.success("Repair complete.");
    Ok(())
}

fn repair_links(links: &IndexMap<String, String>, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let dry_run = context.dry_run;
    let message = &context.message;

    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str);

        if !source_path.exists() {
            message.unlink(&format!("Source missing: {}", source_path.display()));
            continue;
        }

        let status = get_destination_status(&source_path, &target_path);

        match status {
            DestinationStatus::AlreadyLinked => {
                if !dry_run {
                    context.state.add_managed_link(
                        source_str.clone(),
                        target_str.clone(),
                        source_path.is_dir(),
                    );
                }
            }
            DestinationStatus::NonExistent => {
                if dry_run {
                    message.success(&format!("Would link {} -> {} (dry run)", source_str, target_str));
                } else {
                    message.success(&format!("Linking {} -> {}", source_str, target_str));
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                    context.state.add_managed_link(
                        source_str.clone(),
                        target_str.clone(),
                        source_path.is_dir(),
                    );
                }
            }
            DestinationStatus::ConflictingSymlink => {
                if dry_run {
                    message.success(&format!("Would relink {} -> {} (dry run)", source_str, target_str));
                } else {
                    message.success(&format!("Relinking {} -> {}", source_str, target_str));
                    if target_path.exists() || target_path.is_symlink() {
                        std::fs::remove_file(&target_path)?;
                    }
                    symlink_with_parents(&source_path, &target_path, dry_run)?;
                    context.state.add_managed_link(
                        source_str.clone(),
                        target_str.clone(),
                        source_path.is_dir(),
                    );
                }
            }
            DestinationStatus::ConflictingFileOrDir => {
                message.error(&format!(
                    "Conflict at {} (File/Dir exists). Manual intervention required.",
                    target_str
                ));
            }
        }
    }
    Ok(())
}

use colored::Colorize;
use config::ConflictAction;
use context::Context;
use indexmap::IndexMap;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use utils::{expand_path, get_destination_status, get_hostname, symlink_with_parents, unlink_profile_links, DestinationStatus};

pub fn run(profile_name: Option<String>, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let profile_name = match profile_name {
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

    if context.dry_run {
        println!("{}", "Switching profile (dry run)...".bold().yellow());
    }

    // apply global symlinks
    if let Some(global) = &context.config.global {
        println!("{}", "Processing global links...".blue());
        let links = global.links.clone();
        let strategy = context.config.settings.on_conflict.clone();
        process_links(&links, &strategy, context).unwrap();
    }

    // unlink other active profiles
    if let Some(profiles) = &context.config.profiles {
        if let Some(active_name) = context.state.active_profile.clone() {
            if active_name != profile_name {
                if let Some(profile) = profiles.get(&active_name) {
                    let name_yellow = active_name.yellow();
                    context.message.info(&format!("Unlinking active profile '{}'...", name_yellow));
                    let links = profile.links.clone();
                    unlink_profile_links(&links, context.dry_run, &context.message).unwrap();
                    if !context.dry_run {
                        for (target, _) in &links {
                            context.state.managed_links.retain(|l| &l.target != target);
                        }
                    }
                } else {
                    context.message.warning(&format!("Active profile '{}' not found in config.", active_name));
                }
            }
        }
    }

    // apply profile symlinks
    if let Some(profiles) = &context.config.profiles {
        if let Some(profile) = profiles.get(profile_name.as_str()) {
            let name_green = profile_name.green();
            context.message.info(&format!("Processing profile '{}'...", name_green));
            let links = profile.links.clone();
            let strategy = context.config.settings.on_conflict.clone();
            process_links(&links, &strategy, context).unwrap();
        } else {
            return Err(format!("Profile '{}' not found in configuration.", profile_name).into());
        }
    } else {
        context.message.error("No profiles defined in config.");
        std::process::exit(1)
    }

    if context.dry_run {
        context.message.success("Switch dry run complete.");
    } else {
        context.state.set_active_profile(profile_name)?;
    }

    Ok(())
}

fn process_links(
    links: &IndexMap<String, String>,
    default_conflict_strategy: &Option<ConflictAction>,
    context: &mut Context,
) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let dry_run = context.dry_run;

    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str);

        if !source_path.exists() {
            context.message.error(&format!("Source not found: {}", source_path.display()));
            continue;
        }

        let status = get_destination_status(&source_path, &target_path);

        match status {
            DestinationStatus::AlreadyLinked => {
                context.message.success(&format!("{} → {} (already linked)", source_str, target_str));
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
                    context.message.link(&format!("Would link {} → {} (dry run)", source_str, target_str));
                } else {
                    symlink_with_parents(&source_path, &target_path, dry_run).unwrap();
                    context.message.link(&format!("{} → {}", source_str, target_str));
                    context.state.add_managed_link(
                        source_str.clone(),
                        target_str.clone(),
                        source_path.is_dir(),
                    );
                }
            }
            _ => {
                let kind = match status {
                    DestinationStatus::ConflictingSymlink => "Symlink",
                    _ => "File/Dir",
                };

                // Resolve the action based on config or prompt
                let action = match default_conflict_strategy {
                    Some(ConflictAction::Ask) | None => {
                        context.message.error(&format!("Conflict: {} → {} ({})", source_str, target_str, kind));
                        if dry_run {
                            context.message.warning("Skipping conflict resolution in dry run");
                            ConflictAction::Skip
                        } else {
                            ConflictAction::prompt(kind).unwrap()
                        }
                    }
                    Some(action) => action.clone(),
                };

                handle_conflict(action.clone(), &source_path, &target_path, Path::new(source_str), dry_run).unwrap();
                if !dry_run && (action == ConflictAction::Overwrite || action == ConflictAction::Adopt) {
                     context.state.add_managed_link(
                        source_str.clone(),
                        target_str.clone(),
                        source_path.is_dir(),
                    );
                }
            }
        }
    }
    Ok(())
}

fn handle_conflict(
    action: ConflictAction,
    source: &Path,
    destination: &PathBuf,
    rel_source: &Path,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    let repo_root = std::env::current_dir()?;

    match action {
        ConflictAction::Skip => println!("  Skipped {}", destination.display()),
        ConflictAction::Abort => return Err("Operation aborted by user.".into()),
        ConflictAction::Overwrite => {
            if dry_run {
                println!("  Would overwrite: {} → {} (dry run)", source.display(), destination.display());
            } else {
                if destination.is_symlink() || destination.is_file() || destination.is_dir() {
                    #[cfg(not(target_os = "android"))]
                    trash::delete(destination).unwrap();
                    #[cfg(target_os = "android")]
                    if destination.is_dir() {
                        fs::remove_dir_all(destination).unwrap();
                    } else {
                        fs::remove_file(destination).unwrap();
                    }
                }
                symlink_with_parents(source, destination, dry_run).unwrap();
                println!("  Overwrite: {} → {}", source.display(), destination.display());
            }
        }
        ConflictAction::Adopt => {
            if dry_run {
                println!("  Would adopt: {} → {} (dry run)", source.display(), destination.display());
            } else {
                let adopt_target = repo_root.join(rel_source);
                // ensure parent exists in repo (it should if source exists, but checking just in case)
                if let Some(parent) = adopt_target.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                // if the source file already exists in repo, trash it before adopting the system one?
                // "Adopt" implies the system one is the truth.
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
                // move the file from destination (system) to source (repo)
                // rename might fail across filesystems, so copy+delete is safer, but rename is atomic on same FS.
                // let's try rename first, fallback to copy/delete if needed?
                // for now, simple rename :D
                fs::rename(destination, &adopt_target).unwrap();
                // Now link back
                symlink_with_parents(source, destination, dry_run).unwrap();
                println!("  Adopted: {} → {}", source.display(), destination.display());
            }
        }
        ConflictAction::Ask => panic!("'Ask' action should have been resolved before handling conflict"),
    }

    Ok(())
}

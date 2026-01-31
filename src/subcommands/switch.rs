use crate::config::Config;
use crate::config::ConflictAction;
use crate::config::hooks::execute_hook;
use crate::config::icons::Icons;
use crate::state::State;
use crate::utils::{
    DestinationStatus, expand_path, find_active_profile, get_destination_status, get_hostname, symlink_with_parents,
    unlink_profile_links,
};
use colored::Colorize;
use indexmap::IndexMap;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn run(profile_name: Option<String>, config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let mut state = State::load()?;
    let cwd = std::env::current_dir().unwrap();
    let icon_style = config.settings.icon_style.unwrap_or_default();
    let icons = Icons::new(icon_style);

    let profile_name = match profile_name {
        Some(name) => name,
        None => {
            if config.settings.auto_detect_profile.unwrap_or(false) {
                if let Some(hostname) = get_hostname() {
                    println!("{} No profile specified. Auto-detecting profile from hostname: '{}'", icons.info.blue(), hostname);
                    hostname
                } else {
                    return Err("Failed to auto-detect hostname.".into());
                }
            } else {
                return Err("No profile specified and auto_detect_profile is disabled.".into());
            }
        }
    };

    if dry_run {
        println!("{}", "Switching profile (dry run)...".bold().yellow());
    }

    // pre-hooks
    if let Some(hooks) = &config.hooks {
        if let Some(pre) = &hooks.pre {
            println!("{}", "Running pre-hooks...".yellow());
            execute_hook(pre, dry_run).unwrap();
        }
    }

    // apply global symlinks
    if let Some(global) = &config.global {
        println!("{}", "Processing global links...".blue());
        process_links(&global.links, &cwd, &config.settings.on_conflict, dry_run, &icons).unwrap();
    }

    // unlink other active profiles
    if let Some(profiles) = &config.profiles {
        if let Some(active_name) = find_active_profile(profiles, state.active_profile.as_ref(), &cwd) {
            if active_name != &profile_name {
                if let Some(profile) = profiles.get(active_name) {
                    println!("Unlinking active profile '{}'...", active_name.yellow());
                    unlink_profile_links(&profile.links, &cwd, dry_run, &icons).unwrap();
                } else {
                    println!("Warning: Active profile '{}' not found in config.", active_name);
                }
            }
        }
    }

    // apply profile symlinks
    if let Some(profiles) = &config.profiles {
        if let Some(profile) = profiles.get(&profile_name) {
            println!("Processing profile '{}'...", profile_name.green());
            process_links(&profile.links, &cwd, &config.settings.on_conflict, dry_run, &icons).unwrap();
        } else {
            return Err(format!("Profile '{}' not found in configuration.", profile_name).into());
        }
    } else {
        println!("No profiles defined in config.");
    }

    // post-hooks
    if let Some(hooks) = &config.hooks {
        if let Some(post) = &hooks.post {
            println!("{}", "Running post-hooks...".yellow());
            execute_hook(post, dry_run).unwrap();
        }
    }

    if dry_run {
        println!("{}", "Switch dry run complete.".green());
    } else {
        state.set_active_profile(profile_name)?;
    }

    Ok(())
}

fn process_links(
    links: &IndexMap<String, String>,
    cwd: &Path,
    default_conflict_strategy: &Option<ConflictAction>,
    dry_run: bool,
    icons: &Icons,
) -> Result<(), Box<dyn Error>> {
    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str).unwrap();

        if !source_path.exists() {
            println!("{} Source not found: {}", icons.error.red(), source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path).unwrap();

        match status {
            DestinationStatus::AlreadyLinked => println!("{} {} → {} (already linked)", icons.success.green(), source_str, target_str),
            DestinationStatus::NonExistent => {
                if dry_run {
                    println!("{} Would link {} → {} (dry run)", icons.link.green(), source_str, target_str);
                } else {
                    symlink_with_parents(&source_path, &target_path, dry_run).unwrap();
                    println!("{} {} → {}", icons.link.green(), source_str, target_str);
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
                        println!("{} Conflict: {} → {} ({})", icons.error.red(), source_str, target_str, kind);
                        if dry_run {
                            println!("  {} Skipping conflict resolution in dry run", icons.warning.yellow());
                            ConflictAction::Skip
                        } else {
                            ConflictAction::prompt(kind).unwrap()
                        }
                    }
                    Some(action) => action.clone(),
                };

                handle_conflict(action, &source_path, &target_path, cwd, Path::new(source_str), dry_run).unwrap();
            }
        }
    }
    Ok(())
}

fn handle_conflict(
    action: ConflictAction,
    source: &Path,
    destination: &PathBuf,
    repo_root: &Path,
    rel_source: &Path,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
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
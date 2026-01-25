use crate::config::Config;
use crate::util::{expand_path, get_destination_status, is_profile_active, symlink_with_parents, unlink_profile_links, DestinationStatus};
use colored::Colorize;
use demand::{DemandOption, Select, Theme};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone)]
enum ConflictAction {
    Abort,
    Adopt,
    Overwrite,
    Skip,
}

impl Display for ConflictAction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConflictAction::Abort => "abort",
                ConflictAction::Adopt => "adopt",
                ConflictAction::Overwrite => "overwrite",
                ConflictAction::Skip => "skip",
            }
        )
    }
}

impl ConflictAction {
    fn prompt(kind: &str) -> Result<ConflictAction, Box<dyn Error>> {
        let selection = Select::new("Conflict")
            .description(format!("Conflict occurred of kind: {kind}.\nhow do you want to handle it?").as_str())
            .theme(&Theme::base16())
            .options(vec![
                DemandOption::new(ConflictAction::Abort).description("Stop switching"),
                DemandOption::new(ConflictAction::Adopt).description("Replace the file in dotfiles with the conflicting one"),
                DemandOption::new(ConflictAction::Overwrite).description("Overwrite conflicting file"),
                DemandOption::new(ConflictAction::Skip).description("Don't symlink this file"),
            ])
            .run()?;

        Ok(selection)
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "abort" => Some(ConflictAction::Abort),
            "adopt" => Some(ConflictAction::Adopt),
            "overwrite" => Some(ConflictAction::Overwrite),
            "skip" => Some(ConflictAction::Skip),
            _ => None, // "ask" or invalid goes here
        }
    }
}

pub fn run(profile_name: String, config_path: Option<String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::load(config_path)?;
    let cwd = std::env::current_dir().unwrap();

    if dry_run {
        println!("{}", "Switching profile (dry run)...".bold().yellow());
    }

    // pre-hooks
    if let Some(hooks) = &config.hooks {
        if let Some(pre) = &hooks.pre {
            println!("{}", "Running pre-hooks...".yellow());
            run_hooks(pre, dry_run).unwrap();
        }
    }

    // apply global symlinks
    if let Some(global) = &config.global {
        println!("{}", "Processing global links...".blue());
        process_links(&global.links, &cwd, &config.settings.on_conflict, dry_run).unwrap();
    }

    // unlink other active profiles
    if let Some(profiles) = &config.profiles {
        for (name, profile) in profiles {
            if name != &profile_name && is_profile_active(profile, &cwd) {
                println!("Unlinking previously active profile '{}'...", name.yellow());
                unlink_profile_links(&profile.links, &cwd, dry_run).unwrap();
            }
        }
    }

    // apply profile symlinks
    if let Some(profiles) = &config.profiles {
        if let Some(profile) = profiles.get(&profile_name) {
            println!("Processing profile '{}'...", profile_name.green());
            process_links(&profile.links, &cwd, &config.settings.on_conflict, dry_run).unwrap();
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
            run_hooks(post, dry_run).unwrap();
        }
    }

    if dry_run {
        println!("{}", "Switch dry run complete.".green());
    }

    Ok(())
}

fn run_hooks(hooks: &HashMap<String, String>, dry_run: bool) -> Result<(), Box<dyn Error>> {
    for (name, command) in hooks {
        if dry_run {
            println!("  Would run {}: {} (dry run)", name.cyan(), command);
        } else {
            println!("  Running {}: {}", name.cyan(), command);
            let status = Command::new("sh").arg("-c").arg(command).status().unwrap();
            if !status.success() {
                eprintln!("{} Hook '{}' failed with status {}", "Warning:".yellow(), name, status);
            }
        }
    }
    Ok(())
}

fn process_links(
    links: &HashMap<String, String>,
    cwd: &Path,
    default_conflict_strategy: &str,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str).unwrap();

        if !source_path.exists() {
            println!("{} Source not found: {}", " ".red(), source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path).unwrap();

        match status {
            DestinationStatus::AlreadyLinked => println!("{} {} → {} (already linked)", " ".green(), source_str, target_str),
            DestinationStatus::NonExistent => {
                if dry_run {
                    println!("{} Would link {} → {} (dry run)", " ".green(), source_str, target_str);
                } else {
                    symlink_with_parents(&source_path, &target_path, dry_run).unwrap();
                    println!("{} {} → {}", " ".green(), source_str, target_str);
                }
            }
            _ => {
                let kind = match status {
                    DestinationStatus::ConflictingSymlink => "Symlink",
                    _ => "File/Dir",
                };

                let action = match ConflictAction::from_str(default_conflict_strategy) {
                    Some(action) => action,
                    _ => {
                        println!("{} Conflict: {} → {} ({})", " ".red(), source_str, target_str, kind);
                        if dry_run {
                            println!("  {} Skipping conflict resolution in dry run", "⚠️ ".yellow());
                            ConflictAction::Skip
                        } else {
                            ConflictAction::prompt(kind).unwrap()
                        }
                    }
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
                    trash::delete(destination).unwrap();
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
                    trash::delete(&adopt_target).unwrap();
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
    }

    Ok(())
}

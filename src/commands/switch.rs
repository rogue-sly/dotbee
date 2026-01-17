use crate::config::Config;
use crate::util::expand_path;
use colored::Colorize;
use demand::{DemandOption, Select, Theme};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::Command,
};

enum DestinationStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink(PathBuf),
    NonExistent,
}

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

pub fn run(profile_name: String) -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("dotsy.toml");
    if !config_path.exists() {
        return Err("dotsy.toml not found. Run 'dotsy init' first.".into());
    }

    let config = Config::load_from_path(config_path).unwrap();
    let cwd = std::env::current_dir().unwrap();

    // pre-hooks
    if let Some(hooks) = &config.hooks {
        if let Some(pre) = &hooks.pre {
            println!("{}", "Running pre-hooks...".yellow());
            run_hooks(pre).unwrap();
        }
    }

    // apply global symlinks
    if let Some(global) = &config.global {
        println!("{}", "Processing global links...".blue());
        process_links(&global.links, &cwd, &config.settings.on_conflict).unwrap();
    }

    // apply profile symlinks
    if let Some(profiles) = &config.profiles {
        if let Some(profile) = profiles.get(&profile_name) {
            println!("Processing profile '{}'...", profile_name.green());
            process_links(&profile.links, &cwd, &config.settings.on_conflict).unwrap();
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
            run_hooks(post).unwrap();
        }
    }

    Ok(())
}

fn run_hooks(hooks: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    for (name, command) in hooks {
        println!("  Running {}: {}", name.cyan(), command);
        let status = Command::new("sh").arg("-c").arg(command).status().unwrap();
        if !status.success() {
            eprintln!("{} Hook '{}' failed with status {}", "Warning:".yellow(), name, status);
        }
    }
    Ok(())
}

fn process_links(links: &HashMap<String, String>, cwd: &Path, default_conflict_strategy: &str) -> Result<(), Box<dyn Error>> {
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
                symlink_with_parents(&source_path, &target_path).unwrap();
                println!("{} {} → {}", " ".green(), source_str, target_str);
            }
            _ => {
                let kind = match status {
                    DestinationStatus::ConflictingSymlink(_) => "Symlink",
                    _ => "File/Dir",
                };

                let action = match ConflictAction::from_str(default_conflict_strategy) {
                    Some(action) => action,
                    _ => {
                        println!("{} Conflict: {} → {} ({})", " ".red(), source_str, target_str, kind);
                        ConflictAction::prompt(kind).unwrap()
                    }
                };

                handle_conflict(action, &source_path, &target_path, cwd, Path::new(source_str)).unwrap();
            }
        }
    }
    Ok(())
}



fn get_destination_status(source: &Path, destination: &Path) -> Result<DestinationStatus, Box<dyn Error>> {
    if !destination.exists() && !destination.is_symlink() {
        return Ok(DestinationStatus::NonExistent);
    }

    let target = match fs::read_link(destination) {
        Ok(v) => v,
        Err(_) => return Ok(DestinationStatus::ConflictingFileOrDir),
    };

    match (destination.is_symlink(), target == source) {
        (true, true) => Ok(DestinationStatus::AlreadyLinked),
        (true, false) => Ok(DestinationStatus::ConflictingSymlink(target)),
        _ => Ok(DestinationStatus::ConflictingFileOrDir),
    }
}

fn handle_conflict(
    action: ConflictAction,
    source: &Path,
    destination: &PathBuf,
    repo_root: &Path,
    rel_source: &Path,
) -> Result<(), Box<dyn Error>> {
    match action {
        ConflictAction::Skip => println!("  Skipped {}", destination.display()),
        ConflictAction::Abort => return Err("Operation aborted by user.".into()),
        ConflictAction::Overwrite => {
            if destination.is_symlink() || destination.is_file() || destination.is_dir() {
                trash::delete(destination).unwrap();
            }
            symlink_with_parents(source, destination).unwrap();
            println!("  Overwrite: {} → {}", source.display(), destination.display());
        }
        ConflictAction::Adopt => {
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
            symlink_with_parents(source, destination).unwrap();
            println!("  Adopted: {} → {}", source.display(), destination.display());
        }
    }

    Ok(())
}

fn symlink_with_parents(source: &Path, destination: &PathBuf) -> std::io::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    symlink(source, destination)
}

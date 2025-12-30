use colored::Colorize;
use demand::{DemandOption, Select, Theme};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Clone)]
enum DestinationStatus {
    AlreadyLinked,
    ConflictingSymlink(PathBuf),
    ConflictingFileOrDir,
    NonExistent,
}

#[derive(Clone, Copy)]
enum ConflictAction {
    Skip,
    Overwrite,
    Adopt,
}

impl Display for ConflictAction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConflictAction::Skip => "skip",
                ConflictAction::Overwrite => "overwrite",
                ConflictAction::Adopt => "adopt",
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
                DemandOption::new(ConflictAction::Skip).description("Don't symlink this file"),
                DemandOption::new(ConflictAction::Overwrite).description("Overwrite conflicting file"),
                DemandOption::new(ConflictAction::Adopt).description("Replace the file in dotfiles with the conflicting one"),
            ])
            .run()?;

        Ok(selection)
    }
}

pub fn run(config: String) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn get_destination_status(source: &Path, destination: &Path) -> Result<DestinationStatus, Box<dyn Error>> {
    if !destination.exists() {
        return Ok(DestinationStatus::NonExistent);
    }

    if destination.is_symlink() {
        let target = std::fs::read_link(destination)?;
        match target == source {
            true => Ok(DestinationStatus::AlreadyLinked),
            false => Ok(DestinationStatus::ConflictingSymlink(target)),
        }
    } else {
        Ok(DestinationStatus::ConflictingFileOrDir)
    }
}

fn handle_conflict(source: &Path, destination: &PathBuf, selected_host: &Path, rel_path: &Path, kind: &str) -> Result<(), Box<dyn Error>> {
    match ConflictAction::prompt(kind)? {
        ConflictAction::Skip => println!("  Skipped {}", destination.display()),
        ConflictAction::Overwrite => {
            if destination.is_file() || destination.is_dir() {
                trash::delete(destination)?;
            }
            symlink(source, destination)?;
            println!(" Removed and symlinked: {} → {}", source.display(), destination.display());
        }
        ConflictAction::Adopt => {
            let adopt_target = selected_host.join(rel_path);
            if let Some(parent) = adopt_target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::rename(destination, &adopt_target)?;
            symlink(source, destination)?;
            println!("󰸧  Adopted existing file into host config and created new symlink.");
        }
    }

    Ok(())
}

fn symlink_with_parents(source: &Path, destination: &PathBuf) -> std::io::Result<()> {
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    symlink(source, destination)
}

fn print_link_status(status: &DestinationStatus, source: &Path, destination: &Path) {
    match status {
        DestinationStatus::AlreadyLinked => {
            println!("{} {} → {} (already linked)", " ".cyan(), source.display(), destination.display());
        }
        DestinationStatus::ConflictingSymlink(target) => {
            println!(
                "{} {} → {} (conflicts with symlink to {})",
                " ".red(),
                source.display(),
                destination.display(),
                target.display()
            );
        }
        DestinationStatus::ConflictingFileOrDir => {
            println!(
                "{} {} → {} (conflicts with existing file/dir)",
                " ".red(),
                source.display(),
                destination.display()
            );
        }
        DestinationStatus::NonExistent => {
            println!("{} {} → {}", " ".green(), source.display(), destination.display());
        }
    }
}

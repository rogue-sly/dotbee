use colored::Colorize;
use demand::{DemandOption, Select, Theme};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Copy, Clone)]
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
            .description(
                format!("Conflict occurred of kind: {kind}.\nhow do you want to handle it?")
                    .as_str(),
            )
            .theme(&Theme::base16())
            .options(vec![
                DemandOption::new(ConflictAction::Skip).description("Don't symlink this file"),
                DemandOption::new(ConflictAction::Overwrite)
                    .description("Overwrite conflicting file"),
                DemandOption::new(ConflictAction::Adopt)
                    .description("Replace the file in dotfiles with the conflicting one"),
            ])
            .run()
            .expect("Error occurred in selection menu");

        Ok(selection)
    }
}

pub fn run(host: String) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let hosts_dir = cwd.join("hosts");
    let selected_host = hosts_dir.join(&host);
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;

    if !selected_host.exists() {
        return Err(format!("Host {host} not found").into());
    }

    println!("{}: {}", "Switching to host".yellow(), host.green().bold());

    for entry in WalkDir::new(&selected_host)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let source = entry.path();
        let rel_path = source.strip_prefix(&selected_host)?;
        let destination = home.join(rel_path);

        if destination.exists() {
            if destination.is_symlink() {
                let target = std::fs::read_link(&destination)?;
                if target == source {
                    println!(
                        "{} {} → {} (already linked)",
                        "🔁".cyan(),
                        source.display(),
                        destination.display()
                    );
                } else {
                    println!(
                        "{} {} → {} (conflicts with symlink to {})",
                        "❌".red(),
                        source.display(),
                        destination.display(),
                        target.display()
                    );

                    handle_conflict(source, &destination, &selected_host, rel_path, "symlink")?;
                }
            } else {
                println!(
                    "{} {} → {} (conflicts with existing file/dir)",
                    "❌".red(),
                    source.display(),
                    destination.display()
                );

                handle_conflict(source, &destination, &selected_host, rel_path, "file/dir")?;
            }
        } else {
            symlink_with_parents(source, &destination)?;
            println!(
                "{} {} → {}",
                "🔗".green(),
                source.display(),
                destination.display()
            );
        }
    }

    Ok(())
}

fn handle_conflict(
    source: &Path,
    destination: &PathBuf,
    selected_host: &Path,
    rel_path: &Path,
    kind: &str,
) -> Result<(), Box<dyn Error>> {
    match ConflictAction::prompt(kind)? {
        ConflictAction::Skip => println!("➡️ Skipped {}", destination.display()),
        ConflictAction::Overwrite => {
            if destination.is_dir() {
                trash::delete(destination)?;
            }
            symlink(source, destination)?;
            println!(
                "🗑️ Removed and symlinked: {} → {}",
                source.display(),
                destination.display()
            );
        }
        ConflictAction::Adopt => {
            let adopt_target = selected_host.join(rel_path);
            if let Some(parent) = adopt_target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::rename(destination, &adopt_target)?;
            symlink(source, destination)?;
            println!("📥 Adopted existing file into host config and created new symlink.");
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

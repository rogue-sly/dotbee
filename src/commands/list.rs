use crate::config::Config;
use crate::util::is_profile_active;
use colored::Colorize;
use std::{error::Error, path::Path};

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("dotsy.toml");
    if !config_path.exists() {
        return Err("dotsy.toml not found. Run 'dotsy init' first.".into());
    }

    let config = Config::load_from_path(config_path).unwrap();
    let cwd = std::env::current_dir()?;

    if let Some(profiles) = config.profiles {
        for (name, profile) in profiles {
            let active = is_profile_active(&profile, &cwd);
            let title = if active {
                format!("{} (active)", name).green().bold()
            } else {
                name.bold()
            };

            println!("{}", title);

            let mut links: Vec<_> = profile.links.iter().collect();
            links.sort_by_key(|(k, _)| k.as_str());

            for (i, (target, source)) in links.iter().enumerate() {
                let is_last = i == links.len() - 1;
                let branch = if is_last { "└──" } else { "├──" };
                println!("{} {} -> {}", branch, target, source);
            }
        }
    } else {
        println!("No profiles defined in config.");
    }

    Ok(())
}

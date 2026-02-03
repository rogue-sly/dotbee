use colored::Colorize;
use dotsy::context::Context;
use dotsy::utils::{find_active_profile, is_profile_active};
use std::error::Error;

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;

    let active_profile_name = if let Some(profiles) = &context.config.profiles {
        find_active_profile(profiles, context.state.active_profile.as_ref(), &cwd)
    } else {
        None
    };

    if let Some(profiles) = &context.config.profiles {
        for (name, profile) in profiles {
            let is_resolved_active = active_profile_name == Some(&name);
            let is_active_in_state = context.state.active_profile.as_ref() == Some(&name);
            let is_physically_active = is_profile_active(&profile, &cwd);

            let title = if is_resolved_active {
                if is_active_in_state {
                    if is_physically_active {
                        format!("{} (active)", name).green().bold()
                    } else {
                        format!("{} (active - broken)", name).yellow().bold()
                    }
                } else {
                    format!("{} (active - inferred)", name).cyan().bold()
                }
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

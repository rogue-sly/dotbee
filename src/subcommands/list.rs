use colored::Colorize;
use context::Context;
use indexmap::IndexMap;
use std::error::Error;

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    // List global symlinks
    if let Some(global) = &context.config.global {
        println!("{}", "global".yellow().bold());
        show_links(&global.links);
    }

    // List profiles symlinks and hightlight the active profile (if present)
    let active_profile = context.state.active_profile.as_ref();
    if let Some(profiles) = &context.config.profiles {
        for (name, profile) in profiles {
            let is_active = active_profile == Some(&name);

            let title = if is_active {
                format!("{} (active)", name).green().bold()
            } else {
                name.bold()
            };

            println!("{}", title);
            show_links(&profile.links);
        }
    }

    Ok(())
}

fn show_links(links: &IndexMap<String, String>) {
    for (i, (target, source)) in links.iter().enumerate() {
        let is_last = i == links.len() - 1;
        let branch = if is_last { "└──" } else { "├──" };
        println!("{} {} -> {}", branch, target, source);
    }
}

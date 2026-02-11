use colored::Colorize;
use context::Context;

use indexmap::IndexMap;
use std::error::Error;

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    if let Some(global) = &context.config.global {
        println!("{}", "global".yellow().bold());
        print_links(&global.links);
    }

    let active_profile_name = context.state.active_profile.as_ref();

    if let Some(profiles) = &context.config.profiles {
        for (name, profile) in profiles {
            let is_active_in_state = active_profile_name == Some(&name);

            let title = if is_active_in_state {
                format!("{} (active)", name).green().bold()
            } else {
                name.bold()
            };

            println!("{}", title);
            print_links(&profile.links);
        }
    } else if context.config.global.is_none() {
        println!("No profiles or global links defined in config.");
    }

    Ok(())
}

fn print_links(links: &IndexMap<String, String>) {
    let mut links_vec: Vec<_> = links.iter().collect();
    links_vec.sort_by_key(|(k, _)| k.as_str());

    for (i, (target, source)) in links_vec.iter().enumerate() {
        let is_last = i == links_vec.len() - 1;
        let branch = if is_last { "└──" } else { "├──" };
        println!("{} {} -> {}", branch, target, source);
    }
}

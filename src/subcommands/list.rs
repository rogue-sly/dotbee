use crate::context::Context;
use colored::Colorize;
use indexmap::IndexMap;

pub fn run(context: &Context) -> anyhow::Result<(), anyhow::Error> {
    // List global symlinks
    if let Some(global_links) = context.config.get_global_links() {
        println!("{}", "global".yellow().bold());
        show_links(global_links);
    }

    // List profiles and highlight the active one
    let active_profile = context.state.get_active_profile();
    for name in context.config.list_profiles() {
        let is_active = active_profile == Some(name);

        let title = if is_active {
            format!("{} (active)", name).green().bold()
        } else {
            name.bold()
        };

        println!("{}", title);
        if let Ok(profile) = context.config.get_profile(name) {
            show_links(&profile.links);
        }
    }

    Ok(())
}

fn show_links(links: &IndexMap<String, String>) {
    let mut iter = links.iter().peekable();
    while let Some((target, source)) = iter.next() {
        let branch = if iter.peek().is_none() { "└──" } else { "├──" };
        println!("{} {} -> {}", branch, target, source);
    }
}

use colored::Colorize;
use context::Context;
use std::error::Error;
use utils::unlink_profile_links;

pub fn run(context: &mut Context) -> Result<(), Box<dyn Error>> {
    let msg = &context.message;

    if context.dry_run {
        println!("{}", "Purging all managed links (dry run)...".bold().red());
    } else {
        println!("{}", "Purging all managed links...".bold().red());
    }

    if let Some(global) = &context.config.global {
        msg.info("Unlinking global links...");
        unlink_profile_links(&global.links, context.dry_run, msg)?;
    }

    if let Some(profiles) = &context.config.profiles {
        for (name, profile) in profiles {
            msg.info(&format!("Unlinking profile '{}'...", name.yellow()));
            unlink_profile_links(&profile.links, context.dry_run, msg)?;
        }
    }

    if context.dry_run {
        msg.success("Purge dry run complete.");
    } else {
        context.state.clear_active_profile()?;
        msg.success("Purge complete.");
    }

    Ok(())
}

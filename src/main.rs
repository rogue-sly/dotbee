mod cli;
mod subcommands;

use clap::Parser;
use cli::{Cli, SubCommand};
use dotsy::context::Context;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dotsy = Cli::parse();
    let mut context = Context::new(dotsy.config, dotsy.dry_run)?;

    match dotsy.subcommand {
        SubCommand::Completion { shell } => subcommands::completion::run(shell)?,
        SubCommand::Doctor => subcommands::doctor::run(&context)?,
        SubCommand::Init => subcommands::init::run(&context)?,
        SubCommand::List => subcommands::list::run(&context)?,
        SubCommand::Purge => subcommands::purge::run(&mut context)?,
        SubCommand::Repair => subcommands::repair::run(&mut context)?,
        SubCommand::Switch { profile } => subcommands::switch::run(profile, &mut context)?,
    }

    Ok(())
}

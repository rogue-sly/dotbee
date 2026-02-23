// my shitty helper code
mod context;
mod utils;
// my shitty app code
mod cli;
mod subcommands;

use clap::Parser;
use cli::{Cli, SubCommand};
use context::Context;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dotbee = Cli::parse();
    let mut context = Context::new(dotbee.config, dotbee.dry_run)?;

    match dotbee.subcommand {
        SubCommand::Completion { shell } => subcommands::completion::run(shell)?,
        SubCommand::Doctor => subcommands::doctor::run(&context)?,
        SubCommand::Init => subcommands::init::run(&mut context)?,
        SubCommand::List => subcommands::list::run(&context)?,
        SubCommand::Purge => subcommands::purge::run(&mut context)?,
        SubCommand::Repair => subcommands::repair::run(&mut context)?,
        SubCommand::Switch { profile } => subcommands::switch::run(profile, &mut context)?,
    }

    Ok(())
}

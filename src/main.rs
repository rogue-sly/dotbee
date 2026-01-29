mod cli;
mod config;
mod state;
mod subcommands;
mod utils;
use clap::Parser;
use cli::{Cli, SubCommand};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dotsy = Cli::parse();

    match dotsy.subcommand {
        SubCommand::Doctor { config } => subcommands::doctor::run(config)?,
        SubCommand::Init { config, dry_run } => subcommands::init::run(config, dry_run)?,
        SubCommand::List { config } => subcommands::list::run(config)?,
        SubCommand::Purge { config, dry_run } => subcommands::purge::run(config, dry_run)?,
        SubCommand::Repair { config, dry_run } => subcommands::repair::run(config, dry_run)?,
        SubCommand::Switch { profile, config, dry_run } => subcommands::switch::run(profile, config, dry_run)?,
    }

    Ok(())
}

mod cli;
mod commands;
mod config;
mod util;
use clap::Parser;
use cli::{Cli, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Doctor { config } => commands::doctor::run(config)?,
        Command::Init { config, dry_run } => commands::init::run(config, dry_run)?,
        Command::List { config } => commands::list::run(config)?,
        Command::Purge { config, dry_run } => commands::purge::run(config, dry_run)?,
        Command::Repair { config, dry_run } => commands::repair::run(config, dry_run)?,
        Command::Switch { profile, config, dry_run } => commands::switch::run(profile, config, dry_run)?,
    }

    Ok(())
}

mod cli;
mod commands;
mod config;
use clap::Parser;
use cli::{Cli, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Init {} => commands::init::run()?,
        Command::List {} => commands::list::run()?,
        Command::Purge {} => commands::purge::run()?,
        Command::Repair {} => commands::repair::run()?,
        Command::Doctor {} => commands::doctor::run()?,
        Command::Switch { config } => commands::switch::run(config)?,
    }

    Ok(())
}

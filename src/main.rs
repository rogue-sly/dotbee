mod cli;
mod commands;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Init { name } => commands::init::run(name)?,
        Command::List {} => commands::list::run()?,
        Command::Purge {} => commands::purge::run()?,
        Command::Repair {} => commands::repair::run()?,
        Command::Status {} => commands::status::run()?,
        Command::Switch { host } => commands::switch::run(host)?,
    }

    Ok(())
}

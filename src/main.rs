mod cli;
mod commands;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { name } => commands::init::run(name)?,
        Commands::List {} => commands::list::run()?,
        Commands::Purge {} => commands::purge::run()?,
        Commands::Repair {} => commands::repair::run()?,
        Commands::Status {} => commands::status::run()?,
        Commands::Switch { host } => commands::switch::run(host)?,
    }

    Ok(())
}

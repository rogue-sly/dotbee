mod cli;
mod context;
mod subcommands;
mod utils;

use anyhow::Context;
use clap::Parser;
use cli::{Cli, SubCommand};
use nix::fcntl::{Flock, FlockArg};
use std::fs::File;
use subcommands::*;

fn main() -> anyhow::Result<()> {
    let dotbee = Cli::parse();
    let _lock = lock_process()?;
    let mut context = context::Context::new(dotbee.config, dotbee.dry_run)?;
    match dotbee.subcommand {
        SubCommand::Completion { shell } => completion::run(shell)?,
        SubCommand::Doctor => doctor::run(&context)?,
        SubCommand::Init => init::run(&mut context)?,
        SubCommand::List => list::run(&context)?,
        SubCommand::Purge => purge::run(&mut context)?,
        SubCommand::Sync { profile } => sync::run(profile, &mut context)?,
        SubCommand::Add { profile } => add::run(&mut context, profile)?,
        SubCommand::Remove { profile } => remove::run(&mut context, profile)?,
        SubCommand::Fetch { method } => fetch::run(&mut context, method)?,
        SubCommand::Edit => edit::run(&context)?,
    }

    Ok(())
}

/// in case of a shmuck user decides to run multiple
/// processes of dotbee at the same time,
/// this should handle this stupid issue
/// I mean honestly, if you do this, it's your fault
/// but I guess I can't get myself to not care about it
/// enough. -w-
fn lock_process() -> anyhow::Result<Flock<File>> {
    let dotbee_state_dir = {
        let base = cfg_select! {
            any(target_os = "linux") => dirs::state_dir(),
            _ => dirs::data_dir(),
        };

        let mut path = base.context("Couldn't determine state directory")?;
        path.push("dotbee");
        path
    };
    std::fs::create_dir_all(&dotbee_state_dir)?;
    let lock_path = dotbee_state_dir.join("dotbee.lock");

    // attempt to open lock file
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open(&lock_path)?;
    // acquire and return lock
    Flock::lock(file, FlockArg::LockExclusiveNonblock).map_err(|_| anyhow::anyhow!("There can only be ONE dotbee process"))
}

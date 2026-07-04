use anyhow::Context;
use clap::Parser;
use dotbee::cli::{Cli, SubCommand};
use dotbee::subcommands;
use nix::fcntl::{Flock, FlockArg};
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let dotbee = Cli::parse();
    let _lock = lock_process()?;
    let mut context = dotbee::context::Context::new(dotbee.config, dotbee.dry_run)?;
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

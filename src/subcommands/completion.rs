use clap::CommandFactory;
use clap_complete::{Shell as ClapShell, generate};
use std::io;

use crate::cli::{Cli, Shell};

pub fn run(shell: Shell) -> anyhow::Result<(), anyhow::Error> {
    let mut cmd = Cli::command();
    generate(
        ClapShell::from(shell),
        &mut cmd,
        "dotbee",
        &mut io::stdout(),
    );
    Ok(())
}

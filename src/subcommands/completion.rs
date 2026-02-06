use clap::CommandFactory;
use clap_complete::{Shell as ClapShell, generate};
use std::error::Error;
use std::io;

use crate::cli::{Cli, Shell};

pub fn run(shell: Shell) -> Result<(), Box<dyn Error>> {
    let mut cmd = Cli::command();
    generate(ClapShell::from(shell), &mut cmd, "dotsy", &mut io::stdout());
    Ok(())
}

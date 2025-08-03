use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Dotsy")]
#[command(about = "An opinionated, file-based dotfiles manager", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// switch to a host
    Switch {
        /// host to switch to
        host: String,
    },

    /// show the current active host and symlinks
    Status {},

    /// scaffold a new dotfiles layout or host
    Init {
        /// name for dotfiles directory
        #[arg(short, long)]
        name: String,
    },

    /// list all available hosts
    List {},

    /// purge symlinks and state
    Purge {},

    /// attempt to fix broken symlinks
    Repair {},
}

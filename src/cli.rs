use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Dotsy")]
#[command(about = "Easy to use dotfiles manager", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// switch to a host
    Switch {
        /// config collection to switch to
        config: String,
    },

    /// show currently used configs and symlinks status
    Status {},

    /// init dotsy :3
    Init {},

    /// list all available configs
    List {},

    /// purge symlinks
    Purge {},

    /// attempt to fix broken symlinks
    Repair {},
}

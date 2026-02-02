use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Dotsy", about = "Easy to use dotfiles manager", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand,

    /// custom config file path
    #[clap(short, long, value_name = "FILE", global = true)]
    pub config: Option<String>,

    /// do not perform any actions, just print what would be done
    #[clap(long, global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// show currently used configs and symlinks status
    Doctor,

    /// init dotsy :3
    Init,

    /// list all available configs
    List,

    /// purge symlinks
    Purge,

    /// attempt to fix broken symlinks
    Repair,

    /// select profile
    Switch {
        /// profile to switch to
        profile: Option<String>,
    },
}

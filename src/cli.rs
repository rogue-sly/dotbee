use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Dotsy", about = "Easy to use dotfiles manager", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// show currently used configs and symlinks status
    Doctor {
        /// custom config file path
        #[clap(short, long, value_name = "FILE", global = true)]
        config: Option<String>,
    },

    /// init dotsy :3
    Init {
        /// custom config file path
        #[clap(short, long, value_name = "FILE", global = true)]
        config: Option<String>,

        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },

    /// list all available configs
    List {
        /// custom config file path
        #[clap(short, long, value_name = "FILE", global = true)]
        config: Option<String>,
    },

    /// purge symlinks
    Purge {
        /// custom config file path
        #[clap(short, long, value_name = "FILE", global = true)]
        config: Option<String>,

        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },

    /// attempt to fix broken symlinks
    Repair {
        /// custom config file path
        #[clap(short, long, value_name = "FILE", global = true)]
        config: Option<String>,

        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },

    /// select profile
    Switch {
        /// profile to switch to
        profile: Option<String>,

        /// custom config file path
        #[clap(short, long, value_name = "FILE", global = true)]
        config: Option<String>,

        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },
}

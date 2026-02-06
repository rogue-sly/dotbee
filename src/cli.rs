use clap::{Parser, Subcommand, ValueEnum};

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

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Elvish,
}

impl From<Shell> for clap_complete::Shell {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::Elvish => clap_complete::Shell::Elvish,
        }
    }
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// generate shell completions
    #[command(visible_alias = "c")]
    Completion {
        /// shell to generate completions for
        shell: Shell,
    },

    /// show currently used configs and symlinks status
    #[command(visible_alias = "dr")]
    Doctor,

    /// init dotsy
    #[command(visible_alias = "i")]
    Init,

    /// list all available configs
    #[command(visible_alias = "ls")]
    List,

    /// purge symlinks
    #[command(visible_alias = "p")]
    Purge,

    /// attempt to fix broken symlinks
    #[command(visible_alias = "r")]
    Repair,

    /// select profile
    #[command(visible_alias = "s")]
    Switch {
        /// profile to switch to
        profile: Option<String>,
    },
}

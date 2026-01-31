use demand::{DemandOption, Select, Theme};
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConflictAction {
    Abort,
    Adopt,
    Overwrite,
    Skip,
    Ask,
}

impl Display for ConflictAction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConflictAction::Abort => "abort",
                ConflictAction::Adopt => "adopt",
                ConflictAction::Overwrite => "overwrite",
                ConflictAction::Skip => "skip",
                ConflictAction::Ask => "ask",
            }
        )
    }
}

impl Default for ConflictAction {
    fn default() -> Self {
        ConflictAction::Ask
    }
}

impl ConflictAction {
    pub fn prompt(kind: &str) -> Result<ConflictAction, Box<dyn Error>> {
        let selection = Select::new("Conflict")
            .description(format!("Conflict occurred of kind: {}.\nhow do you want to handle it?", kind).as_str())
            .theme(&Theme::base16())
            .options(vec![
                DemandOption::new(ConflictAction::Abort).description("Stop switching"),
                DemandOption::new(ConflictAction::Adopt).description("Replace the file in dotfiles with the conflicting one"),
                DemandOption::new(ConflictAction::Overwrite).description("Overwrite conflicting file"),
                DemandOption::new(ConflictAction::Skip).description("Don't symlink this file"),
            ])
            .run()?;

        Ok(selection)
    }
}

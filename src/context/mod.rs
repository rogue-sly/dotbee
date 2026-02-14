pub mod message;

use crate::config::Config;
use crate::config::icons::IconStyle;
use crate::state::State;
use message::Message;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Context {
    pub config: Config,
    pub state: State,
    pub message: Message,
    pub dry_run: bool,
    pub config_path: Option<String>,
}

impl Context {
    pub fn new(config_path: Option<String>, dry_run: bool) -> Result<Self, Box<dyn Error>> {
        let mut state = State::load()?;

        // Determine effective config path
        let effective_config_path = match config_path.as_ref() {
            Some(p) => Some(p.clone()),
            None => state
                .dotfiles_path
                .as_ref()
                .map(|p| p.join("dotsy.toml").to_string_lossy().to_string()),
        };

        let (config, loaded_path) = Config::load(effective_config_path)?;

        // If a config was loaded from a file, update dotfiles_path in state
        if let Some(path) = loaded_path {
            if let Some(parent) = path.parent() {
                let dotfiles_path = parent.to_path_buf();
                if state.dotfiles_path.as_ref() != Some(&dotfiles_path) {
                    state.dotfiles_path = Some(dotfiles_path);
                    state.save()?;
                }
            }
        }

        let icon_style = config.settings.icon_style.unwrap_or(IconStyle::Text);
        let message = Message::new(icon_style);

        Ok(Context {
            config,
            state,
            message,
            dry_run,
            config_path,
        })
    }
}

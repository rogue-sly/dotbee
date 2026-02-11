pub mod message;

use crate::message::Message;
use config::Config;
use config::icons::IconStyle;
pub use state::{ManagedLink, State};
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
        let config = Config::load(config_path.clone())?;
        let state = State::load()?;
        let icon_style = config.settings.icon_style.clone().unwrap_or(IconStyle::Text);
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

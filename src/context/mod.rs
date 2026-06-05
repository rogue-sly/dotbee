pub mod config;
pub mod state;
pub mod symlink;

use crate::context::{config::Config, state::State, symlink::Symlink};
pub struct Context {
    pub symlink: Symlink,
    pub state: State,
    pub config: Config,
    pub dry_run: bool,
}

impl Context {
    pub fn new(path_to_config: Option<String>, dry_run: bool) -> anyhow::Result<Self, anyhow::Error> {
        let mut state = State::load()?;

        // determine effective config path from explicit arg or stored dotfiles path
        let effective_config_path = match path_to_config.as_ref() {
            Some(p) => Some(p.clone()),
            None => state
                .get_dotfiles_path()
                .map(|p| p.join("dotbee.toml").to_string_lossy().to_string()),
        };

        let config = Config::load(effective_config_path)?;

        // sync dotfiles path between config and state
        if let Some(new_dotfiles_path) = config
            .get_config_path()
            .and_then(|p| p.parent())
            .filter(|p| state.get_dotfiles_path() != Some(p))
        {
            state.set_dotfiles_path(Some(new_dotfiles_path.to_path_buf()))?;
        } else if config.get_config_path().is_none() && state.get_dotfiles_path().is_some() {
            // config no longer exists but state still references it - clear stale path
            state.set_dotfiles_path(None)?;
        }

        Ok(Self {
            symlink: Symlink,
            state,
            config,
            dry_run,
        })
    }
}

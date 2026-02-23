pub mod config;
pub mod state;
pub mod symlink;

use config::ConfigManager;
use state::StateManager;
use std::error::Error;
use symlink::SymlinkManager;

/// A part of the context singleton. Responsible for managing the following:
/// - Symlinks
/// - State file
/// - Config file
pub struct Manager {
    pub symlink: SymlinkManager,
    pub state: StateManager,
    pub config: ConfigManager,
}

impl Manager {
    pub fn new(path_to_config: Option<String>) -> Result<Self, Box<dyn Error>> {
        let mut state = StateManager::load()?;

        // Determine effective config path from explicit arg or stored dotfiles path
        let effective_config_path = match path_to_config.as_ref() {
            Some(p) => Some(p.clone()),
            None => state
                .get_dotfiles_path()
                .map(|p| p.join("dotbee.toml").to_string_lossy().to_string()),
        };

        let config = ConfigManager::load(effective_config_path)?;

        // Sync dotfiles path between config and state
        if let Some(new_dotfiles_path) = config
            .get_config_path()
            .and_then(|p| p.parent())
            .filter(|p| state.get_dotfiles_path() != Some(p))
        {
            state.set_dotfiles_path(Some(new_dotfiles_path.to_path_buf()))?;
        } else if config.get_config_path().is_none() && state.get_dotfiles_path().is_some() {
            // Config no longer exists but state still references it - clear stale path
            state.set_dotfiles_path(None)?;
        }

        Ok(Self {
            symlink: SymlinkManager::new(),
            state,
            config,
        })
    }
}

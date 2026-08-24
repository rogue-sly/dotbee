use anyhow::{Context as _, Result};
use demand::{Input, Theme};
use dirs::{data_dir, home_dir};
use git2::{
    Cred, CredentialType, FetchOptions, RemoteCallbacks,
    build::{CheckoutBuilder, RepoBuilder},
};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::context::Context;
use crate::utils::message;

pub fn run(context: &mut Context, url: String) -> Result<()> {
    if context.dry_run {
        message::info(&format!("Would fetch {}", url));
        return Ok(());
    }

    let path = data_dir().context("Could not determine data directory")?.join("dotbee");

    if path.is_dir() && fs::read_dir(&path)?.next().is_some() {
        message::warning(&format!("Directory {} already exists and is not empty, skipping", path.display()));
        return Ok(());
    }

    clone(&url, &path).with_context(|| format!("Failed to clone {}", url))?;

    context.state.set_dotfiles_path(Some(path.clone()))?;

    message::success(&format!("Cloned to {}", path.display()));
    Ok(())
}

fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = 1024 * KB;
    const GB: usize = 1024 * MB;
    match bytes {
        b if b >= GB => format!("{:.2} GiB", bytes as f64 / GB as f64),
        b if b >= MB => format!("{:.2} MiB", bytes as f64 / MB as f64),
        b if b >= KB => format!("{:.2} KiB", bytes as f64 / KB as f64),
        _ => format!("{} B", bytes),
    }
}

fn find_ssh_keys() -> Vec<PathBuf> {
    let mut keys = Vec::new();
    if let Some(ssh_dir) = home_dir().map(|h| h.join(".ssh")) {
        let key = ssh_dir.join("id_ed25519");
        if key.is_file() {
            keys.push(key);
        }
    }
    keys
}

fn prompt_passphrase(description: &str) -> Result<String> {
    let input = Input::new("SSH Passphrase")
        .description(description)
        .password(true)
        .theme(&Theme::base16())
        .run()?;
    Ok(input)
}

fn prompt_input(title: &str, placeholder: &str) -> Result<String> {
    let input = Input::new(title).placeholder(placeholder).theme(&Theme::base16()).run()?;
    Ok(input)
}

fn is_ssh_key_encrypted(path: &Path) -> bool {
    let Ok(content) = fs::read_to_string(path) else {
        return false;
    };

    if let Ok(key) = ssh_key::PrivateKey::from_openssh(&content) {
        return key.is_encrypted();
    }

    false
}

#[derive(Default)]
struct AuthState {
    tried_agent: bool,
    key_index: usize,
    tried_key_without_passphrase: bool,
    passphrase_attempts: usize,
    total_attempts: usize,
}

fn clone(url: &str, path: &Path) -> Result<()> {
    let existed_before = path.exists();

    let mut state = AuthState::default();
    let mut callbacks = RemoteCallbacks::new();

    callbacks.sideband_progress(|data| {
        if let Ok(msg) = std::str::from_utf8(data) {
            let msg = msg.trim();
            if !msg.is_empty() {
                print!("\r\x1b[2Kremote: {}\r", msg);
                let _ = io::stdout().flush();
            }
        }
        true
    });

    callbacks.transfer_progress(|stats| {
        let total_objects = stats.total_objects();
        let received_objects = stats.received_objects();
        let indexed_deltas = stats.indexed_deltas();
        let total_deltas = stats.total_deltas();
        let received_bytes = stats.received_bytes();

        if total_objects > 0 && received_objects < total_objects {
            let percent = (received_objects * 100).checked_div(total_objects).unwrap_or(0);
            let bytes = format_bytes(received_bytes);
            print!(
                "\r\x1b[2KReceiving objects: {:3}% ({}/{}) | {}",
                percent, received_objects, total_objects, bytes
            );
            let _ = io::stdout().flush();
        } else if total_deltas > 0 && indexed_deltas < total_deltas {
            let percent = (indexed_deltas * 100).checked_div(total_deltas).unwrap_or(0);
            print!("\r\x1b[2KResolving deltas: {:3}% ({}/{})", percent, indexed_deltas, total_deltas);
            let _ = io::stdout().flush();
        }
        true
    });

    callbacks.credentials(move |cred_url, username_from_url, allowed_types| {
        let user = username_from_url.unwrap_or("git");

        if allowed_types.contains(CredentialType::SSH_KEY) {
            if state.total_attempts >= 10 {
                return Err(git2::Error::from_str("Maximum authentication attempts exceeded"));
            }
            state.total_attempts += 1;

            // try SSH agent first if not yet tried
            if !state.tried_agent {
                state.tried_agent = true;
                if let Ok(cred) = Cred::ssh_key_from_agent(user) {
                    return Ok(cred);
                }
            }

            let ssh_keys = find_ssh_keys();

            // iterate through discovered SSH keys
            while state.key_index < ssh_keys.len() {
                let key_path = &ssh_keys[state.key_index];
                let pub_key = key_path.with_extension("pub");
                let pub_key_opt = if pub_key.is_file() { Some(pub_key.as_path()) } else { None };

                let encrypted = is_ssh_key_encrypted(key_path);

                if !encrypted && !state.tried_key_without_passphrase {
                    state.tried_key_without_passphrase = true;
                    if let Ok(cred) = Cred::ssh_key(user, pub_key_opt, key_path, None) {
                        return Ok(cred);
                    }
                }

                // prompt for passphrase for current key (allow up to 3 attempts per key)
                if state.passphrase_attempts < 3 {
                    state.passphrase_attempts += 1;
                    let desc = format!("Enter passphrase for {}", key_path.display());
                    if let Ok(passphrase) = prompt_passphrase(&desc) {
                        let pass_opt = if passphrase.is_empty() { None } else { Some(passphrase) };
                        if let Ok(cred) = Cred::ssh_key(user, pub_key_opt, key_path, pass_opt.as_deref()) {
                            return Ok(cred);
                        }
                    }
                }

                // advance to next key after attempts exhausted
                state.key_index += 1;
                state.tried_key_without_passphrase = false;
                state.passphrase_attempts = 0;
            }

            // if no standard keys found or all standard keys tried, prompt for custom key
            if let Ok(path_str) = prompt_input("SSH Key Path", "~/.ssh/id_ed25519") {
                let expanded = if let Some(stripped) = path_str.strip_prefix("~/") {
                    dirs::home_dir()
                        .map(|h| h.join(stripped))
                        .unwrap_or_else(|| PathBuf::from(&path_str))
                } else {
                    PathBuf::from(&path_str)
                };

                if expanded.is_file() {
                    let pub_key = expanded.with_extension("pub");
                    let pub_key_opt = if pub_key.is_file() { Some(pub_key.as_path()) } else { None };

                    let desc = format!("Enter passphrase for {}", expanded.display());
                    let passphrase = prompt_passphrase(&desc).unwrap_or_default();
                    let pass_opt = if passphrase.is_empty() { None } else { Some(passphrase.as_str()) };
                    if let Ok(cred) = Cred::ssh_key(user, pub_key_opt, &expanded, pass_opt) {
                        return Ok(cred);
                    }
                }
            }

            return Err(git2::Error::from_str("SSH authentication failed"));
        }

        if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
            if state.total_attempts >= 10 {
                return Err(git2::Error::from_str("Maximum authentication attempts exceeded"));
            }
            state.total_attempts += 1;

            if let Ok(config) = git2::Config::open_default()
                && let Ok(cred) = Cred::credential_helper(&config, cred_url, username_from_url)
            {
                return Ok(cred);
            }

            let username = match username_from_url {
                Some(u) => u.to_string(),
                None => prompt_input("Username", "git").unwrap_or_else(|_| "git".to_string()),
            };
            if let Ok(password) = prompt_passphrase("Enter password or personal access token")
                && let Ok(cred) = Cred::userpass_plaintext(&username, &password)
            {
                return Ok(cred);
            }
        }

        if allowed_types.contains(CredentialType::DEFAULT)
            && let Ok(cred) = Cred::default()
        {
            return Ok(cred);
        }

        Cred::default()
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut checkout = CheckoutBuilder::new();
    checkout.progress(|_, completed, total| {
        if let Some(percent) = (completed * 100).checked_div(total) {
            print!("\r\x1b[2KUpdating files: {:3}% ({}/{})", percent, completed, total);
            let _ = io::stdout().flush();
        }
    });

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.with_checkout(checkout);

    let result = builder.clone(url, path);

    print!("\r\x1b[2K");
    let _ = io::stdout().flush();

    if let Err(err) = result {
        if !existed_before && path.exists() {
            let _ = fs::remove_dir_all(path);
        }
        return Err(anyhow::anyhow!(err));
    }

    Ok(())
}

use anyhow::{Context as _, Result};
use dirs::data_dir;
use std::{fs, path::Path, process::Command};

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

fn clone(url: &str, path: &Path) -> Result<()> {
    let status = Command::new("git")
        .args(["clone", url])
        .arg(path)
        .status()
        .context("Failed to execute 'git'. Is it installed?")?;

    if !status.success() {
        anyhow::bail!("git clone failed");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::TempDir;

    fn git(cwd: &Path, args: &[&str]) {
        let output = Command::new("git")
            .arg("-c")
            .arg("commit.gpgsign=false")
            .args(args)
            .current_dir(cwd)
            .env("GIT_AUTHOR_NAME", "test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .expect("failed to spawn git");
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn source_repo(url_holder: &TempDir) -> PathBuf {
        let source = url_holder.path().join("source");
        fs::create_dir_all(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        fs::write(source.join("dotbee.toml"), "[profiles.global.links]\n").unwrap();
        git(&source, &["add", "."]);
        git(&source, &["commit", "-m", "init"]);
        source
    }

    #[test]
    fn clones_local_repository() {
        let holder = TempDir::new().unwrap();
        let target = holder.path().join("target");

        clone(&source_repo(&holder).to_string_lossy(), &target).unwrap();

        assert!(target.join(".git").exists());
        assert!(target.join("dotbee.toml").exists());
    }

    #[test]
    fn failed_clone_leaves_no_target_directory() {
        let holder = TempDir::new().unwrap();
        let target = holder.path().join("target");
        let missing = holder.path().join("does-not-exist");

        assert!(clone(&missing.to_string_lossy(), &target).is_err());
        assert!(!target.exists());
    }

    #[test]
    fn empty_existing_directory_is_usable_as_target() {
        let holder = TempDir::new().unwrap();
        let target = holder.path().join("target");
        fs::create_dir_all(&target).unwrap();

        clone(&source_repo(&holder).to_string_lossy(), &target).unwrap();

        assert!(target.join("dotbee.toml").exists());
    }
}

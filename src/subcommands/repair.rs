use crate::context::Context;
use crate::context::manager::symlink::SymlinkStatus;
use crate::utils::common::expand_tilde;
use crate::utils::message;

pub fn run(context: &mut Context) -> anyhow::Result<(), anyhow::Error> {
    let dotfiles_root = context
        .manager
        .state
        .get_dotfiles_path()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    let dry_run = context.dry_run;
    let mut has_actions = false;

    // collect links upfront to avoid borrow conflicts when mutating state
    let mut links: Vec<(String, String)> = Vec::new();

    if let Some(global_links) = context.manager.config.get_global_links() {
        links.extend(global_links.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    if let Some(active_profile) = context.manager.state.get_active_profile()
        && let Ok(profile) = context.manager.config.get_profile(active_profile)
    {
        links.extend(profile.links.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    for (target_str, source_str) in &links {
        let source_path = dotfiles_root.join(source_str);
        let target_path = expand_tilde(target_str);

        if !source_path.exists() {
            message::miss(&format!("Source missing: {}", source_str));
            has_actions = true;
            continue;
        }

        let status = context.manager.symlink.check(&source_path, &target_path);
        let is_dir = source_path.is_dir();

        match status {
            SymlinkStatus::AlreadyLinked => {
                let in_state = context.manager.state.get_links().iter().any(|l| l.target == *target_str);
                if !in_state {
                    if dry_run {
                        message::info(&format!("Would add to state: {} -> {} (dry run)", source_str, target_str));
                    } else {
                        message::info(&format!("Updating state for: {} -> {}", source_str, target_str));
                        context.manager.state.add_link(source_str.clone(), target_str.clone(), is_dir)?;
                    }
                    has_actions = true;
                }
            }
            SymlinkStatus::NonExistent => {
                if dry_run {
                    message::success(&format!("Would link {} -> {} (dry run)", source_str, target_str));
                } else {
                    message::success(&format!("Linking {} -> {}", source_str, target_str));
                    context.manager.symlink.create(&source_path, &target_path)?;
                    context.manager.state.add_link(source_str.clone(), target_str.clone(), is_dir)?;
                }
                has_actions = true;
            }
            SymlinkStatus::ConflictingSymlink => {
                if dry_run {
                    message::success(&format!("Would relink {} -> {} (dry run)", source_str, target_str));
                } else {
                    message::success(&format!("Relinking {} -> {}", source_str, target_str));
                    if target_path.exists() || target_path.is_symlink() {
                        std::fs::remove_file(&target_path)?;
                    }
                    context.manager.symlink.create(&source_path, &target_path)?;
                    context.manager.state.add_link(source_str.clone(), target_str.clone(), is_dir)?;
                }
                has_actions = true;
            }
            SymlinkStatus::ConflictingFileOrDir => {
                message::error(&format!(
                    "Conflict at {}: File/Dir exists. Manual intervention required.",
                    target_str
                ));
                has_actions = true;
            }
        }
    }

    if !has_actions {
        message::success("No repairs needed. All symlinks are healthy.");
    } else if !dry_run {
        message::success("Repair complete.");
    }

    Ok(())
}

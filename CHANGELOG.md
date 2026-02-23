# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-02-23

### Changed
- **BREAKING CHANGE:** Renamed project from **Dotsy** to **Dotbee**.
    - Command: `dotsy` -> `dotbee`
    - Config file: `dotsy.toml` -> `dotbee.toml`
    - Schema: `dotsy.json` -> `dotbee.json`
    - State directory: `~/.local/state/dotsy` -> `~/.local/state/dotbee`
    - Completions: Shell completion scripts must be regenerated using `dotbee completion <SHELL>`.

### Migration
If you are upgrading from `v0.3.0`, perform the following manual steps:

1.  **Rename Config:** `mv dotsy.toml dotbee.toml`
2.  **Move State (Optional):** `mv ~/.local/state/dotsy ~/.local/state/dotbee`
3.  **Update Aliases:** Update your shell configuration to use `dotbee` instead of `dotsy`.

## [0.3.0] - 2026-02-22

### Added
- Unit test coverage across all three manager structs (26 tests total).
  - `ConfigManager`: loading, profile lookup, global links, settings, config path.
  - `StateManager`: active profile, dotfiles path, managed links, clear, `ManagedLink` equality.
  - `SymlinkManager`: check, create, force-create, remove-existing.

### Changed
- **Major Internal Refactor:** Introduced dedicated `ConfigManager`, `StateManager`, and
  `SymlinkManager` structs. Each manager owns its data privately and exposes a clean API,
  replacing the previous pattern of `Manager` directly owning raw `Config`, `State`, and
  `Symlink` structs.
- `StateManager` now auto-saves state to disk on every mutation.
- `SymlinkManager` is now a plain struct rather than a trait implemented by a `Symlink` struct.
- Config path tracking moved out of the `Config` data struct and into `ConfigManager` as a
  separate field, keeping `Config` as a pure mirror of the TOML structure.
- `ConflictAction::Ask` variant removed; conflict resolution is now cleaner and less error-prone.
- `repair` subcommand logic refactored to use functional chaining, removing deeply nested
  conditionals.
- State is now fully cleared on `purge` and on load failures.

## [0.2.1] - 2026-02-07

### Changed
- `dotbee list` now displays the global profile configuration if one is defined.
- Internal refactoring of `list` command to better handle global links.

## [0.2.0] - 2026-02-06

### Added
- New `completion` command to generate shell completion scripts (Bash, Zsh, Fish, Elvish, PowerShell).
- Command aliases for improved developer experience:
    - `ls` for `list`
    - `s` for `switch`
    - `dr` for `doctor`
- `CONTRIBUTING.md` guide for new contributors.

### Changed
- **Major Refactor:** Modularized the codebase and separated application logic from the library.
- **Context Pattern:** Introduced a `Context` module to manage shared state (config, paths, flags) across subcommands more efficiently.
- **Messaging System:** Centralized output handling in a dedicated `message` module, improving consistency and allowing for easier styling updates.
- **CI/CD:** Simplified release process by using `musl` as the default target for all Linux builds.

## [0.1.0] - 2026-01-31

### Added
- **CLI Commands:**
    - `init`: Initialize Dotbee configuration.
    - `list`: List available configuration profiles.
    - `switch <config>`: Apply a specific configuration profile.
    - `doctor`: Validate symlink status and configuration health.
    - `purge`: Remove all active symlinks managed by Dotbee.
    - `repair`: Detect and fix broken or missing symlinks.
- **Features:**
    - `auto_detect_profile` setting: Automatically select a profile based on the system's hostname when no profile is specified in `dotbee switch`.
    - Dry-run mode (`--dry-run`) to preview filesystem changes.
    - Custom config path support (`--config`).
    - Configurable icon styles (Text, Emoji, NerdFont).
    - Global and profile-specific symlink management.
- **Infrastructure:**
    - JSON Schema for `dotbee.toml` to provide LSP completions via Taplo.
    - Project Roadmap (`ROADMAP.md`).
    - GitLab CI configuration for automated multi-platform builds (x86_64, aarch64, musl) and releases.
    - `mise` tasks for containerized development and CI testing.

### Changed
- Refactored `on_conflict` configuration setting to use a typed enum instead of a string.
- Moved `ConflictAction` enum to a dedicated module in `src/config/conflict.rs`.

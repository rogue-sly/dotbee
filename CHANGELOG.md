# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-02-07

### Changed
- `dotsy list` now displays the global profile configuration if one is defined.
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
    - `init`: Initialize Dotsy configuration.
    - `list`: List available configuration profiles.
    - `switch <config>`: Apply a specific configuration profile.
    - `doctor`: Validate symlink status and configuration health.
    - `purge`: Remove all active symlinks managed by Dotsy.
    - `repair`: Detect and fix broken or missing symlinks.
- **Features:**
    - `auto_detect_profile` setting: Automatically select a profile based on the system's hostname when no profile is specified in `dotsy switch`.
    - Dry-run mode (`--dry-run`) to preview filesystem changes.
    - Custom config path support (`--config`).
    - Configurable icon styles (Text, Emoji, NerdFont).
    - Global and profile-specific symlink management.
- **Infrastructure:**
    - JSON Schema for `dotsy.toml` to provide LSP completions via Taplo.
    - Project Roadmap (`ROADMAP.md`).
    - GitLab CI configuration for automated multi-platform builds (x86_64, aarch64, musl) and releases.
    - `mise` tasks for containerized development and CI testing.

### Changed
- Refactored `on_conflict` configuration setting to use a typed enum instead of a string.
- Moved `ConflictAction` enum to a dedicated module in `src/config/conflict.rs`.

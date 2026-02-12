# Dotsy Project Context

## Project Overview

**Dotsy** is a CLI-based dotfiles manager written in **Rust**. It is designed to be simple, easy to use, and focused solely on managing dotfiles via symlinks, avoiding the complexity of other tools like `stow` or `chezmoi`.

**Status:** Early development (Alpha).
**Architecture:**
-   **Language:** Rust (Edition 2024, v1.92.0)
-   **CLI Framework:** `clap`
-   **Configuration:** TOML
-   **Core Logic:** Symlink management (creation, purging, repair), state tracking.

## Key Files & Directories

-   `src/`: Source code.
    -   `main.rs`: Entry point.
    -   `cli.rs`: CLI command definitions and argument parsing.
    -   `subcommands/`: Implementation of specific CLI commands.
    -   `config/`: Configuration handling (TOML parsing, conflict resolution, icons).
    -   `state.rs`: Persistent state management (e.g., active profile).
    -   `utils.rs`: Shared utility functions (path expansion, symlink status, unlinking).
-   `mise.toml`: Project tool configuration and development tasks.
-   `Cargo.toml`: Rust dependencies and package metadata.
-   `schema/dotsy.json`: JSON schema for `dotsy.toml` validation.

## Building and Running

### Prerequisites
-   **Rust:** v1.92.0 (managed via `mise` or `rustup`).
-   **Mise:** Recommended for environment and task management.

### Development Commands

1.  **Build:**
    ```bash
    cargo build
    ```

2.  **Run:**
    ```bash
    cargo run -- <command>
    # Example: cargo run -- list
    ```

3.  **Containerized Environment (Recommended):**
    The project includes `mise` tasks to run Dotsy in a container to avoid accidental data loss on the host system during development.
    ```bash
    # Check available tasks
    mise run build-container
    mise run run-container
    ```

## Development Conventions

-   **Formatting:** Follows standard Rust formatting (`rustfmt.toml` is present). Run `cargo fmt` before committing.
-   **Configuration:** Uses TOML (`dotsy.toml`) for user configuration, validated against `schema/dotsy.json`.
-   **Safety:** Due to the nature of file system operations (symlinking, deletion), testing in a container is highly encouraged.
-   **State:** Uses `~/.local/state/dotsy/state.json` to persist information like the currently active profile.

## CLI Commands (`src/cli.rs`)

-   `init`: Initialize Dotsy by creating a default configuration.
-   `list`: List all available config profiles.
-   `switch <profile>`: Switch to a specific config profile (symlinks files and updates state).
-   `doctor`: Show currently active profile and status of all symlinks.
-   `purge`: Remove all symlinks managed by Dotsy.
-   `repair`: Attempt to fix broken or incorrect symlinks.
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
    -   `config/`: Configuration module, handling TOML parsing, conflict resolution, and icons. Contains `dotsy.toml` as a default/example configuration.
    -   `context/`: Application context management, holding configuration, state, and messaging.
    -   `state/`: Persistent state management (e.g., active profile, managed links).
    -   `utils.rs`: Shared utility functions (path expansion, symlink status, removing links).
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

## Interaction Guidelines

-   **Suggestions & Hints:** Prioritize providing suggestions, architectural hints, and explanations over immediate file edits or command execution.
-   **Show, Don't Just Do:** When proposing changes, show the code blocks in the chat and explain the logic. Only edit files or run commands when explicitly told to do so by the user.
-   **Command Execution:** Explain the purpose and effect of shell commands. Avoid running them directly; instead, present them for review or wait for a specific request to execute.
-   **Collaborative Approach:** Act more as a consultant/guide. Focus on helping the user understand the codebase and the "why" behind changes.

## CLI Commands (`src/cli.rs`)

-   `init`: Initialize Dotsy by creating a default configuration.
-   `list`: List all available config profiles.
-   `switch <profile>`: Switch to a specific config profile (symlinks files and updates state).
-   `doctor`: Show currently active profile and status of all symlinks.
-   `purge`: Remove all symlinks managed by Dotsy.
-   `repair`: Attempt to fix broken or incorrect symlinks.
-   `completion <shell>`: Generate shell completion scripts for supported shells.

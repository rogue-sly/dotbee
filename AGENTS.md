# Dotbee Agent Guide

## Project Overview

**Dotbee** is a CLI dotfiles manager written in Rust. It manages dotfiles via symlinks and is
intentionally minimal — no encryption, no templating, no package management. Just symlinks.

- **Language:** Rust (Edition 2024, `rust-version = "1.92.0"`)
- **CLI Framework:** `clap` (derive API)
- **Config format:** TOML (`dotbee.toml`)
- **State format:** JSON (`~/.local/state/dotbee/state.json`)
- **Status:** Alpha

---

## Build, Run & Test Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run a subcommand
cargo run -- <subcommand>          # e.g. cargo run -- list

# Run all tests
cargo test

# Run a single test by name (substring match)
cargo test <test_name>             # e.g. cargo test test_load_valid_config

# Run all tests in a specific module
cargo test context::manager::config

# Lint (cargo check)
mise run lint                      # runs: cargo check --all

# Format Rust + TOML
mise run format                    # alias: mise run fmt
mise run format:rust               # runs: cargo fmt --all
mise run format:toml               # runs: taplo format --check --diff

# Bump version (major | minor | bugfix | literal e.g. 1.2.3)
mise run bump-version <increment>

# Test in a container (safe — avoids mutating host dotfiles)
mise run try-dotbee                 # builds binary, spins up Docker container with example dotfiles
```

**Always run `cargo fmt` before committing.** The project enforces `max_width = 140` and
`edition = "2024"` via `rustfmt.toml`.

---

## Source Layout

```
src/
├── main.rs                         # Entry point: parse CLI, build Context, dispatch subcommand
├── cli.rs                          # Clap structs: Cli, SubCommand, Shell
├── utils.rs                        # expand_tilde(), get_hostname()
├── subcommands/                    # One file per CLI subcommand
│   ├── completion.rs
│   ├── doctor.rs
│   ├── init.rs
│   ├── list.rs
│   ├── purge.rs
│   ├── repair.rs
│   └── switch.rs
└── context/
    ├── mod.rs                      # Context struct: owns Manager + Message + dry_run flag
    ├── message.rs                  # Coloured terminal output (success/error/warning/info/link/…)
    └── manager/
        ├── mod.rs                  # Manager struct: owns ConfigManager, StateManager, SymlinkManager
        ├── symlink.rs              # SymlinkManager + SymlinkStatus enum
        ├── config/
        │   ├── mod.rs              # ConfigManager (pure TOML data + config_path metadata)
        │   ├── conflict.rs         # ConflictAction enum + interactive prompt
        │   ├── icons.rs            # IconStyle enum + Icons struct
        │   └── dotbee.toml          # Default config template (embedded via include_str!)
        └── state/
            └── mod.rs              # StateManager + ManagedLink (auto-saves on every mutation)
```

Key supporting files:
- `Cargo.toml` — dependencies and package metadata
- `schema/dotbee.json` — JSON schema for `dotbee.toml` (enables Taplo LSP completions)
- `mise.toml` / `.mise-tasks/` — dev task runner
- `example/` — sample dotfiles directory used by the containerised test environment

---

## Architecture

### Context → Manager → {ConfigManager, StateManager, SymlinkManager}

`Context` is passed by reference to every subcommand. It contains:

- `manager.config` — `ConfigManager`: wraps parsed TOML, exposes read-only accessors
- `manager.state` — `StateManager`: wraps JSON state, **auto-saves on every mutation**
- `manager.symlink` — `SymlinkManager`: stateless filesystem operations
- `message` — `Message`: coloured terminal output
- `dry_run: bool` — subcommands check this before touching the filesystem

### Plan / Execute Pattern

`switch`, `repair`, and `purge` all follow this two-phase pattern:

1. **`generate_plan()`** — reads config + state, returns `Vec<Action>` with no side effects
2. **`execute()` / `execute_dry_run()`** — applies or describes the plan

New subcommands that mutate the filesystem should follow this pattern.

### State persistence

`StateManager` persists to `~/.local/state/dotbee/state.json`. Every setter calls `save()`
internally — callers do not manage persistence manually.

---

## Code Style Guidelines

### Formatting
- Line width: **140 characters** (`rustfmt.toml`)
- Edition: **2024**
- Run `cargo fmt` before every commit. CI will reject unformatted code.

### Imports
- Group standard library, then external crates, then internal `crate::` paths.
- Use explicit paths rather than glob imports (`use foo::*` only in `#[cfg(test)]` via `use super::*`).
- Prefer importing the type rather than the full path at call site:
  ```rust
  // preferred
  use std::path::{Path, PathBuf};

  // avoid
  std::path::PathBuf::from(...)
  ```
- Multi-item imports from the same module should use braces:
  ```rust
  use std::{error::Error, fs, path::Path};
  ```

### Types and Error Handling
- Use `Box<dyn Error>` as the error type throughout — no custom error enums yet.
- Return `Result<(), Box<dyn Error>>` from subcommand `run()` functions.
- Use `.into()` to convert string literals to boxed errors:
  ```rust
  return Err("No profile specified.".into());
  ```
- Use `ok_or_else` with a closure for lazily-allocated error strings:
  ```rust
  profiles.get(name).ok_or_else(|| format!("Profile '{}' not found.", name).into())
  ```

### Functional Style
- Prefer functional chains over nested `if let` or `match`:
  ```rust
  // preferred
  context.manager.state
      .get_active_profile()
      .and_then(|p| context.manager.config.get_profile(p).ok())

  // avoid
  if let Some(p) = context.manager.state.get_active_profile() {
      if let Ok(profile) = context.manager.config.get_profile(p) { ... }
  }
  ```
- Use `.map()`, `.and_then()`, `.unwrap_or_default()`, `.unwrap_or_else()`, `.filter()` freely.

### Naming Conventions
- Types and enums: `PascalCase`
- Functions, methods, variables, modules: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Unused-but-intentionally-kept fields/methods: prefix with `_` (e.g. `_force_create`, `_source_path`)

### Structs and Visibility
- Keep internal data private; expose behaviour through methods.
- Pub fields on structs are acceptable when the struct is a simple data carrier (e.g. `Profile`, `Global`, `ManagedLink`).
- `ConfigManager`, `StateManager`, and `SymlinkManager` all keep their inner data private.

### Doc Comments
- Public methods on manager structs should have `///` doc comments.
- Document `# Arguments`, `# Returns`, and `# Errors` / `# Panics` sections where non-obvious.
- Internal helpers don't require doc comments but benefit from a one-liner.

### Tests
- Tests live in `#[cfg(test)] mod tests` at the bottom of the file they test.
- Use `tempfile::tempdir()` for any test that touches the filesystem.
- For `StateManager` tests, construct `StateManager { state }` directly (same module) to avoid writing to `~/.local/state/dotbee/` during test runs.
- Test names follow `test_<what>_<condition>` (e.g. `test_get_profile_not_found`).
- Prefer `assert!(result.is_ok())` / `assert!(result.is_err())` before unwrapping to get clearer failure messages.

---

## Adding a New Subcommand

1. Add a variant to `SubCommand` in `src/cli.rs`.
2. Create `src/subcommands/<name>.rs` with a `pub fn run(context: ...) -> Result<(), Box<dyn Error>>`.
3. Add `pub mod <name>;` to `src/subcommands/mod.rs` (or the inline module list in `main.rs`).
4. Match the new variant in `main.rs`.
5. If the command mutates the filesystem, use the Plan / Execute pattern.

---

## Safety Notes

- **Always test filesystem-mutating changes in the container** (`mise run try-dotbee`) to avoid
  accidentally symlinking or deleting files on the host.
- The container mounts `example/` as the dotfiles directory and uses `hostname=laptop`.
- `StateManager` will overwrite `~/.local/state/dotbee/state.json` on the host whenever any
  setter is called during a non-containerised run.

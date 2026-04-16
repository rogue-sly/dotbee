# Dotbee Roadmap

This document outlines the planned development path for **Dotbee**. As an alpha project, the primary focus is moving toward a stable, reliable `v1.0.0` release.

## Phase 1: Foundation & Validation (Current)

_Goal: Solidify the core specification and ensure reliability._

- [x] **Formalize Specification:** Finalize the `dotbee.toml` format (JSON Schema provided).
- [x] **LSP Support:** Complete the `schema/dotbee.json` JSON schema to provide completions and validation via Taplo.
- [x] **Shell Completions:** Provide completions for `bash`, `zsh` and `fish`.
- [x] **Documentation:** Finalize the `README.md` and establish a `CHANGELOG.md`.
- [x] **Wiki:** Write a wiki that explains everything about dotbee.
- [x] **Usage Examples:** Write usage examples and troubleshooting tips.
- [x] **Base Directory Resolution:** Fix CWD-dependency by resolving relative paths from the config file's location.
- [x] **Core Testing:** Implement a comprehensive test suite for symlink management (creation, purging, repair) and edge cases.
- [ ] **Cross-Platform Support:** Verify and polish experience on macOS and Termux.
- [ ] **Explicit Defaults:** Warn user when running without a configuration file.

## Phase 2: Safety & Reliability (Beta)

_Goal: Ensure users can trust Dotbee with their configuration files._

- [ ] **Auto-Backup System:** Automatically back up existing files before they are replaced or modified by a `switch`.
- [ ] **Android/Termux Safety:** Implement a custom Trash implementation (Freedesktop.org spec) for Android/Termux to avoid permanent deletion.
- [x] **Transaction-Based Execution:** Separate planning from execution to enable reliable dry-runs and potential undo functionality.
- [ ] **Unified Error Recovery:** Implement standard strategy for partial failures during multi-file operations.
- [ ] **State Resilience:** Improve error reporting for corrupted state files instead of silent failure.
- [ ] **Path Security:** Sanitize source paths to prevent directory traversal and ensure they stay within bounds.
- [ ] **Canonical Path Comparison:** Use canonical paths for all comparisons to prevent false conflict reports.
- [ ] **Link Type Verification:** Proactively verify file vs. directory mismatches during planning.
- [ ] **Robust File Operations:** Handle cross-filesystem moves in `Adopt` strategy and remove unsafe `.unwrap()` calls.
- [ ] **Graceful Signal Handling:** Handle SIGINT (Ctrl+C) to safely finish or rollback operations.
- [ ] **Enhanced Error Recovery:** Improve the `repair` command and provide more meaningful error messages.
- [ ] **State Consistency:** Ensure `repair` synchronizes `state.json` with the current configuration.
- [ ] **Robust Testing:** Add tests for core functionality and complex edge cases.
- [ ] **CI Integration:** Fully utilize GitLab CI for automated linting, testing, and multi-platform builds.

## Phase 3: Portability & Polish (`v1.0`)

_Goal: Broaden support and optimize the user experience._

- [ ] **Runtime Schema Validation:** Enforce `dotbee.toml` schema validation at runtime during config load.
- [ ] **CI Pipeline Modernization:** Add dedicated test stages and better artifact management.
- [ ] **Centralized CI Dependencies:** Synchronize CI toolchain versions with project config.
- [ ] **Broader Platform Support:** Support even more platforms and provide packages for popular Linux distros (Debian, Fedora, ArchLinux, Nix/OS).
- [ ] **Performance Optimization:** Implement bulk state updates to improve efficiency for large configurations.
- [ ] **Refinement:** Polish CLI output (icons, colors, and progress indicators).
- [ ] **Stable Release:** Tag and release `v1.0.0`.

## Completed (`v0.1.0`)

- [x] Implement `dotbee init`
- [x] Implement `dotbee list`
- [x] Implement `dotbee switch <config>`
- [x] Implement `dotbee doctor`
- [x] Implement `dotbee purge`
- [x] Implement `dotbee repair`
- [x] Dry-run mode (`--dry-run`)
- [x] Custom config path (`--config`)
- [x] `auto_detect_profile` setting implementation
- [x] JSON Schema for LSP support (`dotbee.json`)
- [x] Initialize `CHANGELOG.md` and merge `TODO.md` into `ROADMAP.md`

## Future Explorations

- **System Profile:** A way to setup symlinks for system configuration files (/etc)
  - Should invoke sudo
  - To activate it, use --system with `dotbee switch` Possible commands to use this flag: \[switch, purge, repair\]

- **Dotfiles Fetching:** Dotbee should be able to fetch a remote dotfiles repository
  - Using git
  - Using http (simply download it)
  - perhaps some other methods

To infinity and beyond...

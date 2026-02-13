# Dotsy Roadmap

This document outlines the planned development path for **Dotsy**. As an alpha project, our primary focus is moving toward a stable, reliable `v1.0.0` release.

## Phase 1: Foundation & Validation (Current)

_Goal: Solidify the core specification and ensure reliability._

- [x] **Formalize Specification:** Finalize the `dotsy.toml` format (JSON Schema provided).
- [x] **LSP Support:** Complete the `schema/dotsy.json` JSON schema to provide completions and validation via Taplo.
- [x] **Shell Completions:** Provide completions for `bash`, `zsh` and `fish`.
- [x] **Documentation:** Finalize the `README.md` and establish a `CHANGELOG.md`.
- [ ] **Wiki:** Write a wiki that explains everything about dotsy.
- [ ] **Usage Examples:** Write usage examples and troubleshooting tips.
- [ ] **Base Directory Resolution:** Fix CWD-dependency by resolving relative paths from the config file's location.
- [ ] **Code Documentation:** Add inline comments and docstrings to the codebase.
- [ ] **Core Testing:** Implement a comprehensive test suite for symlink management (creation, purging, repair) and edge cases.
- [ ] **Cross-Platform Support:** Verify and polish experience on macOS and Termux.

## Phase 2: Safety & Reliability (Beta)

_Goal: Ensure users can trust Dotsy with their configuration files._

- [ ] **Auto-Backup System:** Automatically back up existing files before they are replaced or modified by a `switch`.
- [ ] **Atomic Switching:** Optimize `switch` to only update changed links and avoid unnecessary deletions.
- [ ] **Robust File Operations:** Handle cross-filesystem moves in `Adopt` strategy and remove unsafe `.unwrap()` calls.
- [ ] **Execution Safety:** Re-verify file types immediately before deletion to prevent race conditions.
- [ ] **Enhanced Error Recovery:** Improve the `repair` command and provide more meaningful error messages.
- [ ] **State Consistency:** Ensure `repair` synchronizes `state.json` with the current configuration.
- [ ] **Robust Testing:** Add tests for core functionality and complex edge cases.
- [ ] **CI Integration:** Fully utilize GitLab CI for automated linting, testing, and multi-platform builds.

## Phase 3: Portability & Polish (`v1.0`)

_Goal: Broaden support and optimize the user experience._

- [ ] **Broader Platform Support:** Support even more platforms and provide packages for popular Linux distros (Debian, Fedora, ArchLinux, Nix/OS).
- [ ] **Performance Optimization:** Implement bulk state updates to improve efficiency for large configurations.
- [ ] **Refinement:** Polish CLI output (icons, colors, and progress indicators).
- [ ] **Stable Release:** Tag and release `v1.0.0`.

## Completed (`v0.1.0`)

- [x] Implement `dotsy init`
- [x] Implement `dotsy list`
- [x] Implement `dotsy switch <config>`
- [x] Implement `dotsy doctor`
- [x] Implement `dotsy purge`
- [x] Implement `dotsy repair`
- [x] Dry-run mode (`--dry-run`)
- [x] Custom config path (`--config`)
- [x] `auto_detect_profile` setting implementation
- [x] JSON Schema for LSP support (`dotsy.json`)
- [x] Initialize `CHANGELOG.md` and merge `TODO.md` into `ROADMAP.md`

## Future Explorations

- **System Profile:** A way to setup symlinks for system configuration files (/etc)
  - Should invoke sudo
  - To activate it, use --system with `dotsy switch`
  Possible commands to use this flag: [switch, purge, repair]

- **Dotfiles Fetching:** Dotsy should be able to fetch a remote dotfiles repository
  - Using git
  - Using http (simply download it)
  - perhaps some other methods

To infinity and beyond...

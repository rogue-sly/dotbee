# Dotbee Roadmap

This document outlines the planned development path for **Dotbee**. As an alpha project, the primary focus is moving toward a stable, reliable `v1.0.0` release.

## Phase 1: Foundation & Validation (Current)

_Goal: Solidify the core specification and ensure reliability._

- [x] `Formalize Specification:` Finalize the `dotbee.toml` format (JSON Schema provided).
- [x] `LSP Support:` Complete the `schema/dotbee.json` JSON schema to provide completions and validation via Taplo.
- [x] `Shell Completions:` Provide completions for `bash`, `zsh` and `fish`.
- [x] `Documentation:` Finalize the `README.md` and establish a `CHANGELOG.md`.
- [x] `Wiki:` Write a wiki that explains everything about dotbee.
- [x] `Usage Examples:` Write usage examples and troubleshooting tips.
- [x] `Base Directory Resolution:` Fix CWD-dependency by resolving relative paths from the config file's location.

## Phase 2: Safety & Reliability (Beta)

_Goal: Ensure users can trust Dotbee with their configuration files._

- [x] `Transaction-Based Execution:` Separate planning from execution to enable reliable dry-runs and potential undo functionality.
- [x] `State Consistency:` Ensure `repair` synchronizes `state.json` with the current configuration.
- [ ] `Cross-Platform Support:` Verify and polish experience on macOS and Termux.
- [x] `Runtime Schema Validation:` Enforce `dotbee.toml` schema validation at runtime during config load.

## Phase 3: Portability & Polish (`v1.0`)

_Goal: Broaden support and optimize the user experience._

- [ ] `Broader Platform Support:` Support even more platforms and provide packages for popular Linux distros (Debian, Fedora, ArchLinux, Nix/OS).
- [ ] `Stable Release:` Tag and release `v1.0.0`.

## Optional

- [ ] `Performance Optimization:` Implement bulk state updates to improve efficiency for large configurations.
- [ ] `Refinement:` Polish CLI output (icons, colors, and progress indicators).
- [ ] `State Resilience:` Improve error reporting for corrupted state files instead of silent failure.

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

- `System Profile:` A way to setup symlinks for system configuration files (/etc)
  - Should invoke sudo
  - To activate it, use --system with `dotbee switch` Possible commands to use this flag: \[switch, purge, repair\]

- `Dotfiles Fetching:` Dotbee should be able to fetch a remote dotfiles repository
  - Using git
  - Using http (simply download it)
  - perhaps some other methods

To infinity and beyond...

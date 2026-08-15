# Dotbee Roadmap

This document outlines the planned development path for **Dotbee**. As an alpha project, the primary focus is moving toward a stable, reliable `v1.0.0` release.

## Completed (`v0.1.0`).

- [x] Implement `dotbee init`
- [x] Implement `dotbee list`
- [x] Implement `dotbee switch <config>`
- [x] Implement `dotbee doctor`
- [x] Implement `dotbee purge`
- [x] Implement `dotbee repair`
- [x] Dry-run mode (`--dry-run`)
- [x] Custom config path (`--config`)
- [x] ~~`auto_detect_profile` setting implementation~~
- [x] JSON Schema for LSP support (`dotbee.json`)
- [x] Initialize `CHANGELOG.md` and merge `TODO.md` into `ROADMAP.md`

## Phase 1: Core.

- [x] **Formalize Specification:** Finalize the `dotbee.toml` format (JSON Schema provided).
- [x] **LSP Support:** Complete the `schema/dotbee.json` JSON schema to provide completions and validation via [tombi](https://github.com/tombi-toml/tombi).
- [x] **Shell Completions:** Provide completions for `bash`, `zsh` and `fish`.
- [x] **Documentation:** Finalize the `README.md` and establish a `CHANGELOG.md`.
- [x] **Base Directory Resolution:** Fix CWD-dependency by resolving relative paths from the config file's location.

## Phase 2: Reliability.

- [x] **Transaction-Based Execution:** Separate planning from execution to enable reliable dry-runs.
- [x] **State Consistency:** Ensure `repair` synchronizes `state.json` with the current configuration.
- [x] **Cross-Platform Support:** Verify and polish experience on macOS and Termux.
- [x] **Runtime Schema Validation:** Enforce `dotbee.toml` schema validation at runtime during config load.
- [x] **Clearer Link Names:** instead of `"~/path/to/destination" = "path/to/source"`, now it's `my_config = {src = "path/to/source", dst = "~/path/to/destination"}`.

## Phase 2.5: Moar features.

- [x] **`dotbee edit` Command:** Open dotbee.toml config file directly using `$EDITOR`.
- [ ] **Variable Interpolation:** Declare variables in dotbee.toml to reuse in your links.

```toml
[vars]
config = "~/.config"

[profiles.desktop.links]
nvim = { src = "editors/nvim/", dst = "{config}/nvim" }
```

- [ ] **Pre/Post Hooks:** Run shell commands before/after profile switch (useful for things like enabling systemd services, but I wouldn't recommend for installing packages.).
- [ ] **Dotfiles Fetching:** Dotbee should be able to fetch a remote dotfiles repository.
  - Using git
  - Using http (simply download it)
  - Perhaps some other methods

- [ ] **System Profile:** A way to setup symlinks for system configuration files (/etc)
  - Should invoke sudo
  - To activate it, use --system with `dotbee switch` Possible commands to use this flag: [switch, purge, repair]

## Phase 3: Polish (`v1.0`)

- [ ] **Broader Platform Support:** Support even more platforms and provide packages for popular Linux distros (Debian, Fedora, ArchLinux, Nix/OS).
- [ ] **Stable Release:** Tag and release `v1.0.0`.
- [ ] **Complete Test Coverage:** Write tests for all possible failure cases.
- [ ] **Man Pages:** Write a manpage that covers all dotbee features.

## Optional

- [ ] **Performance Optimization:** Implement bulk state updates to improve efficiency for large configurations.
- [ ] **Copy mode**: Copy the file instead of creating a symlink.

To infinity and beyond...

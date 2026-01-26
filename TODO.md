# TODO's

## Spec

- [ ] Write complete spec file for dotsy `dotsy.toml`
  - After finalizing the configuration file, write a `dotsy.json` file
    so taplo lsp can provide completions for it

## Core Features

- [x] Implement `dotsy init`
- [x] Implement `dotsy list`
  - List configs(a config is a collection of configuration files)
- [x] Implement `dotsy switch <config>`
- [x] Implement `dotsy doctor`.
  - Shows the status of the selected config
  - Displays each config whether it's symlink is broken or not
- [x] Implement `dotsy purge`
  - Remove all active symlinks.
- [x] Implement `dotsy repair`
  - Detect and fix missing or broken symlinks for the active config

## Error Handling & UX

- [x] Provide meaningful error messages for:
  - Missing configs during switch.
  - Permission issues with symlinks.

## Documentation

- [ ] Finalize and maintain `README.md`.
- [ ] Write usage examples and troubleshooting tips.
- [ ] Add inline comments and docstrings to codebase.

## Future Enhancements

- [x] (--dry-run) Dry-run mode to preview changes before applying.
- [x] (--config) For specifying custom config path
- [ ] Auto-backup current dotfiles before switching or purging.
- [ ] Cross-platform support improvements (macOS, Termux, W*ndows?(most likely not)).
- [ ] Add tests for core functionality and edge cases.

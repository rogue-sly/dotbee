# Dotsy TODO

## Core Features

- [ ] Implement `dotsy init`
- [ ] Implement `dotsy list`
  - List configs(a config is a collection of configuration files)
- [ ] Implement `dotsy switch <config>`
- [ ] Implement `dotsy status`.
  - Shows the status of the selected config
  - Displays each config whether it's symlink is broken or not
- [ ] Implement `dotsy purge`
  - Remove all active symlinks.
- [ ] Implement `dotsy repair`
  - Detect and fix missing or broken symlinks for the active config

## Error Handling & UX

- [ ] Provide meaningful error messages for:
  - Missing configs during switch.
  - Permission issues with symlinks.
- [ ] Add confirmation prompts for destructive operations (e.g., purge).
- [ ] Support dry-run mode for previewing changes (planned).

## Documentation

- [ ] Finalize and maintain `README.md`.
- [ ] Write usage examples and troubleshooting tips.
- [ ] Add inline comments and docstrings to codebase.

## Future Enhancements

- [ ] Dry-run mode to preview changes before applying.
- [ ] Auto-backup current dotfiles before switching or purging.
- [ ] Cross-platform support improvements (macOS, Termux, Windows?(most likely not)).
- [ ] Add tests for core functionality and edge cases.

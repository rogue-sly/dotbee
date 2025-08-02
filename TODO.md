# Dotsy TODO

## Core Features

- [x] Implement `dotsy init <name>`
  - Create dotfiles root folder structure with `hosts/` and default `global/`.
- [x] Implement `dotsy list`
  - List all available hosts inside the `hosts/` folder.
  - List their configs(a config is a collection of configuration files)
- [ ] Implement `dotsy switch <host>`
  - Remove current active symlinks.
  - Symlink files from `hosts/global` and `hosts/<host>` into home directory.
  - Prompts user when there's an already existing config file (write over, ignore or adopt)
  - Update the state file with the active host.
- [ ] Implement `dotsy status`.
  - Shows the status of the selected host
  - Displays each config whether it's symlink is broken or not
- [ ] Implement `dotsy purge`
  - Remove all active symlinks.
  - Clear/reset the state file.
- [ ] Implement `dotsy repair`
  - Detect and fix missing or broken symlinks for the active host.

## State Management

- [ ] Design and maintain state file at `~/.local/state/dotsy.json`.
- [ ] Ensure atomic updates to state during switching and purging.

## Error Handling & UX

- [ ] Provide meaningful error messages for:
  - Missing hosts during switch.
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
- [ ] Git integration per host for versioning and syncing.
- [ ] Add tests for core functionality and edge cases.

# Dotbee

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.3.0-blue)](https://gitlab.com/rogue87/dotbee)

**Dotbee** is a simple, symlink-based dotfiles manager written in Rust. It focuses on doing one thing well: managing your configuration files without the complexity of shell scripts or bloated feature sets.

> [!WARNING]
> Dotbee is in **Alpha**. While functional, it is recommended to back up your dotfiles before use. For testing, use the provided containerized environment.

## Features

- **Profile Support:** Switch between different environments (e.g., Desktop, Server, Termux).
- **Global Configs:** Define links that apply across all profiles.
- **Health Checks:** `dotbee doctor` and `repair` help you identify and fix broken symlinks.
- **LSP Support:** Full JSON schema provided for autocompletion in `dotbee.toml`.
- **Dry Run:** Preview changes with `--dry-run` before applying them.

## Documentation

For more details, see the [GitLab Wiki](https://gitlab.com/rogue87/dotbee/-/wikis).

## Installation

### Using [Mise](https://mise.jdx.dev/)

```bash
mise use gitlab:rogue87/dotbee
```

### From Source

```bash
cargo install --git https://gitlab.com/rogue87/dotbee
```

## Quick Start

1. **Initialize:**
   ```bash
   dotbee init
   ```
   This creates a default `dotbee.toml` in your current directory.

2. **Configure:**
   Edit `dotbee.toml` to define your links.
   ```toml
   [global.links]
   "~/.gitconfig" = "git/gitconfig"

   [profiles.desktop.links]
   "~/.config/i3/config" = "i3/config"
   ```

3. **List Available Profiles:**

   ```bash
   dotbee list
   ```

4. **Switch Profile:**

   ```bash
   dotbee switch desktop
   ```

5. **Check Status:**

   ```bash
   dotbee doctor
   ```

6. **Remove All Symlinks:**

   ```bash
   dotbee purge
   ```

## Shell Completions

Dotbee can generate completion scripts for your shell.

```bash
dotbee completion <SHELL>
```

Supported shells: `bash`, `zsh`, `fish`, `elvish`.

### Permanent Installation

| Shell      | Command                                                                    |
| :--------- | :------------------------------------------------------------------------- |
| **Bash**   | `dotbee completion bash > ~/.local/share/bash-completion/completions/dotbee` |
| **Zsh**    | `dotbee completion zsh > ~/.zfunc/_dotbee`                                   |
| **Fish**   | `dotbee completion fish > ~/.config/fish/completions/dotbee.fish`            |
| **Elvish** | `dotbee completion elvish > ~/.config/elvish/lib/dotbee.elv`                 |

### On the fly

You can also load completions directly into your current session:

> [!TIP]
> For **Zsh**, ensure `~/.zfunc` is in your `$fpath` by adding `fpath+=~/.zfunc` to your `.zshrc` before calling `compinit`.
> For **Elvish**, after permanent installation, add `use dotbee` to your `rc.elv`.

```bash
# Fish
dotbee completion fish | source

# Bash
source <(dotbee completion bash)

# Zsh
source <(dotbee completion zsh)

# Elvish
eval (dotbee completion elvish | slurp)
```

## Configuration

Dotbee uses TOML for configuration.

### Example `dotbee.toml`

```toml
[settings]
on_conflict = "ask"
icon_style = "nerdfont"

[global.links]
"~/.bashrc" = "bashrc"
```

## Development & Testing

To avoid accidental data loss on your host system during development, use the provided `mise` tasks to run Dotbee in a container:

```bash
mise run try-dotbee --profile dev
```

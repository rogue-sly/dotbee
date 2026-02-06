# Dotsy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://gitlab.com/rogue87/dotsy)

**Dotsy** is a simple, symlink-based dotfiles manager written in Rust. It focuses on doing one thing well: managing your configuration files without the complexity of shell scripts or bloated feature sets.

> [!WARNING]
> Dotsy is in **Alpha (`v0.1.0`)**. While functional, it is recommended to back up your dotfiles before use. For testing, use the provided containerized environment.

## Features

- **Profile Support:** Switch between different environments (e.g., Desktop, Server, Termux).
- **Global Configs:** Define links that apply across all profiles.
- **Health Checks:** `dotsy doctor` and `repair` help you identify and fix broken symlinks.
- **LSP Support:** Full JSON schema provided for autocompletion in `dotsy.toml`.
- **Dry Run:** Preview changes with `--dry-run` before applying them.

## Installation

### Using [Mise](https://mise.jdx.dev/)

```bash
mise use gitlab:rogue87/dotsy
```

### From Source

```bash
cargo install --git https://gitlab.com/rogue87/dotsy
```

## Quick Start

1. **Initialize:**
   ```bash
   dotsy init
   ```
   This creates a default `dotsy.toml` in your current directory.

2. **Configure:**
   Edit `dotsy.toml` to define your links.
   ```toml
   [global.links]
   "~/.gitconfig" = "git/gitconfig"

   [profiles.desktop.links]
   "~/.config/i3/config" = "i3/config"
   ```

3. **List Available Profiles:**

   ```bash
   dotsy list
   ```

4. **Switch Profile:**

   ```bash
   dotsy switch desktop
   ```

5. **Check Status:**

   ```bash
   dotsy doctor
   ```

6. **Remove All Symlinks:**

   ```bash
   dotsy purge
   ```

## Shell Completions

Dotsy can generate completion scripts for your shell.

```bash
dotsy completion <SHELL>
```

Supported shells: `bash`, `zsh`, `fish`, `elvish`.

### Permanent Installation

| Shell      | Command                                                                    |
| :--------- | :------------------------------------------------------------------------- |
| **Bash**   | `dotsy completion bash > ~/.local/share/bash-completion/completions/dotsy` |
| **Zsh**    | `dotsy completion zsh > ~/.zfunc/_dotsy`                                   |
| **Fish**   | `dotsy completion fish > ~/.config/fish/completions/dotsy.fish`            |
| **Elvish** | `dotsy completion elvish > ~/.config/elvish/lib/dotsy.elv`                 |

### On the fly

You can also load completions directly into your current session:

> [!TIP]
> For **Zsh**, ensure `~/.zfunc` is in your `$fpath` by adding `fpath+=~/.zfunc` to your `.zshrc` before calling `compinit`.
> For **Elvish**, after permanent installation, add `use dotsy` to your `rc.elv`.

```bash
# Fish
dotsy completion fish | source

# Bash
source <(dotsy completion bash)

# Zsh
source <(dotsy completion zsh)

# Elvish
eval (dotsy completion elvish | slurp)
```

## Configuration

Dotsy uses TOML for configuration.

### Example `dotsy.toml`

```toml
[settings]
on_conflict = "ask"
icon_style = "nerdfont"

[global.links]
"~/.bashrc" = "bashrc"
```

## Development & Testing

To avoid accidental data loss on your host system during development, use the provided `mise` tasks to run Dotsy in a container:

```bash
mise run build-container
mise run run-container
```

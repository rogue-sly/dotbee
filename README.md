# Dotbee

![Github License](https://img.shields.io/github/license/rogue-sly/dotbee)
![Github Version](https://img.shields.io/github/v/release/rogue-sly/dotbee)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/dotbee)

**Dotbee** is a simple, symlink-based dotfiles manager. It focuses on doing one thing well: managing your configuration files without the complexity of shell scripts or bloated feature sets.

## Features

- **Profile Support:** Switch between different environments (e.g., Desktop, Server, Termux).
- **Global Configs:** Define links that apply across all profiles via the reserved `profiles.global` profile.
- **Health Checks:** `dotbee doctor` and `dotbee sync` help you identify and fix broken symlinks.
- **LSP Support:** Full JSON schema provided for autocompletion in `dotbee.toml`.
- **Dry Run:** Preview changes with `--dry-run` before applying them.

## Documentation

Check the [Roadmap](ROADMAP.md) to see current progress of dotbee's development.

## Installation

### Using [Mise](https://mise.jdx.dev/)

```sh
mise use github:rogue-sly/dotbee
```

### Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```sh
cargo binstall dotbee
```

### Using your package manager

1. apt

- x86_64/amd64

```sh
# fetch the package first
wget https://github.com/rogue-sly/dotbee/-/releases/v.../downloads/dotbee-v...-amd64.deb
# then install it
sudo apt install ./dotbee-v...-amd64.deb
```

- aarch64/arm64

```sh
# fetch the package first
wget https://github.com/rogue-sly/dotbee/-/releases/v.../downloads/dotbee-v...-arm64.deb
# then install it
sudo apt install ./dotbee-v...-arm64.deb
```

2. dnf/yum

- x86_64/amd64

```sh
sudo dnf install https://github.com/rogue-sly/dotbee/-/releases/v.../downloads/dotbee-v...-x86_64.rpm
```

- aarch64/arm64

```sh
sudo dnf install https://github.com/rogue-sly/dotbee/-/releases/v.../downloads/dotbee-...-aarch64.rpm
```

### From Source

1. From [crates.io](https://crates.io/crates/dotbee):

```sh
cargo install dotbee
```

2. Directly From Repository:

```sh
cargo install --git https://github.com/rogue-sly/dotbee
```

## Quick Start

1. **Initialize:**

```sh
dotbee init
```

This creates a default `dotbee.toml` in your current directory.

2. **Configure:**

Edit `dotbee.toml` to define your links.

```toml
[profiles.global.links]
gitconfig = { src = "git/gitconfig", dst = "~/.gitconfig" }

[profiles.desktop.links]
i3_config = { src = "i3/config", dst = "~/.config/i3/config" }
```

3. **List Available Profiles:**

```sh
dotbee list
```

4. **Sync Profile:**

```sh
dotbee sync --profile desktop
```

5. **Check Status:**

```sh
dotbee doctor
```

6. **Remove All Symlinks:**

```sh
dotbee purge
```

## Shell Completions

Dotbee can generate completion scripts for your shell.

```sh
dotbee completion <SHELL>
```

Supported shells: `bash`, `zsh`, `fish`, `elvish`.

### Permanent Installation

| Shell      | Command                                                                      |
| :--------- | :--------------------------------------------------------------------------- |
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
source <(dotbee completion zsh) # make sure to have `autoload -Uz compinit && compinit` in your .zshrc!

# Elvish
eval (dotbee completion elvish | slurp) # slurp lol
```

## Configuration

Dotbee uses TOML for configuration.

### Example `dotbee.toml`

```toml
[settings]
on_conflict = "ask"

[profiles.global.links]
bashrc = { src = "bashrc", dst = "~/.bashrc" }
```

### Variables

Define reusable variables under `[vars]` and reference them in link `src`/`dst` paths with `{name}`:

```toml
[vars]
config = "~/.config"
repos = "~/git"

[profiles.desktop.links]
nvim = { src = "editors/nvim/", dst = "{config}/nvim" }
```

Variables are plain strings and are not interpolated themselves. An undefined `{name}` causes a configuration error.

## Development & Testing

To avoid accidental data loss on your host system during development, use the provided `mise` tasks to run Dotbee in a container:

```sh
mise run try-dotbee --profile dev
```

### Acknowledgments

- [Dotsy](https://github.com/NICHTJ3/Dotsy): My project was initially named dotsy until I discovered there's another project that does the same stuff as mine on crates.io lol xD.

- [Stow](https://www.gnu.org/software/stow/): While I kinda hated how stow works, I have to admit it's very plain and simple dotfiles management tool which is something I really appreciate. I don't like that fact that I have to organize my files in a specific way and run some scripts to make things work. It was also kind difficult to undo things with stow or fix up any broken symlinks.

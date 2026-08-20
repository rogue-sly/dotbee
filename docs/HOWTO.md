# How to Use Dotbee

## CLI

1. **Initialize:**
   This creates a default `dotbee.toml` in your current directory.

```sh
dotbee init
```

2. **List Available Profiles:**
   Gives a quick view of what configs you have available.

```sh
dotbee list
```

3. **Sync Profile:**
   Creates the symlinks (you must pick a profile with `--profile` flag!)

```sh
dotbee sync --profile desktop
```

4. **Check Status:**
   Checks for issues like broken/missing symlinks

```sh
dotbee doctor
```

5. **Remove All Symlinks:**
   Removes all symlinks and resets the state file

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
# Bash
source <(dotbee completion bash)

# Zsh
source <(dotbee completion zsh) # make sure to have `autoload -Uz compinit && compinit` in your .zshrc!

# Fish
dotbee completion fish | source

# Elvish
eval (dotbee completion elvish | slurp)
```

## Configuration

Dotbee uses TOML for configuration.

### Example `dotbee.toml`

```toml
[settings]
# when a conflict occurs during creation of symlinks, you can tell dotbee how to handle it
# by default, it'll ask you how to handle every single case
on_conflict = "ask" # abort, adopt, overwrite, skip, ask (default)

# the global profile is special
# it pretty much makes it possible to have shared configs among multiple profiles
# like for example, you'll probably have the same gitconfig across
# all of your machines right?
[profiles.global.links]
# what you see on the left handside is just the label for you config
# on the right handside, you define the source of config file (src)
# and the location for your target (dst)
gitconfig = { src = "configs/gitconfig", dst = "~/.gitconfig" }
```

### Variables

Define reusable variables under `[vars]` and reference them in link `src`/`dst` paths with `{name}`:

```toml
[vars]
cfg = "~/.config" # neat shortcut
bad_var = "{cfg}/foo/bar" # this will result into an error! vars are only interpolated in dst & src

[profiles.desktop.links]
nvim = { src = "editors/nvim/", dst = "{cfg}/nvim" }
```

Variables are plain strings and are not interpolated themselves. An undefined `{name}` causes a configuration error.

---
title: DOTBEE
section: 1
header: General Commands Manual
footer: dotbee 0.9.0
date: August 24, 2026
---

# NAME

dotbee - Easy to use dotfiles manager

# SYNOPSIS

**dotbee** [OPTIONS] <COMMAND>

# DESCRIPTION

dotbee is a lightweight manager designed to simplify the handling of dotfiles across different environments. It allows users to fetch configurations, manage symlinks, and synchronize profiles with ease.

# OPTIONS

`-c, --config` FILE
: Specify a custom path to the dotbee configuration file.

`--dry-run`
: Do not perform any actions; only print the operations that would be executed.

`--help`
: Print help information.

`--version`
: Print the version number of dotbee.

# COMMANDS

completion, c
: Generate shell completions for the specified shell.

    Supported shells: bash, zsh, fish, elvish.

doctor, dr
: Perform a system check to show currently used configs and the status of active symlinks.

init, i
: Initialize dotbee in the current environment.

list, ls
: List all available configurations currently managed by dotbee.

purge, p
: Remove and purge existing symlinks managed by dotbee.

sync, s [OPTIONS]
: Synchronize configurations.

    --profile PROFILE
        Specify the target profile to sync.

fetch, f URL
: Download a dotfiles repository from the specified Git URL.

edit, e
: Open the dotbee.toml configuration file in the default system editor.

# EXAMPLES

Initialize dotbee for the first time:

    dotbee init

Fetch a remote dotfiles repository:

    dotbee fetch https://github.com/rogue-sly/dotfiles.git

Sync a specific profile without making actual changes:

    dotbee --dry-run sync --profile work

Generate completions for Zsh:

    dotbee completion zsh > ~/.zsh/_dotbee

# FILES

~/.config/dotbee/config.toml

    Configuration file for dotbee

~/.local/state/dotbee/dotbee.lock

    Prevent multiple concurrent processes

~/.local/state/dotbee/state.json

    Ensure symlink integrity

~/.local/share/dotbee/

    Location where dotfiles repository files are stored

# ENVIRONMENT VARIABLES

**VISUAL**, **EDITOR**
The editor used by the `edit` command.

# BUGS

Report bugs at https://github.com/rogue-sly/dotbee/issues

# SEE ALSO

git(1), ln(1)

# Dotsy

<!--toc:start-->

- [Dotsy](#dotsy)
  - [Motivation](#motivation)
  - [Key Concepts](#key-concepts)
    - [Hosts](#hosts)
    - [State File](#state-file)
  - [How It Works](#how-it-works)
  - [Commands](#commands)
    - [dotsy init <name>](#dotsy-init-name)
    - [dotsy list](#dotsy-list)
    - [dotsy switch <host>](#dotsy-switch-host)
    - [dotsy status](#dotsy-status)
    - [dotsy purge](#dotsy-purge)
    - [dotsy repair](#dotsy-repair)

<!--toc:end-->

Dotsy is an opinionated, file-based dotfiles manager designed to keep your configuration organized, and easy to manage across multiple machines and environments.

## Motivation

Managing dotfiles across different machines, operating systems, and user roles can quickly become complicated. Many existing tools are either too generic or too complex.

Dotsy aims to simplify dotfile management by:

- Using an **explicit, file-based structure** without the need for a separate config file.
- Providing **opinionated conventions** that suit common workflows (hosts).
- Keeping track of which configurations are currently active via a small state file.

---

## Key Concepts

### Hosts

- Represent **device- or environment-specific configurations**.
- Examples: `global` (shared configs between hosts like gitconfig), `foo`, `bar`, `baz`.
- Stored in `hosts/` directory.

### State File

- Located at `~/.local/state/dotsy.json` (following XDG Base Directory spec).
- Tracks the **currently active host**.
- Example content:
  ```json
  {
    "active-host": "foo"
  }
  ```

## How It Works

You organize your dotfiles like this:

```sh
.
├── global
│   ├── .config
│   │   └── something.txt
│   └── note.txt
└── hosts
    ├── bar
    │   └── .config
    │       ├── nushell
    │       │   └── config.nu
    │       ├── rio
    │       │   └── config.toml
    │       └── zed
    │           ├── keymap.json
    │           ├── settings.json
    │           └── themes
    ├── baz
    │   ├── .config
    │   │   ├── inner
    │   │   │   └── something.txt
    │   │   ├── lol.txt
    │   │   └── nushell
    │   │       └── config.nu
    │   └── .haha.txt
    └── foo
        ├── .config
        │   ├── kitty
        │   │   ├── current-theme.conf
        │   │   ├── kitty.conf
        │   │   ├── settings
        │   │   └── themes
        │   ├── mako
        │   │   └── config
        │   ├── neovide
        │   │   └── config.toml
        │   ├── niri
        │   │   └── config.kdl
        │   ├── nvim
        │   │   └── init.lua
        │   ├── waybar
        │   │   ├── config.jsonc
        │   │   └── style.css
        │   └── wofi
        │       ├── config
        │       ├── select.css
        │       └── style.css
        └── .zshrc

```

When you run:

`dotsy switch foo`

Dotsy will:

- Remove existing active symlinks from your home directory (if any).
- Create symlinks for everything inside `global/` and `hosts/foo` into your home directory.
- Update the state file to mark foo as the active host.

## Commands

### dotsy init <name>

Initializes a new dotfiles structure in the current directory. This creates a `hosts/` folder and a `global/` host for shared configs.

### dotsy list

Lists all available hosts in the hosts/ folder.

### dotsy switch <host>

Activates the selected host. Removes the current symlinks and sets up new ones for the chosen host and global.

### dotsy status

Shows the status of the selected host(Checks whether symlinks are broken or not or configs not symlinked at all).

### dotsy purge

Removes all currently active symlinks and clears the state file. Effectively deactivates the current host.

### dotsy repair

Attempts to repair missing or broken symlinks for the currently active host.

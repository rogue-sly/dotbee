# Dotbee

![Github License](https://img.shields.io/github/license/rogue-sly/dotbee)
![Github Version](https://img.shields.io/github/v/release/rogue-sly/dotbee)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/rogue-sly/dotbee/total?logo=github&logoColor=black)
![Crates.io Total Downloads](https://img.shields.io/crates/d/dotbee?logo=rust&logoColor=orange)

**Dotbee** is a simple, symlink-based dotfiles manager. It focuses on doing one thing well: managing your configuration files without the complexity of shell scripts or bloated feature sets.

## Features

- **Profiles:** multiple setups for multiple machines.
- **Reliable:** dotbee can fix broken symlinks, handle file conflicts, validate config and more!
- **Safe:** dotbee won't delete any files/directories without telling you first!
- **Simple:** easy to use cli and simple configuration.

## Installation

### Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```sh
cargo binstall dotbee
```

### Using [Mise](https://mise.jdx.dev/)

```sh
mise use github:rogue-sly/dotbee
```

### From Source

1. From [crates.io](https://crates.io/crates/dotbee):

```sh
cargo install dotbee
```

2. Directly From Repository:

```sh
# if you're gonna do it that way, I recommend picking a tag like v0.8.0 instead of main
cargo install --git https://github.com/rogue-sly/dotbee
```

> [!NOTE]
> There are also `.deb` and `.rpm` packages in the releases page.
> You can use `apt` or `dnf` to install them respectively.

## Development & Testing

To avoid accidental data loss on your host system during development, use the provided `mise` tasks to run Dotbee in a container:

```sh
mise run try-dotbee --profile dev
```

## Acknowledgments

- [Dotsy](https://github.com/NICHTJ3/Dotsy): My project was initially named dotsy until I discovered there's another project that does the same stuff as mine on crates.io lol xD.

- [Stow](https://www.gnu.org/software/stow/): While I kinda hated how stow works, I have to admit it's very plain and simple dotfiles management tool which is something I really appreciate. I don't like that fact that I have to organize my files in a specific way and run some scripts to make things work. It was also kind difficult to undo things with stow or fix up any broken symlinks.

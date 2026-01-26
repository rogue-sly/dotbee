# Dotsy

Easy to use dotfiles manager.

> [!WARNING]
> Dotsy is still in early development
> I highly recommend trying it in a docker or podman container for now
> You can use `mise tasks` to easily setup a container and run it

## Motivation

I've tried soo many dotfiles managers, and most of the time they require shell scripts to be used effectively (I'm looking at you stow), do too many things (ahem ahem chezmoi), or just too complicated. All I want is just a simple, symlink-based dotfiles manager that is easy to use, configure and does one thing well which is managing dotfiles and nothing else :D

Initially, I wanted to make it so that the file system hierarchy acts as a way to configure dotsy but I realized it's too hard to implement (skill issues ig), and has some edge cases. I decided to scrap all that junk and just use TOML for configuration :D

This is still in development, so expect some bugs.

## How to Use

### Prerequisites

- **Rust:** v1.92.0 (managed via `mise` or `rustup`).
- **Mise:** Recommended for environment and task management. (simply run `mise use` to install rust version specific to this project)

### Running in a container (using [mise](mi) ) (Recommended)

To avoid accidental data loss on your host system during development or testing, it's highly recommended to run Dotsy in a container using `mise`.

1. **Build the container image:**
   ```bash
   mise run build-container
   ```
2. **Run Dotsy inside the container:**
   ```bash
   mise run run-container
   dotsy init
   dotsy --help
   dotsy switch something
   ```

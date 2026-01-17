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

If you don't wanna end up erasing precious files like I did, run dotsy in a container using docker, podman or whatever.

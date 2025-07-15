# Dotsy

Dotsy is an opinionated, file-based dotfiles manager designed to keep your configuration organized, reproducible, and easy to manage across multiple machines and environments. It focuses on simplicity and clarity by leveraging a structured folder layout and minimal configuration files, with a small state file tracking active profiles and groups.

---

## Table of Contents

- [Motivation](#motivation)
- [Key Concepts](#key-concepts)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
- [State Management](#state-management)
- [Commands](#commands)
- [Why Opinionated?](#why-opinionated)
- [Technology Choice](#technology-choice)
- [Future Enhancements](#future-enhancements)
- [Contributing](#contributing)
- [License](#license)

---

## Motivation

Managing dotfiles across different machines, operating systems, and user roles can quickly become complicated. Many existing tools are either too generic or too complex.

Dotsy aims to simplify dotfile management by:

- Using an **explicit, file-based structure** without the need for a separate config file.
- Providing **opinionated conventions** that suit common workflows (profiles and groups).
- Keeping track of which configurations are currently active via a small state file.
- Supporting Linux, macOS, and Termux environments out of the box.
- Offering a CLI interface that is easy to use and scriptable.
- Being easy to build and distribute as a self-contained binary.

---

## Key Concepts

### Profiles

- Represent **device- or environment-specific configurations**.
- Examples: `global` (common configs), `laptop`, `termux`, `macbook`.
- Stored in `profiles/` directory.
- Each profile contains multiple config folders, e.g., `fish/`, `git/`, `nvim/`.

### Groups

- Represent **collections of related configurations** that are often used together.
- Examples: `niri` (includes Waybar, Mako, swayidle.service, etc.).
- Stored in `groups/` directory.
- Useful for bundling configs for specific workflows or window managers.

### State File

- Located at `~/.local/state/dotsy.json` (following XDG Base Directory spec).
- Tracks the **currently active profile** and **active groups**.
- Example content:
  ```json
  {
    "activeProfile": "laptop",
    "activeGroups": ["niri", "devtools"]
  }
  ```

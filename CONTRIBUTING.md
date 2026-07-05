# Contributing to Dotbee

Thank you for considering spending some of your time on this project! :D

## Getting Started

### Prerequisites

To work on Dotbee, you will need the following tools installed on your system:

- **Rust Toolchain**: Install using [rustup](https://rustup.rs/)
- **Mise**: Use [mise](https://mise.jdx.dev/) for managing development tasks and tools.
- **Docker or Podman**: Highly recommended for running and testing Dotbee safely without affecting your host system's files.

### Setting up the Environment

1. **Clone the repository:**

   ```bash
   git clone https://github.com/rogue-sly/dotbee.git
   cd dotbee
   ```

1. **Install dependencies:**
   If you are using `mise`, it might automatically detect and suggest installing the required tools defined in `mise.toml`.

## Development Workflow

### Building the Project

You can build the project using standard Cargo commands:

```bash
cargo build
```

To run the binary directly (use with caution on your local machine):

```bash
cargo run -- <command>
# Example: cargo run -- list
```

### Safe Development (Recommended)

> [!WARNING]
> Since Dotbee creates/removes symlinks, files or whatever, **I strongly recommend running it inside a container** during development to avoid accidentally modifying your personal dotfiles.

I've provided `mise` tasks to simplify this process:

1. **Run Dotbee inside the container:**
   This command compiles your current code, mounts the binary into the container, and drops you into a shell where you can safely run `dotbee`.

   ```bash
   mise run try-dotbee
   ```

   To test with a release build:

   ```bash
   mise run run-container --release
   ```

## Code Quality

### Formatting & Linting

Please, adhere to standard Rust coding conventions.

- **Formatting:** Ensure your code is formatted using `rustfmt`.

  ```bash
  cargo fmt
  ```

- **Linting:** Use `clippy` to catch common mistakes.

  ```bash
  cargo clippy
  ```

## Submitting Changes

### Commit Message Guidelines

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages. This helps in generating change-logs and managing versions.

Please use the following prefixes in your commit messages:

- `feat:`: A new feature for the user.
- `fix:`: A bugfix for the user.
- `docs:`: Documentation only changes.
- `style:`: Changes that do not affect the meaning of the code (white-space, formatting, etc).
- `refactor:`: A code change that neither fixes a bug nor adds a feature.
- `perf:`: A code change that improves performance.
- `test:`: Adding missing tests or correcting existing tests.
- `chore:`: Changes to the build process or auxiliary tools and libraries.

Example:

```bash
git commit -m "feat: add support for custom icon sets"
```

### Pull Request Process

1. **Fork the repository** on Github.

1. **Create a new branch** for your feature or bug fix.

   ```bash
   git checkout -b feature/amazing-feature
   ```

1. **Commit your changes** following the guidelines above.

1. **Push to your fork**:

   ```bash
   git push origin feature/amazing-feature
   ```

1. **Open a Merge Request** (MR) against the `main` branch of the `dotbee` repository.

## Note on Using LLM/AI Assistants

You can use LLM/AI for help, but please don't use it as a human substitute. For example, you can ask it questions about the codebase or perhaps questions about a specific library, but don't use it for generating code. [I will know if you do that.](https://tenor.com/view/tuco-salamanca-powder-gif-24543326)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

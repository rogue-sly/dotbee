# Contributing to Dotbee

First off, thank you for considering contributing to Dotbee! It's people like you that make the open-source community such an amazing place to learn, inspire, and create.

## Getting Started

### Prerequisites

To work on Dotbee, you will need the following tools installed on your system:

- **Rust**: Version 1.95.0 or later.
- **Mise**: Use [mise](https://mise.jdx.dev/) for managing development tasks and tools.
- **Docker or Podman**: Highly recommended for running and testing Dotbee safely without affecting your host system's files.
- **Cross**: [cross](https://github.com/cross-rs/cross) is needed in order to test dotbee in a container.

### Setting up the Environment

1. **Clone the repository:**

   ```bash
   git clone https://gitlab.com/rogue-sly/dotbee.git
   cd dotbee
   ```

2. **Install dependencies:**
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

- **Linting:** We use `clippy` to catch common mistakes.

  ```bash
  cargo clippy
  ```

### Testing

Please ensure that you add tests for any new features or bug fixes.

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

1. **Fork the repository** on GitLab.
2. **Create a new branch** for your feature or bug fix.

   ```bash
   git checkout -b feature/amazing-feature
   ```

3. **Commit your changes** following the guidelines above.
4. **Push to your fork**:

   ```bash
   git push origin feature/amazing-feature
   ```

5. **Open a Merge Request** (MR) against the `main` branch of the `dotbee` repository.

## Note on Using LLM/AI Assistants

After using `gemini-cli` and ending up blindly shooting myself in the foot, I decided to limit the capabilities of AI and let it only give suggestions and hints and only spit out code when I tell it to do so.

While the LLM being able to quickly edit files is convenient, it's also kind of a double edged sword as it will make you lazier.

I also suggest not copying and pasting but rather writing it line by line, word by word. This should help you discover errors and mistakes before they even happen.

~~Personally, I find it very useful for things like writing small scripts like the ones I have inside of `.mise-tasks` and for doing annoying tasks like writing change-logs.~~

Yeah... I don't use that anymore lol

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

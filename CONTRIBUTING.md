# Contributing to Dotsy

First off, thank you for considering contributing to Dotsy! It's people like you that make the open-source community such an amazing place to learn, inspire, and create.

## Getting Started

### Prerequisites

To work on Dotsy, you will need the following tools installed on your system:

- **Rust**: Version 1.92.0 or later.
- **Mise**: Use [mise](https://mise.jdx.dev/) for managing development tasks and tools.
- **Docker or Podman**: Highly recommended for running and testing Dotsy safely without affecting your host system's dotfiles.

### Setting up the Environment

1. **Clone the repository:**
   ```bash
   git clone https://gitlab.com/rogue87/dotsy.git
   cd dotsy
   ```

2. **Install dependencies:**
   If you are using `mise`, it might automatically detect and suggest installing the required tools defined in `mise.toml` (like `wrkflw`).

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

### ⚠️ Safe Development (Recommended)

Since Dotsy creates and removes symlinks, **I strongly recommend running it inside a container** during development to avoid accidentally modifying your personal dotfiles.

I've provided `mise` tasks to simplify this process:

1. **Build the development container:**
   ```bash
   mise run build-container
   ```

2. **Run Dotsy inside the container:**
   This command compiles your current code, mounts the binary into the container, and drops you into a shell where you can safely run `dotsy`.

   ```bash
   mise run run-container
   ```

   To test with a release build:
   ```bash
   mise run run-container --release
   ```

## Code Quality

### Formatting & Linting

We adhere to standard Rust coding conventions.

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

To run the GitLab CI pipeline locally (requires `wrkflw`):

```bash
mise run test-gitlab-ci
```

## Submitting Changes

### Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for our commit messages. This helps in generating changelogs and managing versions.

Please use the following prefixes in your commit messages:

-   `feat:`: A new feature for the user.
-   `fix:`: A bugfix for the user.
-   `docs:`: Documentation only changes.
-   `style:`: Changes that do not affect the meaning of the code (white-space, formatting, etc).
-   `refactor:`: A code change that neither fixes a bug nor adds a feature.
-   `perf:`: A code change that improves performance.
-   `test:`: Adding missing tests or correcting existing tests.
-   `chore:`: Changes to the build process or auxiliary tools and libraries.

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
5. **Open a Merge Request** (MR) against the `main` branch of the `dotsy` repository.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

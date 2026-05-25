## [unreleased]

### ⚙️ Miscellaneous Tasks

- Update roadmap
- Add metadata for cargo-binstall
- Remove deprecated authors field
## [0.6.1] - 2026-05-15

### 🐛 Bug Fixes

- Ensure duplicate check only occurs between the global profile and any other profile

### ⚙️ Miscellaneous Tasks

- Bump version
## [0.6.0] - 2026-05-15

### 🚀 Features

- Add a config validation step

### 🐛 Bug Fixes

- Put archive filetype

### ⚙️ Miscellaneous Tasks

- Specify asset type
- Use podman for try-dotbee script
- Bump dotbee
## [0.5.0] - 2026-05-13

### 🚀 Features

- [**breaking**] Remove message module from context and generalize it to just a logging utility

### 💼 Other

- Remove trash crate and simply delete files for the sake of consistency

### 🚜 Refactor

- Move tests to a dedicated tests folder + add more unit tests
- Rename managed_links -> links
- Rename functions that has the word "managed" into ones that doesn't
- Remove some unnecessary clones
- Format messages nicely
- Use anyhow crate for error handling
- Remove unused code
- Use ConflictKind enum instead of a String
- Use sane defaults for settigns struct
- Format and explain ask

### 📚 Documentation

- Update readme installation instructions
- Add a link that points to the roadmap milestone
- Update readme
- Yet another update to the readme
- Add some badges and slight changes to readme
- Bring back roadmap file
- Update AGENTS.md due to project changes
- Redefine project scope
- Add acknowledgements to README.md
- Update CONTRIBUTING.md
- Update ROADMAP.md
- Add a prerequisite
- Add some doc comments
- Revise plan
- Get rid of more tasks because I'm lazy :D

### ⚙️ Miscellaneous Tasks

- Remove some files from termux example config
- *(Cargo.toml)* Exclude some files
- Bump dotsy version to 0.4.0
- Remove some unused baggage
- Add checks prior to build step
- Ensure lint and test only run on commit tag
- Remove example folder and simplify try-dotsy & Dockerfile
- Use lefthook to setup git hooks and remove format and lint mise tasks
- Remove rust-toolchain.toml related stuff in favour of cross
- Merge all includes into .gitlab-ci.yml
- Delete mise.toml and update lefthook config
- Remove AGENTS.md
- Update links
- Delete Cross.toml
- Bump rust version
- Manage cargo packages
- Delete unit tests for now
- Remove tempfile dependency
- Comment out pre-push git hook
- Add a mise.toml file for project tools
- Use git-cliff for generating changelog
- Update author name
- Remove thiserror crate
- Use tombi
- Simplify gitlab-ci script
- Setup cargo-deb and cargo-generate-rpm
- Add packaging step
- Fix potential issues in the gitlab-ci script
- Make it possible to run the ci manually
- Fuck my life
- Finalize CI script
- Update lefthook
- Format lefthook.yml
- Update lefthook config once again
- Bump dotbee to v0.5.0
- Add description to release stage
## [0.4.0] - 2026-02-24

### 🐛 Bug Fixes

- Add trailing forward slashes

### 💼 Other

- Add cargo to PATH in build.yml

### 📚 Documentation

- Rewrite AGENTS.md with accurate commands, architecture, and code style
- Update ROADMAP.md
- Add a link that points to the wiki
- Update roadmap
- Update changelogs

### ⚙️ Miscellaneous Tasks

- Modularize gitlab-ci script and add cross
- Rename dotsy to dotbee
- Fix gitlab ci script
- Disable lint and test ci jobs
- Comment out import for validation
- Add Cross.toml and enable lint and test ci
- Fix release-cli download and improve artifact handling
## [0.3.0] - 2026-02-22

### 🚀 Features

- Remove the profile inferring algorithm
- Make the state file track managed symlinks
- Make purge, repair and switch commands make use of the new state file managed links feature
- Implement Plan/Execution pattern for switch, repair, purge

### 🐛 Bug Fixes

- Forgor to update get_hostname in switch lol
- Ensure ghost links are removed when switching
- Ensure state is fully cleared on purge and load failures

### 💼 Other

- Exit process when no profile was found when running `dotsy switch`

### 🚜 Refactor

- Try to untangle some nested ifs plus some unit tests
- Use nix crate for getting hostname
- Move unit tests to test folder
- Break src/lib.rs into multiple workspaces
- Untangle the nested ifs in doctor.rs
- Change color for delete message to red
- List subcommand
- Chnage expect error message
- Refactor expand_path and rename it to expand_tilde
- Fix rust-analyzer warnings
- Make functions that take either message or dry_run take context instead
- Collapse some ifs and run cargo fmt
- Remove unused function
- Add some unit tests to utils.rs and rewrite some methods
- Just a rename
- Add some doc comments and rename a function
- Implement manager
- Remove Ask from ConflictAction enum
- Untangle nested ifs in repair.rs
- Small changes not worth having a commit message for :p
- Introduce ConfigManager, StateManager, SymlinkManager structs
- Move config path tracking from Config struct to ConfigManager

### 📚 Documentation

- Add new tasks to the roadmap
- Change how gemini-cli works and add a note on using LLMs in GEMINI.md
- Once again update contributing.md
- Update GEMINI.md file
- Update roadmap
- Add more tasks to the roadmap

### 🧪 Testing

- Add unit tests for ConfigManager and StateManager

### ⚙️ Miscellaneous Tasks

- Make a script that bumps up version number
- Update bump-verison script
- Add tempfile crate
- Delete tests
- Run `cargo fmt` and `taplo format`
- Switch from toml to json for the state file
- Minify neovim config example to just a single file
- Make files writable in the container
- Discard workspaces
- Rename GEMINI.md to AGENTS.md
- Reflect project changes in AGENTS.md
- Release v0.3.0
## [0.2.1] - 2026-02-07

### 🚜 Refactor

- Make `dotsy ls` show global profile

### 📚 Documentation

- Rename sw to s
- Change roadmap

### ⚙️ Miscellaneous Tasks

- Delete example folder
- Update Dockerfile to use fish instead of bash
## [0.2.0] - 2026-02-06

### 🚀 Features

- Provide aliases for commands
- Implement completion command

### 🚜 Refactor

- Use message modules for printing instead of using icons modules directly
- Separate app and lib code

### ⚙️ Miscellaneous Tasks

- Run cargo fmt
- Fix dotsy container script to handle multiple targets
- Add actual example config to use with the try-dotsy mise task
- Update readme on how to setup shell completions
- Update readme
- Remove gnu target and use musl as the default
- Bump dotsy version to 0.2.0
## [0.1.0] - 2026-01-31

### 🚀 Features

- Implement `dotsy init`
- Implement list command
- Implement switch command
- Create a state.rs file with some basic functions
- Add --config and --dry-run flags
- Add icon_style {TEXT, Emoji, NerdFont}
- Implement profile auto detection based on hostname

### 🐛 Bug Fixes

- Fix icon label

### 🚜 Refactor

- Move todo!'s into each command instead of main.rs
- Rethink the entire idea of approaching configs and groups
- Make list command show only hosts
- Rename Commands to Command
- Move std::fmt::Formatter up to `use` instead
- Update code to use demand instead of dialoguer
- Change error message
- Add descriptions for each conflict action
- Change config to dotfiles
- Change expect() to ?
- Change icons
- Rename example configs to something that makes more sense
- Update icons again
- Import std::error::Error instead of calling the full path to module
- Simplify code in switch.rs
- Get_destination_status function
- Rename status to doctor
- Rename profiles to configs(config_collection)
- Shorten some lines of code
- Get rid of PathBuf in ConflictingSymlink enum entry
- Reimplement the configuration load function
- Rename stuff to something more clear
- Put hooks and icons in their own separate module
- Make on_conflict use an enum instead of a string

### 📚 Documentation

- Change profiles to hosts
- Add `dotsy status command`
- Match with example files
- Match some other stuff within the readme to example files once again

### ⚙️ Miscellaneous Tasks

- Add `colored` crate
- Add a todo
- Add `dialoguer` crate
- Update example config
- Add more info to Cargo.toml
- Add trash crate
- Fix typo
- Add demand crate
- Remove dialoguer crate
- Add some crates
- Run `cargo fmt`
- Add indexmap crate

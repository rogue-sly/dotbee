# Changelog
## [unreleased]

### 🚀 Features

- Prevent multiple instances of dotbee from running

### 🐛 Bug Fixes

- Use data directory on other platforms for state file
- Eliminate possible symlink-following TOCTOU during conflict overwrite

### 🚜 Refactor

- Remove bad plan-execute patterns in favor of something simpler
- Add extra checks for init subcommand
- Shorten macro
- Simplify check function
- Propagate more errors during config and state load
- Flatten manager struct
- Inline manager structs
- Use bail! macro
- Use peek instead of faulty arithmetics
- Iron out all `unwrap()`s and `expect()`s

### 📚 Documentation

- Update roadmap

### 🧪 Testing

- Write some basic unit tests for config loading

### ⚙️ Miscellaneous Tasks

- Update cliff and lefthook config
- Delete gen-changelog task
- Scratch macOS
- Git rid of cross
- Add .nvim.lua script
- Remove the damn clippy checks for rust-analyzer and githook
- Add fs nix create feature
- Add tmux package to Dockerfile
- Format main.rs
- Update plan
- Update rust-analyzer nvim lsp settings
- Bump rust version
- Add moar tools
## [0.6.3] - 2026-05-25

### 🐛 Bug Fixes

- Set bin-dir properly

### 💼 Other

- Apparently crates.io doesn't like indents

### 🚜 Refactor

- Why do I always forget to remove this shit

### ⚙️ Miscellaneous Tasks

- Bump version
## [0.6.2] - 2026-05-25

### 🐛 Bug Fixes

- Use inline tables instead of multiline ones
- Strip ensure to strip ./ prefix for all sources

### 📚 Documentation

- Update install instructions

### ⚙️ Miscellaneous Tasks

- Update roadmap
- Add metadata for cargo-binstall
- Remove deprecated authors field
- Add overrides to binstall
- Update bump-version script
- Bump dotbee version
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

- I find this cringe. why did I write that?
- Remove trash crate and simply delete files for the sake of consistency
- Revert "ci: make it possible to run the ci manually"

This reverts commit bf90b407784b77e89f59eef9a00a66fb9efef928.

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
- Yes at this point, I'm praying that it'll work

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
- Rename test to tests
- Add format and lint mise tasks
- Check point

check point (purge.rs)
- Merge branch 'refactor/destroy-nests' into 'main'

Well this counts more as a project restructure

See merge request rogue87/dotsy!5
- Make state save the path to dotfiles folder
- Merge branch 'refactor/manager' into 'main'

refactor: implement manager

See merge request rogue87/dotsy!6

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

### 💼 Other

- Revert "chore: delete example folder"

This reverts commit 11f580b8c2d1404505dfb8773ffcf437c37443d7.

I forgot I need this for try-dotsy mise task lol
- Bump dotsy version

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

### 💼 Other

- Update readme
- Update readme again lol
- Add CONTRIBUTING.md

and after that, I should really stop pushing to main and making branches
myself (why am I soo lazy sometimes?)
- Who's we? -w-
- Update mise settings
- Create context module
- Globalize some flags
- Use context in main function
- Merge branch 'refactor/context-pattern' into 'main'

Use context pattern instead of loading things over and over again

See merge request rogue87/dotsy!1
- Merge branch 'refactor/message' into 'main'

refactor: use message modules for printing instead of using icons modules directly

See merge request rogue87/dotsy!2
- Merge branch 'refactor/code-modularization' into 'main'

Refactor/code modularization

See merge request rogue87/dotsy!3
- Merge branch 'feat/shell-completion' into 'main'

Feat/shell completion

See merge request rogue87/dotsy!4
- Update roadmap

### 🚜 Refactor

- Refactor state to support context module
- Refactor subcommands to support context
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

- Fix panic on file conflict
- Fix icon label

### 💼 Other

- Init
- Update readme and add a todo.md file
- Create an example config
- Add some deps
- Basic project setup
- Add testing/ to gitignore
- Update readme and add toml crate
- Yetus deletus example folder
- Create rustfmt.toml file
- Remove state logic and get rid of serde_json crate
- Clear up things and rename certain stuff
- Rename host to config
- Update todos
- Add a todo
- Create an initial config and schema files for dotsy
- Make a diagram
- Update repo link and author name
- Finalize config template and make a config loader
- Throw in a gitkeep file there
- Lazily implement init command :p
- Make a dockerfile so I won't be playing russian roulette whenever testing dotsy
- Implement basic functionality for switch command
- Update readme
- Revert "refactor: rename profiles to configs(config_collection)"

This reverts commit fa096aa0dd3f203e71b920eb4556e21c981bd888.
- Change cargo build from release to debug
- Update Cargo.lock
- Use unwraps here and there
- Bump rust MSRV to '1.92.0'
- Improve docker image building and add some mise tasks
- Add bat package to the fedora docker image
- Sorta implement list command (needs work)
- Create a util module
- Move expand_path function from switch to util.rs
- Make list command print profiles in a noice format
- Update readme
- Forgor to add task after mise lol
- Move is_profile_active function from list.rs to util.rs
- Properly handle switching between profiles
- Move mise tasks from mise.toml to mise-tasks folder and add rust@1.92.0 to mise.toml
- Add rust@1.92.0 to mise.toml and fix mise tasks scripts
- Move some code from switch.rs to util.rs
- Implement doctor subcommand
- Implement purge subcommand
- Add a config path check in list.rs
- Move symlink_with_parents function from switch.rs to util.rs and remove unused imports
- Implement repair command
- Remove unused imports and get_config function
- Why am I having such a hard time removing unused imports? lol
- Update readme
- Delete excalidraw file cuz I don't need it
- Update todos
- Change dotsy config
- Make config flag local
- Remove unused dependencies
- Yet another refactor
- Replace hashmap with indexmap for preserving order
- Update readme
- Update todos
- Move tasks back to mise.toml
- Make it possible to build dotsy in release mode for container
- Use state for figuring out current active

- added a state module
- encapsulated the old logic for figuring out the current active profile
in util.rs and using as a fallback in case state file is deleted or
corrupted for whatever reason`
- Run `cargo fmt`
- Add GEMINI.md
- Remove deadcode attribute macro
- Rename util.rs to utils.rs
- Rename resolve_active_profile to find active profile
- Format and remove comment
- Make list subcommand use fallback logic
- Update icons struct
- Use an enum for icons
- Run `cargo fmt`
- Setup gitlab-ci yaml script
- Update gitlab ci yaml script
- Update schema for dotsy
- Make mise respect rust-toolchain.toml
- Handle trash dependency on android target
- Update docker related stuff
- Update readme
- Merge TODO.md into ROADMAP.md
- Add changelog
- Update GEMINI.md
- Update dotsy.json schema

now taplo should provide completion for this
- Scratch hooks entirely

I don't see the point of having something like this in a dotfiles
manager
- Update roadmap
- Run `cargo fmt`
- Add musl target for both aarch64 & x86_64

- rust-toolchain
- .gitlab-ci.yaml

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

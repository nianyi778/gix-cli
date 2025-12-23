# gix-cli Copilot Instructions

## Project Overview
**gix** is a Git extension CLI tool with two implementations:
- **Rust version** ([rust/](../rust/)) — Modern, performant native binary (recommended)
- **TypeScript version** ([typescript/](../typescript/)) — Original Node.js implementation

Both provide identical functionality. This guide focuses on the **Rust version**.

## Architecture
- **Entry Point**: [rust/src/main.rs](../rust/src/main.rs) — Defines CLI structure with Clap and dispatches to command modules
- **Commands**: [rust/src/commands/](../rust/src/commands/) — Each module exports an `Args` struct (if needed) and `execute()` function
- **Build**: Cargo builds optimized binary to `rust/target/release/gix` (see [rust/Cargo.toml](../rust/Cargo.toml))

## Command Pattern
Every command follows this structure (see [rust/src/commands/merge.rs](../rust/src/commands/merge.rs)):
1. Define `Args` struct with Clap derive macros for CLI options
2. Implement `execute(args: Args) -> Result<()>` function
3. Validate working directory is clean: `is_working_directory_clean()?`
4. Use `inquire::Text/Select/Confirm` for missing CLI inputs
5. Execute git via `exec_git()` or `exec_git_interactive()` helpers
6. Provide clear emoji-prefixed messages: ✅ success, ❌ error, ⚠️ warning, 🔧 execution

## Key Conventions
- **Git Safety**: Always check working directory is clean before destructive operations
- **Upstream Handling**: Auto-detect upstream with `has_upstream()`. If missing, use `git push --set-upstream` instead of `--force-with-lease` (see [rust/src/commands/merge.rs#L116-L129](../rust/src/commands/merge.rs#L116-L129))
- **Error Handling**: Return `Result<()>` from all command functions. Use `?` operator to propagate errors
- **Interactive I/O**: Use `exec_git_interactive()` for commands requiring user input (commit editor, rebase, etc.)
- **Force Push**: Use `--force-with-lease` for safety, with user confirmation prompt

## Development Workflow
- **Local Dev**: `cd rust && cargo run -- <command>` — Run directly with Cargo
- **Build**: `cd rust && cargo build --release` — Creates optimized binary in `rust/target/release/gix`
- **Testing**: No formal tests. Test interactively with `cargo run -- <command>`
- **Format**: `cargo fmt` — Auto-format code before committing
- **Lint**: `cargo clippy` — Run linter for code quality checks

## Git Command Patterns
- **Merge Commits**: `git reset --soft <from>^ && git commit` — Reset to parent of start commit, then new commit (see [rust/src/commands/merge.rs#L96-L103](../rust/src/commands/merge.rs#L96-L103))
- **Root Commit Check**: Prevent merge from root with `git rev-list --max-parents=0 HEAD`
- **Clean Check**: `git status --porcelain` must return empty string
- **Current Branch**: `git symbolic-ref --short HEAD`
- **Upstream Check**: `git rev-parse --abbrev-ref <branch>@{u}` (suppress errors if no upstream)

## Error Handling
- All commands return `Result<()>` where `Err(String)` is a user-facing error message
- Validate inputs before execution (working directory clean, valid commit hashes, etc.)
- Return early on validation failures — do NOT proceed with destructive operations
- Use helper functions from [rust/src/commands/mod.rs](../rust/src/commands/mod.rs) for common operations

## Adding New Commands
1. Create file in [rust/src/commands/](../rust/src/commands/) (e.g., `mycommand.rs`)
2. Define `Args` struct with `#[derive(Args)]` and Clap field attributes
3. Implement `execute(args: Args) -> Result<()>` function
4. Export in [rust/src/commands/mod.rs](../rust/src/commands/mod.rs): `pub mod mycommand;`
5. Add enum variant to `Commands` in [rust/src/main.rs](../rust/src/main.rs)
6. Add match arm in `main()` to dispatch to your command

## External Dependencies
- **clap**: CLI framework with derive macros (`#[derive(Parser)]`, `#[arg(...)]`)
- **inquire**: Interactive prompts (`Text`, `Select`, `Confirm`)
- **colored**: Terminal colors (`.red()`, `.green()`, `.yellow()`, `.cyan()`)
- **std::process::Command**: Execute git commands with `.status()` or `.output()`

## Migration Notes (TypeScript → Rust)
- Replaced `commander` with `clap` (derive-based API)
- Replaced `inquirer` with `inquire` (similar API, more type-safe)
- Replaced `execSync()` with `std::process::Command` (explicit I/O handling)
- No build step for development — use `cargo run` directly
- Binary size optimized with `opt-level = "z"`, LTO, and strip in release profile

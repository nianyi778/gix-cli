# gix — Rust Implementation

A fast, standalone Git extension CLI tool written in Rust.

## 🚀 Features

- Zero runtime dependencies (compiled binary)
- Fast startup and execution
- Cross-platform support (Linux, macOS, Windows)
- Identical functionality to TypeScript version

## 📦 Installation

### From crates.io

```bash
cargo install gix-cli
```

### Build from Source

```bash
cargo build --release
# Binary will be at target/release/gix
```

### Install Locally

```bash
cargo install --path .
```

## 🛠 Development

### Run locally
```bash
cargo run -- <command>
```

### Build optimized binary
```bash
cargo build --release
```

### Format code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

## 📖 Usage

See the [main README](../README.md) for detailed usage examples.

## 🏗 Architecture

- **Entry**: [src/main.rs](./src/main.rs) — CLI parser with Clap
- **Commands**: [src/commands/](./src/commands/) — Each command module
- **Dependencies**: Clap (CLI), Inquire (prompts), Colored (output)

See [.github/copilot-instructions.md](../.github/copilot-instructions.md) for development guidelines.

# gix — TypeScript Implementation

The original Node.js implementation of gix CLI tool.

## 🚀 Features

- npm/pnpm package management
- Quick installation via package manager
- Familiar JavaScript/TypeScript ecosystem

## 📦 Installation

### Global Installation

```bash
npm install -g gix-cli
# or
pnpm add -g gix-cli
```

### From Source

```bash
pnpm install
pnpm build
```

## 🛠 Development

### Run locally
```bash
pnpm dev <command>
```

### Build
```bash
pnpm build
```

### Watch mode
```bash
pnpm dev
```

## 📖 Usage

See the [main README](../README.md) for detailed usage examples.

## 🏗 Architecture

- **Entry**: [src/bin/gix.ts](./src/bin/gix.ts) — CLI setup with Commander
- **Commands**: [src/commands/](./src/commands/) — Command implementations
- **Build**: tsup bundles to ESM in [dist/](./dist/)
- **Dependencies**: Commander (CLI), Inquirer (prompts)

## 🔄 Migration Note

This is the original implementation. For better performance and no runtime dependencies, consider using the [Rust version](../rust/).

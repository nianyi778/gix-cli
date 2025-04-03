# 🧰 gix

[🇨🇳 中文文档](./README.zh.md) | [🇺🇸 English](./README.en.md)

**gix** is a Git extension CLI tool designed to simplify common Git workflows like commit merging and force pushing. Built with TypeScript, it is extensible, interactive, and ready for automation.

---

## 🚀 Features

- 🔧 Interactively merge multiple Git commits
- 💬 Customize commit message
- 🚦 Optional force-push confirmation (enabled by default)
- 🧱 Fully typed TypeScript support
- 🔌 Easy to extend with commands like `undo`, `squash`, etc.

---

## 📦 Installation

### ✅ Option 1: Install via npm (Recommended for most users)

```bash
npm install -g gix
```

Then simply run:

```bash
gix merge
```

Use this for quickly merging commits in daily development workflows.

---

### 🛠 Option 2: Clone the repo and develop locally (For contributors/devs)

```bash
git clone https://github.com/nianyi778/gix-cli.git
cd gix
pnpm install
pnpm build
pnpm link
```

Then you can use the CLI globally:

```bash
gix merge
```

#### If you encounter a `pnpm link` error (ERR_PNPM_NO_GLOBAL_BIN_DIR):

```bash
pnpm setup
source ~/.zshrc  # Or restart your terminal
```

---

## 🧪 Usage Example

```bash
gix merge
```

You will be prompted to enter:

- ✅ Start commit hash (required)
- ✅ End commit hash (optional, default is HEAD)
- ✅ New commit message
- ✅ Whether to force push (default is yes)

### Direct command-line usage:

```bash
gix merge -f a1b2c3d -m "feat: optimize printer logic"
```

---

## 📄 Available Commands

- `gix merge`: Merge multiple commits and optionally force-push

### Coming soon

- `gix squash`: Interactively squash recent commits
- `gix undo`: Revert the last merge/reset
- `gix log`: Print formatted commit log history

---

## 👨‍💻 Development

```bash
pnpm dev       # Dev mode (ts-node + esm loader)
pnpm build     # Build output using tsup
```

---

## 📦 Publish

1. Bump version: `npm version patch | minor | major`
2. Build output: `pnpm build`
3. Login to npm: `npm login`
4. Publish: `npm publish`

---

## ✨ Badges

![npm](https://img.shields.io/npm/v/gix?style=flat-square)
![license](https://img.shields.io/npm/l/gix?style=flat-square)

---

## 📄 License

MIT © 2025 [License](./LICENSE)

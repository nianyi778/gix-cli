# 🧰 gix

[🇨🇳 中文文档](./README.zh.md) | [🇺🇸 English](./README.en.md)

**gix** is a Git extension CLI tool designed to simplify your Git workflows — especially for merging commits and force pushing, with safety and clarity.

---

## 🚀 Features

- 🔧 Interactively merge multiple Git commits
- 💬 Custom commit message input
- 🚦 Confirm force push automatically
- 🧱 Fully typed, powered by TypeScript
- 🔌 Modular command system — more to come

---

## 📦 Installation

```bash
npm install -g gix-cli
```

Then use it globally:

```bash
gix merge
```

---

## 🛠 Usage Examples

### Merge commits

```bash
gix merge
```

Follow the prompts to:
- Select start and end commits
- Write a new commit message
- Optionally force-push

Or directly:

```bash
gix merge -f <from> -m "your message"
```

### Squash commits

```bash
gix squash -n 3      # squash last 3 commits
```

```bash
gix squash --all     # squash all history from root
```

### Check your Git state

```bash
gix doctor
```

- Check git & node version
- Git repo state
- Working directory clean?
- Remote & branch info

---

## 📄 License

MIT © 2025 [Nian Yi](./LICENSE)

# 🧰 gix

[🇨🇳 中文文档](./README.zh.md) | [🇺🇸 English](./README.md)

**gix** is a Git extension CLI tool designed to simplify your Git workflows — especially for merging commits and force pushing, with safety and clarity.

---

## 🚀 Features

- 🔧 Interactively merge multiple Git commits
- 💬 Custom commit message input
- 🚦 Confirm force push automatically
- ♻️ Reset local commits to remote (soft)
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

### Discard all local commits (soft reset to remote)

```bash
gix reset
```

- Soft reset to origin/branch
- Keeps all local file changes
- Removes unpushed commits only

---

## 📄 License

MIT © 2025 [Li Kai](./LICENSE)

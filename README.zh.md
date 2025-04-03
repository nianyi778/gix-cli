# 🧰 gix

[🇨🇳 中文文档](./README.zh.md) | [🇺🇸 English](./README.en.md)

**gix** 是一个 Git 扩展命令行工具，专为合并提交、强推等 Git 流程优化而设计，提供交互式体验和类型安全支持。

---

## 🚀 功能特性

- 🔧 交互式合并多个 Git 提交
- 💬 自定义提交信息
- 🚦 自动确认是否强推（默认开启）
- 🧱 使用 TypeScript 编写，类型安全
- 🔌 命令模块化设计，易于扩展

---

## 📦 安装方式

```bash
npm install -g gix-cli
```

安装后即可全局使用：

```bash
gix merge
```

---

## 🛠 使用示例

### 合并提交

```bash
gix merge
```

交互输入：
- 起始和结束 commit（可选）
- 新的 commit 信息
- 是否强推

或直接命令行执行：

```bash
gix merge -f <from> -m "你的提交信息"
```

### 压缩提交（Squash）

```bash
gix squash -n 3       # 合并最近 3 次提交
```

```bash
gix squash --all      # 从第一个提交开始合并（全量 squash）
```

### 检查当前 Git 状态

```bash
gix doctor
```

- 检查 Git 和 Node 版本
- 当前是否为 Git 仓库
- 工作区是否干净
- 是否配置远程仓库
- 当前所在分支

---

## 📄 License

MIT © 2025 [Nian Yi](./LICENSE)
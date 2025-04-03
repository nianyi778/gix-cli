# 🧰 gix

[🇨🇳 中文文档](./README.zh.md) | [🇺🇸 English](./README.en.md)

**gix** 是一个 Git 扩展命令行工具，旨在简化 Git 常见操作，如合并多个提交、自动强推等。基于 TypeScript 构建，具有良好的可扩展性和交互体验。

---

## 🚀 功能特性

- 🔧 交互式合并多个 Git 提交
- 💬 自定义提交信息
- 🚦 自动强推前进行确认提示（默认开启）
- 🧱 使用 TypeScript 编写，类型安全
- 🔌 易于扩展更多命令（如 `undo`、`squash` 等）

---

## 📦 安装方式

### ✅ 方式一：通过 npm 全局安装（推荐普通用户使用）

```bash
npm install -g gix
```

安装后，你可以直接运行：

```bash
gix merge
```

适合日常开发中快速合并 commit 的使用场景。

---

### 🛠 方式二：克隆源码并本地开发（适合贡献者/开发者）

```bash
git clone https://github.com/nianyi778/gix-cli.git
cd gix
pnpm install
pnpm build
pnpm link
```

然后你可以全局使用：

```bash
gix merge
```

#### 如果遇到 `pnpm link` 报错（ERR_PNPM_NO_GLOBAL_BIN_DIR）：

```bash
pnpm setup
source ~/.zshrc  # 或重启终端
```

---

## 🛠 使用示例

```bash
gix merge
```

将提示你输入：

- ✅ 起始提交 hash（必填）
- ✅ 结束提交 hash（可选，默认 HEAD）
- ✅ 新的提交信息
- ✅ 是否自动强推（默认是）

### 命令行参数方式：

```bash
gix merge -f a1b2c3d -m "feat: 优化打印逻辑"
```

---

## 📄 可用命令

- `gix merge`：合并多个提交并可选择强推

### 即将支持

- `gix squash`：自动合并最近多个提交
- `gix undo`：撤销上次 merge/reset 操作
- `gix log`：输出合并历史日志

---

## 👨‍💻 开发指南

```bash
pnpm dev       # 开发模式（使用 ts-node + esm loader）
pnpm build     # 使用 tsup 构建产物
```

---

## 📦 发布说明

1. 更新版本号：`npm version patch | minor | major`
2. 构建产物：`pnpm build`
3. 登录 npm：`npm login`
4. 发布包：`npm publish`

---

## ✨ 徽章展示

![npm](https://img.shields.io/npm/v/gix?style=flat-square)
![license](https://img.shields.io/npm/l/gix?style=flat-square)

---

## 📄 License

MIT © 2025 [License](./LICENSE)

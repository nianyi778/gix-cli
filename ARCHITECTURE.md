# 项目结构说明

本仓库采用 **Monorepo** 结构，维护 gix CLI 工具的两个实现版本。

## 目录结构

```
gix/
├── rust/                   # Rust 实现（推荐）
│   ├── Cargo.toml         # Rust 项目配置
│   ├── src/
│   │   ├── main.rs        # 入口文件
│   │   └── commands/      # 命令模块
│   ├── target/            # 编译输出（已忽略）
│   └── README.md          # Rust 版本文档
│
├── typescript/            # TypeScript 实现
│   ├── package.json       # npm 配置
│   ├── tsconfig.json      # TypeScript 配置
│   ├── tsup.config.ts     # 打包配置
│   ├── src/
│   │   ├── bin/          # CLI 入口
│   │   └── commands/      # 命令模块
│   ├── dist/              # 编译输出（已忽略）
│   └── README.md          # TypeScript 版本文档
│
├── .github/
│   ├── copilot-instructions.md  # AI 开发指南
│   └── workflows/         # CI/CD 配置
│
├── README.md              # 主文档
├── README.zh.md           # 中文文档
└── LICENSE                # MIT 许可证
```

## 两个版本对比

| 特性 | Rust 版本 | TypeScript 版本 |
|------|----------|----------------|
| **性能** | 快速启动，低内存 | 依赖 Node.js 运行时 |
| **依赖** | 无运行时依赖 | 需要 Node.js 18+ |
| **安装** | `cargo install` | `npm install -g` |
| **构建** | `cargo build` | `pnpm build` |
| **二进制** | 单个可执行文件 | 需要 node_modules |
| **状态** | ✅ 推荐使用 | 🔄 原始实现 |

## 开发工作流

### Rust 版本
```bash
cd rust
cargo run -- <command>      # 本地运行
cargo build --release        # 发布构建
cargo test                   # 运行测试
```

### TypeScript 版本
```bash
cd typescript
pnpm dev <command>           # 本地运行
pnpm build                   # 构建
pnpm test                    # 运行测试
```

## 功能一致性

两个版本提供完全相同的功能：
- ✅ `merge` - 合并多个提交
- ✅ `squash` - 压缩提交
- ✅ `doctor` - 检查 Git 环境
- ✅ `reset` - 重置本地提交

## 维护建议

1. **新功能开发**：优先在 Rust 版本实现
2. **Bug 修复**：两个版本同步修复
3. **文档更新**：保持两个版本文档一致
4. **测试**：确保两个版本行为一致

## CI/CD

- GitHub Actions 自动构建两个版本
- 发布时同时更新 crates.io 和 npm
- 自动生成 Release Notes

## 迁移路线

长期目标是推荐用户迁移到 Rust 版本，但 TypeScript 版本将继续维护以保证向后兼容。

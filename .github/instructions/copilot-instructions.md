# gix-cli Copilot 开发指南

## 项目概览
**gix** 是一个 Git 扩展 CLI 工具，包含两个实现版本：
- **Rust 版本** ([rust/](../../rust/)) — 现代、高性能的原生二进制（**推荐**）
- **TypeScript 版本** ([typescript/](../../typescript/)) — 原始 Node.js 实现

两个版本功能完全一致。本指南主要关注 **Rust 版本** 的开发。

## 架构设计
- **入口文件**: [rust/src/main.rs](../../rust/src/main.rs) — 使用 Clap 定义 CLI 结构并分发到各命令模块
- **命令模块**: [rust/src/commands/](../../rust/src/commands/) — 每个模块导出 `Args` 结构体（如需）和 `execute()` 函数
- **构建输出**: Cargo 将优化后的二进制文件构建到 `rust/target/release/gix` (详见 [rust/Cargo.toml](../../rust/Cargo.toml))

## 命令开发模式
每个命令必须遵循以下结构 (参考 [rust/src/commands/merge.rs](../../rust/src/commands/merge.rs)):
1. 使用 Clap 宏定义 `Args` 结构体用于接收 CLI 参数
2. 实现 `execute(args: Args) -> Result<()>` 函数
3. **前置检查**: 必须先调用 `is_working_directory_clean()?` 确保工作区干净
4. **交互输入**: 使用 `inquire::Text/Select/Confirm` 获取缺失的参数
5. **执行 Git**: 使用 `exec_git()` (获取输出) 或 `exec_git_interactive()` (交互式执行)
6. **用户反馈**: 使用带 Emoji 的清晰提示: ✅ 成功, ❌ 错误, ⚠️ 警告, 🔧 执行中

## 核心约定
- **Git 安全**: 破坏性操作前必须检查工作区是否干净 (`git status --porcelain`)
- **Upstream 处理**: 使用 `has_upstream()` 自动检测。如果缺失，使用 `git push --set-upstream` 而不是 `--force-with-lease`
- **错误处理**: 所有命令函数返回 `Result<()>`。使用 `?` 传播错误，`Err(String)` 返回用户友好的错误信息
- **交互式 I/O**: 需要用户输入（如编辑器、rebase 交互）的命令必须使用 `exec_git_interactive()`
- **Force Push**: 必须使用 `--force-with-lease` 保证安全，并总是先请求用户确认

## 开发工作流 (必读)
为确保功能设计合理且 CI/CD 一次通过，请严格遵守以下 **"设计-批准-实现"** 流程：

1. **设计阶段 (Design Phase)**:
   - 在 `_docs/space/` 下创建设计文档 (命名格式: `YYYYMMDD_feature_name.md`)。
   - 文档需包含：背景、核心逻辑、参数设计 (`Args` 结构)、执行流程、冲突处理策略、测试计划。
   - **必须等待用户确认批准**后，方可进入编码阶段。

2. **编码实现 (Implementation Phase)**:
   - 按照设计文档编写 Rust 代码。
   - 遵循“命令开发模式”和“核心约定”。

3. **代码检查 (Verification Phase)**:
   - 在提交前必须在 `rust` 目录下执行：
   ```bash
   cargo fmt                       # 格式化
   cargo clippy -- -D warnings     # 静态检查 (零警告)
   cargo check                     # 编译检查
   cargo build --release           # 构建检查
   ```

4. **提交交付 (Delivery Phase)**:
   - 更新设计文档状态为“已完成”。
   - 提交代码：
   ```bash
   git add .
   git commit -m "feat: add <feature>"
   git push
   ```

## Git 命令模式参考
- **Merge Commits**: `git reset --soft <from>^ && git commit` — 重置到起始 commit 的父节点，然后提交
- **Root Commit Check**: 使用 `git rev-list --max-parents=0 HEAD` 防止从根 commit 合并
- **Clean Check**: `git status --porcelain` 必须返回空字符串
- **Current Branch**: `git symbolic-ref --short HEAD`
- **Upstream Check**: `git rev-parse --abbrev-ref <branch>@{u}` (忽略错误即为无 upstream)

## 添加新命令步骤
1. **编写设计文档**并获得批准 (参考开发工作流)。
2. 在 [rust/src/commands/](../../rust/src/commands/) 创建文件 (例如 `mycommand.rs`)
3. 定义 `Args` 结构体并添加 `#[derive(Args)]`
4. 实现 `execute(args: Args) -> Result<()>`
5. 在 [rust/src/commands/mod.rs](../../rust/src/commands/mod.rs) 导出: `pub mod mycommand;`
6. 在 [rust/src/main.rs](../../rust/src/main.rs) 的 `Commands` 枚举中添加变体
7. 在 `main()` 的 `match` 语句中添加分发逻辑

## 外部依赖
- **clap**: CLI 框架 (`#[derive(Parser)]`, `#[arg(...)]`)
- **inquire**: 交互式提示 (`Text`, `Select`, `Confirm`)
- **colored**: 终端颜色 (`.red()`, `.green()`, `.yellow()`, `.cyan()`)
- **std::process::Command**: 执行 Git 命令 (`.status()` 或 `.output()`)


# 实战复盘：我为什么把 TypeScript 写的 CLI 工具用 Rust 重写了一遍？

> 仓库地址：[https://github.com/nianyi778/gix-cli](https://github.com/nianyi778/gix-cli)

## 前言

作为一个开发者，我们每天都在和 Git 打交道。为了简化一些繁琐的 Git 流程（比如合并多个 commit、交互式 rebase、清理工作区），我之前用 TypeScript 写了一个名为 **gix** 的 CLI 工具。

它工作得很好，但随着时间的推移，我开始思考：**是不是可以用 Rust 重写它？**

这不仅仅是为了蹭 "Rewrite it in Rust" 的热度，更是为了解决 Node.js CLI 工具的一些原生痛点，同时探索 Rust 在命令行工具开发领域的强大能力。

今天就来复盘一下，我是如何将 `gix` 从 TypeScript 迁移到 Rust，以及在这个过程中踩过的坑和收获的经验。

## 为什么要重写？

TypeScript 版本虽然开发效率高，但作为 CLI 工具，它有几个无法忽视的短板：

1.  **环境依赖**：用户必须安装 Node.js 才能运行。这对于非前端开发者来说是一个额外的负担。
2.  **分发困难**：虽然可以通过 npm 安装，但生成的 `node_modules` 体积庞大。
3.  **启动速度**：Node.js 的运行时启动虽然不慢，但相比于 Rust 编译后的原生二进制，还是有肉眼可见的差距。

而 Rust 完美解决了这些问题：
*   **零依赖**：编译成单个二进制文件，扔到哪里都能跑。
*   **高性能**：启动瞬间完成，内存占用极低。
*   **类型安全**：比 TypeScript 更严格的类型系统，在编译阶段就能拦截绝大多数错误。

## 技术栈对比

在重写过程中，我寻找了 TypeScript 生态中对应库的 Rust 替代品，发现 Rust 的 CLI 生态简直太丰富了：

| 功能 | TypeScript 方案 | Rust 方案 | 体验对比 |
| :--- | :--- | :--- | :--- |
| **CLI 框架** | `commander` | **`clap`** | `clap` 的 Derive 宏简直是魔法，直接通过结构体定义参数，自动生成帮助文档。 |
| **交互式提示** | `inquirer` | **`inquire`** | 类型更安全，API 设计非常直观。 |
| **终端颜色** | `chalk` | **`colored`** | 用法几乎一致，简单易用。 |
| **Git 执行** | `child_process` | **`std::process::Command`** | Rust 的标准库对进程控制提供了更细粒度的支持。 |

## 核心架构设计

### 1. Monorepo 结构
为了平滑过渡，我采用了 Monorepo 结构，同时保留了 `typescript` 和 `rust` 两个目录。这样老用户可以继续使用 npm 版本，新用户可以尝试 Rust 版本。

```text
.
├── rust/           # Rust 实现 (新)
│   ├── src/
│   │   ├── commands/   # 独立的命令模块
│   │   └── main.rs     # 入口分发
│   └── Cargo.toml
├── typescript/     # TypeScript 实现 (旧)
└── .github/        # CI/CD 工作流
```

### 2. 命令模式 (Command Pattern)
在 Rust 版本中，我利用 `clap` 的子命令功能，将每个功能（merge, rebase, doctor 等）封装成独立的模块。

```rust
// main.rs
#[derive(Subcommand)]
enum Commands {
    /// Merge multiple git commits
    Merge(merge::MergeArgs),
    /// Rebase current branch onto upstream
    Rebase(rebase::RebaseArgs),
    // ...
}
```

每个命令模块都遵循统一的接口：定义参数结构体 -> 实现 `execute` 函数。这种结构非常清晰，添加新命令只需“填空”即可。

## 踩坑与亮点

### 1. 交互式 Git 命令的“黑魔法”
在 CLI 中执行 `git rebase -i` 或 `git commit` 时，Git 可能会唤起编辑器（如 Vim）。在 Node.js 中处理这个需要一些技巧，而在 Rust 中，我们需要正确处理 `stdio`。

如果直接用 `.output()`，用户是看不到 Vim 界面的。必须使用 `.status()` 并继承父进程的 stdio：

```rust
pub fn exec_git_interactive(args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        // 关键：让子进程直接使用当前的终端输入输出
        .status()
        .map_err(|e| format!("❌ Failed to execute git command: {}", e))?;

    if !status.success() {
        return Err("❌ Git command failed".to_string());
    }
    Ok(())
}
```

### 2. 严格的开发工作流 (Copilot 辅助)
为了保证代码质量，我制定了一套严格的 **"Design-Approve-Implement"** 工作流，并写进了 `.github/instructions/copilot-instructions.md` 给 AI 助手看。

1.  **Design**: 先写 Markdown 设计文档，定义参数、流程、异常处理。
2.  **Approve**: **必须**等待我回复“批准”才能开始写代码。
3.  **Implement**: 编写 Rust 代码。
4.  **Verify**: 运行 `cargo clippy` 和 `cargo fmt`，**必须**等待我回复“测试通过”才能提交。

这套流程极大地减少了返工，比如最新的 `gix rebase` 功能就是完全按照这个流程一次性开发成功的。

### 3. 自动化的 CI/CD
为了省事，我配置了 GitHub Actions 实现全自动发版：
*   **TypeScript**: 监听 `typescript/package.json` 变化，自动发布到 npm。
*   **Rust**: 监听 `rust/Cargo.toml` 变化，自动打 Tag -> 编译 Release -> 发布到 GitHub Releases。

特别是 Rust 的自动打标流程，通过解析 `Cargo.toml` 版本号，自动判断是否需要创建 Git Tag，实现了“改个版本号就发版”的丝滑体验。

## 总结

从 TypeScript 到 Rust 的重写过程，不仅让 `gix` 变得更快、更轻，也让我深刻体会到了 Rust 在工程化方面的优势。

*   **Clap** 让 CLI 开发变成了一种享受。
*   **Rust 的类型系统** 让我在重构时充满信心，不用担心运行时莫名其妙的 `undefined`。
*   **Cargo** 的构建和依赖管理体验一流。

如果你也在维护 CLI 工具，强烈建议尝试用 Rust 重写一下，绝对会有意想不到的收获！

---

**欢迎 Star 和试用：** [https://github.com/nianyi778/gix-cli](https://github.com/nianyi778/gix-cli)

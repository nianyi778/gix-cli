# 设计文档：gix rebase 功能

**创建时间**: 2025-12-23
**状态**: 已完成

## 1. 背景与目标
用户希望通过 `gix rebase` 命令，将本地的 commit 记录“移动”到远程仓库最新记录的后面（即变基），以保持线性的提交历史。这比默认的 `git merge` 更能保持历史整洁。

## 2. 核心逻辑

该命令将自动化以下标准 Git 工作流：
1.  **检查环境**：确保工作区是干净的（无未提交更改）。
2.  **同步远程**：执行 `git fetch` 获取最新远程记录。
3.  **执行变基**：执行 `git rebase <upstream>`。
4.  **冲突处理**：如果变基过程中发生冲突，优雅地提示用户解决。

## 3. 详细设计 (Rust 版本)

### 3.1 新增模块
- 文件：`rust/src/commands/rebase.rs`
- 注册：在 `rust/src/main.rs` 中注册 `Rebase` 子命令。

### 3.2 参数设计 (`RebaseArgs`)
```rust
pub struct RebaseArgs {
    /// 指定上游分支 (例如 origin/main)
    /// 如果不指定，默认使用当前分支配置的 upstream
    #[arg(short, long)]
    upstream: Option<String>,
}
```

### 3.3 执行流程 (`execute`)

1.  **前置检查 (`Pre-check`)**
    - 调用 `is_working_directory_clean()`。如果不干净，报错并建议 `stash` 或 `commit`。

2.  **获取上游 (`Resolve Upstream`)**
    - 如果用户提供了 `--upstream`，直接使用。
    - 如果未提供，通过 `git rev-parse --abbrev-ref @{u}` 获取当前分支的 upstream。
    - 如果没有配置 upstream，报错提示用户指定或先 push。

3.  **拉取更新 (`Fetch`)**
    - 执行 `git fetch` (或者 `git fetch origin`，视 upstream 而定)。
    - 显示进度提示：`🔄 Fetching latest changes...`

4.  **执行变基 (`Rebase`)**
    - 执行 `git rebase <upstream>`。
    - 使用 `exec_git_interactive` 以便让用户看到 git 的原生输出。

5.  **结果处理**
    - **成功**：显示 `✅ Rebase successful!`。
    - **失败 (冲突)**：
        - 捕获非 0 退出码。
        - 显示清晰的冲突提示：
          ```text
          ❌ Rebase encountered conflicts.
          ⚠️  Please resolve conflicts manually.
          👉 After resolving:
             1. git add <files>
             2. git rebase --continue
          👉 To abort:
             git rebase --abort
          ```

## 4. 冲突处理策略
- v1 版本**不自动**处理冲突，也不封装 `continue` 命令。
- 理由：冲突解决高度依赖上下文，且 Git 原生提示已经足够清晰。`gix` 的价值在于自动化“Fetch + Rebase”的动作，并在出错时提供更友好的引导。

## 5. 验收测试范围

开发完成后，将进行以下测试：

1.  **Happy Path (无冲突)**：
    - 本地落后于远程，执行 `gix rebase`，成功将本地 commit 接到远程后面。
2.  **Dirty Worktree (工作区不干净)**：
    - 修改文件不提交，执行 `gix rebase`，应报错并停止。
3.  **Conflict (有冲突)**：
    - 制造本地和远程修改同一行的场景。
    - 执行 `gix rebase`，应报错并显示上述引导信息。
    - 手动解决后 `git rebase --continue` 能成功完成。
4.  **No Upstream (无上游)**：
    - 新建本地分支不推送到远程，执行 `gix rebase`，应报错提示。

## 6. 待确认事项
- 是否需要默认支持 `--autostash`？
    - **建议**：否。为了安全起见，要求用户明确处理本地变更（commit 或 stash）后再执行变基，避免隐式 stash 导致混乱。

---
请确认以上设计方案是否符合预期？如果确认，我将开始编码。

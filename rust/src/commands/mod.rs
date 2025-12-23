pub mod doctor;
pub mod merge;
pub mod rebase;
pub mod reset;
pub mod squash;

use colored::Colorize;
use std::process::{Command, Output};

pub type Result<T> = std::result::Result<T, String>;

/// Execute a git command and return the output
pub fn exec_git(args: &[&str]) -> Result<Output> {
    Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("❌ Failed to execute git command: {}", e))
}

/// Execute a git command with inherited stdio (for interactive commands)
pub fn exec_git_interactive(args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .status()
        .map_err(|e| format!("❌ Failed to execute git command: {}", e))?;

    if !status.success() {
        return Err("❌ Git command failed".to_string());
    }

    Ok(())
}

/// Check if working directory is clean
pub fn is_working_directory_clean() -> Result<bool> {
    let output = exec_git(&["status", "--porcelain"])?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().is_empty())
}

/// Get current branch name
pub fn get_current_branch() -> Result<String> {
    let output = exec_git(&["symbolic-ref", "--short", "HEAD"])?;
    if !output.status.success() {
        return Err("❌ Failed to get current branch".to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check if branch has upstream
pub fn has_upstream(branch: &str) -> bool {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", &format!("{}@{{u}}", branch)])
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Print success message
pub fn print_success(msg: &str) {
    println!("{}", format!("✅ {}", msg).green());
}

/// Print error message
pub fn print_error(msg: &str) {
    eprintln!("{}", format!("❌ {}", msg).red());
}

/// Print warning message
pub fn print_warning(msg: &str) {
    println!("{}", format!("⚠️  {}", msg).yellow());
}

/// Print info message
#[allow(dead_code)]
pub fn print_info(msg: &str) {
    println!("{}", format!("🔧 {}", msg).cyan());
}

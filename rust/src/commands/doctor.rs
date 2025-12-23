use super::*;
use std::process::Command;

pub fn execute() -> Result<()> {
    println!("🩺 Running gix health check...\n");

    // Check Node.js version (since this is now a Rust binary, we check Rust version instead)
    if let Ok(output) = Command::new("rustc").arg("--version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        print_success(&format!("Rust version: {}", version.trim()));
    } else {
        print_warning("Rust compiler not found (but gix is running)");
    }

    // Check if git is available
    match exec_git(&["--version"]) {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            print_success(&format!("Git installed: {}", version.trim()));
        }
        Err(_) => {
            print_error("Git not found in PATH");
            return Ok(());
        }
    }

    // Check if inside a git repository
    match exec_git(&["rev-parse", "--is-inside-work-tree"]) {
        Ok(output) if output.status.success() => {
            print_success("Inside a Git repository");
        }
        _ => {
            print_error("Not inside a Git repository");
            return Ok(());
        }
    }

    // Check working directory status
    if is_working_directory_clean()? {
        print_success("Working directory is clean");
    } else {
        print_warning("Working directory has uncommitted changes");
    }

    // Check for remotes
    match exec_git(&["remote"]) {
        Ok(output) => {
            let remotes = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !remotes.is_empty() {
                print_success(&format!("Remote(s) configured: {}", remotes));
            } else {
                print_warning("No Git remotes configured");
            }
        }
        Err(_) => {
            print_warning("Failed to check remotes");
        }
    }

    // Get current branch
    match get_current_branch() {
        Ok(branch) => {
            println!("📍 Current branch: {}", branch);
        }
        Err(_) => {
            print_warning("Failed to get current branch");
        }
    }

    println!("\n🧩 Done.");
    Ok(())
}

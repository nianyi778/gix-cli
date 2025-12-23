use super::*;
use clap::Args;

#[derive(Args)]
pub struct RebaseArgs {
    /// Specify upstream branch (e.g. origin/main)
    /// If not specified, uses the configured upstream of the current branch
    #[arg(short, long)]
    upstream: Option<String>,
}

pub fn execute(args: RebaseArgs) -> Result<()> {
    // 1. Pre-check: Ensure working directory is clean
    if !is_working_directory_clean()? {
        return Err(
            "❌ Your working directory is not clean. Please commit, stash, or reset changes before rebasing."
                .to_string(),
        );
    }

    // 2. Resolve Upstream
    let upstream = match args.upstream {
        Some(u) => u,
        None => {
            // Try to get the configured upstream
            let current_branch = get_current_branch()?;
            let output = std::process::Command::new("git")
                .args([
                    "rev-parse",
                    "--abbrev-ref",
                    &format!("{}@{{u}}", current_branch),
                ])
                .output()
                .map_err(|e| format!("❌ Failed to execute git command: {}", e))?;

            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                // If no upstream is configured, prompt the user or error out
                // According to design: "If no upstream, error out"
                // But maybe we can be nicer and ask? Design says "Error out".
                // Let's stick to design for now, but maybe add a hint.
                return Err(format!(
                    "❌ No upstream configured for branch '{}'.\n👉 Please specify one with --upstream or set it via `git branch --set-upstream-to <remote>/<branch>`",
                    current_branch
                ));
            }
        }
    };

    println!("🔧 Target upstream: {}", upstream);

    // 3. Fetch latest changes
    println!("🔄 Fetching latest changes...");
    let fetch_result = exec_git_interactive(&["fetch"]);
    if let Err(e) = fetch_result {
        return Err(format!("❌ Failed to fetch: {}", e));
    }

    // 4. Execute Rebase
    println!("🚀 Rebasing onto {}...", upstream);
    let rebase_result = exec_git_interactive(&["rebase", &upstream]);

    match rebase_result {
        Ok(_) => {
            print_success("Rebase successful!");
            Ok(())
        }
        Err(_) => {
            // 5. Handle Conflict/Failure
            eprintln!("\n❌ Rebase encountered conflicts or failed.");
            print_warning("Please resolve conflicts manually.");
            println!("👉 After resolving:\n   1. git add <files>\n   2. git rebase --continue");
            println!("👉 To abort:\n   git rebase --abort");

            // We return an error so the main process exits with non-zero,
            // but we've already printed the helpful message.
            // Returning a specific error string that main will print.
            Err("Rebase failed. See instructions above.".to_string())
        }
    }
}

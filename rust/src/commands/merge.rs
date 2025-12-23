use super::*;
use clap::Args;
use inquire::{Select, Text};

#[derive(Args)]
pub struct MergeArgs {
    /// Start commit hash (required)
    #[arg(short, long)]
    from: Option<String>,

    /// End commit hash (default is HEAD)
    #[arg(short, long)]
    to: Option<String>,

    /// New commit message
    #[arg(short, long)]
    msg: Option<String>,
}

pub fn execute(args: MergeArgs) -> Result<()> {
    // Get or prompt for 'from' commit hash
    let from = match args.from {
        Some(hash) => hash,
        None => Text::new("Enter the start commit hash:")
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(inquire::validator::Validation::Invalid(
                        "Start commit hash is required.".into(),
                    ))
                } else {
                    Ok(inquire::validator::Validation::Valid)
                }
            })
            .prompt()
            .map_err(|e| format!("❌ Failed to get input: {}", e))?,
    };

    // Get or prompt for 'to' commit hash (not currently used in reset command)
    let _to = match args.to {
        Some(hash) => hash,
        None => {
            let input = Text::new("Enter the end commit hash (leave blank for HEAD):")
                .with_default("HEAD")
                .prompt()
                .map_err(|e| format!("❌ Failed to get input: {}", e))?;
            if input.trim().is_empty() {
                "HEAD".to_string()
            } else {
                input
            }
        }
    };

    // Get or prompt for commit message
    let msg = match args.msg {
        Some(message) => message,
        None => Text::new("Enter the new commit message:")
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(inquire::validator::Validation::Invalid(
                        "Commit message cannot be empty.".into(),
                    ))
                } else {
                    Ok(inquire::validator::Validation::Valid)
                }
            })
            .prompt()
            .map_err(|e| format!("❌ Failed to get input: {}", e))?,
    };

    // Check if trying to merge from root commit
    let root_commits = exec_git(&["rev-list", "--max-parents=0", "HEAD"])?;
    let root_commits_str = String::from_utf8_lossy(&root_commits.stdout);
    if root_commits_str.lines().any(|line| line.trim() == from.trim()) {
        return Err(
            "❌ Cannot merge from the first (root) commit — it has no parent.\n👉 Consider using `git rebase --root` instead."
                .to_string(),
        );
    }

    // Check if working directory is clean
    if !is_working_directory_clean()? {
        return Err(
            "❌ Your working directory is not clean. Please commit, stash, or reset changes before merging."
                .to_string(),
        );
    }

    // Execute reset and commit
    let reset_cmd = format!("git reset --soft {}^", from);
    let commit_cmd = format!("git commit --edit -m \"{}\" --no-verify", msg);

    println!("\n🔧 Executing:\n{} && {}\n", reset_cmd, commit_cmd);

    // Reset
    exec_git_interactive(&["reset", "--soft", &format!("{}^", from)])?;
    print_success("Reset successful");

    // Commit
    exec_git_interactive(&["commit", "--edit", "-m", &msg, "--no-verify"])?;
    print_success("Commit successful");

    // Ask for push confirmation
    let options = vec!["✅ Yes (default)", "❌ No, I will push manually"];
    let push_confirm = Select::new("Do you want to force push automatically?", options)
        .with_starting_cursor(0)
        .prompt()
        .map_err(|e| format!("❌ Failed to get confirmation: {}", e))?;

    if push_confirm == "✅ Yes (default)" {
        let current_branch = get_current_branch()?;

        if !has_upstream(&current_branch) {
            println!(
                "\n🚀 No upstream detected. Executing: git push --set-upstream origin {}\n",
                current_branch
            );
            exec_git_interactive(&["push", "--set-upstream", "origin", &current_branch])?;
            print_success("Push & upstream set successfully");
        } else {
            println!("\n🚀 Executing git push --force-with-lease\n");
            exec_git_interactive(&["push", "--force-with-lease"])?;
            print_success("Force push successful");
        }
    } else {
        print_warning("Please push manually using git push");
    }

    Ok(())
}

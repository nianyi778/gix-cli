use super::*;
use clap::Args;

#[derive(Args)]
pub struct SquashArgs {
    /// Number of commits to squash (default 2)
    #[arg(short, long, default_value = "2")]
    number: u32,

    /// Squash all commits from the beginning
    #[arg(long)]
    all: bool,
}

pub fn execute(args: SquashArgs) -> Result<()> {
    // Check if working directory is clean
    if !is_working_directory_clean()? {
        return Err(
            "❌ Your working directory is not clean. Please commit, stash, or reset changes before squashing."
                .to_string(),
        );
    }

    if args.all {
        println!("🧨 Running: git rebase -i --root");
        exec_git_interactive(&["rebase", "-i", "--root"])?;
    } else {
        let count = args.number;
        println!("🧨 Running: git rebase -i HEAD~{}", count);
        exec_git_interactive(&["rebase", "-i", &format!("HEAD~{}", count)])?;
    }

    Ok(())
}

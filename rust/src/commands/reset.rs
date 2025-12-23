use super::*;
use inquire::Confirm;

pub fn execute() -> Result<()> {
    let branch = get_current_branch()?;

    // Get remote name for current branch
    let remote_output = exec_git(&["config", &format!("branch.{}.remote", branch)])?;
    if !remote_output.status.success() {
        return Err(format!("❌ Failed to get remote for branch '{}'", branch));
    }
    let remote = String::from_utf8_lossy(&remote_output.stdout).trim().to_string();
    let remote_ref = format!("{}/{}", remote, branch);

    // Confirm with user
    let proceed = Confirm::new(&format!(
        "⚠️  This will remove all local commits not pushed to {}. Proceed?",
        remote_ref
    ))
    .with_default(false)
    .prompt()
    .map_err(|e| format!("❌ Failed to get confirmation: {}", e))?;

    if !proceed {
        println!("❌ Cancelled.");
        return Ok(());
    }

    println!("\n🧨 Running: git reset --soft {}", remote_ref);
    exec_git_interactive(&["reset", "--soft", &remote_ref])?;
    print_success("Local commits have been discarded (soft reset)");

    Ok(())
}

mod commands;

use clap::{Parser, Subcommand};
use commands::{doctor, merge, reset, squash};

#[derive(Parser)]
#[command(name = "gix")]
#[command(version, about = "Gix: A Git extension CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Merge multiple git commits
    Merge(merge::MergeArgs),
    /// Squash recent commits into one
    Squash(squash::SquashArgs),
    /// Check git working directory and environment
    Doctor,
    /// Discard all local unpushed commits (soft reset to remote)
    Reset,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Merge(args) => merge::execute(args),
        Commands::Squash(args) => squash::execute(args),
        Commands::Doctor => doctor::execute(),
        Commands::Reset => reset::execute(),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

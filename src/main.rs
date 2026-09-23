use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod listen;
mod runner;
mod server;

/// Utility tool for competitive companion
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Listens to Competitive Companion
    Listen {
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
        /// Listen to one problem only
        #[arg(short, long)]
        once: bool,
    },
    /// Compiles, runs and compares program output with tests
    Run {
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
        /// Set a time limit for test cases (Default = 1)
        #[arg(short, long, default_value_t = 1)]
        limit: u64,
    },
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Listen { path, once } => listen::cli_listen(path.as_deref(), *once)?,
        Commands::Run { path, limit } => runner::run(path.as_deref(), *limit)?,
    };
    Ok(())
}

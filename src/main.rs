use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod listen;
mod server;

// Utility tool for competitive companion
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Listens to Competitive Companion
    Listen {
        #[arg(value_name="PATH")]
        path: Option<PathBuf>,
    }
}
fn main (){
    let cli = Cli::parse();
    match &cli.command {
        Commands::Listen { path } => match listen::cli_listen(path.as_deref()){
            Ok(()) => (),
            Err(e) => eprintln!("Error {e}"),
        },
    };

}

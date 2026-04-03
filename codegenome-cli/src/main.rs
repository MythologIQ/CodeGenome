mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "codegenome", about = "Unified Code Reality Graph")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the index pipeline on a repository
    Index,
    /// Query the code graph
    Query,
    /// Show index freshness and system state
    Status,
    /// Verify Merkle chain integrity
    Verify,
    /// Start MCP tool server
    Serve,
    /// Run autonomous experiment loop
    Experiment {
        /// Source directory to index
        #[arg(long, default_value = ".")]
        source_dir: String,
        /// TSV log file path
        #[arg(long, default_value = "experiments.tsv")]
        log_file: String,
        /// Maximum iterations (infinite if omitted)
        #[arg(long)]
        max_iterations: Option<u64>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Index => commands::stub("index"),
        Commands::Query => commands::stub("query"),
        Commands::Status => commands::stub("status"),
        Commands::Verify => commands::stub("verify"),
        Commands::Serve => commands::stub("serve"),
        Commands::Experiment {
            source_dir,
            log_file,
            max_iterations,
        } => commands::experiment::run(
            &source_dir,
            &log_file,
            max_iterations,
        ),
    }
}

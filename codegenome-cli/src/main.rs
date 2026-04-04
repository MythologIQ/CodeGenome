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
        /// LLM model ID for Tier 2 advisor (e.g. microsoft/Phi-3.5-mini-instruct)
        #[arg(long)]
        model: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Experiment {
            source_dir,
            log_file,
            max_iterations,
            model,
        } => commands::experiment::run(
            &source_dir,
            &log_file,
            max_iterations,
            model,
        ),
    }
}

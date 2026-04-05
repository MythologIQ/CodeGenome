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
    /// Build the code graph from source files
    Index {
        #[arg(long, default_value = ".")]
        source_dir: String,
        #[arg(long, default_value = ".codegenome")]
        store_dir: String,
    },
    /// Query impact from a file:line location
    Query {
        #[arg(long, default_value = ".codegenome")]
        store_dir: String,
        #[arg(long)]
        file: String,
        #[arg(long)]
        line: u32,
        #[arg(long, default_value = "downstream")]
        direction: String,
        #[arg(long)]
        json: bool,
    },
    /// Show index status and overlay counts
    Status {
        #[arg(long, default_value = ".codegenome")]
        store_dir: String,
        #[arg(long)]
        json: bool,
    },
    /// Verify experiment TSV chain integrity
    Verify {
        #[arg(long, default_value = "experiments.tsv")]
        log_file: String,
    },
    /// Run autonomous experiment loop
    Experiment {
        #[arg(long, default_value = ".")]
        source_dir: String,
        #[arg(long, default_value = "experiments.tsv")]
        log_file: String,
        #[arg(long)]
        max_iterations: Option<u64>,
        #[arg(long, default_value = "microsoft/Phi-3-mini-4k-instruct")]
        model: String,
        #[arg(long)]
        no_model: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Index { source_dir, store_dir } => {
            commands::index::run(&source_dir, &store_dir);
        }
        Commands::Query { store_dir, file, line, direction, json } => {
            commands::query::run(&store_dir, &file, line, &direction, json);
        }
        Commands::Status { store_dir, json } => {
            commands::status::run(&store_dir, json);
        }
        Commands::Verify { log_file } => {
            commands::verify::run(&log_file);
        }
        Commands::Experiment {
            source_dir, log_file, max_iterations, model, no_model,
        } => commands::experiment::run(
            &source_dir, &log_file, max_iterations,
            if no_model { None } else { Some(model) },
        ),
    }
}

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
    /// Start MCP tool server (stdio)
    Serve {
        #[arg(long, default_value = ".")]
        source_dir: String,
        #[arg(long, default_value = ".codegenome")]
        store_dir: String,
    },
    /// Initialize .mcp.json for Claude Code integration
    Init {
        #[arg(long, default_value = ".")]
        source_dir: String,
        #[arg(long, default_value = ".codegenome")]
        store_dir: String,
    },
    /// Verify experiment TSV chain integrity
    Verify {
        #[arg(long, default_value = "experiments.tsv")]
        log_file: String,
    },
    /// Analyze repo-local experiment results
    Analyze {
        #[arg(long, default_value = "experiments.tsv")]
        log_file: String,
        #[arg(long)]
        json: bool,
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
    /// Build explicit workspace federation overlay
    Federate {
        #[arg(long)]
        workspace_config: String,
        #[arg(long, default_value = ".codegenome-workspace")]
        store_dir: String,
    },
    /// Report workspace federation metrics
    WorkspaceReport {
        #[arg(long, default_value = ".codegenome-workspace")]
        store_dir: String,
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Index {
            source_dir,
            store_dir,
        } => {
            commands::index::run(&source_dir, &store_dir);
        }
        Commands::Query {
            store_dir,
            file,
            line,
            direction,
            json,
        } => {
            commands::query::run(&store_dir, &file, line, &direction, json);
        }
        Commands::Status { store_dir, json } => {
            commands::status::run(&store_dir, json);
        }
        Commands::Serve {
            source_dir,
            store_dir,
        } => {
            commands::serve::run(&source_dir, &store_dir);
        }
        Commands::Init {
            source_dir,
            store_dir,
        } => {
            commands::init::run(&source_dir, &store_dir);
        }
        Commands::Verify { log_file } => {
            commands::verify::run(&log_file);
        }
        Commands::Analyze { log_file, json } => {
            commands::analyze::run(&log_file, json);
        }
        Commands::Experiment {
            source_dir,
            log_file,
            max_iterations,
            model,
            no_model,
        } => commands::experiment::run(
            &source_dir,
            &log_file,
            max_iterations,
            if no_model { None } else { Some(model) },
        ),
        Commands::Federate {
            workspace_config,
            store_dir,
        } => {
            commands::federate::run(&workspace_config, &store_dir);
        }
        Commands::WorkspaceReport { store_dir, json } => {
            commands::workspace_report::run(&store_dir, json);
        }
    }
}

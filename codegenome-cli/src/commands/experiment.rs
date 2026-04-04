use std::path::{Path, PathBuf};

use codegenome_core::experiments::config::*;
use codegenome_core::experiments::runner;
use codegenome_core::graph::overlay::OverlayKind;

pub fn run(
    source_dir: &str,
    log_file: &str,
    max_iterations: Option<u64>,
) {
    let infra = ExperimentInfra {
        source_dir: PathBuf::from(source_dir),
        overlays: vec![OverlayKind::Syntax, OverlayKind::Semantic, OverlayKind::Flow],
        fitness_fn: FitnessFunction::ImpactAccuracy,
    };
    let params = ExperimentParams::default();

    println!("=== CODEGENOME Experiment Loop ===");
    println!("Source: {source_dir}");
    println!("Log:    {log_file}");
    match max_iterations {
        Some(n) => println!("Iterations: {n}"),
        None => println!("Iterations: infinite (Ctrl+C to stop)"),
    }
    println!("==================================");

    runner::run_continuous(
        &infra,
        params,
        Path::new(log_file),
        max_iterations,
    );

    println!("Experiment loop complete.");
}

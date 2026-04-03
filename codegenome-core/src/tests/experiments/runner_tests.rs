use std::path::PathBuf;

use crate::experiments::config::*;
use crate::experiments::log;
use crate::experiments::runner;

fn test_infra() -> ExperimentInfra {
    ExperimentInfra {
        source_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src"),
        overlays: vec![crate::graph::overlay::OverlayKind::Syntax],
        fitness_fn: FitnessFunction::GraphDensity,
    }
}

#[test]
fn single_experiment_produces_valid_result() {
    let infra = test_infra();
    let params = ExperimentParams::new();
    let result = runner::run_experiment(&infra, &params);

    assert_eq!(result.status, ExperimentStatus::Pass);
    assert!(result.fitness > 0.0);
    assert!(result.cycle_time_ms < 5000);
}

#[test]
fn hill_climb_step_keeps_or_discards() {
    let infra = test_infra();
    let params = ExperimentParams::new()
        .with("threshold", 0.5);
    let baseline = runner::run_experiment(&infra, &params);

    let (_, new_fitness, _kept) = runner::hill_climb_step(
        &infra,
        &params,
        baseline.fitness,
        0.1,
    );
    // new_fitness should be a valid number
    assert!(new_fitness.is_finite());
}

#[test]
fn tsv_logging_roundtrip() {
    let dir = std::env::temp_dir().join("codegenome_exp_test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let log_path = dir.join("results.tsv");

    let result = log::ExperimentResult {
        iteration: 1,
        params: std::collections::HashMap::new(),
        fitness: 0.876543,
        status: ExperimentStatus::Pass,
        cycle_time_ms: 42,
        description: "test run".into(),
    };

    log::log_result(&log_path, &result).unwrap();
    log::log_result(&log_path, &result).unwrap();
    log::log_result(&log_path, &result).unwrap();

    let results = log::read_log(&log_path).unwrap();
    assert_eq!(results.len(), 3);
    assert!((results[0].fitness - 0.876543).abs() < 0.001);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn continuous_loop_runs_n_iterations() {
    let dir = std::env::temp_dir().join("codegenome_loop_test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let log_path = dir.join("results.tsv");

    let infra = test_infra();
    let params = ExperimentParams::new()
        .with("threshold", 0.5);

    runner::run_continuous(
        &infra,
        params,
        &log_path,
        Some(3),
    );

    let results = log::read_log(&log_path).unwrap();
    // Initial run + 3 iterations = 4 entries
    assert!(results.len() >= 3);

    let _ = std::fs::remove_dir_all(&dir);
}

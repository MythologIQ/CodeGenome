use std::collections::HashMap;

use crate::experiments::analysis;
use crate::experiments::config::ExperimentStatus;
use crate::experiments::log::{self, ExperimentResult};

fn result(iteration: u64, fitness: f64, alpha: f64) -> ExperimentResult {
    let mut params = HashMap::new();
    params.insert("alpha".into(), alpha);
    ExperimentResult {
        iteration,
        params,
        fitness,
        stability: 0.8 + (iteration as f64 * 0.01),
        status: ExperimentStatus::Pass,
        cycle_time_ms: 10 + iteration,
        description: format!("run {iteration}"),
        chain_hash: String::new(),
    }
}

#[test]
fn report_uses_logged_values_for_correlations() {
    let dir = std::env::temp_dir().join(format!("cg_analysis_{:?}", std::thread::current().id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("results.tsv");

    let mut prev = log::genesis_hash();
    for (i, fitness, alpha) in [(1, 0.2, 1.0), (2, 0.4, 2.0), (3, 0.6, 3.0)] {
        prev = log::log_result(&path, &result(i, fitness, alpha), &prev).unwrap();
    }

    let report = analysis::build_report(&path).unwrap();
    assert_eq!(report.run_count, 3);
    assert!(report.convergence_rate > 0.0);
    assert_eq!(report.parameter_correlations.len(), 1);
    assert_eq!(report.parameter_correlations[0].name, "alpha");
    assert!(report.parameter_correlations[0].correlation > 0.99);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn report_is_deterministic_for_same_log() {
    let dir = std::env::temp_dir().join(format!(
        "cg_analysis_stable_{:?}",
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("results.tsv");

    let mut prev = log::genesis_hash();
    for (i, fitness, alpha) in [(1, 0.5, 1.0), (2, 0.4, 2.0), (3, 0.7, 3.0)] {
        prev = log::log_result(&path, &result(i, fitness, alpha), &prev).unwrap();
    }

    let a = analysis::build_report(&path).unwrap();
    let b = analysis::build_report(&path).unwrap();
    assert_eq!(a.convergence_rate, b.convergence_rate);
    assert_eq!(a.fitness.mean, b.fitness.mean);
    assert_eq!(a.parameter_correlations, b.parameter_correlations);
    let _ = std::fs::remove_dir_all(&dir);
}

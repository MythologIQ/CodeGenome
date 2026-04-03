use std::path::Path;

use rand::Rng;

use crate::experiments::config::*;
use crate::experiments::fitness;
use crate::experiments::log::{log_result, ExperimentResult};

/// Run one experiment cycle. Returns fitness + stability.
pub fn run_experiment(
    infra: &ExperimentInfra,
    params: &ExperimentParams,
) -> ExperimentResult {
    let start = std::time::Instant::now();

    let accuracy = fitness::impact_accuracy(&infra.source_dir, params);
    let stab = fitness::stability(&infra.source_dir, params);

    let elapsed = start.elapsed();

    ExperimentResult {
        iteration: 0,
        params: params.values.clone(),
        fitness: accuracy,
        stability: stab,
        status: ExperimentStatus::Pass,
        cycle_time_ms: elapsed.as_millis() as u64,
        description: format_params(params),
    }
}

/// Hill-climbing: perturb params, run, keep if better.
/// Returns the perturbed result so we don't re-run.
pub fn hill_climb_step(
    infra: &ExperimentInfra,
    current: &ExperimentParams,
    current_fitness: f64,
    current_stability: f64,
    perturbation_scale: f64,
) -> (ExperimentParams, ExperimentResult, bool) {
    let perturbed = perturb(current, perturbation_scale);
    let result = run_experiment(infra, &perturbed);
    let dominated = result.fitness > current_fitness;
    let pareto = (result.fitness - current_fitness).abs() < 0.0001
        && result.stability > current_stability;
    let kept = dominated || pareto;
    (perturbed, result, kept)
}

/// Run experiments continuously until max_iterations.
pub fn run_continuous(
    infra: &ExperimentInfra,
    initial_params: ExperimentParams,
    log_path: &Path,
    max_iterations: Option<u64>,
) {
    let mut params = initial_params;
    let mut result = run_experiment(infra, &params);
    result.iteration = 0;
    result.description = "baseline".into();
    let _ = log_result(log_path, &result);
    let mut best_fitness = result.fitness;
    let mut best_stability = result.stability;

    eprintln!(
        "[0] baseline: fitness={:.4} stability={:.4} ({} ms)",
        result.fitness, result.stability, result.cycle_time_ms,
    );

    let limit = max_iterations.unwrap_or(u64::MAX);
    for i in 1..=limit {
        let (perturbed, mut step_result, kept) =
            hill_climb_step(infra, &params, best_fitness, best_stability, 0.1);

        step_result.iteration = i;
        step_result.description = if kept {
            format!("keep: {}", format_params(&perturbed))
        } else {
            format!("discard: {}", format_params(&perturbed))
        };
        let _ = log_result(log_path, &step_result);

        eprintln!(
            "[{i}] {}: fitness={:.4} stability={:.4} ({} ms)",
            if kept { "KEEP" } else { "discard" },
            step_result.fitness,
            step_result.stability,
            step_result.cycle_time_ms,
        );

        if kept {
            params = perturbed;
            best_fitness = step_result.fitness;
            best_stability = step_result.stability;
        }
    }
}

fn perturb(
    params: &ExperimentParams,
    scale: f64,
) -> ExperimentParams {
    let mut rng = rand::rng();
    let mut new = params.clone();
    for value in new.values.values_mut() {
        let delta: f64 = rng.random_range(-scale..=scale);
        *value += delta;
    }
    new
}

fn format_params(params: &ExperimentParams) -> String {
    if params.values.is_empty() {
        return "default".into();
    }
    params
        .values
        .iter()
        .map(|(k, v)| format!("{k}={v:.3}"))
        .collect::<Vec<_>>()
        .join(", ")
}

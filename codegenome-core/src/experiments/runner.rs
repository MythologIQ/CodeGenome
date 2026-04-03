use std::path::Path;

use crate::experiments::config::*;
use crate::experiments::log::{log_result, ExperimentResult};
use crate::overlay::syntax::parse_rust_files;
use crate::graph::overlay::Overlay;
use crate::signal::impact::propagate_impact;

/// Run one experiment cycle. Returns fitness score.
pub fn run_experiment(
    infra: &ExperimentInfra,
    params: &ExperimentParams,
) -> ExperimentResult {
    let start = std::time::Instant::now();

    let files = collect_source_files(&infra.source_dir);
    let overlay = parse_rust_files(&files);

    let fitness = evaluate_fitness(
        &infra.fitness_fn,
        &overlay,
        params,
    );

    let elapsed = start.elapsed();

    ExperimentResult {
        iteration: 0,
        params: params.values.clone(),
        fitness,
        status: ExperimentStatus::Pass,
        cycle_time_ms: elapsed.as_millis() as u64,
        description: format_params(params),
    }
}

/// Hill-climbing: perturb params, run, keep if better.
pub fn hill_climb_step(
    infra: &ExperimentInfra,
    current: &ExperimentParams,
    current_fitness: f64,
    perturbation_scale: f64,
) -> (ExperimentParams, f64, bool) {
    let perturbed = perturb(current, perturbation_scale);
    let result = run_experiment(infra, &perturbed);

    let kept = result.fitness > current_fitness;
    if kept {
        (perturbed, result.fitness, true)
    } else {
        (current.clone(), current_fitness, false)
    }
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
    let _ = log_result(log_path, &result);
    let mut best_fitness = result.fitness;

    let limit = max_iterations.unwrap_or(u64::MAX);
    for i in 1..=limit {
        let (new_params, new_fitness, kept) =
            hill_climb_step(infra, &params, best_fitness, 0.1);

        let mut step_result = run_experiment(infra, &new_params);
        step_result.iteration = i;
        step_result.description = if kept {
            format!("keep: {}", format_params(&new_params))
        } else {
            format!("discard: {}", format_params(&new_params))
        };
        let _ = log_result(log_path, &step_result);

        if kept {
            params = new_params;
            best_fitness = new_fitness;
        }
    }
}

fn evaluate_fitness(
    fitness_fn: &FitnessFunction,
    overlay: &dyn Overlay,
    _params: &ExperimentParams,
) -> f64 {
    match fitness_fn {
        FitnessFunction::GraphDensity => {
            let nodes = overlay.nodes().len() as f64;
            let edges = overlay.edges().len() as f64;
            if nodes == 0.0 { 0.0 } else { edges / nodes }
        }
        FitnessFunction::PropagationDepth => {
            if overlay.nodes().is_empty() { return 0.0; }
            let root = overlay.nodes()[0].address;
            let impact = propagate_impact(
                &[root],
                &[overlay],
            );
            impact.len() as f64
        }
        FitnessFunction::CycleTime => {
            // Lower is better — return negative so hill-climb maximizes
            0.0
        }
        _ => 0.0,
    }
}

fn perturb(
    params: &ExperimentParams,
    scale: f64,
) -> ExperimentParams {
    let mut new = params.clone();
    for value in new.values.values_mut() {
        // Simple random walk: +/- scale
        let direction = if (*value * 1000.0) as u64 % 2 == 0 {
            1.0
        } else {
            -1.0
        };
        *value += direction * scale;
    }
    new
}

fn format_params(params: &ExperimentParams) -> String {
    params
        .values
        .iter()
        .map(|(k, v)| format!("{k}={v:.3}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn collect_source_files(
    dir: &std::path::Path,
) -> Vec<(std::path::PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_source_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            if let Ok(content) = std::fs::read(&path) {
                files.push((path, content));
            }
        }
    }
    files
}

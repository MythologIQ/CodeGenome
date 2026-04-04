use std::path::Path;

use rand::Rng;

use crate::experiments::config::*;
use crate::experiments::fitness;
use crate::experiments::log::{log_result, ExperimentResult};
use crate::experiments::advisor;
use crate::experiments::review::{self, Action, ReviewState};

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

/// Mutable state for the continuous loop.
struct LoopState {
    params: ExperimentParams,
    best_fitness: f64,
    best_stability: f64,
    scale: f64,
    reviewer: ReviewState,
}

/// Run experiments continuously with adaptive review.
pub fn run_continuous(
    infra: &ExperimentInfra,
    initial_params: ExperimentParams,
    log_path: &Path,
    max_iterations: Option<u64>,
) {
    let reviewer = ReviewState::new(10, 3, 0.1);
    let mut state = LoopState {
        scale: reviewer.base_scale(),
        params: initial_params,
        best_fitness: 0.0,
        best_stability: 0.0,
        reviewer,
    };

    let mut result = run_experiment(infra, &state.params);
    result.iteration = 0;
    result.description = "baseline".into();
    let _ = log_result(log_path, &result);
    state.best_fitness = result.fitness;
    state.best_stability = result.stability;
    state.reviewer.assess(result.fitness);
    log_iteration(0, "baseline", &result);

    let limit = max_iterations.unwrap_or(u64::MAX);
    for i in 1..=limit {
        run_iteration(infra, &mut state, i, log_path);
    }
}

/// One iteration: hill-climb, review, adapt, log.
fn run_iteration(
    infra: &ExperimentInfra,
    state: &mut LoopState,
    i: u64,
    log_path: &Path,
) {
    let (perturbed, mut result, kept) = hill_climb_step(
        infra, &state.params, state.best_fitness,
        state.best_stability, state.scale,
    );
    result.iteration = i;

    let mut action = state.reviewer.assess(result.fitness);
    if matches!(action, Action::Restart) {
        action = maybe_consult_advisor(infra, log_path, action);
    }
    state.scale = apply_action(
        action, &mut state.params, state.scale,
        &state.reviewer, &mut result,
    );

    if kept {
        state.params = perturbed;
        state.best_fitness = result.fitness;
        state.best_stability = result.stability;
        result.description = format!("KEEP: {}", format_params(&state.params));
    } else if result.description.is_empty() {
        result.description = "discard".into();
    }

    log_iteration(i, &result.description, &result);
    let _ = log_result(log_path, &result);
}

fn apply_action(
    action: Action,
    params: &mut ExperimentParams,
    scale: f64,
    reviewer: &ReviewState,
    result: &mut ExperimentResult,
) -> f64 {
    match action {
        Action::Continue => scale,
        Action::WidenSearch(s) => {
            result.description = format!("widen: scale={s:.3}");
            s
        }
        Action::Restart => {
            *params = review::random_params();
            result.description = "RESTART: random params".into();
            reviewer.base_scale()
        }
        Action::SwitchFitness(name) => {
            result.description = format!("SWITCH_FITNESS: {name}");
            reviewer.base_scale()
        }
    }
}

fn maybe_consult_advisor(
    infra: &ExperimentInfra,
    log_path: &Path,
    fallback: Action,
) -> Action {
    let Some(model_id) = &infra.model_id else {
        return fallback;
    };
    let Ok(history) = crate::experiments::log::read_log(log_path) else {
        return fallback;
    };
    let available = &["ImpactAccuracy", "PropagationDepth", "CycleTime", "GraphDensity"];
    advisor::advise(&history, model_id, available)
}

fn log_iteration(i: u64, label: &str, result: &ExperimentResult) {
    eprintln!(
        "[{i}] {label}: fitness={:.4} stability={:.4} ({} ms)",
        result.fitness, result.stability, result.cycle_time_ms,
    );
}

fn perturb(
    params: &ExperimentParams,
    scale: f64,
) -> ExperimentParams {
    let mut rng = rand::rng();
    let mut new = params.clone();
    for (key, value) in new.values.iter_mut() {
        let delta: f64 = rng.random_range(-scale..=scale);
        *value += delta;
        match key.as_str() {
            "confidence_threshold" => *value = value.clamp(0.01, 0.99),
            "attenuation_factor" => *value = value.clamp(0.1, 2.0),
            "max_depth" => *value = value.clamp(1.0, 20.0),
            _ => {}
        }
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

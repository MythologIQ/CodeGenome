use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::experiments::config::ExperimentStatus;
use crate::experiments::log;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentRun {
    pub iteration: u64,
    pub params: BTreeMap<String, f64>,
    pub fitness: f64,
    pub stability: f64,
    pub cycle_time_ms: u64,
    pub status: ExperimentStatus,
    pub description: String,
    pub chain_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DistributionStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub p90: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ParameterCorrelation {
    pub name: String,
    pub correlation: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusCounts {
    pub pass: usize,
    pub fail: usize,
    pub inconclusive: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentReport {
    pub run_count: usize,
    pub convergence_rate: f64,
    pub fitness: DistributionStats,
    pub stability: DistributionStats,
    pub cycle_time_ms: DistributionStats,
    pub parameter_correlations: Vec<ParameterCorrelation>,
    pub status_counts: StatusCounts,
}

pub fn build_report(path: &Path) -> Result<ExperimentReport, String> {
    let runs: Vec<_> = log::read_log(path)?
        .into_iter()
        .map(|r| ExperimentRun {
            iteration: r.iteration,
            params: r.params.into_iter().collect(),
            fitness: r.fitness,
            stability: r.stability,
            cycle_time_ms: r.cycle_time_ms,
            status: r.status,
            description: r.description,
            chain_hash: r.chain_hash,
        })
        .collect();
    let fitness: Vec<_> = runs.iter().map(|r| r.fitness).collect();
    let stability: Vec<_> = runs.iter().map(|r| r.stability).collect();
    let cycle: Vec<_> = runs.iter().map(|r| r.cycle_time_ms as f64).collect();
    Ok(ExperimentReport {
        run_count: runs.len(),
        convergence_rate: convergence_rate(&runs),
        fitness: stats(&fitness),
        stability: stats(&stability),
        cycle_time_ms: stats(&cycle),
        parameter_correlations: parameter_correlations(&runs),
        status_counts: status_counts(&runs),
    })
}

fn convergence_rate(runs: &[ExperimentRun]) -> f64 {
    if runs.is_empty() {
        return 0.0;
    }
    let mut best = f64::NEG_INFINITY;
    let mut improvements = 0;
    for run in runs {
        if run.fitness > best {
            improvements += 1;
            best = run.fitness;
        }
    }
    improvements as f64 / runs.len() as f64
}

fn parameter_correlations(runs: &[ExperimentRun]) -> Vec<ParameterCorrelation> {
    let mut values: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for run in runs {
        for (name, value) in &run.params {
            values
                .entry(name.clone())
                .or_default()
                .push((*value, run.fitness));
        }
    }
    values
        .into_iter()
        .filter_map(|(name, pairs)| {
            pearson(&pairs).map(|correlation| ParameterCorrelation { name, correlation })
        })
        .collect()
}

fn status_counts(runs: &[ExperimentRun]) -> StatusCounts {
    let mut counts = StatusCounts {
        pass: 0,
        fail: 0,
        inconclusive: 0,
    };
    for run in runs {
        match run.status {
            ExperimentStatus::Pass => counts.pass += 1,
            ExperimentStatus::Fail => counts.fail += 1,
            ExperimentStatus::Inconclusive => counts.inconclusive += 1,
        }
    }
    counts
}

fn stats(values: &[f64]) -> DistributionStats {
    if values.is_empty() {
        return DistributionStats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            median: 0.0,
            std_dev: 0.0,
            p90: 0.0,
        };
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
    let variance = sorted.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / sorted.len() as f64;
    DistributionStats {
        min: *sorted.first().unwrap(),
        max: *sorted.last().unwrap(),
        mean,
        median: percentile(&sorted, 0.5),
        std_dev: variance.sqrt(),
        p90: percentile(&sorted, 0.9),
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    let idx = ((sorted.len().saturating_sub(1)) as f64 * p).round() as usize;
    sorted[idx]
}

fn pearson(pairs: &[(f64, f64)]) -> Option<f64> {
    if pairs.len() < 2 {
        return None;
    }
    let (sum_x, sum_y) = pairs
        .iter()
        .fold((0.0, 0.0), |(sx, sy), (x, y)| (sx + x, sy + y));
    let n = pairs.len() as f64;
    let (mx, my) = (sum_x / n, sum_y / n);
    let mut num = 0.0;
    let mut dx = 0.0;
    let mut dy = 0.0;
    for (x, y) in pairs {
        num += (x - mx) * (y - my);
        dx += (x - mx).powi(2);
        dy += (y - my).powi(2);
    }
    let den = dx.sqrt() * dy.sqrt();
    (den > 0.0).then_some(num / den)
}

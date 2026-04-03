use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::experiments::config::ExperimentStatus;

/// Result of a single experiment run.
#[derive(Clone, Debug)]
pub struct ExperimentResult {
    pub iteration: u64,
    pub params: HashMap<String, f64>,
    pub fitness: f64,
    pub stability: f64,
    pub status: ExperimentStatus,
    pub cycle_time_ms: u64,
    pub description: String,
}

/// Append one experiment result to TSV log.
/// Opens, appends, flushes, closes. Crash-safe.
pub fn log_result(
    path: &Path,
    result: &ExperimentResult,
) -> Result<(), String> {
    let needs_header = !path.exists() || file_is_empty(path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    if needs_header {
        writeln!(file, "iteration\tfitness\tstability\tcycle_ms\tstatus\tdescription")
            .map_err(|e| e.to_string())?;
    }

    let status_str = match result.status {
        ExperimentStatus::Pass => "pass",
        ExperimentStatus::Fail => "fail",
        ExperimentStatus::Inconclusive => "inconclusive",
    };

    writeln!(
        file,
        "{}\t{:.6}\t{:.6}\t{}\t{}\t{}",
        result.iteration,
        result.fitness,
        result.stability,
        result.cycle_time_ms,
        status_str,
        result.description,
    )
    .map_err(|e| e.to_string())?;

    file.flush().map_err(|e| e.to_string())
}

/// Read all results from a TSV log.
pub fn read_log(path: &Path) -> Result<Vec<ExperimentResult>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| e.to_string())?;
        if i == 0 || line.trim().is_empty() {
            continue; // skip header
        }
        if let Some(result) = parse_tsv_line(&line) {
            results.push(result);
        }
    }
    Ok(results)
}

fn parse_tsv_line(line: &str) -> Option<ExperimentResult> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 6 { return None; }

    Some(ExperimentResult {
        iteration: parts[0].parse().ok()?,
        fitness: parts[1].parse().ok()?,
        stability: parts[2].parse().ok()?,
        cycle_time_ms: parts[3].parse().ok()?,
        status: match parts[4] {
            "pass" => ExperimentStatus::Pass,
            "fail" => ExperimentStatus::Fail,
            _ => ExperimentStatus::Inconclusive,
        },
        description: parts[5].to_string(),
        params: HashMap::new(),
    })
}

fn file_is_empty(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.len() == 0)
        .unwrap_or(true)
}

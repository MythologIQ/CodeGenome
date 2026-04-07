use std::path::Path;

use codegenome_substrate::experiments::analysis;

pub fn run(log_file: &str, json: bool) {
    match analysis::build_report(Path::new(log_file)) {
        Ok(report) if json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).unwrap_or_default()
            );
        }
        Ok(report) => {
            println!("Runs: {}", report.run_count);
            println!("Convergence rate: {:.3}", report.convergence_rate);
            println!("Fitness mean: {:.3}", report.fitness.mean);
            println!("Fitness p90: {:.3}", report.fitness.p90);
            println!("Cycle mean (ms): {:.3}", report.cycle_time_ms.mean);
            for corr in report.parameter_correlations {
                println!("Correlation {}: {:.3}", corr.name, corr.correlation);
            }
        }
        Err(e) => {
            eprintln!("Analyze failed: {e}");
            std::process::exit(1);
        }
    }
}

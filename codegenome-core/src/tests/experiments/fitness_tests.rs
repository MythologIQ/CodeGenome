use std::path::PathBuf;

use crate::experiments::config::ExperimentParams;
use crate::experiments::fitness;

fn src_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn impact_accuracy_baseline_non_degenerate() {
    let params = ExperimentParams::default();
    let accuracy = fitness::impact_accuracy(&src_dir(), &params);
    eprintln!("Impact accuracy baseline: {accuracy:.4}");
    assert!(accuracy < 1.0, "Accuracy must not be trivially 1.0");
}

#[test]
fn stability_baseline_above_threshold() {
    let params = ExperimentParams::default();
    let stab = fitness::stability(&src_dir(), &params);
    eprintln!("Stability baseline: {stab:.4}");
    assert!(stab > 0.5, "Stability should be > 0.5");
    assert!(stab < 1.0, "Stability must not be trivially 1.0");
}

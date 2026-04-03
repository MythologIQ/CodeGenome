use std::path::PathBuf;

use crate::experiments::config::ExperimentParams;
use crate::experiments::fitness;

fn src_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn impact_accuracy_baseline_non_degenerate() {
    let accuracy = fitness::impact_accuracy(&src_dir());
    eprintln!("Impact accuracy baseline: {accuracy:.4}");
    assert!(accuracy > 0.0, "Accuracy should be non-zero");
}

#[test]
fn stability_baseline_above_threshold() {
    let params = ExperimentParams::default();
    let stab = fitness::stability(&src_dir(), &params);
    eprintln!("Stability baseline: {stab:.4}");
    assert!(stab > 0.5, "Stability should be > 0.5");
}

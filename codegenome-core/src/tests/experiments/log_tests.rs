use crate::experiments::config::ExperimentStatus;
use crate::experiments::log::{self, ExperimentResult};

fn sample_result(iter: u64) -> ExperimentResult {
    ExperimentResult {
        iteration: iter,
        params: std::collections::HashMap::new(),
        fitness: 0.5,
        stability: 0.9,
        status: ExperimentStatus::Pass,
        cycle_time_ms: 100,
        description: format!("test {iter}"),
        chain_hash: String::new(),
    }
}

#[test]
fn chain_hash_is_deterministic() {
    let h1 = log::row_hash("same content", "same_prev");
    let h2 = log::row_hash("same content", "same_prev");
    assert_eq!(h1, h2);
}

#[test]
fn genesis_hash_is_stable() {
    let g1 = log::genesis_hash();
    let g2 = log::genesis_hash();
    assert_eq!(g1, g2);
    assert!(!g1.is_empty());
}

#[test]
fn chain_verifies_on_clean_read() {
    let dir = std::env::temp_dir().join(format!("cg_log_clean_{:?}", std::thread::current().id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("log.tsv");

    let mut prev = log::genesis_hash();
    for i in 0..5 {
        prev = log::log_result(&path, &sample_result(i), &prev).unwrap();
    }

    let results = log::read_log(&path).unwrap();
    assert_eq!(results.len(), 5);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tampered_row_fails_verification() {
    let dir = std::env::temp_dir().join(format!("cg_log_tamper_{:?}", std::thread::current().id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("log.tsv");

    let mut prev = log::genesis_hash();
    for i in 0..3 {
        prev = log::log_result(&path, &sample_result(i), &prev).unwrap();
    }

    // Tamper with the file: change fitness in row 2
    let content = std::fs::read_to_string(&path).unwrap();
    let tampered = content.replacen("0.500000", "0.999999", 1);
    std::fs::write(&path, tampered).unwrap();

    let err = log::read_log(&path).unwrap_err();
    assert!(err.contains("Chain integrity failure"), "Got: {err}");
    let _ = std::fs::remove_dir_all(&dir);
}

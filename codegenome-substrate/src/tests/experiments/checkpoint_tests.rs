use std::collections::HashMap;

use crate::experiments::checkpoint::{self, Checkpoint};

fn sample_checkpoint() -> Checkpoint {
    Checkpoint {
        iteration: 42,
        params: {
            let mut m = HashMap::new();
            m.insert("confidence_threshold".into(), 0.75);
            m.insert("attenuation_factor".into(), 0.85);
            m
        },
        fitness_fn: "ImpactAccuracy".into(),
        best_fitness: 0.35,
        best_stability: 0.92,
        scale: 0.1,
        plateau_count: 3,
        widen_count: 1,
        last_chain_hash: "abc123".into(),
    }
}

#[test]
fn save_load_roundtrip() {
    let dir = std::env::temp_dir().join(format!("cg_cp_rt_{:?}", std::thread::current().id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test.checkpoint.json");

    let cp = sample_checkpoint();
    checkpoint::save(&path, &cp).unwrap();
    let loaded = checkpoint::load(&path).unwrap();

    assert_eq!(loaded.iteration, 42);
    assert!((loaded.best_fitness - 0.35).abs() < 0.001);
    assert_eq!(loaded.fitness_fn, "ImpactAccuracy");
    assert_eq!(loaded.plateau_count, 3);
    assert_eq!(loaded.last_chain_hash, "abc123");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn missing_checkpoint_returns_error() {
    let path = std::env::temp_dir().join("nonexistent.checkpoint.json");
    assert!(checkpoint::load(&path).is_err());
}

#[test]
fn checkpoint_path_derives_from_log() {
    let log = std::path::Path::new("/tmp/experiments.tsv");
    let cp = checkpoint::checkpoint_path(log);
    assert!(cp.to_str().unwrap().contains("checkpoint.json"));
}

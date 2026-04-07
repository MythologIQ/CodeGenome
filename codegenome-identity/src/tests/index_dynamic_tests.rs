use std::io::Write;
use std::path::PathBuf;

use crate::graph::edge::Relation;
use crate::index::dynamic::ingest_trace;

#[test]
fn tsv_trace_produces_calls_edges() {
    let code = b"fn caller() {}\nfn callee() {}";
    let files = vec![(PathBuf::from("test.rs"), code.to_vec())];

    let dir = std::env::temp_dir().join("cg_dyn_test");
    let _ = std::fs::create_dir_all(&dir);
    let trace_path = dir.join("trace.tsv");
    let mut f = std::fs::File::create(&trace_path).unwrap();
    writeln!(f, "caller\tcallee\tcount\tduration_ns").unwrap();
    writeln!(f, "caller\tcallee\t5\t100").unwrap();
    drop(f);

    let result = ingest_trace(&trace_path, &files).unwrap();
    let _ = std::fs::remove_dir_all(&dir);

    let calls: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.relation == Relation::Calls)
        .collect();
    assert_eq!(calls.len(), 1);
    // confidence = min(5/10, 1.0) = 0.5
    assert!(
        (calls[0].confidence - 0.5).abs() < f64::EPSILON,
        "Expected confidence 0.5, got {}",
        calls[0].confidence
    );
}

#[test]
fn high_count_caps_confidence_at_one() {
    let code = b"fn a() {}\nfn b() {}";
    let files = vec![(PathBuf::from("test.rs"), code.to_vec())];

    let dir = std::env::temp_dir().join("cg_dyn_test2");
    let _ = std::fs::create_dir_all(&dir);
    let trace_path = dir.join("trace.tsv");
    let mut f = std::fs::File::create(&trace_path).unwrap();
    writeln!(f, "caller\tcallee\tcount\tduration_ns").unwrap();
    writeln!(f, "a\tb\t100\t500").unwrap();
    drop(f);

    let result = ingest_trace(&trace_path, &files).unwrap();
    let _ = std::fs::remove_dir_all(&dir);

    let calls: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.relation == Relation::Calls)
        .collect();
    assert_eq!(calls.len(), 1);
    assert!(
        (calls[0].confidence - 1.0).abs() < f64::EPSILON,
        "Expected confidence 1.0, got {}",
        calls[0].confidence
    );
}

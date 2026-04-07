use crate::governance::policy::Decision;
use crate::governance::write_gate::{WriteGatePolicy, WriteRequest};
use crate::store::meta::FreshnessReport;

fn fresh_report() -> FreshnessReport {
    FreshnessReport {
        is_fresh: true,
        last_indexed: 1000,
        files_changed: 0,
        files_added: 0,
        files_removed: 0,
    }
}

fn stale_report() -> FreshnessReport {
    FreshnessReport {
        is_fresh: false,
        last_indexed: 500,
        files_changed: 3,
        files_added: 1,
        files_removed: 0,
    }
}

fn valid_request() -> WriteRequest {
    WriteRequest {
        actor: "claude-code".into(),
        toolchain_version: "0.1.0".into(),
        source_freshness: fresh_report(),
        min_edge_confidence: 0.8,
    }
}

#[test]
fn default_policy_allows_valid_request() {
    let policy = WriteGatePolicy::default_policy();
    let decision = policy.evaluate(&valid_request());
    assert_eq!(decision, Decision::Allow);
}

#[test]
fn empty_actor_denied_with_provenance_reason() {
    let policy = WriteGatePolicy::default_policy();
    let mut req = valid_request();
    req.actor = String::new();
    let decision = policy.evaluate(&req);
    match decision {
        Decision::Deny(reason) => {
            assert!(
                reason.contains("provenance"),
                "Reason should mention provenance, got: {reason}"
            );
        }
        other => panic!("Expected Deny, got {other:?}"),
    }
}

#[test]
fn stale_source_denied() {
    let policy = WriteGatePolicy::default_policy();
    let mut req = valid_request();
    req.source_freshness = stale_report();
    let decision = policy.evaluate(&req);
    match decision {
        Decision::Deny(reason) => {
            assert!(
                reason.contains("stale"),
                "Reason should mention stale, got: {reason}"
            );
        }
        other => panic!("Expected Deny, got {other:?}"),
    }
}

#[test]
fn low_confidence_denied_with_values() {
    let policy = WriteGatePolicy::default_policy();
    let mut req = valid_request();
    req.min_edge_confidence = 0.3;
    let decision = policy.evaluate(&req);
    match decision {
        Decision::Deny(reason) => {
            assert!(
                reason.contains("0.30") && reason.contains("0.50"),
                "Reason should contain both values, got: {reason}"
            );
        }
        other => panic!("Expected Deny, got {other:?}"),
    }
}

#[test]
fn above_floor_confidence_allowed() {
    let policy = WriteGatePolicy::default_policy();
    let mut req = valid_request();
    req.min_edge_confidence = 0.8;
    let decision = policy.evaluate(&req);
    assert_eq!(decision, Decision::Allow);
}

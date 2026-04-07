use crate::belief::create::{create_belief, BeliefSpec};
use crate::graph::edge::Relation;
use crate::graph::node::{NodeKind, Source};
use crate::identity::address_of;

fn addr(name: &str) -> crate::identity::UorAddress {
    address_of(name.as_bytes())
}

fn spec(claim: &str) -> BeliefSpec {
    BeliefSpec {
        claim: claim.into(),
        subject: addr("subject_fn"),
        confidence: 0.8,
        supporting_evidence: vec![addr("evidence_1")],
        contradicting_evidence: vec![],
        actor: "test-agent".into(),
    }
}

#[test]
fn creates_belief_node_with_about_and_supports_edges() {
    let (node, edges) = create_belief(&spec("X is dead code"));
    assert_eq!(node.kind, NodeKind::Belief);
    assert_eq!(edges.len(), 2); // AboutSubject + Supports
    assert_eq!(edges[0].relation, Relation::AboutSubject);
    assert_eq!(edges[0].target, addr("subject_fn"));
    assert_eq!(edges[1].relation, Relation::Supports);
    assert_eq!(edges[1].target, addr("evidence_1"));
}

#[test]
fn contradicting_evidence_produces_contradicts_edge() {
    let s = BeliefSpec {
        contradicting_evidence: vec![addr("counter")],
        ..spec("X is reachable")
    };
    let (_, edges) = create_belief(&s);
    let contradicts: Vec<_> = edges
        .iter()
        .filter(|e| e.relation == Relation::Contradicts)
        .collect();
    assert_eq!(contradicts.len(), 1);
    assert_eq!(contradicts[0].target, addr("counter"));
}

#[test]
fn belief_node_has_inferred_source() {
    let (node, _) = create_belief(&spec("test claim"));
    assert_eq!(node.provenance.source, Source::Inferred);
}

#[test]
fn same_claim_produces_same_address() {
    let (a, _) = create_belief(&spec("deterministic"));
    let (b, _) = create_belief(&spec("deterministic"));
    assert_eq!(a.address, b.address);
}

#[test]
fn confidence_propagates_to_node_and_edges() {
    let s = BeliefSpec {
        confidence: 0.65,
        ..spec("low confidence")
    };
    let (node, edges) = create_belief(&s);
    assert!((node.confidence - 0.65).abs() < 0.01);
    for edge in &edges {
        assert!((edge.confidence - 0.65).abs() < 0.01);
    }
}

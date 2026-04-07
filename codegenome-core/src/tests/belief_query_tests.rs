use crate::belief::create::{create_belief, BeliefSpec};
use crate::belief::query;
use crate::graph::node::{Node, NodeKind, Provenance, Timestamp};
use crate::identity::address_of;

fn addr(name: &str) -> crate::identity::UorAddress {
    address_of(name.as_bytes())
}

fn subject_node() -> Node {
    let a = addr("subject_fn");
    Node {
        address: a,
        kind: NodeKind::Symbol,
        provenance: Provenance::tool("test", Timestamp(0)),
        confidence: 1.0,
        created_at: Timestamp(0),
        content_hash: a,
        span: None,
    }
}

fn make_belief(claim: &str, actor: &str) -> (Node, Vec<crate::graph::edge::Edge>) {
    create_belief(&BeliefSpec {
        claim: claim.into(),
        subject: addr("subject_fn"),
        confidence: 0.8,
        supporting_evidence: vec![addr("ev1")],
        contradicting_evidence: vec![addr("counter1")],
        actor: actor.into(),
    })
}

#[test]
fn beliefs_about_finds_linked_beliefs() {
    let (bnode, bedges) = make_belief("X is dead", "agent-a");
    let nodes = vec![subject_node(), bnode];
    let beliefs = query::beliefs_about(addr("subject_fn"), &nodes, &bedges);
    assert_eq!(beliefs.len(), 1);
    assert_eq!(beliefs[0].0.kind, NodeKind::Belief);
}

#[test]
fn beliefs_about_returns_empty_for_unrelated() {
    let (bnode, bedges) = make_belief("X is dead", "agent-a");
    let nodes = vec![subject_node(), bnode];
    let beliefs = query::beliefs_about(addr("other_fn"), &nodes, &bedges);
    assert!(beliefs.is_empty());
}

#[test]
fn beliefs_by_actor_filters_correctly() {
    let (b1, _) = make_belief("claim 1", "agent-a");
    let (b2, _) = make_belief("claim 2", "agent-b");
    let nodes = vec![b1, b2];
    let found = query::beliefs_by_actor("agent-a", &nodes);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].provenance.actor, "agent-a");
}

#[test]
fn supporting_evidence_returns_correct_addresses() {
    let (bnode, bedges) = make_belief("X is dead", "agent");
    let evs = query::supporting_evidence(bnode.address, &bedges);
    assert_eq!(evs.len(), 1);
    assert_eq!(evs[0], addr("ev1"));
}

#[test]
fn contradicting_evidence_returns_correct_addresses() {
    let (bnode, bedges) = make_belief("X is dead", "agent");
    let evs = query::contradicting_evidence(bnode.address, &bedges);
    assert_eq!(evs.len(), 1);
    assert_eq!(evs[0], addr("counter1"));
}

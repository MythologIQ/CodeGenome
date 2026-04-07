use crate::graph::edge::Relation;
use crate::graph::node::Timestamp;
use crate::identity::address_of;
use crate::lineage::query::{ancestors, descendants, lineage_chain};
use crate::lineage::record::{record_mutation, MutationRecord};

fn addr(name: &str) -> crate::identity::UorAddress {
    address_of(name.as_bytes())
}

fn mutation(parent: &str, child: &str) -> MutationRecord {
    MutationRecord {
        parent: addr(parent),
        child: addr(child),
        actor: "test".into(),
        timestamp: Timestamp(0),
        delta_summary: "test mutation".into(),
    }
}

#[test]
fn record_produces_mutated_from_edge() {
    let edge = record_mutation(&mutation("A", "B"));
    assert_eq!(edge.relation, Relation::MutatedFrom);
    assert_eq!(edge.source, addr("B")); // child
    assert_eq!(edge.target, addr("A")); // parent
}

#[test]
fn ancestors_returns_chain_oldest_first() {
    let edges = vec![
        record_mutation(&mutation("A", "B")),
        record_mutation(&mutation("B", "C")),
    ];
    let result = ancestors(addr("C"), &edges);
    assert_eq!(result, vec![addr("A"), addr("B")]);
}

#[test]
fn descendants_returns_chain_newest_last() {
    let edges = vec![
        record_mutation(&mutation("A", "B")),
        record_mutation(&mutation("B", "C")),
    ];
    let result = descendants(addr("A"), &edges);
    assert_eq!(result, vec![addr("B"), addr("C")]);
}

#[test]
fn lineage_chain_returns_full_history() {
    let edges = vec![
        record_mutation(&mutation("A", "B")),
        record_mutation(&mutation("B", "C")),
    ];
    let result = lineage_chain(addr("B"), &edges);
    assert_eq!(result, vec![addr("A"), addr("B"), addr("C")]);
}

#[test]
fn no_mutations_returns_empty() {
    let edges = vec![];
    assert!(ancestors(addr("X"), &edges).is_empty());
    assert!(descendants(addr("X"), &edges).is_empty());
}

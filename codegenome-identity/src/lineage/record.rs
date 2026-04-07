use crate::graph::edge::{Edge, Relation};
use crate::graph::node::{Provenance, Source, Timestamp};
use crate::identity::UorAddress;

/// A mutation event: parent artifact was transformed into child.
pub struct MutationRecord {
    pub parent: UorAddress,
    pub child: UorAddress,
    pub actor: String,
    pub timestamp: Timestamp,
    pub delta_summary: String,
}

/// Record a mutation as a graph edge.
/// Returns a MutatedFrom edge: child → parent.
/// Pure value — caller decides persistence.
pub fn record_mutation(record: &MutationRecord) -> Edge {
    Edge {
        source: record.child,
        target: record.parent,
        relation: Relation::MutatedFrom,
        confidence: 1.0,
        provenance: Provenance {
            source: Source::ToolOutput,
            actor: record.actor.clone(),
            timestamp: record.timestamp,
            justification: None,
        },
        evidence: vec![record.parent, record.child],
    }
}

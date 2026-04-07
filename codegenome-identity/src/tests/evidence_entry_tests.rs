use crate::evidence::entry::*;
use crate::graph::node::Timestamp;
use crate::identity::address_of;

fn sample_entry() -> TypedEvidenceEntry {
    let content = address_of(b"test-content");
    let prev = address_of(b"previous");
    TypedEvidenceEntry {
        entry_id: 1,
        timestamp: Timestamp(1000),
        operation: TypedOperation::Implement,
        actor: "test-agent".into(),
        target: "plan-test.md".into(),
        content_hash: content,
        previous_hash: prev,
        chain_hash: compute_chain_hash(content, prev),
    }
}

#[test]
fn serialize_deserialize_round_trip() {
    let entry = sample_entry();
    let json = to_json(&entry);
    let restored = from_json(&json).unwrap();
    assert_eq!(restored.entry_id, entry.entry_id);
    assert_eq!(restored.operation, entry.operation);
    assert_eq!(restored.actor, entry.actor);
    assert_eq!(restored.content_hash, entry.content_hash);
    assert_eq!(restored.chain_hash, entry.chain_hash);
}

#[test]
fn chain_hash_is_deterministic() {
    let a = address_of(b"content");
    let b = address_of(b"prev");
    let h1 = compute_chain_hash(a, b);
    let h2 = compute_chain_hash(a, b);
    assert_eq!(h1, h2);
}

#[test]
fn chain_hash_changes_with_different_inputs() {
    let a = address_of(b"content-1");
    let b = address_of(b"content-2");
    let prev = address_of(b"prev");
    let h1 = compute_chain_hash(a, prev);
    let h2 = compute_chain_hash(b, prev);
    assert_ne!(h1, h2);
}

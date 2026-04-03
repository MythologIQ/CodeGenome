use crate::graph::node::{Node, NodeKind, Provenance, Span};
use crate::graph::edge::{Edge, Relation};
use crate::identity::{address_of, UorAddress};

/// Node kinds recognized from tree-sitter Rust grammar.
const SYMBOL_KINDS: &[&str] = &[
    "function_item",
    "struct_item",
    "enum_item",
    "impl_item",
    "use_declaration",
    "mod_item",
    "trait_item",
];

/// Extracts graph Nodes and Contains edges from a parsed
/// tree-sitter tree. Pure function — no side effects.
pub fn extract_symbols(
    file_address: UorAddress,
    source: &[u8],
    tree: &tree_sitter::Tree,
    provenance: &Provenance,
) -> (Vec<Node>, Vec<Edge>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if !SYMBOL_KINDS.contains(&child.kind()) {
            continue;
        }
        let name = symbol_name(&child, source);
        let span = node_span(&child);
        let content = format!("{}:{}", child.kind(), name);
        let address = address_of(content.as_bytes());

        nodes.push(Node {
            address,
            kind: NodeKind::Symbol,
            provenance: provenance.clone(),
            confidence: 1.0,
            created_at: provenance.timestamp,
            content_hash: address_of(
                &source[child.start_byte()..child.end_byte()],
            ),
            span: Some(span),
        });

        edges.push(Edge {
            source: file_address,
            target: address,
            relation: Relation::Contains,
            confidence: 1.0,
            provenance: provenance.clone(),
            evidence: vec![],
        });
    }

    (nodes, edges)
}

fn symbol_name(node: &tree_sitter::Node, source: &[u8]) -> String {
    node.child_by_field_name("name")
        .and_then(|n| n.utf8_text(source).ok())
        .unwrap_or(node.kind())
        .to_string()
}

fn node_span(node: &tree_sitter::Node) -> Span {
    Span {
        start_byte: node.start_byte() as u32,
        end_byte: node.end_byte() as u32,
        start_line: node.start_position().row as u32 + 1,
        end_line: node.end_position().row as u32 + 1,
    }
}

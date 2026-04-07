use crate::graph::edge::Relation;
use crate::graph::node::{NodeKind, Span};
use crate::lang::graph_builder::build_file_graph;
use crate::lang::ir::*;

fn span(line: u32, start: u32, end: u32) -> Span {
    Span {
        start_byte: start,
        end_byte: end,
        start_line: line,
        end_line: line,
    }
}

#[test]
fn graph_builder_produces_nodes_and_contains_edges() {
    let source = b"fn alpha() {}\nfn beta() {}\n";
    let symbols = vec![
        SymbolDef {
            name: "alpha".into(),
            kind: SymbolKind::Function,
            span: span(1, 0, 13),
            source_kind: "function_item".into(),
        },
        SymbolDef {
            name: "beta".into(),
            kind: SymbolKind::Function,
            span: span(2, 14, 26),
            source_kind: "function_item".into(),
        },
    ];

    let (nodes, edges) = build_file_graph(
        std::path::Path::new("test.rs"),
        source,
        "rust",
        &symbols,
        &[],
        &[],
        &[],
    );

    // 1 File node + 2 Symbol nodes
    assert_eq!(nodes.len(), 3);
    let file_nodes: Vec<_> = nodes
        .iter()
        .filter(|n| n.kind == NodeKind::File)
        .collect();
    assert_eq!(file_nodes.len(), 1);

    // 2 Contains edges
    let contains: Vec<_> = edges
        .iter()
        .filter(|e| e.relation == Relation::Contains)
        .collect();
    assert_eq!(contains.len(), 2);
}

#[test]
fn graph_builder_produces_calls_edge() {
    let source = b"fn caller() { callee(); }\nfn callee() {}";
    let symbols = vec![
        SymbolDef {
            name: "caller".into(),
            kind: SymbolKind::Function,
            span: span(1, 0, 25),
            source_kind: "function_item".into(),
        },
        SymbolDef {
            name: "callee".into(),
            kind: SymbolKind::Function,
            span: span(2, 26, 40),
            source_kind: "function_item".into(),
        },
    ];
    let calls = vec![CallRef {
        caller_span: span(1, 0, 25),
        callee_name: "callee".into(),
        span: span(1, 14, 23),
    }];

    let (_, edges) = build_file_graph(
        std::path::Path::new("test.rs"),
        source,
        "rust",
        &symbols,
        &[],
        &calls,
        &[],
    );

    let call_edges: Vec<_> = edges
        .iter()
        .filter(|e| e.relation == Relation::Calls)
        .collect();
    assert_eq!(call_edges.len(), 1);
    assert!((call_edges[0].confidence - 0.7).abs() < f64::EPSILON);
}

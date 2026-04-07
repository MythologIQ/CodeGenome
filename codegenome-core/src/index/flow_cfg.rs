use crate::graph::node::Span;

/// A control flow edge between two statement spans.
#[derive(Clone, Debug)]
pub struct CfgEdge {
    pub source_span: Span,
    pub target_span: Span,
    pub kind: CfgKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CfgKind {
    Sequential,
    Branch,
    BackEdge,
    Return,
}

/// A data flow edge from definition to use.
#[derive(Clone, Debug)]
pub struct DfgEdge {
    pub def_span: Span,
    pub use_span: Span,
    pub var_name: String,
}

/// Extract control flow edges within function bodies.
pub fn extract_control_flow(
    source: &[u8],
    tree: &tree_sitter::Tree,
) -> Vec<CfgEdge> {
    let mut edges = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() == "function_item" {
            if let Some(body) = child.child_by_field_name("body") {
                extract_block_flow(&body, source, &mut edges);
            }
        }
    }
    edges
}

fn extract_block_flow(
    block: &tree_sitter::Node,
    source: &[u8],
    edges: &mut Vec<CfgEdge>,
) {
    let stmts: Vec<_> = block_statements(block);
    for pair in stmts.windows(2) {
        edges.push(CfgEdge {
            source_span: node_span(&pair[0]),
            target_span: node_span(&pair[1]),
            kind: CfgKind::Sequential,
        });
    }
    for stmt in &stmts {
        extract_control_structure(stmt, source, edges);
    }
}

fn extract_control_structure(
    node: &tree_sitter::Node,
    source: &[u8],
    edges: &mut Vec<CfgEdge>,
) {
    match node.kind() {
        "if_expression" => extract_if_flow(node, source, edges),
        "match_expression" => extract_match_flow(node, edges),
        "loop_expression" | "while_expression"
        | "for_expression" => {
            extract_loop_flow(node, source, edges);
        }
        "return_expression" => extract_return_flow(node, edges),
        "expression_statement" => {
            if let Some(inner) = node.child(0) {
                extract_control_structure(&inner, source, edges);
            }
        }
        _ => {}
    }
}

fn extract_if_flow(
    node: &tree_sitter::Node,
    source: &[u8],
    edges: &mut Vec<CfgEdge>,
) {
    let if_span = node_span(node);
    if let Some(consequence) =
        node.child_by_field_name("consequence")
    {
        edges.push(CfgEdge {
            source_span: if_span,
            target_span: node_span(&consequence),
            kind: CfgKind::Branch,
        });
        extract_block_flow(&consequence, source, edges);
    }
    if let Some(alt) = node.child_by_field_name("alternative") {
        edges.push(CfgEdge {
            source_span: if_span,
            target_span: node_span(&alt),
            kind: CfgKind::Branch,
        });
        if alt.kind() == "else_clause" {
            let mut c = alt.walk();
            for child in alt.children(&mut c) {
                if child.kind() == "block" {
                    extract_block_flow(&child, source, edges);
                } else if child.kind() == "if_expression" {
                    extract_if_flow(&child, source, edges);
                }
            }
        }
    }
}

fn extract_match_flow(
    node: &tree_sitter::Node,
    edges: &mut Vec<CfgEdge>,
) {
    let match_span = node_span(node);
    if let Some(body) = node.child_by_field_name("body") {
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            if child.kind() == "match_arm" {
                edges.push(CfgEdge {
                    source_span: match_span,
                    target_span: node_span(&child),
                    kind: CfgKind::Branch,
                });
            }
        }
    }
}

fn extract_loop_flow(
    node: &tree_sitter::Node,
    source: &[u8],
    edges: &mut Vec<CfgEdge>,
) {
    let loop_span = node_span(node);
    if let Some(body) = node.child_by_field_name("body") {
        edges.push(CfgEdge {
            source_span: node_span(&body),
            target_span: loop_span,
            kind: CfgKind::BackEdge,
        });
        extract_block_flow(&body, source, edges);
    }
}

fn extract_return_flow(
    node: &tree_sitter::Node,
    edges: &mut Vec<CfgEdge>,
) {
    edges.push(CfgEdge {
        source_span: node_span(node),
        target_span: node_span(node),
        kind: CfgKind::Return,
    });
}

fn block_statements<'a>(
    block: &'a tree_sitter::Node<'a>,
) -> Vec<tree_sitter::Node<'a>> {
    let mut stmts = Vec::new();
    let mut cursor = block.walk();
    for child in block.children(&mut cursor) {
        match child.kind() {
            "{" | "}" => continue,
            _ => stmts.push(child),
        }
    }
    stmts
}

pub fn node_span(node: &tree_sitter::Node) -> Span {
    Span {
        start_byte: node.start_byte() as u32,
        end_byte: node.end_byte() as u32,
        start_line: node.start_position().row as u32 + 1,
        end_line: node.end_position().row as u32 + 1,
    }
}

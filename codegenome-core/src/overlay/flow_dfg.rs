use crate::graph::node::Span;
use crate::overlay::flow_extract::DfgEdge;

/// Extract data flow edges (let def → identifier use) within functions.
pub fn extract_data_flow(
    source: &[u8],
    tree: &tree_sitter::Tree,
) -> Vec<DfgEdge> {
    let mut edges = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() == "function_item" {
            if let Some(body) = child.child_by_field_name("body") {
                extract_fn_data_flow(&body, source, &mut edges);
            }
        }
    }
    edges
}

/// Collect let bindings and identifier uses within a function body.
fn extract_fn_data_flow(
    body: &tree_sitter::Node,
    source: &[u8],
    edges: &mut Vec<DfgEdge>,
) {
    let mut defs: Vec<(String, Span)> = Vec::new();
    collect_let_defs(body, source, &mut defs);

    let mut uses: Vec<(String, Span)> = Vec::new();
    collect_ident_uses(body, source, &mut uses);

    for (use_name, use_span) in &uses {
        for (def_name, def_span) in &defs {
            if use_name == def_name && use_span.start_byte > def_span.end_byte {
                edges.push(DfgEdge {
                    def_span: *def_span,
                    use_span: *use_span,
                    var_name: use_name.clone(),
                });
                break;
            }
        }
    }
}

fn collect_let_defs(
    node: &tree_sitter::Node,
    source: &[u8],
    defs: &mut Vec<(String, Span)>,
) {
    if node.kind() == "let_declaration" {
        if let Some(pat) = node.child_by_field_name("pattern") {
            if let Ok(name) = pat.utf8_text(source) {
                defs.push((name.to_string(), node_span(&pat)));
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_let_defs(&child, source, defs);
    }
}

fn collect_ident_uses(
    node: &tree_sitter::Node,
    source: &[u8],
    uses: &mut Vec<(String, Span)>,
) {
    if node.kind() == "identifier" && !is_definition_site(node) {
        if let Ok(name) = node.utf8_text(source) {
            uses.push((name.to_string(), node_span(node)));
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_ident_uses(&child, source, uses);
    }
}

fn is_definition_site(node: &tree_sitter::Node) -> bool {
    node.parent()
        .map(|p| {
            p.kind() == "let_declaration"
                && p.child_by_field_name("pattern") == Some(*node)
        })
        .unwrap_or(false)
}

fn node_span(node: &tree_sitter::Node) -> Span {
    Span {
        start_byte: node.start_byte() as u32,
        end_byte: node.end_byte() as u32,
        start_line: node.start_position().row as u32 + 1,
        end_line: node.end_position().row as u32 + 1,
    }
}

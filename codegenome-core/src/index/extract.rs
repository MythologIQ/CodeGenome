use crate::graph::node::Span;

/// A `use` declaration target: the final name segment imported.
pub struct UseTarget {
    pub name: String,
    pub span: Span,
}

/// A function call site within a function body.
pub struct CallSite {
    pub caller_span: Span,
    pub callee_name: String,
    pub span: Span,
}

/// An `impl` block target: type name and optional trait.
pub struct ImplTarget {
    pub type_name: String,
    pub trait_name: Option<String>,
    pub span: Span,
}

/// Extract the final path segment from each `use_declaration`.
pub fn extract_use_targets(
    source: &[u8],
    tree: &tree_sitter::Tree,
) -> Vec<UseTarget> {
    let mut targets = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() != "use_declaration" {
            continue;
        }
        if let Some(name) = use_leaf_name(&child, source) {
            targets.push(UseTarget {
                name,
                span: node_span(&child),
            });
        }
    }
    targets
}

/// Extract call sites from function bodies.
pub fn extract_call_sites(
    source: &[u8],
    tree: &tree_sitter::Tree,
) -> Vec<CallSite> {
    let mut sites = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() != "function_item" {
            continue;
        }
        let fn_span = node_span(&child);
        collect_calls(&child, source, fn_span, &mut sites);
    }
    sites
}

/// Extract impl block targets.
pub fn extract_impl_targets(
    source: &[u8],
    tree: &tree_sitter::Tree,
) -> Vec<ImplTarget> {
    let mut targets = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() != "impl_item" {
            continue;
        }
        if let Some(target) = parse_impl_item(&child, source) {
            targets.push(target);
        }
    }
    targets
}

/// Walk a use_declaration to find the leaf identifier name.
fn use_leaf_name(
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<String> {
    let mut best: Option<String> = None;
    walk_for_ident(node, source, &mut best);
    best
}

fn walk_for_ident(
    node: &tree_sitter::Node,
    source: &[u8],
    best: &mut Option<String>,
) {
    if node.kind() == "identifier" || node.kind() == "type_identifier" {
        if let Ok(text) = node.utf8_text(source) {
            *best = Some(text.to_string());
        }
    }
    let mut child_cursor = node.walk();
    for child in node.children(&mut child_cursor) {
        walk_for_ident(&child, source, best);
    }
}

/// Recursively collect call_expression nodes within a function.
fn collect_calls(
    node: &tree_sitter::Node,
    source: &[u8],
    fn_span: Span,
    sites: &mut Vec<CallSite>,
) {
    if node.kind() == "call_expression" {
        if let Some(callee) = call_callee_name(node, source) {
            sites.push(CallSite {
                caller_span: fn_span,
                callee_name: callee,
                span: node_span(node),
            });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_calls(&child, source, fn_span, sites);
    }
}

/// Extract the callee name from a call_expression.
fn call_callee_name(
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<String> {
    let func = node.child_by_field_name("function")?;
    match func.kind() {
        "identifier" => func.utf8_text(source).ok().map(String::from),
        "scoped_identifier" | "field_expression" => {
            last_identifier(&func, source)
        }
        _ => func.utf8_text(source).ok().map(String::from),
    }
}

/// Find the last identifier child in a scoped path.
fn last_identifier(
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<String> {
    let mut cursor = node.walk();
    let mut last = None;
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier"
            || child.kind() == "type_identifier"
        {
            last = child.utf8_text(source).ok().map(String::from);
        }
    }
    last
}

/// Parse an impl_item into type name and optional trait.
fn parse_impl_item(
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<ImplTarget> {
    let type_node = node.child_by_field_name("type")?;
    let type_name = type_text(&type_node, source)?;

    let trait_name = node
        .child_by_field_name("trait")
        .and_then(|t| type_text(&t, source));

    Some(ImplTarget {
        type_name,
        trait_name,
        span: node_span(node),
    })
}

fn type_text(
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<String> {
    match node.kind() {
        "type_identifier" | "identifier" => {
            node.utf8_text(source).ok().map(String::from)
        }
        _ => last_identifier(node, source)
            .or_else(|| node.utf8_text(source).ok().map(String::from)),
    }
}

pub fn node_span(node: &tree_sitter::Node) -> Span {
    Span {
        start_byte: node.start_byte() as u32,
        end_byte: node.end_byte() as u32,
        start_line: node.start_position().row as u32 + 1,
        end_line: node.end_position().row as u32 + 1,
    }
}

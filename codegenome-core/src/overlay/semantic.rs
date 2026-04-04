use std::collections::HashMap;
use std::path::PathBuf;

use crate::graph::edge::{Edge, Relation};
use crate::graph::node::{Node, Provenance, Source, Timestamp};
use crate::graph::overlay::{Overlay, OverlayKind};
use crate::identity::{address_of, UorAddress};
use crate::measurement::GroundTruthLevel;
use crate::overlay::semantic_extract::*;
use crate::overlay::syntax::SyntaxOverlay;

pub struct SemanticOverlay {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Overlay for SemanticOverlay {
    fn kind(&self) -> OverlayKind {
        OverlayKind::Semantic
    }
    fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    fn edges(&self) -> &[Edge] {
        &self.edges
    }
    fn ground_truth(&self) -> GroundTruthLevel {
        GroundTruthLevel::Constructible
    }
}

impl SemanticOverlay {
    /// Build semantic edges by resolving AST references against
    /// the syntax overlay's symbol table. Pure function.
    pub fn from_syntax(
        syntax: &SyntaxOverlay,
        files: &[(PathBuf, Vec<u8>)],
    ) -> Self {
        let provenance = Provenance {
            source: Source::Inferred,
            actor: "heuristic-resolver".into(),
            timestamp: Timestamp(0),
            justification: None,
        };
        let symbol_table = build_symbol_table(files);
        let span_index = build_span_index(syntax);
        let mut edges = Vec::new();

        for (path, source) in files {
            let Some(tree) = parse_file(source) else {
                continue;
            };
            let file_addr = file_address(path);
            resolve_uses(&tree, source, file_addr, &symbol_table, &provenance, &mut edges);
            resolve_calls(&tree, source, &symbol_table, &span_index, &provenance, &mut edges);
            resolve_impls(&tree, source, &symbol_table, &provenance, &mut edges);
        }

        Self { nodes: Vec::new(), edges }
    }
}

type SymbolTable = HashMap<String, UorAddress>;
type SpanIndex = HashMap<(u32, u32), UorAddress>;

fn build_symbol_table(files: &[(PathBuf, Vec<u8>)]) -> SymbolTable {
    let mut table = HashMap::new();
    for (_, source) in files {
        let Some(tree) = parse_file(source) else { continue };
        let root = tree.root_node();
        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            if let Some(name) = symbol_node_name(&child, source) {
                let content = format!("{}:{}", child.kind(), name);
                table.insert(name, address_of(content.as_bytes()));
            }
        }
    }
    table
}

fn build_span_index(syntax: &SyntaxOverlay) -> SpanIndex {
    let mut index = HashMap::new();
    for node in syntax.nodes() {
        if let Some(span) = &node.span {
            index.insert((span.start_line, span.end_line), node.address);
        }
    }
    index
}

fn resolve_uses(
    tree: &tree_sitter::Tree,
    source: &[u8],
    file_addr: UorAddress,
    symbols: &SymbolTable,
    prov: &Provenance,
    edges: &mut Vec<Edge>,
) {
    for target in extract_use_targets(source, tree) {
        if let Some(&addr) = symbols.get(&target.name) {
            edges.push(Edge {
                source: file_addr,
                target: addr,
                relation: Relation::Imports,
                confidence: 0.8,
                provenance: prov.clone(),
                evidence: vec![],
            });
        }
    }
}

fn resolve_calls(
    tree: &tree_sitter::Tree,
    source: &[u8],
    symbols: &SymbolTable,
    spans: &SpanIndex,
    prov: &Provenance,
    edges: &mut Vec<Edge>,
) {
    for site in extract_call_sites(source, tree) {
        let Some(&callee) = symbols.get(&site.callee_name) else {
            continue;
        };
        let caller = find_enclosing_symbol(
            &site.caller_span, spans,
        );
        let Some(caller_addr) = caller else { continue };
        edges.push(Edge {
            source: caller_addr,
            target: callee,
            relation: Relation::Calls,
            confidence: 0.7,
            provenance: prov.clone(),
            evidence: vec![],
        });
    }
}

fn resolve_impls(
    tree: &tree_sitter::Tree,
    source: &[u8],
    symbols: &SymbolTable,
    prov: &Provenance,
    edges: &mut Vec<Edge>,
) {
    for target in extract_impl_targets(source, tree) {
        let trait_name = match &target.trait_name {
            Some(n) => n,
            None => continue,
        };
        let Some(&type_addr) = symbols.get(&target.type_name) else {
            continue;
        };
        let Some(&trait_addr) = symbols.get(trait_name) else {
            continue;
        };
        edges.push(Edge {
            source: type_addr,
            target: trait_addr,
            relation: Relation::Implements,
            confidence: 0.8,
            provenance: prov.clone(),
            evidence: vec![],
        });
    }
}

fn find_enclosing_symbol(
    fn_span: &crate::graph::node::Span,
    spans: &SpanIndex,
) -> Option<UorAddress> {
    spans
        .get(&(fn_span.start_line, fn_span.end_line))
        .copied()
}

const SYMBOL_KINDS: &[&str] = &[
    "function_item", "struct_item", "enum_item",
    "impl_item", "mod_item", "trait_item",
];

fn symbol_node_name(
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<String> {
    if !SYMBOL_KINDS.contains(&node.kind()) {
        return None;
    }
    node.child_by_field_name("name")
        .and_then(|n| n.utf8_text(source).ok())
        .map(String::from)
}

fn parse_file(source: &[u8]) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .ok()?;
    parser.parse(source, None)
}

fn file_address(path: &std::path::Path) -> UorAddress {
    let content = format!("file:{}", path.display());
    address_of(content.as_bytes())
}

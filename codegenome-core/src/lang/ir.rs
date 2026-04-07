use crate::graph::node::Span;

/// A symbol definition: function, class, struct, trait, enum, module.
#[derive(Clone, Debug)]
pub struct SymbolDef {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    /// Original AST node kind for provenance (e.g. "function_item").
    pub source_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Class,
    Trait,
    Enum,
    Module,
    Other(String),
}

/// An import reference.
#[derive(Clone, Debug)]
pub struct ImportRef {
    pub imported_name: String,
    pub span: Span,
}

/// A function call reference.
#[derive(Clone, Debug)]
pub struct CallRef {
    pub caller_span: Span,
    pub callee_name: String,
    pub span: Span,
}

/// An implementation/extension reference.
#[derive(Clone, Debug)]
pub struct ImplRef {
    pub type_name: String,
    pub trait_name: Option<String>,
    pub span: Span,
}

/// A control flow edge (language-neutral).
#[derive(Clone, Debug)]
pub struct CfEdge {
    pub source_span: Span,
    pub target_span: Span,
    pub kind: CfKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CfKind {
    Sequential,
    Branch,
    BackEdge,
    Return,
}

/// A data flow edge (language-neutral).
#[derive(Clone, Debug)]
pub struct DfEdge {
    pub def_span: Span,
    pub use_span: Span,
    pub var_name: String,
}

/// Helper: convert a tree-sitter node to a Span.
pub fn node_span(node: &tree_sitter::Node) -> Span {
    Span {
        start_byte: node.start_byte() as u32,
        end_byte: node.end_byte() as u32,
        start_line: node.start_position().row as u32 + 1,
        end_line: node.end_position().row as u32 + 1,
    }
}

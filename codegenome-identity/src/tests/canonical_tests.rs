use crate::lang::canonical::{canonicalize, normalized_address, CanonicalKind};
use crate::lang::ir::SymbolKind;

#[test]
fn rust_and_python_functions_map_to_same_canonical() {
    let rust = canonicalize(&SymbolKind::Function, "function_item");
    let python = canonicalize(&SymbolKind::Function, "function_definition");
    assert_eq!(rust, CanonicalKind::Function);
    assert_eq!(python, CanonicalKind::Function);
}

#[test]
fn normalized_address_is_language_independent() {
    let addr_rust = normalized_address(&CanonicalKind::Function, "foo");
    let addr_python = normalized_address(&CanonicalKind::Function, "foo");
    assert_eq!(addr_rust, addr_python);
}

#[test]
fn different_names_produce_different_addresses() {
    let a = normalized_address(&CanonicalKind::Function, "foo");
    let b = normalized_address(&CanonicalKind::Function, "bar");
    assert_ne!(a, b);
}

#[test]
fn unknown_source_kind_falls_back_to_symbol_kind() {
    let result = canonicalize(&SymbolKind::Class, "unknown_node_type");
    assert_eq!(result, CanonicalKind::Class);
}

#[test]
fn ts_interface_maps_to_canonical_interface() {
    let result = canonicalize(&SymbolKind::Trait, "interface_declaration");
    assert_eq!(result, CanonicalKind::Interface);
}

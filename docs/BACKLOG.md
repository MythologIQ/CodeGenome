# Project Backlog

## Blockers (Must Fix Before Progress)

### Security Blockers
- [x] [S1] Define cryptographic identity scheme: BLAKE3 vs SHA-256 vs dual-hash for UOR addresses (v0.1.0 - Complete: BLAKE3 selected)
- [ ] [S2] Define capability broker integration points with FailSafe-Pro
- [ ] [S3] Establish Merkle ledger entry format for code graph evidence
- [ ] [S4] Secret scanning pre-commit hook + CI configuration

### Development Blockers
- [x] [D1] Workspace Cargo.toml with core crate configured (v0.1.0 - Complete: Phase 1 scope)
- [x] [D2] Tree-sitter grammar loading strategy (v0.2.0 - Complete: bundled via tree-sitter-rust crate)
- [x] [D3] On-disk graph store format specification (v0.2.0 - Complete: bincode, one dir per overlay)

## Backlog (Planned Work)

### Phase 1: Core Identity + Graph Engine
- [x] [B1] Implement UOR addressing (v0.1.0 - Complete)
- [x] [B2] Implement property graph engine (v0.1.0 - Complete: node, edge, overlay trait)
- [x] [B3] Implement on-disk graph store (v0.2.0 - Complete: StoreBackend trait + OnDiskStore)
- [x] [B4] Tree-sitter CST/AST extraction for Rust (v0.2.0 - Complete: SyntaxOverlay)

### Phase 2: Symbol Resolution + Diff Mapping
- [x] [B5] Heuristic symbol resolver (import/call/inherit edges from AST) (v0.5.0 - Complete: SemanticOverlay)
- [x] [B6] Git diff-to-symbol mapper (v0.2.0 - Complete: detect_changes via span intersection)
- [x] [B7] Impact propagation (symbol → affected processes via graph traversal) (v0.19.0 - Complete: detect_changes blast radius)
- [x] [B8] Confidence scoring and multi-resolver fusion (v0.11.0 - Complete: FusedOverlay with noisy-OR)

### Phase 3: Flow Overlays
- [x] [B9] CFG construction from AST (v0.5.0 - Complete: FlowOverlay intraprocedural CFG)
- [x] [B10] DFG overlay builder (v0.5.0 - Complete: FlowOverlay intraprocedural DFG)
- [x] [B11] Process tracer (entrypoint detection + flow tracing) (v0.13.0 - Complete: ProcessOverlay)

### Phase 4: Governance Integration
- [x] [B12] Evidence bundle compiler for code graph operations (v0.13.0 - Complete: evidence log with BLAKE3 chain)
- [x] [B13] Merkle ledger integration (write entries for index/query operations) (v0.15.0 - Complete: governance/ledger.rs)
- [x] [B14] Policy evaluation bridge (allow/deny/require-approval) (v0.15.0 - Complete: governance/policy.rs + governance.toml)
- [ ] [B15] Capability broker integration

### Phase 5: Developer UX
- [x] [B16] MCP tool server (context, impact, detect_changes, trace) (v0.14.0 - Complete: codegenome-mcp crate)
- [x] [B17] CLI binary (index, query, status, verify) (v0.12.0 - Complete)
- [x] [B18] Index freshness / staleness detection (v0.15.0 - Complete: store/meta.rs)

### Phase 6: Advanced Overlays
- [x] [B19] LSP integration for higher-fidelity semantic resolution (v0.16.0 - Complete: overlay/lsp.rs stub + graceful degradation)
- [x] [B20] SCIP ingestion as portable precise index backbone (v0.16.0 - Complete: overlay/scip.rs JSON+protobuf)
- [x] [B21] Runtime trace ingestion (dynamic overlay) (v0.16.0 - Complete: overlay/runtime.rs TSV traces)
- [x] [B22] PDG (Program Dependence Graph) overlay (v0.16.0 - Complete: overlay/pdg.rs)

## Wishlist (Nice to Have)
- [ ] [W1] IDE panels + code lenses integration
- [x] [W2] Cross-repository graph federation (v0.18.0 - Complete: explicit workspace federation overlay + CLI/MCP surfaces)
- [x] [W3] Embedding-based similarity search (as observer frame, not identity) (v0.20.0 - Complete: embedding/store + embedding/similarity)
- [ ] [W4] SLSA supply-chain attestation integration
- [ ] [W5] Sigstore signing for graph artifacts
- [x] [W6] Community/module detection algorithms (v0.20.0 - Complete: graph/community.rs filtered connected components)
- [x] [W7] Visualization dashboard for code graph exploration (v0.20.0 - Complete: codegenome visualize + Cytoscape.js)

---
_Updated by /qor-* commands automatically_

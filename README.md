<p align="center">
  <strong>CODEGENOME</strong><br>
  <em>Unified Code Reality Graph</em>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.88%2B-orange?logo=rust&logoColor=white" alt="Rust 1.88+"></a>
  <a href="https://github.com/MythologIQ/CodeGenome/actions"><img src="https://img.shields.io/badge/tests-125%20passing-brightgreen" alt="Tests: 125 passing"></a>
  <a href="https://github.com/MythologIQ/CodeGenome"><img src="https://img.shields.io/badge/clippy-clean-blue" alt="Clippy: clean"></a>
  <a href="https://github.com/MythologIQ/CodeGenome"><img src="https://img.shields.io/badge/overlays-9-blueviolet" alt="Overlays: 9"></a>
  <a href="https://github.com/MythologIQ/CodeGenome"><img src="https://img.shields.io/badge/status-research%20prototype-yellow" alt="Status: research prototype"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License: MIT"></a>
</p>

---

CODEGENOME is a content-addressed, multi-layer program analysis graph that merges syntax, semantics, control/data flow, process traces, and runtime data into a single queryable substrate. Every node has a cryptographic identity (BLAKE3). Every edge carries a confidence score. Independent evidence sources fuse via noisy-OR. The system hill-climbs on its own accuracy using an adaptive, self-evolving experiment engine.

> **Research prototype.** Not production-ready. APIs will change.

## Problem

Existing code intelligence tools operate on a single representation: syntax (tree-sitter), semantics (LSP), or flow (compiler IR). No tool unifies all layers into a single graph with confidence-scored edges, content-addressed identity, evidence fusion, and self-evaluating accuracy.

## Architecture

```
Source Files
    |
    v
[Syntax Overlay]      tree-sitter AST -> Contains edges (confidence: 1.0)
[Semantic Overlay]    Heuristic name resolution -> Calls, Imports, Implements (0.7-0.8)
[Flow Overlay]        Intraprocedural CFG/DFG -> ControlFlow, DataFlow (1.0)
[Process Overlay]     Entrypoint detection -> PartOfProcess edges (0.9^depth)
[PDG Overlay]         CFG+DFG composition -> ControlDependence (1.0)
[Runtime Overlay]     TSV trace ingestion -> weighted Calls (count/10)
[SCIP Overlay]        Compiler index -> References, Implements (1.0)
[LSP Overlay]         rust-analyzer -> References (1.0)
    |
    v
[Confidence Fusion]   Noisy-OR merge: two 0.7 edges -> one 0.94 edge
    |
    v
[Signal Propagation]  Impact forward, staleness backward
    |
    v
[Experiment Engine]   Adaptive hill-climbing with LLM advisor
```

### Workspace

| Crate | Purpose |
|-------|---------|
| `codegenome-core` | Graph types, 9 overlays, fusion, signal, experiments, governance, evidence |
| `codegenome-cli` | 6 CLI commands (index, query, status, verify, serve, experiment) |
| `codegenome-mcp` | MCP stdio tool server with 4 tools |

### Core Concepts

**UOR Address.** Every graph node is identified by `BLAKE3(content)`. The same code always produces the same address, regardless of toolchain version.

**Overlay.** A composable layer that contributes nodes and edges without knowledge of other overlays. Nine overlays exist, each implementing the `Overlay` trait.

**Confidence Fusion.** When multiple overlays produce edges between the same nodes with the same relation, their confidences fuse via noisy-OR: `P = 1 - (1-c1)(1-c2)`. Two independent 0.7 estimates become 0.94.

**Signal Propagation.** Impact flows forward through fused edges. Staleness flows backward. Both attenuate by edge confidence per hop.

## Quick Start

```bash
# Clone and build
git clone https://github.com/MythologIQ/CodeGenome.git
cd CodeGenome
cargo build --workspace

# Index a codebase
cargo run -p codegenome-cli -- index --source-dir codegenome-core/src

# Query impact from a location
cargo run -p codegenome-cli -- query --file codegenome-core/src/overlay/syntax.rs --line 44

# Check index freshness
cargo run -p codegenome-cli -- status

# Start MCP server for AI assistant integration
cargo run -p codegenome-cli -- serve

# Run the self-evolving experiment engine
cargo run -p codegenome-cli -- experiment --source-dir codegenome-core/src --max-iterations 100

# Verify experiment chain integrity
cargo run -p codegenome-cli -- verify
```

## Graph Composition

Nine overlay layers compose through a shared `Overlay` trait:

| Layer | Edge Types | Source | Confidence |
|-------|-----------|--------|-----------|
| Syntax | `Contains` | tree-sitter AST | 1.0 |
| Semantic | `Calls`, `Imports`, `Implements` | Heuristic name resolution | 0.7-0.8 |
| Flow | `ControlFlow`, `DataFlow` | Intraprocedural AST walk | 1.0 |
| Process | `PartOfProcess` | Entrypoint BFS tracing | 0.9^depth |
| PDG | `ControlDependence`, `DataFlow` | CFG+DFG composition | 1.0 |
| Runtime | `Calls` (weighted) | TSV trace file | count/10 |
| SCIP | `References`, `Implements` | Compiler index (protobuf) | 1.0 |
| LSP | `References` | rust-analyzer subprocess | 1.0 |
| **Fused** | **All types merged** | **Noisy-OR fusion** | **max ≤ fused ≤ 1.0** |

## Experiment Engine & Karpathy Loop

The engine implements a four-tier autoresearch pattern inspired by Karpathy's approach to self-improving systems:

| Tier | Behavior | Trigger |
|------|---------|---------|
| 1 — Mechanical | Hill-climb 3 parameters | Every iteration |
| 1.5 — Adaptive | Widen search exponentially, restart with random params | Plateau (10 no-improvement) |
| 2 — Agent-Guided | Local LLM (Phi-3 Mini, 4-bit) reads TSV, recommends strategy | After mechanical restart |
| 3 — Autopoietic | Switch fitness function mid-run | LLM recommends SWITCH_FITNESS |

Four fitness functions (switchable at runtime):

| Function | Measures | Range |
|----------|---------|-------|
| ImpactAccuracy | Sibling reachability via propagation | [0, 1] |
| PropagationDepth | Average max hops from sampled symbols | [0, 1] |
| CycleTime | Parse-propagate speed (faster = higher) | [0, 1] |
| GraphDensity | Edge-to-node ratio in fused overlay | [0, 1] |

### Experiment Parameters

| Parameter | Default | Range | Effect |
|-----------|---------|-------|--------|
| `confidence_threshold` | 0.75 | 0.01 - 0.99 | Minimum confidence to include in propagation |
| `attenuation_factor` | 0.85 | 0.1 - 2.0 | Signal decay per hop |
| `max_depth` | 5.0 | 1.0 - 20.0 | Maximum propagation depth |

## Experimental Results

### Run 001: Pre-Fusion Engine (224,703 iterations)

The first autonomous run used the three-layer engine (syntax + semantic + flow) without confidence fusion.

```
Duration:     ~62 hours (~100 iterations/minute)
Iterations:   224,703
Fitness peak: 0.896
Avg fitness:  0.59
Total KEEPs:  8 (0.004%)
Restarts:     ~7,500
```

**Trend analysis (sampled every 25K iterations):**

| Iteration | Avg Fitness | Max Fitness | KEEPs | Restarts |
|-----------|------------|------------|-------|----------|
| 0-25K | 0.591 | 0.894 | 6 | 331 |
| 25K-50K | 0.588 | 0.894 | 0 | 334 |
| 50K-75K | 0.620 | 0.894 | 0 | 333 |
| 100K-125K | 0.612 | 0.894 | 0 | 333 |
| 125K-150K | 0.565 | 0.895 | 1 | 333 |
| 175K-200K | 0.596 | 0.896 | 1 | 332 |
| 200K-225K | 0.620 | 0.896 | 0 | 334 |

**Hypothesis**: The fitness function has a structural ceiling determined by the graph's edge confidence distribution. With heuristic edges at 0.7-0.8, propagation accuracy is limited by the weakest links in the chain.

**Actualization**: The ceiling at 0.896 was confirmed stable across 224K iterations. The engine found it within the first 1,000 iterations and spent the remaining 223K cycling between plateau→widen→restart without improvement. Average fitness (0.59) reflects the restart penalty — each restart drops to ~0.30 before climbing back.

**Discovery**: The engine converged on `attenuation_factor ≈ 1.3-1.5` as optimal — signal amplification rather than decay. This was unexpected: we assumed attenuation < 1.0 (signal weakens per hop), but the engine discovered that mild amplification compensates for low-confidence heuristic edges. When attenuation exceeded ~1.7, BFS scores exploded (producing -inf/NaN), revealing a stability boundary that required a code fix (score clamping).

### Run 002: Post-Fusion Engine (in progress)

After adding confidence fusion (noisy-OR), the engine was restarted with the same parameters.

```
Baseline:     0.644 (vs 0.292 pre-fusion — 2.2x higher starting point)
Early peak:   0.825 at iteration 20
Architecture: Fused overlay merges duplicate edges
```

**Hypothesis**: Fusion raises the ceiling by strengthening weak edges. Two independent 0.7 estimates fuse to 0.94. The propagation engine now operates on higher-confidence paths, enabling deeper and more accurate impact analysis.

**Early signal**: The baseline jumped from 0.29 to 0.64 without any parameter changes — purely from structural improvement (fused edges). The engine reached 0.825 in 20 iterations (vs 224K iterations at 0.896 pre-fusion). This run is ongoing.

### Phase-by-Phase Fitness Evolution

```
Phase  4 (baseline):     fitness = 0.29   (syntax only, 1 overlay)
Phase  6 (three-layer):  fitness = 0.29   (same params, 3 overlays, but unfused)
Phase  7 (adaptive):     fitness = 0.35   (plateau detection + widen)
Phase 11 (fusion):       fitness = 0.87   (noisy-OR edge merge — breakthrough)
Run 001 (224K iters):    ceiling = 0.896  (pre-fusion, exhaustive search)
Run 002 (post-fusion):   baseline = 0.64  (fusion raises the floor)
```

**Key insight**: Structural changes to the graph (adding overlays, fusing edges) produce larger fitness improvements than parameter optimization. The engine spent 224K iterations improving parameters by 0.006 (0.890→0.896). One architectural change (fusion) improved the baseline by 0.35 (0.29→0.64).

## Governance

Every graph operation is governed:

- **Merkle Ledger**: BLAKE3-chained record of all index/query/fuse operations
- **Evidence Log**: Tamper-evident TSV with chain verification
- **Policy Engine**: TOML rules for allow/deny/require-approval decisions
- **Index Freshness**: Source hash comparison detects stale indexes

```toml
# governance.toml
[[rules]]
operation = "query"
condition = "impact_nodes > 10"
action = "require-approval"
```

## MCP Integration

CODEGENOME exposes four tools via the Model Context Protocol (stdio):

| Tool | Input | Output |
|------|-------|--------|
| `codegenome_context` | file, line | Node + neighbors |
| `codegenome_impact` | file, line | Blast radius with confidence scores |
| `codegenome_detect_changes` | diff text | Affected symbols + impact map |
| `codegenome_trace` | entrypoint name | Process call chain |

```bash
# Start the MCP server
codegenome index --source-dir .
codegenome serve
```

## Project Stats

| Metric | Value |
|--------|-------|
| Source files | 92 Rust |
| Tests | 125 (all passing) |
| Crates | 3 (core + cli + mcp) |
| Overlays | 9 |
| Edge types | 10 |
| CLI commands | 6 |
| MCP tools | 4 |
| Fitness functions | 4 |
| Governance entries | 78 |
| Backlog completion | 21 of 22 |
| Experiment data | 224K+ iterations archived |

## Dependencies

| Package | Purpose |
|---------|---------|
| `blake3` | Content-addressed hashing for UOR identity + chain integrity |
| `serde` + `bincode` + `serde_json` | Serialization for graph store, checkpoints, JSON output |
| `tree-sitter` + `tree-sitter-rust` | Incremental parsing for syntax overlay |
| `clap` | CLI argument parsing |
| `rand` | Randomized parameter perturbation |
| `mistralrs` | Embedded local LLM for Tier 2 experiment advisor |
| `tokio` | Async runtime for LLM + MCP server |
| `rmcp` | MCP protocol SDK for tool server |
| `toml` | Governance policy config parsing |
| `prost` | Protobuf decoding for SCIP index files |
| `lsp-types` | LSP protocol types for rust-analyzer integration |
| `proptest` | Property-based testing (dev only) |

## Roadmap

| Item | Status | Source |
|------|--------|--------|
| UOR addressing | `implemented` | `identity/` |
| 9 overlay types | `implemented` | `overlay/` |
| Confidence fusion | `implemented` | `overlay/fused.rs` |
| Signal propagation | `implemented` | `signal/` |
| Karpathy Loop (Tier 1-3) | `implemented` | `experiments/` |
| Secured experiments | `implemented` | BLAKE3 chain + checkpoint |
| CLI (6 commands) | `implemented` | `codegenome-cli` |
| MCP server (4 tools) | `implemented` | `codegenome-mcp` |
| Governance (ledger + policy) | `implemented` | `governance/` |
| Evidence log | `implemented` | `evidence/` |
| Index freshness | `implemented` | `store/meta.rs` |
| Capability broker | `planned` | Awaiting FailSafe-Pro |
| Multi-language support | `deferred` | Currently Rust-only |

## Contributing

This is an active research project. The codebase evolves through a governance protocol (plan -> audit -> implement -> substantiate) with cryptographic session seals. Contact the maintainers before submitting changes.

## License

[MIT](LICENSE)

---

<p align="center">
  <em>92 files. 125 tests. 78 governance entries. 16 phases. 9 overlays.<br>
  The graph fuses independent evidence into stronger signals.<br>
  The engine evolves, adapts, thinks, and chooses.</em>
</p>

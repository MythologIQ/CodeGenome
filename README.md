<p align="center">
  <strong>CODEGENOME</strong><br>
  <em>Unified Code Reality Graph</em>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.88%2B-orange?logo=rust&logoColor=white" alt="Rust 1.88+"></a>
  <a href="https://github.com/MythologIQ/CodeGenome/actions"><img src="https://img.shields.io/badge/tests-72%20passing-brightgreen" alt="Tests: 72 passing"></a>
  <a href="https://github.com/MythologIQ/CodeGenome"><img src="https://img.shields.io/badge/clippy-clean-blue" alt="Clippy: clean"></a>
  <a href="https://github.com/MythologIQ/CodeGenome"><img src="https://img.shields.io/badge/status-research%20prototype-yellow" alt="Status: research prototype"></a>
  <a href="https://github.com/MythologIQ/CodeGenome"><img src="https://img.shields.io/badge/license-TBD-lightgrey" alt="License: TBD"></a>
</p>

---

CODEGENOME is a content-addressed, multi-layer program analysis graph that merges syntax, semantics, and control/data flow into a single queryable substrate. Every node has a cryptographic identity (BLAKE3). Every edge carries a confidence score. The system hill-climbs on its own accuracy using an adaptive experiment engine.

> **Research prototype.** Not production-ready. APIs will change.

## Problem

Existing code intelligence tools operate on a single representation: syntax (tree-sitter), semantics (LSP), or flow (compiler IR). No tool unifies all three into a single graph with confidence-scored edges, content-addressed identity, and self-evaluating accuracy.

## Architecture

```
Source Files
    |
    v
[Syntax Overlay]     tree-sitter AST -> Contains edges (confidence: 1.0)
    |
    v
[Semantic Overlay]   Heuristic name resolution -> Calls, Imports, Implements (0.7-0.8)
    |
    v
[Flow Overlay]       Intraprocedural CFG/DFG -> ControlFlow, DataFlow (1.0)
    |
    v
[Signal Propagation] Impact forward, staleness backward, confidence attenuation
    |
    v
[Experiment Engine]  Adaptive hill-climbing on fitness(graph) with plateau detection
```

### Workspace

| Crate | Purpose |
|-------|---------|
| `codegenome-core` | Graph types, overlays, signal propagation, experiments |
| `codegenome-cli` | Binary entry point for experiment loop |

### Core Concepts

**UOR Address.** Every graph node is identified by `BLAKE3(content)`. The same code always produces the same address, regardless of toolchain version.

**Overlay.** A composable layer that contributes nodes and edges without knowledge of other overlays. Three overlays exist: Syntax, Semantic, Flow.

**Confidence.** Every edge carries a `[0.0, 1.0]` confidence score. Syntax edges are 1.0 (deterministic). Semantic edges are 0.7-0.8 (heuristic). Signal attenuates as it propagates.

**Signal Propagation.** Impact flows forward through edges (A changed -> B affected). Staleness flows backward (B stale -> A may need re-evaluation). Both attenuate by edge confidence.

## Quick Start

```bash
# Clone
git clone https://github.com/MythologIQ/CodeGenome.git
cd CodeGenome

# Build
cargo build --workspace

# Run tests
cargo test --workspace

# Run the experiment engine (indexes its own source code)
cargo run -p codegenome-cli -- experiment \
  --source-dir codegenome-core/src \
  --max-iterations 10
```

### Example Output

```
[0] baseline: fitness=0.2933 stability=0.9292 (902 ms)
[1] KEEP: fitness=0.3239 stability=0.9218 (1000 ms)
[2] KEEP: fitness=0.3451 stability=0.9143 (970 ms)
[3] KEEP: fitness=0.3470 stability=0.8122 (984 ms)
```

The engine indexes its own 54 source files into a three-layer graph, then hill-climbs parameters to maximize impact prediction accuracy. Plateau detection widens the search; repeated failure triggers random restarts.

## Graph Composition

The graph merges three overlay layers through a shared `Overlay` trait:

| Layer | Edge Types | Source | Ground Truth |
|-------|-----------|--------|--------------|
| Syntax | `Contains` | tree-sitter AST | Available (deterministic) |
| Semantic | `Calls`, `Imports`, `Implements` | Heuristic name resolution | Constructible (testable) |
| Flow | `ControlFlow`, `DataFlow` | Intraprocedural AST walk | Available (deterministic) |

Overlays compose without mutual knowledge. Signal propagation traverses all layers simultaneously.

## Experiment Engine

The adaptive experiment loop implements a simplified Karpathy-style autoresearch pattern:

1. **Measure.** Parse source into a three-layer graph. Compute impact accuracy and stability fitness.
2. **Perturb.** Randomly adjust parameters (confidence threshold, attenuation factor, max depth).
3. **Select.** Keep if fitness improves. Discard otherwise.
4. **Adapt.** Detect plateaus (10 iterations no improvement). Widen search exponentially. Restart with random parameters after 3 failed widenings.

Results log to TSV for post-hoc analysis.

### Parameters

| Parameter | Default | Range | Effect |
|-----------|---------|-------|--------|
| `confidence_threshold` | 0.75 | 0.01 - 0.99 | Minimum confidence to include in propagation |
| `attenuation_factor` | 0.85 | 0.1 - 2.0 | Signal decay per hop |
| `max_depth` | 5.0 | 1.0 - 20.0 | Maximum propagation depth |

## Project Stats

| Metric | Value |
|--------|-------|
| Source files | 54 Rust |
| Tests | 72 (all passing) |
| Crates | 2 (core + cli) |
| Overlays | 3 (syntax, semantic, flow) |
| Edge types | 6 |
| Dependencies | 8 (blake3, serde, bincode, tree-sitter, tree-sitter-rust, clap, rand, proptest) |

## Dependencies

| Package | Purpose |
|---------|---------|
| `blake3` | Content-addressed hashing for UOR identity |
| `serde` + `bincode` | Serialization for on-disk graph store |
| `tree-sitter` + `tree-sitter-rust` | Incremental parsing for syntax overlay |
| `clap` | CLI argument parsing |
| `rand` | Randomized parameter perturbation in experiments |
| `proptest` | Property-based testing (dev only) |

## Roadmap

Status labels: `implemented` | `planned` | `deferred`

| Item | Status | Source |
|------|--------|--------|
| UOR addressing (BLAKE3) | `implemented` | `identity/` module |
| Syntax overlay (tree-sitter) | `implemented` | `overlay/syntax.rs` |
| Semantic overlay (heuristic) | `implemented` | `overlay/semantic.rs` |
| Flow overlay (CFG/DFG) | `implemented` | `overlay/flow.rs` |
| Signal propagation | `implemented` | `signal/` module |
| Adaptive experiment engine | `implemented` | `experiments/` module |
| On-disk graph store | `implemented` | `store/ondisk.rs` |
| Diff-to-symbol mapping | `implemented` | `diff/mapper.rs` |
| MCP tool server | `planned` | BACKLOG B16 |
| CLI commands (index, query, status, verify) | `planned` | BACKLOG B17 |
| LSP integration | `planned` | BACKLOG B19 |
| Governance integration | `planned` | BACKLOG B12-B15 |
| Runtime trace ingestion | `planned` | BACKLOG B21 |
| Local model for agent-guided evolution | `planned` | Phase 8 (Tier 2 Karpathy Loop) |
| Multi-language support | `deferred` | Currently Rust-only |

## Contributing

This is an active research project. The codebase evolves through a governance protocol (plan -> audit -> implement -> substantiate) with cryptographic session seals. Contact the maintainers before submitting changes.

## License

License TBD. All rights reserved until a license is selected.

---

<p align="center">
  <em>54 files. 72 tests. 38 governance entries. The graph understands.</em>
</p>

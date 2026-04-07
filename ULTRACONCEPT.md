# ULTRACONCEPT

## System Definition

CODEGENOME is a canonical code reality substrate.

It is designed to represent software systems as a governed, multi-layer, content-addressed graph in which structural facts, semantic facts, flow facts, runtime facts, cross-repository relations, confidence, provenance, and derived reasoning outputs can coexist inside one explicit model.

The intended outcome is not “better code search” or “a smarter assistant.” The intended outcome is a universal internal representation from which those functions can be derived as query, traversal, policy, and mutation operations over the same canonical reality model.

## Problem Statement

The current tooling landscape fragments code understanding across incompatible models:

- syntax-aware parsers
- semantic indexers
- dependency analyzers
- runtime tracing systems
- repository relationship tools
- assistant retrieval pipelines

Each system builds a partial internal truth optimized for one surface area. The ecosystem then compensates for those incompatible truths by repeatedly translating, summarizing, or approximating across boundaries.

This produces four persistent failures:

1. Identity drift
   The same artifact is represented differently across tools and sessions.

2. Semantic drift
   A relationship recovered in one subsystem cannot be expressed directly in another without lossy conversion.

3. Provenance loss
   High-level conclusions frequently outlive the evidence and method that produced them.

4. Governance weakness
   Read and write operations operate without a single substrate-level model for confidence, policy, and auditability.

CODEGENOME addresses these failures by replacing tool-local truth with canonical graph reality.

## Architectural Thesis

Everything relevant to code understanding should reduce to explicit graph state plus governed operations over that state.

This includes:

- artifact identity
- containment and structure
- symbol relations
- control/data/process flow
- runtime observations
- inter-repository dependencies
- uncertainty
- provenance
- evaluation signals
- reasoning outputs

The system should therefore be designed around the following invariant:

> If a feature cannot be expressed as graph construction, graph traversal, evidence fusion, policy evaluation, or controlled graph mutation, it is architectural drift.

## Primary Invariants

### 1. Canonical Identity

All durable artifacts must have stable identity independent of model vendor, analyzer implementation, UI surface, or session.

This requires:

- content-addressed identity
- deterministic canonicalization where required
- graph references based on identity rather than transient handles

Identity is not a convenience. It is the basis for deduplication, federation, reproducibility, and long-horizon memory.

### 2. Explicit Uncertainty

Confidence must be represented directly in graph state and in every derived operation that depends on uncertain evidence.

Confidence is required for:

- multi-resolver fusion
- traversal filtering
- policy decisions
- write gating
- staging or rejection of low-confidence claims

If uncertainty is hidden inside a tool-specific heuristic, it is unavailable to the substrate and therefore unusable by governance.

### 3. Provenance-Carrying Data

Every non-trivial graph artifact should carry provenance sufficient to answer:

- what created this claim
- when it was created
- whether it was inferred or observed
- what evidence supports it

Provenance is part of the graph model, not metadata attached after the fact.

### 4. Observer Separation

External systems do not define reality; they contribute observations about reality.

Examples:

- tree-sitter contributes structural observations
- LSP/SCIP contribute semantic observations
- runtime traces contribute dynamic observations
- git diffs contribute change observations
- future reasoning systems contribute derived interpretations

No observer is canonical. The graph is canonical.

### 5. Governance-Native Execution

If the substrate is to support both read and write operations, governance must sit inside the operating model.

That means:

- policy is evaluated against graph/provenance/confidence state
- write gating happens before mutation
- ledgering is tied to substrate operations
- auditability is first-class

Governance cannot be a downstream wrapper if the substrate will eventually power mutation, promotion, approval, and autonomous flows.

### 6. Federation as a Base Capability

Repository-local reasoning is insufficient for real software memory.

The substrate must support:

- isolated repository subgraphs
- workspace-level federated overlays
- explicit cross-repository evidence edges
- query modes that preserve the distinction between local and federated reasoning

Federation is not “multi-root indexing mode.” It is a higher-order graph layer with its own evaluation space.

## Layer Model

### Layer 1: Canonical Reality

Purpose:
Store what exists.

Responsibilities:

- node and edge primitives
- content-addressed identity
- overlay model
- confidence representation
- provenance representation
- persistent graph storage

This layer should remain pure, explicit, and transport-independent.

### Layer 2: Extraction and Observation

Purpose:
Convert source evidence into graph artifacts.

Responsibilities:

- syntax extraction
- semantic extraction
- flow overlay construction
- runtime trace ingestion
- git diff conversion
- cross-repository evidence extraction

This layer produces observations; it does not own canonical truth.

### Layer 3: Composition and Fusion

Purpose:
Combine observations into coherent graph state.

Responsibilities:

- overlay orchestration
- confidence fusion
- deduplication by canonical identity
- federation overlay construction

This layer is where independent evidence sources become a usable substrate.

### Layer 4: Query and Traversal

Purpose:
Make graph reality operationally usable.

Responsibilities:

- address resolution
- traversal execution
- path collection
- impact analysis
- context retrieval
- workspace-level tracing

This layer must be storage-agnostic and operate on explicit graph data.

### Layer 5: Governance and Mutation Control

Purpose:
Constrain and audit actions.

Responsibilities:

- policy evaluation
- write gating
- confidence floors
- approval boundaries
- mutation recording
- ledger continuation

This layer should consume provenance and confidence rather than duplicate them.

### Layer 6: Evaluation and Evolution

Purpose:
Measure and improve substrate quality over time.

Responsibilities:

- experiment logging
- analytics over experiment runs
- architecture quality metrics
- workspace quality metrics
- adaptive reconfiguration and future autotuning

The system should not only answer questions about code. It should also answer questions about its own accuracy and coverage.

## Capability Classes the Substrate Must Internalize

The long-term goal is not interoperability with existing product categories as peers. The long-term goal is to subsume their useful capability classes into one substrate.

That includes native support for:

- context assembly
- structural repository understanding
- dependency reasoning
- conditional and flow reasoning
- impact propagation
- change detection
- cross-repo topology
- assistant-serving retrieval
- governed write actions
- persistent code memory

In this architecture, those are not separate products.
They are views and operations over one graph substrate.

## Product Surfaces as Derived Views

Under this model:

- code search becomes graph query
- dependency mapping becomes graph query
- impact analysis becomes graph traversal
- assistant context becomes graph retrieval plus evidence formatting
- workspace memory becomes federation plus query
- repository intelligence becomes substrate introspection
- governed write workflows become policy-evaluated mutation pathways

This is the key distinction:

The system should not be assembled from product features.
Product features should emerge from the substrate.

## Non-Goals

CODEGENOME is not intended to be:

- a thin wrapper around existing assistant products
- a single-tool integration hub
- a UI-first code intelligence product with a weak internal model
- an embedding-defined repository memory
- a black-box reasoning engine with unverifiable outputs
- a repo-local-only analyzer

## Engineering Consequences

If the thesis is taken seriously, several design consequences follow:

### Keep boundary logic at the boundary

Transport layers such as MCP or CLI should not own domain semantics.

They should:

- receive inputs
- call core operations
- format outputs
- enforce policy decisions returned by core

### Keep traversal pure

Traversal should execute over graph data, not over storage abstractions.

This preserves:

- testability
- reusability
- deterministic behavior
- compatibility with fused and federated overlays

### Prefer incremental compute before incremental mutation

File-granular recomputation with full overlay writes is a better first step than partial subgraph surgery if the latter introduces store complexity without proportional benefit.

### Separate repository-local and federated reasoning

Federation should remain explicit in both data model and interface. Silent cross-repo behavior creates ambiguity in meaning, performance, and governance.

### Treat reasoning as a graph artifact class

Future high-level conclusions should be representable inside the same substrate with provenance and confidence rather than existing only as transient assistant text.

## Success Criteria

CODEGENOME succeeds if it can become:

1. The canonical identity system for software artifacts
2. The canonical relation system for code structure and dependency
3. The canonical traversal/query substrate for reasoning over code reality
4. The canonical governance surface for controlled read/write actions
5. The canonical long-horizon memory layer across repositories and sessions

At that point, assistant systems, repository intelligence systems, and higher-order reasoning systems become clients of the same underlying reality rather than separate truth engines.

## Final Definition

CODEGENOME is a governed, content-addressed, multi-representation code reality substrate.

Its role is to make the software ecosystem legible as one explicit, queryable, confidence-aware, provenance-carrying graph from which analysis, retrieval, reasoning, federation, and governance can all be derived without redefining reality at each layer.

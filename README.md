# AXIOM Mathematics Academy

> A rigorous mathematics academy with an honest practice loop, from numeracy through proof-based undergraduate pure mathematics, graduate pathways, research preparation, and soroban mental arithmetic.

Authoritative specification: [`docs/AXIOM-SOURCE-OF-TRUTH.md`](docs/AXIOM-SOURCE-OF-TRUTH.md).

## Architecture & System Overview

- **`crates/axiom-math`**: Pure Rust mathematics engine:
  - Exact rational arithmetic ($\mathbb{Q}$) and finite decimals via `num-bigint` and `num-rational`.
  - Deterministic RNG v1 (`SHA256(D || S || BE64(c))`) with rejection sampling and golden vectors.
  - RFC 8785 JSON Canonicalization Scheme (JCS) and artifact hashing.
  - Bounded scalar grammar (`scalar-exact-v1`) with ambiguity rejection (mixed numbers, implicit multiplication, decimal comma vs point).
  - Soroban 1:4 bead abacus state machine and legal transition reducer.
  - Deterministic exercise generation and exact answer checking with actionable mathematical feedback.
- **`crates/axiom-learning`**: Pure learning policies:
  - Multi-dimensional adaptive mastery evidence model (Conceptual, Procedural, Reasoning, Transfer) using Beta-style updates.
  - Spaced retrieval review scheduling ladder (`[1, 3, 7, 14, 30, 60, 120]` days).
  - Append-only reward ledger with unique award keys and daily practice caps (100 XP/day).
  - Next-activity recommendation engine (review, next learning, repair).
- **`crates/axiom-content`**: Curriculum compiler, lesson manifest validation, and acyclic dependency graph verification.
- **`crates/axiom-wasm`**: Narrow `wasm-bindgen` facade compiling the exact mathematical engine for client-side execution in the browser Web Worker.
- **`apps/web`**: Responsive React + TypeScript + Vite learner workspace and PWA.

## Getting Started

### Prerequisites
- Rust 1.98+ with `wasm32-unknown-unknown` target
- Node.js 20+ and `pnpm` 10+
- `wasm-pack`

### Quick Start
```bash
# 1. Run workspace mathematical tests
cargo test --workspace

# 2. Build the WASM client package
wasm-pack build crates/axiom-wasm --target web --out-dir ../../packages/axiom-wasm-web

# 3. Install web dependencies & start the learner interface
pnpm install
pnpm --filter web dev
```

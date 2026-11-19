# AXIOM Mathematics Academy — local prototype

> A rigorous mathematics academy with an honest practice loop, from numeracy through proof-based undergraduate pure mathematics, graduate pathways, research preparation, and soroban mental arithmetic.

Authoritative specification: [`docs/AXIOM-SOURCE-OF-TRUTH.md`](docs/AXIOM-SOURCE-OF-TRUTH.md). Read [the current independent review handoff](docs/handoffs/2026-10-10-independent-review.md) before continuing.

## Current implementation status

Practice now includes encouragement for correct answers and successful retries, progressive hints, and “Show me how” worked help from the Rust engine. Lessons acknowledge corrected predictions and written reasoning; support use is recorded locally without claiming mastery. See [the feedback handoff](docs/handoffs/2026-10-10-positive-feedback.md) and run `node apps/web/tests/positive-feedback.cjs` against an isolated server.

The abacus now opens with six guided draft stages from one-rod bead reading to initial mental rehearsal, with legal Rust teaching traces, deterministic practice and progressively hidden visual support. Free exploration remains available. Fraction comparison and base-ten regrouping include learner-controlled diagrams. Read [the abacus handoff](docs/handoffs/2026-10-10-abacus-learning.md) and run `node apps/web/tests/abacus-tutor.cjs` against an isolated dev server. These stages are a beginner workshop, not completed advanced abacus courses or certified mental technique.

This is a **local prototype**, not completed Phase 2. Twelve expanded draft lessons cover five partial courses. The 83-course catalogue is the roadmap, including graduate and research preparation; its planned entries are not authored courses. All pilot material awaits independent mathematical and pedagogical review.

Browser generation/grading now uses the compiled Rust engine in a worker, rather than a separate JavaScript RNG/checker. The soroban uses Rust transitions and values. Local practice rewards, drafts, and reasoning notes are device history, not authoritative mastery evidence. The learning policies described below are scaffolds; full evidence eligibility, prerequisite mappings, trusted retention, and assessment orchestration remain incomplete.

Real identity, durable PostgreSQL account storage, account sync, formal-proof verification, signed content packs, full accessibility review, and public-release content validation remain incomplete. Private API routes fail closed. The API executable requires explicit `AXIOM_LOCAL_DEMO=1`, binds loopback, uses one synthetic principal and a memory repository, rejects `x-user-id`, and reports not ready for accounts. Do not deploy it as an account service.

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
- Rust with `wasm32-unknown-unknown` target
- Node.js and `pnpm`
- `wasm-pack`

### Quick Start
```bash
# 1. Run workspace mathematical tests
cargo test --workspace --locked

# 2. Build the WASM client package
wasm-pack build crates/axiom-wasm --target web --dev --out-dir ../../packages/axiom-wasm-web

# 3. Install web dependencies & start the learner interface
pnpm install --frozen-lockfile
pnpm --filter web dev
```

Rebuild the WASM package after Rust changes. It is required; there is no JavaScript mathematics fallback. The dev server defaults to loopback. The review used Rust 1.98.1, Node 26.4.0 and pnpm 10.34.6; these are observed versions, not a reproducible support matrix. Pin and validate supported toolchains before release, and separately build/measure optimized WASM.

## Verification

```sh
cargo run -p axiom-cli --locked
pnpm --filter web build
pnpm --filter web test
```

The content CLI validates draft metadata, not mathematical correctness or human review, and refuses publication until full release validation exists. The browser test requires a running local web server, Playwright, and Chrome on Windows. `AXIOM_TEST_URL` defaults to `http://127.0.0.1:3000/`; `AXIOM_RUNTIME_NODE_MODULES` selects an existing Playwright dependency directory, `AXIOM_BROWSER_EXECUTABLE` selects a browser executable, and `AXIOM_EVIDENCE_DIR` selects evidence output. Existing Codex bundled dependencies are supported without adding them to the application runtime.

Browser regressions compare nine native Rust artifacts against the browser worker, then exercise grading, duplicate rewards, draft recovery, content, mobile views, and keyboard abacus controls. Production service-worker offline behavior is a separate check. Evidence is under `docs/evidence/`; green builds do not prove release readiness.

The [UI polish handoff](docs/handoffs/2026-10-10-ui-polish.md) records the academy interface changes. Additional checks: `node apps/web/tests/ui-polish.cjs` against a dev server, and `node apps/web/tests/offline-review.cjs` against a production preview. Set `AXIOM_TEST_URL` to that server. These scripts reuse the existing Playwright/Chrome development setup.

Preserve existing branding, notes, local history and historical replay identities. Template 1.0.0 presentations remain replayable; corrected presentations are 1.1.0 and corrected exact checkers are v2. Regrading must record its new checker version rather than rewrite an old artifact.

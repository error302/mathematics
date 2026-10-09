# AXIOM Handoff Record

**Date:** 2026-10-09  
**Task and owner request:** "using the axiom source of truth md file lets build this" (repo: https://github.com/error302/mathematics)  
**Current phase and document/ADR versions:** Phase 0 (Foundation) & Phase 1 (Vertical Slice); Source of Truth v1.0.0 (`AXIOM-SOT-001`).  
**Repository branch and commit:** `main` (commit `ab87159` on `origin/main`).  
**Working tree state:** Clean.  

## What Changed and Why
1. **Mathematical Core (`crates/axiom-math`)**:
   - Exact rational numbers $\mathbb{Q}$ and finite decimals implemented using `num-bigint` and `num-rational`.
   - Deterministic RNG v1 (`SHA256(D || S || BE64(c))`) with rejection sampling and golden vectors.
   - RFC 8785 JSON Canonicalization Scheme (JCS) and SHA-256 artifact hashing.
   - Bounded scalar grammar (`scalar-exact-v1`) with ambiguity rejection (mixed numbers, implicit multiplication, decimal comma vs point).
   - Soroban 1:4 bead abacus model and legal transition reducer (ten states per rod, exact place value computation, overflow detection).
   - Deterministic exercise generation (`fractions.compare.positive`, `fractions.equivalent.find`, `arithmetic.whole.addition`, `abacus.read.state`).
   - Answer checking with actionable mathematical feedback (identifying common denominators and domain restrictions).

2. **Learning Policies (`crates/axiom-learning`)**:
   - Multi-dimensional adaptive mastery evidence model (Conceptual, Procedural, Reasoning, Transfer) using Beta-style updates ($\alpha = 1, \beta = 1$).
   - Spaced retrieval review scheduling ladder (`[1, 3, 7, 14, 30, 60, 120]` days).
   - Append-only reward ledger with unique award keys (`instance:first-success:<id>`, `milestone:provisional:<id>`) and daily practice caps (100 XP/day).
   - Next-activity recommendation engine.

3. **Content and Manifest Validation (`crates/axiom-content`, `crates/axiom-cli`)**:
   - Manifest schema and content models.
   - DAG cycle detection for prerequisites.
   - Authored and verified lessons:
     - `F04`: `foundation.fractions.compare` (10-step pedagogical journey).
     - `A01`: `abacus.orientation.place_value` (1:4 soroban place value).
   - Content verification CLI tool passing with canonical JCS hashes.

4. **WASM Facade (`crates/axiom-wasm`)**:
   - Narrow `wasm-bindgen` interface for browser Web Worker.

5. **Learner Web Interface (`apps/web`)**:
   - React + TypeScript + Vite.
   - Full vertical slice:
     - Curriculum Atlas with dependency filtering.
     - Fraction comparison 10-step lesson with prediction challenge and visual number line.
     - Practice workspace with deterministic exercise generator and exact checker.
     - Virtual 5-rod Soroban abacus studio with full keyboard and tap controls ($\ge 44 \times 44$ px touch targets).
     - Mastery & Reward Ledger dashboard with 1-click JSON backup export/import.

## Verification Commands
- `cargo test --workspace`: 38 tests passed, 0 failed.
- `cargo run -p axiom-cli`: all authored manifests verified with canonical hashes.
- `pnpm --filter web build`: compiled with Vite 6, zero errors.
- `pnpm --filter web dev`: live at `http://localhost:3000`.

## External Gates
- Remote Git repository: pushed to `https://github.com/error302/mathematics.git`.
- Lean 4 formal verifier: online-only worker architecture specified, deferred to Phase 4.
- Identity Provider: OIDC adapter specified for Phase 2; Phase 1 guest mode with local data sovereignty is active.

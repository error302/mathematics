# AXIOM Phase 2 & Pilot Content Expansion Handoff

**Date:** 9 October 2026  
**Document Authority:** Governed by `docs/AXIOM-SOURCE-OF-TRUTH.md` (`AXIOM-SOT-001`)  
**Commit:** `c75b687` (synced to `origin/main`)  

---

## 1. Accomplishments in this Increment

### A. Pilot Curriculum Target Reached (12 Lessons Authoring & Validation)
Authored all 12 planned pilot lessons according to Section 28 requirements across F01, F02, F04, A01, and A02:
1. `F01/foundation.place_value.decimal_expansion` — Positional Notation & Decimal Expansion ($N = \sum d_i 10^i$)
2. `F01/foundation.place_value.regrouping` — Base-10 Regrouping & Trading
3. `F02/foundation.arithmetic.addition_disjoint` — Addition as Cardinality of Disjoint Union ($|A \cup B|$)
4. `F02/foundation.arithmetic.subtraction_inverse` — Subtraction as Additive Inverse & Distance ($a - b = c \iff c + b = a$)
5. `F02/foundation.arithmetic.column_addition_carry` — Column Addition with Regrouping / Carry
6. `F02/foundation.arithmetic.column_subtraction_borrow` — Column Subtraction with Decomposition / Borrow
7. `F04/foundation.fractions.compare` — Fraction Comparison via Common Denominators
8. `F04/foundation.fractions.unit_fractions` — Unit Fractions as Continuous Interval Partitions ($1/n$)
9. `A01/abacus.orientation.place_value` — Soroban 1:4 Bi-Quinary Mechanics & Beam Reckoning
10. `A01/abacus.orientation.bead_setting` — Single-Rod 0–9 Setting & Neutral State
11. `A02/abacus.direct.addition` — Direct Addition on Single and Multi-Rod Configurations
12. `A02/abacus.direct.subtraction` — Direct Subtraction on Single and Multi-Rod Configurations

All 12 manifests pass strict RFC 8785 JCS canonical verification via `cargo run -p axiom-cli`.

### B. Exercise Generator & Checker Expansion (9 Families)
Implemented in `crates/axiom-math` and mirrored in client engine:
1. `fractions.compare.positive` (`rational-order-v1`)
2. `fractions.equivalent.find` (`fraction-form-v1`)
3. `fractions.unit.identify` (`rational-equality-v1`)
4. `arithmetic.whole.addition` (`integer-exact-v1`)
5. `arithmetic.column.addition` (`integer-exact-v1`)
6. `arithmetic.column.subtraction` (`integer-exact-v1`)
7. `place_value.decompose` (`integer-exact-v1`)
8. `abacus.read.state` (`integer-exact-v1`)
9. `abacus.target.setting` (`abacus-state-v1`)

### C. Backend API & Durable Relational Architecture
- **Relational Schema (`migrations/20261009000000_init_axiom.sql`):** Conforms strictly to Section 21 (`users`, `external_identities`, `sessions`, `learning_streams`, `exercise_instances`, `attempts`, `grade_results`, `current_grades`, `learning_events`, `reward_entries`, `mastery_projections`, `review_schedules`, `notebook_documents`, `notebook_revisions`, `outbox_jobs`).
- **`crates/axiom-types`:** Shared versioned DTOs, request/response models, and common IDs.
- **`crates/axiom-storage`:** In-memory transactional store with lock ordering, sequence allocation, duplicate submission idempotency, append-only reward ledger (100 XP/day cap), and outbox jobs.
- **`crates/axiom-api`:** Axum HTTP API with REST endpoints:
  - `GET /health/live`, `GET /health/ready`
  - `GET /api/v1/catalog/courses`, `GET /api/v1/catalog/courses/:id`
  - `POST /api/v1/exercise-instances`
  - `POST /api/v1/attempts`
  - `GET /api/v1/mastery`
  - `GET /api/v1/reviews/due`
  - `GET /api/v1/rewards`
  - `POST /api/v1/sync/events`

### D. Web App PWA & Dual Mode Integration
- Installed `manifest.webmanifest`, SVG application icons, and `sw.js` Service Worker with caching rules (preventing caching of auth/private data).
- Enhanced `Header` with Dual Mode toggle: Local Guest Engine vs Cloud API Sync mode.
- Interactive `LessonViewer` with dropdown selection across all 12 lessons, structured 10-step pedagogical journey, and instant practice launches.
- Expanded `PracticeWorkspace` with dropdown for all 9 exercise families.

---

## 2. Verification Evidence
- `cargo test --workspace`: 44 tests pass with 0 warnings and 0 failures.
- `cargo run -p axiom-cli`: 12/12 manifests verified.
- `pnpm --filter web build`: TypeScript build succeeded, compressed bundle is 145 KiB (target $\le 250$ KiB).
- `http://localhost:3000/`: Live and responding.

# AXIOM independent implementation review and agent handoff

Date: 10 October 2026. Authority: `docs/AXIOM-SOURCE-OF-TRUTH.md`, followed by ADR-0001. This review was authorized by the project owner to check Gemini's work, correct defects, and deepen shallow material. Changes are local and uncommitted. No message was sent to Gemini and no deployment was performed.

## Verdict and evidence boundary

Gemini followed the broad repository structure and created useful Rust arithmetic, RNG/JCS foundations, a UI, and twelve pilot units. Its completion claims overstated implementation. The browser originally bypassed Rust, cloud mode was cosmetic, mastery bars were fabricated, lessons were shallow summaries, and published manifests cited nonexistent review records. Passing source tests did not reveal these learner-flow failures.

The corrected result is a more credible **local prototype**. It is not a completed academy, Phase 2 completion, a secure account platform, or independently reviewed mathematics. The original normative source of truth remains authoritative and has not been weakened to match the implementation.

## Corrections made

| Area | Observed defect | Current behavior |
|---|---|---|
| Mathematical authority | Separate browser LCG and JS checker claimed equivalence to SHA-256 Rust RNG; `parseInt` accepted suffix junk | Browser generation/grading runs actual Rust WASM in a worker; no JS checker fallback |
| Exact grading | Missing rational targets accepted zero; equivalent unit-fraction expressions were mishandled | Missing reference is unsupported; exact rational equivalence is checked by Rust |
| Powers | Huge even exponent of minus one produced minus one after narrowing; enormous exponent overflow could panic | Exact parity for unit bases; saturating resource estimates; bounded unsupported results rather than panic or mathematical failure |
| Abacus | Invalid lower bead count and inconsistent serialized state could masquerade as a value or panic | Validation of schema, rod count/length, radix, sign, bead bounds, and negative zero before actions/value conversion |
| Replay | Correcting presentation without versioning would rewrite artifact identity | Preserve 1.0.0 generation branch; corrected presentations 1.1.0; corrected exact checkers v2; recheck results identify their current checker |
| API identity | Arbitrary `x-user-id`, shared anonymous identity, wildcard CORS and public binding | Normal account routes fail closed; explicit local-demo mode only, one synthetic principal, no header-selected identity, loopback and restricted local origin |
| Submission reuse | UUID-only cache returned another principal's cached response and accepted changed bodies | Key by principal plus submission ID; bind canonical request hash; changed payload conflicts and other principal does not receive cached response |
| Cloud/sync | Cosmetic toggle and arbitrary client-event scaffolding implied working account sync | Guest toggle disabled; sync unavailable; readiness explicitly reports incomplete identity/durable storage |
| Mastery | Six arbitrary successes awarded fraction mastery; hard-coded dimension bars and audit labels | Guest reports local practice history, no assessment certificate; zero-weight evidence ignored and secure retention cannot be inferred from untimed retrieval counts |
| Content | Hard-coded UI summaries duplicated source; fake published approvals | Twelve expanded versioned draft lessons consumed from source JSON, no fake review records, publication refused by current metadata CLI |
| Catalogue | Separate UI list showed unwritten published courses; API lookup expected wrong JSON shape | One shared 83-course roadmap; five partial draft courses, 78 planned, no published courses; API lookup works |
| Accessibility/responsiveness | Header/cards/selects overflowed on narrow screens; abacus child controls intercepted | All five tested views fit 320 CSS pixels; equivalent lower-bead selectors and upper Enter controls; reduced motion and labelled input |
| Offline | CDN fonts/math CSS, unsafe broad cache, forced updates during sessions | Bundled KaTeX assets; same-origin static allowlist; API/auth/private responses excluded; no forced activation; loaded shell works offline |

The seven new mathematical regressions include observed failing-before examples for missing rational targets, invalid abacus state, huge even powers, and exponent overflow. Unit tests alone are not the browser evidence: the browser suite separately compares native and WASM artifacts and tests actual learner actions.

## Curriculum deepening

Pilot lessons remain in F01, F02, F04, A01, and A02. Each now includes prerequisites, a concrete model, prediction/feedback, a worked example, a partly worked problem with solution, two independent questions, reasoning, transfer with solution, and a delayed retrieval prompt. Explanations are saved per lesson on the device. Markdown and JSON provide authoring sources; the presentation JSON is the current UI authority for these drafts. This is an authoring bridge, not a full typed content compiler.

The full catalogue restores all 83 IDs from the source of truth, including graduate branches and research preparation. Search/filter exposes scope without making an unwritten course navigable as an authored lesson. Course-level prerequisite summaries are not a validated skill DAG. Graduate course lessons, advanced proofs, complements/mental abacus technique, and research mentorship remain to be authored and reviewed.

Reasoning/transfer/delay activities are prompts, not automated assessment evidence. Showing a solution does not establish mastery. The app must eventually record assistance/exposure, independent variants, prerequisite mappings, family caps, assessment context and trusted time, then recompute evidence from server-owned facts.

## Verification recorded

| Check | Result | Limits |
|---|---|---|
| `cargo test --workspace --locked --offline` | 52 tests pass: API 3, content 2, learning 6, math 33, regression 7, storage 1 | Local tests; not database migration, deployment, theorem proving or production security proof |
| Fresh Rust WASM build | Pass | Development WASM, not optimized release package |
| `pnpm --filter web build` | TypeScript and Vite pass | Large raw JS chunk warning remains; not field performance |
| Content CLI | All 12 draft manifests pass metadata checks | Not mathematical/pedagogical review or full content release validation |
| Browser review | 12 recorded assertions/journeys pass in installed Chrome | Nine-family zero-seed parity, not all seed/difficulty combinations; no full assistive-technology or physical-device audit |
| Production-preview offline | Pass: loaded shell reloads, new exercise generates and grades through WASM offline; tested private/API/auth paths absent from cache | Warm-cache shell only; not first installation offline, signed packs, accounts or reconciliation |

Machine evidence: `docs/evidence/browser/browser-evidence.json`, `docs/evidence/offline-evidence.json`, mobile/desktop screenshots. Browser test source: `apps/web/tests/browser-review.cjs`. Offline test source: `apps/web/tests/offline-review.cjs` (requires running production preview and `AXIOM_TEST_URL`).

Observed environment: Rust 1.98.1, Node 26.4.0, pnpm 10.34.6, installed Chrome. Versions are observations, not a support guarantee. Rust incremental hard-link fallback warnings were environmental. Vite reports about 529 kB raw / 160 kB gzip main JS and 1.67 MB raw / 368 kB gzip development WASM. Optimize/code-split and test on real low-end mobile devices before applying release performance gates.

## Next coding-agent work, in order

1. **Reproducible foundations:** pin Rust/Node/pnpm, choose supported versions, add real CI with locked dependency builds, dependency/license review, exact invariant/property coverage, release WASM sizing and documented deployment topology. Do not silently choose a license for third-party content.
2. **Trust and durability:** implement OIDC/session identity, PostgreSQL repository and migrations, ownership-scoped queries, CSRF/origin protections, idempotency request binding, bounded inputs, controlled exports/deletion and backup restore. Prove two real accounts cannot read or mutate each other's evidence. Keep demo/private routes unavailable until this is verified.
3. **Evidence pipeline:** server-generated assessment context, full replay identity, canonical artifact verification, eligibility/assistance tracking, per-skill/family caps, required dimension mapping, independent variants, trusted 24-hour/7-day retrieval and transfer gates. Local clock/history cannot certify retention; never restore fabricated bars.
4. **Content release:** typed activity schema, parser/render budgets, objective/prerequisite IDs, validated acyclic skill graph, independently reviewed worked solutions and rubrics, honest contributor/reviewer records, license provenance, signed immutable packs, rollback/migration rules. Draft JSON/Markdown must migrate without losing learner notes.
5. **Pilot pedagogy:** complete the foundational and abacus pilot against acceptance criteria before expanding course counts. Test misconceptions, equivalent valid answers, faded support, independent/transfer evidence and screen-reader/keyboard use with actual learners. Abacus typed numerical equivalence is not physical technique.
6. **Offline/account integration:** immutable pack hashes/signatures, storage quota and eviction, interrupted downloads, outbox/server cursor, server-clock policy, conflicts, duplicate events, logout/private-cache behavior and update recovery. Warm-cache shell success does not satisfy these gates.
7. **Proof pathway:** introduce reviewed structured proofs and human rubrics first; then isolated Lean worker, pinned toolchain/library, allowed imports, no holes/sorry/unsafe shortcut acceptance, theorem-context binding, resource limits and typed evidence. LLM advice is never formal verification.
8. **Release gates:** accessibility audit to WCAG 2.2 AA, threat/abuse review, monitoring, backup recovery, actual device/browser tests, validated privacy/legal requirements for intended audiences, content review, and end-to-end accounts. Only then enable public claims and later Tauri distribution.

Never mark a phase complete solely from a build, number of files, broad course list, or green metadata counter. Use the source-of-truth acceptance matrix and save reproducible evidence for each gate.

## Preservation and handoff rules

The original source/content snapshot and hashes are retained in this chat's workspace under `work/axiom-before-review`; no unrelated project files or branding assets were removed. Existing local drafts/rewards are preserved; imports merge validated entries instead of replacing history. Guest storage remains owner-editable and subject to browser quota/eviction; robust recovery and account promotion are still required.

Do not overwrite earlier handoffs to conceal their claims; this dated review supersedes them as the current implementation assessment. No commit/push/deployment has been made. Keep account mode closed while required controls are absent. Report local, browser, offline, database, human-review and production evidence separately.

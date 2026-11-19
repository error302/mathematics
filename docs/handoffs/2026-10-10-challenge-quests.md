# AXIOM challenge quests, visual mathematics and certificate design

The owner requested learning challenges for analytical thinking, logic, comprehension and patterns, Crossmath-like arithmetic practice, proper geometry/graphs, and an achievement certificate design. Source-of-truth v1.2.0 Section 33 and ADR-0005 record the scope and mathematical boundaries. Changes are local and uncommitted. The existing 12 pilot lessons remain drafts; full academic pathways are not implemented or certified.

## Try the experience

Open Practice. **Foundation practice** preserves the earlier families. **Puzzle quests** provides arithmetic crossings, planning, logic/counterexamples, word-problem comprehension and stated-rule patterns. **Visual lab** provides rectangle area/perimeter, straight-line equations and quadratics. Start with the strategy example, solve a new item, explain your thinking, then choose a fresh challenge or the next band. Three bands are learner-selected and untimed, not calibrated/adaptive mastery. Lessons now has **Apply it in a reasoning challenge**.

Incorrect grid answers identify a failing clue; first hints and explicit worked help are available. Correct answers and corrected mistakes receive praise. Requested support marks the local attempt assisted; no independent practice XP is awarded afterward. Rechecking or reaching the daily XP cap still receives encouragement without a duplicate reward. Local explanations are saved, not AI/human graded.

Progress has a certificate specimen and download. It identifies the proposed advanced pathway and its future evidence requirements, not an earned degree or mastery of all mathematics. The downloadable HTML prints as a clean certificate design, with permanent specimen/not-earned labeling and unfilled identity/reviewer/verification fields.

## Implementation

- `crates/axiom-math/src/challenges.rs`: original generator, exact precedence/line evaluation, bounded unique-grid enumeration, reasoning/visual references, replay-validated grading and coaching. Eight new families × three bands. No browser mathematical fallback.
- `exercise.rs`, `checker.rs`, `coaching.rs`, `lib.rs`: route new IDs to that module while retaining historical families.
- `PracticeHub.tsx`: Foundation/Puzzle/Visual navigation. `ChallengeWorkshop.tsx`: prepared strategies, bands, exact submission controls, rewards, saved reasoning and draft restore. `ChallengeVisual.tsx`: SVG shape and continuous curve rendering, speech and exact point tables.
- `CertificatePreview.tsx`, public `axiom-certificate-specimen.html`: specimen only; no credential issuance route or earned-state switch.
- `challenges.css`: responsive navy/gold study layout, visible focus and 44-pixel minimum grid choice controls.
- `public/sw.js`: versioned warm-cache shell including the single public specimen. Private/API/auth routes still bypass caches; no forced mid-session service-worker replacement.

Replay contract: new families at template 1.1.0, checker `academy-challenge-v1`, parameter policy 1, existing `rng-sha256-v1`; coaching has `challenge-coaching-v1`. Grading and coaching regenerate from the complete replay and compare the deserialized artifact. Grid answers are arrays in blank order, with exact pool membership. Scalar answers use the existing bounded exact grammar; budget/unsupported exponent cases are unsupported, not wrong. The graph is rendered from Rust points/control; unit scales and axes are documented.

Draft key: `axiom_challenge_draft_v1:<template>:<band>`. Stores current replay artifact, unfinished scalar/choice answer or blank selections, explanation and requested assistance. Restore regenerates trusted reference data and compares the hash. Existing foundation/abacus drafts, lesson notes, reward history, source backups and branding are preserved. These records are local/editable and cannot support high-integrity credentials.

## Reproduce checks

Run the workspace Rust tests; build the actual WASM web package; build TypeScript/Vite. On an isolated dev server, run `apps/web/tests/challenges.cjs`, `browser-review.cjs`, `abacus-tutor.cjs`, `positive-feedback.cjs` and `ui-polish.cjs` with `AXIOM_TEST_URL` set to its address. On the production preview, run `offline-review.cjs`. Native replay fixtures are in `crates/axiom-math/examples/challenge_fixture.rs`; browser comparisons require a dev server for the engine-module import.

Evidence goes to `docs/evidence/challenges/`, the existing browser/tutor/UI/feedback folders, and `docs/evidence/offline-evidence.json`. The authority amendment hash/backup are in `docs/evidence/challenge-authority-evidence.json` and `AXIOM-SOURCE-OF-TRUTH-1.1.0-before-quests.md`. Do not confuse an older owner-vision hash with the amended canonical document; both are historical records.

## Remaining work

Obtain actual mathematical/pedagogical review for examples, prompts and support; test with beginners and assistive technology. Add diverse outcome-linked transfer/explanation tasks, retain versioned old generators, migrate authored strategies into the content compiler/manifest, and implement trusted assessment/retention/account persistence. General graphing requires domain-aware rendering, discontinuity/endpoint conventions and keyboard exploration. Advanced geometry is not supplied by the rectangle pilot.

Credential work is planned: published pathway definitions and specialization scope, actual reviewer/rubric evidence, eligibility evaluation, consented verification, signing/idempotency/revocation and integrity policy. No local XP, self-check, proof-kernel result alone, made-up reviewer or frontend toggle can authorize issuance. The specimen is explicitly unearned. The app remains a local draft academy; no neurological, accredited-degree, universal-mastery or full-curriculum completion claim is introduced.

Final verification: **64 workspace Rust tests passed**, including 96 unique-grid seed/band cases, 768 graph viewport cases, exact continuous quadratic checks and unsupported-math dispositions. WASM and TypeScript/Vite builds passed. The new browser suite passed nine grouped journeys and 24 new-family native/browser replay-and-grade comparisons. The existing foundation (12), abacus (10), encouragement (9) and UI (10) checks passed. Production warm-cache offline checks passed, including fresh puzzle generation/grading, quadratic visuals and the public specimen. No JavaScript page errors were reported. Core contrast tokens passed; that is not full WCAG conformance.

The final JS chunk is approximately 575 kB raw / 174 kB gzip, and dev-profile WASM is approximately 2.16 MB raw / 466 kB gzip. The chunk-size warning remains; performance optimization and release-profile packaging remain release gates. Four original branding assets match the pre-review snapshot hashes. Source-of-truth copies agree after their intentional relative-link differences are normalized. Detailed final evidence: `docs/evidence/challenges/final-verification.json`.

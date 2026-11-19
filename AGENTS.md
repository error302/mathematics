# AXIOM Mathematics Academy — Agent Guidelines

**Document Authority:** Governed strictly by [`docs/AXIOM-SOURCE-OF-TRUTH.md`](docs/AXIOM-SOURCE-OF-TRUTH.md).

Read [`docs/PRODUCT-VISION.md`](docs/PRODUCT-VISION.md) for the owner’s 10 October 2026 academy/brain-gym direction and scientific claims boundary. The source of truth is now version 1.2.0. Mathematical mastery is an evidence-based goal; do not promise universal mastery or treat practice XP as measured neuroplasticity.

## Non-Negotiable Engineering Rules
1. **Mathematical correctness outranks speed and aesthetics.** Exact values use `num-rational` and `num-bigint` over $\mathbb{Q}$. Floating-point arithmetic is never used for foundational mathematical truth.
2. **Rust owns mathematical and learning-policy logic.** Client UI runs the exact same engine via WASM or HTTP API. Client-reported clocks, scores, or mastery percentages are never accepted as authority for synchronized accounts.
3. **Deterministic generation and replay.** Every exercise is strictly reproducible from its `ReplayIdentity` (`template_id`, `template_version`, `generator_version`, `checker_version`, `content_manifest_hash`, `seed_hex`, `difficulty_band`, `parameter_policy_version`) and AXIOM RNG v1.
4. **Actionable mathematical feedback.** Feedback must identify the concrete mathematical obstacle (e.g. converting to common denominators, checking domain restrictions like division by zero), not just state "wrong".
5. **Accessibility is a core requirement.** WCAG 2.2 AA target: full keyboard operability, visible focus, touch targets, plain-text alternatives, screen-reader MathML/speech equivalents, and no timed assessments by default.
6. **Append-only reward ledger.** XP and milestones are derived from immutable ledger entries with unique award keys (`instance:first-success:<id>`, `milestone:provisional:<id>`). Daily practice XP is capped at 100 XP/day. Milestones are lifetime one-time awards.

## Verification Commands
- Check all Rust crates: `cargo test --workspace --locked` (toolchains are pinned in `rust-toolchain.toml`, `.nvmrc` and the root `packageManager` field; CI mirrors these locked commands — see `docs/operations/reproducible-builds.md` and ADR-0007)
- Build WASM web package: `wasm-pack build crates/axiom-wasm --target web --out-dir ../../packages/axiom-wasm-web`
- Run web frontend: `pnpm --filter web dev`
- Run web tests: `pnpm --filter web test`

## Current independent review — 10 October 2026

Read `docs/decisions/ADR-0001-shared-math-and-honest-phase-status.md` and `docs/handoffs/2026-10-10-independent-review.md` before continuing.

- This is a **local prototype**, not completed Phase 2. Cloud mode, durable PostgreSQL account storage, identity, signed content packs, and trusted retention remain incomplete.
- All 12 pilot lessons are **drafts**. Never create fake reviewer approvals or change them to published to satisfy a content counter.
- Browser mathematical generation/checking uses `packages/axiom-wasm-web` through the real Rust worker. Do not reintroduce a JavaScript checker/RNG fallback.
- New lesson/presentation versions are 1.1.0; retain 1.0.0 replay compatibility.
- `pnpm --filter web test` now runs real browser regressions. Set `AXIOM_TEST_URL` to the isolated local dev server. Playwright resolves from installed packages or the Codex bundled runtime; `AXIOM_RUNTIME_NODE_MODULES` can override the location.
- Preserve branding assets, existing notes, local history, and unrelated user files. Local rewards do not certify mastery.

## UI polish follow-up

The owner's positive-reinforcement requirements are in ADR-0004 and `docs/handoffs/2026-10-10-positive-feedback.md`. Use Rust-owned exact worked help, give constructive feedback after mistakes, acknowledge successful retries, and clear stale praise when an answer is edited. Run `node apps/web/tests/positive-feedback.cjs` after relevant changes. Never manufacture mastery or infer internal reasoning from praise.

The latest abacus teaching implementation is documented in `docs/decisions/ADR-0003-beginner-abacus-and-mental-bridge.md` and `docs/handoffs/2026-10-10-abacus-learning.md`. Read them before changing abacus UI or generators. Run `node apps/web/tests/abacus-tutor.cjs` after relevant changes. Keep Rust-owned exact teaching traces and never leak bead-state answers into hidden mental practice.

Read `docs/decisions/ADR-0002-study-interface.md` and `docs/handoffs/2026-10-10-ui-polish.md`. The focused visual layer is `apps/web/src/study.css`. Run `node apps/web/tests/ui-polish.cjs` against an isolated dev server and `node apps/web/tests/offline-review.cjs` against a production preview when changing the application shell. Keep all five destinations visible on mobile and maintain honest availability/draft labels.

Attention-friendly focus tools are documented in ADR-0006 and `docs/handoffs/2026-10-15-attention-friendly-ui.md`. The former rotating wellness/affirmation bar was removed for claims-boundary and WCAG 2.2.2 reasons; do not reintroduce auto-rotating copy, `aria-live` churn, mandatory timers or medical/ADHD/neurological claims. Focus view, the optional focus timer and the Take-a-pause panel are learner-controlled preferences; the practice working-notes scratchpad stays ungraded and local. Run `node apps/web/tests/focus-tools.cjs` on the isolated dev server after relevant changes.

## Challenge quests and certificates

Read source-of-truth Section 33, ADR-0005 and `docs/handoffs/2026-10-10-challenge-quests.md`. Practice now includes original arithmetic crossings, four reasoning families and rectangle/linear/quadratic visuals. Rust owns their generation, exact checks and replay-validated help. Run `node apps/web/tests/challenges.cjs` on the isolated dev server and the production offline regression after changes. The downloadable certificate is a specimen, not earned or issued; never promote local XP or draft lessons to credential eligibility.

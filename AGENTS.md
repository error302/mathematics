# AXIOM Mathematics Academy — Agent Guidelines

**Document Authority:** Governed strictly by [`docs/AXIOM-SOURCE-OF-TRUTH.md`](docs/AXIOM-SOURCE-OF-TRUTH.md).

## Non-Negotiable Engineering Rules
1. **Mathematical correctness outranks speed and aesthetics.** Exact values use `num-rational` and `num-bigint` over $\mathbb{Q}$. Floating-point arithmetic is never used for foundational mathematical truth.
2. **Rust owns mathematical and learning-policy logic.** Client UI runs the exact same engine via WASM or HTTP API. Client-reported clocks, scores, or mastery percentages are never accepted as authority for synchronized accounts.
3. **Deterministic generation and replay.** Every exercise is strictly reproducible from its `ReplayIdentity` (`template_id`, `template_version`, `generator_version`, `checker_version`, `content_manifest_hash`, `seed_hex`, `difficulty_band`, `parameter_policy_version`) and AXIOM RNG v1.
4. **Actionable mathematical feedback.** Feedback must identify the concrete mathematical obstacle (e.g. converting to common denominators, checking domain restrictions like division by zero), not just state "wrong".
5. **Accessibility is a core requirement.** WCAG 2.2 AA target: full keyboard operability, visible focus, touch targets, plain-text alternatives, screen-reader MathML/speech equivalents, and no timed assessments by default.
6. **Append-only reward ledger.** XP and milestones are derived from immutable ledger entries with unique award keys (`instance:first-success:<id>`, `milestone:provisional:<id>`). Daily practice XP is capped at 100 XP/day. Milestones are lifetime one-time awards.

## Verification Commands
- Check all Rust crates: `cargo test --workspace`
- Build WASM web package: `wasm-pack build crates/axiom-wasm --target web --out-dir ../../packages/axiom-wasm-web`
- Run web frontend: `pnpm --filter web dev`
- Run web tests: `pnpm --filter web test`

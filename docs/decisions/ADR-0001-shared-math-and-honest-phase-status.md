# ADR-0001 — Restore shared mathematical authority and honest prototype status

Date: 10 October 2026. Status: implemented for the local prototype under the owner's request to review and correct Gemini's work.

The source of truth requires one Rust mathematics engine, evidence-based mastery, durable authenticated account data, and actual content review. The implementation had a separate browser PRNG/checker, a cosmetic cloud switch, hard-coded mastery bars, an in-memory API trusting `x-user-id`, and published manifests referencing nonexistent review records.

Decision: remove the duplicate JavaScript mathematical implementation and run the compiled Rust engine, with generation/grading in a Web Worker. Rust also determines abacus transitions/values and local reward eligibility. JavaScript adapts rendering, storage, and user interaction.

Treat existing account APIs as synthetic development scaffolding. Their production constructor fails closed; the executable requires `AXIOM_LOCAL_DEMO=1`, binds loopback, rejects client-selected identity, and has no ready/account/sync claim. The guest UI cannot enable cloud mode. PostgreSQL migrations remain unapplied and the repository remains memory-backed; these are explicit implementation gaps.

Remove unsupported mastery declarations from the guest UI while preserving old local data and cosmetic reward history. The Rust prototype no longer treats zero-weight assistance as independent evidence, checks all currently required dimensions with exact thresholds, and cannot infer secure retention from counters without trusted time evidence. Full family caps, prerequisite mappings, retention, and transfer orchestration remain gated.

Keep all 12 expanded lessons as draft version 1.1.0, remove placeholder review approvals, and refuse publication through the current metadata CLI. Lesson presentations and activities live in versioned JSON next to their Markdown/manifest, rather than hard-coded UI arrays. This is an authoring bridge until a full typed content compiler/release validator is implemented.

Corrected exercise presentations use template version 1.1.0. Preserve the 1.0.0 presentation branch for historical replay. Guest exercises carry a content hash derived by the Rust JCS implementation from their current lesson collection. A content hash demonstrates identity, not human review or a signed public release.

Corrected exact checkers identify as v2; old v1 artifacts can be explicitly rechecked under v2 without rewriting their replay identity. Huge powers retain exact parity for unit bases and enforce resource limits without integer-estimate overflow. The shared catalogue contains the complete 83-course roadmap, with only five partial draft courses selectable. Production-preview evidence confirms loaded-shell offline generation/grading, separately from dev browser checks; signed content packs and account reconciliation remain pending.

Alternatives rejected: maintain a supposedly equivalent JavaScript checker; show a fake account service; award mastery based on six arbitrary successes; manufacture reviewer records; claim a green build proves Phase 2 completion.

Validation: native Rust regression suite, independent native/browser artifact comparisons for nine exercise families, real headless-browser learner journeys, narrow-screen checks, content metadata validation, and build evidence recorded separately. Manual screen-reader testing, public release security, real identity/database integration, signed offline packs, formal proofs, and independent mathematical review are not claimed.

Rollback: pre-review source/content snapshots are preserved outside the repository in this chat's workspace. Reverting UI changes must not erase newer local drafts or reward entries. Account endpoints must remain unavailable until their required trust and durability controls exist.

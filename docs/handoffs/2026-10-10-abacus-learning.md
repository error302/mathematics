# AXIOM beginner abacus and mental-arithmetic handoff

The owner requested clearer abacus teaching, a mental-math pathway and visual demonstrations. This implementation follows the existing source of truth, PRODUCT-VISION.md and ADR-0003. All teaching material is draft, all changes are local and no public deployment or commit was made.

## What a beginner now experiences

Guided learning opens with one Ones rod and an explanation of the counting bar. The demonstration shows zero, then the five bead, then one and two lower beads: 0 → 5 → 6 → 7. Every move has a caption, exact numerical value and highlighted rod. Learners can go backward, replay and advance at their own pace.

The route continues through small direct addition and subtraction, then a five exchange (4 + 3 = 4 + 5 − 2) and a ten exchange (8 + 7 = 8 + 10 − 3). A second rod appears only when it is needed. Rod labels explain the actual place weights: a lower Tens bead counts as ten; the upper Tens bead counts as fifty.

Each stage offers fresh Rust-generated practice. In virtual practice, the live total is initially hidden so the learner reads the beads; requesting it is a value hint. The checker accepts any correct final state, rather than requiring the learner to copy the animation's exact route. The teaching trace verifies legal mathematical demonstration steps; learner grading currently checks the final value, not a recorded trajectory transcript.

Initial mental rehearsal uses short one-rod sequences (start, add, subtract). The learner can choose starting beads, an empty frame, or hidden beads. Hidden mode removes the board and its textual state descriptions from the DOM. No automatic countdown, speed ranking, or claim of verified internal mental technique. Worked solutions remain available and are labelled supported practice. Showing support is part of learning, not a punishment.

Stage selection, current replayable challenge, board, unfinished answer, chosen support and assistance flags are saved under new isolated local keys. Existing lessons/notes/reward history are preserved. A checked instance without hints can earn local practice XP using the shared Rust ledger; repeats do not add duplicate rewards and the existing daily cap still applies.

Free exploration preserves the original five-rod instrument and its keyboard controls, with corrected counted-bead geometry. It is a separate view rather than the beginner default.

## Additional mathematics visuals

The fraction comparison lesson now shows equal-length 3/4 and 5/8 strips. A button splits each quarter into two eighths; the shaded amount stays the same and 6/8 can be compared with 5/8.

The regrouping lesson shows fourteen unit squares. A button bundles ten into one ten rod, leaving four ones. Unbundling restores the representation without changing the total. Both visuals provide accessible text explanations and operate offline.

## Architecture and verification

`crates/axiom-math/src/abacus_tutor.rs` owns the six-stage catalogue, exact teaching traces and generator parameter constraints. `crates/axiom-wasm` exports the catalogue, while worker generation/grading uses the existing shared API. `BeadBoard.tsx` renders exact states and sends legal actions to Rust. `AbacusStudio.tsx` orchestrates learning/support/drafts; `AbacusPlayground.tsx` retains free exploration. `ConceptVisual.tsx` renders the two concrete mathematics demonstrations.

Native tests passed: 55 workspace tests including three new tutor tests. The new tests verify legal replay and correct totals for all 35 beginner carry cases and all 10 five-exchange pairs, reject invalid teaching inputs, and verify 144 generated exercises (24 seeds for each of six families) are deterministic and checkable. The six tutor families also match native Rust and the browser worker at the zero-seed reference.

The tutor browser suite exercises the one-rod start, step values, fresh digit-setting, exact grading, repeat reward prevention, 8 → 18 → 15, hidden mental output, malformed/correct typed answers, draft recovery, support hints, mobile fit and both lesson visuals. Evidence is in `docs/evidence/abacus-tutor/`. Existing whole-app and UI regression evidence remains separate.

Final verification passed: the 10 tutor browser checks, 12 existing learner checks, 10 UI checks across 320/390/768/1440-pixel views, TypeScript/Vite build, and production-preview offline checks. Offline verification specifically generated and graded a fresh hidden mental challenge and played a beginner teaching move without a connection. Core keyboard controls and skip navigation were tested; full screen-reader/physical-device validation is not claimed. The large raw JavaScript chunk warning remains (approximately 549 kB raw / 165 kB gzip).

Run after a Rust change:

```sh
cargo test --workspace --locked
wasm-pack build crates/axiom-wasm --target web --dev --out-dir ../../packages/axiom-wasm-web
pnpm --filter web build
```

Against an isolated dev server, set `AXIOM_TEST_URL` and run `node apps/web/tests/abacus-tutor.cjs`; its native reference is `tutor_fixture.rs`. Existing `pnpm --filter web test` also checks free exploration. Against the built preview, run the production offline test. Browser tools use the existing Playwright/Chrome development setup.

## Next work and honest limits

1. Have an abacus instructor and mathematics/pedagogy reviewer examine explanations, physical conventions and accessible adaptations. These six draft workshop stages do not complete A01–A04/A09 courses; the broader course catalogue remains honest about planned material.
2. Extend five/ten teaching to subtraction/borrowing, combined complements, multi-digit zero handling and cascaded carries. Preserve exact-state invariants and add counterexample-based tests before extending mental sequences.
3. Add progressively longer and multi-rod mental practice from reviewed prerequisites. Record task mode, actual exposures and assistance; evaluate delayed unseen transfer separately. Never infer mental technique from a typed number or simply selecting Hidden.
4. Move the tutorial catalogue into the full versioned content compiler once its typed release/reviewer/schema gates are implemented. Preserve current replay identities and learner drafts during migration.
5. Add journalled learner actions/state hashes when implementing trajectory assessment. Current final-state checking must not be relabelled as observed finger skill or fully assessed mastery.
6. Run physical-device and screen-reader testing, storage-quota/recovery tests, actual learner usability sessions and educational outcome evaluation. The automated checks are not those reviews.

No advanced multiplication/division/decimals, adaptive readiness certification, trusted account sync or measured neural-change feature is introduced here. Mental rehearsal is an initial optional pathway, not a replacement for conceptual mathematics.

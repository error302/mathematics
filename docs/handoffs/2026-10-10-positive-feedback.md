# AXIOM positive reinforcement and coaching handoff

The owner's request is now implemented across ordinary practice, guided/mental abacus practice and lesson interactions. ADR-0004 records the intended tone and evidence boundaries. Changes are local and uncommitted; no release-readiness claim is introduced.

## Learner behavior

Correct answers receive varied acknowledgement such as “Nice work. You found a correct answer.” A successful revision after an actual prior incorrect answer receives “That’s progress—you kept going and corrected your answer.” Supported recovery is praised too. Encouragement continues independently of XP and does not create duplicate rewards, timed pressure, fake mastery or neurological claims.

An incorrect answer gets constructive language and a relevant first hint. “Give me a hint” and “Show me how” remain available before or after a check. Full worked help provides exact column arithmetic, equivalent-fraction/common-denominator explanations, unit-fraction guidance, place-value explanation, or the current abacus trace, as appropriate. Worked fraction strips have readable text labels outside the SVG; abacus answer states appear only after an explicit worked reveal.

Editing an answer clears its old success state. Syntax problems are acknowledged as writing/formatting problems; unsupported/inconclusive checking is not described as mathematical failure. New challenges clear prior mistakes/support; local drafts preserve attempt/mistake counts and requested assistance. Those counts are local UI history, not trusted assessment evidence.

Lesson predictions get encouragement, a representation hint and a link to the worked example. Correcting a prediction gets a recovery message. Finished written reasoning, following the final teaching move, and opening a self-check acknowledge the learner's observed effort without claiming an unassessed answer is correct.

## Technical boundary

`Encouragement.tsx` selects wording from actual dispositions and local attempt history. It does not grade mathematics or change reward policy. `FeedbackSupport.tsx` progressively renders Rust-owned `CoachingMaterial` through the worker. The mathematical explanation layer is `crates/axiom-math/src/coaching.rs`; it checks reference consistency and bounded teaching data. Generation/checker identities and outcomes are unchanged. The tutor catalogue and lesson content hash still identify their teaching sources; coaching has its own version.

Ordinary justified feedback after a mistake is distinct from a requested solution. Explicit hint/solution use marks support for that local attempt, and prevents presenting the subsequent practice as independent. Existing Rust ledger uniqueness/caps still apply. Looking at a worked explanation after an already-correct answer is reflection; it does not rewrite the earlier reward or assistance status. No points are taken away for mistakes or help.

## Verification and remaining gates

The new coaching tests cover all 15 current exercise families over 16 reference seeds each (240 exercises), borrowing across zero columns and rejection of an inconsistent reference. Three coaching tests bring the workspace count to 58. The new browser suite checks incorrect arithmetic feedback, first hints without solution dumping, successful retry praise, repeat reward safety, stale-success clearing, input-format messages, exact worked visuals, supported success, hidden-abacus reveal boundaries and lesson prediction recovery.

Evidence is in `docs/evidence/positive-feedback/`; prior whole-app, abacus and UI suites remain separate. Run `node apps/web/tests/positive-feedback.cjs` with `AXIOM_TEST_URL` pointing at an isolated dev or production-preview server. Rust/WASM changes require a fresh WASM build, native tests and web build before browser checks. The existing offline script also verifies worked help without a connection.

Final evidence: 58 workspace Rust tests passed after the final reference-consistency changes; all nine new feedback browser checks passed, including a capped-XP scenario. The 12 ordinary learner checks, 10 tutor checks and 10 UI checks also passed. The final TypeScript/Vite build passed, and the production-preview offline check loaded exact worked help and generated/graded hidden mental practice without a connection. The large raw JS chunk warning remains (approximately 556 kB raw / 167 kB gzip); these are not release performance or full accessibility results.

This is draft coaching for the existing families, not a complete pedagogy evaluator. Future work includes instructor/content review of explanatory language, screen-reader announcements, actual learner trials, journalled help/exposure events for trusted assessments, full content compiler migration and account persistence. Avoid claiming that a typed correct answer establishes the learner's internal reasoning strategy. Keep praise specific, respectful and independent of reward limits.

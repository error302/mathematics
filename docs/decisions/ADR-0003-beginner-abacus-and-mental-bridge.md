# ADR-0003 — Teach the abacus before asking for mental arithmetic

Date: 10 October 2026, Africa/Nairobi. Implemented locally under the owner's request for beginner-friendly abacus training, a path toward mental math and visual demonstrations.

The previous view exposed five rods and a “make 15” task without enough instruction. Its lower-bead positioning did not consistently put the first counted bead beside the bar. A virtual instrument alone was not a teaching sequence.

Keep the existing navy/gold palette and typography, with one memorable visual: a readable one-rod abacus whose gold beads actually meet the counting bar. Expand to two rods only for carrying. The presentation is a left/right worked example and coach on desktop, stacked on mobile. Step numbers represent a real teaching sequence; movement follows learner actions and respects reduced motion.

Add six draft stages: number reading/setting, direct addition, direct subtraction, five exchange, ten exchange, and initial mental rehearsal. Each stage has an exact step-by-step worked example, a fresh deterministic challenge, accessible controls, local draft recovery and optional support. All stages remain explorable; they are not falsely certified or locked by unsupported mastery rules.

Rust owns teaching states, legal moves, arithmetic values, generation and grading. `abacus_tutor.rs` constructs bounded one/two-rod traces through the existing reducer. Six new `abacus.trainer.*` templates use replay identity 1.1.0, beginner difficulty band 1 and the existing v2 exact/state checkers. The native/WASM content hash now includes the draft tutor catalogue alongside lesson JSON. Existing exercise replay branches are retained.

Mental practice fades support through starting beads, an empty frame, and a hidden board. Hidden mode unmounts the state/value controls and descriptions; it does not conceal a screen-reader answer using opacity. A worked-example reveal records assistance. Grading verifies the final mathematical answer; it does not verify a learner's internal visualization or physical technique. Selected support and assistance are local practice state, not trusted retention evidence.

The original instrument remains as Free exploration. Its lower-bead geometry is corrected. Add learner-controlled fraction-strip subdivision and base-ten grouping visuals to the corresponding existing lessons. These demonstrate concrete invariant-preserving transformations and include text alternatives.

Rewards use the existing Rust ledger: a checked challenge without a value/worked-example hint may receive local practice XP, once per instance, subject to the existing cap. They are not mental mastery certificates. Do not add a separate browser mathematical checker or a neuroplasticity score.

Acceptance: legal exact teaching-trace replay tests; generated exercise determinism/checkability; native/browser parity; real beginner-to-mental interactions; no hidden answer descriptions; saved stage/challenge/draft/support; unique rewards; keyboard/mobile controls; working concept visuals; build and warm-cache offline evidence. Full instructor review, signed content release, observed finger technique and delayed independent mastery remain future gates.

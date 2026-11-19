# AXIOM attention-friendly focus tools handoff

The owner asked to continue building with better UI ideas for learning mathematics effectively, especially for people with ADHD. Implemented direction: ADR-0006. Changes are local and uncommitted. No medical, ADHD-treatment, neurological or efficacy claim is introduced; these are ordinary optional interface preferences.

## What changed

- **Removed** `WellnessAffirmationBar.tsx` and `wellness.css`. The bar auto-rotated pseudo-scientific affirmations every 14 seconds in an `aria-live` region on every screen (WCAG 2.2.2 problem, claims-boundary violation, attention distraction) and offered a "Nervous System Reset" modal. No test referenced it; nothing else imported its stylesheet.
- **Added** `FocusTools.tsx` + `focusTools.css`, mounted between the header and main content: a Focus view toggle (persisted `axiom_focus_view`; hides XP-cap detail, footer and card glows via `.axiom-focus-view`; navigation and honest draft labels stay visible), an optional learner-started focus timer (10/20/30/45 minutes, pause/resume/end, one polite break suggestion at completion, never a modal or penalty), and a manually opened "Take a pause" panel with a text breathing pace and explicit non-medical boundary copy.
- **Added** the source-of-truth section 5 practice-workspace scratchpad: a collapsed "Working notes" `<details>` in `PracticeWorkspace.tsx`, labelled "Working notes (kept on this device; never graded)", persisted as `scratch` inside the existing `axiom_practice_draft_v2:<template>` drafts, restored with them, 4000-character cap. It never reaches the Rust checker, coaching, assistance marking or the reward ledger.
- `study.css` gained `.axiom-focus-view` quiet-chrome overrides and `.axiom-scratchpad` styles at the end of the file; existing tokens and layers are untouched.

## Try it

Open the app: the focus bar sits under the header on every destination. Toggle Focus view, start a 10-minute session, pause/end it, open Take a pause, then Practice → Working notes. Everything is opt-in and reversible; reload to confirm persistence.

## Verification

- `npx tsc --noEmit` passed; `pnpm --filter web build` passed (the existing raw-JS-chunk warning remains; no Rust or WASM change was made, so `cargo test --workspace` was not re-run).
- New `node apps/web/tests/focus-tools.cjs` (with `AXIOM_TEST_URL` at an isolated dev server, default port 39018) checks removal of the rotating bar and its copy, absence of live regions in the focus bar, focus-view toggle/persistence/navigation visibility, timer start-pause-resume-end, pause-panel honesty and close paths, scratchpad draft persistence, 320-pixel fit, and deterministic session completion via Playwright `page.clock` fast-forward. Evidence: `docs/evidence/focus-tools/`.
- Regression: `node apps/web/tests/ui-polish.cjs` passed all 10 UI checks after the changes (the focus bar occupies the former wellness slot; all five views, 320-pixel fit, skip link and contrast still pass). `node apps/web/tests/browser-review.cjs` passed all 12 core mathematical/learner checks (native/WASM parity, drafts, rewards, mobile fit, keyboard abacus), and `node apps/web/tests/positive-feedback.cjs` passed all 9 coaching checks. Run `browser-review.cjs` with `AXIOM_TEST_URL` set; its default port is 3000, unlike the other suites' 39018.

## Boundaries and remaining questions

- These tools are universal-design focus supports, not ADHD treatment; the pause panel says so directly. Do not add outcome claims, neuroplasticity language, auto-interruptions or mandatory timers.
- `GamerProgressTracker` (trophy/rank display in Progress) was reviewed and left unchanged: it makes no scientific claim, but its emoji/trophy-gamified tone sits against ADR-0002 restraint and source-of-truth section 17's anti-pressure stance. An owner decision should confirm whether to keep, soften or replace it.
- Not done here: focus timer integration with the future brain-gym session contract (PRODUCT-VISION), scratchpad in `ChallengeWorkshop`/lessons, screen-reader pass, and a full WCAG audit.
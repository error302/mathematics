# ADR-0006 — Attention-friendly study tools

15 October 2026. Requested by the project owner ("continue building… better ideas of how the UI should look for anyone to learn math effectively, especially for people with ADHD").

## Context

The active `WellnessAffirmationBar` rotated eight pseudo-scientific affirmations ("You are vibrating at the frequency of insight", "Manifest wellness through stillness", "Calm focus dissolves confusion. You have all the time and intelligence you need.") every 14 seconds inside an `aria-live="polite"` region on every screen, with a "Nervous System Reset" breathing modal. This conflicted with the PRODUCT-VISION.md claims boundary (no neurological or wellness-effect claims, no effortless-mastery promises), violated WCAG 2.2.2 (moving content lasting over five seconds with no pause control), contradicted ADR-0002's calm academy identity, and is precisely the irrelevant automatic motion that harms attention. Separately, source-of-truth section 5 requires a scratchpad in the practice workspace, which did not exist.

## Decision

Remove the wellness bar and `wellness.css` entirely. Add honest, opt-in, learner-controlled focus supports. These are ordinary interface preferences aligned with universal design for attention: reducing distraction, externalising working memory, and giving explicit control over pace. They are not ADHD diagnosis or treatment and make no neurological claim.

### FocusTools bar (`FocusTools.tsx`, `focusTools.css`)

A static bar below the header with three tools. Nothing auto-rotates, auto-starts, or interrupts typing; there are no `aria-live` regions that update on their own.

1. **Focus view** toggle, persisted in `axiom_focus_view`. Adds `axiom-focus-view` on the app root: flat background, XP-cap detail and footer hidden, card glows removed. All five navigation destinations, honest draft/availability labels and study content stay visible. Chrome returns exactly when toggled off.
2. **Optional focus session timer.** Learner chooses 10/20/30/45 minutes and starts it explicitly. Pause/resume/end are always available; nothing else happens while it runs (`role="timer"`, no live announcements). On completion, one polite `role="status"` message appears: "Session finished. A short break may help, or keep going—either is fine." with Keep going and Start another session actions. No modal, sound, penalty or XP effect. Per PRODUCT-VISION, a duration is a preference, not a pressure timer.
3. **Take a pause** panel. Manually opened dialog with a text breathing pace (four seconds in, six out), an explicit boundary statement ("an everyday pause habit—not medical advice, treatment, or ADHD care—and it is never required before practice"), Escape/close support and focus handling. The countdown is `aria-hidden` text with a screen-reader text alternative; reduced-motion users get the same content.

### Working notes scratchpad (`PracticeWorkspace.tsx`)

A collapsed `<details>` "Working notes" region after the answer form (source-of-truth section 5 practice workspace contract). The textarea is labelled "Working notes (kept on this device; never graded)", persists in the existing `axiom_practice_draft_v2:<template>` draft as `scratch`, is restored with the draft, capped at 4000 characters, and never affects Rust checking, coaching, assistance marking or rewards. Collapsed by default keeps a single task in focus while making working-memory support one click away.

## Boundaries

No medical, neurological, or ADHD-treatment claim is introduced; the copy states this directly. No timer runs unless the learner starts one; no content moves on its own; no reward, mastery or streak logic changed. Rust mathematical authority, replay identities, reward ledger rules, brand assets and existing learner data are untouched. `GamerProgressTracker` was reviewed and left unchanged this pass; its local trophy display makes no scientific claim, though its gamified tone versus ADR-0002 restraint remains a documented open design question.

## Validation

`apps/web/tests/focus-tools.cjs` checks: wellness bar and its copy are gone; the focus bar has no live regions; focus view toggles, hides XP/footer chrome, keeps navigation visible and persists; timer start/pause/resume/end; pause panel honest copy and close paths; scratchpad draft persistence; 320-pixel fit; and deterministic session completion via Playwright clock fast-forward. Evidence in `docs/evidence/focus-tools/`. This is not a clinical evaluation or a complete WCAG audit.
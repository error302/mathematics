# ADR-0004 — Encourage the learner and make mistakes useful

Date: 10 October 2026, Africa/Nairobi. The owner requested praise whenever they solve something or learn through mistakes, and useful hints or demonstrations when an answer is wrong.

Give a visible acknowledgement after every check. Praise a correct result, a successful revision after an actual earlier mistake, and constructive use of support. Acknowledge predictions, written reasoning, following a worked demonstration and checking an explanation without asserting those actions establish understanding. Do not call someone a genius, claim their brain changed, invent mastery or say an answer is “close” when the checker has not established that.

Keep the academy palette and reading layout. Encouragement is a short, readable text callout inside mathematical feedback, with restrained success/support accents. No compulsory sound, flashing/confetti, streak threats or loss of practice access. Keyboard and reduced-motion users get the same recognition. Repeated correct checks and capped XP still receive acknowledgement, with no duplicate reward.

Separate the mathematical judgement from coaching. Rust owns bounded exact hints, worked calculations and reference visuals in `coaching.rs`, exposed through WASM and the existing worker. This independent `coaching-v1` material does not alter checker results or historical exercise replay identities. Missing/inconsistent reference data yields unavailable help, never a fabricated solution.

After a mathematical mismatch, automatically offer a concept-specific first hint. Full solutions and answer beads appear only after “Show me how.” Learner-requested support is recorded in local drafts; later supported success is still praised. A review requested after an already-correct answer does not retroactively turn that earlier attempt into an assisted one. Input-format and unsupported-checker states are distinguished from mathematical failure. Editing an answer clears stale success feedback.

The lesson prediction loop has constructive feedback, concrete representation hints and a working link to the authored example. A revised correct prediction gets a recovery message. Writing an explanation or viewing a self-check receives an acknowledgement of that observed action, not an unreviewed correctness claim.

Acceptance: actual wrong → hint → retry → correct journeys, correct supported attempts, syntax edits, repeat/cap-safe reinforcement, exact Rust worked steps and visuals, hidden mental-answer support boundaries, persisted assistance, lesson prediction recovery, mobile/keyboard and offline behavior. Content and pedagogical review remain draft release gates.

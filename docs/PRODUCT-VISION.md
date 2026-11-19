# AXIOM product vision — owner direction

Accepted direction: 10 October 2026, Africa/Nairobi. This records the project owner's direct request and extends the source of truth. It does not declare unimplemented features complete.

## Identity and ambition

**Mathematics, the language of the universe.**

An academy for curious and ambitious minds: help learners build mathematical fluency, insight and proof ability, from their present starting point toward advanced study. The owner's “mathematician wizard” expresses delight, confidence and ambition. It is not a guarantee that every user will master every subject, an accredited qualification, or a substitute for reviewed evidence.

Brand copy: “Discover patterns, ask better questions, and learn to think like a mathematician. Build toward mastery through understanding, practice, and proof—one idea at a time.” Mathematical patterns in nature, science and technology can motivate learning, but do not collapse pure mathematics into physics. Label mathematical theorems, physical models, observations and conjectures distinctly.

## Academy and brain gym

The academy provides coherent prerequisite-based learning. The **mathematical brain gym** is the complementary entry for purposeful practice: arithmetic, number sense, reasoning, explanation, concrete/abstract representation, retrieval and soroban work. The two modes share the Rust mathematics engine, content and evidence policies. No second grading engine or unearned mastery system.

Currently implemented homepage entries open: whole-number addition practice; the fraction comparison lesson and its reasoning draft; the interactive soroban. They are activity shortcuts, not a newly validated adaptive workout, scheduler or retention assessment. Sessions are untimed by default. Learners can select a starting point and take breaks; speed is not the definition of intelligence or mastery.

Future gym session contract: let learners choose a goal and optional duration; compose activities from reviewed prerequisite/skill mappings; distinguish independent, assisted, self-check and transfer attempts; preserve attempts/drafts; give a specific explanation and next step. A duration is a preference, not a pressure timer. Adaptive selection, trusted retrieval and session summaries must be implemented and validated before being advertised. Rewards recognize eligible practice, while mastery follows the existing multidimensional and delayed-evidence gates.

## Neuroplasticity and claims policy

The owner's interest in neuroplasticity is captured as a research direction. AXIOM does not currently measure neural change. Ordinary task improvement, XP, a streak or faster abacus answers do not establish neuroplasticity, higher general intelligence or transfer to unrelated skills. Do not claim neurological treatment, dementia prevention, universal cognitive enhancement or guaranteed mathematical mastery.

Primary studies illustrate the need to distinguish trained-task improvement from broad transfer: [Owen et al., Nature (2010)](https://www.nature.com/articles/nature09042) found gains in trained tasks without demonstrated transfer to the untrained tasks tested; [Kable et al., Journal of Neuroscience (2017)](https://pmc.ncbi.nlm.nih.gov/articles/PMC5546110/) found no advantage of the tested commercial cognitive training over its active comparison on the studied outcomes. Neither study evaluates AXIOM or rules out every learning benefit. These sources justify a cautious claims boundary, not a claim that mathematical learning has no value.

Evaluate mathematical outcomes first: accuracy on unseen problems, explanatory reasoning, retained knowledge and transfer within specified mathematical contexts. Any broader cognitive or neural claim requires a separately approved study, suitable comparison group, predefined measures, informed consent, attrition reporting and independent interpretation. Do not infer efficacy from an uncontrolled before/after score or one learner's experience.

## Acceptance and agent handoff

- Homepage displays the owner-approved phrase and speaks to curious and ambitious learners.
- Brain-gym entries open actual available activities and remain keyboard/mobile accessible.
- Draft lessons and planned roadmap entries remain honestly labelled; current proofs and adaptive workout features remain gated.
- No mandatory timer, neuroplasticity score, intelligence ranking, fabricated evidence, or scientific outcome promise.
- Preserve branding, exact Rust checking, local drafts, reward rules and source-of-truth acceptance criteria.
- Store new product decisions in ADRs and update both specification copies together. User-facing copy must reflect demonstrated capability, not future aspiration presented as current functionality.

Shared conversation reference supplied by the owner: https://chatgpt.com/share/6ac9677a-668c-83e9-843b-d4dcd72bb77e. The available web reader retrieved its title but no message body; browser access timed out. This amendment relies on the owner's direct message, not an inferred reading of that conversation.

## Implementation evidence — 10 October 2026

The homepage and PWA metadata now carry the approved identity. All three brain-gym entries were exercised in a fresh Chrome production-preview session. The UI suite passed 10 checks, including the gym routes, five-view mobile fit at 320/390/768/1440 pixels, keyboard skip navigation, search reset and preserved-history backup errors. TypeScript/Vite builds passed; the existing raw JavaScript chunk warning remains. Evidence: `docs/evidence/owner-vision-evidence.json` and `docs/evidence/ui-polish/ui-evidence.json`. Mathematical/adaptive/review capabilities are unchanged by these entry points.

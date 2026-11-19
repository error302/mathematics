# ADR-0002 — A focused academy study interface

10 October 2026. Requested by the project owner after the independent implementation review.

The existing UI used competing navy/purple/gold panels, oversized badges, a horizontally clipped navigation strip, verbose developer labels, and an 83-card wall that hid the twelve usable draft lessons. Several rendered lesson classes had no corresponding styles. Reward backups used browser alert dialogs and exposed long internal keys in the main reading flow.

Keep the existing pi logo, navy/gold identity, React/CSS stack, lesson sources and Rust engine. Use locally available Georgia for academic display headings and Segoe UI/system sans for reading and controls, with tabular figures for values. Do not add font/CDN dependencies to the offline shell.

Tokens: ink `#0d1521`, surface `#162231`, raised surface `#1e2d3f`, readable text `#edf1f5`, secondary text `#b5c1cd`, academy gold `#e6c580`. Gold is the primary action color; green/red communicate meaningful outcomes only. No decorative gradient badges.

Layout: compact brand/status header with five visible navigation destinations; a curriculum introduction pairing a concrete half/quarters diagram with an immediate lesson entry; available draft courses first and a restrained expandable roadmap; a centered lesson reading column; deliberate exercise/feedback hierarchy; quieter local reward history. All views remain honest about draft status and local-only data.

The focal visual is mathematics, not a duplicated glowing logo or marketing illustration. The diagram illustrates one half as two quarters; its text alternative states that equality. Course availability is real metadata, never fake learner progress. Labels name learner actions. Empty/error states point to a working next action.

Validate with actual screenshots, all-view 320-pixel checks, keyboard navigation, contrast measurements and existing browser mathematics/reward/draft regressions. Preserve source-of-truth requirements and brand file hashes. This polish does not imply full screen-reader certification or production readiness.

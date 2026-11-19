# ADR-0007 — Reproducible foundations: pinned toolchains, locked CI, dependency policy

15 October 2026. Continues Phase 2 of the source of truth (§28) following the independent review's ordered backlog item 1, "reproducible foundations".

## Context

The independent review (10 October 2026) required pinned Rust/Node/pnpm versions, real CI with locked dependency builds, dependency/license review, and documented deployment topology before trust-and-durability work. The repository had `Cargo.lock` and `pnpm-lock.yaml`, but `rust-toolchain.toml` tracked floating `stable`, Node/pnpm versions were unpinned, the GitHub workflow ran unlocked `cargo build`/`cargo test` only, and no license policy existed.

## Decision

1. **Pin toolchains at the versions actually used**: Rust 1.98.1 in `rust-toolchain.toml` (with the wasm32 target), Node 26.4.0 in `.nvmrc`, pnpm 10.34.6 as the `packageManager` field in the root `package.json` with `engines` bounds. Upgrades are deliberate edits to these files, reviewed like code.
2. **Locked builds everywhere**: root `test` script now runs `cargo test --workspace --locked`; CI uses `--locked` for build and test and `--frozen-lockfile` for pnpm.
3. **CI replaces the minimal workflow** with four jobs (`.github/workflows/ci.yml`): locked Rust build+test; `cargo-deny` policy gate; release WASM package build with size report and uploaded artifact; frozen-lockfile web type-check/build. `permissions: contents: read`.
4. **Dependency policy in `deny.toml`**, audited against `cargo metadata --locked` on 15 October 2026: 122 packages, all permissive (MIT/Apache-2.0/BSD family/BSL-1.0/Unicode-3.0/Unlicense and expressions over them). Advisories and yanked crates denied; unknown git sources denied; wildcards denied; our own private unlicensed workspace crates warned, not failed.
5. **Honest topology record** in `docs/operations/reproducible-builds.md`: local-only current state, planned §27 topology explicitly unimplemented, and a gap list (no CI run yet executed on GitHub, no Playwright suites in CI, no npm license gate).

## Boundaries

Nothing here implements identity, PostgreSQL durability, sync, signed packs or staging; those remain Phase 2 items 2+. Green CI, once it first runs, will not mean production readiness (§26.4). No third-party content license was chosen on the owner's behalf; workspace crates stay `UNLICENSED`/private until the owner decides.

## Validation

Local commands run on 15 October 2026 with the pinned toolchain: `cargo test --workspace --locked`, release `wasm-pack` build with size report, `cargo deny check` against `deny.toml`, and `pnpm --filter web build`. Evidence in `docs/evidence/phase2-reproducible-builds.json`. The CI workflow itself is written to these exact commands but has not yet executed on GitHub; treat its first real run as unverified.
# AXIOM Phase 2 reproducible foundations handoff

Continues Phase 2 (source of truth §28) with the independent review's ordered backlog item 1, "reproducible foundations". Decision record: ADR-0007. Operational record: `docs/operations/reproducible-builds.md`. Changes are local and uncommitted; the dev server for owner inspection runs at `http://127.0.0.1:39018/`.

## What changed

- **Toolchain pins:** `rust-toolchain.toml` now pins Rust **1.98.1** (with wasm32 target) instead of floating `stable`; new `.nvmrc` pins Node **26.4.0**; root `package.json` gains `packageManager: pnpm@10.34.6` and `engines` bounds, and its `test` script now runs `cargo test --workspace --locked`.
- **CI:** the minimal `.github/workflows/rust.yml` is replaced by `.github/workflows/ci.yml` with four jobs on push/PR to `main`: locked Rust build+test; `cargo-deny` gate (advisories, bans, licenses, sources); release WASM package build with size report and uploaded artifact; frozen-lockfile pnpm install + web type-check/build. `permissions: contents: read`.
- **Dependency policy:** new `deny.toml` audited against `cargo metadata --locked` (122 crates, all permissive) and `pnpm licenses list` (75 web packages, all permissive, including **CC-BY-4.0 KaTeX fonts** — a recorded attribution obligation for any public release).
- **Topology/gaps record:** `docs/operations/reproducible-builds.md` documents the honest local-only current state, the planned §27 topology as unimplemented, and open gaps (CI never yet executed on GitHub, no Playwright suites in CI, no npm license gate).

## Environment repair note

Pinning exposed that the pre-existing `1.98.1` rustup toolchain on this machine was corrupt (rustc present, cargo missing). It was reinstalled with `rustup toolchain install 1.98.1-x86_64-pc-windows-msvc --profile minimal --target wasm32-unknown-unknown`. If cargo suddenly reports "binary … not applicable to the … toolchain" after a toolchain pin, check `rustup toolchain list -v` and reinstall the pinned toolchain.

## Verification (15 October 2026, local)

- `cargo test --workspace --locked` under the pinned 1.98.1 toolchain: **82 tests passed, 0 failed** across the workspace (re-run to green after the manifest fixes below).
- `pnpm install --frozen-lockfile`: passed (no lockfile changes).
- `pnpm --filter web build`: passed (existing raw-JS-chunk warning only).
- `cargo deny check advisories bans licenses sources` (cargo-deny 0.20.2, matching the CI action): **passed, exit 0**, after the fixes below; only a warn-level duplicate `syn` v2/v3 note remains.
- Release `wasm-pack build crates/axiom-wasm --target web --release` to a temporary directory (the running dev server's dev-profile package was not touched): `axiom_wasm_bg.wasm` **706,012 bytes**, package total **736,679 bytes** — roughly a third of the 2.16 MB dev-profile binary, a useful release-size baseline.
- Browser suites were re-run earlier this session against the dev server (focus-tools 8, ui-polish 10, browser-review 12, positive-feedback 9 — all passing); they are unchanged by this pass.

## Defects found and fixed by the new gates

The first real `cargo deny` run immediately earned its place:

1. `axiom-types`, `axiom-api` and `axiom-storage` were missing `publish.workspace = true`, making them **accidentally publishable** with an invalid `UNLICENSED` SPDX string. Fixed.
2. Internal crates mixed direct `path` dependencies with workspace dependencies; the version-less path deps tripped the wildcard ban and cargo-deny's workspace resolution. All internal deps now use `*.workspace = true`, and the workspace `axiom-*` entries carry `version = "0.1.0"` alongside `path`. `--locked` builds unaffected.
3. **RUSTSEC-2024-0449**: `unsafe-libyaml` (via deprecated `serde_yaml`) is unmaintained with no safe upgrade. It feeds only the offline CLI/content YAML reader. Ignored in `deny.toml` with a recorded reason; follow-up is to migrate `axiom-content`/`axiom-cli` to a maintained safe-Rust YAML parser (e.g. `serde-saphyr`) and remove the ignore.
4. Environment repair: pinning exposed a corrupt local `1.98.1` rustup toolchain (rustc present, cargo missing); reinstalled with `rustup toolchain install 1.98.1-x86_64-pc-windows-msvc --profile minimal --target wasm32-unknown-unknown`.

## Boundaries and next work

CI is written to these exact commands but has not executed on GitHub (no push was made); its first real run is unverified evidence. Nothing here implements identity, PostgreSQL durability, sync or signed packs — the next ordered item is review backlog 2, "trust and durability" (OIDC/session identity, PostgreSQL repository and migrations, ownership-scoped queries, CSRF/origin protections, idempotency binding, controlled export/deletion, backup restore), which needs a real database and identity provider decision from the owner. Do not treat green local builds or this CI definition as phase completion (§26.4).
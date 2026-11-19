# Reproducible builds, toolchain pins, and dependency policy

Status: implemented 15 October 2026 (Phase 2 backlog item 1, "reproducible foundations"). This records the pinned toolchain, the locked verification commands, the dependency/license policy, and the honest current deployment topology. It does not claim deployment, staging or production readiness.

## Toolchain pins

| Tool | Pinned version | Pin location |
|---|---|---|
| Rust | 1.98.1 (+ `wasm32-unknown-unknown`) | `rust-toolchain.toml` |
| Node.js | 26.4.0 | `.nvmrc` (web CI reads it via `actions/setup-node`) |
| pnpm | 10.34.6 | `packageManager` field in root `package.json` (corepack/`pnpm/action-setup` read it) |
| Web browser target for suites | Installed Chrome via Playwright channel | `apps/web/tests/*.cjs` resolve Playwright from the repo or `AXIOM_RUNTIME_NODE_MODULES` |

Rust and JS dependency graphs are pinned by `Cargo.lock` and `pnpm-lock.yaml`. Workspace crates are private (`publish = false`, `license = "UNLICENSED"`); third-party code is crates.io/npm only.

## Locked verification commands

```bash
cargo build --workspace --locked
cargo test --workspace --locked
cargo deny check advisories bans licenses sources   # after `cargo install cargo-deny`
wasm-pack build crates/axiom-wasm --target web --release --out-dir ../../packages/axiom-wasm-web
pnpm install --frozen-lockfile
pnpm --filter web build
```

Browser suites need a local dev server and Chrome; see AGENTS.md. CI (`.github/workflows/ci.yml`) runs four jobs on every push/PR to `main`: locked Rust build+test, `cargo-deny` policy (deny.toml), release WASM package build with size report and uploaded artifact, and frozen-lockfile web type-check/build. CI does not yet run the Playwright suites (they need Chrome plus a live dev server); that is a documented gap, not a completed gate.

## Dependency policy (`deny.toml`)

Audited 15 October 2026 against `cargo metadata --locked`: 122 packages, all under permissive licenses (MIT, Apache-2.0, BSD-3-Clause, BSL-1.0, Unicode-3.0, Unlicense, and dual/triple expressions over those). The allow list in `deny.toml` matches that inventory. Policy: advisories and yanked crates denied; unknown git sources denied; crates.io only; wildcard requirements denied; multiple versions warned; our own private (`publish = false`) workspace crates skipped via `[licenses] private.ignore`.

The first real `cargo deny check` runs surfaced and resolved, on 15 October 2026:

- **Three crates were accidentally publishable** (`axiom-types`, `axiom-api`, `axiom-storage` lacked `publish.workspace = true`) and were therefore treated as third-party unlicensed crates. Fixed in their manifests.
- **Version-less path-only internal dependencies** were flagged as wildcard dependencies and failed cargo-deny's workspace-dependency resolution. Internal `axiom-*` workspace dependencies now carry `version = "0.1.0"` alongside `path`, and all member crates use `*.workspace = true` instead of direct `path` entries.
- **RUSTSEC-2024-0449** (`unsafe-libyaml`, unmaintained, via deprecated `serde_yaml` 0.9): ignored in `deny.toml` with a recorded reason. It is used only by the offline CLI/content YAML reader, never by learner-facing runtime paths. Follow-up: migrate `axiom-content`/`axiom-cli` to a maintained safe-Rust YAML parser (e.g. `serde-saphyr`), then remove the ignore.
- Remaining warning only: duplicate `syn` v2/v3 lock entries (warn-level, expected during the serde ecosystem's syn 3 transition).

Web-side inventory (15 October 2026, `pnpm licenses list`): 75 direct+transitive packages — MIT (65), ISC (6), Apache-2.0 (2), BSD-3-Clause (1), and CC-BY-4.0 (1: KaTeX's distributed fonts, a content license requiring attribution if those assets ship publicly). All permissive; the CC-BY-4.0 font attribution is an explicit release-time obligation, recorded here so it is not forgotten. The inventory is documented but not yet machine-enforced on the web side; adding an npm license gate is follow-up work.

## Deployment topology (honest current state)

Current: local development only. The web app builds to static assets served by Vite dev or `vite preview`; the Rust engine runs in the browser via WASM in a worker; all learner data is device-local (localStorage); the API crate exists but account routes fail closed outside explicit local-demo mode. There is no hosting account, no staging database, no identity provider, no signing key custody.

Planned (source of truth §27, not implemented): static/CDN frontend; container-hosted Rust API and ordinary worker; managed PostgreSQL with point-in-time recovery; private artifact storage; separately isolated proof hosts; stable HTTPS origin; explicit migration deployment step; separately versioned content releases with feature flags. Public launch gates remain in §28 cross-phase requirements.

## Gaps

- CI has not yet executed on GitHub from this repository state (no push was made); the workflow is written to the documented commands but its first real run is unverified.
- Playwright browser suites, storage-migration tests and service-worker upgrade tests are not in CI.
- npm license automation, SBOM generation and signed release artifacts are not implemented.
- No pin for the Playwright/Chrome browser pair used by local suites.
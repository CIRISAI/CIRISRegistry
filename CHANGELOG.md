# Changelog

All notable changes to **CIRISRegistry** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
with one project-specific rule: **MAJOR version bumps signal CEG wire-format
conformance breaks**. See [`RELEASE.md`](RELEASE.md) for the full versioning
discipline + release process.

| Series | CEG conformance |
|---|---|
| 1.x | FSD-002 v1.4.3 baseline |
| 2.x | CEG 0.2 |

Prior to v1.1.0 baseline tagging, per-feature commit history lives in
`git log`; the CHANGELOG starts from this baseline forward.

---

## [Unreleased]

Tracking under [#33 umbrella](https://github.com/CIRISAI/CIRISRegistry/issues/33):
substrate-sister adoption + CEG 0.2 conformance + fold-readiness for CIRISAgent.

Upcoming phases (in waterfall order):

- Phase 2 (v1.2.0) — DONE (this release). Pending follow-up: wire engine
  construction at boot path + enable `FEDERATION_DUAL_WRITE_ENABLED` in
  staging.
- **v1.3.0** — Edge wiring (CEG 0.2 §10.1 + connects #18): `PeerResolver` client
  + `holds_bytes` directory consumption + ContentFetch/ContentBody/ContentMiss
  round-trip with full-SHA verification (§10.1.1) + 24h TTL discipline (§10.1.2).
- **v1.4.0** — Crate-ify split: `ciris-registry-core` library (for CIRISAgent
  workspace cohabit with `ciris-lens-core` + `ciris-node-core`) + `ciris-registry`
  binary (standalone deployments).
- **v2.0.0** — CEG-0.2 conformance MAJOR + fold-readiness; closes #17 + #18 + #32.

---

## [2.2.0] — 2026-06-05

**Release-train R2 — §10.0 request-context layer (first half of #35).** Adds the CEG 0.2 §10.0 common-response surface as an additive, handler-untouched layer; the §10.0.1 error-body conversion (the 60-site refactor) follows as v2.2.1 (R2b) so the high-surface change gets its own validation gate.

- **New `api::error` module** — `ApiError` + the 13-value `ErrorCode` enum (`MALFORMED_REQUEST` … `WITNESS_DIRECTORY_UNAVAILABLE`) serializing to the §10.0.1 envelope `{error: {code, http_status, message, request_id, details}}`, with an `IntoResponse` impl + an `ApiError::from_status` adapter so the v2.2.1 site conversions are near-mechanical. Typed constructors (`malformed` / `not_found` / `unauthenticated` / …). 4 unit tests pinning code strings + status mapping against the spec table.
- **`request_context_mw`** — per-request UUID exposed via a task-local (so error bodies carry `request_id` without threading it through handlers) + sets `X-Request-Id`, `CEG-Version: 0.10`, and `X-CEG-Server-Time` (RFC 3339 millis per §0.5) on **every** response (success or error). Wired as the outermost layer over all routes.
- No handler error bodies changed yet (still legacy shapes until R2b); this release is purely additive — every response now carries the §10.0 headers + the conformant error type is in place.

**Rollback metadata:**
- **Digest**: (operator-resolvable via `crane digest …:v2.2.0`)
- **Migrations**: additive-only (no schema change)
- **Config**: none

---

## [2.1.4] — 2026-06-05

**Release-train R1 — pipeline canary (docs-only; zero binary-behavior change).** First of a staged release train (milestones v2.1.4 → v2.4.0) where each release is prod-validated before the next. v2.1.4 ships no binary change — its purpose is to validate the full commit → CI smoke-gate → `:latest` → watchtower → `/health` deploy pipeline is green end-to-end on a no-risk change before binary releases (#35/#34/#46) follow.

- **#42** — RELEASE.md gains a **Rollback** section answering the four safety questions (last-known-good digest source; forward-only/additive migration discipline + rollback floor; per-release config/env deltas; sister-stack compat matrix at `CIRISConformance/matrices/current.yaml`). Establishes the per-release rollback-metadata discipline (Digest / Migrations / Config lines) from v2.1.4 forward.
- Retired the stale ECS ansible playbook → `deploy/aws-archive/rollback.yml` + archive README (it referenced `amazon.aws.ecs_service_info` for an AWS deployment we don't run; was misleading during the v2.1.0 incident).

**Rollback metadata:**
- **Digest**: (filled post-CI from the published GHCR `sha256:`)
- **Migrations**: additive-only (no schema change — docs + version bump)
- **Config**: none

---

## [2.1.3] — 2026-05-30

**Substrate triple sync to Conformance-matrix canonical (post-v2.1.x cascade):**

| Crate | v2.1.2 | v2.1.3 | What landed upstream |
|---|---|---|---|
| `ciris-crypto` | v4.4.2 | **v4.4.3** | Workspace-level rusqlite drops macOS from `bundled`; macOS now links against `/usr/lib/libsqlite3.dylib` (CIRISVerify#45) |
| `ciris-keyring` | v4.4.2 | **v4.4.3** | Same workspace-level bump |
| `ciris-persist` | v3.6.3 | **v3.6.9** | Mach-O parity CI gate (CIRISPersist#141); bundled-libsqlite3 reopen on darwin closure ; verify pin bump |
| `ciris-edge` | v1.0.1 | **v1.1.3** | MINOR 1.0→1.1 cascade: routing-table FFI Leviculum gap-stub flips (CIRISEdge#44), scrub_signer routing (#137/#138 mirror), darwin libsqlite3 cascade closure (CIRISEdge#50 darwin follow-on) |

### macOS-only fix; doesn't affect Registry's Linux Docker image

The upstream cascade closed a darwin-only cohabitation bug: when both `ciris-persist` and `ciris-edge` Python wheels statically bundled libsqlite3, dyld loaded two separate copies in the same Python process and edge's never-initialized copy crashed (SIGSEGV). Fix: drop macOS from rusqlite's `bundled` feature so both wheels share `/usr/lib/libsqlite3.dylib` via dyld.

**Registry runs on Linux (Debian bookworm Docker image) and ships as a standalone binary, not a Python wheel**. The macOS-cohabit failure mode doesn't apply. Our [`libsqlite3-sys` bundled feature](#211--2026-05-30) from v2.1.1 stays — correct for our Linux standalone-binary case.

### Verification

- `cargo check --workspace` clean
- `cargo test --workspace` — 150/150 passing
- `cargo tree -i ciris-{persist,keyring,edge}` confirms single unified version each across the dep graph

---

## [2.1.2] — 2026-05-30

**Hotfix for v2.1.1 — fixes the CI smoke-gate's `:latest` promote step.**

v2.1.1's libsqlite3 fix worked correctly (CI smoke test confirmed "OK: binary loads cleanly" + boot got to "Starting CIRISRegistry v2.1.1" / "Loaded configuration"). But the new `:latest` promote step I added in v2.1.1 had a bug: hand-rolled `LATEST_TAG="${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest"` doesn't lowercase the IMAGE_NAME, and Docker rejects mixed-case repository names:

```
Error parsing reference: "ghcr.io/CIRISAI/CIRISRegistry:latest" is not a valid repository/tag:
invalid reference format: repository name (CIRISAI/CIRISRegistry) must be lowercase
```

The `docker/metadata-action` auto-lowercases its outputs (which is why the SHA_TAG worked), but my hand-rolled construction in the Promote step didn't. Fix: explicit `tr '[:upper:]' '[:lower:]'` on IMAGE_NAME before constructing LATEST_TAG.

Net effect: v2.1.1's `:latest` was never promoted (production still on the broken v2.1.0 image until this fix builds + smoke-passes + promotes). v2.1.2 = same code as v2.1.1 + the lowercase fix.

---

## [2.1.1] — 2026-05-30

**HOTFIX — production crash-loop in both regions on missing `libsqlite3.so.0` after v2.1.0 :latest auto-deploy.**

### What broke

v2.1.0's substrate triple migration pulled `ciris-persist v3.6.3+` which transitively pulls `rusqlite v0.31.0` → `libsqlite3-sys v0.28.0`. By default, `libsqlite3-sys` dynamic-links against system `libsqlite3.so.0`. The Dockerfile runtime stage was last updated for v2.0.0 (when persist was at v3.3.1 with bundled libsqlite3) and did NOT install `libsqlite3-0`. Result:

```
$ docker logs ciris-registry --tail 1
/app/ciris-registry: error while loading shared libraries:
libsqlite3.so.0: cannot open shared object file: No such file or directory
```

Watchtower auto-pulled the new `:latest` on both `registry-us` and `registry-eu` within seconds of publish; both containers crash-looped (260ms/restart, 64+ restarts on each region). All 4 public registry hostnames returned 502.

### Two fixes

**1. Cargo.toml — `libsqlite3-sys` with `bundled` feature**

Forces all transitive consumers (rusqlite, ciris-persist, ciris-edge, ciris-verify-core) to share a single bundled libsqlite3 build via Cargo's feature-union semantics. Binary statically links sqlite3; runtime no longer requires `libsqlite3.so.0`.

**2. Dockerfile — `libsqlite3-0` in runtime apt (belt-and-suspenders)**

Both root `Dockerfile` and `rust-registry/Dockerfile` now `apt-get install libsqlite3-0` in the runtime stage. Even if the bundled feature ever stops propagating (e.g., a future dep adds `default-features = false` and breaks the union), the runtime image still has the system lib.

### CI smoke gate added (prevents recurrence)

`.github/workflows/docker.yml` rewritten to gate `:latest` push behind a smoke test:

1. Build with all tags EXCEPT `:latest`
2. Push SHA + branch tags
3. **Smoke-test the SHA-pinned image** — `ldd /app/ciris-registry` MUST NOT show "not found"; binary's first 5 lines of stderr MUST NOT contain "loading shared libraries" or "cannot open shared object"
4. Only if smoke passes, retag and push `:latest`

This closes the v2.1.0 incident class: a broken binary cannot reach `:latest` even if the build step succeeds.

### CI process audit (gap surfaced by this incident)

The build-push step had been emitting all tags (including `:latest`) in a single push, with the post-build "Sign + publish registry BuildManifest" step running AFTER. So an image that passed build but failed the publish step still went live on `:latest`. The new smoke-gate fixes the immediate crash-class; the publish step still runs after but no longer determines `:latest` promotion.

### Rollback procedure (for ops)

To pin back to the last-known-good image while this fix builds:

```bash
# Find the last-known-good digest (image published before v2.1.0)
gh api orgs/CIRISAI/packages/container/cirisregistry/versions \
  --jq '.[] | select(.metadata.container.tags[] | contains("fd37a30") or contains("2.0.0")) | "\(.created_at)  \(.name)  \(.metadata.container.tags | join(\",\"))"' \
  | head -3

# Pin docker-compose to the digest, OR temporarily disable watchtower
# auto-pull on the registry containers until v2.1.1 publishes.
```

The last-known-good build was the v2.0.0 release at commit `fd37a30f` (published 2026-05-29T19:21Z per watchtower logs).

### Verification

- `cargo check --workspace` clean
- 150/150 tests passing
- The bundled libsqlite3 build adds ~30s to build time and ~500KB to the binary (acceptable)

---

## [2.1.0] — 2026-05-30

**Substrate triple migration to the Conformance-matrix canonical version set + drop the v1.3.0 PersistPqcAdapter local stopgap.**

Per [CIRISConformance/matrices/current.yaml](https://github.com/CIRISAI/CIRISConformance/blob/main/matrices/current.yaml) (the cohabit-set canonical version reference), the substrate triple has converged at:

| Crate | Old (v2.0.0) | New (v2.1.0) | Notes |
|---|---|---|---|
| `ciris-crypto` | v4.1.0 | **v4.4.2** | |
| `ciris-keyring` | v4.0.0 | **v4.4.2** | Unifies with what persist/edge now pull transitively |
| `ciris-persist` | v3.3.1 | **v3.6.3** | Multimedia tier #134 + trust_scoring_capsule #128/#129 + libsqlite3 closure chain. **NOTE**: Conformance matrix specifies v3.6.4 (carries the additional #130 list_holders_json LOCAL-holdings fix), but edge v1.0.1 transitively pins v3.6.3 — Cargo cannot unify across distinct git-tag pins from the same source. Pinning to v3.6.3 aligns the graph; the v3.6.3→v3.6.4 gap is internal to Persist's blob-holder query and not surfaced to Registry consumers. Will close when edge v1.0.2+ bumps its transitive pin. |
| `ciris-edge` | v0.18.0 | **v1.0.1** | MAJOR bump 0.x → 1.x; carries CIRISEdge#50 durable-send SIGSEGV fix (libsqlite3 auditwheel exclude). |

### Local PqcSigner stopgap deleted

v1.3.0 shipped `src/crypto/persist_signer.rs` — a `PersistPqcAdapter` wrapping `ciris_crypto::MlDsa65Signer` for `ciris_keyring::PqcSigner` because CIRISVerify#39's upstream `impl PqcSigner for MlDsa65Signer` (shipped in keyring v4.1.0) was unreachable from our dep graph at the time (cohabit set still pinned keyring v4.0.0 transitively).

The cohabit-set convergence at v4.4.2 across persist/edge/verify-core makes the upstream impl reachable directly. **`src/crypto/persist_signer.rs` deleted**; `HybridCrypto::build_persist_local_signer` now wraps `MlDsa65Signer` in `Arc<dyn ciris_keyring::PqcSigner>` using the upstream impl.

### Bin Cargo.toml also bumped

`ciris-registry` bin had its own `ciris-persist` dep pinned at v3.3.1 (Phase 3-followup leftover). Bumped to v3.6.3 to match the lib's pin and unify the graph.

### Open issues now visible (post-v2.0.0)

The umbrella #33 closed at v2.0.0; remaining open issues are governance + future-spec work:

| # | Topic |
|---|---|
| #40 | CEG 0.4 codification — event_listing + live_stream sub_kinds (downstream from CIRISNodeCore#25) |
| #36 | CEWP orientation — informational |
| #35 | CEG §10.0.1 error envelope codes (post-v2.0.0 polish) |
| #34 | CEG §10.3.1 STH consistency-proof at cosign admission (post-v2.0.0 polish) |
| #31 | CEG v0.2 candidates from 240-story stress test (spec) |
| #30 | v1.5 grammar candidates from 283-story stress test (spec) |
| #25 | Compliance umbrella — federation taxonomy expansion (spec) |
| #22 | RFC: evidence_refs binding to SEED_DIMENSIONS (spec) |
| #20 | v1.5+ wire-format candidates from trio mapping (spec) |

None block v2.1.0. The spec-only items (#22 / #25 / #30 / #31 / #40) accumulate in the CEG-spec pipeline; the implementation items (#34 / #35) are post-v2.0.0 polish that bin development can pick up when implementer attention is available.

### Verification

- `cargo check --workspace` clean
- `cargo test --workspace` — 150/150 passing
- `cargo tree -i ciris-keyring` shows single v4.4.2 across the dep graph
- `cargo tree -i ciris-persist` shows single v3.6.3 (no more v3.3.1 leftover; no v3.6.4 conflict)

### Why this is a MINOR not a MAJOR

Per CEG-conformance versioning rule: MAJOR bumps signal CEG wire-format conformance breaks. v2.1.0 carries no CEG-spec change (CEG remains at 0.3); only a substrate-dep version update + an internal cleanup. The Registry CCC + CCP + CCS conformance shape is unchanged.

---

## [2.0.0] — 2026-05-29

**Phase 5 of #33 — CEG 0.2 conformance MAJOR. Substrate-sister adoption complete. `ciris-registry-core` is fold-ready for CIRISAgent's workspace.**

This release is the documentation tag declaring **CEG 0.2 conformance** as the cumulative outcome of Phases 0–4. The codebase delta vs `v1.4.0` is just the version bump + this CHANGELOG entry — every substantive change landed in the preceding minors. Major version is bumped per the project's versioning rule: **MAJOR signals CEG wire-format conformance breaks**.

### Cumulative achievement (v1.1.0 → v2.0.0)

Three load-bearing transitions in one release line:

1. **Substrate-sister adoption** (Phases 1–3-followup):
   - `ciris-crypto` v1.14.0 → **v4.1.0** (4-major catchup; closes long-standing pkcs8 transitive-pin lag tracked at #10)
   - `ciris-persist` **NEW** at v3.3.1 (federation directory + blob storage + signing surface)
   - `ciris-edge` **NEW** at v0.18.0 (Reticulum-native transport substrate)
   - `ciris-keyring` **NEW** at v4.0.0 (matches cohabit-set transitive pins)

2. **CEG 0.2 §10.1 wire-format conformance** (Phase 3 + Phase 3-followup):
   - `edge_transport` module with TTL filtering (§10.1.2), full-SHA verification (§10.1.1), ContentMiss-feedback emission, three-layer `agent_files` composition (§8.1.6)
   - Persist `Engine` constructed at boot with `LocalSigner` derived from HybridCrypto's existing keypair
   - `FederationDirectory` on `AppState`; NoOp by default, real Persist client when `FEDERATION_DUAL_WRITE_ENABLED=true`
   - `/v1/agent_files/{kind}` endpoint live — queries federation directory + composes three-layer trust + returns layered `AgentFileAttesterEntry` lists
   - `PersistPqcAdapter` (local stopgap until cohabit-set unifies on keyring v4.1.0) bridges ciris-crypto's `MlDsa65Signer` to ciris-keyring's `PqcSigner` trait

3. **Crate-ify split for CIRISAgent fold-readiness** (Phase 4):
   - Workspace shape: `ciris-registry-core` (lib) + `ciris-registry` (bin) at `rust-registry/`
   - `ciris-registry-core` is the cohabit-target library — CIRISAgent's workspace can pull it alongside `ciris-lens-core` + `ciris-node-core`
   - Single `Cargo.lock` at workspace root; shared `[profile.release]`
   - Bin is the thin shim — 190-line `main.rs` constructs the gRPC + HTTP server stack from the lib

### CEG 0.2 conformance posture

What this release CONFORMS to:

- **§5.2** — Registry-emitted attestation strings use the mechanism-only form (`attestation:self_verify` etc.); no `attestation:l{N}:*` strings in Registry code (verified via grep)
- **§10.1** — transport substrate read-side helpers shipped + Engine wired through federation directory + endpoint composition live
- **§5.6.7** — `agent_files:*` joint claim with CIRISNodeCore; three-layer trust composition per §8.1.6 implemented + anti-tricking guarantee encoded as test assertions
- **§5.2.1** — `provenance:build_manifest:*` per-target + per-locale Merkle composition surfaces shipped in earlier work (FSD-002 v1.4.1)

What this release ACKNOWLEDGES as v2.0.x follow-up work (known gaps; documented):

- **§10.3.1** — STH cosignature endpoint accepts cosignatures without verifying consistency proof against prior STH. `witness_quorum_met` is currently "quorum on string" not "quorum on log consistency." Tracked at [#34](https://github.com/CIRISAI/CIRISRegistry/issues/34).
- **§10.0.1** — HTTP error responses don't conform to the standard error-envelope shape (\`code\`, \`http_status\`, \`message\`, \`request_id\`, \`details\`). Tracked at [#35](https://github.com/CIRISAI/CIRISRegistry/issues/35).
- **Steward-triple identity vocabulary** — `agent_files` endpoint's three-layer composition queries an empty `steward_triple` set in v2.0.0 (no `federation_keys.identity_type = 'steward_triple_member'` rows yet); awaiting CIRISPersist#102 vocabulary extension. §8.1.6 anti-tricking guarantee is conservatively-correct (no canonical promotion ever) but Layer 1 stays empty until the vocabulary lands.

Per the versioning rule, these gaps land as **v2.0.x minor + patch updates**, not as a future MAJOR — the v2.x series is the CEG-0.2 conformance line.

### CIRISAgent fold-readiness

CIRISAgent's Cargo.toml can now declare:

```toml
ciris-registry-core = { git = "https://github.com/CIRISAI/CIRISRegistry", tag = "v2.0.0" }
```

…alongside the existing `ciris-lens-core` and `ciris-node-core` cohabit deps. All three pull from the substrate triple (`ciris-persist`, `ciris-crypto`, `ciris-edge`); all three implement their respective slices of the CEG namespace.

### Closes

- [#17](https://github.com/CIRISAI/CIRISRegistry/issues/17) — substrate-conformance plan + crate-ify as ciris-registry-core ✅
- [#18](https://github.com/CIRISAI/CIRISRegistry/issues/18) — public installer landing + agent_files Contribution surface ✅ (endpoint structural integration; the actual installer-page deployment is operational work outside this repo)
- [#32](https://github.com/CIRISAI/CIRISRegistry/issues/32) — SEED_DIMENSIONS.yaml v1.0 → v1.1 refresh (Registry-side dep tracking; the YAML lives in CIRISAgent so the actual file refresh lands there) ✅

### Verification

- `cargo check --workspace` clean
- `cargo test --workspace` — 150/150 passing

### What's NEXT after v2.0.0

The umbrella [#33](https://github.com/CIRISAI/CIRISRegistry/issues/33) closes with this commit. Post-v2.0.0 work:

- [#34](https://github.com/CIRISAI/CIRISRegistry/issues/34) + [#35](https://github.com/CIRISAI/CIRISRegistry/issues/35) — the two CEG 0.2 conformance gaps named above
- Content split of the lib/bin boundary (move bin-only modules from lib to bin)
- CIRISAgent-side fold-in workstream (separate repo)

---

## [1.4.0] — 2026-05-29

**Phase 4 of #33 — Workspace split: `ciris-registry-core` (lib) + `ciris-registry` (bin).**

The lib half is the cohabit-target library that CIRISAgent will pull into
its workspace alongside `ciris-lens-core` + `ciris-node-core` post-fold.
The bin half is the thin shim: a `main.rs` that constructs the gRPC + HTTP
server stack using everything from the lib.

### Layout

```
rust-registry/
├── Cargo.toml              # workspace root
├── Cargo.lock              # single workspace lock
├── ciris-registry-core/    # LIB
│   ├── Cargo.toml
│   ├── build.rs            # proto codegen (path updated to ../../protocol/)
│   ├── migrations/         # sqlx migrations
│   ├── src/
│   │   ├── lib.rs          # NEW — module declarations
│   │   ├── api/  config.rs  crypto/  db/  edge_transport/  error.rs
│   │   ├── federation/  middleware/  proto types  services/
│   │   └── ... (all existing modules)
│   └── tests/              # integration tests stay with lib
└── ciris-registry/         # BIN
    ├── Cargo.toml          # depends on ciris-registry-core
    └── src/
        └── main.rs         # gRPC + HTTP server entrypoint
```

### Minimal-blast-radius approach

v1.4.0 ships the workspace **structure** with the source tree wholesale-
moved to the lib crate. The lib's public surface is therefore broader than
CIRISAgent strictly needs (it includes gRPC/HTTP server modules CIRISAgent
won't link against). Intentional: structural split first, content split
later. A future minor (likely v1.4.x or v1.5.0) will relocate bin-only
modules (`api`, `services`, `middleware`, `rate_limiter`, `play_integrity`,
`app_attest`) to the bin crate, keeping only the cohabit-target surface in
the lib.

### Workspace-level configuration

- `[profile.release]` (`lto = true`, `codegen-units = 1`, `strip = true`)
  moved to workspace root per Cargo's "profiles for the non-root package
  will be ignored" warning.
- `[workspace.package]` carries the shared author / license / edition /
  rust-version / repository values (sister crates can `version.workspace
  = true` if they want to inherit).
- `resolver = "2"` declared at workspace.
- Cargo.lock stays at workspace root — single resolver run across both
  crates.

### Bin `Cargo.toml` shape

The bin crate's `Cargo.toml` is thin: it pulls `ciris-registry-core` via a
path dep + just enough direct deps for the boot stack (tokio, tonic,
metrics, ciris-persist for `Engine::with_signer`). All other deps come
transitively through the lib.

### Main.rs shim

The bin's `main.rs` previously had ~210 lines of module declarations + boot
sequencing. The module declarations now live in `lib.rs`; the bin's main.rs
imports them via `use ciris_registry_core::{...}` and contains only the
boot sequencing.

### CIRISAgent fold-readiness

CIRISAgent's `Cargo.toml` can now declare:

```toml
ciris-registry-core = { git = "https://github.com/CIRISAI/CIRISRegistry", tag = "v1.4.0" }
```

…alongside the existing `ciris-lens-core` and `ciris-node-core` cohabit
deps. The lib's `pub` surface exposes everything Agent needs:
`federation::FederationDirectory`, `crypto::HybridCrypto`,
`edge_transport::compose_trust_layers`, `build_manifest::BuildManifest`, etc.

### Build script path update

`ciris-registry-core/build.rs` updated to reference `../../protocol/`
(two levels up from the lib crate's `Cargo.toml`, reaching the repo root's
`protocol/` directory) instead of the old `../protocol/`.

### Verification

- `cargo check --workspace` clean (lib has 10 pre-existing warnings;
  bin has 0)
- `cargo test --workspace` — 150/150 passing (same as v1.3.0, all lib
  unit tests + 4 integration test binaries; the bin has 0 tests, as
  expected for a thin shim)

### Closes (half of) #17

CIRISRegistry#17 — "crate-ify as ciris-registry-core" half closes with
this commit. The "prepare to fold into CIRISAgent" half is satisfied
structurally: CIRISAgent can now pull `ciris-registry-core` directly as
a workspace dep.

---

## [1.3.0] — 2026-05-29

**Phase 3-followup of #33 — Engine wired at boot, AppState carries federation directory, `/v1/agent_files/{kind}` composes via edge_transport. CEG 0.2 §10.1 read-side helpers now live end-to-end.**

### Deps bumped

- `ciris-crypto` v4.0.0 → **v4.1.0** (CIRISVerify#39 release)
- `ciris-keyring` **NEW** at v4.0.0 (matches what ciris-persist v3.3.1 / ciris-edge v0.18.0 / ciris-verify-core v4.0.0 transitively pull; Cargo cannot unify across distinct git-tag pins from the same source so we use v4.0.0 to match)
- `ciris-persist` v3.3.0 → **v3.3.1**

### Adapter: `crate::crypto::persist_signer`

CIRISVerify#39 shipped `impl PqcSigner for MlDsa65Signer` in `ciris-keyring` v4.1.0, but the upstream cohabit set (edge / persist / verify-core) all still pin keyring v4.0.0 transitively. We tried `[patch."https://github.com/CIRISAI/CIRISVerify"]` to force-unify the graph to v4.1.0 — Cargo refused with "patch must point to a different source." So Registry ships its own local adapter as a stopgap:

```rust
// src/crypto/persist_signer.rs
pub struct PersistPqcAdapter { inner: MlDsa65Signer }

#[async_trait]
impl PqcSigner for PersistPqcAdapter {
    fn algorithm(&self) -> PqcAlgorithm { PqcAlgorithm::MlDsa65 }
    fn hardware_type(&self) -> HardwareType { HardwareType::SoftwareOnly }
    async fn public_key(&self) -> Result<Vec<u8>, KeyringError> { ... }
    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, KeyringError> { ... }
    async fn attestation(&self) -> Result<PlatformAttestation, KeyringError> { ... }
    fn current_alias(&self) -> &str { "ciris-registry/PersistPqcAdapter" }
    fn storage_descriptor(&self) -> StorageDescriptor { StorageDescriptor::InMemory }
}
```

Orphan rules satisfied: local struct (we own) + foreign trait (keyring's). Same net effect as the v4.1.0 upstream impl; the file deletes cleanly once upstream cohabit set pulls v4.1.0.

### `HybridCrypto::build_persist_local_signer`

New method on `HybridCrypto`:

```rust
pub fn build_persist_local_signer(&self) -> Result<Arc<ciris_persist::signing::LocalSigner>> {
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&self.ed25519_seed);
    let pqc_signer = MlDsa65Signer::from_seed(&self.mldsa_seed)?;
    let pqc_signer_arc = persist_signer::arc_persist_pqc(pqc_signer);
    Ok(Arc::new(LocalSigner::from_parts(
        signing_key,
        self.key_id.clone(),
        Some(pqc_signer_arc),
        Some(self.key_id.clone()),
    )))
}
```

To support this, `HybridCrypto` now retains `mldsa_seed: Vec<u8>` so the PQC signer can be reconstructed cleanly for the LocalSigner without consuming Registry's own hybrid-signing path.

### Engine at boot (main.rs)

After building `HybridCrypto`, main.rs now constructs:

```rust
let local_signer = crypto.build_persist_local_signer()?;
let engine = ciris_persist::engine::Engine::with_signer(
    local_signer,
    &settings.federation.persist_dsn,
).await?;
let federation_directory = crate::federation::build_client(
    Some(Arc::new(engine)),
    &settings.federation,
);
```

The federation directory is then passed through to `api::http::serve` and ends up on `AppState`.

### Config: `FEDERATION_PERSIST_DSN`

New env var per `FederationSettings::persist_dsn`. Defaults to `sqlite::memory:` (ephemeral; dev only — federation directory lost on restart). Production overrides to a postgres URL via `FEDERATION_PERSIST_DSN=postgres://...`, typically the same database Registry already uses for its own tables (Persist runs its own migrations and cohabits cleanly per CIRISPersist's design).

### Endpoint live: `/v1/agent_files/{kind}`

Previously returned hard-coded empty lists ("v1.4 interim"). Now:

1. Synthesizes attested-key as `agent_files:{kind}:{platform_or_target}`
2. Queries `state.federation.list_attestations_for(&attested_key)`
3. Composes three-layer trust via `edge_transport::compose_trust_layers`
4. Returns layered `AgentFileAttesterEntry` lists

When the federation directory is NoOp (default `FEDERATION_DUAL_WRITE_ENABLED=false`) or the substrate has no matching attestations, the response is still empty — same shape as before. When the flag is enabled and there's real data, the endpoint composes for real.

### Limitations acknowledged

- **steward_triple set is empty in the composition call**. Production wiring needs to load it from `federation_keys` rows with `identity_type = 'steward_triple_member'` (TBD vocabulary per CIRISPersist#102). Until then, no attester qualifies as Layer 1 canonical regardless of attestation data — the §8.1.6 anti-tricking guarantee is conservatively-correct (no canonical promotion) but Layer 1 stays empty.
- **vote_weights map is empty**. NodeCore P4 read API isn't wired yet; Layer 3 stays empty.
- **file_sha256 stays empty** in the response entries. The attestation's `evidence_refs[]` carries the SHA per CEG §5.6.7 but isn't extracted here yet — Phase 4 (crate-ify) is the natural moment to lift this into a typed `agent_files_envelope` extractor.

### CIRISVerify#39 cross-ref

[CIRISVerify#39](https://github.com/CIRISAI/CIRISVerify/issues/39) is closed via v4.1.0. Registry's local adapter at `src/crypto/persist_signer.rs` is the temporary bridge until the cohabit set (edge / persist / verify-core) all release with keyring v4.1.0 pinned. When that lands, the adapter file deletes and Registry adopts the upstream impl directly.

### Tests

150/150 passing (same as v1.3.0-rc.1). `cargo check` clean.

---

## [1.3.0-rc.1] — 2026-05-29

**Phase 3 (partial) of #33 — Edge transport read-side helpers shipped; runtime integration deferred to v1.3.0 final.**

This release-candidate marks the "read-side helpers complete, runtime
integration not yet wired" state. The CEG §10.1 normative rules are
implemented as pure-logic helpers with unit tests proving each invariant;
lighting them up against live federation data requires the
Phase-2-follow-up (Engine construction at boot + AppState wiring + DSN
config) which lands separately.

### New module: `src/edge_transport/`

Four sub-modules, each addressing a CEG §10.1 normative rule:

| Module | Spec section | What it does |
|---|---|---|
| `ttl` | §10.1.2 | 24h TTL filtering on `holds_bytes:sha256:{prefix}` attestations; `filter_fresh_holders` + `classify_freshness` |
| `verify` | §10.1.1 | Full SHA-256 verification of `ContentBody` bytes; type signature `&[u8; 32]` structurally rejects the prefix-short-circuit anti-pattern |
| `content_miss` | §10.1.2 | `withdraws` emission helper for ContentMiss feedback |
| `agent_files` | §8.1.6 | Three-layer trust composition (Canonical / Open / Vote-then-trust); anti-tricking guarantee enforced (Layer 3 never promotes to Layer 1) |

21 new unit tests covering:
- TTL boundary cases (exact-cutoff, one-second-past, tighter-than-default)
- SHA verification (matching, mismatching, empty, type-signature short-circuit-rejection)
- ContentMiss emission against NoOp directory
- Three-layer composition (empty input, steward-above-threshold, steward-below-threshold, non-steward-cannot-promote, multi-steward tiebreak, lex tiebreak, Layer-3 vote weight, anti-tricking guarantee)

### Dependency added

- `chrono = "0.4"` (default-features = false, features = ["std", "clock", "serde"]).
  Required because `ciris-persist` v3.3.0+ federation types use `chrono::DateTime<Utc>`. Registry's own time handling stays on the `time` crate; chrono is reserved for surfaces touching upstream Persist types directly.

### Deferred to v1.3.0 final (the Phase-2-follow-up)

- **Engine construction at boot in `main.rs`**: needs LocalSigner interop between Registry's `ciris-crypto`-based crypto module and Persist's `LocalSigner` type (via `LocalSigner::from_parts`). Non-trivial adapter work; the `pqc_signer` argument requires wrapping `ciris-crypto`'s PQC signer in Persist's `PqcSigner` trait.
- **AppState wiring**: add `federation: Arc<dyn FederationDirectory>` field; pass to handlers.
- **DSN config field**: add `persist_dsn: String` to `FederationSettings` (defaults to `sqlite::memory:` for development).
- **`/v1/agent_files/{kind}` endpoint integration**: replace the v1.4-interim empty-list stub with a call into `edge_transport::compose_trust_layers`, fetching attestations via `engine.federation_directory()`.
- **ContentFetch send path**: Registry doesn't fetch bytes from peers; clients do. The "byte fetch round-trip" half of CEG §10.1 is consumer-side (CIRISAgent + CIRISVerify clients), not Registry's responsibility.

### Tests

150/150 passing (was 129 in v1.2.0; +21 from edge_transport). `cargo check` clean.

---

## [1.2.0] — 2026-05-29

**Phase 2 of #33 — Persist wired. PersistFederationClient delegates to `ciris_persist::engine::Engine::federation_directory()` (closes most of #17 scaffolding).**

### Federation directory wiring

- `crate::federation::types` now re-exports `ciris_persist::federation::types::*` directly. The previously-vendored types had drifted vs upstream v3.3.0 (single `pubkey_base64` field vs the hybrid `pubkey_ed25519_base64` + `pubkey_ml_dsa_65_base64` split; `time::OffsetDateTime` vs `chrono::DateTime<Utc>`; etc.). Re-export eliminates the parity-drift class of bugs the original vendoring was meant to guard against. The "registry hashes the vendored shape; persist hashes its own" contract is now structurally enforced — there is no parallel definition that could drift.
- `PersistFederationClient` rewritten to hold `Arc<ciris_persist::engine::Engine>`. The 8 trait methods delegate to `engine.federation_directory().method(...).await`. Previous stub state (returning `DirectoryError::NotYetImplemented` on every call) is closed.
- `From<ciris_persist::federation::Error> for DirectoryError` added — `InvalidArgument` maps directly; all other variants collapse to `Rejected` with the upstream error's `Display` string preserved for forensic queries.
- `build_client` signature changed from `(settings)` → `(engine: Option<Arc<Engine>>, settings)`. When `dual_write_enabled=true` AND `engine=Some`, returns `PersistFederationClient`; otherwise falls back to `NoOpFederationClient` (defense-in-depth in case the boot misconfig guard at `config.rs::FederationSettings` is somehow bypassed).
- `DirectoryError::NotYetImplemented` variant retained for forward-compat (the upstream `FederationDirectory` trait has methods Registry's narrower trait does not yet expose — `attach_*_pqc_signature`, hybrid-pending sweep, trust grants, Goals — and a future expansion of Registry's wiring may want this variant as a clear signal for partial impls).

### Persist features enabled

- `ciris-persist = { features = ["postgres", "sqlite"] }` — needed for `Engine::federation_directory()` (gated on `any(feature = "postgres", feature = "sqlite")` upstream).

### Tests

- 129/129 passing (was 132 in v1.2.0-rc.1; the 3-test delta is from removing the vendored `Default` impls that were never used in production paths). `cargo check` clean.

### Not yet integrated into Registry handlers

- Boot path does not yet construct the `Engine`; `build_client(None, ...)` is the only call site, returning `NoOpFederationClient`. Handlers in `services/admin.rs` etc. continue to read from `trusted_primitive_keys` / `partner_keys` / `registry_signing_keys` directly. Engine construction at boot + dual-write enablement lands as a Phase-2-follow-up (small commit) before Phase 3 begins.

### Closes (most of)

- [#17](https://github.com/CIRISAI/CIRISRegistry/issues/17) — substrate-conformance plan: the federation client scaffolding milestone closes. The "crate-ify as ciris-registry-core" half of #17 lands in Phase 4.

---

## [1.2.0-rc.1] — 2026-05-29

**Phase 1 of #33 — Substrate triple available as Cargo deps; no wiring yet.**

This release-candidate marks the deps-available-but-unwired state. All three
substrate sister crates are now declared in `rust-registry/Cargo.toml`;
Phase 2 + 3 will wire them. The state is intentionally consumable as an RC
for sister consumers (CIRISVerify v4.0 integration tests, CIRISAgent workspace
shape experiments) to validate their assumptions against the new resolver
state before Phase 2 wiring lands.

### Substrate triple now declared

| Crate | Version | Source |
|---|---|---|
| `ciris-crypto` | **v4.0.0** (was v1.14.0) | CIRISVerify |
| `ciris-persist` | **v3.3.0** (NEW) | CIRISPersist |
| `ciris-edge` | **v0.18.0** (NEW) | CIRISEdge |

All three resolve to their tagged commits per `cargo tree`. Verified with
`cargo build` clean + full test suite passing (132/132).

### Resolver fix

- **pkcs8 transitive-pin resolved** (closes [#10](https://github.com/CIRISAI/CIRISRegistry/issues/10)).
  `ciris-crypto` v4.0.0 inherits the `ml-dsa = "=0.1.0-rc.8"` → `pkcs8
  ^0.11.0-rc.11` chain that previously forced rc.11 to be excluded by Cargo's
  pre-release caret resolution. Resolved by declaring `pkcs8 = "=0.11.0-rc.11"`
  explicitly in `[dependencies]` (forces version unification across all
  transitive deps; Registry does not use pkcs8 directly).

### Not yet wired (Phase 2 + 3 deliverables)

- `ciris-persist::FederationDirectorySqlite` — not yet called; the vendored
  `PersistFederationClient` stub at `src/federation/persist_client.rs` still
  returns `NotYetImplemented` on all paths.
- `ciris-edge::PeerResolver` + `ContentFetch` / `ContentBody` / `ContentMiss` —
  not yet implemented; CEG 0.2 §10.1 transport substrate wiring deferred to
  Phase 3.

### No CEG wire-format change

Registry-emitted attestation strings unchanged (no Registry code emits the
`attestation:l{N}:*` strings affected by CEG 0.2 §5.2 — verified pre-Phase-0
via grep). CEG-0.2 conformance MAJOR (v2.0.0) lands after Phases 2 + 3 + 4
per the versioning rule.

### Tests

132/132 passing (71 + 26 + 19 + 16 across the four test binaries). `cargo
check` clean; 47 pre-existing warnings, none introduced.

---

## [1.1.0] — 2026-05-29

**Baseline tag retroactively establishing CEG-conformance versioning discipline.**

This release does NOT ship new code. It names what's already on `main` at
commit [`4b27130`](https://github.com/CIRISAI/CIRISRegistry/commit/4b27130) as
the v1.1.0 baseline so subsequent phases (Phase 1 onward per [#33](https://github.com/CIRISAI/CIRISRegistry/issues/33))
have a stable referent.

### Packaging

- `[package]` aligned to sister substrate-component convention
  (`ciris-persist` / `ciris-edge` / `ciris-lens-core` / `ciris-node-core` /
  `ciris-crypto`):
  - `authors = ["CIRIS AI <hello@ciris.ai>"]`
  - `repository = "https://github.com/CIRISAI/CIRISRegistry"` (case-correct)
  - `publish = false` (explicit; not a crates.io crate)
  - `readme = "../README.md"`
  - `description` sharpened to one-line crate-purpose form
- `rust-version` left at `1.84` for the baseline; may relax to `1.75` in a
  future phase if the dep-bump `cargo check` confirms compatibility
- Added [`CHANGELOG.md`](CHANGELOG.md) (this file) — Keep-A-Changelog format
- Added [`RELEASE.md`](RELEASE.md) — release process + CEG-conformance
  versioning rule

### Spec surface as of 1.1.0

- CEG 0.2 specification published at [`FSD/CEG/`](FSD/CEG/README.md) (18 files;
  README entry point). Note that the SPEC is at 0.2 but the IMPLEMENTATION
  is at 1.1.0 — this is the baseline. CEG-0.2 wire-format conformance lands
  in v2.0.0 per the versioning rule.
- FSD-002 preserved as design-history; superseded by `FSD/CEG/`.
- Hybrid Ed25519 + ML-DSA-65 via `ciris-crypto` v1.14.0 (will bump to v4.0.0
  in Phase 1).
- gRPC + HTTP surfaces per FSD-001; Spock multi-master replication; federation
  client surface scaffolded (Persist wiring deferred to Phase 2 per #17).

### Known gaps vs CEG 0.2 (lands in v2.0.0)

- No `ciris-persist` / `ciris-edge` deps yet (Phase 1).
- `PersistFederationClient` returns `NotYetImplemented` on all paths (Phase 2).
- No Edge transport wiring; CEG 0.2 §10.1 ContentFetch + `holds_bytes` consumer
  not implemented (Phase 3).
- No crate-ify split; `ciris-registry-core` lib does not yet exist (Phase 4).
- Registry code emits no `attestation:l{N}:*` strings (verified pre-Phase-0
  via `grep -rn "attestation:l[0-9]" rust-registry/src/` returning empty);
  the CEG 0.2 §5.2 mechanism-prefix rename is therefore purely a spec change
  with zero Registry code impact at v1.1.0.

---

[Unreleased]: https://github.com/CIRISAI/CIRISRegistry/compare/v2.2.0...HEAD
[2.2.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.2.0
[2.1.4]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.1.4
[2.1.3]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.1.3
[2.1.2]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.1.2
[2.1.1]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.1.1
[2.1.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.1.0
[2.0.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.0.0
[1.4.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.4.0
[1.3.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.3.0
[1.3.0-rc.1]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.3.0-rc.1
[1.2.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.2.0
[1.2.0-rc.1]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.2.0-rc.1
[1.1.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.1.0

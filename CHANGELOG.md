# Changelog

All notable changes to **CIRISRegistry** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
with one project-specific rule: **MAJOR version bumps signal CEG wire-format
conformance breaks**. See [`RELEASE.md`](RELEASE.md) for the versioning rule and
the release process.

| Series | CEG conformance |
|---|---|
| 1.x | FSD-002 v1.4.3 baseline |
| 2.x | CEG 0.2 |
| 3.x | CC 1.0-rc4 |
| 4.x | CC 1.0-rc5 through rc12 (authority triad; the registry slice) |

Per-feature history before v1.1.0 lives in `git log`.

---

## [Unreleased]

### Added

- **Evidence of work on the registry slice** (`fold_evidence`): `POST /v1/evidence/{suite}` admits a self-report of work cited by hash as a `scores` Contribution on `evidence:{suite}:{kind}:{cell}:{prefix}:v1` (CC 3.1.5.5) with the bundle's manifest as a commons blob; `GET /v1/evidence/{suite}` lists verified rows filtered by cell, kind, agent version, rubric and `since`; one row and its bytes by manifest hash. `safety_battery` is the registered suite (capture and interpret bundles; the interpretation rides its capture's dimension prefix and, under the family's `signed` polarity, scores `2·r − 1` for pass ratio `r`). A row may cite the `goal:` / `approach:` / `method:` row it is offered against in `references_attestation_id` (resolved through `supersedes`; withdrawn or unknown targets refused); reads carry `offered_against` and `counted_by` (the `progress_measure` rows citing the row), and `GET /v1/goals/{attestation_id}/evidence` lists the evidence counted under a goal. Same authority as build manifests: the signer must hold `infra:attest`. The build-manifest intake is now the family-generic admission the evidence door shares; its behaviour and routes are unchanged.

## [4.1.0] — 2026-10-28

Serves the production genesis and the CEG-native build-manifest surface on the registry slice. Substrate: persist v59.1.0, edge v43.2.1, verify v20.2.0 (CIRISServer 0.5.240's triple). `CEG-Version: 1.0-rc12`.

### Wire changes

- **`GET /v1/steward-key` and `GET /v1/trust-root/bundle` serve the version-4 genesis bundle**: holders A1/B1/C1 with custody evidence, the `ciris-canonical` birth record, serve node `ciris-canonical-1`. Clients that pinned the version-2 bundle's fingerprint see it change.
- **New `community` member** on both routes: the `ciris-canonical` community birth record the bundle carries, `null` when the bundle carries none. Same field and semantics as CIRISServer's `GET /v1/trust-root/bundle`.
- **`/v1/builds` on the fold router**: CEG-native build manifests. A pipeline is blessed by a `delegates_to(root → pipeline, infra:attest)` grant or by the accord role on its key record. Routes: submit, read per version, read per manifest hash, manifest bytes. The standalone binary's provenance walk uses the dimension spelling `provenance:build_manifest:{target}:v1`.

### Substrate adoption

- Every registry read pages persist's bounded `AttestationQuery` (`federation::persist_client::collect_pages`), narrowed to the attestation family where one is known; the unbounded list reads are no longer used.
- Key registration writes a `key:record:v1` row under the key's own name; reads that mean "this key's Contributions" filter by dimension.
- CIRISVerify emits the registry's build-manifest wire shape (row mirror, `:v1`, `manifest_size`); `tests/build_manifest_fold.rs` pins that verify's producer output goes through `POST /v1/builds` unchanged.

### Consumer notes

- `GET /v1/steward-key` carries no response signature. CIRISVerify through v20.2.0 expects the older `{stewards, threshold_policy, response_signature}` shape and falls back to DNS-only trust; consuming the `bundle` member is verify-side work.

**Rollback metadata:** Migrations: none in the registry's sqlx tree. Config: none new.

## [4.0.0] — 2026-10-01

**MAJOR.** Adopts the CC 1.0-rc5 authority triad on the wire, closes an authorization hole, and moves to CIRISServer 0.5.218's substrate. From this release `ciris-registry-core` composes into CIRISServer as the registry slice.

### Wire changes

- **`GET /v1/partner/{key_id}`: `licensure[].authority_id` is the organisation's `org_id` UUID** (CC 2.4.1.2.1, part_3 §3.3.9). It previously emitted the partner's tax or registration number on a public route. The id resolves through `organizations.partner_id` (oldest row); a partner with no organisation, or whose `org_id` is not UUID-shaped, gets no licensure entry.
- **`licensure[].status` uses the rc5 vocabulary**: `issued`, `suspended`, `revoked`. `suspended` is reversible and `revoked` is terminal, and they are no longer collapsed into one token. An unrecognised status emits `unspecified`, which withholds rather than escalates.
- **`CEG-Version` header: `1.0-rc5`.**

### Fixes

- **Partner lookups on a self-migrated database.** The `responsible_party` and `public_contact_email` columns were queried but never added by the sqlx migration tree, so `LookupPartner` errored and `/v1/partner/{key_id}` served an empty composition. Migration 031 adds both columns idempotently.
- **`/v1/identity` serves all six keys.** The edge transport is built with the registry's federation signer (`HybridCrypto::build_edge_local_signer`), so the two `reticulum_*_pubkey_b64` keys the `ciris-canonical` enrollment resolves a member by are published again. The transport identity stays a separate dual-key (AV-17).

### Security

- **`PortalService.GetUser` and `GetUserByEmail` are authorized.** The caller must be the user, a SYSTEM_ADMIN, or hold Viewer or better in an org the target belongs to. Non-admins get the same `PermissionDenied` for not-found and not-allowed, so the RPCs cannot probe which ids or emails exist. Denials are audit-logged.
- Known and unchanged: `POST /v1/integrity/auth` reports `authenticated` and `authorized: true` for any `Bearer ` prefix without validating the token (FSD-004 §7).

### Fold surface

- **FSD-004** specifies the post-fold registry surface: every route and RPC with its post-fold path, substrate call, caller tier and node posture, and the licensure, grant and delegation legs as three objects with three wire shapes. Custodial signing is dropped and CIRISPortal becomes CIRISClient screens. There is no community licence to issue.
- **Two builds of `ciris-registry-core`.** `standalone` (default) keeps sqlx, tonic, persist's `postgres` backend, the gRPC services and the full HTTP router. Without default features the crate has no sqlx, tonic or tokio-postgres in its graph, and CI fails the build if any returns.
- **`fold::router(engine, node_key_id)`** serves `/v1/steward-key`, `/v1/trust-root/bundle` and `/v1/agent_files/{kind}` over the server's shared persist engine. The standalone HTTP server mounts the same handlers. The router does not mount `/v1/identity`, `/v1/accord-holders` or health, which the server owns.

### Substrate

persist v52.0.1, edge v38.1.0, verify v18.0.0: the tags CIRISServer 0.5.218 pins. Each git pin carries a `version = "N"` major guard. `rust-version` is 1.86. edge brings `leviculum-std` → `bluer` → `dbus` with vendored libdbus linked statically, so the binary gains no runtime `.so` dependency.

**Rollback metadata:** persist applies **V136–V166** at boot against `FEDERATION_PERSIST_DSN`; after they apply, a v3.0.0 image will not boot against that database with dual-write on. Snapshot before deploying. The registry's own sqlx migrations are unchanged. Config: none new; `CIRIS_REGISTRY_EDGE_IDENTITY_PATH` takes effect as documented.

## [3.0.0] — 2026-09-04

**MAJOR.** Adopts the ratified CIRIS Constitution (1.0-rc4) and stops the registry asserting trust it never verified.

### Wire changes

- **`GET /v1/steward-key` serves the portable trust root**: the `GenesisBundle` persist bakes, with the `humanity-accord` charter, the A1/B1/C1 holder roster, the `infra:*` scopes the charter confers and the serve-node grants. `GET /v1/trust-root/bundle` serves the identical schema, field for field CIRISServer's. The bundle is self-authenticating; the outer envelope carries no `response_signature` by design. The previous body published the registry's own key as a root, unsigned, in a shape none of verify's client schemas parsed.
- **`GET /v1/builds/{version}` provenance** emits `provenance:build_manifest:{target}` only when a pipeline-signed Contribution binding the exact `(target, binary_version, manifest_hash)` exists in the federation directory, signed by a node key carrying the accord-conferred `infra:attest` role, whose bound-hybrid signature re-verifies against directory-pinned pubkeys. Otherwise the dimension is absent and `note` says so. Manifest bytes keep serving from the build table.
- **`-stable` is folded on version lookup**; `-rc*` and `-beta*` are not.

### Substrate

persist v40.0.0, edge v20.1.1, verify v14.1.0. `ciris-verify-core` is a direct dependency (`default-features = false`, `pqc`). MSRV 1.86. The obsolete `pkcs8` resolver pin is removed.

### Spec and docs

- The Constitution corpus is homed at CIRISAI/CIRISConstitution; `FSD/CEG/` is a redirect stub with the section-to-Part map. `CEG_VERSION` header is `1.0-rc4`.
- FSD-002 reconciled with CC: mechanism-named attestation prefixes, the v2 JCS canonical-bytes form (the v1 line-oriented form is retired as a newline-injection surface), and no `accountability:*` family (a mode shift is a superseding `delegates_to` on the delegation plane).

### CI

`ci.yml` runs `build + test` without a database and `db integration` against Postgres 16. `cargo fmt --check` and clippy are not gating.

**Rollback metadata:** Migrations: none. Config: none new. `FEDERATION_DUAL_WRITE_ENABLED` also governs the provenance walk; with it off the manifest dimension is always absent.

## [2.3.0] — 2026-06-05

**STH consistency proof at cosign admission** (CEG 0.2 §10.3.1). A witness cosigning a Signed Tree Head proves the new tree is an append-only extension of the prior STH it cosigned.

- RFC 6962 §2.1.2 consistency verifier in `db::merkle_consistency`, tested against independent known-answer vectors.
- `sth_cosign` anchors the check against the prior `(tree_size, root_hash)` the registry recorded for that witness, never a root the requester claims. First cosignature is exempt. A tree size behind the witness's prior STH or a missing proof is `MALFORMED_REQUEST`; a proof that does not reconstruct both roots is `CONSISTENCY_PROOF_INVALID` (422).
- Request body gains `consistency_proof_path_b64: Vec<String>` (base64 32-byte RFC 6962 node hashes), defaulted so the wire shape is backward compatible.

**Consumer note:** witnesses that have cosigned before must supply the proof on their next cosign. Migrations: none. Config: none.

## [2.2.1] — 2026-06-05

**Unified error envelope** (CEG 0.2 §10.0.1). Every HTTP error response serializes to `{error: {code, http_status, message, request_id, details}}`; the twelve ad-hoc error structs are gone.

- Spec-precision codes: `UNKNOWN_WITNESS` is 404 (was 403); Ed25519 and ML-DSA-65 verification failures are 422 `SIGNATURE_VERIFICATION_FAILED` with `details.algorithm` (was 400); hex-format failures are `CANONICAL_BYTES_VIOLATION`.
- `error.code` is the stable discriminator; consumers branching on status for these paths should update.

Migrations: none. Config: none.

## [2.2.0] — 2026-06-05

**Request-context layer** (CEG 0.2 §10.0).

- `api::error` module: `ApiError` and the 13-value `ErrorCode` enum serializing to the §10.0.1 envelope.
- `request_context_mw` sets `X-Request-Id`, `CEG-Version` and `X-CEG-Server-Time` (RFC 3339 millis) on every response, success or error.

Migrations: none. Config: none.

## [2.1.4] — 2026-06-05

Docs-only release. `RELEASE.md` gains the Rollback section (last-known-good digest, migration discipline and rollback floor, per-release config deltas, sister-stack compatibility matrix) and establishes the per-release rollback-metadata lines. The stale ECS ansible playbook moves to `deploy/aws-archive/`.

## [2.1.3] — 2026-05-30

Substrate pins per the CIRISConformance matrix: `ciris-crypto` and `ciris-keyring` v4.4.3, `ciris-persist` v3.6.9, `ciris-edge` v1.1.3. The upstream change is a darwin-only libsqlite3 linking fix; the registry's Linux image is unaffected and keeps the bundled `libsqlite3-sys` feature.

## [2.1.2] — 2026-05-30

CI fix: the `:latest` promotion step lowercases the image name, which Docker requires. Same code as 2.1.1.

## [2.1.1] — 2026-05-30

**Hotfix for a crash loop on missing `libsqlite3.so.0`.** `libsqlite3-sys` is built with the `bundled` feature so the binary statically links sqlite3, and the runtime image installs `libsqlite3-0` as a second line of defence. The Docker workflow gains a smoke gate: `:latest` is pushed only after the SHA-pinned image passes an `ldd` check and a boot check.

## [2.1.0] — 2026-05-30

Substrate pins per the CIRISConformance matrix: `ciris-crypto` and `ciris-keyring` v4.4.2, `ciris-persist` v3.6.3, `ciris-edge` v1.0.1. The local `PersistPqcAdapter` is removed in favour of the upstream `PqcSigner` implementation. No CEG wire-format change.

## [2.0.0] — 2026-05-29

**MAJOR: CEG 0.2 conformance.** `ciris-registry-core` is consumable by git tag.

Conforms to §5.2 (mechanism-only attestation strings), §10.1 (transport substrate read helpers, persist Engine wired through the federation directory, `/v1/agent_files/{kind}` live), §5.6.7 (`agent_files:*` three-layer trust composition with the anti-tricking guarantee) and §5.2.1 (`provenance:build_manifest:*` per-target and per-locale Merkle composition). The §10.3.1 and §10.0.1 gaps are closed in 2.3.0 and 2.2.1.

## [1.4.0] — 2026-05-29

Workspace split under `rust-registry/`: `ciris-registry-core` (library: api, config, crypto, db, edge_transport, federation, middleware, proto types, services, migrations, integration tests) and `ciris-registry` (binary: the boot sequence). One `Cargo.lock` at the workspace root; `[profile.release]` and shared package metadata at the workspace.

## [1.3.0] — 2026-05-29

- The persist `Engine` is constructed at boot with a `LocalSigner` derived from the registry's keypair (`HybridCrypto::build_persist_local_signer`).
- `AppState` carries the federation directory: the persist client when `FEDERATION_DUAL_WRITE_ENABLED=true`, otherwise a no-op client.
- `/v1/agent_files/{kind}` queries the federation directory and composes three-layer trust.
- New env var `FEDERATION_PERSIST_DSN` (default `sqlite::memory:`; production points at Postgres).
- Deps: `ciris-crypto` v4.1.0, `ciris-keyring` v4.0.0, `ciris-persist` v3.3.1.

## [1.3.0-rc.1] — 2026-05-29

New `edge_transport` module: TTL filtering on `holds_bytes:sha256:*` attestations (§10.1.2), full SHA-256 verification of `ContentBody` bytes (§10.1.1), `ContentMiss` feedback emission, and three-layer `agent_files` composition (§8.1.6), each with unit tests. Adds the `chrono` dependency for persist's federation types.

## [1.2.0] — 2026-05-29

- `crate::federation::types` re-exports `ciris_persist::federation::types::*`; the vendored copies are gone.
- `PersistFederationClient` delegates to `ciris_persist::engine::Engine::federation_directory()`.
- `build_client(engine, settings)` returns the persist client when dual-write is enabled and an engine is present, otherwise the no-op client.
- `ciris-persist` is built with the `postgres` and `sqlite` features.

## [1.2.0-rc.1] — 2026-05-29

The substrate triple is declared as Cargo dependencies: `ciris-crypto` v4.0.0, `ciris-persist` v3.3.0, `ciris-edge` v0.18.0. The `pkcs8` transitive pin is resolved explicitly.

## [1.1.0] — 2026-05-29

Baseline tag. Packaging aligned to the sister substrate crates (`publish = false`, AGPL-3.0-or-later, case-correct repository URL, `readme = "../README.md"`). Adds `CHANGELOG.md` and `RELEASE.md`. The CEG 0.2 specification published under `FSD/CEG/` at this release has since moved to CIRISAI/CIRISConstitution.

---

[Unreleased]: https://github.com/CIRISAI/CIRISRegistry/compare/v4.1.0...HEAD
[4.1.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v4.1.0
[4.0.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v4.0.0
[3.0.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v3.0.0
[2.3.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.3.0
[2.2.1]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v2.2.1
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

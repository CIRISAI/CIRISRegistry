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

[Unreleased]: https://github.com/CIRISAI/CIRISRegistry/compare/v1.3.0-rc.1...HEAD
[1.3.0-rc.1]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.3.0-rc.1
[1.2.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.2.0
[1.2.0-rc.1]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.2.0-rc.1
[1.1.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.1.0

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

- **v1.2.0** — Persist wiring (#17 most): replace `PersistFederationClient`
  `NotYetImplemented` stubs with real `ciris_persist::FederationDirectorySqlite`
  calls; map vendored federation types to upstream.
- **v1.3.0** — Edge wiring (CEG 0.2 §10.1 + connects #18): `PeerResolver` client
  + `holds_bytes` directory consumption + ContentFetch/ContentBody/ContentMiss
  round-trip with full-SHA verification (§10.1.1) + 24h TTL discipline (§10.1.2).
- **v1.4.0** — Crate-ify split: `ciris-registry-core` library (for CIRISAgent
  workspace cohabit with `ciris-lens-core` + `ciris-node-core`) + `ciris-registry`
  binary (standalone deployments).
- **v2.0.0** — CEG-0.2 conformance MAJOR + fold-readiness; closes #17 + #18 + #32.

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

[Unreleased]: https://github.com/CIRISAI/CIRISRegistry/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/CIRISAI/CIRISRegistry/releases/tag/v1.1.0

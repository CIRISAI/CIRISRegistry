# Release process

CIRISRegistry follows [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html) with
**one project-specific rule**: MAJOR version bumps signal CIRIS Epistemic
Grammar (CEG) wire-format conformance breaks.

## Versioning rule

| Series | CEG conformance |
|---|---|
| 1.x | FSD-002 v1.4.3 baseline |
| 2.x | CEG 0.2 |

When CEG bumps from 0.x → 0.(x+1) and Registry adopts the new spec, Registry
bumps MAJOR. Within a series, MINOR bumps add wire-compatible features (new
endpoint shapes, new prefix consumption, new composition policy support) and
PATCH bumps fix bugs without changing the wire-facing surface.

Sister precedent: CIRISVerify shipped v4.0.0 as its CEG-0.2-conformance major.
Registry follows the same shape.

## Steps for a release

1. **Verify clean build + tests**

   ```bash
   cd rust-registry
   cargo build --release
   cargo test
   ```

2. **Update [`CHANGELOG.md`](CHANGELOG.md)**

   - Move `[Unreleased]` content into a new `[X.Y.Z] — YYYY-MM-DD` section
   - Add a fresh `[Unreleased]` header above it
   - Update the comparison link at the bottom (`[X.Y.Z]: ...releases/tag/vX.Y.Z`)

3. **Bump the version**

   ```bash
   # Edit rust-registry/Cargo.toml: version = "X.Y.Z"
   cargo update --workspace  # refresh Cargo.lock with the new version
   ```

4. **Commit + tag**

   ```bash
   git add CHANGELOG.md rust-registry/Cargo.toml rust-registry/Cargo.lock
   git commit -m "chore(release): vX.Y.Z"
   git tag -a vX.Y.Z -m "vX.Y.Z"
   ```

5. **Push**

   ```bash
   git push origin main
   git push origin vX.Y.Z
   ```

6. **Publish release notes**

   - Open the [Releases](https://github.com/CIRISAI/CIRISRegistry/releases)
     page; new tag should appear unreleased.
   - Click "Draft a new release" against the tag.
   - Paste the CHANGELOG section verbatim as release notes.
   - Cross-link the umbrella tracking issue and any per-phase issues that
     close at this release.

## Packaging conventions matched to sister substrate components

This repository follows the packaging conventions of the cohabit set
(`ciris-persist`, `ciris-edge`, `ciris-lens-core`, `ciris-node-core`,
`ciris-crypto`):

- `publish = false` — not published to crates.io; consumers pin by git tag
- `license = "AGPL-3.0-or-later"`
- `authors = ["CIRIS AI <hello@ciris.ai>"]`
- `repository` uses the case-correct GitHub URL
- `readme = "../README.md"` (Cargo.toml is in `rust-registry/`; README is at
  repo root)
- Versioned git tags signal releases; no pre-release artifacts on package
  registries (release candidates use `vX.Y.Z-rc.N` git tags only).
- `rust-version` reflects MSRV; raised conservatively. Sisters typically pin
  to `1.75`; Registry currently pins `1.84` and may relax in a future phase
  once Phase 1's dep bumps are verified compatible at the lower MSRV.

## Cohabitation end-state

Per [umbrella issue #33](https://github.com/CIRISAI/CIRISRegistry/issues/33):
the v2.0.0 release deliverable is a `ciris-registry-core` library (Phase 4
split) that CIRISAgent pulls into its workspace **alongside `ciris-lens-core`
and `ciris-node-core`**, all three cohabiting on the substrate triple
(`ciris-persist`, `ciris-crypto`, `ciris-edge`). The `ciris-registry` binary
half stays at this repo for partners running standalone Registry deployments
(Stripe billing, public installer endpoint, admin UI backing).

```
              CIRISAgent workspace (post-fold target)
   ┌─────────────────────────────────────────────────────┐
   │  ciris-agent (bin)                                  │
   │     │                                               │
   │     ├─ ciris-lens-core                              │
   │     ├─ ciris-node-core                              │
   │     └─ ciris-registry-core  (Phase 4 deliverable)   │
   └─────────────────────────────────────────────────────┘
                            │
                            ▼  all three pull from:
                ┌──────────────────────────┐
                │   Substrate triple       │
                │   ciris-persist  v3.3.0  │
                │   ciris-crypto   v4.0.0  │
                │   ciris-edge     v0.18.0 │
                └──────────────────────────┘
```

## Pre-release process (release candidates)

For breaking changes (typically a MAJOR bump), tag release candidates first:

```bash
# rc.1
git tag -a v2.0.0-rc.1 -m "v2.0.0-rc.1: substrate-sister adoption preview"
git push origin v2.0.0-rc.1
```

Sister consumers (CIRISVerify, CIRISAgent) can pin to the RC for integration
testing. When clean, drop the `-rc.N` suffix for the final release.

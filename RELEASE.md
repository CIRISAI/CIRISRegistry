# Release process

CIRISRegistry follows [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html) with
**one project-specific rule**: MAJOR version bumps signal CIRIS Epistemic
Grammar (CEG) wire-format conformance breaks.

## Versioning rule

| Series | CEG conformance |
|---|---|
| 1.x | FSD-002 v1.4.3 baseline |
| 2.x | CEG 0.2 |
| 3.x | CC 1.0-rc4 |
| 4.x | CC 1.0-rc5 through rc12 |

When the Constitution's wire format moves to a new series and the registry
adopts it, the registry bumps MAJOR. Within a series, MINOR bumps add wire-compatible features (new
endpoint shapes, new prefix consumption, new composition policy support) and
PATCH bumps fix bugs without changing the wire-facing surface.

CIRISVerify follows the same shape.

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
   - Cross-link the issues that close at this release.

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
- `rust-version` is the MSRV (`1.86`, verify's floor) and is raised
  conservatively.

## Workspace shape

`ciris-registry-core` is the library CIRISServer composes as the registry
slice, alongside `ciris-lens-core` and `ciris-node-core`, all three on the
substrate triple (`ciris-persist`, `ciris-verify`, `ciris-edge`). The
`ciris-registry` binary is the standalone deployment of the same library.
The substrate pins each release carries are recorded in its CHANGELOG entry.

```
              CIRISServer workspace
   ┌─────────────────────────────────────────────────────┐
   │  ciris-server (bin)                                 │
   │     │                                               │
   │     ├─ ciris-lens-core                              │
   │     ├─ ciris-node-core                              │
   │     └─ ciris-registry-core  (no default features)   │
   └─────────────────────────────────────────────────────┘
                            │
                            ▼  all three pull from:
                ┌──────────────────────────┐
                │   Substrate triple       │
                │   ciris-persist          │
                │   ciris-verify           │
                │   ciris-edge             │
                └──────────────────────────┘
```

## Pre-release process (release candidates)

For breaking changes (typically a MAJOR bump), tag release candidates first:

```bash
# rc.1
git tag -a vX.Y.Z-rc.1 -m "vX.Y.Z-rc.1"
git push origin vX.Y.Z-rc.1
```

Sister consumers (CIRISVerify, CIRISServer) can pin to the RC for integration
testing. When clean, drop the `-rc.N` suffix for the final release.

## Rollback

Deployment is GHCR + watchtower on Vultr/Hetzner (US + EU). The mechanical pin
lives in CIRISCore's `scripts/watchtower-rollback.py` runbook (recreate the
container against `image@sha256:...`, unpin later). This section answers the
four *safety* questions that runbook defers to the upstream project.

> ⚠️ `deploy/aws-archive/rollback.yml` is archived ECS tooling for an AWS
> task-definition deployment that is not run. Do not invoke it against the
> GHCR + watchtower infrastructure; it errors on `amazon.aws.ecs_service_info`.

### 1. What's the last known-good digest?

**Canonical source: the GHCR SHA-pinned tag for the target version.** Every
release pushes both `:vX.Y.Z` and a SHA-tag; the smoke-gated `:latest` only ever
points at a smoke-passed SHA (see `.github/workflows/docker.yml`). To roll back:

```bash
# resolve the digest of a known-good version
crane digest ghcr.io/cirisai/cirisregistry:vX.Y.Z
# → sha256:...   pin the container to image@<that digest>
```

The CHANGELOG `[X.Y.Z]` entry is the human index of which version was last
known-good in prod; the GHCR tag → digest is the machine-canonical pin.

### 2. Are there forward-only DB migrations between versions?

**Migrations are forward-only and additive by discipline** (see CLAUDE.md
"Database Migration Notes" — `ADD COLUMN IF NOT EXISTS`, `CREATE TABLE IF NOT
EXISTS`, idempotent so a per-region re-apply is safe). `sqlx migrate run` at boot applies
only unapplied migrations; it never reverses. Consequence for rollback:

- **Pinning the binary back a few versions is safe** as long as the older binary
  tolerates a schema that is *ahead* of it — which it does for additive
  migrations (extra columns/tables the old binary ignores).
- **A release that ships a non-additive migration MUST say so in its CHANGELOG
  entry** with an explicit "rollback floor" (the oldest binary version safe to
  pin back to). Absent such a note, the additive-only default holds and any
  recent version is a safe pin-back target.
- The DB is never rolled back; only the binary is pinned. Do not hand-revert
  `_sqlx_migrations`.

### 3. Are there breaking config / env / volume changes between versions?

Pinning the binary back can succeed but leave the container misconfigured if a
newer version added a required env var or changed a mount. **Each CHANGELOG
`[X.Y.Z]` entry MUST call out any new required env var, changed default, or
volume change vs the previous version** under a "Config" line. Rollback ops set
or unset those alongside the pin. Required env: `DATABASE_URL`, `GRPC_PORT`,
`HTTP_PORT`, the steward seed-file paths (`ED25519_KEY_PATH`, `MLDSA_KEY_PATH`),
`FEDERATION_PERSIST_DSN`, and `FEDERATION_DUAL_WRITE_ENABLED` when federation
is on.

### 4. Sister-stack compatibility matrix

Registry interoperates with the substrate triple (CIRISPersist / CIRISVerify /
CIRISEdge) and is consumed by CIRISVerify / CIRISAgent. **The canonical
compatibility pin is
[`CIRISConformance/matrices/current.yaml`](https://github.com/CIRISAI/CIRISConformance/blob/main/matrices/current.yaml)** —
roll back only to a Registry version whose substrate-triple pins
(`rust-registry/*/Cargo.toml`) are compatible with the matrix the rest of prod
is running. A Registry version pinning an older `ciris-persist` than the running
substrate is the dangerous case; the CHANGELOG substrate-triple table per
release records what each version pinned.

### Per-release rollback metadata

Each CHANGELOG entry carries the rollback-relevant lines so the four questions
above are answerable per release:

- **Digest**: the published GHCR `sha256:` (filled post-CI).
- **Migrations**: `additive-only` (default) OR an explicit rollback floor.
- **Config**: `none` OR the new/changed env/volume vs the previous version.

# Runbook — unique registry keys, and the final fold into CIRISServer

**Status:** PROPOSAL, 2026-10-02. Nothing in it has been executed.
**Goal:** CIRISServer replaces the registry entirely. canonical-2 and canonical-3 are
fresh Server nodes baked into the final genesis; the registry surface is served by Server
under the roles the genesis confers; the standalone registry service, its databases and
its shared key go away. No legacy registry endpoint survives.

---

## 0. What is true today (measured 2026-10-02, read-only)

| Fact | Evidence |
|---|---|
| us and eu run on **one** key, `75c29fcc…d265a` | `/opt/ciris/registry/keys/{ed25519,mldsa}.key` byte-identical on 207.148.13.157 and 91.99.150.121 (same sha256), both dated 2026-05-01; `/v1/identity` reports identical Ed25519 and ML-DSA pubkeys |
| That key is each node's **whole identity** | `ciris-registry/src/main.rs`: the same seed builds the persist Engine signer and the edge signer, and is reported as `served_by.node_key_id` |
| The persist side is **not durable** | neither container sets `FEDERATION_DUAL_WRITE_ENABLED` or a persist DSN, so the Engine is `sqlite::memory:`; content-KEM keys are re-minted each boot, which is why us and eu publish different content keys under the same key_id |
| The key **re-signs function manifests** today | `function_manifests`: 1,790 rows with `signature_key_id = 75c29fcc…`, latest 2026-10-02 (`ciris-agent` 19.0.0 posted through the legacy path). Verify treats these as chained to the registry's pubkey |
| It is **not** the CI build-signing key | `trusted_primitive_keys`, project `ciris-registry` = `567513a0…`, a separate key. Changing the runtime seed does not break the registry's self-publication |
| The two databases are one replicated store | pgEdge between regions; both show identical row counts. ufw allows 5432 only from the peer region |
| The registry hosts are **full** | 3 GB RAM, 2–3 vCPU, already running registry, node, bench, portal, two Postgres. canonical-1 alone uses 1.7 of its 2 GiB cap at ~2 CPUs |

**Ruling this implies:** do not adopt the shared seed into either new node.
`FSD/REGISTRY_FOLD_DERISK.md` §2 in CIRISServer says "ADOPT byte-identical … if literally
shared, split into per-node identities". It is literally shared, and whichever region
adopted it would hold a key the other region also holds. Both canonicals are **fresh
mints**, the same flow canonical-1 used (`BRIDGE_SEED_MESH.md`).

---

## 1. Decisions needed before anything runs

1. **Hosts for canonical-2 / -3.** Recommended: dedicated hosts sized like canonical-1
   or larger (≥4 GB). Alternatives: resize the two registry hosts, or use the lightly
   loaded node hosts (207.148.15.107 us, 142.132.232.203 eu: node + postgres only, 56/68 GB
   free). Putting CIRISServer next to the full registry stack on 3 GB will not fit.
2. **Legacy builds.** 3,588 `builds` rows and 1,790 registry-re-signed function manifests
   have no CEG-native Contribution behind them, and the registry is not staying up to
   serve them. Recommended: before cutover, each primitive's CI republishes its
   **currently supported** releases CEG-native (Verify 19 `--emit-contribution`), and the
   rest are dropped. Older agents that fetch an unsupported version's manifest get a 404.
   The alternative is a one-shot import of the legacy rows into Server as a frozen table;
   it keeps old agents working but carries registry-signed data into a system with no
   registry signer.
3. **Hostnames.** `api/us/eu.registry.ciris-services-1.ai` move to the canonical
   Server nodes at cutover, or are retired once clients use Server hostnames.

---

## 2. Prerequisites (other repos; none of this is done by this runbook)

| Prerequisite | State |
|---|---|
| persist v53 + verify v19.0.0 tagged; CIRISServer 0.5.220 on that triple | in flight |
| CIRISRegistry: repin to the 0.5.220 triple, add the `community` field to `/v1/trust-root/bundle` and `/v1/steward-key` (agreed with Server, `null` when not baked) | after the tags |
| CIRISRegistry#143 (CEG-native `/v1/builds`, both blessing paths) merged and tagged | open |
| CIRISServer#442 (registry slice composed; commons consent for conferred nodes) merged | open |
| CIRISServer#710 — a first-run claim writes the owner's root acceptance | open; until then, restart once after the claim |
| CIRISEdge#785 — commons holder roster is fixed at puller spawn | open; until then, restart after canonical admission changes |
| Agent / Verify / Persist / Edge / Lens CI emit CEG-native Contributions (`ciris-build-sign sign --emit-contribution`, Verify 19) | Verify side done on its branch; each repo's CI still to switch |

---

## 3. Phase A — mint canonical-2 and canonical-3

Per host, through CIRISBridge's `ciris-server` role, fresh-mint flow:

```bash
# CIRISBridge/ansible, one host at a time
ansible-playbook playbooks/site.yml -l <canonical-2 host> -t ciris-server \
  -e ciris_server_key_id=ciris-canonical-2 -e ciris_server_fresh=true
```

1. The node mints its own hybrid identity under `--key-id ciris-canonical-2` on a wiped
   home. Nothing is copied from the registry hosts.
2. Record `key_id` and `pqc_key_id` from the role's identity print (`tasks/main.yml`
   prints both). The wire id is `ciris-canonical-2-<fingerprint>`, derived from the
   pubkey (`fedcode::derive_key_id`); `ciris_server_key_id` is only the alias.
3. Expect `registry slice WITHHELD` in the log. That is correct: nothing has conferred
   `infra:attest` yet.
4. Repeat for canonical-3 on the eu host.

**Check:** the two `key_id`s differ from each other, from canonical-1, and from `75c29fcc…`.
**Rollback:** stop the container and remove the data dir. Nothing else references these keys yet.

---

## 4. Phase B — bake them into the final genesis

Owned by the CIRISServer genesis ceremony (design: CIRISServer `FSD/FINAL_GENESIS.md`).
Not close: it waits on persist's production assembler (CIRISPersist#973), the server's
ceremony routes, the client's 3-of-3 re-mint sheet and a dry run with a real YubiKey.
The two mints (Phase A) are needed only before that dry run. What registry needs from it:

1. canonical-1, -2, -3 in the bundle, each co-scrubbed by the accord quorum (2-of-3),
   each with `roles: [infra:serve, infra:attest]` **and** a matching quorum-scrubbed
   `trust:confers:v1` grant for the same scopes (CC 3.4.7: one holder alone is no grant). The registry
   slice gate walks for `infra:attest`; the harness showed a canonical granted
   `infra:serve` alone does not confer the slice.
2. A signed transport hint for each, so peers can dial them.
3. CI pipeline keys blessed in the same ceremony, either by role co-scrub (the CI-key
   ceremony's shape, read by persist's `is_infra_attest_effective`) or by grant. The
   registry door accepts both. Recommended: role co-scrub, since it is not
   reader-relative.
4. The `ciris-canonical` community birth asset: the accord holders A1/B1/C1 are its
   founders (they sign the birth); canonical-1/-2/-3 are seated as non-signing members
   (CC rc6 P3:192, P3:670-722). Nodes have no agency, so a node is never a founder.

**Check, on the minted bundle before it is baked:** `verify_ceremony_outputs(bundle,
community)` passes, and three distinct canonical key_ids appear, none of them `75c29fcc…`.

---

## 5. Phase C — prove it on the mesh before production

1. CIRISServer `harness/mesh-repro`: run the `manifest` scenario against a bundle minted
   the Phase B way (three canonicals, a role-blessed pipeline, an unblessed one). All
   rungs green, including `ceremony_admitted`, `ceremony_on_b` and `unblessed_is_nowhere`.
2. Same run with `MAN_EXTERNAL_PIPELINE` pointed at a Verify 19 Contribution (done once
   on 2026-10-01: 15/15).

**Gate:** do not proceed to Phase D on a red ladder.

---

## 6. Phase D — boot the canonicals on the final genesis

1. Upgrade canonical-1, -2, -3 to the release carrying the final bundle.
2. On each node, expect:
   - `registry slice CONFERRED by the trust root`;
   - `commons blobs HELD … holders=[the other two canonicals]`.
3. Claim each node (owner pen), then **restart once** (CIRISServer#710) so the owner's
   root acceptance is written.
4. Wire checks per node:
   - `GET /v1/trust-root/bundle` serves `bundle` + `community`, no wrapper signature,
     and `served_by.node_key_id` is that node's own key.
   - `GET /v1/identity` key_ids are distinct across the three nodes.
   - `POST /v1/builds` from a test pipeline returns 201 on one canonical, and the build
     is served with its bytes by the other two (anti-entropy, a few minutes).

**Rollback:** previous image plus the `.bak` snapshot the role takes before every upgrade.

---

## 7. Phase E — move registry data that has to survive

Per FSD-004 §4. Only what is not re-published from source:

| Data | Disposition |
|---|---|
| Builds and function manifests | per Decision 2. New builds arrive CEG-native from CI |
| Partners, organizations, licensure | signed operational envelopes (`partner_record`, `organization`, `licensure:{authority_id}` rows) written by their authorities into Server. **Blocking:** the Portal-as-Server-cards write path must exist before cutover, since there is no registry left to write to |
| Revocations | `SignedRevocation` through persist |
| Audit log | export and archive. Not migrated into the federation |
| Steward-signed responses | none since 4.0.0 |

**Check:** for each migrated kind, row counts and spot-checked records match between the
standalone DB and a canonical's directory.

---

## 8. Phase F — cutover

1. Point Caddy for `api/us/eu.registry.ciris-services-1.ai` at the canonical Server nodes.
2. Repeat the Phase D wire checks against the public hostnames.
3. Watch agents for a full release cycle: manifest fetches, licensure lookups, steward-key
   bundle reads.

**Rollback:** revert the Caddy upstreams. The standalone registry stays running, idle,
until Phase G; that idle window is the only rollback path, so keep it short but real
(one agent release).

---

## 9. Phase G — retire the shared key and the standalone registry

In this order:

1. **Stop the legacy re-sign path.** Turn off `POST /v1/verify/binary-manifest` and
   `/function-manifest`, where the registry signs (FSD-004 §4.2: DROP), once every CI
   emits Contributions. That is the last use of the shared private key.
2. **Keep the shared public key published** as a legacy verification key, so the 1,790
   historical manifests still verify. Name it as legacy in the bundle's documentation.
   It is never a canonical or a trust root.
3. **Shred the shared seed** on both hosts (`shred -u /opt/ciris/registry/keys/*`), and
   delete it from any vault or secret store that holds it.
4. **Decommission** the standalone registry containers, `ciris-registry-postgres`, the
   pgEdge replication between the registry hosts, and the ufw 5432 rule.
5. Close or move the registry repo's open issues (to CIRISServer, or the Constitution
   for the move-to-constitution set), then archive CIRISRegistry. Decide whether
   `ciris-registry-core` moves into CIRISServer as a crate (as `ciris-lens-core` did)
   before archiving, so Server does not depend on an archived repo.

**Check:** no running process loads `75c29fcc…`; the only references left are historical
rows and the legacy-verification listing.

---

## 10. Risks

- **Host capacity** (§1.1). An undersized canonical demotes itself to relay-only on low
  disk, and was observed near its memory cap on canonical-1.
- **Owner root acceptance** (#710). A claimed node withholds every third-party row from
  peers until restart; the ladder's `owners_accept` rung exists for this.
- **Commons roster** (CIRISEdge#785). A canonical admitted after a node boots is not a
  manifest holder for that node until it restarts.
- **Legacy manifest consumers.** With no registry left, any agent version whose manifest
  was not republished CEG-native cannot resolve its manifest (Decision 2). The shared
  pubkey stays published only so historical signatures remain checkable.

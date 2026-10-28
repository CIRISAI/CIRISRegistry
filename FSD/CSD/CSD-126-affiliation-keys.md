# CSD-126 — Affiliation keys (register, rotate, revoke, escrow — and the registry holds nothing)

**CSD**: CSD-126 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) + CSD/4 (`CIRISServer/FSD/CSD4.md`, catalogue entry T6, `csd:tests` in §3a) · **Origin**: the registry fold, FSD-004 §4.3 / §6.1; CIRISRegistry#65, #66; CIRISPersist#752
**Flow**: unwritten
**Reads with**: CSD-037 (a person's own devices and keys — the same primitives for one person), CSD-095 (the self roster's held changes), CSD-104 (key verification), CSD-121 (the `KeyManager` role that may do this), CSD-105 (the trust root), `COMMON_CARDS_ANALYSIS.md` (recommends FOLD INTO CSD-037 + CSD-055 §7 on adoption)

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**An organization's key manager can register a key the organization holds,
rotate it with a successor it committed to in advance, revoke it, and place it
in escrow with a named custodian; everyone can see which keys speak for the
organization and since when; and no service anywhere holds the organization's
private key for it.** Serves **Integrity**, on FSD-004 §6.1 (settled): the
Registry signing *as* a partner with no `delegates_to` edge was delegation
laundering (CC 4.1.1), so `GenerateKeyPair` and `RequestSignature` are gone and
stay gone. Keys are self-custodied; what the federation holds is the public
record.

**The Portal's key screens are retired, not migrated (steward, 2026-10-28).**
`ActivateKey`, `RotateKey`, `RevokeKey`, `Register*Key`,
`RequestKeyEscrow`/`RequestKeyRecovery` were operations on registry-custodied
keys, each charged per key by tier (issuance fee plus a refundable bond); with
portal.ciris.ai decommissioned there is no custodian to charge and nothing to
charge for. Here they are operations a person performs on a key they hold,
which the affiliation recognises, and the only "activation" is a signed row.

**What "the organization's key" means on rc12.** An affiliation signs through its
founders' quorum and the keys its members hold roles with (CSD-121). A key
"registered to the organization" is a member's self-signed `key:record:v1`
(persist v57 writes it on registration under the key's own name) bound to the
affiliation by a `delegates_to` edge carrying the `KeyManager` scope, cosigned
under the affiliation's protocol — CC 3.3.9 (rc12): "a keyless affiliation's key
manager holds the registration binding under the quorum's cosignature" — not a
key the affiliation owns apart from a person. The `ciris-canonical` installs
(`ciris-canonical-1`) are node keys seated by the accord's blessing, carry only
`infra:*`, and never sign a licence (CSD-122).

## 2. Surface (what)

```yaml csd:surface
surface: proposed:affiliation-keys
screen: proposed:AffiliationKeys
```

Placement: Communities and Businesses › Rules, behind the `KeyManager` role;
mirrors CSD-037's layout so a person who manages their own devices recognises
the controls.

```yaml csd:shows
registry_sha256: d587156605dd8de7c0c56e271625293eb05b3f9b5debd44cf8bc822d4e701c5b
fields:
  - ceg: x_private:key_id
    use: display-only
    type: string
    example: "wa-ciris-l3c-signing-7d2f"
    renders: "one row per key that speaks for the organization: its id, who holds it, since when"
    tag: "proposed:row_affiliation_key_*"
  - ceg: x_private:held_by
    use: display-only
    type: string
    example: "wa-registrar-3e90"
    renders: "'Held by' — the member whose self-signed record it is; never 'the registry'"
    tag: "proposed:txt_key_held_by_*"
  - ceg: x_private:bound_under
    use: display-only
    type: string
    example: "quorum:2/3 — A1, B1 signed"
    renders: "'Bound to this organization by' — the cosigned KeyManager edge (CC 3.3.9 rc12); a binding one founder signed alone reads 'not bound'"
    tag: "proposed:txt_key_bound_under_*"
  - ceg: x_private:pre_rotation_commitment
    use: emit
    type: string
    example: "52de6e5b…5736"
    renders: "'Successor committed' — the hash of the next key set, published before it is needed (CC 3.2 T3); a key with none shows 'no successor committed — rotation would be a fresh registration'"
    tag: "proposed:txt_key_successor_*"
  - ceg: x_private:rotation
    use: emit
    type: string
    example: "wa-ciris-l3c-signing-8a01"
    renders: "'Rotate to' — the successor the commitment named; a supersession, the old key's rows stay verifiable"
    tag: "proposed:btn_key_rotate_*"
  - ceg: "key:revocation"
    use: display-only
    type: string
    example: "key:revocation:v1"
    renders: "'Revoked on {when} by {who}' — revoker-signed, judged without the subject's record, never withdrawn (CC 3.1.3); `revoked_after` bounds it, no bound revokes all (CC 2.4.1.1); the row stays listed, struck through"
    tag: "proposed:row_key_revoked_*"
  - ceg: x_private:escrow_custodian
    use: emit
    type: string
    example: "wa-counsel-11be"
    renders: "'In escrow with' — a named custodian (steward, attorney, dual custody); the node keeps the custody index (persist's `registry_key_escrows`, CIRISPersist#752), the custodian keeps the material"
    tag: "proposed:txt_key_escrow_*"
  - ceg: x_private:recovery_request
    use: emit
    type: "enum[requested,approved,refused]"
    example: "requested"
    renders: "'Recovery requested by {who} on {when}' — the custodian's approval is their own signed act; the card never implies the node can release anything"
    tag: "proposed:txt_key_recovery_*"
  - ceg: "ownership:{relation}:{target_kind}:{version}"
    bind: {relation: responsible_party, target_kind: node, version: v1}
    use: display-only
    type: string
    example: "ownership:responsible_party:node:v1"
    renders: "'Responsible party' on a node key the organization operates — the single live steward (CC 3.2); an owner-binding that names an affiliation is admitted only cosigned under its protocol (persist v60 #1062, `owner_binding_affiliation_refused:*`); cardinality ≠ 1 is shown as an error, not a list"
    tag: "proposed:txt_key_responsible_*"
```

```yaml csd:states
populated: {tag: "proposed:list_affiliation_keys"}
empty:     {tag: "proposed:txt_affiliation_keys_empty", renders: "No keys registered to this organization beyond its founders'."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_affiliation_keys_error", renders: "Couldn't read this organization's keys from this node, so this list is not complete."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| register a key (self-signed `key:record:v1` + the cosigned `KeyManager` binding) | partial: a person registers their own (`/v1/federation/self-key-record`, CSD-037); the affiliation binding is CSD-124's edge | CIRISServer | `blocked_by: CIRISServer#752` (CIRISRegistry#65 was the standalone registry's ask and folds into it) |
| rotate (supersession under a pre-rotation commitment) | none | CIRISServer | `blocked_by: CIRISServer#752` |
| revoke (`key:revocation`, with or without `revoked_after`) | partial (own devices, CSD-037; `POST /v1/admin/deadmit` preview/commit per CSD4.md §5) | CIRISServer | unconfirmed |
| escrow / recovery (custody index) | none; `registry_key_escrows` exists in persist (CIRISPersist#752) | CIRISServer | `blocked_by: CIRISServer#752`; the product decision FSD-005 §7.4 asked for is now the steward's: no custodian is sold, the custodian is whoever the affiliation names |
| list keys that speak for an affiliation (outbound `KeyManager` edges with their records) | none | CIRISServer | `blocked_by: CIRISServer#663` (the same inbound/outbound chain read CSD-055 waits on) |
| an owner-binding naming an affiliation admitted only cosigned under its protocol | CIRISPersist | CIRISPersist | confirmed — v60.0.0 (#1062) |

## 3a. Topology (CSD/4)

T6 is the entry (the affiliation and the `KeyManager` edge as a `delegation`);
the revocation atoms live in **T19** (`key.revocation.revoked_after`,
`key.revocation.revoked_all`, `node.announce.announced`) and the self-roster
changes in T10. `requirements.yaml` carries no CSD-126 yet; the block is the
entry to add.

```yaml csd:tests
CSD-126:
  catalogue_version: 3
  topology: T6
  tests:
    bound:
      at: [delegation.chain, community.member]
    revoked_after:
      at: [key.revocation.revoked_after]
    revoked_all:
      at: [key.revocation.revoked_all]
```

The two revocation tests hold only in T19 (`deadmit`, `PENDING_BUILDER`); the
planner reselects it. What this card adds to the T6 layer graph (CSD-120 §3a):

```yaml
  relations:
    # PENDING (`delegation`, CSD4.md §5): the KeyManager binding, cosigned by the founders
    - {rel: delegation, from: authority, to: practitioner, scopes: [key_manager], depth: 1, over: AFF, term_days: 365}
    # PENDING: key_record(person, key) with a pre-rotation commitment; rotate(person, key, to: successor);
    #          escrow(person, key, custodian) — none typed in TOPOLOGY.md; T19's `deadmit` is the only key-plane relation
    - {rel: deadmit, person: practitioner, after: "<ts>"}                 # key.revocation.revoked_after (T19 shape)
  checks:
    relations:
      - delegation: {scopes: [key_manager], cosigned_under: founder_only, community_id: AFF}
      - deadmit: {revoked_after: "<ts>", rows_before: still_verifiable, rows_after: refused}
    actor: [reads: "keys that speak for AFF" → one row held by practitioner, bound under the founder's protocol]
  negatives:
    - {check: holds_no_row, node: C, dimension: "key:record", person: practitioner}   # the canonical holds the PUBLIC record; control: ≥1 public row
```

(The `holds_no_row` negative is listed to be honest about its control clause:
a key record IS public, so the negative as written fails as unproven; what the
card actually wants is `holds_no_private_material`, which is not a row and
cannot be a topology negative. The private key never being on any node is a
property of the client, pinned in its own tests, not of the fixture.)

## 4. Flow

Unwritten at `envisioned`. The chain: a KeyManager registers a key with a
successor commitment → the founders cosign the binding → the key signs a
licence (CSD-122) → rotation to the committed successor → the earlier licence
still verifies → revoke the old key with `revoked_after` → it is struck through
and still listed, and rows it signed before the bound still verify.

## 5. Acceptance

Unsigned. Not tested here: a person's own devices (CSD-037), the self roster's
held changes (CSD-095), the trust root's keys (CSD-105), hardware custody classes.

# CSD-126 — Affiliation keys (register, rotate, revoke, escrow — and the registry holds nothing)

**CSD**: CSD-126 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-004 §4.3 / §6.1; CIRISRegistry#65, #66; CIRISPersist#752
**Flow**: unwritten
**Reads with**: CSD-037 (a person's own devices and keys — the same primitives for one person), CSD-104 (key verification), CSD-121 (the `KeyManager` role that may do this), CSD-105 (the trust root)

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

**What "the organization's key" means on rc7.** An affiliation signs through its
founders' quorum and the keys its members hold roles with (CSD-121). A key
"registered to the organization" is a member's self-signed `SignedKeyRecord`
bound to the affiliation by a `delegates_to` edge carrying the `KeyManager`
scope (FSD-005 §4), not a key the affiliation owns apart from a person. The
Portal's `ActivateKey` / `RotateKey` / `RevokeKey` were operations on
registry-custodied keys; here they are operations a person performs on a key
they hold, which the affiliation recognises.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:affiliation-keys
screen: proposed:AffiliationKeys
```

Placement: Communities and Businesses › Rules, behind the `KeyManager` role;
mirrors CSD-037's layout so a person who manages their own devices recognises
the controls.

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
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
    renders: "'Revoked on {when} by {who}' — revoker-signed, judged without the subject's record, never withdrawn (CC 3.1.3); the row stays listed, struck through"
    tag: "proposed:row_key_revoked_*"
  - ceg: x_private:escrow_custodian
    use: emit
    type: string
    example: "wa-counsel-11be"
    renders: "'In escrow with' — a named custodian (steward, attorney, dual custody); the node keeps the custody index (CIRISPersist#752), the custodian keeps the material"
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
    renders: "'Responsible party' on a node key the organization operates — the single live steward (CC 3.2); cardinality ≠ 1 is shown as an error, not a list"
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
| register a key (self-signed `SignedKeyRecord` + the `KeyManager` binding) | partial: a person registers their own (`/v1/federation/self-key-record`, CSD-037); the affiliation binding is CSD-124's edge | CIRISServer | `blocked_by: CIRISRegistry#65` |
| rotate (supersession under a pre-rotation commitment) | none | CIRISServer, CIRISPersist | `blocked_by: FSD-005 §7` |
| revoke (`key:revocation`) | partial (own devices, CSD-037) | CIRISServer | unconfirmed |
| escrow / recovery (custody index) | none; `registry_key_escrows` exists in persist | CIRISServer | `blocked_by: CIRISPersist#752` |
| list keys that speak for an affiliation | none | CIRISServer | `blocked_by: FSD-005 §7` |

## 4. Flow

Unwritten at `envisioned`. The chain: a KeyManager registers a key with a
successor commitment → the key signs a licence (CSD-122) → rotation to the
committed successor → the earlier licence still verifies → revoke the old key →
it is struck through and still listed.

## 5. Acceptance

Unsigned. Not tested here: a person's own devices (CSD-037), the trust root's
keys (CSD-105), hardware custody classes.

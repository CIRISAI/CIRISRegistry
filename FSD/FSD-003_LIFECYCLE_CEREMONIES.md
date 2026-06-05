# FSD-003 — Self / Family / Community Lifecycle Ceremony Envelopes

**Status**: Proposed
**Date**: 2026-06-05
**Authors**: CIRIS L3C
**Tracks**: [CIRISRegistry#52](https://github.com/CIRISAI/CIRISRegistry/issues/52) Ask 1 (ceremony envelope vocabulary).
**Companion**: [CIRISPersist#161](https://github.com/CIRISAI/CIRISPersist/issues/161) (substrate revocation/removal primitives — the sister-spec for the persist side).

> **Normative-source notice.** The wire format is normative in [`FSD/CEG/`](CEG/README.md) — specifically §5.6.8.8 (`identity_occurrence`), §5.6.8.9 (`family`), §5.6.8.10 (`community`), §5.6.8.11 (`location_proof`), §4 (envelope fields `family_id` / `community_id` / `listed` / `history_on_join`), §7.7 + §7.8 (substrate-emitted reserved prefixes), §11.7 + §11.8 (governance). This FSD **does not redefine** any of those; it specifies the canonical signed **ceremony envelopes** Registry emits and CIRISVerify counter-signs — the orchestration layer that composes the locked CEG primitives into auditable lifecycle operations. Where this FSD and CEG disagree, CEG wins; report the divergence as a bug against this FSD.

---

## Why this lives here (and not in CEG, Verify, or Persist)

The CEG spec locks the lifecycle **semantics** (consensus_protocol vocabulary, Option-A forward secrecy, single-vouch self-occurrence, consensus-gated family/community membership, rough-only location). It deliberately does **not** specify the ceremony-orchestration layer — CEG §11.7.6 explicitly excludes "the CIRISPersist#152 at-rest encryption flow details (substrate-side; persist spec)"; the symmetric exclusion is that ceremony orchestration is Registry's, not CEG's.

- **CIRISVerify** is signing/verification only. It counter-signs ceremony envelopes Registry emits and aggregates witness signatures via its existing `HybridVerifier`; it does not orchestrate ceremonies. This FSD specifies the envelope schema Verify validates against, not any Verify change.
- **CIRISPersist** stores what the chain admits. Persist's current admission is value-validation only (form check); its inline docs (`src/federation/mod.rs:307-311`, `:352-357`) explicitly defer "the v3.13+ admission gate that needs the trust-graph walk." That gate's *home* is Registry. Persist's removal/revocation primitives are tracked in CIRISPersist#161.
- **CIRISRegistry** already owns user-credential lifecycle (partner records, bond, license attestations, triple witnessing) and the portal UX baseline ([`UIUX-001_PORTAL_SCREENS.md`](UIUX-001_PORTAL_SCREENS.md)). Consensus-model ceremonies are the natural extension.

---

## Table of contents

- [§0 Scope and non-goals](#0-scope-and-non-goals)
- [§1 Common ceremony-envelope frame](#1-common-ceremony-envelope-frame)
- [§2 Identity / self-collective ceremonies](#2-identity--self-collective-ceremonies)
  - [§2.1 `establish_identity`](#21-establish_identity)
  - [§2.2 `add_occurrence`](#22-add_occurrence)
  - [§2.3 `revoke_occurrence`](#23-revoke_occurrence)
- [§3 Family ceremonies](#3-family-ceremonies)
  - [§3.1 `create_family`](#31-create_family)
  - [§3.2 `add_family_member`](#32-add_family_member)
  - [§3.3 `remove_family_member`](#33-remove_family_member)
  - [§3.4 `amend_family_consensus_protocol`](#34-amend_family_consensus_protocol)
  - [§3.5 `dissolve_family`](#35-dissolve_family)
- [§4 Community ceremonies](#4-community-ceremonies)
  - [§4.1 `create_community`](#41-create_community)
  - [§4.2 `add_community_member`](#42-add_community_member)
  - [§4.3 `remove_community_member`](#43-remove_community_member)
  - [§4.4 `location_proof` ceremony](#44-location_proof-ceremony)
  - [§4.5 `listed: public` opt-in](#45-listed-public-opt-in)
- [§5 Ceremony → consensus_protocol gate map](#5-ceremony--consensus_protocol-gate-map)
- [§6 Persist ↔ Registry boundary](#6-persist--registry-boundary)
- [§7 Reserved-prefix emission triggers](#7-reserved-prefix-emission-triggers)
- [§8 Known gaps flagged for review](#8-known-gaps-flagged-for-review)
- [§9 What this spec does NOT do](#9-what-this-spec-does-not-do)

---

## §0 Scope and non-goals

### §0.1 What this FSD locks

The canonical signed ceremony-envelope vocabulary for every self / family / community lifecycle operation named in CIRISRegistry#52 Ask 1. For each ceremony: the payload fields, who signs (proposer), the witness-set requirement, what CIRISVerify counter-signs, and the resulting substrate effect (which CIRISPersist V059 / V060 / V061 row it produces).

### §0.2 What this FSD does NOT cover

See [§9](#9-what-this-spec-does-not-do). In short: this FSD does **not** implement the signature-counting predicate (Ask 2 — separate phase), does **not** build the portal UX (Ask 4), and does **not** re-spec the wire format (CEG is normative).

### §0.3 Key-id representation

All key-id fields in every envelope below are **plain strings** (`federation_keys.key_id` values — hybrid Ed25519 + ML-DSA public-key identifiers). This FSD does **not** introduce a `KeyId` newtype; the federation key-ids are plain strings across the ecosystem, and the ceremony layer follows suit. (CIRISPersist#161's `Vec<KeyId>` sketch should likewise be read as `Vec<String>`.)

### §0.4 The witness-set verifiability invariant

Every ceremony envelope carries a `witness_set` — a list of `{ signer_key_id, signature }` pairs where each `signature` is over the **canonical envelope bytes** (the JCS-canonical form of the envelope `payload` per CEG §0.9, excluding the `witness_set` and `verify_countersignature` members themselves). This is the load-bearing invariant: the witness set MUST be cryptographically verifiable by any peer, not merely a count. CIRISVerify's `HybridVerifier` multi-sig aggregation validates each witness signature against the same canonical bytes before counter-signing. A `witness_set` whose signatures do not verify over the canonical envelope bytes is malformed and rejected at the gate.

**Reuse the substrate's existing `WitnessSet` type — do NOT invent a ceremony-private one.** The federation substrate already ships `WitnessSet` (`ciris_persist::cirisnode::types::WitnessSet`, types.rs:105) carried on `ContributionEnvelope.witness_set` (types.rs:164), and NodeCore's engine already enforces a **jump-threshold witness-set gate** (NodeCore §3.5/§3.7 — a `witness_set` is required whenever an action "jumps" a target's standing: expertise-attestation jumping target standing, registry-vouch jumping K_C coherence, moderation events always). **Ceremony admission is an instance of that same jump-threshold pattern**: a family/community member-add *jumps the roster* (changes standing), so it carries a `witness_set` and the gate enforces it. The ceremony `witness_set` IS `ContributionEnvelope.witness_set`, not a parallel field; the §5 gate IS the jump-threshold gate specialized to membership-roster changes, calling the same `evaluate_consensus_protocol` predicate (NodeCore, shipped) the rest of the jump-threshold surface uses. This keeps one `WitnessSet` type, one jump-threshold semantic, and one consensus predicate across the federation.

---

## §1 Common ceremony-envelope frame

Every lifecycle ceremony is a single signed JSON object with this outer frame. The per-ceremony sections below specify only the `payload` member; the frame is invariant.

```json
{
  "ceremony_kind": "<one of the kinds in §2-§4>",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:abc123...",
  "proposed_at": "2026-06-05T14:00:00Z",
  "payload": { /* per-ceremony, §2-§4 */ },
  "witness_set": [
    {
      "signer_key_id": "key:ed25519+mldsa:abc123...",   // the proposer self-witnesses
      "signature": "base64(hybrid-sig over canonical(payload))"
    },
    {
      "signer_key_id": "key:ed25519+mldsa:def456...",
      "signature": "base64(hybrid-sig over canonical(payload))"
    }
  ],
  "verify_countersignature": {
    "verifier_key_id": "key:ed25519+mldsa:verify-tenant-log...",
    "signature": "base64(hybrid-sig over canonical(payload ‖ witness_set))",
    "countersigned_at": "2026-06-05T14:00:03Z"
  }
}
```

| Frame field | Required | Meaning |
|---|---|---|
| `ceremony_kind` | yes | The lifecycle operation. Closed set per §2-§4. Selects which `payload` schema applies (payload discriminator, parallel to CEG §4 `subject_kind`). |
| `ceremony_version` | yes | Ceremony-envelope schema version. `"1"` for this FSD. |
| `proposer_key_id` | yes | The proposer's `federation_keys.key_id` (string). The proposer MUST be a member authorized to propose this ceremony per [§5](#5-ceremony--consensus_protocol-gate-map). |
| `proposed_at` | yes | RFC 3339 canonical per CEG §0.5. |
| `payload` | yes | Per-ceremony object (§2-§4). The unit over which `witness_set` signatures are computed (JCS-canonical per CEG §0.9). |
| `witness_set` | yes | List of `{ signer_key_id, signature }`. Each signature is a hybrid Ed25519 + ML-DSA signature over `canonical(payload)`. Cardinality + which keys may sign are governed by the ceremony's consensus gate ([§5](#5-ceremony--consensus_protocol-gate-map)). Minimum one entry (the proposer self-witnesses). |
| `verify_countersignature` | added by Verify | Absent in the proposed envelope; populated by CIRISVerify **after** the consensus gate passes (Ask 2 predicate). Verify signs over `canonical(payload ‖ witness_set)` — binding both the payload AND the witness set it validated. This is what makes the counter-signature a receipt for "the gate saw this exact witness set and it satisfied the protocol." |

**Why Verify counter-signs over `payload ‖ witness_set`, not `payload` alone.** The counter-signature must attest to *the validated witness set*, not merely the payload. If Verify signed only the payload, a relay could swap a satisfying witness set for a non-satisfying one and the counter-signature would still verify. Binding both closes that gap and gives persist a single field to verify (the counter-signature) that transitively covers the witness set.

---

## §2 Identity / self-collective ceremonies

Self-occurrence admission is **single-vouch** per CEG §11.7.4 — NOT consensus-gated. The witness-set rule for §2 ceremonies is fixed by CEG, independent of any consensus_protocol.

### §2.1 `establish_identity`

Bootstrap a fresh root `identity_key_id`. This is the genesis ceremony — there is no prior occurrence to vouch, so the root key self-witnesses.

```json
{
  "ceremony_kind": "establish_identity",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-06-05T14:00:00Z",
  "payload": {
    "identity_key_id": "key:ed25519+mldsa:alice-root...",
    "root_occurrence": {
      "occurrence_key_id": "key:ed25519+mldsa:alice-root...",
      "device_class": "laptop",
      "hardware_attestation": null,
      "asserted_at": "2026-06-05T14:00:00Z",
      "valid_until": null
    }
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG | Notes |
|---|---|---|---|
| `identity_key_id` | yes | §5.6.8.8 root | The new logical identity. String. |
| `root_occurrence` | yes | §5.6.8.8 `identity_occurrence` | The first occurrence. `occurrence_key_id` MAY equal `identity_key_id` (the root key is its own first device) or name a distinct device key the root vouches for at genesis. |
| `root_occurrence.device_class` | yes | §5.6.8.8 closed set | `phone \| laptop \| server \| embedded \| agent \| service`. |
| `root_occurrence.hardware_attestation` | no | §5.6.8.8 | base64 TPM/Secure-Enclave/StrongBox/SGX blob; `null` for software-only. |

- **Proposer**: the root `identity_key_id`.
- **Witness-set requirement**: exactly one — the root key self-signs (`attesting_key_id == identity_key_id` per CEG §5.6.8.8 admission). No external witness.
- **Verify counter-signs**: the genesis binding `(identity_key_id, root_occurrence)`. There is no consensus gate; Verify validates the self-signature and counter-signs.
- **Substrate effect**: one `put_identity_occurrence(SignedIdentityOccurrence)` call → one V059 `federation_identity_occurrences` row for the root occurrence.
- **Reserved-prefix emission**: `hard_case:identity_occurrence_added:{identity_key_id}` (CEG §7.7) — the genesis occurrence is the first "added" occurrence. See [§7](#7-reserved-prefix-emission-triggers).

### §2.2 `add_occurrence`

Admit a new occurrence (device / agent) to an existing identity. **Single-vouch** per CEG §11.7.4: signed by the root `identity_key_id` OR any currently-admitted occurrence of that identity.

```json
{
  "ceremony_kind": "add_occurrence",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-laptop...",
  "proposed_at": "2026-06-05T14:05:00Z",
  "payload": {
    "identity_key_id": "key:ed25519+mldsa:alice-root...",
    "occurrence": {
      "occurrence_key_id": "key:ed25519+mldsa:alice-phone...",
      "device_class": "phone",
      "hardware_attestation": "base64(strongbox-blob)...",
      "asserted_at": "2026-06-05T14:05:00Z",
      "valid_until": null
    }
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-laptop...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG | Notes |
|---|---|---|---|
| `identity_key_id` | yes | §5.6.8.8 | The identity admitting the new occurrence. String. |
| `occurrence` | yes | §5.6.8.8 `identity_occurrence` | Same shape as §2.1 `root_occurrence`. |

- **Proposer**: the vouching key — either `identity_key_id` itself OR any currently-admitted occurrence of `identity_key_id` (Signal-style "trust any device I've already onboarded").
- **Witness-set requirement**: exactly one — the single vouch. The signer MUST be `identity_key_id` OR a current occurrence of it. The gate (Ask 2) verifies the signer's membership in the identity's current occurrence set via persist's `list_identity_occurrences_active(identity_key_id)`.
- **Verify counter-signs**: the `(identity_key_id, occurrence)` binding after confirming the single-vouch rule.
- **Substrate effect**: one `put_identity_occurrence(SignedIdentityOccurrence)` → one new V059 row.
- **Reserved-prefix emission**: `hard_case:identity_occurrence_added:{identity_key_id}` (CEG §7.7).
- **Optional higher assurance**: consumer policy MAY require a non-null `hardware_attestation` (CEG §11.7.4). This FSD does not mandate it; it is a consumer-side layered requirement.

### §2.3 `revoke_occurrence`

Remove an occurrence from a self-collective. Forward-secrecy (Option A) per CEG §11.7.1: the removed occurrence retains existing `key_grant`s for historical content; substrate stops wrapping new `key_grant`s.

```json
{
  "ceremony_kind": "revoke_occurrence",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-06-06T09:00:00Z",
  "payload": {
    "identity_key_id": "key:ed25519+mldsa:alice-root...",
    "occurrence_key_id": "key:ed25519+mldsa:alice-old-phone...",
    "revoked_at": "2026-06-06T09:00:00Z",
    "effective_at": "2026-06-06T09:00:00Z",
    "reason": "device lost"
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG / Persist | Notes |
|---|---|---|---|
| `identity_key_id` | yes | §5.6.8.8 | The identity revoking an occurrence. String. |
| `occurrence_key_id` | yes | §5.6.8.8 revocation | The occurrence being evicted. String. |
| `revoked_at` | yes | #161 `IdentityOccurrenceRevocation` | When the proposer issued the revocation. |
| `effective_at` | yes | #161 | When forward-secrecy takes effect (≥ `revoked_at`). |
| `reason` | no | #161 | Free-text; observability only. |

- **Proposer**: the root `identity_key_id` OR any currently-admitted occurrence (a `withdraws` against an `identity_occurrence` is issued by `identity_key_id` or any current occurrence per CEG §5.6.8.8). Same single-vouch authority as `add_occurrence`.
- **Witness-set requirement**: exactly one — the single vouch (same rule as §2.2). The signer MUST be a *currently-active* member of the identity's occurrence set. **A revoked occurrence cannot witness its own re-admission or further revocations** once `effective_at` has passed.
- **Verify counter-signs**: the revocation binding after confirming the single-vouch rule against the *active* occurrence set.
- **Substrate effect**: one `put_identity_occurrence_revocation(SignedIdentityOccurrenceRevocation)` (CIRISPersist#161 Ask 1) → one V061 `federation_identity_occurrence_revocations` row. This flips the occurrence to inactive in `list_identity_occurrences_active(...)` (CIRISPersist#161 Ask 2) and triggers the Option-A forward-secrecy gate (CIRISPersist#161 Ask 4 — stop wrapping new `key_grant`s to the revoked occurrence).
- **Reserved-prefix emission**: **GAP — see [§8.1](#81-occurrence-removal-has-no-symmetric-reserved-prefix-ceg-spec-amendment).** CEG §7.7 has `identity_occurrence_added` (add only) with no symmetric removal prefix.

---

## §3 Family ceremonies

Family membership changes are **consensus-gated** per CEG §11.7.5 — NOT single-vouch. The family's `consensus_protocol` field governs the witness-set requirement. See [§5](#5-ceremony--consensus_protocol-gate-map) for the per-protocol gate.

### §3.1 `create_family`

Bootstrap a `family_key_id` with founding members and a chosen consensus_protocol.

```json
{
  "ceremony_kind": "create_family",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-06-05T15:00:00Z",
  "payload": {
    "family_key_id": "key:ed25519+mldsa:acme-household...",
    "family_name": "Acme Household",
    "members": [
      { "key_id": "key:ed25519+mldsa:alice-root...", "joined_at": "2026-06-05T15:00:00Z", "role": "founder" },
      { "key_id": "key:ed25519+mldsa:bob-root...",   "joined_at": "2026-06-05T15:00:00Z", "role": "founder" }
    ],
    "founded_at": "2026-06-05T15:00:00Z",
    "consensus_protocol": "founder_only",
    "consensus_protocol_entrenched": false
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." },
    { "signer_key_id": "key:ed25519+mldsa:bob-root...",   "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG §5.6.8.9 | Notes |
|---|---|---|---|
| `family_key_id` | yes | `family_key_id` | The family's own federation key. String. |
| `family_name` | yes | `family_name` | Human-readable; non-unique. |
| `members[]` | yes | `members` | Each `{ key_id, joined_at, role }`. `key_id` is an **identity** key (NOT an occurrence key). `role ∈ founder \| member \| null`. |
| `founded_at` | yes | `founded_at` | RFC 3339 canonical. |
| `consensus_protocol` | yes | `consensus_protocol` | One of the six canonical kinds or `custom:{id}`. Locked at creation. |
| `consensus_protocol_entrenched` | yes | `consensus_protocol_entrenched` | If `true`, [§3.4](#34-amend_family_consensus_protocol) is refused at the gate. |

- **Proposer**: any founder.
- **Witness-set requirement**: **all founders sign** the genesis envelope. Rationale: there is no prior family snapshot to evaluate a consensus_protocol against, so the bootstrap rule is unanimity-of-founders (the founding set must all consent to forming the family — the protocol takes effect *for subsequent* changes). This is the one family ceremony whose witness rule is NOT read from `consensus_protocol`; it is fixed by this FSD. **Flagged for review — see [§8.2](#82-create_family-bootstrap-witness-rule-is-under-specified-in-ceg).**
- **Verify counter-signs**: the genesis family binding after confirming all founders signed.
- **Substrate effect**: one `put_family(SignedFamily)` → one V059 `federation_families` row.
- **Reserved-prefix emission**: `hard_case:family_membership_change:{family_key_id}` (CEG §7.7) — the founding roster is the first "membership change."

### §3.2 `add_family_member`

Admit a new identity to `family.members`. Consensus-gated per the family's current `consensus_protocol` (CEG §11.7.5). Rides the `supersedes` primitive — the new envelope supersedes the current family Contribution with the expanded roster.

```json
{
  "ceremony_kind": "add_family_member",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:bob-root...",
  "proposed_at": "2026-06-07T18:00:00Z",
  "payload": {
    "family_key_id": "key:ed25519+mldsa:acme-household...",
    "supersedes_row_hash": "sha256:current-family-row...",
    "added_member": {
      "key_id": "key:ed25519+mldsa:carol-phone-root...",
      "joined_at": "2026-06-07T18:00:00Z",
      "role": "member",
      "valid_until": "2026-06-14T18:00:00Z"
    },
    "resulting_members": [
      { "key_id": "key:ed25519+mldsa:alice-root...", "role": "founder" },
      { "key_id": "key:ed25519+mldsa:bob-root...",   "role": "founder" },
      { "key_id": "key:ed25519+mldsa:carol-phone-root...", "role": "member", "valid_until": "2026-06-14T18:00:00Z" }
    ]
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:bob-root...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG | Notes |
|---|---|---|---|
| `family_key_id` | yes | §5.6.8.9 | String. |
| `supersedes_row_hash` | yes | `supersedes` primitive | The `persist_row_hash` of the family Contribution this one supersedes. Pins the membership snapshot the consensus gate evaluates against (prevents racing two adds against stale rosters). |
| `added_member` | yes | §5.6.8.9 `members` entry | `{ key_id, joined_at, role, valid_until? }`. |
| `resulting_members[]` | yes | §5.6.8.9 `members` | The full post-add roster — what the V059 row becomes. |

- **Proposer**: any current member (CEG §5.6.8.9 membership-change ceremony step 1).
- **Witness-set requirement**: **governed by the family's `consensus_protocol`** evaluated against the membership snapshot pinned by `supersedes_row_hash`. See [§5](#5-ceremony--consensus_protocol-gate-map). If the rule is not yet satisfied, the proposal is held pending until additional member signatures arrive (CEG §5.6.8.9 step 2 — configurable window, operator policy).
- **Verify counter-signs**: the expanded roster after the consensus gate (Ask 2 predicate) confirms the witness set satisfies the protocol.
- **Substrate effect**: one `put_family(SignedFamily)` with `resulting_members` → supersedes the prior V059 row. On admission, substrate emits retroactive `key_grant`s wrapping all `cohort_scope: family` content DEKs to the new member (CEG §5.6.8.9 step 3 / CIRISPersist#152).
- **Reserved-prefix emission**: `hard_case:family_membership_change:{family_key_id}` (CEG §7.7).

### §3.3 `remove_family_member`

Drop a member. Consensus-gated; forward-secrecy (Option A) applies.

```json
{
  "ceremony_kind": "remove_family_member",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-06-15T10:00:00Z",
  "payload": {
    "family_key_id": "key:ed25519+mldsa:acme-household...",
    "supersedes_row_hash": "sha256:current-family-row...",
    "removed_identity_key_id": "key:ed25519+mldsa:carol-phone-root...",
    "removed_at": "2026-06-15T10:00:00Z",
    "effective_at": "2026-06-15T10:00:00Z",
    "reason": "visit ended",
    "resulting_members": [
      { "key_id": "key:ed25519+mldsa:alice-root...", "role": "founder" },
      { "key_id": "key:ed25519+mldsa:bob-root...",   "role": "founder" }
    ]
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG / Persist | Notes |
|---|---|---|---|
| `family_key_id` | yes | §5.6.8.9 | String. |
| `supersedes_row_hash` | yes | `supersedes` | Snapshot pin (as §3.2). |
| `removed_identity_key_id` | yes | #161 `FamilyMembershipRevocation` | The identity being removed. String. |
| `removed_at` / `effective_at` | yes | #161 | Forward-secrecy effective time. |
| `reason` | no | #161 | Observability only. |
| `resulting_members[]` | yes | §5.6.8.9 | Post-removal roster. |

- **Proposer**: any current member.
- **Witness-set requirement**: governed by the family's `consensus_protocol` against the pinned snapshot ([§5](#5-ceremony--consensus_protocol-gate-map)). **The removed member's own signature does NOT count toward quorum for their own removal** (a member cannot be required to consent to their own eviction, nor block it by abstaining — the protocol counts the *remaining* roster). **Flagged for review — see [§8.3](#83-self-removal-counting-is-not-addressed-by-ceg-1175).**
- **Verify counter-signs**: the reduced roster after the gate confirms quorum.
- **Substrate effect**: `put_family(SignedFamily)` with `resulting_members` (supersedes the row) **AND** `put_family_membership_revocation(SignedFamilyMembershipRevocation)` (CIRISPersist#161 Ask 1) → one V061 `federation_family_membership_revocations` row. The revocation row drives `list_family_members_active(...)` and the Option-A forward-secrecy gate (stop wrapping new `key_grant`s to the removed member — CIRISPersist#161 Ask 4). The removed member retains existing grants (CEG §11.7.1).
- **Reserved-prefix emission**: `hard_case:family_membership_change:{family_key_id}` (CEG §7.7 — symmetric: emitted on both add and remove per CEG §5.6.8.9 step description "addition or removal").

### §3.4 `amend_family_consensus_protocol`

Meta-amendment of the family's `consensus_protocol`. Per CEG §11.7.5 + §5.6.8.9: admitted ONLY IF (a) `consensus_protocol_entrenched == false` AND (b) the **CURRENT** protocol's rule is satisfied on the amendment envelope. MUST be refused if entrenched.

```json
{
  "ceremony_kind": "amend_family_consensus_protocol",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-07-01T12:00:00Z",
  "payload": {
    "family_key_id": "key:ed25519+mldsa:acme-household...",
    "supersedes_row_hash": "sha256:current-family-row...",
    "current_consensus_protocol": "founder_only",
    "new_consensus_protocol": "majority",
    "new_consensus_protocol_entrenched": false
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." },
    { "signer_key_id": "key:ed25519+mldsa:bob-root...",   "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG §5.6.8.9 | Notes |
|---|---|---|---|
| `family_key_id` | yes | | String. |
| `supersedes_row_hash` | yes | `supersedes` | Snapshot pin. |
| `current_consensus_protocol` | yes | | Echoed for audit; MUST match the current row or the gate rejects (stale-amendment guard). |
| `new_consensus_protocol` | yes | | The replacement protocol. |
| `new_consensus_protocol_entrenched` | yes | | MAY flip `false → true` (entrench going forward); MAY NOT be set on an already-entrenched family (the whole ceremony is refused). |

- **Proposer**: any current member.
- **Witness-set requirement**: governed by the **CURRENT** `consensus_protocol` (not the new one) per CEG §5.6.8.9 — the amendment must pass the rules in force at proposal time. See [§5](#5-ceremony--consensus_protocol-gate-map).
- **Entrenchment gate**: if the current family row has `consensus_protocol_entrenched == true`, this ceremony is **REFUSED at the gate** regardless of witness set (CEG §5.6.8.9). Verify does NOT counter-sign; the substrate emits `hard_case:family_consensus_protocol_violation:{family_key_id}`.
- **Verify counter-signs**: the amended protocol after confirming non-entrenchment AND that the current protocol's rule is satisfied.
- **Substrate effect**: one `put_family(SignedFamily)` superseding the row with the new `consensus_protocol`. Persist's current cut admits on consensus_protocol *form* only; the entrenchment-rejection + current-rule-satisfaction is the Registry-side gate (CIRISPersist `src/federation/mod.rs:352-357` defers it).
- **Reserved-prefix emission**: on success `hard_case:family_consensus_protocol_change:{family_key_id}`; on refusal (entrenched OR rule-unsatisfied) `hard_case:family_consensus_protocol_violation:{family_key_id}` (CEG §7.7).

### §3.5 `dissolve_family`

Terminal ceremony — wind the family down. No CEG primitive names dissolution explicitly; it is modeled as a consensus-gated terminal `supersedes` to an empty roster + a dissolution marker.

```json
{
  "ceremony_kind": "dissolve_family",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-08-01T00:00:00Z",
  "payload": {
    "family_key_id": "key:ed25519+mldsa:acme-household...",
    "supersedes_row_hash": "sha256:current-family-row...",
    "dissolved_at": "2026-08-01T00:00:00Z",
    "reason": "household dissolved",
    "resulting_members": []
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." },
    { "signer_key_id": "key:ed25519+mldsa:bob-root...",   "signature": "..." }
  ]
}
```

| Payload field | Required | Notes |
|---|---|---|
| `family_key_id` | yes | String. |
| `supersedes_row_hash` | yes | Snapshot pin. |
| `dissolved_at` | yes | Terminal timestamp. |
| `reason` | no | Observability. |
| `resulting_members` | yes | MUST be `[]`. The dissolution is the empty-roster terminal state. |

- **Proposer**: any current member.
- **Witness-set requirement**: governed by the family's `consensus_protocol`, evaluated against the **full current roster** (everyone the protocol counts). Dissolution is the maximal-consequence membership change; this FSD treats it as removal-of-all and applies the same gate. **Flagged for review — see [§8.4](#84-dissolution-is-not-a-ceg-primitive).**
- **Verify counter-signs**: the terminal empty-roster binding after the gate.
- **Substrate effect**: `put_family(SignedFamily)` with `resulting_members: []` superseding the row + `put_family_membership_revocation(...)` for each prior member (CIRISPersist#161). All members lose forward `key_grant`s per Option A; historical grants retained.
- **Reserved-prefix emission**: `hard_case:family_membership_change:{family_key_id}` (terminal removal of all).

---

## §4 Community ceremonies

Communities mirror families (same six consensus_protocols, same supersedes-based membership changes) but are **NOT structurally invisible** — community content emits `holds_bytes:sha256:*` and federates per status quo (CEG §5.6.8.10; §8.1.13.3). There is **no at-rest DEK cascade**, so the §3 `key_grant`-cascade language does NOT apply: community membership changes gate *visibility* via the roster + `community_id` envelope field, not byte-level DEK wraps.

### §4.1 `create_community`

```json
{
  "ceremony_kind": "create_community",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-06-05T16:00:00Z",
  "payload": {
    "community_key_id": "key:ed25519+mldsa:austin-community...",
    "community_name": "Austin",
    "cohort_subkind": "geographic",
    "cohort_subkind_payload": {
      "geographic_constraint": { "cell_id": "85283473fffffff", "cell_resolution": 5 }
    },
    "members": [
      { "key_id": "key:ed25519+mldsa:alice-root...", "joined_at": "2026-06-05T16:00:00Z", "role": "founder" },
      { "key_id": "key:ed25519+mldsa:bob-root...",   "joined_at": "2026-06-05T16:00:00Z", "role": "founder" }
    ],
    "founded_at": "2026-06-05T16:00:00Z",
    "consensus_protocol": "majority",
    "consensus_protocol_entrenched": false
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." },
    { "signer_key_id": "key:ed25519+mldsa:bob-root...",   "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG §5.6.8.10 | Notes |
|---|---|---|---|
| `community_key_id` | yes | `community_key_id` | String. |
| `community_name` | yes | `community_name` | Non-unique. |
| `cohort_subkind` | yes | `cohort_subkind` | Open vocab; canonical `geographic`. |
| `cohort_subkind_payload` | conditional | `cohort_subkind_payload` | REQUIRED for `geographic` — carries `geographic_constraint { cell_id, cell_resolution }`. Community-side `cell_resolution` is NOT bounded to ≤ 7 (that bound applies only to `location_proof`). |
| `members[]` / `founded_at` / `consensus_protocol` / `consensus_protocol_entrenched` | yes | as §5.6.8.10 | Same shape as family. |

- **Proposer**: any founder.
- **Witness-set requirement**: all founders sign (same bootstrap rule as `create_family` §3.1; [§8.2](#82-create_family-bootstrap-witness-rule-is-under-specified-in-ceg) applies symmetrically).
- **Verify counter-signs**: genesis community binding.
- **Substrate effect**: one `put_community(SignedCommunity)` → one V060 `federation_communities` row.
- **Reserved-prefix emission**: `hard_case:community_membership_change:{community_key_id}` (CEG §7.8).

### §4.2 `add_community_member`

Mirror of `add_family_member` (§3.2) with the community gate. For `cohort_subkind: geographic`, admission **additionally** requires the candidate's most-recent `location_proof` (within `valid_until`) to be contained in `geographic_constraint.cell_id` (CEG §5.6.8.10; §0.8.2 containment).

```json
{
  "ceremony_kind": "add_community_member",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-06-08T11:00:00Z",
  "payload": {
    "community_key_id": "key:ed25519+mldsa:austin-community...",
    "supersedes_row_hash": "sha256:current-community-row...",
    "added_member": { "key_id": "key:ed25519+mldsa:dave-root...", "joined_at": "2026-06-08T11:00:00Z", "role": "member" },
    "location_proof_ref": "sha256:dave-location-proof-contribution...",
    "resulting_members": [
      { "key_id": "key:ed25519+mldsa:alice-root...", "role": "founder" },
      { "key_id": "key:ed25519+mldsa:bob-root...",   "role": "founder" },
      { "key_id": "key:ed25519+mldsa:dave-root...",  "role": "member" }
    ]
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." },
    { "signer_key_id": "key:ed25519+mldsa:bob-root...",   "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG | Notes |
|---|---|---|---|
| `community_key_id` | yes | §5.6.8.10 | String. |
| `supersedes_row_hash` | yes | `supersedes` | Snapshot pin. |
| `added_member` | yes | §5.6.8.10 `members` entry | `{ key_id, joined_at, role }`. |
| `location_proof_ref` | conditional | §5.6.8.11 | REQUIRED iff `cohort_subkind == geographic`. The content-hash of the candidate's `location_proof` Contribution. The gate checks containment of its `cell_id` in the constraint. |
| `resulting_members[]` | yes | §5.6.8.10 | Post-add roster. |

- **Proposer**: any current member.
- **Witness-set requirement**: the community's `consensus_protocol` ([§5](#5-ceremony--consensus_protocol-gate-map)) **AND**, for `geographic`, a valid contained `location_proof` for the candidate. The location_proof is a *precondition*, not a witness signature; it is referenced, not counted toward quorum.
- **Verify counter-signs**: the expanded roster after gate (consensus + geographic containment).
- **Substrate effect**: one `put_community(SignedCommunity)` superseding the V060 row. **No** retroactive `key_grant` cascade (community has no at-rest DEK).
- **Reserved-prefix emission**: `hard_case:community_membership_change:{community_key_id}` (CEG §7.8).

### §4.3 `remove_community_member`

Mirror of `remove_family_member` (§3.3) with the community gate. Forward-only leave per CEG §11.8.2 — the historical roster + any `location_proof` remain in the audit chain.

```json
{
  "ceremony_kind": "remove_community_member",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:alice-root...",
  "proposed_at": "2026-09-01T08:00:00Z",
  "payload": {
    "community_key_id": "key:ed25519+mldsa:austin-community...",
    "supersedes_row_hash": "sha256:current-community-row...",
    "removed_identity_key_id": "key:ed25519+mldsa:dave-root...",
    "removed_at": "2026-09-01T08:00:00Z",
    "effective_at": "2026-09-01T08:00:00Z",
    "reason": "moved away",
    "resulting_members": [
      { "key_id": "key:ed25519+mldsa:alice-root...", "role": "founder" },
      { "key_id": "key:ed25519+mldsa:bob-root...",   "role": "founder" }
    ]
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:alice-root...", "signature": "..." },
    { "signer_key_id": "key:ed25519+mldsa:bob-root...",   "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG / Persist | Notes |
|---|---|---|---|
| `community_key_id` | yes | §5.6.8.10 | String. |
| `supersedes_row_hash` | yes | `supersedes` | Snapshot pin. |
| `removed_identity_key_id` | yes | #161 `CommunityMembershipRevocation` | String. |
| `removed_at` / `effective_at` | yes | #161 | Forward-only effective time. |
| `reason` | no | #161 | Observability. |
| `resulting_members[]` | yes | §5.6.8.10 | Post-removal roster. |

- **Proposer**: any current member.
- **Witness-set requirement**: the community's `consensus_protocol` against the pinned snapshot; the removed member's signature does NOT count toward their own removal (same rule as §3.3 / [§8.3](#83-self-removal-counting-is-not-addressed-by-ceg-1175)).
- **Verify counter-signs**: reduced roster after gate.
- **Substrate effect**: `put_community(SignedCommunity)` superseding the row + `put_community_membership_revocation(SignedCommunityMembershipRevocation)` (CIRISPersist#161) → one V061 `federation_community_membership_revocations` row, driving `list_community_members_active(...)`. The withdrawn `location_proof` (if any) is NOT expunged — per CEG §11.8.2 it remains in the audit chain; forward visibility evicts only.
- **Reserved-prefix emission**: `hard_case:community_membership_change:{community_key_id}` (CEG §7.8 — symmetric add/remove).

### §4.4 `location_proof` ceremony

The geographic-community disclosure ceremony. Wire-format-enforced rough-only: `cell_resolution ≤ 7` (CEG §11.8.1 / §0.8.1). Emitted **only** by the subject (`attesting_key_id == subject_key_id`) or a `delegates_to` chain with `scope: [consent_revocation]` (CEG §11.8.3) — the substrate has no path to mint a location_proof for a non-consenting key.

```json
{
  "ceremony_kind": "location_proof",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:dave-root...",
  "proposed_at": "2026-06-08T10:30:00Z",
  "payload": {
    "subject_key_id": "key:ed25519+mldsa:dave-root...",
    "cell_id": "87283472bffffff",
    "cell_resolution": 7,
    "asserted_at": "2026-06-08T10:30:00Z",
    "valid_until": "2026-07-08T10:30:00Z",
    "attestation_evidence": null
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:dave-root...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG §5.6.8.11 | Notes |
|---|---|---|---|
| `subject_key_id` | yes | `subject_key_id` | The asserting party. MUST equal `proposer_key_id` (or the proposer is a `delegates_to` proxy with `scope: [consent_revocation]`). |
| `cell_id` | yes | `cell_id` | H3 cell, lowercase hex per §0.8. |
| `cell_resolution` | yes | `cell_resolution` | **MUST be ≤ 7** per §0.8.1 / §11.8.1. Wire-format-enforced; `> 7` is rejected at the substrate gate. |
| `asserted_at` | yes | `asserted_at` | RFC 3339 canonical. |
| `valid_until` | no | `valid_until` | `null` = indefinite (consumer policy SHOULD treat as stale after 30 days). |
| `attestation_evidence` | no | `attestation_evidence` | Optional hardware-attested location blob from ciris-keyring (TPM / Secure Enclave); `null` for self-asserted. |

- **Proposer**: the subject only (`subject_key_id`), or a `delegates_to` proxy with `scope: [consent_revocation]` (CEG §11.8.3).
- **Witness-set requirement**: exactly one — the subject's self-signature. This is NOT a consensus ceremony; truth-grounding is consumer-side (the community's admission protocol decides whether to *believe* the claim; CEG §5.6.8.11). The substrate does not verify location truth.
- **Verify counter-signs**: the subject's self-asserted location binding after confirming (a) `cell_resolution ≤ 7` and (b) the signer is the subject (or authorized proxy).
- **Substrate effect**: a `location_proof` Contribution (`scores` attestation_type, `subject_kind: location_proof`). Typically `cohort_scope: federation` (the disclosure IS the opt-in). This is NOT a V059/V060 row — it is a standard Contribution; the community-admission ceremony (§4.2) *references* it via `location_proof_ref`.
- **Reserved-prefix emission on rejection**: `hard_case:location_proof_resolution_violation` against the producer's `key_id` if `cell_resolution > 7` (CEG §7.8 — observability for malformed-client patterns, NOT a slashing trigger per §11.8.1).
- **Withdrawal**: a `withdraws` against the `location_proof` evicts *forward* visibility; the historical claim remains in the audit chain (CEG §11.8.2). Optional `consent_record` with `stance: revoked` composes the deletion-compel surface (CEG §5.6.8.11).

### §4.5 `listed: public` opt-in

Per-membership roster-visibility opt-in (CEG §4 `listed` envelope field; CEG 0.10). Default absent: the roster is producer- and self-queryable but NEVER globally enumerable. Opting in is a one-way disclosure the member chooses; the substrate does NOT solicit it (mirrors the §11.8.3 location opt-in discipline).

This is not a standalone lifecycle ceremony that creates a roster row; it is a **per-membership flag** a member sets on their own membership. It is specified here as a ceremony so the opt-in is itself an auditable, signed act.

```json
{
  "ceremony_kind": "set_membership_listed",
  "ceremony_version": "1",
  "proposer_key_id": "key:ed25519+mldsa:dave-root...",
  "proposed_at": "2026-06-09T09:00:00Z",
  "payload": {
    "community_key_id": "key:ed25519+mldsa:austin-community...",
    "member_key_id": "key:ed25519+mldsa:dave-root...",
    "listed": "public"
  },
  "witness_set": [
    { "signer_key_id": "key:ed25519+mldsa:dave-root...", "signature": "..." }
  ]
}
```

| Payload field | Required | Maps to CEG §4 | Notes |
|---|---|---|---|
| `community_key_id` | yes | — | The community whose roster the member opts into being listed in. String. (MAY be a `family_key_id` if a family adopts the same opt-in; the field is named for the community case, the dominant one.) |
| `member_key_id` | yes | — | MUST equal `proposer_key_id`. A member can only opt **themselves** in — no one lists another member. |
| `listed` | yes | `listed` envelope field | Value `public`. Absence (no ceremony) = default unlisted. |

- **Proposer**: the member themselves (`member_key_id == proposer_key_id`).
- **Witness-set requirement**: exactly one — the member's self-signature. NOT consensus-gated: listing oneself is a self-disclosure, like `location_proof`. **Flagged for review — see [§8.5](#85-listed-public-is-an-envelope-field-not-a-ceremony-in-ceg).**
- **Verify counter-signs**: the self-listed opt-in.
- **Substrate effect**: the member's subsequent community-scoped Contributions carry `listed: public` in the CEG §4 envelope; consumers MAY enumerate that member in the roster. No new V059/V060 row kind — `listed` is an envelope field, so this ceremony's durable effect is the signed opt-in record + the member's go-forward envelope flag. (Whether persist materializes a per-membership `listed` column is a CIRISPersist follow-up, not specified here.)
- **Opt-out**: a `withdraws` against the opt-in reverts to default-unlisted going forward (forward-only, same as §4.4).

---

## §5 Ceremony → consensus_protocol gate map

Each ceremony must pass a gate before Verify counter-signs. The gate is the **signature-counting predicate** — Ask 2 of CIRISRegistry#52, a **separate issue/phase**. This FSD names *which* gate each ceremony invokes and the witness-set rule per protocol; it does NOT implement the predicate (see [§9](#9-what-this-spec-does-not-do)).

**Single-implementation rule (the gate CALLS NodeCore's shipped predicate — it does NOT reimplement it).** The signature-counting predicate already exists: NodeCore shipped `ciris_node_core::evaluate_consensus_protocol(protocol, current_members, witness_sigs) → ConsensusResult` (commit [7469dcc](https://github.com/CIRISAI/CIRISNodeCore/commit/7469dcc), per [CIRISNodeCore#30](https://github.com/CIRISAI/CIRISNodeCore/issues/30)) — all six canonical kinds + `WeightedRubricResolver` / `CustomPredicateResolver` hooks, non-member-sig filtering, duplicate-sig dedup. The Ask-2 Registry gate is **orchestration that calls this predicate**, NOT a fresh `match protocol.as_str() { … }` reimplementation. A reimplementation would be the *third* copy of signature-counting (Registry gate + NodeCore evaluator + any Persist re-check) and would drift the moment one adds a 7th protocol kind. The predicate is implemented **exactly once** (NodeCore); the gate's novel surface is witness-set collection → call the predicate → entrenchment refusal → Verify counter-sign → emit Contribution. This composes in-process under the CEWP single-process cohabitation model (registry-core + node-core + persist in one process). Full boundary table at [CIRISRegistry#52 comment](https://github.com/CIRISAI/CIRISRegistry/issues/52#issuecomment-4635757400).

The §5.2 per-protocol rules below are therefore the **documented contract of NodeCore's `evaluate_consensus_protocol` arms**, not a second specification of them.

### §5.1 Per-ceremony gate

| Ceremony | Gate | Witness rule source |
|---|---|---|
| `establish_identity` | self-vouch (genesis) | This FSD §2.1 — root self-signs, exactly 1 |
| `add_occurrence` | single-vouch | CEG §11.7.4 — root OR any current occurrence, exactly 1 |
| `revoke_occurrence` | single-vouch | CEG §5.6.8.8 — root OR any current occurrence, exactly 1 |
| `create_family` | founder-unanimity (bootstrap) | This FSD §3.1 — all founders (CEG silent; [§8.2](#82-create_family-bootstrap-witness-rule-is-under-specified-in-ceg)) |
| `add_family_member` | family `consensus_protocol` | CEG §11.7.5 / §5.6.8.9 |
| `remove_family_member` | family `consensus_protocol` (over remaining roster) | CEG §11.7.5 + §11.7.1; self-exclusion [§8.3](#83-self-removal-counting-is-not-addressed-by-ceg-1175) |
| `amend_family_consensus_protocol` | **current** family `consensus_protocol` + entrenchment refusal | CEG §5.6.8.9 |
| `dissolve_family` | family `consensus_protocol` (over full roster) | This FSD §3.5 ([§8.4](#84-dissolution-is-not-a-ceg-primitive)) |
| `create_community` | founder-unanimity (bootstrap) | This FSD §4.1 (CEG silent; [§8.2](#82-create_family-bootstrap-witness-rule-is-under-specified-in-ceg)) |
| `add_community_member` | community `consensus_protocol` (+ geographic containment) | CEG §5.6.8.10 + §11.8 |
| `remove_community_member` | community `consensus_protocol` (over remaining roster) | CEG §5.6.8.10 |
| `location_proof` | self-vouch (subject only) | CEG §11.8.3 — subject or `delegates_to` proxy, exactly 1 |
| `set_membership_listed` | self-vouch (member only) | This FSD §4.5 ([§8.5](#85-listed-public-is-an-envelope-field-not-a-ceremony-in-ceg)) |

### §5.2 Witness-set rule per consensus_protocol

For the consensus-gated ceremonies, the witness-set requirement per the six canonical CEG §5.6.8.9 protocols, evaluated against the membership snapshot pinned by `supersedes_row_hash`:

| Protocol | Witness-set rule (Ask 2 predicate) |
|---|---|
| `founder_only` | ≥ 1 valid witness signature from a member with `role: founder`. |
| `unanimous` | A valid witness signature from **every** current member counted by the rule (for removal/dissolution: every member except the one being removed — [§8.3](#83-self-removal-counting-is-not-addressed-by-ceg-1175)). |
| `majority` | Valid witness signatures from **> 50%** of the counted roster. |
| `quorum:{m}/{n}` | Valid witness signatures from **≥ m** members — `m` is **absolute** (never rescales with roster growth); `n` is documentary. **RESOLVED in CEG §8.1.12.3.1** (was flagged at [§8.6](#86-quorummn-n-vs-live-roster-size-precedence-is-resolved)): a `quorum:2/3` collective grown to 5 still admits at 2 sigs. Operators wanting roster-proportional quorum use `weighted:{rubric}`, not `quorum:`. |
| `weighted:{rubric}` | Sum of member weights (per the named operator rubric) of the valid witnesses **exceeds the rubric's threshold**. The rubric is operator-defined; **CEG does not specify where the rubric's weight table or threshold lives — see [§8.7](#87-weightedrubric-rubric-resolution-is-not-specified)**. |
| `custom:{id}` | Operator-defined predicate. CEG §11.7.5 leaves this fully open; the gate dispatches to an operator-registered predicate. **Not resolvable from CEG — operator-supplied.** |

**All six rules share the [§0.4](#04-the-witness-set-verifiability-invariant) verifiability invariant**: a "valid witness signature" is one that cryptographically verifies (via Verify's `HybridVerifier`) over the canonical envelope bytes AND whose `signer_key_id` is a current member of the family/community at the pinned snapshot. Counting an unverifiable or non-member signature is a gate error.

---

## §6 Persist ↔ Registry boundary

### §6.1 The wire that carries the validated witness set

Registry emits the counter-signed ceremony envelope as a Contribution. Persist absorbs it via the existing `put_*` entrypoints (CIRISPersist `src/federation/mod.rs`):

| Ceremony | Persist entrypoint | Substrate version |
|---|---|---|
| `establish_identity`, `add_occurrence` | `put_identity_occurrence(SignedIdentityOccurrence)` | V059 |
| `revoke_occurrence` | `put_identity_occurrence_revocation(SignedIdentityOccurrenceRevocation)` | V061 (CIRISPersist#161 Ask 1) |
| `create_family`, `add_family_member`, `amend_family_consensus_protocol`, `dissolve_family` | `put_family(SignedFamily)` | V059 |
| `remove_family_member`, `dissolve_family` | `put_family_membership_revocation(SignedFamilyMembershipRevocation)` | V061 (CIRISPersist#161 Ask 1) |
| `create_community`, `add_community_member` | `put_community(SignedCommunity)` | V060 |
| `remove_community_member` | `put_community_membership_revocation(SignedCommunityMembershipRevocation)` | V061 (CIRISPersist#161 Ask 1) |
| `location_proof` | standard Contribution put (not a V059/V060 row) | — |

**Persist v4.0 DAS convergence (verified 2026-06-05 against `../CIRISPersist`).** Persist's own [v4.0 Data Access Surface FSD](https://github.com/CIRISAI/CIRISPersist/blob/main/FSD/V4_0_DATA_ACCESS_SURFACE.md) §4.6 **already defers to this FSD** — its `put_*` admission table reads: *"`put_identity_occurrence` / `put_family` / `put_community` — Membership-roster writes — admission is the ceremony witness-set (CIRISRegistry#52), not a cohort_scope claim; no downgrade concept."* The two specs are convergent; Persist is waiting on FSD-003, not diverging from it. Confirmed substrate state:
- `src/federation/` is **NOT** moved by the v4.0 `src/read/* → src/ceg/*` reorg (DAS §3.3 explicit) — the `put_*` entrypoints below keep their `src/federation/` home; the `src/ceg/{family,community,identity}/` modules are the *read-surface* that consumes the federation substrate, not a replacement.
- Substrate tables are `federation_keys` / `federation_identity_occurrences` / `federation_families` / `federation_communities`. V060 (`federation_communities`) **landed** (Persist commit `bae5e72`). V059 (identity_occurrence + family) present. V061 (revocations) is Persist#161-pending.
- `src/ceg/streaming/` exists — the CEG 0.10 streaming substrate (Persist#142) is beginning to land.

### §6.2 The `witness_set` field is the boundary contract

Persist's current admission is **value-validation only** (form check) — its `src/federation/mod.rs` inline docs (the `self-vouch / single-vouch admission per §5.6.8.8 ... v3.13+ admission gate` + `consensus-protocol enforcement ... v3.13+ admission gate` notes, ~mod.rs:308/311/356) defer the consensus enforcement to "the v3.13+ admission gate." This FSD pins the field that carries the Registry-validated decision across the boundary:

- Each `Signed*` substrate type carries a `witness_set` field (CIRISPersist#161 sketches `witness_set: Vec<KeyId>` on the revocation types; read as `Vec<String>` per [§0.3](#03-key-id-representation)). For the ceremony layer the carried value is the **full `{ signer_key_id, signature }` set from the envelope**, not just the key-ids — so persist can re-verify cryptographically, not merely trust a count.
- Persist's `engine.rs` already has a `witness_set: None` slot (`src/engine.rs:2276`) — the materialization point for this contract.
- **What persist verifies**: persist re-checks each witness signature over the canonical envelope bytes (via CIRISVerify's `HybridVerifier` multi-sig aggregation) AND verifies the `verify_countersignature` over `canonical(payload ‖ witness_set)`. The counter-signature transitively attests the gate ran; the witness-set re-verification is defense-in-depth so persist never admits a Registry-asserted membership change it cannot itself cryptographically confirm.
- **What persist does NOT do**: persist does NOT re-run the consensus-counting predicate (that is the Registry-side gate, Ask 2). Persist verifies signatures and the counter-signature; it trusts the *count decision* to Registry + Verify. This keeps the trust-graph walk upstream (CEG §11.7.6 symmetric exclusion) and persist append-only.

### §6.3 Active-state read semantics

The revocation rows (V061) drive persist's `list_*_active(...)` read paths (CIRISPersist#161 Ask 2): a binding is active iff it was admitted AND no matching revocation exists with `effective_at <= now()`. `CallerAdmission` resolution (CIRISPersist#160 §4.1) populates `family_key_ids` + `community_key_ids` from the active set — so a removed member stops being admitted at the read boundary, closing the naively-inclusive gap CIRISPersist#161 names.

---

## §7 Reserved-prefix emission triggers

Per CEG §7.7 + §7.8, the `hard_case:*` membership-event prefixes are **substrate-emitted** (`attesting_key_id` MUST be an `identity_type="substrate_persist"` key). Registry does NOT emit them directly. Registry's role is to decide a ceremony **succeeded** (gate passed, Verify counter-signed) and absorb it into persist via §6.1; **persist's substrate emission path** then emits the reserved-prefix event as a side effect of the row write. The trigger semantics:

| Ceremony outcome | Substrate emits | When |
|---|---|---|
| `establish_identity` / `add_occurrence` admitted | `hard_case:identity_occurrence_added:{identity_key_id}` | On V059 row write |
| `revoke_occurrence` admitted | **GAP — no symmetric prefix** ([§8.1](#81-occurrence-removal-has-no-symmetric-reserved-prefix-ceg-spec-amendment)) | On V061 row write |
| `create_family` / `add_family_member` / `remove_family_member` / `dissolve_family` admitted | `hard_case:family_membership_change:{family_key_id}` | On V059/V061 row write (symmetric add/remove) |
| `amend_family_consensus_protocol` admitted | `hard_case:family_consensus_protocol_change:{family_key_id}` | On V059 row write |
| `amend_family_consensus_protocol` refused (entrenched OR rule unsatisfied) | `hard_case:family_consensus_protocol_violation:{family_key_id}` | On gate rejection |
| `create_community` / `add_community_member` / `remove_community_member` admitted | `hard_case:community_membership_change:{community_key_id}` | On V060/V061 row write |
| community consensus_protocol amended | `hard_case:community_consensus_protocol_change:{community_key_id}` | On V060 row write |
| community amendment refused | `hard_case:community_consensus_protocol_violation:{community_key_id}` | On gate rejection |
| `location_proof` rejected (`cell_resolution > 7`) | `hard_case:location_proof_resolution_violation` (against producer `key_id`) | On gate rejection |

The trigger boundary: Registry tells persist *"this ceremony is admitted"* by calling the corresponding `put_*`; persist emits the reserved-prefix event as part of that write. The full trigger-semantics spec (when exactly Registry decides a ceremony "succeeded enough") is Ask 5 of CIRISRegistry#52 — a later phase. This FSD names the mapping; it does not spec the Ask 5 trigger machinery.

---

## §8 Known gaps flagged for review

These are points where CEG §11.7 / §11.8 (or the envelope layer) is ambiguous or silent for an implementable ceremony schema. Per the issue's instruction, they are reported rather than invented-around.

### §8.1 Occurrence-removal has no symmetric reserved prefix (CEG-spec amendment)

CEG §7.7 has `hard_case:identity_occurrence_added:{identity_key_id}` (**add only**) and `hard_case:family_membership_change:{family_key_id}` (**add-or-change** — symmetric). There is **no symmetric occurrence-removal reserved prefix**. `revoke_occurrence` (§2.3) therefore has no `hard_case:*` event to emit on success, breaking the observability symmetry families already have.

**Recommendation (preferred): rename `hard_case:identity_occurrence_added:*` → `hard_case:identity_occurrence_change:*`** in CEG §7.7, mirroring the family `_membership_change` symmetry (which covers both add and remove under one prefix). This is the cleanest fix: it makes occurrence and family membership-event observability structurally parallel, and it matches the precedent CEG already set for families. The substrate emits the same prefix on both admit and revoke, with consumers distinguishing direction from the row delta.

**Alternative (if a clean break is undesirable): add `hard_case:identity_occurrence_removed:{identity_key_id}`** as a new prefix alongside the existing `_added`. This preserves the existing prefix but doubles the occurrence event surface (add and remove as distinct prefixes — asymmetric with the family single-prefix model).

**Fallback (no amendment): `revoke_occurrence` emits no reserved-prefix event.** Acceptable but loses substrate-self-report observability for occurrence removal — operators cannot watch occurrence-eviction the way they watch family membership change. Not recommended.

**This is a CEG-spec amendment, not a Registry-only call.** It must route through the CEG §11.2 amendment process (federation Contribution + WA quorum + 1-of-6 sign-off). This FSD recommends the rename and flags it for the CEG-authority maintainers; it cannot land the change unilaterally. CIRISPersist#161 Ask 5 makes the parallel observation from the substrate side ("ensure the substrate emission path covers both directions").

### §8.2 `create_family` bootstrap witness rule is under-specified in CEG

CEG §5.6.8.9 specifies the witness rule for membership *changes* against an existing family ("the CURRENT family's `consensus_protocol`"), but a **genesis family has no current snapshot** — there is no prior roster to evaluate a protocol against. CEG does not name the bootstrap witness rule. This FSD adopts **founder-unanimity** (all founders sign the genesis envelope) as the most defensible default — forming a family requires every founder's consent — but this is an FSD choice, not a CEG-locked rule. Same gap applies to `create_community` (§4.1). **Recommend CEG add an explicit bootstrap-witness sentence to §5.6.8.9 / §5.6.8.10.**

### §8.3 Self-removal counting is not addressed by CEG §11.7.5

For `remove_family_member` / `remove_community_member`, CEG §5.6.8.9 says membership changes are gated by the `consensus_protocol` but does NOT say whether the member being removed counts toward (or can block) their own removal quorum. Two intuitions conflict: (a) a member should not be forced to consent to their own eviction, nor block it by abstaining — count the *remaining* roster; (b) under `unanimous`, requiring the removee's signature makes involuntary removal impossible. This FSD adopts **interpretation (a): the removed member is excluded from the counted roster for their own removal** (the protocol counts the remaining members). This is the only interpretation that makes involuntary removal possible under `unanimous`/`quorum`. **Flagged: CEG should state the self-exclusion rule explicitly.**

### §8.4 Dissolution is not a CEG primitive

No CEG section names family/community dissolution. This FSD models `dissolve_family` (§3.5) as a consensus-gated terminal `supersedes` to an empty roster. The witness rule (full-roster consensus) is an FSD choice. CEG may prefer to (a) bless this empty-roster modeling, (b) add an explicit dissolution ceremony/`subject_kind`, or (c) declare dissolution out of scope (families just go dormant). **Flagged for CEG decision.** (Community dissolution is not separately specced here; it would mirror §3.5 if §8.4 resolves toward (a) or (b).)

### §8.5 `listed: public` is an envelope field, not a ceremony, in CEG

CEG §4 defines `listed` as a per-Contribution envelope field (CEG 0.10), not a lifecycle ceremony. §4.5 here lifts it into a signed `set_membership_listed` ceremony so the opt-in is itself auditable. This is a modeling choice: the alternative is that `listed: public` is simply set on each Contribution with no standalone ceremony record. **Flagged: confirm whether a standalone signed opt-in record is wanted, or whether the per-Contribution envelope flag suffices.** If the latter, drop the §4.5 ceremony and treat `listed` purely as a §6 envelope-field passthrough.

### §8.6 `quorum:{m}/{n}` — `n` vs live roster size precedence is RESOLVED

~~CEG §5.6.8.9 leaves it ambiguous whether the literal `{n}` is fixed or rescales with the live roster.~~ **RESOLVED in CEG [§8.1.12.3.1](CEG/08_composition.md)** (pinned 2026-06-05): **`m` is absolute, `n` is documentary.** A `quorum:2/3` collective grown to 5 members still admits at **2** signatures; `m` never rescales. Rationale: matches NodeCore's shipped `evaluate_consensus_protocol` (already treats `m` as absolute), simpler invariant for a deterministic gate, and roster-proportional quorum stays expressible via `weighted:{rubric}` (members weight 1, threshold = `ceil(roster/2)` recomputed by the rubric resolver). Applies identically to community admission (CEG §8.1.13.2). The gate is no longer under-specified for non-entrenched `quorum:` families whose roster ≠ `{n}`.

### §8.7 `weighted:{rubric}` rubric resolution is not specified

CEG §5.6.8.9 names `weighted:{rubric}` ("sum of member weights per a named operator rubric must exceed a threshold") but does not specify **where the rubric's weight table or threshold lives** — is it carried in `cohort_subkind_payload`, a separate Contribution, an operator-side registry? The Ask 2 predicate cannot count weighted consensus without a resolution path from `{rubric}` → `{weight_table, threshold}`. **Flagged: CEG (or this FSD in a later cut) must specify rubric resolution before `weighted:` is implementable.** `custom:{id}` has the same open-endedness but is explicitly operator-supplied by design, so it is not a gap — `weighted:` reads as if it should be substrate-computable but lacks the inputs.

---

## §9 What this spec does NOT do

- **Does NOT implement the signature-counting predicate.** The `check_consensus_admission(...)` gate (counting witness signatures per the protocol's rule) is **Ask 2** of CIRISRegistry#52 — a separate issue/phase. This FSD names which gate each ceremony invokes ([§5](#5-ceremony--consensus_protocol-gate-map)) and the per-protocol witness rule, but the deterministic predicate is not specified here.
- **Does NOT build portal UX.** The create-family wizard, membership-management screens, occurrence-onboarding QR flow, community founder UX, and amendment-ceremony feedback are **Ask 4** (extends [`UIUX-001_PORTAL_SCREENS.md`](UIUX-001_PORTAL_SCREENS.md)) — a later phase.
- **Does NOT re-spec the wire format.** CEG §5.6.8.8 / §5.6.8.9 / §5.6.8.10 / §5.6.8.11 + §4 envelope shape + §7.7/§7.8 reserved prefixes are **normative**. This FSD composes those locked primitives into ceremony envelopes; it does not redefine them. Where this FSD and CEG disagree, CEG wins.
- **Does NOT spec CIRISVerify changes.** Verify's `HybridVerifier` already does multi-sig aggregation. This FSD specifies the envelope schema Verify validates against and the `verify_countersignature` it adds; no new Verify primitive is required.
- **Does NOT build the persist substrate primitives.** The `put_*_revocation` methods + V061 tables + Option-A forward-secrecy gate are **CIRISPersist#161**. This FSD specifies the ceremony envelopes those primitives absorb and the boundary contract ([§6](#6-persist--registry-boundary)); it does not implement the substrate side.
- **Does NOT introduce a `KeyId` newtype.** Key fields are plain strings throughout ([§0.3](#03-key-id-representation)).
- **Does NOT spec the Ask 5 reserved-prefix trigger machinery.** [§7](#7-reserved-prefix-emission-triggers) names the ceremony → prefix mapping; the full "when does Registry decide a ceremony succeeded enough to instruct persist to emit" trigger semantics are Ask 5.

---

## §10 References

- [`FSD/CEG/05_namespace.md`](CEG/05_namespace.md) §5.6.8.8 / §5.6.8.9 / §5.6.8.10 / §5.6.8.11 — wire-format primitives (normative)
- [`FSD/CEG/04_envelope.md`](CEG/04_envelope.md) §4 — envelope fields (`family_id`, `community_id`, `listed`, `history_on_join`)
- [`FSD/CEG/07_reserved.md`](CEG/07_reserved.md) §7.7 + §7.8 — substrate-emitted reserved prefixes
- [`FSD/CEG/11_governance.md`](CEG/11_governance.md) §11.7 + §11.8 — self/family + geographic-community governance (normative semantics)
- [`FSD/FSD-002_FEDERATION_SURFACE.md`](FSD-002_FEDERATION_SURFACE.md) §5 — envelope-schema documentation style this FSD mirrors
- [`FSD/UIUX-001_PORTAL_SCREENS.md`](UIUX-001_PORTAL_SCREENS.md) — portal UX baseline (Ask 4 extends)
- [CIRISRegistry#52](https://github.com/CIRISAI/CIRISRegistry/issues/52) — parent issue (this FSD = Ask 1)
- [CIRISPersist#161](https://github.com/CIRISAI/CIRISPersist/issues/161) — sister-spec: substrate revocation/removal primitives
- CIRISPersist `src/federation/mod.rs:307-311` + `:352-357` — the deferred "v3.13+ admission gate" this FSD homes at Registry
- CIRISPersist `src/federation/mod.rs:312` (`put_identity_occurrence`) / `:358` (`put_family`) / `:391` (`put_community`); `src/engine.rs:2276` (`witness_set` materialization slot)

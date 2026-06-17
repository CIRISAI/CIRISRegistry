# Part V — Transport & Substrate

**CC decimal range** `5.x` · **35 concepts** · **page budget 11.0pp** (∝ importance) · [← master index](README.md)

> Byte-level content transport, structural invisibility, epoch keying, and delivery — how content
> moves without leaking what it is.

The grammar ([Part II](part_2_the_grammar.md)) carries *claims*; it does not carry *bytes*. An
attestation says "the blob at this SHA-256 is the medical-triage adapter v3"; the blob itself — the
installer, the config file, the family photo, the live video chunk — travels a different road. Part V
is that road. Its single thesis, the one every mechanism here serves: **content can move and be
delivered without the network learning that it exists.** Structural invisibility (5.2) hides not just
the bytes but the *fact of the bytes* — and that is a privacy floor, enforced by the wire format
rather than by an operator's promise. It is [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence)
and [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) made structural: the federation
cannot leak what it was never able to carry.

Two ideas lead because the importance graph leads with them. **Epoch keying** (5.1) is how a stream
of content stays confidential as its audience changes — the cryptographic rotation that makes
"unsubscribe" mean something. **Structural invisibility** (5.2) is the privacy floor: self- and
family-scoped content never emits the directory attestation that would announce it to the rest of the
network. Everything below them — the endpoint shapes, the witness directory, the streaming
machinery — is the substrate that makes those two promises operational and verifiable.

---

## 5.1 `epoch` — Epoch keying + cascade (normative — D2 / D3; substrate-pending #142)
<sub>budget 1.34pp · import #12 · from **§10.5.3** (ceg)</sub>

A live stream has an audience, and audiences change. The problem epoch keying solves is the one that
makes "you have been removed from this stream" a real cryptographic event rather than a UI gesture:
once someone leaves, the next bytes must be sealed under a key they do not hold. **An epoch is a
keyspace generation** — a window during which every chunk is sealed under one data-encryption key
(DEK), bounded so that membership change forces a new key.

**The cost shape — O(1) seal, O(N) distribute.** The stream-epoch DEK seals content **O(1)** (one
symmetric key, AES-256-GCM); the per-subscriber `key_grant` cascade distributes that 32-byte epoch
key **O(N) per epoch** — the sender-key / Megolm shape, [§8.1.12.4](part_4_composition_governance.md)'s
Policy-L cascade applied to a community roster against a *rotating* key. The expensive asymmetric
work is the distribution, not the sealing; the design keeps it on the cold path (rekey) and off the
hot path (per-chunk seal).

**PQC at rest is MANDATORY.** The epoch-DEK cascade MUST wrap the DEK with **`wrap_algorithm: v2 =
x25519 + ml-kem-768`** (hybrid, FIPS 203) — never `v1` (X25519-only). The DEK protects content that
may persist indefinitely, so a classical-only KEM is a harvest-now-decrypt-later exposure even though
the content AEAD and the wrap *signature* are already PQC-safe. A consumer MUST reject a streaming
epoch grant carrying `wrap_algorithm: v1`. The wire string is pinned byte-exactly:
`x25519_mlkem768_aes256_gcm_hkdf_sha256` (matching `ciris-crypto` `KEY_GRANT_ALGORITHM_V2`); a
mismatch silently fails grant decode. The **full PQC envelope for streaming**: content = AES-256-GCM ·
DEK wrap = X25519+ML-KEM-768 · authenticity = Ed25519+ML-DSA-65 · hashes = SHA-256 · in-transit =
the two-layer hybrid wrap (5.3.3.5, E1). This is [fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)
projected forward in time: the safe-against-a-future-adversary posture is the *default*, with no
operator toggle to weaken it.

**The epoch index** is monotonic, per-`stream_id`, greenfield — a separate addressing axis from
`key_grant.rotation_chain`. Supersession reuses the same `rotation_chain` payload-level lineage on the
new `(stream_id, epoch[, recipient])` axis; the wire shape is unchanged, the addressing axis is new.

> ⚠️ **Not pure-additive at the Persist constraint layer.** The epoch-key axis carries a NULL
> `media_content_sha256`, which today's V054 cross-column CHECK (requiring `key_grant` rows be
> content-addressed) would reject. Introducing the axis requires a **parallel CHECK-arm migration**
> (content-addressed OR stream/epoch-addressed) — a bounded constraint migration, not a pure
> index-add. The spec does not claim "purely additive" at that layer.

**Epoch triggers (D3).** Three events govern rotation, and the asymmetry between them is the
forward-secrecy guarantee:

| Trigger | Behavior | Forward-secrecy implication |
|---|---|---|
| **Member removal** | MANDATORY rotation (coalesced; exempt for ungated public broadcasts) | Subsequent epochs sealed under a DEK the removed member doesn't have |
| **Member addition** | NO rotation + Option-A catch-up (subject to `history_on_join`) | New member gets grants for the current epoch + optionally prior epochs |
| **Time / bytes** | Optional hygiene rotation | Operator policy; default off |

**Removal coalescing (normative).** At broadcast scale, naive per-removal rotation is unaffordable:
at N = 10⁶ with realistic churn (~30%/hr), one rotation per departure ≈ 83 epochs/s → a flat-unicast
cascade of **~3.1 Tbps, exceeding the content fan-out itself**. The substrate therefore MUST
**coalesce** all removals admitted within one STH cadence window **T (default 2 s)** into a single
epoch rotation. This caps the rotation rate at 1/T regardless of churn (~18.7 Gbps at N = 10⁶, ~0.9%
of content fan-out) and bounds a removed viewer's exposure to **≤ T** — the same grain the stream's
equivocation window already accepts.

**Public-broadcast exemption (normative).** A `live_stream` whose roster is *ungated* (`listed:
public`, grants issued to any requester with no admission ceremony) carries **no confidentiality
claim against departed viewers** — anyone, including the departed viewer, can re-subscribe and receive
the current DEK. For such streams, member removal MUST NOT force rotation (it buys nothing and costs
the full cascade). Rotation-on-removal remains MANDATORY for **every gated roster**, where the DEK
*is* the access-control boundary and the forward-only-unsubscribe guarantee is real.

**Rekey conforms to MLS TreeKEM ([RFC 9420](https://www.rfc-editor.org/rfc/rfc9420), normative).** The
epoch-DEK rekey on member change is the MLS TreeKEM construction — an O(log N) path rekey, commit
signed once, with RFC 9420 blank-node / unmerged-leaf handling and parent-hash integrity. The hybrid
KEM (X25519+ML-KEM-768) is the ciphersuite's HPKE KEM. **Delivery is decisive**: TreeKEM's advantage
is *multicast aggregation* (one commit serves all, O(log N) egress); over unicast the commit must
reach each member (O(N log N)), in which case the **flat per-member cascade (O(N)) is
competitive-to-better**. The substrate selects per deployment based on whether the transport
multicasts, and the choice is **wire-invisible** (tree position rides the opaque `key_grant` payload).
**Post-Compromise Security** is now available (inherent to TreeKEM key-updates — a compromised member
heals on the next commit) and is OPTIONAL, operator-enabled; forward secrecy is forward-only by
default.

**Catch-up bound (P4).** `min(operator depth cap, chunk-eviction horizon)`. Three windows that are
NOT conflated: chunk-eviction horizon ≠ the `holds_bytes` 24 h TTL (5.3.2.1) ≠ grant durability. A
catch-up request against an evicted epoch returns **`ContentMiss` — fail-honest, no silent gap**.
Operators MUST ship the P4 cap *with* the cascade, else 10⁶ grant Contributions per rekey is the
unbounded worst case.

---

## 5.2 `family` — Structural invisibility — `holds_bytes:sha256:*` suppression for `cohort_scope: self | family`
<sub>budget 1.04pp · import #19 · from **§10.1.4** (ceg)</sub>

This is the privacy floor, and it is the most quietly radical thing in Part V. The load-bearing
claim, from [ciris.ai/cewp](https://ciris.ai/cewp):

> Self and family content never emits the attestation that would tell the rest of the network it
> exists. You don't need a privacy policy to keep family photos off the federation — the wire format
> can't carry them in the first place.

When a Contribution carries `cohort_scope: self` OR `cohort_scope: family`, the substrate MUST NOT
emit a corresponding `holds_bytes:sha256:{prefix}` directory attestation. The content's bytes reach
admitted members of the self-collective or family via the at-rest encryption flow — never via the
public holder-discovery directory. **The property is structural, not policy:**

- A non-member peer cannot issue `ContentFetch` for the bytes, because no `holds_bytes:*` attestation
  names a holder.
- A non-member peer cannot even *discover* the bytes exist — the only attestations referencing them
  are scoped to the cohort and never federate beyond it.

This is the wire-format closure of the cewp **structural invisibility** claim: privacy emerges from
format constraints, not operator policy or legal undertaking. It is also why the federation *scales*
— the **locality dividend**: `cohort_scope: self|family` content (family photos, personal notes,
in-household chatter) is the bulk of daily activity, and because it never federates, ~65% of activity
stays local in the cewp scaling model. Operators do not configure this; the wire format enforces it.

> **Scope (normative).** Structural invisibility buys **content-holding confidentiality only** — it
> is NOT relationship-existence, metadata, traffic-analysis, or unobservability privacy. The bounding
> non-goals are stated canonically at [§1.13.3](part_1_foundation.md#1133-adversary--adversary-model--privacy-non-goals);
> do not represent CEG as providing the stronger properties. Stating the limit is itself an act of
> [integrity](part_1_foundation.md#18-integrity--integrity).

**Two layers, not one (normative split).** (1) **Structural invisibility** — suppressing
`holds_bytes:*` plus non-propagation beyond scope — is the **unconditional** privacy promise; it holds
even when the at-rest bytes are plaintext, because no discovery attestation federates. (2) **At-rest
encryption** — the §8.1.12.4 DEK cascade — is **defense-in-depth** against local-disk forensics, host
operator, and cloud-substrate operator; it is operator-policy and MAY default off as a v1 migration
posture, **but when enabled MUST use `wrap_algorithm: v2`** (hybrid PQC) — never v1. The 1.0 target is
at-rest-on for self/family.

**Recipient resolution + fail-secure exclusion.** The wrap target is **not** a recipient's signing
key — `wrap_algorithm: v2` needs the recipient's `{x25519, ml_kem_768}` **content-KEM** keys, which
the recipient self-certifies via its `identity_occurrence.encryption_pubkeys`. Because this layer
mandates v2, a recipient whose current occurrence carries no valid ML-KEM-768 key MUST be **fail-secure
*excluded*** from the grant: the content stays encrypted and unreachable to it, and the substrate MUST
NOT fall back to plaintext or to `wrap_algorithm: v1`. A missing key denies access, never downgrades
the protection — [non-maleficence / fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)
made concrete.

**Exclusion MUST NOT be silent.** A bare `skip` makes a fail-secure exclusion indistinguishable from
"the family went quiet" — a soft-censorship vector. On every fail-secure skip the substrate MUST emit
**`hard_case:recipient_excluded:{scope_key_id}`** *into the affected self/family scope itself*,
carrying the excluded recipient's `key_id`, a reason, and the skipped Contribution's envelope ref — so
the excluded member has something to audit and remediate. The event is cohort-scoped: it MUST NOT
federate beyond the self/family (the invisibility promise is preserved — the *fact* of the family's
content is not leaked by its exclusion events).

**Composition.** When self/family at-rest encryption is enabled, persist wraps the DEK under each
admitted occurrence's key (self) or each member's key (family); new occurrence / new family-member
admission triggers retroactive `key_grant` emission for all extant cohort content (the "I bought a new
phone and want my history" / "I added Carol to the household" flows).

**Boundary cases.** `cohort_scope: community | affiliations | federation` content emits `holds_bytes:*`
per status-quo — CEG 0.7 changes ONLY the self/family path. A `self` Contribution later promoted via
`supersedes` to `community` emits `holds_bytes:*` at promotion time on the **new** Contribution; the
original's bytes stay structurally-invisible. A `self` note Alice writes *about* Bob stays in Alice's
self-collective — Bob receives no key_grant (two distinct identities), though Bob's subject-side
revocation authority over the note still composes; the bytes never reach Bob without Alice's explicit
re-emit at a higher scope including him.

---

## 5.3 `endpoint` — Endpoint shapes
<sub>budget 0.22pp · import #132 · from **§10** (ceg)</sub>

CEG specifies five public plus one admin HTTP endpoint shape for the discovery and cosigning surfaces.
Wire-format consumers — CIRISVerify v3.1.0+, the CIRISAgent KMP UI, the iOS/Android FFI — read these.
The subsections below give the witness directory (5.3.1), the byte-transport substrate (5.3.2), the
streaming surface (5.3.3), steward discovery (5.3.4), the remaining Registry endpoints (5.3.5), and
the common response shape (5.3.6) that frames them all.

### 5.3.1 `witness` — STH cosigning + witness directory
<sub>budget 0.89pp · import #25 · from **§10.3** (ceg)</sub>

Transparency logs are only as trustworthy as the witnesses watching them. A witness cosigns a Signed
Tree Head (STH) — a commitment to `(tree_size, root_hash, signed_at)` — and a quorum of independent
cosignatures is what turns "the log says X" into "many independent parties watched the log say X and
none saw it fork." This is the structural backbone of [integrity](part_1_foundation.md#18-integrity--integrity):
the log cannot lie consistently to everyone at once.

**`POST /v1/transparency/sth/cosign` (public).** A witness posts a cosignature on
`(tree_size, root_hash, signed_at)`. The Registry verifies hybrid Ed25519 + ML-DSA-65 against the
witness pubkey in the directory and persists on success. The canonical bytes the witness MUST sign:

```
canonical = sha256(
    "ciris.sth_cosign.v1\n" ||
    "tree_size=" || decimal_no_leading_zeros || "\n" ||
    "root_hash_sha256=" || sha256_hex_lowercase || "\n" ||  // per §0.6
    "signed_at=" || rfc3339_canonical                       // per §0.5
)
```

Ed25519 signs `canonical`; ML-DSA-65 signs `canonical || ed25519_sig` (bound payload). The directory
itself is served by **`GET /v1/transparency/witnesses`** (paginated), and an STH's cosignatures with
their `witness_quorum_met` verdict by **`GET /v1/transparency/sth/{tree_size}/witnesses`**.

**`POST /v1/transparency/witnesses` (admin; multi-party-gated).** Registering a witness is
power-conferring, so it is gated hardest. In the 0.1 interim it is bearer-token-gated by
`REGISTRY_ADMIN_TOKEN`; **0.2 hardens this to 2-of-3 steward sign-off** — the request body MUST carry
signatures from at least two of the three regional stewards, verified against `GET /v1/steward-key`
(5.3.4). Single-token admission is a known 0.1 weakness; production deployments SHOULD operate the
endpoint behind a corporate IDP gate enforcing multi-party admission out-of-band until the multi-sig
requirement is normative. This is the [Recursive Golden Rule](part_1_foundation.md#1132-structure-recursive--the-recursive-golden-rule-structural)
applied to the directory's own roots: the parties who watch the log are admitted under the same
multi-party discipline the federation imposes everywhere else.

#### 5.3.1.1 `consistency-proof` — Consistency-proof requirement (normative; addresses CEG 0.1 distsys review)
<sub>budget 0.26pp · import #112 · from **§10.3.1** (ceg)</sub>

A quorum on a *string* is worthless if the strings describe inconsistent logs. So a witness signing an
STH MUST first verify a **consistency proof** from the prior STH it cosigned (or from genesis, if it
is the witness's first cosignature against this log). The Registry MUST reject cosign requests that
omit the `consistency_proof_*` fields, or whose proof does not verify against the named prior STH.
`witness_quorum_met` is therefore "quorum on **log consistency**," not "quorum on a string."

**Enforced as of Registry v2.3.0.** The request carries `consistency_proof_path_b64[]` (base64
RFC 6962 §2.1.2 node hashes). The Registry anchors the check against the prior `(tree_size, root_hash)`
**it recorded** for that witness — not a root the requester claims — and rejects a missing proof when
a prior STH exists (or a tree_size behind the witness's prior cosigned STH) as `MALFORMED_REQUEST`, or
a proof that fails to reconstruct both roots as `CONSISTENCY_PROOF_INVALID`. A witness's first
cosignature is exempt ("from genesis"). The RFC 6962 verifier is vendored from
`ciris-verify-core::transparency` and proven against independent known-answer vectors.

### 5.3.2 `transport` — Transport substrate for byte-level content
<sub>budget 0.5pp · import #58 · from **§10.1** (ceg)</sub>

Here is the bytes road itself. Wire-format attestations carry claims; they don't carry bytes. When a
claim's `evidence_refs[]` cites a SHA-256-addressed blob — an installer binary, a config file, an
adapter package — the bytes travel via the Edge transport substrate: `MessageType::ContentFetch` +
`ContentBody` + `ContentMiss`. Holder-discovery runs through Persist's `holds_bytes:sha256:*`
directory; peer-resolution through Edge's `PeerResolver::resolve_holders`; node-mode peers serve the
bytes per their cohabitation contract. **The attestation envelope shape is unchanged** — a SHA-256 in
`evidence_refs[]` simply becomes universally resolvable to bytes through the substrate.

#### 5.3.2.1 `holder` — Holder directory TTL + ContentMiss feedback
<sub>budget 0.47pp · import #63 · from **§10.1.2** (ceg)</sub>

A `holds_bytes:sha256:{prefix}` attestation has a default validity of **24 hours** from `signed_at`;
after that the holder is stale. Consumer policy MUST attempt at most 2 holders in parallel and accept
the first successful full-SHA verification. On `ContentMiss` (holder no longer has the blob), the
consumer MUST emit a `withdraws` against the holder's attestation with
`withdrawal_reason: "content_miss"`. Holders consistently failing ContentMiss are downweighted in
`PeerResolver::resolve_holders` — the directory self-heals by negative feedback rather than central
pruning.

#### 5.3.2.2 `consent-revocations` — Consent revocations are NOT local-tier-eligible (CEG 0.6 addition)
<sub>budget 0.38pp · import #80 · from **§10.1.3** (ceg)</sub>

The local-tier write path (5.3.2.4) lets a producer defer the expensive hybrid signature until
federation-promotion. That is sound for self-attestations — but it has one carve-out that
[autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) demands, and the discriminator is
subtle. **The discriminator is *who holds revocation authority*, NOT whether `subject_key_ids` is
empty.** A producer-authority self-attestation MAY name a subject (e.g. `observed:user:{hash}:*`, a
self-`consent:partnered:{user}`) and still ride local-tier, because the producer holds the authority
and no *other* subject can revoke it.

The single exception: **a subject's consent revocation.** When a Contribution carries non-empty
`subject_key_ids`, any subsequent `consent:state:revoked` or `withdraws` from a subject in that set
MUST promote to federation-tier within a bounded window — default **24 hours**, operator-tunable.
**Rationale:** subject-side revocation is the wire primitive federation peers depend on to honor
consent. If a user revokes consent in one agent's local-tier scope and that revocation stays
unsigned/unpromoted, other peers keep propagating the user's data — exactly the failure CEG 0.6
exists to close. The substrate MUST emit `hard_case:consent_revocation_promotion_overdue` when a
subject-side revocation exceeds the window without promotion. This preserves the cardinality wins of
the local-tier pattern while closing the leak window.

#### 5.3.2.3 `merge` — Cross-region merge intents — CEG-declared per subject_kind (normative; CEG 1.0-RC2 addition)
<sub>budget 0.23pp · import #123 · from **§10.1.6** (ceg)</sub>

With operational data joining the CEG-native replication stream, the substrate runs **more than one
merge policy**. The policy is a normative property of the `subject_kind`, **declared here** — the
substrate reads and dispatches on the declaration; it MUST NOT infer policy per record type. The
substrate enforces declared merges; it does not invent policy.

| subject_kind(s) | Merge intent | Semantics |
|---|---|---|
| `organization`, `org_membership` | **`lww_skew_bounded` + `withdrawal_forward_only`** | Stable-id grouping; an admitted `withdraws` is forward-only (a later non-withdrawn write does NOT resurrect); else latest `asserted_at` wins, tie-break smallest `attestation_id`. The forward-only here is the lightweight authz flag, NOT the DEK-cascade crypto path. |
| `partner_record` | **`monotonic_quorum`** (the V058 R1/Q1 machinery generalized) | Anti-rollback first: a write whose `revision` decreases never enters the merge. Then the `MergeBallot` comparator: `quorum_weight` → signed timestamp → content hash. Quorum-above-time neutralizes timestamp front-running. More-restrictive wins: `revoked` > `suspended` > `active`. |
| `revocation` + the three membership-revocations | V058 R1/Q1 (unchanged) | As shipped. |
| keys / attestations / occurrences / families / communities / location_proofs | Content-addressed idempotent admission (unchanged) | Same content → same `envelope_hash` → dedup; rotation collisions rejected non-destructively. |

**Skew-bounded admission (normative).** For every `lww_skew_bounded` subject_kind, the substrate MUST
reject any envelope with `asserted_at > now + tolerance`, where `tolerance` is the ±5-minute clock-skew
bound. Without it, a signer with a forward-skewed clock future-dates `asserted_at` and wins LWW
indefinitely; with it, a forward-dated write wins for at most the skew window. This matters precisely
because `org_membership` carries `role: OrgAdmin` — unbounded LWW on authz data is a role-escalation
surface. **The two quorums are different layers**: the steward-signature *admission* quorum (Verify's
layer) is NOT the region *merge* quorum (the substrate's layer); neither counts the other's signatures.

#### 5.3.2.4 `attestation-tier` — The attestation tier model — local-tier write, query, promotion (normative)
<sub>budget 0.14pp · import #194 · from **§10.1.5** (ceg)</sub>

The tier model pins the **shared attestation surface** the four CEG implementations (CIRISAgent,
CIRISNodeCore, CIRISLensCore, CIRISRegistry) write, query, and promote through, so it is one
analyzable contract. The substrate exposes the *methods*; CEG pins the *wire/conformance semantics*
every implementation MUST agree on. **No 1+4 change** — a "tier" is recorded state on an attestation,
not a new primitive.

##### 5.3.2.4.1 `authority-local` — Local-tier eligibility — the discriminator is *revocation authority*, not subject-set emptiness
<sub>budget 0.18pp · import #158 · from **§10.1.5.2** (ceg)</sub>

A write is local-tier-eligible **iff the producer holds sole revocation authority** over it. The
discriminator (revocation authority, NOT empty `subject_key_ids`), the producer-authority-with-named-
subject examples, and the single carve-out (a subject other than the producer holds revocation
authority — a `withdraws` under §3.2.3 rule 2/3 or a subject-emitted `consent:state:revoked`, which
MUST go signed / promote per the 24-hour obligation, 5.3.2.2) are defined canonically there.
Tier-specific addition: `witness_relation` MUST be `self` for any local-tier write.

##### 5.3.2.4.2 `local` — Promotion — `local → federation` (the deferred-signature moment)
<sub>budget 0.15pp · import #180 · from **§10.1.5.3** (ceg)</sub>

Promotion computes the hybrid signature and flips the row federation-visible. It is **idempotent**
(promoting a `federation` row returns it unchanged), and at the promotion instant the tiered-scope
promotion and `holds_bytes` emission fire exactly as for any federation write — **promotion *is* the
federation-emit moment.** The signature MUST cover `JCS(contribution_envelope)`, the identical
canonical bytes any natively-federation attestation signs, so a promoted row is
**byte-indistinguishable on the wire** from one born federation-tier — there is no "was-promoted"
marker in the signed bytes. **Substrate columns (`tier`, `promoted_at`) are NOT in the canonical
bytes.** Promotion MUST canonicalize the **exact member set the producer committed at local-write
time** — a field omitted at local write MUST NOT be materialized at promote, or the recomputed bytes
diverge and the hybrid sig fails. The substrate serializes the stored row → committed envelope → JCS →
sign; it MUST NOT re-default.

##### 5.3.2.4.3 `two` — The two tiers
<sub>budget 0.15pp · import #181 · from **§10.1.5.1** (ceg)</sub>

| Tier | Signature | Federation-visible | Read-visibility | Written by |
|---|---|---|---|---|
| `local` | MAY be absent (deferred per 5.3.2.2) | **No** | **only the producing occurrence** (self-read); every other caller — even an authorized family/community peer — sees nothing | local-tier write |
| `federation` | hybrid Ed25519 + ML-DSA-65 **present** | Yes | per `cohort_scope` + the 5.2 invisibility rule | direct signed write OR `local → federation` promotion |

**Invariant (substrate MUST enforce):** `tier = federation ⟹ hybrid signature present`. Nothing crosses
to federation-visible unsigned. A `local` row is labelled local and MUST NOT be served as
federation-authoritative ([fail-honest](../../MISSION.md)). The read-gate is **orthogonal** to
`cohort_scope` — an additional filter (`local ⟹ caller is the producing occurrence`) composing with
the 5.2 target-membership predicate. Threat entries: AV-59 (local row leaked to a non-self caller),
AV-60 (unsigned local served as authoritative), AV-61 (the two gates de-synced).

###### 5.3.2.4.3.1 `admission-pqc` — The PQC half is MANDATORY at admission — no classical-only, no hybrid-pending accommodation (normative, CEG 1.0)
<sub>budget 0.15pp · import #179 · from **§10.1.5.1.1** (ceg)</sub>

The invariant `tier = federation ⟹ hybrid signature present` is **enforced at the admission gate as of
CEG 1.0**, and "present" means **verified**: every federation-tier admission gate MUST check **both**
halves — Ed25519 over `JCS(envelope)` **and** ML-DSA-65 over `JCS(envelope) ‖ ed25519_sig` — and MUST
**reject** a federation-tier Contribution carrying only the classical half. This binds **all
operational-authority admission gates** (key_grant / partner / org-membership / license writes, the
`transport_destination` binding, the `partner_record` / founder-quorum gates).

**This is an immediate 1.0 requirement — not a phased cutover.** Pre-1.0 the PQC half was honored
*opportunistically* (a rollout accommodation so hybrid-pending members could federate while ML-DSA-65
wiring landed); **CEG 1.0 closes that window with no fleet-floor and no calendar trigger.** The
rationale is exactly the threat hybrid exists to defend: an adversary holding a future Ed25519 break
could forge a grant / binding / partner-record and have it admitted if the classical half alone
sufficed. There is **no `require_hybrid: false` posture at 1.0** — a verifier's hybrid-required check
is always-on, never an operator toggle. A key that has not completed PQC wiring is not 1.0-conformant
for federation-tier emission and is confined to local-tier until it does.

**The mandate binds the durable store + replication path, not only the authority gates.**
"Federation-tier" means **every** federation-tier attestation, including the bulk per-trace /
store-and-replicate path. **Content-addressing is NOT a defense against forge-later:** a CRQC-era
adversary who breaks Ed25519 mints a backdated trace under a historical key and hashes *their own*
forgery — the content hash matches by construction, so the address proves nothing about authenticity.
Because the trace store is kept for posterity (it outlives the classical primitive), the per-trace
signature is the single most forge-exposed surface in the federation — and the PQC half is mandatory
there for the same harvest-now / exploit-later reason as the at-rest DEK cascade. A substrate
persisting a federation-tier row whose signature lacks a valid ML-DSA-65 half MUST reject it at the
ingest gate — store-then-quarantine is non-conformant.

##### 5.3.2.4.4 `persist-query` — Query — open-prefix dimensions, bounded operators (normative; resolves CIRISPersist#172 OQ-2)
<sub>budget 0.11pp · import #239 · from **§10.1.5.4** (ceg)</sub>

The uniform read filters on `(dimensions[], valid_at, confidence_floor, subject_key_id?, scope)`.
**`dimensions[]` is an OPEN-vocabulary set of prefix strings**, matched by hierarchical prefix — NOT a
closed enum (a closed enum would force a substrate redeploy per CEG namespace addition and break
forward-compat). A query for `detection:*` matches `detection:correlated_action:bribery`; an exact
string matches exactly. **The bounded surface is the *operator set*, not the *vocabulary*** — the
apophatic discipline (a fixed, named surface, not an OLAP/graph engine) is preserved by the closed set
of five predicates plus no caller-composed SQL. *Open data, closed operators.* `scope` applies BOTH
the 5.2 target-membership gate AND the 5.3.2.4.3 tier gate.

##### 5.3.2.4.5 `holistic` — For holistic analysis + modeling (informative)
<sub>budget 0.11pp · import #240 · from **§10.1.5.5** (ceg)</sub>

The tier model is the *same KEM-then-symmetric placement* the streaming model uses, applied to
attestations: the **expensive op (hybrid sign) is on the cold path** (promotion, at federation-emit),
while the **hot path (local self-attestation write) is O(1) and unsigned.** Cost shape: local write =
O(1), no asymmetric crypto; promotion = one hybrid sign (~ML-DSA-65 sign ~330 µs) + JCS
canonicalization, only at the federation-emit moment; query = the scope-filtered read, independent of
tier. A federation's attestation load models as *(local-write rate, promotion rate, query rate)*, with
promotion carrying the only asymmetric-crypto cost — the dual of the streaming model's epoch-rekey
tail.

#### 5.3.2.5 `full-sha` — Full-SHA verification before consumption (normative)
<sub>budget 0.11pp · import #224 · from **§10.1.1** (ceg)</sub>

A CEG-Conforming Consumer MUST verify the **full** SHA-256 of received bytes against the value in
`evidence_refs[]` BEFORE handing the bytes to any consumer (Agent loader, Portal renderer, etc.). The
`holds_bytes:sha256:{prefix}` directory carries only a short prefix for index efficiency; the consumer
MUST NOT short-circuit verification to the prefix. Bytes failing the full-SHA check MUST be discarded
and the holder reported via the `holds_bytes` chain (a `withdraws` or negative score per consumer
policy). This is the floor under all of 5.3.2: content-addressing is only a trust primitive if the
*full* address is checked.

### 5.3.3 `transport-streaming` — Streaming transport, per-stream logs & delivery receipts (CEG 0.10 addition)
<sub>budget 0.45pp · import #67 · from **§10.5** (ceg)</sub>

Streaming is the **delivery axis** — the third orthogonal envelope concern alongside visibility
(`cohort_scope`) and revocability (`subject_key_ids`). The 1+4 primitive set is untouched; this is an
endpoint + envelope + composition extension, NOT a grammar change. The surface **bifurcates** by
cardinality: **observer-share / directed delivery** (single Contribution → subscriber-set; N=1
typical; per-subscriber `key_grant`) is impl-live; **media / streaming multicast** (a `live_stream`
chunk-DAG with per-`(stream_id, epoch)` keys; N>1) is spec-now, impl substrate-pending CIRISPersist#142.
The subsections below carry that dependency.

#### 5.3.3.1 `stream` — Chunk seal + STREAM nonce (normative — V2 lock)
<sub>budget 0.5pp · import #56 · from **§10.5.2** (ceg)</sub>

Per-chunk content sealing **conforms to SFrame** ([draft-ietf-sframe](https://datatracker.ietf.org/wg/sframe/about/),
normative): per-frame AEAD with a `(KID, CTR)` header, bound here as `KID = stream_id` and
`CTR = counter`, AEAD = AES-256-GCM, keys derived per MLS epoch (5.1). The 12-byte nonce follows the
**STREAM** layout (Hoang–Reyhanitabar–Rogaway–Vizár, CRYPTO 2015):

```
nonce[12] = prefix[7] ‖ counter_be[4] ‖ last_flag[1]
```

- **`prefix[7]`** — derived, NOT transmitted:
  `prefix = HKDF-SHA256(epoch_dek; info)[0..7]`, where `info = b"ciris-stream-nonce/v1" ‖ stream_id_utf8
  ‖ epoch_be8` with **`epoch_be8` = the u64 epoch as 8-byte big-endian**. This encoding is normative
  and MUST be byte-identical across producer, substrate, and every consumer — consumers recompute this
  exact nonce to *open* chunks, so any BE/LE/ASCII disagreement on `epoch` yields a different prefix →
  different nonce → GCM auth-tag failure (silent whole-stream decryption failure). No length-prefix on
  `stream_id` is needed: the fixed-length tag prefix + fixed 8-byte `epoch` suffix make the parse
  unambiguous.
- **`counter_be[4]`** — 32-bit big-endian; hard ceiling 2³²−1 chunks per epoch. The substrate MUST
  force an epoch roll before wrap. Recommended cap: `MAX_CHUNKS_PER_EPOCH = 2²⁴` (~16.7M, operator-tunable).
- **`last_flag[1]`** — `0x01` on the final chunk of an epoch, `0x00` otherwise. The distinct nonce on
  the final chunk gives **truncation + append resistance**: an adversary cannot drop the final chunk
  and pass off a short stream, nor append past a sealed segment.

**Cross-epoch counter reset is nonce-safe.** GCM's catastrophic case is reuse of a `(key, nonce)`
pair; on epoch roll the DEK changes, so a reset counter lives in a different keyspace — `(DEK_e,
nonce=0)` and `(DEK_{e+1}, nonce=0)` are distinct pairs. The enforced invariant is therefore only
*within* an epoch: counter strictly monotonic, never wraps. **Single-sender-per-`(stream_id, epoch)`
invariant (normative):** the counter space of a `(stream_id, epoch)` is owned by exactly one sender —
the producer. Two distinct senders MUST NOT seal under the same `(stream_id, epoch)` DEK, or their
independently-incremented counters could collide into a reused pair. In group video (5.3.3.2) each
participant emits their *own* `live_stream` with their *own* `stream_id`, so the nonce prefix is
per-sender-unique by construction.

#### 5.3.3.2 `composition` — Realtime group communication — composition (CEG 0.13 addition)
<sub>budget 0.46pp · import #66 · from **§10.5.8** (ceg)</sub>

The delivery axis was framed for 1:1 observer-share and 1:N broadcast. Realtime **group
communication** — group video, voice, screen sharing, text chat, channels-with-sub-channels — is the
**same primitive set at N↔N cardinality**. It composes entirely from `community` + `live_stream` +
`chat_message` + member/transport resolution. **Zero new structural primitives** — this is the
thirteenth path on the [1+4 adequacy claim](part_1_foundation.md#17-minimal-and-adequate--the-14-claim)
and the most product-complete: the whole realtime-collaboration surface is composition.

| Surface | Composes from |
|---|---|
| **1:1 / group video call** | each participant emits a `live_stream` (their A/V) scoped to the channel `community`; each subscribes to peers — N↔N = N simultaneous bidirectional streams over a small roster |
| **Voice channel** | identical with an audio-only codec; same `live_stream` wire |
| **Desktop / screen sharing** | a `live_stream` whose source is a screen capture (1→N within the channel); same chunk-seal + epoch-DEK wire |
| **Text chat** | `chat_message` at `cohort_scope: community`; threads via `topical_relation: replies_to` |
| **Channel** | a persistent `community` — its roster gates who can join the call / read the chat |
| **Sub-channels** | nested `community` membership: a parent "space" whose members admit child channel-communities |
| **Presence ("who's here")** | the D6 reachable set (5.3.3.4) — node-local, never an attestation, never logged |
| **Invite / join / leave** | community admission ceremony — invite = proposal; join = admitted member; leave = forward-only `withdraws` |

**Transport profiles (normative — extends 5.3.3.5).** Broadcast (1:N, large N) is pull-only
`ContentFetch`, relay tree → 1.x. **Realtime small/medium group** (calls, huddles, ≤ ~50) uses **direct
Reticulum Links** between participants — low-latency push, no relay tree needed at small N — and is
**in 1.0 scope**, because small rosters need no relay tree and RNS Links give encrypted low-latency
point-to-point natively. Realtime large group (≫50 in one A/V room) uses a selective-forwarding relay
(SFU) → 1.x. The wire is identical to broadcast; only the transport profile + roster size differ. **PQC
is unchanged and mandatory.** A persistent channel is a long-lived `community`; an ad-hoc call is an
**ephemeral `community`** (short `valid_until`, torn down on last-leave) — same primitive, different
lifetime, no special-case "session" primitive.

##### 5.3.3.2.1 `namespace-realtime` — `codec_id` namespace — realtime A/V chunk codec discriminator (normative, 1.0-RC9 — ratifies [CIRISRegistry#84](https://github.com/CIRISAI/CIRISRegistry/issues/84))
<sub>budget 0.23pp · import #125 · from **§10.5.8.2** (ceg)</sub>

A relay fanning out chunks at scale must drop chunks for per-receiver bandwidth degradation **without
decrypting them** — so the codec's layer-numbering must be a **clear (non-AEAD) discriminator.** CEG
ratifies a 1-byte `codec_id` namespace so every implementation reads the same meaning. **This is a
namespace ratification on the transport-layer chunk header, not a change to the attestation
envelope** — the frozen 1+4 surface and its canonicalization are untouched.

**Wire position (normative).** `codec_id` (1 byte) + `ChunkLayer { spatial, temporal, quality: u8 }`
(3 bytes) = a 4-byte additive block at `SealedAvChunk` header offset **48..52**. **Clear metadata, NOT
inside the AEAD** — a relay drops by `codec_id`/`ChunkLayer` without touching the inner epoch-DEK seal;
tampering causes mis-decode or drop, never a crypto break. **Additive + backward-compatible**: a
v3.7.0 chunk round-trips identically as `codec_id = 0xFF` + `ChunkLayer { 0, 0, 0 }`.

| Hex | Codec | Semantics |
|---|---|---|
| `0x01` | **AV1 SVC** | 3 spatial × 4 temporal × N SNR layers; base layer required. Production default (WebRTC-native, royalty-free). |
| `0x02` | JPEG XS (layered) | Low-latency intra-only; broadcast. **Reserved.** |
| `0x03` | **Symmetric MDC** | any subset of chunks decodes at proportional fidelity, no base-layer floor. **The substrate design target. Reserved.** |
| `0xFF` | Opaque | No scalable-coding semantics; v3.7.0 wire-compat. `ChunkLayer` MUST be `{0,0,0}`. |
| `0x04`–`0x7F` | — | **Reserved** for future standardized codecs (CEG-assigned). |
| `0x80`–`0xFE` | — | **Experimental / per-deployment** — no cross-federation meaning. |

The **MDC-primacy design intent** (informative): the user-facing contract is *"any node can request a
lower-bandwidth stream from peers — as simple as taking every other chunk, down to a blinking dot."*
MDC matches that *symmetrically* (drop any subset → decode the rest, no coordination, no base-layer
floor); SVC is production-deployable today but has a floor and needs coordination for an "every other
chunk" drop. The substrate is **MDC-shaped** even while production ships SVC.

##### 5.3.3.2.2 `realtime` — Realtime non-A/V data streams (normative scope boundary)
<sub>budget 0.16pp · import #175 · from **§10.5.8.1** (ceg)</sub>

The realtime profile is **not media-only.** Any high-frequency mutable shared state — multiplayer game
ticks, collaborative-editing ops (CRDT/OT), live cursors/whiteboard strokes, remote-control input,
high-rate telemetry — rides the **same** realtime transport with an application-defined payload codec
in place of an A/V codec. The wire is identical (per-`(stream_id, epoch)` epoch DEK, STREAM-nonce chunk
seal, per-stream STH). A data-stream chunk is just a chunk. **Scope boundary:** ordered, sealed,
authenticated, PQC-encrypted realtime *delivery* is IN scope; the conflict-free / convergent-*merge*
logic for shared mutable state (CRDT, OT, LWW) is **application-layer**, NOT a CEG primitive —
consistent with the discipline that the wire transports and the consumer composes. CEG carries the
ops; the application converges them.

##### 5.3.3.2.3 `registry-wire` — `SealedAvChunk` wire layout (normative, 1.0-RC10 — absorbs CIRISEdge v4.0.0 per [CIRISRegistry#85](https://github.com/CIRISAI/CIRISRegistry/issues/85) §N)
<sub>budget 0.14pp · import #195 · from **§10.5.8.3** (ceg)</sub>

The realtime A/V chunk that lands on each RNS Link payload (and the broadcast pull path). Byte layout
(normative — transcribed from the edge v4.0.0 reference `SealedAvChunk::to_bytes`):

```
offset  field                       encoding
0..32   stream_id                   32 bytes (caller-derived: sha256(stream_meta))
32..40  epoch                       u64 big-endian
40..48  chunk_seq                   u64 big-endian
48..49  codec_id                    u8  (5.3.3.2.1 namespace)
49..50  layer.spatial               u8
50..51  layer.temporal              u8
51..52  layer.quality               u8
52..    double_sealed_ciphertext    remaining bytes
```

`CHUNK_HEADER_LEN = 48` (stable since v3.7.0); `CHUNK_CODEC_LAYER_LEN = 4`. **Backward compatibility
(length-disambiguated):** a wire carrying only the 48-byte header MUST be read as `codec_id = 0xFF`
(opaque) + `layer = {0,0,0}`, bytes `48..` as ciphertext (the v3.7.0 shape); ≥ `48+4` bytes after the
header is read as v3.8.0+ (codec+layer present). New writes always include the 4-byte block.
`codec_id` + `layer` are **clear metadata, NOT inputs to the AEAD.**

##### 5.3.3.2.4 `chunklayer` — `ChunkLayer` + `ReceiverLayerPolicy` — SVC layer model (normative, 1.0-RC10 — §85 §N.2)
<sub>budget 0.14pp · import #197 · from **§10.5.8.4** (ceg)</sub>

`ChunkLayer` is the 3-byte SVC layer descriptor (`spatial`, `temporal`, `quality`, each `u8`). Each
axis is **monotonic**: layer `0` is the base (lowest fidelity, always required); each increment is an
additive enhancement. A receiver reconstructs from the prefix `0..=max_*` of cells. The base cell
`{0,0,0}` is the **"blinking dot"** — the minimum a participant can subscribe to. `ReceiverLayerPolicy
{ max_spatial, max_temporal, max_quality }` is the per-receiver drop policy, **advertised over the
existing `federation_session` / `key_grant` entitlement surface — NOT a new wire**; the sender drops
chunks above the cap without re-encoding. `admits(layer)` is the per-axis test; a chunk tagged
`codec_id = 0xFF` MUST be admitted **unconditionally** (the fan-out filter short-circuits before
consulting `admits`). Canonical policies: `BLINKING_DOT = {0,0,0}`, `UNCAPPED = {255,255,255}`. This
composes with the inner-once / outer-N fan-out optimization: the inner seal runs once per chunk; the
outer seal runs only for the `(receiver, chunk)` pairs the policy admits.

##### 5.3.3.2.5 `stream-double` — Double-seal + deterministic nonce derivation (normative, 1.0-RC10 — §85 §N)
<sub>budget 0.11pp · import #243 · from **§10.5.8.5** (ceg)</sub>

`double_sealed_ciphertext` is **outer-AEAD( inner-AEAD( chunk_plaintext ) )** — two independent
AES-256-GCM layers. The **inner** seal is end-to-end (the epoch-DEK content seal); the **outer** seal is
the per-RNS-Link transit wrap (a relay sees the outer layer only, never plaintext — the 5.3.3.5 E1
two-layer posture). Both nonces are **deterministic** (none transmitted — every holder recomputes;
collision-safety rides the single-sender-per-`(stream_id, epoch)` invariant):

```
inner_nonce = SHA-256( b"CIRIS-AV-INNER-V1" ‖ stream_id[32] ‖ epoch_be8 ‖ chunk_seq_be8 )[0..12]
outer_nonce = SHA-256( b"CIRIS-AV-OUTER-V1" ‖ link_id ‖ link_seq_be8 )[0..12]
```

The label bytes are domain separators pinned by this section — they bind the nonce to its layer and
prevent cross-layer reuse. `epoch`, `chunk_seq`, `link_seq` are `u64` **big-endian**; `link_seq` is
monotonic per RNS Link (transit replay guard). Conformance is proven by the §85 vector set.

#### 5.3.3.3 `per-stream` — Per-stream log + stream-root (normative — V1 lock)
<sub>budget 0.36pp · import #82 · from **§10.5.1** (ceg)</sub>

A stream is **its own per-stream transparency-log instance** (`log_id = stream_id`). A `live_stream`
MUST NOT append chunks into the federation provenance log (the global log carrying builds / licenses /
identities) — millions of media chunks would pollute provenance and inflate the global tree. The path
**reuses** the 5.3.1 `SignedTreeHead` / consistency-proof / cosign abstractions, instantiated per-stream
as separate log instances under the same RFC 6962 algorithm. For each live_stream: chunks = leaves;
stream-root = `SignedTreeHead{ log_id: stream_id, tree_size: chunk_count, root_hash, timestamp,
signature }`. **The producer signs the STH — MANDATORY** authenticity root (hybrid Ed25519 +
ML-DSA-65). **Witness cosign — OPTIONAL**, via 5.3.1 verbatim — the best-effort / accountable split:
open media → producer-signed root only; **accountable** (paid media, registry propagation, emergency) →
witness cosign per the 5.3.1.1 consistency proof, the **anti-equivocation guarantee** (the producer
cannot show different chunk-K to different subscribers nor rewrite mid-stream). **Cadence:** a signed
root every `K` chunks OR `T` seconds, always at an epoch boundary + at `sealed_at` (default K=64, T=2s,
operator-tunable). Witness cosign runs per-epoch to keep cosign-quorum cost off the hot path.

#### 5.3.3.4 `liveness` — D6 liveness invariant — entitled vs reachable (normative)
<sub>budget 0.35pp · import #84 · from **§10.5.6** (ceg)</sub>

Two sets are **NEVER conflated**, and the distinction is itself a privacy property:

- **Entitlement roster** (Persist-owned): a signed CEG envelope, Edge-propagated, durable, logged. It
  is *evidence* — it MUST propagate and be auditable.
- **Live-reachability set** (Edge-owned): a node-local presence tracker (generalizing the 24 h
  `holds_bytes` TTL down to seconds-to-minutes for live-multicast). It is **NEVER an attestation,
  never `holds_bytes`, never replicated, never logged.**

**Fan-out invariant: `fan_out(C) = entitled(C) ∩ reachable(now)`.** The **heartbeat-suppression
discipline** is a producer-side-refusal invariant of the **same class as the 5.2 `cohort_scope:
self|family` suppression**: missed (entitled-but-unreachable) members fall back to pull on reconnect —
the substrate does NOT keep retrying push, does NOT emit a "delivery_failed" attestation, does NOT log
liveness state. The reconnect-then-pull catch-up rides 5.1 `history_on_join`. Reachability is a fact
about *now*, deliberately left off the wire — the same instinct that keeps family content invisible
keeps presence unlogged.

#### 5.3.3.5 `transport-edge` — Transport — Edge layer (normative — E1–E4 lock per [§15.6.2](15_gaps.md))
<sub>budget 0.29pp · import #101 · from **§10.5.5** (ceg)</sub>

| Decision | Behavior |
|---|---|
| **E2 — pull-only RC1** | RC1 multicast = pull-only: producer seals chunks under the epoch DEK → emits `holds_bytes:sha256:*` → subscribers pull via `ContentFetch`. Relay / fan-out tree → 1.x. |
| **E1 — two-layer crypto (security-critical)** | The transit-key is a **hop-by-hop transport wrap UNDER the E2E epoch DEK** (two independent crypto layers). It MUST NOT replace the cascade — a relay never sees plaintext. **PQC in transit:** BOTH layers MUST use PQC-grade key agreement — the transit-key wrap MUST be hybrid X25519+ML-KEM-768, and the underlying Edge/Reticulum transport MUST negotiate the hybrid `CIR2` crypto-kind, never classical-only `CIR1`. Classical-only transport is rejected for streaming. |
| **E3 — fan-out = entitled ∧ reachable** | **Persist owns durable entitlement** (the signed, replicated, logged roster); **Edge owns transport-reachability** via a node-local presence tracker. Fan-out targets the intersection. Reachability is never an attestation, never replicated — consistent with the 5.2 structural-invisibility shape. |
| **E4 — durable side rides existing federation-attestation path** | Durable entitlement (roster + epoch-key grants) rides the existing federation-attestation Edge path — just more `federation_attestations` rows. Net-new is only on the 5.3.3.3 streaming-log endpoints. |

#### 5.3.3.6 `delivery` — Delivery receipts (normative — D5 / V3 lock)
<sub>budget 0.27pp · import #105 · from **§10.5.4** (ceg)</sub>

A `delivery_receipt:{stream_id}` Contribution is a subscriber's signed acknowledgement that they
received chunk K under the named stream + epoch. **Best-effort default**; opt-in for **accountable**
profiles. The signed bytes (domain-separated + length-prefixed):

```
receipt_signing_bytes =
    "ciris-delivery-receipt/v1"          // domain separator
  ‖ len(subscriber_key) ‖ subscriber_key
  ‖ len(stream_id)      ‖ stream_id
  ‖ epoch        (u64 LE)
  ‖ chunk_root   ([u8; 32])
  ‖ K            (u64 LE)
```

**The Verify check is a JOIN, NOT a sig-check.** (1) Signature valid over the canonical bytes —
necessary but not sufficient. (2) **`chunk_root` is a real published STH root** — it MUST equal a
`SignedTreeHead.root_hash` actually published for `log_id = stream_id` at `tree_size ≥ K`; a phantom
root → REJECT; for accountable streams "published" means witness-cosigned, so the subscriber cannot
collude with the producer on a private root. (3) *(Recommended for accountable)* an inclusion proof
chunk K → `chunk_root`. **Semantics — proof-of-DELIVERY, not proof-of-CONSUMPTION**: the receipt proves
the subscriber received bytes committing to chunk K; it does NOT prove they decrypted them (they may
not hold the epoch DEK). Consumers MUST NOT overclaim it as proof of consumption — Verify validates
the receipt; the "delivered" verdict is consumer policy, the
[fail-honest](../../MISSION.md) validation-not-adjudication line.

#### 5.3.3.7 `normative` — Framing (normative)
<sub>budget 0.11pp · import #241 · from **§10.5.0** (ceg)</sub>

The 1+4 wire-format lockdown holds across the entire streaming surface: there are **no new
`attestation_type` values.** Stream chunks ride content addressing; stream-roots ride the existing
`SignedTreeHead` shape; delivery receipts ride `scores` against the `delivery_receipt:{stream_id}`
reserved prefix. The streaming machinery is a delivery axis *on top of* the grammar, never a change
*to* it.

#### 5.3.3.8 `documents-what` — What CEG 0.10 documents
<sub>budget 0.11pp · import #242 · from **§10.5.7** (ceg)</sub>

Scope pointer. The streaming surface (5.3.3.3–5.3.3.4) does **not** change the 1+4 primitive set
(delivery rides existing primitives), does **not** bundle the streaming-half substrate impl
(Persist#142 + the RC1-1c CHECK-arm migration), keeps push-mode multicast relay/fan-out **pull-only at
RC1** (push tree → 1.x), and leaves the K / T / MAX_CHUNKS_PER_EPOCH constants + accountable-stream
quorum operator-tunable (ratification pending).

### 5.3.4 `multi-steward` — Multi-steward + accord-holder discovery
<sub>budget 0.32pp · import #94 · from **§10.2** (ceg)</sub>

The federation's trust roots are themselves discoverable — and, crucially, **self-describing about
their own readiness.** `GET /v1/steward-key` returns the multi-steward set with its M-of-N policy:
each steward carries `region`, `key_id`, hybrid Ed25519 + ML-DSA-65 pubkeys, `hardware_class`, a
`deployed` flag, a SHA-256 fingerprint, and a self-attested cert validity. The response itself is
hybrid-signed by the serving region's steward over
`canonical = "ciris.steward_key_response.v1\n" || sha256_hex_lowercase(canonicalized_json_body_excluding_signature)`.
**Consumers MUST verify the response signature before trusting any field** — and placeholder pubkeys
without `deployed: true` MUST NOT be promoted to trust roots. This is
[fail-secure](part_1_foundation.md#15-fail-secure--fail-secure) applied to the roots themselves: an
undeployed steward is *visible* but *not trusted*, never silently promoted.

`GET /v1/accord-holders` returns the three named [HUMANITY_ACCORD](part_4_composition_governance.md)
holders with hybrid pubkeys, per-holder `hardware_class`, and a `provisioned` flag; the v1.4 interim
ships placeholder fingerprints with `provisioned: false`, and consumers MUST NOT honor CONSTITUTIONAL
invocations against placeholders. `GET /v1/accord/holders` is a UI wrapper with per-holder
`accord_emissions[]`. `GET /v1/rotation-history` returns chronological rotation events. All carry the
same response-signing requirement.

### 5.3.5 `registry-other` — Other Registry endpoints
<sub>budget 0.13pp · import #207 · from **§10.4** (ceg)</sub>

The remaining Registry endpoints compose existing data into consumer-facing shapes.
`GET /v1/builds/{version}` returns the BuildRecordResponse with a `federation_provenance` block (SLSA
emission discipline). `GET /v1/verify/build-manifest/{project}/{version}/{target}` (Path B) returns the
verbatim signed BuildManifest. `GET /v1/agent_files/{kind}?platform_or_target=...` returns the
trust-composition layers. `GET /v1/partner/{key_id}` composes ProfileScorecard data from existing
tables. Full response schemas land in the Rust handlers + OpenAPI export; CEG commits to publishing a
versioned OpenAPI spec alongside this document.

### 5.3.6 `common` — Common response shape
<sub>budget 0.11pp · import #223 · from **§10.0** (ceg)</sub>

All CEG endpoints return `Content-Type: application/json` (other Accept types → `406 Not Acceptable`),
a `CEG-Version` header on every response (clients SHOULD echo `CEG-Accept-Version` naming the version
they were built against; per SemVer policy, MAJOR mismatch is a wire-incompat reject, MINOR is
compatible), an `X-CEG-Server-Time` header for clock-skew bounds, and — where applicable — cursor +
limit pagination with `next_cursor` and `total_estimate`. Rate-limit headers (`X-RateLimit-Limit /
-Remaining / -Reset`) ride every response.

#### 5.3.6.1 `envelope-error` — Error envelope
<sub>budget 0.33pp · import #92 · from **§10.0.1** (ceg)</sub>

All error responses MUST conform to a single shape — an `error` object carrying `code`, `http_status`,
a human-readable `message`, a server-assigned `request_id`, and error-specific `details`. The closed
error-code set:

| HTTP | Error code | Meaning |
|---|---|---|
| 400 | `MALFORMED_REQUEST` | Invalid JSON, missing required field, bad field type |
| 400 | `CANONICAL_BYTES_VIOLATION` | Date-time / hex / encoding doesn't match the canonicalization rules |
| 401 | `UNAUTHENTICATED` | Bearer token missing or invalid (admin endpoints) |
| 403 | `RESERVED_PREFIX_VIOLATION` | Producer emitted under a reserved prefix without authority |
| 404 | `UNKNOWN_WITNESS` | Witness key_id not registered in the directory (5.3.1) |
| 404 | `NOT_FOUND` | Generic resource not found (build, partner, key) |
| 409 | `IDEMPOTENT_CONFLICT` | Replay detected (e.g. duplicate `(tree_size, witness_key_id)` with different signatures) |
| 422 | `SIGNATURE_VERIFICATION_FAILED` | Ed25519 or ML-DSA-65 failed; `details.algorithm` names which |
| 422 | `CLOCK_SKEW_VIOLATION` | `signed_at` exceeds the ±5-minute tolerance |
| 422 | `WITNESS_QUORUM_NOT_MET` | Insufficient cosignatures to validate |
| 422 | `CONSISTENCY_PROOF_INVALID` | A cosignature's consistency proof (5.3.1.1) is absent or fails |
| 429 | `RATE_LIMITED` | `X-RateLimit-*` headers set; `Retry-After` honored |
| 500 | `INTERNAL_ERROR` | Server-side fault; `request_id` usable for support |
| 503 | `WITNESS_DIRECTORY_UNAVAILABLE` | Substrate replication lag exceeds the liveness bound |

A closed, named error set is itself an [integrity](part_1_foundation.md#18-integrity--integrity)
discipline: every failure has a re-derivable name, so "it was rejected" is always a checkable verdict
rather than an opaque server mood.

---

[← Part IV — Composition & Governance](part_4_composition_governance.md) | **Part V — Transport & Substrate** | [Part VI — The Coherence Mathematics →](part_6_the_coherence_mathematics.md)

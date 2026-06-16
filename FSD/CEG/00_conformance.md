[← Back to CEG README](README.md) | **§0 Conformance** | [Next: §1 Foundation →](01_foundation.md)

---

# §0 Foreword

CEG — the CIRIS Epistemic Grammar — is the federation's language for making **structured, signed, machine-checkable claims about reality and each other**. It is the wire format the federation's peers speak.

The grammar has exactly five wire-format primitives (one workhorse + four structural composers) and an open-vocabulary dimension namespace organized by mechanism-descriptive prefixes. Consumers compose verdicts from primitive attestations using the policies in [§8](08_composition.md); nothing in the wire format prescribes what verdict to reach.

CEG is **substrate-consuming**: it sits above the federation substrate (CIRISPersist for storage, CIRISVerify for crypto, CIRISEdge for transport) and below the application tier (CIRISAgent). It does not author primitives in the substrate it consumes; it composes policy over them. It is also **substrate-supplying** for the second-tier consensus crates (CIRISNodeCore for consensus, CIRISLensCore for detection) — they own slices of the dimension namespace and emit attestations that other CEG consumers read.

This specification has **two readerships**:
- **Implementers** of federation primitives consuming or emitting CEG attestations: read §1-§11 normative.
- **Translators** mapping substantive content into CEG envelopes: read §12-§14 + the [`LANGUAGE_PRIMER.md`](../LANGUAGE_PRIMER.md) companion.

Both readerships should read [§1](01_foundation.md) first.

---

## §0.1 Conformance language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in [BCP 14](https://www.rfc-editor.org/info/bcp14) ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) + [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174)) when, and only when, they appear in all capitals, as shown here.

### §0.1.1 Normative vs informative content (what interop requires)

A reader may **agree with the protocol and disagree with its philosophy** and still build a fully conforming implementation. The two are deliberately separable:

- **Normative (binding for interoperability):** the wire format and its conformance surface — the [§3](03_primitives.md) structural primitives; the [§4](04_envelope.md) envelope fields; the [§5](05_namespace.md) namespace + `subject_kind`s; the [§0.5–0.9](00_conformance.md) canonicalization rules; the [§6](06_relations.md) relation precedence; the [§7](07_reserved.md) reserved-prefix rules; the [§8](08_composition.md) composition policies; the [§10](10_endpoints.md) endpoint shapes; and every RFC-2119-keyworded statement. This is exactly the surface enumerated in [§1.4](01_foundation.md) ("report the surface beside the invariant"). **Conformance is judged against this and nothing else.**
- **Informative (explanatory framing; NOT binding):** the motivating philosophy and rationale — notably [§1.2](01_foundation.md) (the Ubuntu / relational-anthropology substrate), the cross-tradition readings, and prose written as motivation rather than requirement. These explain *why* the normative choices were made; they add **no** conformance obligations. An implementer who rejects the anthropology but emits/consumes wire-correct Contributions is conforming.

Where framing produced a concrete wire consequence, that consequence is restated as a normative rule in its own section (e.g., structural invisibility is motivated informatively but enforced normatively at [§10.1.4](10_endpoints.md), bounded by [§1.6](01_foundation.md)). When in doubt, the RFC-2119 keywords and the [§1.4](01_foundation.md) surface govern; informative prose never overrides them.

---

## §0.2 Conformance levels

A **Producer** is any peer that emits Contributions onto the federation wire. A **Consumer** is any peer that reads and composes verdicts over received Contributions. A **Substrate Implementation** is the storage + transport + crypto layer (CIRISPersist + CIRISEdge + CIRISVerify) underneath both.

Three normative conformance profiles:

1. **CEG-Conforming Producer (CCP)** — emits well-formed envelopes per [§4](04_envelope.md), signs per §0.4 References [hybrid-sig], respects reserved-prefix rules per [§7](07_reserved.md), declares its `oversight_mode` and `witness_relation` per [§4](04_envelope.md).
2. **CEG-Conforming Consumer (CCC)** — verifies hybrid signatures, enforces reserved-prefix rules at admission, implements at least Policy A ([§8.1.1](08_composition.md)) with the default aggregation rules from [§8.2](08_composition.md), MUST honor `null` placeholder/dev hardware-class rejection per [§9.4](09_humanity_accord.md).
3. **CEG-Conforming Substrate (CCS)** — implements the storage + transport guarantees referenced in [§10.1](10_endpoints.md) + [§10.3](10_endpoints.md), including idempotent replication, full-SHA blob verification before consumption ([§10.1](10_endpoints.md)), and witness-quorum multi-party admission per [§10.3](10_endpoints.md).

Sections that follow MAY add per-feature conformance subsections; the three profiles above are the minimums.

---

## §0.3 Versioning policy

CEG follows **SemVer 2.0.0** with these mapping rules:

- **MAJOR (X.0.0)** — any wire-incompatible change: removal of an envelope field, change of a field's semantic, removal of a structural primitive, change to canonical-bytes domain-separation labels, removal or breaking-redefinition of a [§5](05_namespace.md) prefix, change to a [§7](07_reserved.md) reservation, or change to the §0.1 / §0.2 conformance language.
- **MINOR (0.X.0)** — wire-compatible additions: new prefix in [§5](05_namespace.md), new envelope field with documented default, new composition policy in [§8](08_composition.md), new endpoint shape in [§10](10_endpoints.md), new optional conformance subsection. Existing Conforming Producers and Consumers continue to interoperate without modification.
- **PATCH (0.0.X)** — clarifications, editorial fixes, additions to non-normative sections ([WITNESS_KIND_REGISTRY](../WITNESS_KIND_REGISTRY.md), glossaries [§14](14_glossaries.md)), addition to [§15](15_gaps.md) acknowledged-gaps, fixes to non-normative examples in [§14](14_glossaries.md).

The 0.x series indicates this specification is a Public Working Draft. Any 0.x → 0.(x+1) bump MAY include wire-breaking changes; consumers MUST treat 0.x as unstable until 1.0 publication. Once 1.0 is published, the rules above bind strictly.

A **deprecation** is announced by adding a `**DEPRECATED in 0.X**` marker to the affected element with a stated removal target (e.g., `removal: 1.2`). Deprecated elements MUST remain interoperable until the announced removal version. Removal in MAJOR or 0.MINOR per the rules above.

---

## §0.4 Normative References

The following documents are normatively cited; implementations MUST conform to them where referenced inline.

| Short name | Normative document |
|---|---|
| [BCP 14] | [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) + [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174) — keywords for use in RFCs |
| [FIPS-180-4] | [FIPS 180-4](https://csrc.nist.gov/pubs/fips/180-4/upd1/final) — SHA-256 and the SHA-2 family |
| [FIPS-202] | [FIPS 202](https://csrc.nist.gov/pubs/fips/202/final) — SHA-3 / SHAKE128 / SHAKE256 / TupleHash |
| [FIPS-204] | [FIPS 204](https://csrc.nist.gov/pubs/fips/204/final) — ML-DSA (Module-Lattice-Based Digital Signature Algorithm); CEG uses parameter set ML-DSA-65 |
| [RFC-3339] | [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339) — Date and Time on the Internet, with fractional-seconds disambiguation in §0.5 below |
| [RFC-5905] | [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905) — Network Time Protocol Version 4 |
| [RFC-6962] | [RFC 6962](https://www.rfc-editor.org/rfc/rfc6962) — Certificate Transparency; this spec's transparency-log discipline tracks 6962 except where 6962-bis (RFC 9162) supersedes |
| [RFC-8032] | [RFC 8032](https://www.rfc-editor.org/rfc/rfc8032) — Edwards-Curve Digital Signature Algorithm (EdDSA); specifically Ed25519 |
| [RFC-8174] | [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174) — Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words (with BCP 14) |
| [RFC-8785] | [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785) — JSON Canonicalization Scheme (JCS); used where this spec serializes JSON for signing |
| [RFC-9162] | [RFC 9162](https://www.rfc-editor.org/rfc/rfc9162) — Certificate Transparency v2.0 (CT-bis); MUST be used for new transparency-log integrations; older 6962 instances continue to interoperate |
| [ISO-639-1] | [ISO 639-1:2002](https://www.iso.org/standard/22109.html) — Codes for the representation of names of languages, two-letter |
| [BCP-47] | [BCP 47](https://www.rfc-editor.org/info/bcp14) ([RFC 5646](https://www.rfc-editor.org/rfc/rfc5646)) — Tags for identifying languages; for locale strings richer than ISO 639-1 alone |
| [RFC-9420] | [RFC 9420](https://www.rfc-editor.org/rfc/rfc9420) — Messaging Layer Security (MLS); the streaming epoch-key rekey ([§10.5.3](10_endpoints.md)) conforms to MLS TreeKEM |
| [SFrame] | [draft-ietf-sframe](https://datatracker.ietf.org/wg/sframe/about/) — Secure Frames; the per-frame AEAD chunk seal ([§10.5.2](10_endpoints.md)) conforms to the SFrame model |
| [FIPS-203] | [FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) — ML-KEM (Module-Lattice KEM); the post-quantum half of the hybrid KEM (ML-KEM-768) |
| [FIPS-204] | [FIPS 204](https://csrc.nist.gov/pubs/fips/204/final) — ML-DSA (Module-Lattice signatures); the post-quantum half of the hybrid signature (ML-DSA-65) |
| [RFC-9180] | [RFC 9180](https://www.rfc-editor.org/rfc/rfc9180) — Hybrid Public Key Encryption (HPKE); the `key_grant` wrap shape |

Informational citations (Magnifica Humanitas, anthropological literature, Ubuntu philosophical literature, etc.) appear in [§16.4](16_references.md) without normative force.

---

## §0.5 Date-time canonicalization

Every ISO 8601 / RFC 3339 datetime in this specification MUST be:

- UTC (suffix: literal `Z`; the offset form `+00:00` MUST NOT be used)
- Millisecond-precision (exactly three digits of fractional seconds; trailing zeros required)
- Lowercase `z` MUST NOT be used; uppercase `Z` only

Canonical form: `YYYY-MM-DDTHH:MM:SS.sssZ`. Example: `2026-05-28T13:45:09.000Z`. Producers MUST emit this form; consumers MUST reject any other form when verifying a signature.

---

## §0.6 Hexadecimal canonicalization

Every hex string used in canonical-bytes encoding (e.g., SHA-256 digests in `root_hash`, public-key fingerprints) MUST be **lowercase**, **unpadded** (no leading `0x`, no separators), and **byte-length-exact** (a SHA-256 digest is exactly 64 hex characters). Producers MUST emit lowercase; consumers MUST reject uppercase when verifying.

---

## §0.7 Time and clocks

Every `signed_at`, `asserted_at`, `valid_until`, `delegation_valid_from`, `delegation_valid_until`, and `cosigned_at` in this specification refers to **wall-clock UTC** at the asserting peer's clock. Producers SHOULD synchronize via [NTPv4 (RFC 5905)](https://www.rfc-editor.org/rfc/rfc5905) or [Roughtime](https://datatracker.ietf.org/doc/draft-ietf-ntp-roughtime/) to a known-good time source. The maximum tolerated skew between attester clock and consumer clock for a freshness check is **±5 minutes** by default; tighter thresholds MAY be applied by per-application consumer policy. Consumers receiving an attestation with `signed_at` more than 5 minutes in the future MUST reject as malformed.

Time-skew between cosigners on a single STH ([§10.3](10_endpoints.md)) is bounded by the STH's own `signed_at` field; cosignatures with `signed_at` farther than 5 minutes from the STH's published `signed_at` MUST be rejected.

For long-lived attestations carrying `valid_until` in the future, the freshness check is "the attestation has not yet reached its `valid_until`, AND the current consumer clock is within ±5 minutes of the substrate's network-consensus clock"; a consumer whose clock drifts past the skew bound MUST fail-secure (reject) rather than accept.

---

## §0.8 H3 cell canonicalization (CEG 0.8 addition)

Per [CIRISRegistry#48](https://github.com/CIRISAI/CIRISRegistry/issues/48) + [§5.6.8.11](05_namespace.md) `location_proof` + [§5.6.8.10](05_namespace.md) `community` with `cohort_subkind: geographic`.

Geographic primitives in CEG use [H3 hierarchical hexagonal indexing](https://h3geo.org/) as the canonical cell-identifier encoding. H3 partitions the Earth's surface into hexagonal cells at 16 resolution levels (0 = coarsest, ~4.3 M km² per cell; 15 = finest, ~0.9 m² per cell). Each cell has a 64-bit integer ID, conventionally encoded as a 15-character lowercase hex string.

**Canonical form for `cell_id`**:

- 15-character lowercase hex string (no `0x` prefix; per [§0.6](#06-hexadecimal-canonicalization))
- The cell encodes its own resolution in the standard H3 index bit layout; a conformant decoder extracts the resolution **via the H3 library** (the resolution field lives in bits 52–55), **NOT** by reading the high 4 bits — the high nibble is the H3 **mode marker** (cell-mode = `1`), not the resolution
- Leading zeros preserved (a resolution-0 cell at base position 0 is `8001fffffffffff`, not `1fffffffffff` — the high nibble `8` is the mode marker, correctly consistent with res-0)

**Canonical form for `cell_resolution`**:

- Integer in `[0, 15]`
- MUST equal the resolution **decoded from the H3 index** of `cell_id` (substrate verifies the redundancy on admission via the H3 library; mismatched pairs MUST be rejected as malformed)

### §0.8.1 Rough-only enforcement for `location_proof` (normative)

Per [CIRISRegistry#48](https://github.com/CIRISAI/CIRISRegistry/issues/48) privacy invariant. A `location_proof` subject_kind Contribution ([§5.6.8.11](05_namespace.md)) MUST carry `cell_resolution ≤ 7` (H3 resolution 7 hexagons average ~5 km² edge-length, sufficient for city/borough-level disclosure without block/building precision). Producers attempting to emit finer-resolution `location_proof` Contributions MUST have admission rejected at the substrate gate.

This is the wire-format-enforced privacy promise: **rough is rough by protocol**, not by operator policy. A producer cannot accidentally over-share at the protocol layer; a malformed client cannot publish a precise location even if its UI fails to gate. Substrate emits `hard_case:location_proof_resolution_violation` ([§7.8](07_reserved.md)) on rejection so operators can observe malformed-producer patterns.

### §0.8.2 Cell containment

A cell `C` at resolution `R_C` is **contained within** a cell `B` at resolution `R_B` iff:

- `R_C >= R_B` (the contained cell is at equal or finer resolution); AND
- The parent-walk from `C` at resolution `R_C - 1, R_C - 2, ..., R_B` reaches exactly `B` (standard H3 hierarchy semantics; `h3ToParent` library call)

Used by community admission gates per [§8.1.13](08_composition.md) Policy M: a `location_proof` Contribution's `cell_id` MUST be contained within the geographic community's `geographic_constraint.cell_id` for membership admission.

### §0.8.3 What CEG 0.8 documents

Scope pointer: the H3 canonicalization rules above (§0.8 lowercase-hex `cell_id`, §0.8.1 rough-only `cell_resolution ≤ 7`, §0.8.2 containment for community admission).

What CEG 0.8 does NOT do:
- Mandate H3 over alternative geospatial systems (S2, Geohash) — H3 is chosen for hex-cell uniformity, well-defined parent/child hierarchy, and protocol-agnostic absence of a centralized gazetteer dependency. Operator-internal use of other systems is unconstrained; the wire format uses H3.
- Provide a place-name registry. Communities self-name; cell IDs are the substrate-level binding. UI may map cell IDs to human-readable names per consumer policy.
- Restrict community-side `geographic_constraint.cell_resolution`. A community CAN scope itself at any resolution (e.g., an "Austin metro" community at resolution 5, ~250 km²; a smaller-scale "Downtown Austin" community at resolution 7, ~5 km²). The rough-only invariant applies to `location_proof`, NOT to community definitions.

---

## §0.9 Envelope canonicalization — JCS + the omit-vs-materialize rule (CEG 0.9 addition)

Per external critical-review surface (the round-trip-determinism concern that landed against CEG ≤ 0.8). Closes the canonical-bytes ambiguity that accumulated as CEG 0.x acquired optional fields with documented defaults — `epistemic_mode`/`witness_relation`/`occurrence_*`/`stake` (pre-0.4 baseline), `oversight_mode` (pre-0.4 + CEG 0.2 ladder rework), `subject_key_ids` (CEG 0.6), `family_id` (CEG 0.7), `community_id` (CEG 0.8).

**The hazard is structural and the reviewer named it correctly.** If Producer A omits an optional field (relying on the [§4](04_envelope.md) documented default) and Relay R re-serializes the attestation with the default materialized into the byte stream, the canonical bytes diverge and the Ed25519 + ML-DSA-65 signatures no longer verify. Pre-§0.9 CEG implicitly required producers, substrates, relays, and consumers to all make the same choice; §0.9 makes the rule explicit and normative.

### §0.9.1 Canonical encoding format (normative)

CEG envelope signing bytes are computed via **JCS — JSON Canonicalization Scheme, [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785)** (Rundgren, Jordan, Erdtman; March 2020). JCS pins, in summary:

- Object members sorted lexicographically by member name (UTF-16 code-unit order, RFC 8785 §3.2.3)
- No insignificant whitespace
- UTF-8 byte encoding (RFC 8259 §8.1)
- Strings escaped per RFC 8259 §7 with the JCS narrowing on `\uXXXX` form
- Numbers serialized per the ES6-derived rule (RFC 8785 §3.2.2.3) — integers without trailing `.0`, no exponent unless the magnitude requires it

A CEG-Conforming Producer (CCP per [§0.2](#02-conformance-levels)) MUST produce signing bytes via JCS over the envelope object. A CEG-Conforming Consumer (CCC) MUST recompute signing bytes via the same JCS rule for signature verification. A CEG-Conforming Substrate (CCS) MUST preserve the as-received envelope object bytes for relay (per §0.9.2 below); it MAY store a parsed representation alongside, but the canonical-bytes contract is against the as-received form, not the parsed-and-re-serialized form.

### §0.9.2 The round-trip rule — defaults are interpretation-time, not encoding-time (normative)

The canonical bytes are computed over **the literal envelope members the producer signs**. Optional fields that the producer omits MUST NOT be materialized into canonical bytes by any party — producer, substrate, relay, or consumer. Optional fields that the producer explicitly emits (even at their default value) MUST be preserved in the canonical bytes by any party. Documented defaults from [§4](04_envelope.md) are **interpretation-time semantics**, not encoding-time content.

**Two valid encodings of the semantically-identical attestation**:

```
# Producer A — omits epistemic_mode (relies on §4 default)
{"attested_key_id":"...","attesting_key_id":"...","confidence":0.9,"dimension":"licensure:CA_medical_board","score":0.8}

# Producer B — explicitly emits epistemic_mode at its default value
{"attested_key_id":"...","attesting_key_id":"...","confidence":0.9,"dimension":"licensure:CA_medical_board","epistemic_mode":"direct","score":0.8}
```

Both producers compute different canonical bytes (Producer B's includes the `"epistemic_mode":"direct"` member) and emit different signatures. **Both signatures verify under their respective canonical bytes.** Both are **semantically equivalent** — a CEG-Conforming Consumer evaluates `effective_epistemic_mode(envelope) = envelope.epistemic_mode if present else "direct"` and proceeds identically. The wire-format admits both encodings; the consumer policy admits no observable difference.

**Relay discipline (normative)**. A substrate, relay, or consumer that re-stores or forwards an attestation MUST preserve member presence/absence exactly as the producer signed it:

- Stripping an explicitly-emitted default ("the producer wrote `epistemic_mode:direct` but I'll save bytes by removing it") MUST NOT happen
- Materializing an omitted default ("the producer omitted `epistemic_mode` so I'll fill in `direct` for clarity") MUST NOT happen
- Reordering object members on re-emission is REQUIRED (JCS lexicographic order; if a producer somehow emits non-canonical order, the relay re-canonicalizes — but member presence/absence stays fixed)

This composes with the [§4.1](04_envelope.md) forward-compatibility rule (which already mandates preserving unknown fields on read and re-emission). §0.9.2 extends the same preservation discipline to known-optional fields.

### §0.9.2.1 Array ordering + byte-field + timestamp encoding (normative — the three determinism rules JCS does NOT pin)

JCS ([§0.9.1](#091-canonical-encoding-format-normative)) canonicalizes **object member order** but is silent on three things that still let two conformant producers emit different bytes (and therefore non-verifying signatures, the §0.9 hazard). All three are pinned here once, globally, for every CEG envelope (raised in CIRISVerify#63 review):

1. **Array element order.** An array field is one of two kinds, stated in its [§4](04_envelope.md)/[§5](05_namespace.md) definition:
   - **Set-semantics** (order carries no meaning) — elements MUST be **lexicographically sorted by their JCS string form (UTF-16 code-unit order, ascending)** before signing. This is producer-independent: any producer building the set in any order yields identical bytes. **`subject_key_ids[]` and `delegates_to.delegated_scope[]` are set-semantics → sorted.**
   - **Sequence-semantics** (order is meaningful) — elements retain their as-authored order. **`transport_destination.aspects[]`** (RNS hash preimage order), **`key_grant.rotation_chain`** (supersession lineage), and **`evidence_refs[]`** (producer-asserted order) are sequence-semantics → preserved. A field's definition MUST declare which kind it is; absent a declaration, set-semantics (sorted) is the default.
2. **Byte-field string encoding.** JSON has no byte type, so every byte/binary field is a string whose encoding MUST be pinned. **Key references, hashes, and identifiers — `key_id` / `attesting_key_id` / `attested_key_id` / `subject_key_ids[]` elements, public keys, `destination_hash`, `root_hash`, fingerprints — MUST be lowercase hex per [§0.6](#06-hexadecimal-canonicalization)** (no `0x`, no separators, byte-length-exact; never base64 in canonical bytes). Fields explicitly documented as base64 (e.g., `hardware_attestation`, an opaque attestation blob) use base64 as their definition states — but the *default* for any key/hash/id byte field is §0.6 lowercase hex.
   - **The base64 variant is pinned (1.0-RC1 — [CIRISVerify#64](https://github.com/CIRISAI/CIRISVerify/issues/64)): every field documented as base64 (the `*_base64` suffix convention — `encryption_pubkeys.x25519_base64` / `.ml_kem_768_base64` per [§5.6.8.8.2](05_namespace.md), the directory `pubkey_ed25519_base64` / `pubkey_ml_dsa_65_base64`, `hardware_attestation`) MUST use the RFC 4648 §4 STANDARD alphabet, WITH padding, no whitespace or embedded newlines** (the `base64::engine::general_purpose::STANDARD` shape `ciris-verify-core` ships). The pin lives at the **protocol layer**: the crypto layer (`ciris-crypto`) is correctly encoding-agnostic and deals in raw bytes; the wire encoding is CEG's to pin. A variant mismatch (URL-safe alphabet, stripped padding) produces different signed bytes and a **silently non-verifying signature** — the same hazard class as the wrap-algorithm wire-string. The [CIRISConformance#9](https://github.com/CIRISAI/CIRISConformance/issues/9) vector set MUST include a base64-variant case (encode→sign→verify round-trip across two independent encoders).
3. **Timestamp encoding.** Every datetime field (`signed_at`, `asserted_at`, `valid_until`, `delegation_valid_from`/`_until`, `valid_at`, …) is the **[§0.5](#05-date-time-canonicalization) canonical string** — UTC literal `Z` (never `+00:00`), millisecond precision (exactly three fractional digits), `YYYY-MM-DDTHH:MM:SS.sssZ`. A producer emitting `+00:00` or a different sub-second precision produces different bytes and a non-verifying signature.

With §0.9.1 (key order) + §0.9.2 (omit-vs-materialize) + these three, **byte-identity across conformant implementations is closed by construction**. The [CIRISConformance#9](https://github.com/CIRISAI/CIRISConformance/issues/9) cross-impl JCS vector set MUST cover all three (a set-vs-sequence array case, a hex-vs-base64 byte case, a timestamp case) alongside the per-Contribution vectors.

### §0.9.3 Per-field encoding table (informational)

The §0.9.2 rule applies uniformly to every optional [§4](04_envelope.md) field. Catalog as of CEG 0.9:

| Field | Introduced | Default per §4 | Canonical when omitted | Canonical when explicit |
|---|---|---|---|---|
| `epistemic_mode` | pre-0.4 | `direct` | member absent | `"epistemic_mode":"direct"` (or other enum value) |
| `witness_relation` | pre-0.4 | `external` | member absent | `"witness_relation":"self"` (or other enum value) |
| `oversight_mode` | pre-0.4 | `null` (per-cell default applies) | member absent | `"oversight_mode":"HITL"` (or other enum value) |
| `occurrence_id` | pre-0.4 | `null` → `"occurrence-0"` at interpretation | member absent | `"occurrence_id":"occurrence-1"` |
| `occurrence_count` | pre-0.4 | `null` → `1` at interpretation | member absent | `"occurrence_count":3` |
| `occurrence_role` | pre-0.4 | `null` → `"primary"` at interpretation | member absent | `"occurrence_role":"shared"` |
| `stake` | pre-0.4 | `reputational` | member absent | `"stake":"capital"` (or other enum value) |
| `context` | pre-0.4 | absent | member absent | `"context":"..."` (free-form) |
| `evidence_refs` | pre-0.4 | absent | member absent | `"evidence_refs":["..."]` |
| `valid_until` | pre-0.4 | absent | member absent | `"valid_until":"2026-12-31T00:00:00.000Z"` |
| `subject_key_ids` | CEG 0.6 | `null`/empty | member absent | `"subject_key_ids":["..."]` |
| `family_id` | CEG 0.7 | n/a (REQUIRED iff `cohort_scope == family`) | member absent (admission rejects if cohort_scope == family) | `"family_id":"..."` |
| `community_id` | CEG 0.8 | n/a (REQUIRED iff `cohort_scope == community`) | member absent (admission rejects if cohort_scope == community) | `"community_id":"..."` |
| `delivery_mode` | CEG 0.10 | `pull` | member absent | `"delivery_mode":"push"` |
| `listed` | CEG 0.10 | absent (private roster) | member absent | `"listed":"public"` (opt-in only per [§11.8.3](11_governance.md)-shape; producers MUST omit unless subject has opted in) |
| `history_on_join` | CEG 0.10 | `from_join` | member absent | `"history_on_join":"full"` |

The conditional-required fields `family_id` and `community_id` are NOT optional-with-default — substrate rejects mis-shape per [§4.2.6](04_envelope.md) + [§11.6.2](11_governance.md) + [§11.7.2](11_governance.md) — but they encode under the same JCS rule when present.

### §0.9.4 Verification flow (informational)

```
On signature verify:
    1. Receive envelope from wire as object O (preserved as-received)
       and signature S
    2. Compute canonical bytes B = JCS(O)
    3. Verify S over B via the hybrid Ed25519 + ML-DSA-65 path per §5.2.1
    4. If signature valid:
        a. Compute effective semantics by applying §4 defaults to absent
           optional fields (interpretation-time only)
        b. Apply consumer policy per §8 over the resulting semantic shape
    5. If forwarding/storing:
        a. Store/forward object O AS RECEIVED — do not normalize, strip,
           or materialize defaults
        b. Forward original signature S unchanged
```

### §0.9.5 Worked attack the §0.9 rule closes

Pre-§0.9 (implicit / unspecified) failure mode:

> Alice's CEG-Conforming Producer signs an envelope omitting `epistemic_mode`. Bob's CEG-Conforming Relay receives, parses, materializes the default `"direct"`, re-serializes via JCS, and forwards to Carol. Carol receives the relay-modified envelope with the new bytes, computes JCS, verifies signature → **FAILS** because Alice signed over bytes without the `epistemic_mode` member. Carol cannot distinguish "Bob is a malicious relay corrupting the bytes" from "Bob is honestly applying defaults to be helpful." Carol rejects. The attestation is lost in transit despite no party acting in bad faith.

Post-§0.9 (explicit, normative):

> Bob's relay MUST preserve member presence/absence exactly. Bob forwards the as-received bytes. Carol's verify succeeds. The semantic interpretation step at Carol (applying default `"direct"` to the absent member) happens AFTER signature verification.

### §0.9.6 What CEG 0.9 (this section) documents

Scope pointer: the JCS canonicalization rules above (§0.9 JCS-as-encoding, §0.9.2 omit-vs-materialize + relay-preservation, §0.9.3 per-field catalog, §0.9.5 worked attack).

What CEG 0.9 (this section) does NOT do:
- Introduce a new encoding format — JCS is the only encoding
- Change any semantic interpretation — defaults still apply at interpretation time per [§4](04_envelope.md)
- Modify the §0.5 datetime / §0.6 hex / §0.7 time / §0.8 H3 sub-rules — those are domain-specific and compose under JCS
- Wire-break prior 0.x emissions — pre-§0.9 emissions that omitted optional fields remain valid; pre-§0.9 emissions that explicitly emitted defaults likewise remain valid; what §0.9 normatively prohibits is RELAY-TIME mutation of presence/absence, which was always wrong but is now explicitly so

## §0.10 NodeCode — the canonical `key_id` shorthand encoding (normative, 1.0-RC3 addition)

Federation `key_id`s are long opaque identifiers. **NodeCode** is the **one** human-shareable shorthand — a compact, QR-able, checksummed render of a peer's identity for **bootstrap UX** (type / paste / scan to add a peer → `trust=UNKNOWN` per [CIRISEdge#46](https://github.com/CIRISAI/CIRISEdge/issues/46); SAS-verify promotes `UNKNOWN → TRUSTED` per [CIRISEdge#47](https://github.com/CIRISAI/CIRISEdge/issues/47)). It is pinned here (per [CIRISRegistry#75](https://github.com/CIRISAI/CIRISRegistry/issues/75), lifted from the shipped `CIRISAgent` codec) so **every implementation renders and parses the same code for the same key** — a cross-impl determinism requirement of the same class as [§0.6](#06-hexadecimal-canonicalization) hex / [§0.9](#09-envelope-canonicalization--jcs--the-omit-vs-materialize-rule-ceg-09-addition) JCS. It is a deterministic *render of an existing `key_id`*, **not** a new envelope field — additive on the frozen 1+4 surface. NodeCode resolution is **DNS-free**: the decoded `key_id` resolves to a destination via the signed `transport_destination` → Reticulum chain ([§5.6.8.8.1](05_namespace.md) / [§8.1.13.1.1](08_composition.md)); a NodeCode carries no hostname.

**Binary payload (normative):**

```
offset  size  field
------  ----  -----
   0      1   version                 = 0x01
   1     32   key_id_hash             = SHA-256(key_id_str, UTF-8)
  33     32   pubkey_ed25519          (raw 32 bytes)
  65      1   key_id_str_len          (0–255)
  66      N   key_id_str              (UTF-8)
 66+N     1   transport_hint_len      (0–255)
 67+N     M   transport_hint          (UTF-8; OPTIONAL — len 0 if absent)
67+N+M    1   alias_hint_len          (0–255)
68+N+M    K   alias_hint              (UTF-8; OPTIONAL — len 0 if absent)
   …      2   crc16                   = CRC-16-CCITT over ALL preceding bytes
```

- All length-prefixed fields are **1-byte** length (max 255 UTF-8 bytes); a field overflow is a malformed NodeCode.
- `key_id_hash` is the stable 32-byte fingerprint (suitable for binary-only Edge ANNOUNCE surfaces); `key_id_str` carries the display form so a round-trip preserves exactly what the user saw. Both are carried — a decoder MUST verify `SHA-256(key_id_str) == key_id_hash`.
- **CRC-16-CCITT**: polynomial `0x1021`, init `0xFFFF`, **no** final xor, big-endian; computed over every byte before the trailing 2.

**String form (normative):** the payload is **RFC 4648 base32** (alphabet `A–Z2–7`) with padding **stripped** on encode (re-padded on decode), then split into **4-character groups joined by `-`** and prefixed with **`CIRIS-V1-`**:

```
CIRIS-V1-ABCD-EFGH-IJKL-…
```

The encoded form is **case-insensitive** (decoder upper-cases input) and a conformant decoder MUST tolerate dashes, embedded whitespace, and the dash-free QR form. The version token in the prefix (`V1`) tracks the payload `version` byte; a future layout bumps both.

---

[← Back to CEG README](README.md) | **§0 Conformance** | [Next: §1 Foundation →](01_foundation.md)

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

[← Back to CEG README](README.md) | **§0 Conformance** | [Next: §1 Foundation →](01_foundation.md)

[← §17 Cadence](17_cadence.md) | **§18 Interoperability profiles** | [Next: §19 Holonomic →](19_holonomic.md)

---

# §18 Interoperability profiles (informative)

> **This whole section is informative ([§0.1.1](00_conformance.md)).** Nothing here touches the frozen normative interior. These are **boundary** profiles — how a CEG node reads and emits the encodings, envelopes, and verification primitives the rest of the world shares, **without** adopting anyone else's *semantics*. The 1+4 grammar, the namespaces, the consent architecture, and the JCS signing interior are unchanged. Conformance is still judged against the normative surface only.

## §18.0 The governing principle — speak CEG inside, standards at the edge

CEG's moat is its **semantics**: the 1+4 grammar, the consent architecture, founder-quorum trust, who-vouches-for-what-revocable-by-whom. We never adopt anyone's semantics. We adopt the **envelopes, encodings, and verification primitives** everyone shares — **at the boundary only**. A second *interior* canonicalization or claim family would recreate the cross-impl divergence hazard the [§0.9](00_conformance.md) JCS freeze exists to close ([§5.2.1](05_namespace.md) records this decision); so the interior stays one family, frozen, and every standard below is reached at an edge.

Four boundary modes (the full roadmap + dispositions live in [CIRISRegistry#72](https://github.com/CIRISAI/CIRISRegistry/issues/72)):

| Mode | Meaning | Standards |
|---|---|---|
| **Export profile** | re-sign / re-encode a CEG attestation so a standard verifier reads it without knowing CEG | COSE Sign1, RFC 9421 (HTTP Message Signatures / Web Bot Auth), SD-JWT VC presentation |
| **Import bridge** | a foreign signed artifact is **cited** via [`evidence_refs`](04_envelope.md) (deliberately lossy) | **C2PA manifests** (§18.1), eIDAS / W3C VC credentials, Sigstore/Rekor bundles |
| **Already interior** | the standard *is* a primitive CEG builds on | MLS / TreeKEM ([§10.5.3](10_endpoints.md)), RFC 6962 / 9162 transparency logs ([§10.3](10_endpoints.md)), SLSA (`provenance:slsa:{level}`, [§5.2](05_namespace.md)) |
| **Explicitly NOT adopted** | vendor rails / competing semantic layers | DIDs *as a resolution stack* (export syntax only), AP2 / Visa TAP / Mastercard Agent Pay (bridge via [§5.6.8.12](05_namespace.md) `settlement`), SPIFFE (datacenter-tier mapping only) |

**The universal "absorb anywhere" surface is [`evidence_refs[]`](04_envelope.md).** Any Contribution may cite an external signed artifact as evidence with **zero wire change**. That is how foreign provenance enters CEG — not by replacing the interior encoding, but by reference, with CEG layering the epistemic claims (who vouches, under what consent, with what confidence) on top. The composition is the differentiated story: **provenance says where the bytes came from; CEG says what a community of signers makes of them.**

## §18.1 C2PA Content Credentials — media-provenance import/emit profile

**Disposition: ADOPT at the media boundary (import bridge + emit), zero interior wire change.** [C2PA](https://c2pa.org/) (Coalition for Content Provenance and Authenticity) Content Credentials are the industry standard (Adobe / Microsoft / Google / BBC …) for cryptographically signed media provenance — origin, edit history, and generator (incl. AI-generation) assertions embedded in or sidecar'd to images / video / audio. **Deadline driver:** EU AI Act Art. 50 machine-readable marking of AI-generated content applies from **2026-08** in the federation's primary jurisdiction, so this profile is calendar-bound, not optional. Owners: NodeCore / LensCore media ingest (the [§5.6.8](05_namespace.md) `multimedia` / `federation_blobs` boundary).

C2PA is **provenance**; CEG is **judgment**. They do not compete — they compose. C2PA answers *"what process produced these bytes, signed by whom?"*; CEG answers *"what does a community of signers, under what consent, make of them — and who can revoke that?"* Neither does the other's job; C2PA has no consent architecture, no revocation, no 1+4.

### §18.1.1 Import — a C2PA manifest as `evidence_refs`

When a `federation_blobs` row (or a `multimedia` Contribution over its SHA-256) carries a C2PA manifest, the manifest is referenced — **never re-encoded into the CEG interior** — through the existing external-reference pattern:

```
evidence_refs: [
  { kind:        "c2pa_manifest",                 // open-vocab evidence kind
    locator:     "<blob_sha256 of the .c2pa manifest store | embedded-offset ref>",
    manifest_sha256: "<sha256 of the active manifest, lowercase hex per §0.6>",
    claim_generator: "<the C2PA claim_generator string, verbatim>",
    validation:  "valid" | "invalid" | "unverified" } // the verifier's C2PA-side result, advisory
]
```

- The C2PA signature is verified **by a C2PA verifier** (trust-list / cert-chain per C2PA), NOT by a CEG signature path — the two trust models stay separate. The `validation` field carries that result as **advisory** evidence; it is never fed to a CEG hybrid-verify path (the [§5.6.8.8.2](05_namespace.md) key-separation discipline generalizes: foreign-trust-root material is payload, never CEG verification material).
- A CEG `scores` attestation may then assert a judgment **about** the provenanced bytes (e.g. `detection:multimedia:ai_generated` from LensCore, or a community `scores` endorsement), linking the C2PA evidence via `evidence_refs` and the media via `subject_key_ids` / the blob SHA. The CEG claim is signed CEG; the provenance it cites is signed C2PA; the reader sees both lineages without either standard absorbing the other.
- **Absent / invalid C2PA is not fail-secure-fatal** — it is itself a recordable observation (`validation: "invalid"` / no manifest). CEG records the gap; consumer/RATCHET policy weights it. The substrate is not a C2PA gatekeeper.

### §18.1.2 Emit — CEG judgment as a C2PA assertion (egress)

At a media-publish boundary a node MAY emit a C2PA assertion carrying a CEG attestation reference (a CAWG-identity-assertion-shaped custom assertion), so a pure-C2PA consumer downstream sees "this media is vouched-for in CEWP" without speaking CEG. This is an **export** at the edge (re-expressing an existing CEG attestation in C2PA's assertion envelope), parallel to the §18.2 COSE export profile — it adds no CEG wire field and re-signs nothing in the interior.

### §18.1.3 What this profile does NOT do

- It does **not** make C2PA an interior format. CEG envelopes are never C2PA-encoded; the JCS interior is untouched.
- It does **not** adopt C2PA's trust model as CEG's. C2PA cert-chains/trust-lists validate C2PA; founder-quorum/web-of-trust validates CEG. They meet only at `evidence_refs`.
- It introduces **no new `subject_kind` and no new structural primitive** — `c2pa_manifest` is one open-vocab `evidence_refs.kind`; the judgment rides existing `scores` + `detection:*`.

## §18.2 Tracked boundary profiles (stubs — see [#72](https://github.com/CIRISAI/CIRISRegistry/issues/72))

These are committed dispositions whose detailed profiles are written as each lands; none touches the frozen interior.

| Profile | Mode | Note |
|---|---|---|
| **RFC 9421 + Web Bot Auth** | export | CIRIS agents sign outbound HTTP with their existing Ed25519 keys; JWKS published at `/.well-known/http-message-signatures-directory` → legible to the existing web. Cheapest win; keys already in `identity_occurrence` / `federation_keys`. |
| **COSE Sign1 / deterministic CBOR** | export | Re-sign profile so any IETF JOSE/COSE verifier (where the ML-DSA registrations land) checks a CEG attestation. Interior stays JCS; if JCS keeps producing cross-impl bite post-1.0, 2.0 is the re-encoding moment, not before. |
| **SD-JWT VC / W3C VC 2.0 + OpenID4VP** | export + import bridge | eIDAS-forced (EUDI wallets); CEG attestation → SD-JWT VC presentation on export, eIDAS credential → `evidence_refs` on import. Never rebuild on VCs. |
| **Tiled/static logs + IETF KEYTRANS** | already-interior + watch | Keep the [§10.3](10_endpoints.md) RFC 6962 abstraction; adopt tiled-log (Sunlight-lineage) serialization for log ops. KEYTRANS is what `resolve_encryption_keys` already *is* — express it there when KEYTRANS stabilizes. |

---

[← §17 Cadence](17_cadence.md) | **§18 Interoperability profiles** | [Next: §19 Holonomic →](19_holonomic.md)

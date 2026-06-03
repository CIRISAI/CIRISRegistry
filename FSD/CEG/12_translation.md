[← §11 Governance](11_governance.md) | **§12 Translation** | [Next: §13 Anti-patterns →](13_anti_patterns.md)

---

# §12 Translation discipline (writing claims in CEG)

When translating substantive content into CEG envelopes, follow the discipline. Full primer at [`LANGUAGE_PRIMER.md`](../LANGUAGE_PRIMER.md); key rules consolidated here.

## §12.1 The five families (organizing the namespace)

Every claim sits in one of five families:

| Family | Question | Analogy |
|---|---|---|
| **STANDING** (about an entity) | "This key_id has property X." | Notarized professional credential record |
| **ACTION** (decision hierarchy) | "We aim for X via approach Y, through methods Z, measured by W." | Research grant proposal |
| **DETECTION** (reality patterns) | "Pattern X is/isn't present in the federation's behavior." | Epidemiological surveillance |
| **CONSENSUS** (collective judgment) | "The federation agrees that X, with these witnesses." | Peer review + jury deliberation |
| **CORRECTION** (self-correction) | "Something went wrong; here's the finding; here's the appeal." | Academic ethics committee + journal retraction + appellate review |

## §12.2 The four verdict categories (STRICT)

| Verdict | Meaning |
|---|---|
| **clean** | Single primitive captures the operational claim without loss. |
| **composed** | Two or three primitives together carry the claim; each is genuinely required. |
| **partial** | The structural core translates but a meaningful operational claim is unmapped. |
| **not-translated** | The paragraph's content does not translate into the wire format at all. Declare T-1 / T-2 / T-3. |

Do not invent intermediate categories.

## §12.3 The not-translated taxonomy

**T-1 — TRADITION_AUTHORITY**: Claim belongs to the source's own theological/philosophical/scholarly tradition's authority. No Contribution owed; the correct posture.

**T-2 — PASTORAL_PROSE**: Claim is moral exhortation, narrative imagery, doxological language, or rhetorical framing. No Contribution owed.

**T-3 — EXPRESSIVE_GAP**: Claim is morally serious, operational, and unmapped. **These are the load-bearing findings.** Each T-3 must name: (a) why existing namespace doesn't reach it, (b) what extension would close it, (c) whether the extension would survive the [§1.3.1](01_foundation.md) four-test gate.

## §12.4 Decision tree

1. **Paragraph TYPE?** Operational claim → continue. Pastoral/rhetorical → T-2. Theological/tradition-specific → T-1.
2. **Which family?** STANDING / ACTION / DETECTION / CONSENSUS / CORRECTION ([§12.1](#121-the-five-families-organizing-the-namespace)).
3. **Which specific prefix?** Scan [§5](05_namespace.md); check composition before reaching for new prefix.
4. **Fill the envelope** ([§4](04_envelope.md)).
5. **Compose only when needed.** Multi-primitive translations for paragraphs that genuinely name multiple structural objects.

A machine-readable namespace manifest (`FSD/CEG/dimensions.json`) is planned for CEG 1.0 — it will enable mechanical prefix lookup, polarity reading, and per-dimension aggregation defaults without human scanning of §5. (Originally targeted for 0.2 per the early-draft roadmap; the 0.3 → 0.8 wave prioritized substantive namespace additions over tooling, so the manifest now lands alongside the 1.0 lock when the namespace stabilizes.)

---

[← §11 Governance](11_governance.md) | **§12 Translation** | [Next: §13 Anti-patterns →](13_anti_patterns.md)

[← §1 Foundation](01_foundation.md) | **§2 Grammar** | [Next: §3 Primitives →](03_primitives.md)

---

# §2 The reasoning grammar — the eight axes

These are how a consumer reasons about an envelope. They are **not wire fields**; they are the canonical questions a consumer can ask about any Attestation.

| Axis | Question | Values |
|---|---|---|
| **Polarity** | Direction of the claim | Positive / Negative / Neutral / Indeterminate{reason} |
| **Object** | What the claim is about | key_id (entity) / attestation_id / contribution_id |
| **Time** | When is the claim valid | `asserted_at` + optional `valid_until`; consumer composes with substrate `expires_at` |
| **Epistemic mode** | How was the claim formed | direct / crypto / hearsay / derivative / appeal |
| **Reversibility** | Can the attestation be reversed | rollbackable / non-rollbackable (consumer policy + per-prefix rule) |
| **Stake** | What's backing the attester's claim | free / reputational / capital / cryptoeconomic |
| **Scope** | At what scale does the claim apply | self / family / community / affiliations / species / biosphere / federation |
| **Inter-attestation relations** | How does this attestation relate to others | standalone / refers-to / supersedes / withdraws / recants / corroborates / contradicts / clarifies (four of these are structural primitives per [§3](03_primitives.md); rest are emergent from scalar composition) |

`biosphere` added to Scope per CIRISRegistry#30 (distinct from `species` which is Homo sapiens cohort). See [§13](13_anti_patterns.md) for the `planet` colloquial alias note.

---

[← §1 Foundation](01_foundation.md) | **§2 Grammar** | [Next: §3 Primitives →](03_primitives.md)

[← §5 Namespace](05_namespace.md) | **§6 Relations** | [Next: §7 Reserved →](07_reserved.md)

---

# §6 Inter-attestation relations — the structural composition graph

Per [§2](02_grammar.md) Inter-attestation-relations axis, attestations relate to each other in eight ways. Four are structural primitives ([§3.2](03_primitives.md)); the other four are emergent from scalar composition:

| Relation | Realization |
|---|---|
| **Standalone** | Self-contained attestation; no `references_attestation_id`. |
| **Refers-to-prior** | Points to another attestation via `evidence_refs[]` or `context`; doesn't modify it. Emergent from independent positive scores on the same dimension+object. |
| **Supersedes-prior** | `supersedes` structural primitive ([§3.2](03_primitives.md)). |
| **Contradicts-prior** | Emergent from negative score on a dimension where a prior positive exists. |
| **Withdraws-prior** | `withdraws` structural primitive ([§3.2](03_primitives.md)). |
| **Recants-prior** | `recants` structural primitive ([§3.2](03_primitives.md)). |
| **Clarifies-prior** | Emergent from updated score with refined context on the same dimension+object. |
| **Delegated** | `delegates_to` structural primitive ([§3.2](03_primitives.md)). |

## §6.1 Concurrent-write precedence (0.1 scaffold)

> **0.1 SCAFFOLD NOTE**: The precedence rule below addresses the concurrent-write hole identified by CEG 0.1 distributed-systems review. Production deployments should treat as authoritative; 0.2 may refine the lexicographic tie-break per implementation feedback.

When two structural composers race on the same `references_attestation_id`, consumers MUST compute a deterministic verdict using the following precedence:

1. **`recants` outranks `withdraws` outranks `supersedes`** at the structural level. If the same attester emits multiple composers against the same prior attestation, `recants` wins regardless of `signed_at` (a falsity admission cannot be subsumed by a retraction or replacement).
2. **For same-type concurrent emissions by the same attester**: the composer with the largest `signed_at` per [§0.5](00_conformance.md) wins.
3. **For same-`signed_at` ties**: the composer whose attestation row's substrate-assigned key (Persist's `federation_attestations.attestation_id`) sorts lexicographically smallest wins.
4. **For cross-attester emissions on the same `references_attestation_id`**: each attester's chain is evaluated independently; the consumer sees N parallel chains and applies [§8](08_composition.md) policy.

Structural composers are **idempotent on `(references_attestation_id, attestation_type, attesting_key_id)`**: replaying the same composer is a no-op. The substrate MUST dedup on this triple.

---

[← §5 Namespace](05_namespace.md) | **§6 Relations** | [Next: §7 Reserved →](07_reserved.md)

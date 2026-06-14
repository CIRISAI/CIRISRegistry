[← §13 Anti-patterns](13_anti_patterns.md) | **§14 Glossaries** | [Next: §15 Gaps →](15_gaps.md)

---

# §14 Glossaries

## §14.0 Core terms (added 1.0-RC5; resolves [CIRISRegistry#77](https://github.com/CIRISAI/CIRISRegistry/issues/77))

These terms are referenced throughout the spec and across sibling repos. Defining them in-spec retires the external `ciris.ai/cewp` placeholder citations.

| Term | Definition |
|---|---|
| **CEG** (CIRIS Epistemic Grammar) | This specification. The federation's wire grammar: the "1+4" attestation model (`scores` plus the four relations `delegates_to` / `supersedes` / `withdraws` / `recants`), its namespaces, admission rules, and composition policies. CEG is the *grammar*; CEWP is the *network that speaks it*. |
| **CEWP** (CIRIS Epistemic Web Platform) | The decentralized network formed when nodes exchange CEG envelopes over [Edge](https://github.com/CIRISAI/CIRISEdge)/Reticulum transport. CEWP is **not a product, server, or central service** — it is the emergent peer-to-peer web of CEG-speaking nodes, exactly as "the Web" is the emergent network of HTTP-speaking servers. It has no owner, no root, and no load-bearing instance (the [§7.0.1](07_reserved.md) fabric-node discipline and the default-not-forced-root rule of [§5.6.8.10](05_namespace.md) guarantee this). A CEWP node is a **fabric node** ([CIRISServer](https://github.com/CIRISAI/CIRISServer)); `agent = fabric node + brain`. |
| **Fabric node** | A headless CEG/CEWP participant: it attests, stores, observes, reaches consensus, and transports, but does **not** reason or act (no brain). Shipped as CIRISServer. Three deployment shapes: standalone server, embedded-in-agent, or family member. See [§7.0.1](07_reserved.md). |
| **`ciris-canonical`** | The bootstrap governed community ([cohort_subkind: infrastructure](08_composition.md)) every node ships trusting by default — but which any consumer MAY untrust or re-root ([§5.6.8.10](05_namespace.md) default-**not**-forced-root). Its founding members (`lens` + `registry-us` + `registry-eu` fabric nodes) hold the founder-quorum (2-of-3, entrenched). Trust in it is **role-scoped and ≠ consent** ([§5.6.8.10](05_namespace.md)). |
| **NodeCode** | The QR-able peer-bootstrap shorthand for a federation key (`CIRIS-V1-…`, base32 + CRC-16). See [§0.10](00_conformance.md). |

## §14.1 Persist `system:*` leaf glossary (narrative → canonical)

Stories under [§5.3](05_namespace.md) sometimes use warm narrative leaves. The canonical wire form is to the right.

| Narrative | Canonical |
|---|---|
| `audit_chain:integrity` | `audit_chain:hash_continuity` |
| `corpus_health:free_disk_bytes` | `corpus_health:n_eff_measurable` |
| `identity_continuity:long_term_key` | `identity_continuity:relational_anchor` |
| `federation_directory:freshness_seconds` | `federation_directory:replication_lag` |

## §14.2 Edge `system:*` leaf glossary (narrative → canonical)

| Narrative | Canonical |
|---|---|
| `transport:tls_handshake_success_rate` | `transport:{kind}` (kind from Reticulum link types) |
| `delivery:retry_count_p99` | `delivery:{class}` (class from Reticulum delivery semantics) |
| `peer_reachability:{peer_id}` per-peer | `peer_reachability:{network}` (aggregate) |
| `key_boundary:{scope}` per-tenant | `key_boundary:{scope}` (scope from §3.4 D26 ext) |

## §14.3 Envelope-reach table (what the story wanted → how to express in existing wire)

| What stories wanted | How to express in CEG |
|---|---|
| introspection as `epistemic_mode` | `witness_relation: self` + low confidence + pending external |
| testimony as `epistemic_mode` | `epistemic_mode: external` + `witness_relation: external` |
| civic stake | `stake: reputational` + `cohort_scope: community` |
| epistemic stake | `confidence` + `stake: reputational` |
| dignitary stake | `harm_class:dignity_harm` (composition; not in stake axis) |
| oversight: deferred / active / advisory | HITL / HITL+monitoring / HOTL respectively |
| transparency:{kind} | `evidence_refs[]` of reasoning-chain hash + downstream `transparency_log:inclusion` |
| provenance_walk | consumer-side composition (Portal/Verify dashboards) |
| renamed capacity factors / HE-300 categories | canonical wire form + LANGUAGE_PRIMER glossary mapping |

## §14.4 Promotion via `supersedes` worked example

A NodeCore consumer maintains private notes in `local_data` Contributions at `cohort_scope: self`. The user decides to promote a private note to a published encyclopedia entry:

```
// Original (local_data, self scope):
{
  "attestation_type": "scores",
  "attesting_key_id": "user-alice-2026",
  "attested_key_id":  "user-alice-2026",
  "attestation_envelope": {
    "dimension": "encyclopedia:draft:notes",
    "score": 1.0,
    "confidence": 0.7,
    "evidence_refs": ["sha256:abc123..."],
    "cohort_scope": "self",
    "asserted_at": "2026-05-28T10:00:00.000Z"
  }
}

// Promoted (encyclopedia_article, global scope) via supersedes:
{
  "attestation_type": "supersedes",
  "attesting_key_id": "user-alice-2026",
  "attested_key_id":  "user-alice-2026",
  "attestation_envelope": {
    "references_attestation_id": "<prior-id>",
    "supersession_reason": "promote_to_published",
    "differs_in": ["cohort_scope", "sub_kind"],
    "new_dimension": "encyclopedia:article:notes",     // sub_kind morphed
    "new_score": 1.0,
    "new_confidence": 0.9,
    "new_evidence_refs": ["sha256:abc123..."],         // same content_sha256
    "new_cohort_scope": "global",                      // widened scope
    "asserted_at": "2026-05-28T15:00:00.000Z"
  }
}
```

Pattern recap per [§8.1.8.1](08_composition.md): widens `cohort_scope`, optionally morphs `sub_kind`, preserves `content_sha256` (no body re-upload), chains via `supersedes`. The promotion lineage is walkable via `references_attestation_id`.

---

[← §13 Anti-patterns](13_anti_patterns.md) | **§14 Glossaries** | [Next: §15 Gaps →](15_gaps.md)

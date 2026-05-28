[← §13 Anti-patterns](13_anti_patterns.md) | **§14 Glossaries** | [Next: §15 Gaps →](15_gaps.md)

---

# §14 Glossaries

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

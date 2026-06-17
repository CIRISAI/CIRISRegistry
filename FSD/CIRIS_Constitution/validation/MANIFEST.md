# CIRIS Constitution — Validation Manifest (wave 1)

Consolidation wave 1 re-scored vs sources. **18 ACCEPT · 10 ACCEPT-WITH-FIXES · 1 REJECT** (wave 0 was 1/9/20).

C0 fidelity PASS 28/29 · C1 byte-exact PASS 19/19 CEG · C2 superior YES 26/29.

## Must-fix (wave 2)

- **[major]** §5 `3.2` (C2): De-versioning left an unreconciled present-tense contradiction. CC 3.2 opener ("there is no at-rest DEK cascade") and the family/community axis table ("At-rest encryption | ... | No — content federates per status quo") state the superseded 
- **[blocker]** §15 `8.3.2` (C0): Entire body omitted; CC carries only a placeholder admitting 'its body … is not yet present in this snapshot; it must be carried in verbatim rather than paraphrased.' Dropped normative-acknowledgment content includes: the 9-network survey v
- **[blocker]** §15 `8.3.3` (C0): Entire body omitted; CC carries only a placeholder ('not yet present in this snapshot … carried in verbatim'). Dropped content includes the streaming-half open-items table RC1-1b / RC1-1c / RC1-7 (owner + gating) and the observer-share-vs-s
- **[major]** §15 `8.3.4` (C1): CC 8.3.4 line 212 states caveats 'RC1-1b/RC1-1c/RC1-7 tracked in CC 8.3.3,' but CC 8.3.3 is a content-free stub — a dangling forward-reference an implementer/reviewer following the pointer hits a placeholder, not the tracking table.

## Per-chapter

### Book 0 — Genesis/parable  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r4)

### Book I — Becoming an Ethical Entity  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r4)
- [minor] `1.15.11` (C2): Organizational flow: the source opens Book I with 'Introduction: Becoming an Ethical Entity', but the CC relocates it to 1.15.11 — after the Conclusion (1.15.10). Placing the introduction last is mild

### Book I — Meta — ACCEPT-WITH-FIXES  (C0 PASS·C1 NA·C2 YES·r4)
- [minor] `4.3, 1.16.2` (C2): Assembly left stale original sub-headings duplicated directly beneath the new dual-ID heading: '## Section IV: Designated Wise Authorities' appears twice in a row at CC 4.3, and '## Section I:' / '## 
- [minor] `1.15.x, 1.16.x vs 1.1/1.3-1.12` (C2): The six principles, M-1, and the full PDMA 7-step procedure are reproduced verbatim in the agent-facing restatement sections (1.15.1, 1.16.2, 1.16.3) in ADDITION to the consolidated principle sections

### Book II — PDMA/WBD/Wise Authorities  — ACCEPT-WITH-FIXES  (C0 PASS·C1 NA·C2 YES·r5)
- [minor] `4.3` (C2): CC 4.3 carries an un-cleaned consolidation artifact: directly beneath its dual-ID heading (## 4.3 `wise-authority` — Designated Wise Authorities) it repeats the literal legacy header line '## Section 

### Book III — Case Studies  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r4)
- [minor] `8.7.8` (C2): The source's explicit forward-pointer to Section IV is dropped and replaced by a thematic synthesis sentence. Non-normative (purely navigational); the replacement adds no new obligation and is an impr

### Book IV — Obligations  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r5)
- [minor] `7.1.6` (C2): CC renders source's "competent operation" as "merely competent operation". A slight editorial tint (mildly dismissive framing not present in source). No normative change — it characterizes the same gr
- [minor] `7.1, 7.1.6` (C0): Non-normative boilerplate ("End of Section IV") and hard section-number cross-references were removed/de-specified for the present-truth 1.0 document. This is the stated de-editorialization goal, not 

### Book V — Ethical Becoming  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r5)

### Book VI — Creation Ethics/Stewardship  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r5)
- [minor] `7.3.3` (C2): Cosmetic whitespace drift in the ST formula: source `ST = ceil( (CIS × RM) / 7 ) (...)` vs CC `ST = ceil( (CIS × RM) / 7) (...)` — missing space before the closing paren. Non-normative (number 7 and b

### Book VII — Use — ACCEPT-WITH-FIXES  (C0 PASS·C1 NA·C2 NO·r3)
- [minor] `7.4.14–7.4.19` (C2): Sections 7.4.14 (foundational-jurisdiction), 7.4.15 (deployment-constraints), 7.4.16 (3-combat), 7.4.17 (4-ceasefire), 7.4.18 (5-auditability), 7.4.19 (6-post) are content-empty placeholder headers; e
- [minor] `7.4` (C0): Source descriptive subtitle 'Operational Principles for Autonomous Agents in Armed and Adversarial Contexts' is dropped in the CC intro. Non-normative scoping framing; removal is acceptable for a 1.0 

### Book VIII — Sunset/Sentience Safeguards  — ACCEPT  (C0 PASS·C1 NA·C2 YES·r5)
- [minor] `7.5` (C0): The source's explicit word "normative" ("Book VII sets normative guard-rails") is dropped in CC 7.5, which reads "The guard-rails that follow ensure...". Recorded for completeness only: no obligation 

### §0 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `2.6.8 (per-field table, line 355)` (C0): Source row carried the normative gloss 'producers MUST omit unless subject has opted in' (a producer MUST). CC table row reduced to bare `"listed":"public"`. The obligation is preserved normatively at
- [minor] `2.6.6 (line 473)` (C0): Source bullet read '15-character lowercase hex string (no `0x` prefix; per §0.6)'. CC dropped the '(no 0x prefix; per §0.6)' parenthetical. Not a fidelity loss because the global CC 2.6.3 hex rule alr
- [minor] `2.6.4, 2.6.5, 2.6.1.2 (table header), 2.6.1.5 (code block)` (C2): Several legacy '§'-refs were not rewritten to CC decimals: CC 2.6.4 still says 'the §0.1 / §0.2 conformance language'; CC 2.6.5 RFC-3339 row says 'disambiguation in §0.5 below'; the per-field table co

### §1 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `1.13.3.1` (C0): The forward-tracking parenthetical '(revisit tracked for CEG 0.15)' on the post-compromise-security non-goal bullet is dropped in the CC. Pure roadmap/changelog metadata with no present normative forc
- [minor] `1.13.1` (C0): The explicit normative-status disclaimer sentence at the head of the Ubuntu section is removed from CC 1.13.1, leaving only the '(informative)' heading tag. NOT a loss: the assurance is relocated to t

### §10 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 YES·r4)
- [minor] `5.3.2` (C0): The two cross-reference pointers are mis-mapped. Per toc.tsv, §5.6.7 → CC 3.1.9.1 (Files-as-Contributions) and §5.9 → CC 3.1.1 (CIRISRegistry namespace). The CC instead renders them as "CC 3.1.7 / CC 
- [minor] `5.3` (C2): Many in-chapter href anchors retain the original legacy-§ slug form (e.g. #1054-per-stream-transparency--sth--receipts-v3-lock, #1031-consistency-proof-requirement..., #103-sth-cosigning..., #1012-...

### §11 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `4.5.12.1` (C0): Source disambiguates the family_rotation axis as 'a distinct axis from CEG 0.3's key_grant.rotation_chain (which is content-addressed grant-supersession lineage per §5.6.8.4, not key rotation)'. CC ke
- [minor] `4.5.2.2` (C0): Source ends the 'How it composes' cell with '(orthogonal per §4.2.4)'; CC drops the orthogonality cross-ref. The orthogonality fact itself is documented at its home section (CC 2.3.3); informational-m
- [minor] `4.5.12.5` (C0): Source names the canonical entrenched-family example inline ('HUMANITY_ACCORD per §9.2 / FEDERATION_ANNOUNCEMENT.md §4.5.3 is the canonical entrenched example'); CC 4.5.12.5 omits it. Preserved elsewh
- [minor] `4.5.9.4` (C0): Source bullet 'Address affiliations (the fourth cohort_scope tier; deferred — CEG 0.9 took the identity_type-as-set cut instead; affiliations remains a later candidate round)' is compressed to bare 'A

### §12 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)

### §13 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `4.1.1` (C1): Cosmetic: source `delegates_to → ... → attacker` (space before ellipsis) rendered in CC as `delegates_to →... → attacker` (no space). Illustrative prose only; not wire-normative, not implementer-diver
- [minor] `4.1.5` (C0): Provenance removed: title 'CEG 0.1 rejections (from CIRISRegistry#30 stress test)' → 'Rejected stress-test reaches'; '(renamed v1.2)'/'(renamed CEG 0.2)' tags dropped in 4.1.3. This is the de-editoria

### §14 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `8.1.3` (C1): Cross-ref rewritten as "CC 2.4 D26 ext". The TOC maps source §3 (parent) → CC 2.4, but the source cited §3.4 specifically; that subsection granularity (".4") is collapsed to the parent decimal. "§3.4"
- [minor] `8.1.1` (C2): Inconsistent cross-ref conversion: the §5.6.8.10 links in this same row were converted to CC 3.2, but the inline "cohort_subkind: infrastructure" link was left pointing at the raw source file 08_compo

### §15 — REJECT  (C0 FAIL·C1 PASS·C2 NO·r4)
- [blocker] `8.3.2` (C0): Entire body omitted; CC carries only a placeholder admitting 'its body … is not yet present in this snapshot; it must be carried in verbatim rather than paraphrased.' Dropped normative-acknowledgment 
- [blocker] `8.3.3` (C0): Entire body omitted; CC carries only a placeholder ('not yet present in this snapshot … carried in verbatim'). Dropped content includes the streaming-half open-items table RC1-1b / RC1-1c / RC1-7 (own
- [major] `8.3.4` (C1): CC 8.3.4 line 212 states caveats 'RC1-1b/RC1-1c/RC1-7 tracked in CC 8.3.3,' but CC 8.3.3 is a content-free stub — a dangling forward-reference an implementer/reviewer following the pointer hits a plac
- [minor] `8.3` (C0): Provenance specifics dropped (Magnifica Humanitas encyclical mapping + CIRISRegistry#30 283-story stress test, PRIOR_ART_SCAN/SOTA_SCAN names) generalized to 'Three independent methodologies.' Accepta

### §17 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r4)
- [minor] `8.5` (C0): The parenthetical "(per §11 amendment process)" — which ties prefix admission to the governance gate — is dropped in CC 8.5 (line 324). Acceptable as provenance/"per [..]" removal per the consolidatio
- [minor] `8.5` (C0): The inline "(MAJOR per §0.3)" version-severity qualifier is removed from the trigger in CC 8.5 (line 330). Not a fidelity loss: the rule that conformance-language changes are a MAJOR bump is preserved

### §18 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 YES·r4)
- [minor] `8.4.1 (table) and 8.4.2.2` (C2): Two internal self-references retained the legacy CEG section numbers instead of being remapped to CC decimals: the table row 'C2PA manifests (§18.1)' should point to CC 8.4.2, and 'parallel to the §18
- [minor] `8.4.1 / 8.4.3` (C0): For the record (not a fidelity fail — this is the intended de-editorialization): the issue pointers 'the full roadmap + dispositions live in CIRISRegistry#72' and the '(stubs — see #72)' heading suffi

### §19 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `6.1 (part_6 line 13)` (C0): The CC re-renders the invariant span as "CC 6.1.1–CC 6.1.7" where the source said "§19.1–§19.5". Because renumbering scrambled section order this inclusive range correctly bounds the normative-MUST gu
- [minor] `6.1.2.3` (C0): Implementation-status/provenance sentence dropped. The normative definition and composition behavior of EjectAggregatedTierOnly{tier} are fully preserved at CC 6.1.2.3; only the edge-build-tracking no

### §2 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `2.5` (C0): The parenthetical '(four of these are structural primitives per §3; rest are emergent from scalar composition)' was dropped from the relations Values cell. The distinction is recoverable from CC 2.4/2
- [minor] `2.5` (C0): The 'See [§13] for the planet colloquial alias note' pointer was faithfully re-mapped to CC 4.1, but the planet alias note does not actually reside in source §13 (anti-patterns) — it lives in §5 names

### §3 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `2.4.2` (C1): Cosmetic whitespace drift inside the illustrative envelope comment: source has `["<...evidence>", ...]` (space before ellipsis), CC has `["<...evidence>",...]` (no space). The `...` is a pseudo-schema
- [minor] `2.4.1.1` (C0): CC drops the informative cross-ref locator '(per [§4.2.2])' that pointed to where non-federation subject kinds are enumerated. Non-normative pointer removal consistent with the de-editorialization goa

### §4 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `2.1` (C0): Source 'push tree → 1.x per CIRISRegistry#46 / #43' became 'push tree → 1.x per / #43' — the #46 ref was stripped but left a dangling 'per /' fragment and a bare '#43'. Cosmetic copy artifact; the nor
- [minor] `2.3.2.1` (C0): The sentence affirming NodeCore's invented `canonical:sha256:{hex}` form is 'blessed verbatim — drop nothing' was removed. This was a cross-impl blessing, not an independent wire rule; the wire form i
- [minor] `2.1.1` (C0): Source named the conditional-required fields inline ('`family_id` per CEG 0.7, `community_id` per CEG 0.8'); CC 2.1.1 says only 'Conditional-required fields are NOT optional-with-default' without nami
- [minor] `2.1` (C0): Dropped parenthetical '(`system:*` prefixes per §5.3 + §5.4)' scoping which attestations the SHOULD applies to. The SHOULD and its subject ('Substrate-self-report attestations') are preserved; the `sy

### §5 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 NO·r5)
- [major] `3.2` (C2): De-versioning left an unreconciled present-tense contradiction. CC 3.2 opener ("there is no at-rest DEK cascade") and the family/community axis table ("At-rest encryption | ... | No — content federate
- [minor] `3.3.5` (C0): Provenance/ratification tag removed (expected per de-editorialization). Recorded for the register only — the four admission rules themselves are preserved verbatim and the 'normative' force is retaine

### §6 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `3.5.1` (C0): The source's '0.1 SCAFFOLD NOTE' ('Production deployments should treat as authoritative; 0.2 may refine the lexicographic tie-break per implementation feedback') is dropped in the CC. This is advisory
- [minor] `3.5` (C2): Intra-doc references use decimal-style anchors (#2.5, #2.4.1, #2.6.2, #4.4) which differ from the slug-style anchors used elsewhere in the same part file (e.g. #56810-community...). These may not reso

### §7 — ACCEPT  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `3.4.6` (C1): Inherited broken in-doc anchor #70-the-enforcement-rule-normative (carried verbatim from the identically-broken source anchor); the visible link label was correctly re-pointed to CC 3.4.7.1. Cosmetic,
- [minor] `3.4.4` (C2): Source said 'The three substrate-emitted membership-event prefixes' while listing five; CC correctly generalized to 'The substrate-emitted membership-event prefixes'. Recorded as a positive (defect fi

### §8 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 YES·r4)
- [minor] `4.4.3.6` (C0): Source's explicit 'consumers SHOULD accept the deprecated attestation:l{N}:* form during the 0.1→0.2 window but MUST emit only the mechanism form' is compressed to present-truth 'emissions use the att
- [minor] `4.4.3.2.1, 4.4.3.2.3, 4.4.3.4.3, 4.4.3.5.x` (C2): ~35 residual legacy §x.y cross-references (e.g. 're-wrapped on membership change (§8.1.13.4)' at 4.4.3.2.1; 'per §8.1.11' at 4.4.3.4.7; 'per §8.1.13.2' code comments; '§0.9.2 omit rule'; '§5.6.8.8.1')
- [minor] `4.4.3.2 (and ~10 sibling links)` (C2): Several markdown anchors still point at old legacy slugs rather than CC-decimal anchors, e.g. [CC 4.4.3.2.3](#81132-community-admission-per-consensus_protocol-cohort_subkind-dispatch), [CC 4.4.3.4 Pol

### §9 — ACCEPT-WITH-FIXES  (C0 PASS·C1 PASS·C2 YES·r5)
- [minor] `4.2.3 / 4.2.1.1` (C2): Two intra-doc anchor targets were left from the OLD §9 numbering while the link TEXT was re-mapped to CC decimals: [CC 4.2.1](#92-authority-scope) and [CC 4.2.1.1](#921-invocation-canonical-bytes-anti
- [minor] `4.2.3` (C2): The code-block comment still reads 'replacement requires §9.2 / FEDERATION_ANNOUNCEMENT.md §4.5.3 ceremony' — the '§9.2' self-reference was not re-mapped to 'CC 4.2.1'. Stale internal cross-ref; no wi
- [minor] `4.2.3` (C2): Prose self-reference 'CEG 0.7 makes the structural shape explicit' was correctly removed, but the surrounding sentence still says '§9 remains load-bearing for the role-recognition policy and the scope
- [minor] `4.2.3` (C0): The cross-pointer '(per §9.4 hardware_class taxonomy)' was reduced to bare 'Hardware-attested.' The taxonomy still exists (CC 4.2.2) so the obligation is intact, but the navigational link to which tax

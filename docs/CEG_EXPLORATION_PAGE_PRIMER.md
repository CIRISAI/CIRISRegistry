# CEG Exploration Page — Builder Primer

**Purpose**: design + content primer for building a public-facing page that lets visitors **explore the CIRIS Epistemic Grammar (CEG)** primitives organically — what they are, how they relate, what they can be used for. Companion document for whoever builds the page; not the page itself.

**Target deployment**: a section under `ciris.ai` (suggested path: `/grammar/` or `/ceg/`). Static-first recommended (the page is reference material; updates land via spec amendments, not real-time data).

**Audience for the page itself**: external developers + reviewers + curious technically-literate visitors. Not for first-time-CIRIS readers; assume they've at least read MISSION.md or ciris.ai/federation.

**Sources of truth** (authoritative; the page must stay in sync):
- [`FSD/CEG/`](../FSD/CEG/README.md) — wire-format spec (CEG 0.1 Public Working Draft, 18 files); the canonical namespace and structural primitives. **This is the authoritative spec as of 2026-05-28.** FSD-002 is preserved as design-history but is no longer authoritative.
- [`FSD/LANGUAGE_PRIMER.md`](../FSD/LANGUAGE_PRIMER.md) — translation grammar (how to write Contributions in CEG)
- [`FSD/PRIOR_ART_SCAN.md`](../FSD/PRIOR_ART_SCAN.md) + [`FSD/SOTA_SCAN.md`](../FSD/SOTA_SCAN.md) — comparative context
- [`MISSION.md`](../MISSION.md) — the federation's mission framing

**Last updated**: 2026-05-28 (CEG 0.1 release; sources-of-truth path moved from FSD-002 to FSD/CEG/; §0.5 fractal-self reading discipline added; §5.8 non-goal added).

---

## §0 What the page is teaching

CEG = **CIRIS Epistemic Grammar**. The federation's language for making structured, signed, machine-checkable claims about reality and each other.

**Four shape facts** a visitor should leave with:

1. **One workhorse + four structural composers** = the entire wire format. Everything else is open vocabulary on top.
2. **The vocabulary is mechanism-descriptive, not judgment-descriptive.** Prefixes describe *what is being measured*; polarity carries *the value claim*. (The operational-language gate per CEG §1.3.1.)
3. **Composition discipline > primitive proliferation.** New requirements are absorbed by composing existing primitives, not by adding atomic ones. The 1+4 minimal-and-adequate claim has now survived encyclical-level stress, internal G3 closure, and the files-as-Contributions extension without expanding the structural set.
4. **Self is self, fractally.** At every scale the federation operates on, "self" means the already-relationally-constituted entity speaking at that scale — never a Cartesian atomic ego asserting pre-relational identity. Self-attestation in CEG is the relational composition speaking AS itself, not a single principal pre-existing the relation. (See §0.5 below — this is the most common misread the page must preempt.)

If a visitor leaves the page with those four facts internalized, the page worked.

---

## §0.5 Reading discipline: how the page must frame "self"

This deserves its own subsection because it is the single most common misread a CEG visitor will make, especially one arriving from prior cryptographic-identity work (PGP / X.509 / DID atomic-principal frames).

The visitor will encounter `witness_relation: self`, `attesting_key_id == attested_key_id` self-references, the attester-supplied `confidence` field, env-var-asserted `occurrence_id`, attester-asserted `hardware_class`, the accord-holder self-attested `accord:lifecycle:active`, and the self-signed `federation_keys` rows. With Cartesian defaults imported, the visitor will think: *"a single atomic principal is asserting its own state; the substrate should require cross-attestation as a precondition before admitting this."*

That reading is wrong, but it is the trained-default reading. The page MUST preempt it.

**The correct frame the page should articulate prominently:**

> At every scale CEG operates on — key, occurrence, agent, fleet, cell, federation, biosphere — "self" means the *already-relationally-constituted entity speaking at that scale*. The cross-attestations that constituted the entity are upstream of the moment it speaks; when it then emits a self-attestation, that is the relational composition speaking AS itself, not a Cartesian atom asserting pre-relational identity.
>
> The substrate cannot require cross-attestation as a precondition for self-attestation because the cross-attestations are what constitute the self in the first place — they are upstream of the self-attestation, not downstream gates on it. The fractal nesting is the point.

**Page rendering recommendation**: a callout box on the landing page above the 1+4 primitives panel, plus a "Fractal self" badge on every primitive card whose default reading invites the Cartesian misread (`witness_relation: self`, `confidence`, `occurrence_id`, `hardware_class`, `accord:lifecycle:active`). The badge expands to the frame above.

**Page should NOT do** what an undisciplined reviewer would suggest:
- "Add a banner explaining the substrate has a Cartesian admission hole" — there is no such hole; the framing is the fix.
- "Document the trade-off between Cartesian wire and Ubuntu commitment" — there is no trade-off; the wire IS Ubuntu-fractal at every scale.
- "Mark self-attestation as a 'limitation' or 'gap'" — it is neither; it is the Ubuntu-fractal substrate operating as designed.

The fractal-self frame is also why the page can be honest about every envelope field WITHOUT defensive language: `confidence` is the relationally-constituted attester reporting its own state; `occurrence_id` is the fleet-constituted occurrence speaking; `hardware_class` is the key-bearing entity reporting provenance. Each is the relational composition speaking at its scale. The page renders them as the substrate operating as designed, not as compromises.

---

## §1 Organic navigation: design the page as a graph, not a tree

The user's instruction was "explore the primitives... in an organic manner." This rules out hierarchical TOCs. The page should have **multiple entry points** and **bidirectional navigation** so visitors can follow their own questions.

### Three orthogonal entry points

Pick what the visitor sees first based on what they're asking:

| Entry point | Question they're asking | Lands them on |
|---|---|---|
| **By family** | "What kinds of claims can the federation make?" | The five families (STANDING / ACTION / DETECTION / CONSENSUS / CORRECTION) — see LANGUAGE_PRIMER §2 |
| **By component** | "What does CIRIS[Component] own?" | The 8 owning components (Agent / Verify / Persist / Edge / LensCore / NodeCore / RATCHET / Bench / Registry) — FSD-002 §3.1-§3.9 |
| **By use case** | "How do I express *X* in CEG?" | Worked translation examples — LANGUAGE_PRIMER §11 + the page's own translation playground |

Recommend a top-level switcher: **`Families | Components | Use cases`** with the page state reflecting which lens is active.

### Bidirectional traversal between entry points

From a primitive's detail card, the visitor can:
- → **family** it belongs to (and see siblings)
- → **owning component** (and see the rest of that component's slice)
- → **use cases** that compose it
- → **prior-art kin** (the system in PRIOR_ART_SCAN closest in shape)
- → **related structural primitives** (e.g., from `scores` to `supersedes` to `recants`)

No "back to top" requirement. Every page is also an entry.

### Suggested visual idiom

Not strictly required, but proven shapes for this kind of content:

- **Card grid** for browse mode (primitives as cards with prefix + family color + one-line description)
- **Force-directed graph** for "show me how X composes" mode (primitives as nodes; composition edges; click a node to expand)
- **Worked-example playground** for use-case exploration (paste a paragraph, see suggested CEG translation)

Don't try to do all three at launch. The card grid is highest-utility; the graph view is high-wow but takes effort; the playground is genuinely valuable but might need an LLM backing.

---

## §2 Inventory: what to expose

### §2.1 — The 5 structural primitives (the "1+4")

This is the **most important** content. Every visitor needs to see this. Render as a prominent panel, not buried in a table.

| Primitive | What it does | When to use |
|---|---|---|
| **`scores`** | Workhorse. Scalar score (`f64` in `[-1, +1]`) + confidence + named dimension. Every substantive claim is a `scores` attestation. | Any claim about an entity, build, license, capability, behavior, state, commitment. |
| **`delegates_to`** | A authorizes B to sign on A's behalf within a bounded scope. | Authority-source claims (constitutional grounding), key rotation delegation, scoped capability handoff. |
| **`supersedes`** | This attestation replaces a prior one by the same attester. No falsity claim. | Doctrinal development; spec revisions; calibration version transitions. |
| **`withdraws`** | I retract my prior attestation. Does NOT claim it was false. | Context changed; prudent retraction without admission of error. |
| **`recants`** | My prior attestation was false at issuance. Admits epistemic error. | Genuine error admission. Distinguished from `withdraws` deliberately — PRIOR_ART_SCAN identified `recants` as a novel-to-distributed-trust primitive. |

Show the canonical-bytes shape for one of them (suggest `scores`) on the same panel so visitors see the wire reality.

### §2.2 — The 5 families (organizing the ~80 prefix families)

Renders well as **five cards with one analogy each** per LANGUAGE_PRIMER §2. Use the analogies — they're load-bearing pedagogy:

- **STANDING** — notarized professional credential record
- **ACTION** — research grant proposal (Goal → Approach → Method → Progress Measure)
- **DETECTION** — epidemiological surveillance
- **CONSENSUS** — peer review + jury deliberation
- **CORRECTION** — academic ethics committee + journal retraction + appellate review

Each card opens a list of prefix families in that family. Each prefix family card opens a detail view.

### §2.3 — The 8 envelope fields

These are subtle and matter: they're how consumer policy weights attestations. Render as a flat list with one-line descriptions:

| Field | Default | What it does |
|---|---|---|
| `polarity` | — (sign of score) | Direction of claim |
| `epistemic_mode` | `direct` | How the attester formed the claim (direct / crypto / hearsay / derivative / appeal) |
| `witness_relation` | `external` | Attester's relation to attested (self / external / derived) |
| `oversight_mode` | `null` | Human-control gradient (HITL / HOTL / HOOTL) |
| `stake` | `reputational` | What's backing the claim (free / reputational / capital / cryptoeconomic) |
| `scope` (cohort_scope) | — | Scale (self / family / community / affiliations / species / planet / federation) |
| `valid_until` | — | Optional expiry; staleness contract |
| `occurrence_id` / `_count` / `_role` | — | Multi-occurrence deployment discriminator |

Note in the page header: these are NOT wire primitives; they're consumer-reasoning axes per FSD-002 §1.1-§1.8.

### §2.4 — The 8 reasoning axes (consumer-side grammar)

Per FSD-002 §1.1-§1.8 — this is HOW consumers think about an envelope. Same eight cards, each with a question + the values:

- **Polarity** — Direction of the claim?
- **Object** — What is the claim about? (key_id / attestation_id / contribution_id)
- **Time** — When is the claim valid?
- **Epistemic mode** — How was the claim formed?
- **Reversibility** — Can it be reversed?
- **Stake** — What's backing the claim?
- **Scope** — At what scale does the claim apply?
- **Inter-attestation relations** — How does this attestation relate to others?

Cross-link to the relevant envelope fields where they line up (Polarity ↔ score sign; Time ↔ valid_until; etc.).

### §2.5 — The ~80 prefix families (organized by owning component)

This is the **deepest content**. The page is going to want to defer this to a "browse all primitives" view that's expandable rather than dumped on the homepage. Organized per FSD-002 §3.1-§3.9:

- **CIRISAgent** — 6 Accord principles + 4 DMA verdicts + 4 conscience verdicts + 22 prohibited categories
- **CIRISVerify** — Attestation ladder L1-L5 + provenance + transparency log + cert validity + hardware custody
- **CIRISPersist** — substrate self-reports (system:* reserved)
- **CIRISEdge** — transport / delivery / peer reachability / key boundary (system:* reserved)
- **CIRISLensCore** — Coherence Ratchet detectors + Capacity Score factors + correlated-action detector + distributive-access detector
- **CIRISNodeCore** — Tier 1-4 (agent state + decision hierarchy + consensus + governance) + locality + need + testimonial witness
- **RATCHET** — anti-Sybil flags (advisory, never sole evidence)
- **CIRISBench** — HE-300 benchmark outcomes
- **CIRISRegistry** — identity / build / license / partner + agent_files + accord (reserved)

Each component card has a count + a "show all" expander. Expanded view shows the prefix table for that component (mirror FSD-002 §3.x tables).

### §2.6 — Composition policies (§6)

Often overlooked but important: visitors should see that consumer-side composition is part of CEG, not bolted on. Render as five named policies (A / B / C / D / E / F where applicable):

- **A — Direct trust** (pinned attesters)
- **B — One-hop transitive**
- **C — Weighted graph (EigenTrust-style)**
- **D — Lexical-vulnerability-priority** (tie-breaking modifier)
- **E — Locality-scaled-quorum** (quorum-sizing modifier; closes G3)
- **F — `agent_files` trust composition** (three-layer canonical / open / vote-then-trust)

Note the structural split: A/B/C are base policies; D/E/F are modifiers/specializations.

---

## §3 Per-primitive card content

Every prefix family + structural primitive + envelope field gets its own page. Suggested card layout:

```
┌─────────────────────────────────────────────────────────────┐
│  PRIMITIVE NAME                          [Family · Owner]   │
│  ─────────────────────────────────────────────────────────  │
│  One-line plain-language description.                       │
│                                                              │
│  Wire shape                                                  │
│    [code block — minimum canonical example]                  │
│                                                              │
│  Polarity        | signed / boolean-via-score / positive-only│
│  Reserved?       | yes (only X may emit) / no                │
│  Composed with   | [linked list of primitives]               │
│  Closes / uses   | [linked GitHub issues / FSD sections]     │
│                                                              │
│  Worked translation example                                  │
│    Source paragraph: "..."                                   │
│    CEG translation:                                          │
│      [YAML envelope]                                         │
│                                                              │
│  See also                                                    │
│    [Family siblings] [Owning component] [Prior-art kin]      │
└─────────────────────────────────────────────────────────────┘
```

For prefix families with sub-vocabularies (e.g., `detection:correlated_action:{axis}` has multiple axes), the card lists the canonical axes + a note that the vocabulary is open-extensible via §4.9.2 amendment.

---

## §4 Recommended use-case stories (page narrative)

These are the "what can I do with CEG" answers. Recommend 5-8 worked stories visitors can click through to see CEG in action. Each story:
1. States a real-world question or task
2. Names the primitives it uses
3. Walks through the translation as a small annotated YAML

Suggested stories:

### Story 1 — "A licensed medical clinic registers a partner agent"
Shows: `partner_role:PROFESSIONAL_MEDICAL` + `licensure:CA_medical_board` + `bond_posted:USD` + (later) `revocation:partner:license_lapsed`. Composition over multiple primitives; consumer-side verdict via §6.1 Policy A.

### Story 2 — "A correlated-action pattern emerges in agent behavior"
Shows: `detection:correlated_action:rights_asymmetry:hiring_pipeline_v2` emitted by LensCore. Walks through how the score gets composed into a `moderation:coordinated_voting` ModerationEvent (NEVER sole evidence for `slashing:*`).

### Story 3 — "Doctrinal development supersedes a prior version"
Shows: `supersedes` structural primitive with `differs_in: ["scope", "evidence_refs"]`. Demonstrates the difference between `supersedes` (no falsity claim) and `recants` (admission of error).

### Story 4 — "Bootstrap content from the encyclical lands"
Shows: the bootstrap-contributions pattern (§10.4). A paragraph from *Magnifica Humanitas* translated into CEG; the 1-of-6 accord/steward sign-off discipline (§4.9.2 step 5); the §10.4.3 first-deployment posture and §10.4.4 multi-source commitment.

### Story 5 — "A constitutional halt is invoked"
Shows: `accord:invoke:CONSTITUTIONAL:{halt_id}` requires 2-of-3 HUMANITY_ACCORD multi-sig. Distinguishes from operational SYSTEM_ADMIN authority. References the §4.9.2 step 5 + the SetEmergencyShutdown admin RPC rejection behavior.

### Story 6 — "An installer file gets the canonical-bootstrap trust path"
Shows: `agent_files:installer:linux-x86_64` from registry-steward = canonical-default. The §6.1.6 three-layer trust composition. The anti-tricking guarantee at `/install`. Third-party `agent_files:*` reachable only via "Browse alternatives" explicit consent.

### Story 7 — "An affected party's testimony is preserved"
Shows: `testimonial_witness:displaced_worker` distinct from `witness_diversity:*`. Singular narrative preservation; never aggregated; never sole evidence for `slashing:*`. Composition with `non_maleficence:*` from external advocates per LANGUAGE_PRIMER §11.14.

### Story 8 — "A locale-targeted attack is detected"
Shows: per-locale `provenance:build_manifest:ios-mobile-bundle:locale:my` (Burmese sub-manifest) detection via Merkle composition per §3.2.1.2. Demonstrates the RFC 6962-style hashing + 29-locale padding.

---

## §5 Information that must NOT be on the page

This is as important as what to include. **Explicit non-goals** prevent the page from drifting into spec-restatement or marketing fluff:

1. **No claim that CEG "solves" anything.** It's a substrate. Consumers compose verdicts. The page's voice is descriptive, not promotional.
2. **No theological framing.** The encyclical bootstrap is named as ONE source (per §10.4.4 multi-source commitment); the framework is tradition-multiplicity-neutral per §1.10.1 operational-language discipline. Don't render the encyclical as the framework's authority.
3. **No "trust scores" without context.** Score values are mechanism-magnitude, NOT moral verdicts. Visitors must see that polarity + axis + cohort context jointly produce the value claim.
4. **No invented categories.** The four-verdict strict list (clean / composed / partial / not-translated) is verbatim from LANGUAGE_PRIMER §9; don't add "verbatim" / "strong" / "weak" / "STRONG_ALIGN" intermediates.
5. **No marketing copy in primitive cards.** Mechanism description only. Use-case stories (§4 above) carry the "why this is useful" narrative; primitive cards stay technical.
6. **No comparison "we're better than" prior art.** The PRIOR_ART_SCAN page (linked separately) describes what CEG inherits + what's genuinely novel; the primitive-exploration page links there but doesn't editorialize.
7. **No claim of completeness.** The namespace is open-extensible. Make it visible that prefix families can be added via the CEG §11.2 amendment process; render the current namespace as "as of CEG 0.1" with a link to [`FSD/CEG/`](../FSD/CEG/README.md) for the live state.

8. **No rendering of self-attestation as a Cartesian admission gap.** Per §0.5 above: self-attestation is the relational composition speaking AS itself at the scale of the speaker; it is not a pre-relational atomic assertion the substrate should gate. Reviewers who suggest "the substrate should add cross-attestation gates before admitting self-attestations" are importing Cartesian defaults; the page MUST NOT echo that framing as if it were a substrate concession. The fractal-self frame is the correct read, and the page should render every self-attestation surface (`witness_relation: self`, attester-supplied `confidence`, `occurrence_id`, `hardware_class`, `accord:lifecycle:active`, self-signed `federation_keys`) as the Ubuntu-fractal substrate operating as designed.

---

## §6 Keeping the page in sync (operational discipline)

The wire format moves. The page must move with it. Three mechanisms:

### §6.1 — Auto-regen from CEG §5 namespace tables

The §5.1-§5.9 prefix tables in [`FSD/CEG/05_namespace.md`](../FSD/CEG/05_namespace.md) are markdown tables with stable column shapes (Prefix | Description | Polarity, plus Reserved? for §5.9). A small script can parse these into JSON and feed the page's primitive-card content. Recommend:

```
docs/scripts/extract_namespace.py  →  data/primitives.json
```

Run on every CEG commit (CI hook). Page consumes `primitives.json` at build time; primitive cards re-render automatically. CEG 0.2 commits to publishing a machine-readable [`FSD/CEG/dimensions.json`](../FSD/CEG/) manifest at the source — when that lands, the extract script becomes a passthrough.

### §6.2 — Spec version watermark

Every page footer carries: **"Reflecting CEG {X.Y.Z} (commit {SHA short})"** with a link to the exact commit. When the spec moves, the watermark moves with the next page regen. Visitors can always trace back to the authoritative source.

### §6.3 — Worked examples reviewed at each minor version

The §4 use-case stories are hand-written narrative. When CEG lands a minor-version change (0.2, 0.3...), review the stories for any primitives that got renamed (e.g., FSD-002 v1.2's `emergent_deception` → `correlated_action` rename would have broken any story referencing the old name).

Recommend a `STORIES_REVIEWED_AT.md` ledger in the page repo listing each story + the CEG version it was last validated against.

---

## §7 Recommended technical stack

Not prescriptive; for whoever builds the page:

- **Static-site generator**: Astro, Eleventy, or just plain Next.js with static export. The page is reference material; SSR not needed.
- **Search**: client-side index (Pagefind, Lunr) over the primitive cards. Spec content is small enough to ship the whole index.
- **Graph view (if built)**: D3 force-directed layout or Cytoscape.js. Render composition edges from a small `composition_edges.json` (manual at first, auto-derivable from the `Composed with` field per-card).
- **Translation playground (if built)**: optional LLM backing (Claude or similar) to suggest CEG translations of pasted text. Per LANGUAGE_PRIMER §6 four-test gate, the LLM output should be flagged as "draft translation, verify against §11 worked examples" — don't render LLM output as authoritative.

---

## §8 Sequencing

If the page is being built incrementally:

**Phase 1 (MVP)**:
- The 1+4 structural primitives panel (§2.1)
- The five families with analogies (§2.2)
- A flat browseable list of prefix families per component (§2.5)
- Spec version watermark + commit link
- 2-3 worked use-case stories (Story 1 + Story 2 + Story 5 cover most of the moral surface)

**Phase 2**:
- Per-primitive detail cards with worked examples
- The 8 envelope fields (§2.3) + 8 axes (§2.4) panels
- Composition policies panel (§2.6)
- 4-5 additional stories

**Phase 3**:
- Force-directed composition graph
- Translation playground
- Auto-regen tooling per §6.1

Phase 1 alone discharges the user-facing ask. Phases 2-3 are quality-of-life additions.

---

## §9 References

- **CIRIS Epistemic Grammar (CEG)** spec: [`FSD/CEG/README.md`](../FSD/CEG/README.md) — 18-file spec under `FSD/CEG/`. The README's "How to read this spec without Cartesian default" callout is the canonical source for the §0.5 fractal-self frame; the page should echo that framing verbatim or near-verbatim.
- **Translation grammar** (for writing Contributions in CEG): [`FSD/LANGUAGE_PRIMER.md`](../FSD/LANGUAGE_PRIMER.md)
- **Comparative context**: [`FSD/PRIOR_ART_SCAN.md`](../FSD/PRIOR_ART_SCAN.md) + [`FSD/SOTA_SCAN.md`](../FSD/SOTA_SCAN.md)
- **Mission framing**: [`MISSION.md`](../MISSION.md)
- **Safety-vs-censorship discipline**: [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/) — the operational-language gate's source
- **Trust contract** (consumer-facing): [`docs/TRUST_CONTRACT.md`](TRUST_CONTRACT.md)
- **FSD-002 (design-history)**: [`FSD/FSD-002_FEDERATION_SURFACE.md`](../FSD/FSD-002_FEDERATION_SURFACE.md) — preserved for lineage; superseded by CEG/. Do NOT link to FSD-002 as the authoritative spec from the public page; link to CEG/.

---

**End CEG_EXPLORATION_PAGE_PRIMER.md**

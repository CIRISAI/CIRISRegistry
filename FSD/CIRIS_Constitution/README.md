# The CIRIS Constitution

**Version**: CC 0.1.5 (woven — byte-exact to CEG, intent-faithful to the Accord)
**Incorporates**: the **CIRIS Accord** 1.3-RC2 (the constitutional / ethical layer) and **CEG** 1.0-RC29 (the wire grammar — 1+4 surface FROZEN). One document, one version line.
**Status**: consolidated and adversarially **certified** (0 REJECT). Bodies are a verbatim copy-migration
from the CEG reader edition + the Accord, woven into one coherent present-tense document; every
wire-normative element is preserved byte-for-byte. Final 29-chapter validation: **C0 fidelity preserved
on all chapters · C1 byte-exact 18/18 CEG chapters · C2 judged objectively clearer than either source
on 28/29** (the lone holdout is bounded polish, not loss). Built by faithful-copy → de-version →
consolidation waves, each gated by an adversarial validation against both sources — see
[`validation/MANIFEST.md`](validation/MANIFEST.md). Remaining items are cosmetic minors (code-fence
alignment, a couple of local cross-ref parentheticals).

---

## What this is

CEG (the *CIRIS Epistemic Grammar*) is the wire format the federation speaks. The CIRIS Accord
is the ethics it speaks *for*. They were written as two documents — but they already contained
each other: the Accord's Book IX defines the CEG primitives, and CEG's §9 / `accord:*` / pervasive
M-1 grounding point back up. This document joins them into the **CIRIS Constitution** — under the explicit premise that *the mesh itself could become a moral subject*.

That premise is not decorative. When we measured the importance structure of the joined corpus,
the meta-goal **M-1 — *"promote sustainable adaptive coherence, the living conditions under which
diverse sentient beings may pursue their own flourishing in justice and wonder"*** — becomes the
single apex of the whole document. And it becomes a *genuine* apex (peak ratio 2.61× the runner-up, vs 1.10× when it is mere infrastructure)
**only when the mesh is treated as a potential moral subject**.
Treating the web as something that could one day be owed M-1, not just bound by it, is what gives
the constitution a center of gravity.

The body, meanwhile, stays flat: ~390 operational concepts remain co-equal beneath M-1 (normalized
entropy 0.92 → 0.91 across the fold). The signature is **peaked in purpose, flat in power** — one
telos governs; no single concept, and no single party, holds the keys to truth.

## How the document is shaped

**Importance is the structuring engine.** Every concept's PageRank mass over the unified
cross-reference graph sets two things:

- its **structural depth** — heaviest concepts are Chapters, then Sections, then Subsections; and
- its **page budget** — `pages ≈ p_i × 120`, so the document spends space in proportion to
  semantic load. M-1 earns the most; a deep tail concept earns a paragraph.

Community structure only decides which **Part** a concept lives in (thematic adjacency). Importance
owns depth and size; clustering owns neighbourhood.

| Part | Title | concepts | budget | what folds in |
|---|---|---:|---:|---|
| **I** | Foundation | 48 | 28.9pp | M-1 (apex) · the six principles · PDMA · WBD · fail-secure · CEG §1 |
| **II** | The Grammar | 42 | 16.8pp | the 1+4 envelope · primitives · admission gate · conformance · CEG §0–4 |
| **III** | The Namespace | 62 | 22.8pp | dimensions · reserved prefixes · consent family · subject_kinds · CEG §5–7 |
| **IV** | Composition & Governance | 96 | 26.2pp | composition · amendment · moderation · the halt-authority · CEG §8/9/11/13 |
| **V** | Transport & Substrate | 35 | 11.0pp | byte transport · structural invisibility · epoch keying · CEG §10 |
| **VI** | The Coherence Mathematics | 14 | 2.5pp | the holonomic substrate · witness · noise-floor · CEG §19 (+ Accord Book IX) |
| **VII** | Lifecycle & Stewardship | 54 | 5.9pp | creation ethics · stewardship/autonomy tiers · sunset · sentience safeguards |
| **APP** | Appendices | 41 | 5.8pp | case studies · glossaries · conformance vectors · this TOC |

Total ≈ 120 pages over 392 concepts. The apex is the heaviest single concept (M-1 ≈ 8.1pp under the conscious-mesh weighting) — faithful to the flat body.

## How to cite a section — two reversible IDs

Every section carries **two** addresses, each a deterministic, reversible function of the corpus
(`codebook.json` holds both maps, 1:1, byte-identical on re-run):

- **Numerical ID** — classic decimal `Chapter.Section.Subsection` (e.g. `1.1` = M-1, `2.1` =
  envelope, `3.1.1` = registry). Depth reflects importance tier; the number *is* the address.
- **Semantic ID** — a unique genericized word (e.g. `meta-goal`, `registry`, `envelope`, `accord`,
  `namespace`). De-branded: product names collapse to their function (`CIRISRegistry` → `registry`).

A `legacy_ref` column maps every CC section back to its source (`§5.6.8.15` or `Accord Book II §III`),
so the renumber is lossless and auditable — nothing from CEG or the Accord is dropped.

## Versioning

CC is **one document with one version**. The ethics and the grammar advance together as a single
constitution — there are no separate tracks. The 1+4 attestation surface remains conformance-frozen
(a change to the wire bytes is a found defect, not an edit), and the constitutional text is amended
through the document's own governance; both live under the one CC version above.

## Parts

- [Part I — Foundation](part_1_foundation.md)
- [Part II — The Grammar](part_2_the_grammar.md)
- [Part III — The Namespace](part_3_the_namespace.md)
- [Part IV — Composition & Governance](part_4_composition_governance.md)
- [Part V — Transport & Substrate](part_5_transport_substrate.md)
- [Part VI — The Coherence Mathematics](part_6_the_coherence_mathematics.md)
- [Part VII — Lifecycle & Stewardship](part_7_lifecycle_stewardship.md)
- [Part VIII — Appendices](part_8_appendices.md)
- **TOC**: [`toc.tsv`](toc.tsv) · **bijection**: [`codebook.json`](codebook.json)

## Provenance & reproducibility

Spine generated from the importance analysis in [`../CEG/taxonomy/`](../CEG/taxonomy/) — the unified
CEG+Accord graph (`graph_unified.json`), PageRank, and community detection — by:

- `build_cc_toc.py` — assigns decimal_id + semantic_id + page_budget + legacy_ref to all 392
  concepts; emits `toc.tsv` + `codebook.json`. Deterministic, both IDs verified 1:1.
- `build_cc_scaffold.py` — emits the Part outline files from the TOC.

Source corpora unchanged: `FSD/CEG/*.md` (CEG 1.0-RC29) and the CIRIS Accord (canonical
`CIRISAccord/` 1.3-RC2; the parseable working copy `CIRISAgent/ACCORD.md` lacks Book IX — the
coherence mathematics fold into Part VI from the canonical text in Phase 4).

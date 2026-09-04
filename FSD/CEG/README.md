# The CEG has moved — see CIRISAI/CIRISConstitution

The CIRIS Ethical Grammar is no longer maintained in this repository. It has been
absorbed into **the CIRIS Constitution**, which is the single source of truth:

> **https://github.com/CIRISAI/CIRISConstitution** — currently **1.0-rc4**

Consumers vendor and pin from that repo. Nothing in this directory is normative.

## Why this stub exists rather than nothing

Seven sibling repos (Server, Persist, Edge, Verify, Agent, Conformance, NodeCore)
carry ~28 links into `CIRISRegistry/FSD/CEG/*.md`. This file is the signpost those
links land on until they are repointed. It is not a mirror and will not be updated
with spec content.

## Versioning — read this before citing a number

The constitution is at **1.0-rc4**. The old `1.0-RC29` line that this directory
used to declare was **discontinued at the re-home** — it is not a later revision
than RC3 and must not be cited as one. Any code or doc still carrying an RC2x
number, or the older `0.x` line, is on dead lineage.

## Where each section went

The 20 CEG sections were reorganised into the constitution's 8 Parts. Section
numbering did **not** survive the move — cite the Part anchor, not the old `§N`.

| was (`FSD/CEG/…`) | now (`constitution/…`) |
|---|---|
| `00_conformance.md` | part_2 §2.2 `conformance` |
| `01_foundation.md` | part_1 §1.1–1.13 |
| `02_grammar.md` | part_2 §2.5 `reasoning` — the eight axes |
| `03_primitives.md` | part_2 §2.4 `primitive` — the 1+4 set |
| `04_envelope.md` | part_2 §2.1 `envelope`, §2.3 `subject_keys` |
| `05_namespace.md` | part_3 §3.1 `namespace`, §3.2 `community`, §3.3 `content-ingestion` |
| `06_relations.md` | part_3 §3.5 `structure-inter` |
| `07_reserved.md` | part_3 §3.4 `reservation` |
| `08_composition.md` | part_4 §4.4 `composition-policies` |
| `09_humanity_accord.md` | part_4 §4.2 `accord`, §4.3 `wise-authority` |
| `10_endpoints.md` | part_5 §5.3 `endpoint` |
| `11_governance.md` | part_4 §4.5 `discipline` |
| `12_translation.md` | part_8 §8.2 `translation` |
| `13_anti_patterns.md` | part_4 §4.1 `anti-pattern` |
| `14_glossaries.md` | part_8 §8.1 `glossary` |
| `15_gaps.md` | part_8 §8.3 `concerns` |
| `16_references.md` | part_8 §8.6 `references-lineage` |
| `17_cadence.md` | part_8 §8.5 `update` |
| `18_interop.md` | part_8 §8.4 `interoperability` |
| `19_holonomic.md` | part_6 §6.1 `holonomic` |

The `taxonomy/`, `pdf/` and `reader-md/` build outputs were not carried over; the
constitution builds its own PDF (`build_pdf.py`) and carries `manifests/` —
including `WIRE_VOCABULARY.md`, which also left this repo.

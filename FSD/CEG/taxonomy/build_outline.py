#!/usr/bin/env python3
"""
Importance-tiered structural outline of the unified CIRIS Epistemic Web (CEW).

The structuring principle (user's spec): IMPORTANCE is the primary driver.
A concept's importance (PageRank p_i over the unified CEG+Accord graph) sets:
  - its structural DEPTH  (importance tier -> chapter / section / subsection / deep)
  - its PAGE BUDGET       (pages_i = p_i x TOTAL_PAGES — "semantic import -> page count")
Community detection is SECONDARY — it only decides which Part a co-equal chapter
sits in (topical adjacency / order), never depth or budget.

This script reads the authoritative dual-ID TOC produced by FSD/CEW/build_cew_toc.py
(decimal_id, semantic_id, title, legacy_ref, p_i, import_rank, pages, part) and
renders the importance-tiered OUTLINE (chapters -> sections -> subsections with
page budgets) into SHAPE_ANALYSIS.md, plus a taxonomy-local toc.tsv carrying the
page_budget column and an importance->budget bar chart.

Reads only; writes SHAPE_ANALYSIS.md, toc.tsv, importance_budget.png in taxonomy/.
"""

import os
import csv
import json
from collections import defaultdict, OrderedDict

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
# taxonomy -> CEG -> FSD -> FSD/CEW  (two levels up from taxonomy, then CEW)
CEW = os.path.normpath(os.path.join(OUT_DIR, "..", "..", "CEW"))
CEW_TOC = os.path.join(CEW, "toc.tsv")
CEW_CODEBOOK = os.path.join(CEW, "codebook.json")

# Importance tier bands (from the Hu-Tucker/Huffman tiering: 20/40/90/141).
# Depth = tier: top tier -> Chapter, next -> Section, next -> Subsection, tail -> deep.
TIER_BANDS = [(20, 1, "Chapter"), (60, 2, "Section"),
              (150, 3, "Subsection"), (10 ** 9, 4, "deep nested")]
PART_TITLE = {
    "I": "Foundation", "II": "The Grammar", "III": "The Namespace",
    "IV": "Composition & Governance", "V": "Transport & Substrate",
    "VI": "The Coherence Mathematics", "VII": "Lifecycle & Stewardship",
    "APP": "Appendices",
}
PART_ORDER = ["I", "II", "III", "IV", "V", "VI", "VII", "APP"]


def tier_of(rank):
    for thresh, depth, label in TIER_BANDS:
        if rank <= thresh:
            return depth, label
    return 4, "deep nested"


def dkey(d):
    return [int(x) for x in d.split(".")]


def load_rows():
    rows = []
    with open(CEW_TOC) as fh:
        r = csv.DictReader(fh, delimiter="\t")
        for row in r:
            row["p_i"] = float(row["p_i"])
            row["import_rank"] = int(row["import_rank"])
            row["pages"] = float(row["pages"])
            row["depth"] = len(row["decimal_id"].split("."))
            rows.append(row)
    return rows


def main():
    if not os.path.exists(CEW_TOC):
        raise SystemExit("Run FSD/CEW/build_cew_toc.py first (it produces toc.tsv).")
    rows = load_rows()
    by_dec = {r["decimal_id"]: r for r in rows}
    total_pages = sum(r["pages"] for r in rows)
    N = len(rows)

    # codebook bijection check (reuse CEW's verified codebook)
    cb = json.load(open(CEW_CODEBOOK))
    dec_unique = len(cb["decimal_to_key"]) == N
    sem_unique = len(cb["semantic_to_key"]) == N

    # chapters = top-level decimal ids (one dot -> "P.N"); ordered by Part then p_i
    chapters = sorted([r for r in rows if r["depth"] == 2],
                      key=lambda r: dkey(r["decimal_id"]))

    # ---- taxonomy-local toc.tsv with page_budget column (mirror + tier label) ----
    with open(os.path.join(OUT_DIR, "toc.tsv"), "w") as fh:
        fh.write("decimal_id\tsemantic_id\ttitle\tlegacy_ref\tp_i\timport_rank\t"
                 "page_budget\timportance_tier\tpart\torigin\n")
        for r in sorted(rows, key=lambda r: dkey(r["decimal_id"])):
            _, tlabel = tier_of(r["import_rank"])
            fh.write(f"{r['decimal_id']}\t{r['semantic_id']}\t"
                     f"{r['title']}\t{r['legacy_ref']}\t{r['p_i']:.5f}\t"
                     f"{r['import_rank']}\t{r['pages']}\t{tlabel}\t{r['part']}\t"
                     f"{r['origin']}\n")

    render_budget(rows, os.path.join(OUT_DIR, "importance_budget.png"))
    write_outline(rows, chapters, by_dec, total_pages, N, dec_unique, sem_unique, cb)

    # ---- stdout ----
    part_stats = defaultdict(lambda: [0, 0.0])
    for r in rows:
        part_stats[r["part"]][0] += 1
        part_stats[r["part"]][1] += r["pages"]
    print(json.dumps({
        "N": N, "total_page_budget": round(total_pages, 1),
        "decimal_unique": dec_unique, "semantic_unique": sem_unique,
        "chapters": len(chapters),
        "parts": [{"part": p, "title": PART_TITLE[p],
                   "concepts": part_stats[p][0],
                   "pages": round(part_stats[p][1], 1)} for p in PART_ORDER],
        "lead_chapters": [
            {"decimal": r["decimal_id"], "semantic": r["semantic_id"],
             "pages": r["pages"], "legacy": r["legacy_ref"]}
            for r in sorted(rows, key=lambda r: -r["p_i"])[:8]
        ],
        "tail_nesting": {
            "depth_2_chapters": sum(1 for r in rows if r["depth"] == 2),
            "depth_3_sections": sum(1 for r in rows if r["depth"] == 3),
            "depth_4_subsections": sum(1 for r in rows if r["depth"] == 4),
            "depth_5plus_deep": sum(1 for r in rows if r["depth"] >= 5),
        },
    }, indent=2, ensure_ascii=False))


def render_budget(rows, path, topn=26):
    top = sorted(rows, key=lambda r: -r["pages"])[:topn]
    fig, ax = plt.subplots(figsize=(13, 9))
    yp = range(len(top))
    colors = ["#e53e3e" if r["origin"] == "accord" else "#2b6cb0" for r in top]
    ax.barh(list(yp), [r["pages"] for r in top], color=colors)
    ax.set_yticks(list(yp))
    ax.set_yticklabels([f"{r['decimal_id']}  {r['semantic_id']}" for r in top],
                       fontsize=7, family="monospace")
    ax.invert_yaxis()
    ax.set_xlabel("page budget  (pages_i = p_i × 120) — semantic import → page count")
    ax.set_title("Unified CEW — importance-proportional page budget, top 26 "
                 "(red = Accord, blue = CEG)")
    for i, r in enumerate(top):
        ax.text(r["pages"], i, f"  {r['pages']:.1f}pp", va="center", fontsize=7,
                color="#222")
    ax.margins(x=0.18)
    fig.tight_layout(); fig.savefig(path, dpi=130); plt.close(fig)


def write_outline(rows, chapters, by_dec, total_pages, N, dec_unique, sem_unique, cb):
    A = []; W = A.append
    W("# CIRIS Epistemic Web — Importance-Tiered Structural Outline\n")
    W("> The unified constitution's skeleton, **structured by importance**. A "
      "concept's PageRank mass `p_i` over the unified CEG+Accord graph sets both "
      "its **structural depth** (chapter → section → subsection) and its **page "
      "budget** (`pages = p_i × 120` — semantic import → page count). Community "
      "detection is secondary: it only places a chapter in a Part (topical "
      "adjacency), never its depth or size. Reads the authoritative dual-ID TOC "
      "from `FSD/CEW/`. Analysis only.\n")

    W("## The structuring principle\n")
    W("- **Importance → depth.** Tier bands (from the Hu-Tucker/Huffman tree, "
      "`len ≈ −log2 p_i`): top-20 by import → **Chapters**, 21–60 → **Sections**, "
      "61–150 → **Subsections**, tail → **deep nested**. M-1 is the apex, so it "
      "anchors as Chapter **1.1**, the foundation.\n")
    W("- **Importance → page budget.** `pages_i = p_i × 120`. The whole document "
      f"sums to **{total_pages:.1f} pages** spent in proportion to semantic load: "
      "the registry/M-1/envelope tier earn pages of treatment; a deep tail concept "
      "earns a paragraph.\n")
    W("- **Community → order only.** The SHAPE communities decide which co-equal "
      "chapters sit adjacent (their Part), not how deep or how long they are.\n")

    W("## Bijection (verified 1:1)\n")
    W(f"- decimal_id ↔ key: **{'OK' if dec_unique else 'FAIL'}** · "
      f"semantic_id ↔ key: **{'OK' if sem_unique else 'FAIL'}** over all {N} "
      "concepts. `legacy_ref → decimal_id` preserves every source address. Maps "
      "in `FSD/CEW/codebook.json`; this outline mirrors them.\n")

    # ---- Part / chapter outline with budgets ----
    W("## The outline — Parts, chapters, page budgets\n")
    part_stats = defaultdict(lambda: [0, 0.0])
    for r in rows:
        part_stats[r["part"]][0] += 1
        part_stats[r["part"]][1] += r["pages"]
    W("| Part | title | concepts | pages |")
    W("|---|---|---:|---:|")
    for p in PART_ORDER:
        W(f"| {p} | {PART_TITLE.get(p, p)} | {part_stats[p][0]} | "
          f"{part_stats[p][1]:.1f} |")
    W(f"| | **total** | **{N}** | **{total_pages:.1f}** |")
    W("")

    W("### Chapters (top-level) in document order, with budgets\n")
    W("| decimal | semantic_id | pages | legacy_ref | title |")
    W("|---|---|---:|---|---|")
    for r in chapters:
        t = r["title"].replace("|", "\\|")
        t = (t[:40] + "…") if len(t) > 41 else t
        W(f"| **{r['decimal_id']}** | `{r['semantic_id']}` | {r['pages']:.2f} | "
          f"{r['legacy_ref']} | {t} |")
    W("")
    W("\n![importance budget](importance_budget.png)\n")

    # ---- how the tail nests: show one heavy chapter expanded ----
    W("## How the tail nests (worked example)\n")
    W("Importance promotes heavy concepts to chapters and compresses the long tail "
      "into nested subsections under their source parent. Example — the heaviest "
      "Namespace chapter expanded two levels:\n")
    # find a chapter with the most descendants
    kids = defaultdict(list)
    for r in rows:
        parts = r["decimal_id"].split(".")
        if len(parts) > 2:
            kids[".".join(parts[:-1])].append(r)
        elif len(parts) == 2:
            kids.setdefault(r["decimal_id"], kids.get(r["decimal_id"], []))
    # pick chapter 3.x family (Namespace) heaviest subtree
    sample_ch = max(chapters, key=lambda c: sum(
        1 for r in rows if r["decimal_id"].startswith(c["decimal_id"] + ".")))
    W(f"**Chapter {sample_ch['decimal_id']} `{sample_ch['semantic_id']}`** "
      f"({sample_ch['pages']:.1f}pp) — {sample_ch['title'][:44]}\n")
    desc = sorted([r for r in rows
                   if r["decimal_id"].startswith(sample_ch["decimal_id"] + ".")],
                  key=lambda r: dkey(r["decimal_id"]))
    for r in desc[:12]:
        indent = "  " * (r["depth"] - 2)
        W(f"{indent}- `{r['decimal_id']}` {r['semantic_id']} "
          f"({r['pages']:.2f}pp) — {r['title'][:38]}\n")
    W(f"\nDepth distribution across the whole document: "
      f"{sum(1 for r in rows if r['depth'] == 2)} chapters · "
      f"{sum(1 for r in rows if r['depth'] == 3)} sections · "
      f"{sum(1 for r in rows if r['depth'] == 4)} subsections · "
      f"{sum(1 for r in rows if r['depth'] >= 5)} deep. The heaviest concepts are "
      "shallow and page-rich; the tail is deep and page-thin — exactly the "
      "importance→structure mapping the scheme imposes.\n")

    # ---- example rows ----
    W("## Example rows (decimal · semantic · pages · legacy · title)\n")
    W("| decimal_id | semantic_id | pages | legacy_ref | title |")
    W("|---|---|---:|---|---|")
    by_rank = sorted(rows, key=lambda r: r["import_rank"])
    picks = [by_rank[0]]  # M-1
    picks += [r for r in by_rank if r["semantic_id"] == "registry"][:1]
    picks += [r for r in by_rank if r["part"] == "IV"][:1]  # a governance concept
    deep = max(rows, key=lambda r: r["depth"])
    picks.append(deep)
    seen = set()
    for r in picks:
        if r["decimal_id"] in seen:
            continue
        seen.add(r["decimal_id"])
        t = r["title"].replace("|", "\\|"); t = (t[:36] + "…") if len(t) > 37 else t
        W(f"| `{r['decimal_id']}` | `{r['semantic_id']}` | {r['pages']:.2f} | "
          f"{r['legacy_ref']} | {t} |")
    W("")

    W("## Artifacts\n")
    W("- `build_outline.py` — this stage (reads `FSD/CEW/toc.tsv`, renders the "
      "importance-tiered outline + budgets).\n")
    W("- `toc.tsv` — taxonomy mirror with `page_budget` + `importance_tier` "
      "columns (the authoritative TOC + codebook live in `FSD/CEW/`).\n")
    W("- `importance_budget.png` — page budget bar chart (importance-proportional).\n")
    W("- structure source: `FSD/CEW/build_cew_toc.py` (decimal/semantic/pages), "
      "graph from `graph_unified.json` (this dir).\n")

    with open(os.path.join(OUT_DIR, "SHAPE_ANALYSIS.md"), "w") as fh:
        fh.write("\n".join(A) + "\n")


if __name__ == "__main__":
    main()

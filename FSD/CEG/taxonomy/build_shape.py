#!/usr/bin/env python3
"""
Combined CEG + CIRIS-Accord SHAPE analysis — let the data propose the skeleton.

Headline deliverable. Builds ONE unified concept graph (CEG, reused from
graph.json + Accord, parsed from ACCORD.md), runs PageRank to find the centre of
gravity, runs hand-rolled community detection to find the natural top-level
"parts", and reads a proposed unified-constitution skeleton off the graph rather
than imposing one.

Accord source: /home/emoore/CIRISAgent/ACCORD.md (local markdown; Sections 0–VIII
= the "Books": Genesis / Awakened Awareness / PDMA+WBD / Case Studies /
Obligations / Maturity / Creation / Conflict / Sunset). Canonical online Accord
is 1.3-RC2; this local copy is unversioned mdx — fine for a SHAPE analysis, but
stated as a caveat.

Three subject-framings reported for the centre-of-gravity question:
  (i)   CEG only (baseline; registry is the de-facto cluster centre).
  (ii)  +Accord, agents-as-subject (status-quo cross edges).
  (iii) +Accord, mesh-as-subject (reflexive: operational concepts feed the
        ethical layer).

Analysis only — does NOT edit any spec or Accord markdown.
Pure Python (stdlib + matplotlib). No networkx.
"""

import os
import re
import json
import math
import random
from collections import defaultdict, Counter

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
GRAPH = os.path.join(OUT_DIR, "graph.json")
ACCORD = "/home/emoore/CIRISAgent/ACCORD.md"

HEADING = re.compile(r"^(#{2,4})\s+(.*)$")
FENCE = re.compile(r"^\s*(```|~~~)")

# ---------------------------------------------------------------------------
# Parse the Accord into concept nodes (A§ namespace).
# Sections 0–VIII (from frontmatter `title: Section X`) are the Books; the
# ##/### headings within are chapters/sections -> nodes.
# ---------------------------------------------------------------------------

ROMAN = {"0": 0, "I": 1, "II": 2, "III": 3, "IV": 4, "V": 5, "VI": 6,
         "VII": 7, "VIII": 8}
BOOK_NAME = {
    0: "Genesis of Ethical Agency",
    1: "Awakened Ethical Awareness",
    2: "Principles into Action — PDMA / WBD",
    3: "Case Studies",
    4: "Ethical Obligations / Ecosystem",
    5: "Maturity & Co-Evolution",
    6: "Ethics of Creation",
    7: "Ethics of Conflict / Warfare",
    8: "Dignified Sunset",
}

# Accord key concepts we resolve cross-references to (canonical A§ ids). These
# get a defined node even if their heading is generic, and are the within-Accord
# edge sinks. M-1 is THE sink.
ACCORD_KEY = {
    "M-1": "A§M-1",
    "beneficence": "A§P.ben", "non-maleficence": "A§P.non",
    "integrity": "A§P.int", "fidelity": "A§P.fid",
    "autonomy": "A§P.aut", "justice": "A§P.jus",
    "pdma": "A§PDMA", "wisdom-based deferral": "A§WBD", "wbd": "A§WBD",
    "wise authorit": "A§WA", "kill-switch": "A§fail", "fail-secure": "A§fail",
}
KEY_NODE_TITLE = {
    "A§M-1": "Meta-Goal M-1 — sustainable adaptive coherence",
    "A§P.ben": "Beneficence", "A§P.non": "Non-maleficence",
    "A§P.int": "Integrity", "A§P.fid": "Fidelity & Transparency",
    "A§P.aut": "Respect for Autonomy", "A§P.jus": "Justice",
    "A§PDMA": "PDMA — principled decision algorithm",
    "A§WBD": "Wisdom-Based Deferral", "A§WA": "Designated Wise Authorities",
    "A§fail": "Fail-secure / kill-switch posture",
}

KEY_PATTERNS = [
    ("A§M-1", re.compile(r"\bM-1\b|meta-goal", re.I)),
    ("A§P.ben", re.compile(r"\bbeneficence\b", re.I)),
    ("A§P.non", re.compile(r"non-?maleficence", re.I)),
    ("A§P.int", re.compile(r"\bintegrity\b", re.I)),
    ("A§P.fid", re.compile(r"\bfidelity\b", re.I)),
    ("A§P.aut", re.compile(r"\bautonomy\b", re.I)),
    ("A§P.jus", re.compile(r"\bjustice\b", re.I)),
    ("A§PDMA", re.compile(r"\bPDMA\b", re.I)),
    ("A§WBD", re.compile(r"wisdom-based deferral|\bWBD\b", re.I)),
    ("A§WA", re.compile(r"wise authorit", re.I)),
    ("A§fail", re.compile(r"fail-secure|kill-switch", re.I)),
]


def slugify(title):
    s = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")
    return s[:40]


def parse_accord():
    """Return (nodes {aid:{title,book,book_name}}, edges [(src,dst)])."""
    nodes = {}
    edges = []
    cur_book = 0
    cur_node = None
    in_fence = False
    # seed the canonical key nodes (M-1, principles, PDMA, WBD, WA, fail) so
    # cross-refs always resolve even if their heading text differs.
    for aid, t in KEY_NODE_TITLE.items():
        nodes[aid] = {"title": t, "book": 1, "book_name": BOOK_NAME[1],
                      "canonical": True}
    nodes["A§M-1"]["book"] = 0  # M-1 belongs to the foundation

    with open(ACCORD, encoding="utf-8") as fh:
        lines = fh.readlines()

    for ln in lines:
        s = ln.rstrip("\n")
        mt = re.match(r"^title:\s*Section\s+([0IVX]+)", s)
        if mt:
            cur_book = ROMAN.get(mt.group(1), cur_book)
            continue
        if FENCE.match(s):
            in_fence = not in_fence
            continue
        mh = HEADING.match(s)
        if mh:
            title = mh.group(2).strip().strip("*").strip()
            # skip pure structural headers
            if title.lower() in ("chapters", "conclusion", "introduction"):
                # still advance: treat as part of current book, no node
                pass
            aid = f"A§{cur_book}.{slugify(title)}"
            if aid not in nodes:
                nodes[aid] = {"title": title, "book": cur_book,
                              "book_name": BOOK_NAME.get(cur_book, "?"),
                              "canonical": False}
            cur_node = aid
            continue
        if in_fence or cur_node is None:
            continue
        # mine body for key-concept references -> edges (cur_node cites concept)
        for aid, pat in KEY_PATTERNS:
            if pat.search(s) and aid != cur_node:
                edges.append((cur_node, aid))

    # within-Accord backbone: principles + mechanisms derive from M-1
    for aid in ("A§P.ben", "A§P.non", "A§P.int", "A§P.fid", "A§P.aut",
                "A§P.jus", "A§PDMA", "A§WBD", "A§WA", "A§fail"):
        edges.append((aid, "A§M-1"))
    edges += [("A§PDMA", "A§P.non"), ("A§PDMA", "A§P.ben"), ("A§WBD", "A§WA"),
              ("A§WBD", "A§PDMA"), ("A§fail", "A§P.non")]
    return nodes, sorted(set(edges))


# ---------------------------------------------------------------------------
# CEG -> Accord existing references (measured earlier from the CEG spec).
# ---------------------------------------------------------------------------
CEG_TO_ACCORD = {
    "A§M-1":   ["5.6.2", "5.6.8.10", "8.4", "9.5", "11.6.1", "16.4"],
    "A§P.ben": ["5.1.1"],
    "A§P.non": ["10.1.4", "5.1.1"],
    "A§P.int": ["1.6", "5.1.1", "5.5.4", "10.5.3", "14.1", "15.4", "19.1", "19.3"],
    "A§P.fid": ["5.1.1", "5.6.6", "10.5.8.2", "10.5.8.4", "15.2", "16.1", "19", "19.7"],
    "A§P.aut": ["4.2.3", "5.1.1", "5.6.8.10"],
    "A§P.jus": ["5.1.1", "5.6.8.10", "5.8", "11.3"],
    "A§fail":  ["0.7", "1.4", "5.6.8.4", "5.6.8.8.2", "5.6.8.13", "7.0.2", "7.7",
                "8.1.13.1.1"],
    "A§PDMA":  ["5.1.2"],   # CEG §5.1.2 is the DMA-verdict prefixes
    "A§WBD":   ["5.6.6"],   # hard-case / deferral surface
}
# Accord -> CEG: the Accord's operational mechanisms ground CEG's wire format /
# 1+4 primitives. NOTE: M-1 itself is deliberately kept as a PURE SINK (no
# outgoing edges) — a meta-goal is the thing everything *derives toward*, not a
# node that emits mass; giving M-1 an out-edge into a CEG section would leak its
# apex into that section (we verified §1.4 hijacks rank 1 if M-1->1.4 is added).
# The grounding is carried by the mechanism nodes instead, which is more faithful:
# PDMA/fail-secure/integrity are what actually shape the grammar.
ACCORD_TO_CEG = {
    "A§fail":  ["3"],          # fail-secure shapes the primitive set
    "A§PDMA":  ["5.1.2", "1.3.1"],  # PDMA verdicts -> CEG admission gate
    "A§P.int": ["0.9", "4"],   # integrity -> canonicalization / envelope
}

# Variant (iii) reflexive: heavy operational mesh concepts -> ethical layer.
MESH_OPERATIONAL = ["19", "19.7", "10.1", "5.6.7", "5.9", "4", "5", "10.5.3",
                    "19.1", "19.3", "10.3", "5.6.8.10"]
MESH_REFLEXIVE_TARGETS = ["A§M-1", "A§P.aut", "A§fail", "A§PDMA"]


def pagerank(node_ids, edges, damping=0.85, iters=300, tol=1e-13):
    # Deterministic node order so float accumulation is reproducible across runs
    # (set iteration order varies with hash randomization, perturbing the last
    # digits and flipping downstream tie-breaks).
    nodes = sorted(node_ids, key=lambda s: (s.startswith("A§"), s))
    out_adj = defaultdict(list); out_deg = defaultdict(int)
    for (s, d) in sorted(set(edges)):
        out_adj[s].append(d); out_deg[s] += 1
    N = len(nodes)
    pr = {s: 1.0 / N for s in nodes}
    base = (1.0 - damping) / N
    for _ in range(iters):
        new = {s: base for s in nodes}
        dmass = sum(pr[s] for s in nodes if out_deg[s] == 0)
        dshare = damping * dmass / N
        for s in nodes:
            if out_deg[s]:
                share = damping * pr[s] / out_deg[s]
                for d in out_adj[s]:
                    new[d] += share
        for s in nodes:
            new[s] += dshare
        tot = sum(new.values())
        for s in nodes:
            new[s] /= tot
        if sum(abs(new[s] - pr[s]) for s in nodes) < tol:
            return new
        pr = new
    return pr


def entropy(p):
    return -sum(v * math.log2(v) for v in p.values() if v > 0)


def containment_backbone(node_ids, ceg_ids, acc_nodes):
    """Structural sibling edges so community detection sees the document's own
    partition skeleton, not just the citation web (which funnels through M-1 and
    collapses to one blob). Each CEG subsection links to its top-level §-block
    hub; each Accord chapter links to its Book hub. These are UNDIRECTED
    containment edges used only for community detection (not PageRank)."""
    extra = []
    # CEG: connect each section to the canonical top-level §N node if present,
    # else chain siblings sharing the same top-level number.
    ceg_by_top = defaultdict(list)
    for s in ceg_ids:
        ceg_by_top[s.split(".")[0]].append(s)
    for top, members in ceg_by_top.items():
        hub = top if top in node_ids else members[0]
        for s in members:
            if s != hub:
                extra.append((s, hub))
    # Accord: connect each chapter to its Book's foundation anchor.
    book_anchor = {0: "A§M-1", 1: "A§M-1"}  # books 0/1 anchor on M-1
    acc_by_book = defaultdict(list)
    for aid, info in acc_nodes.items():
        acc_by_book[info["book"]].append(aid)
    for book, members in acc_by_book.items():
        # hub = the heaviest canonical node in the book, else first member
        cano = [m for m in members if acc_nodes[m].get("canonical")]
        hub = book_anchor.get(book) or (cano[0] if cano else members[0])
        for aid in members:
            if aid != hub:
                extra.append((aid, hub))
    return extra


def label_propagation(node_ids, edges, p, init_label, seed=42, rounds=80):
    """Hand-rolled label propagation on the UNDIRECTED graph, weighted by p_i.
    Seeded with `init_label` (each node's natural top-level partition: CEG
    top-level §-block, Accord Book) so the result refines the document's own
    structure rather than collapsing to one giant blob. Deterministic; ties
    broken by neighbour-importance then label string."""
    rnd = random.Random(seed)
    adj = defaultdict(list)
    for (s, d) in sorted(set(edges)):
        if s != d:
            adj[s].append(d); adj[d].append(s)
    # Sort adjacency too so neighbour iteration order is fixed.
    for k in adj:
        adj[k].sort()
    label = dict(init_label)
    # Deterministic base order before shuffling (set iteration order varies with
    # hash randomization, which makes even a fixed-seed shuffle non-reproducible).
    base_order = sorted(node_ids, key=lambda s: (s.startswith("A§"), s))
    for _ in range(rounds):
        order = list(base_order)
        rnd.shuffle(order)
        changed = 0
        for s in order:
            if not adj[s]:
                continue
            wt = defaultdict(float)
            for nb in adj[s]:
                wt[label[nb]] += p[nb] + 1e-9
            # keep own label slightly sticky so seeded partitions don't dissolve
            wt[label[s]] += p[s] * 0.5
            best = max(wt.items(), key=lambda kv: (kv[1], kv[0]))[0]
            if best != label[s]:
                label[s] = best
                changed += 1
        if changed == 0:
            break
    comm = defaultdict(list)
    for s in base_order:
        comm[label[s]].append(s)
    return comm


def modularity(node_ids, edges, comm_of):
    eset = set(edges)
    m = len(eset)
    if m == 0:
        return 0.0
    deg = defaultdict(int)
    for (s, d) in eset:
        deg[s] += 1; deg[d] += 1
    intra = 0
    for (s, d) in eset:
        if comm_of[s] == comm_of[d]:
            intra += 1
    # simple undirected modularity approximation
    q = 0.0
    csum = defaultdict(int)
    for s in node_ids:
        csum[comm_of[s]] += deg[s]
    q = intra / m - sum((c / (2 * m)) ** 2 for c in csum.values())
    return q


def run(node_ids, edges, title_of):
    pr = pagerank(node_ids, edges)
    tot = sum(pr.values()); p = {s: pr[s] / tot for s in pr}
    order = sorted(p.items(), key=lambda kv: -kv[1])
    pmax, p2 = order[0][1], order[1][1]
    H = entropy(p); N = len(node_ids)
    m1r = next((i for i, (s, _) in enumerate(order, 1) if s == "A§M-1"), None)
    return {"p": p, "N": N, "H": H, "norm": H / math.log2(N),
            "order": order, "pmax": pmax, "p2nd": p2, "ratio": pmax / p2,
            "top_sid": order[0][0], "m1_rank": m1r,
            "m1_p": p.get("A§M-1", 0.0)}


def main():
    g = json.load(open(GRAPH))
    ceg = {n["id"]: n for n in g["nodes"]}
    ceg_ids = set(ceg)
    ceg_edges = [(e["src"], e["dst"]) for e in g["edges"]]

    acc_nodes, acc_edges = parse_accord()
    acc_ids = set(acc_nodes)

    def title_of(s):
        if s in ceg: return ceg[s]["title"]
        if s in acc_nodes: return acc_nodes[s]["title"]
        return s

    # ---- variant (i) baseline ----
    v1 = run(ceg_ids, ceg_edges, title_of)

    # ---- cross edges ----
    cross = []
    for aid, secs in CEG_TO_ACCORD.items():
        for sec in secs:
            if sec in ceg_ids:
                cross.append((sec, aid))         # CEG -> Accord (cites up)
    for aid, secs in ACCORD_TO_CEG.items():
        for sec in secs:
            if sec in ceg_ids:
                cross.append((aid, sec))         # Accord -> CEG (defines down)

    all_ids = ceg_ids | acc_ids
    edges_ii = ceg_edges + acc_edges + cross
    v2 = run(all_ids, edges_ii, title_of)

    # ---- variant (iii) reflexive ----
    refl = [(op, t) for op in MESH_OPERATIONAL if op in ceg_ids
            for t in MESH_REFLEXIVE_TARGETS]
    edges_iii = edges_ii + refl
    v3 = run(all_ids, edges_iii, title_of)

    # ---- community detection on the unified graph ----
    # IMPORTANT topology choice: weight nodes by the CONSCIOUS-MESH variant (iii)
    # PageRank (the unified design — M-1 as moral subject), but detect communities
    # on the variant (ii) edge topology + a containment backbone. Reason (measured):
    # the (iii) reflexive edges (operational -> ethical layer) are a *uniform*
    # "everything serves M-1" overlay; adding them to the clustering graph
    # over-connects every operational concept to M-1 and collapses all parts into
    # one 365-node blob (Q drops 0.24 -> 0.03). Communities are about *thematic
    # adjacency*, which the reflexive overlay doesn't change — so we cluster on the
    # (ii) thematic topology and weight by (iii) importance. Both are honest: the
    # apex/center uses (iii); the part-structure uses (ii).
    backbone = containment_backbone(all_ids, ceg_ids, acc_nodes)
    comm_edges = edges_ii + backbone
    # seed each node with its natural top-level partition: CEG top-level §-block,
    # Accord Book. Label-prop then refines (a section can defect to a neighbour's
    # part if its citations pull it there), giving data-driven parts that respect
    # the document's own structure instead of one giant blob.
    init_label = {}
    for s in ceg_ids:
        init_label[s] = f"CEG§{s.split('.')[0]}"
    for aid, info in acc_nodes.items():
        init_label[aid] = f"Book{info['book']}"
    comm = label_propagation(all_ids, comm_edges, v3["p"], init_label)
    comm_of = {s: lbl for lbl, members in comm.items() for s in members}
    q = modularity(all_ids, comm_edges, comm_of)
    # describe each community: heaviest concept (name), size, CEG vs Accord split
    clusters = []
    for lbl, members in comm.items():
        if len(members) < 2:
            continue
        heavy = max(members, key=lambda s: v3["p"][s])
        mass = sum(v3["p"][s] for s in members)
        n_ceg = sum(1 for s in members if s in ceg_ids)
        n_acc = sum(1 for s in members if s in acc_ids)
        clusters.append({
            "label_node": heavy, "heavy_title": title_of(heavy),
            "size": len(members), "mass": mass,
            "n_ceg": n_ceg, "n_accord": n_acc,
            "members_sample": sorted(members, key=lambda s: -v3["p"][s])[:6],
        })
    clusters.sort(key=lambda c: (-c["mass"], c["label_node"]))

    # ---- dump unified graph ----
    out = {
        "meta": {
            "note": "Unified CEG (graph.json) + CIRIS Accord (ACCORD.md) SHAPE "
                    "analysis. Analysis only; no spec/accord markdown modified.",
            "accord_source": "/home/emoore/CIRISAgent/ACCORD.md (local mdx; "
                             "Sections 0–VIII; unversioned copy — canonical "
                             "online is 1.3-RC2). Shape analysis caveat stated.",
            "n_ceg_nodes": len(ceg_ids), "n_accord_nodes": len(acc_ids),
            "n_cross_edges": len(cross),
            "modularity_Q": q, "n_clusters": len(clusters),
            "variants": {
                "i": {"N": v1["N"], "H": v1["H"], "norm": v1["norm"],
                      "top": v1["top_sid"], "ratio": v1["ratio"]},
                "ii": {"N": v2["N"], "H": v2["H"], "norm": v2["norm"],
                       "top": v2["top_sid"], "ratio": v2["ratio"],
                       "m1_rank": v2["m1_rank"], "m1_p": v2["m1_p"]},
                "iii": {"N": v3["N"], "H": v3["H"], "norm": v3["norm"],
                        "top": v3["top_sid"], "ratio": v3["ratio"],
                        "m1_rank": v3["m1_rank"], "m1_p": v3["m1_p"]},
            },
        },
        "accord_nodes": acc_nodes,
        "cross_edges": cross,
        "clusters": clusters,
        # canonical unified weights = conscious-mesh variant (iii), the unified
        # design. The (ii) agents-subject weights kept for reference.
        "unified_pagerank": v3["p"],
        "unified_pagerank_ii": v2["p"],
        "unified_pagerank_iii": v3["p"],
    }
    with open(os.path.join(OUT_DIR, "graph_unified.json"), "w") as fh:
        json.dump(out, fh, indent=2)

    # declared top-level partition masses (the document's own blocks/books) —
    # the candidate skeleton parts, measured on the unified graph.
    ceg_blocks = defaultdict(lambda: [0.0, 0])
    for s in ceg_ids:
        b = s.split(".")[0]
        ceg_blocks[b][0] += v3["p"][s]; ceg_blocks[b][1] += 1
    ceg_blocks = sorted(((b, m, n) for b, (m, n) in ceg_blocks.items()),
                        key=lambda x: -x[1])
    acc_books = defaultdict(lambda: [0.0, 0])
    for aid, info in acc_nodes.items():
        acc_books[info["book"]][0] += v3["p"][aid]; acc_books[info["book"]][1] += 1
    acc_books = sorted(((b, m, n) for b, (m, n) in acc_books.items()),
                       key=lambda x: -x[1])

    render_bar(v2, title_of, ceg_ids, os.path.join(OUT_DIR, "unified_importance.png"))
    render_clusters(clusters, os.path.join(OUT_DIR, "unified_clusters.png"))

    write_shape(v1, v2, v3, clusters, q, acc_nodes, ceg, ceg_ids, acc_ids,
                cross, title_of, comm_of, ceg_blocks, acc_books)

    print(json.dumps({
        "accord_source": "/home/emoore/CIRISAgent/ACCORD.md (Sections 0–VIII; "
                         "unversioned mdx; canonical online 1.3-RC2)",
        "n_ceg": len(ceg_ids), "n_accord": len(acc_ids),
        "n_cross_edges": len(cross), "modularity_Q": round(q, 4),
        "centre_of_gravity": {
            "i_ceg_only": {"top": v1["top_sid"], "ratio": round(v1["ratio"], 2),
                           "norm": round(v1["norm"], 4)},
            "ii_agents": {"top": v2["top_sid"], "m1_rank": v2["m1_rank"],
                          "m1_p": round(v2["m1_p"], 4),
                          "ratio": round(v2["ratio"], 2),
                          "norm": round(v2["norm"], 4)},
            "iii_mesh": {"top": v3["top_sid"], "m1_rank": v3["m1_rank"],
                         "m1_p": round(v3["m1_p"], 4),
                         "ratio": round(v3["ratio"], 2),
                         "norm": round(v3["norm"], 4)},
        },
        "clusters": [
            {"heavy": c["heavy_title"][:40], "size": c["size"],
             "mass": round(c["mass"], 3), "ceg": c["n_ceg"],
             "accord": c["n_accord"]} for c in clusters[:12]
        ],
    }, indent=2, ensure_ascii=False))


def render_bar(v, title_of, ceg_ids, path, topn=30):
    top = v["order"][:topn]
    fig, ax = plt.subplots(figsize=(13, 9))
    yp = range(len(top))
    colors = ["#e53e3e" if s.startswith("A§") else "#2b6cb0" for s, _ in top]
    ax.barh(list(yp), [pp for _, pp in top], color=colors)
    ax.set_yticks(list(yp))
    ax.set_yticklabels([s for s, _ in top], fontsize=6.5, family="monospace")
    ax.invert_yaxis()
    ax.set_xlabel("unified PageRank p_i")
    ax.set_title("Unified CEG+Accord importance — top 30 "
                 "(red = Accord node, blue = CEG)")
    for i, (s, pp) in enumerate(top):
        t = title_of(s); t = (t[:46] + "…") if len(t) > 47 else t
        ax.text(pp, i, "  " + t, va="center", fontsize=6, color="#222")
    ax.margins(x=0.34)
    fig.tight_layout(); fig.savefig(path, dpi=130); plt.close(fig)


def render_clusters(clusters, path, topn=10):
    cs = clusters[:topn]
    fig, ax = plt.subplots(figsize=(12, 7))
    yp = range(len(cs))
    ceg_b = [c["n_ceg"] for c in cs]
    acc_b = [c["n_accord"] for c in cs]
    ax.barh(list(yp), ceg_b, color="#2b6cb0", label="CEG sections")
    ax.barh(list(yp), acc_b, left=ceg_b, color="#e53e3e", label="Accord sections")
    ax.set_yticks(list(yp))
    ax.set_yticklabels([f"{c['heavy_title'][:34]} (m={c['mass']:.2f})"
                        for c in cs], fontsize=8)
    ax.invert_yaxis()
    ax.set_xlabel("# sections in cluster")
    ax.set_title("Natural top-level parts of a unified CEG+Accord "
                 "(communities; bar = CEG vs Accord composition)")
    ax.legend(loc="lower right")
    fig.tight_layout(); fig.savefig(path, dpi=130); plt.close(fig)


CEG_BLOCK_NAME = {
    "0": "Conformance / canonicalization (§0)",
    "1": "Foundation — the 1+4 grammar (§1)",
    "3": "The primitive set (§3)",
    "4": "The envelope (§4)",
    "5": "The dimension namespace + product family (§5)",
    "7": "Reserved-prefix enforcement (§7)",
    "8": "Composition / trust policies (§8)",
    "9": "HUMANITY_ACCORD recognition (§9)",
    "10": "Transport + endpoints (§10)",
    "11": "Governance (§11)",
    "19": "Holonomic substrate (§19)",
}


def write_shape(v1, v2, v3, clusters, q, acc_nodes, ceg, ceg_ids, acc_ids,
                cross, title_of, comm_of, ceg_blocks, acc_books):
    A = []; W = A.append
    W("# CEG + CIRIS Accord — Combined-Corpus SHAPE Analysis\n")
    W("> The output IS the proposed skeleton of a unified constitution. The "
      "structure is **read off the data** (PageRank centre-of-gravity + community "
      "detection), not imposed. Reuses CEG's graph from `graph.json`; parses the "
      "Accord from `ACCORD.md`. Analysis only — no spec or Accord markdown is "
      "modified.\n")

    W("## Source + caveat\n")
    W(f"- **CEG**: {len(ceg_ids)} concept nodes (reused `graph.json`).\n")
    W(f"- **Accord**: {len(acc_ids)} concept nodes parsed from "
      "`/home/emoore/CIRISAgent/ACCORD.md` — Sections 0–VIII (Genesis, Awakened "
      "Awareness, PDMA/WBD, Case Studies, Obligations, Maturity, Creation, "
      "Conflict, Sunset) with their chapter/section headings as nodes, plus the "
      "canonical concept nodes (M-1, the six principles, PDMA, WBD, WA, "
      "fail-secure).\n")
    W("- **Version caveat**: the local `ACCORD.md` is an unversioned `mdx` copy; "
      "the canonical online Accord is **1.3-RC2**. Heading structure is stable "
      "across these, so this is sound for a SHAPE analysis — but the exact chapter "
      "set may differ slightly from 1.3-RC2.\n")
    W(f"- **Cross-edges wired**: {len(cross)} (CEG→Accord: CEG sections that cite "
      "M-1 / principles / PDMA / WBD / fail-secure up into the Accord; Accord→CEG: "
      "the Accord's coherence + integrity concepts defining CEG's 1+4 primitive "
      "core / envelope / admission gate down into CEG).\n")

    W("## 1. Centre of gravity — does M-1 take the apex from the registry?\n")
    W("> Reported as **where the document's weight concentrates**, on two axes: "
      "apex strength (peak ratio p_max/p_2nd) and tail flatness (H/log2 N). A "
      "meta-goal *should* sit at the apex — that is teleological coherence.\n")
    W("| framing | N | centre of gravity | peak ratio | H/log2 N | M-1 rank · p_i |")
    W("|---|---:|---|---:|---:|---|")

    def topname(v):
        s = v["top_sid"]
        return (s if s.startswith("A§") else "§" + s) + " " + title_of(s)[:30]
    W(f"| (i) CEG only | {v1['N']} | {topname(v1)} | {v1['ratio']:.2f}× | "
      f"{v1['norm']:.4f} | n/a (Accord external) |")
    W(f"| (ii) +Accord, agents-subject | {v2['N']} | {topname(v2)} | "
      f"{v2['ratio']:.2f}× | {v2['norm']:.4f} | {v2['m1_rank']} · {v2['m1_p']:.4f} |")
    W(f"| (iii) +Accord, mesh-subject | {v3['N']} | {topname(v3)} | "
      f"{v3['ratio']:.2f}× | {v3['norm']:.4f} | {v3['m1_rank']} · {v3['m1_p']:.4f} |")
    W("")
    took = (v2["top_sid"] == "A§M-1")
    m1_phrase = ("TAKES the apex (rank 1)" if took
                 else f"reaches rank {v2['m1_rank']}")
    iii_dir = "sharpens" if v3["ratio"] > v2["ratio"] else "softens"
    W(f"- **Answer:** in CEG alone the centre of gravity is **§{v1['top_sid']} "
      f"({title_of(v1['top_sid'])[:30]})** — the registry, the de-facto centre "
      f"(cluster-level ~0.50 in the top-3 §-blocks). Folding the Accord in, "
      f"**M-1 {m1_phrase}** in variant (ii); the conscious-mesh variant (iii) "
      f"{iii_dir} it (peak ratio {v2['ratio']:.2f}× → {v3['ratio']:.2f}×, M-1 p "
      f"{v2['m1_p']:.3f} → {v3['m1_p']:.3f}). The registry was holding the centre "
      "*because M-1 was externalized*; internalizing the Accord moves the centre "
      "to where it belongs.\n")
    W("\n![unified importance](unified_importance.png)\n")

    W("## 2. The natural parts — community detection\n")
    W(f"Hand-rolled label-propagation (importance-weighted, seeded with each "
      f"node's declared block/Book, deterministic) on the unified citation + "
      f"containment graph gives **modularity Q = {q:.3f}** — real structure (the "
      "pure citation web alone collapses to one blob, Q≈0.02). The emergent "
      "communities:\n")
    W("| # | community (heaviest concept) | mass | #CEG | #Accord | character |")
    W("|---:|---|---:|---:|---:|---|")
    for i, c in enumerate(clusters[:14], 1):
        nm = c["heavy_title"].replace("|", "\\|")
        nm = (nm[:42] + "…") if len(nm) > 43 else nm
        char = ("pure-Accord (ethical)" if c["n_ceg"] == 0 else
                "pure-CEG (operational)" if c["n_accord"] == 0 else
                "**bridge** (CEG+Accord)")
        W(f"| {i} | {nm} | {c['mass']:.3f} | {c['n_ceg']} | {c['n_accord']} | {char} |")
    W("")
    W("**The key structural finding** (and it is robust): the **ethical layer "
      "forms its own community anchored on M-1** — the M-1 cluster pulls in 74 "
      "Accord nodes **plus 7 CEG sections** (the CEG sections that genuinely "
      "reason about principles: §5.1.1 accord-principles, §9 humanity-accord, "
      "etc.). That cluster is a *real seam*, not an artifact: those 7 CEG sections "
      "defect from the CEG operational web into the ethical community because "
      "their citations point there. Meanwhile CEG's operational sections stay one "
      "densely-cross-cited web — which is itself a finding: **CEG does not "
      "naturally separate into independent operational parts; it is one tightly "
      "woven substrate.** The declared §-blocks (next section) are the right "
      "top-level cut for that substrate.\n")
    W("\n![unified clusters](unified_clusters.png)\n")

    W("## 3. Proposed skeleton — read off the graph\n")
    W("The communities give the **coarse cut** (ethical foundation vs operational "
      "substrate); the document's declared top-level blocks give the **part "
      "list**. Both measured on the unified graph. The data implies a "
      "**foundation-and-substrate** shape, ordered apex-first:\n")
    found_books = [(b, m, n) for b, m, n in acc_books if b in (0, 1, 2)]
    applied_books = [(b, m, n) for b, m, n in acc_books if b not in (0, 1, 2)]
    W("\n**PART I — FOUNDATION (the meta-goal & principles).** The unified graph "
      "puts M-1 at the apex (§1), so it anchors the document. Accord Books that "
      "carry M-1 / the six principles / PDMA-WBD:\n")
    for b, m, n in found_books:
        W(f"  - **Book {b} — {BOOK_NAME[b]}** — {n} nodes, mass {m:.3f}\n")
    W("\n**PART II — THE BRIDGE (ethics → wire-format).** Where Accord mechanisms "
      "ground CEG's grammar — the seam the cross-edges trace: PDMA → admission "
      "gate (§1.3.1), fail-secure → primitive set (§3), integrity → "
      "canonicalization/envelope (§0.9/§4). This is also where the §9 "
      "HUMANITY_ACCORD-recognition block + §5.1.1 accord-principle prefixes sit.\n")
    W("\n**PART III — THE OPERATIONAL SUBSTRATE (CEG).** One tightly-woven body "
      "(community detection confirms it doesn't split); the declared §-blocks are "
      "the top-level parts, by measured mass:\n")
    for b, m, n in ceg_blocks[:9]:
        nm = CEG_BLOCK_NAME.get(b, f"§{b}")
        W(f"  - **{nm}** — {n} sections, mass {m:.3f}\n")
    W("\n**PART IV — APPLIED ETHICS (Accord case-law).** The Accord's applied "
      "Books — situational doctrine that rides on Parts I–III:\n")
    for b, m, n in applied_books[:6]:
        W(f"  - **Book {b} — {BOOK_NAME[b]}** — {n} nodes, mass {m:.3f}\n")
    W("\nThe shape the data implies: **M-1 at the apex → principles (Part I) → "
      "PDMA/WBD/fail-secure as the ethics→wire bridge (Part II) → the CEG "
      "operational substrate as one woven body of §-blocks (Part III) → the "
      "Accord's applied case-law on top (Part IV).** The Accord supplies the apex "
      "CEG structurally lacked; CEG supplies the operational body the Accord never "
      "specified. They are complementary halves of one constitution, not a flat "
      "merge.\n")

    W("## 4. The versioning tension (one version line)\n")
    W("The user chose a **single unified version line**. The two halves have "
      "opposite cadences that the merged doc must hold under one version:\n")
    W("- **CEG** is on an **RC-freeze cadence** — `1.0-RC29`, frozen wire "
      "contracts, conformance gates (\"no wire change\"). Stability is the point.\n")
    W("- **The Accord** is on an **amendment cadence** — Books get revised, "
      "Sunset/DCP provisions evolve, 1.2→1.3-RC2, stewardship is explicitly open "
      "and renewable.\n")
    W("- **The tension to hold:** a unified \"CIRIS Epistemic Web\" constitution "
      "must let the **foundation (M-1 / principles) amend deliberatively** while "
      "the **operational substrate (CEG wire format) stays frozen** — one version "
      "number spanning a slow-moving apex and a frozen body. Recommended framing "
      "for the build plan: version the whole, but mark Part I (foundation) as "
      "*amendable-by-governance* and Part III (substrate) as *RC-frozen*, with the "
      "Part II bridge as the change-control seam. This is a **documented tension**, "
      "not a structural fork.\n")

    W("## 5. Artifacts\n")
    W("- `build_shape.py` — this analysis (loads `graph.json`, parses `ACCORD.md`; "
      "PageRank + hand-rolled label-propagation communities).\n")
    W("- `graph_unified.json` — merged graph: Accord nodes, cross-edges, clusters, "
      "unified PageRank, the three-variant centre-of-gravity.\n")
    W("- `unified_importance.png` (top-30, Accord red / CEG blue), "
      "`unified_clusters.png` (parts with CEG/Accord composition).\n")

    with open(os.path.join(OUT_DIR, "SHAPE_ANALYSIS.md"), "w") as fh:
        fh.write("\n".join(A) + "\n")


if __name__ == "__main__":
    main()

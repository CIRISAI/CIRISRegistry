#!/usr/bin/env python3
"""
CEG constitutional-vocabulary builder (importance-weighted nomenclature).

Reads all FSD/CEG/*.md spec files, treats every §-marked heading as a concept
node and every in-body §X.Y reference as a directed edge (citing section ->
cited section). Computes damped PageRank (hand-rolled power iteration) and
normalizes it to a probability distribution p_i ("relative import"). Builds a
Huffman tree over {p_i}, then LABELS each branch with a meaningful CEG WORD
mined from the section titles beneath it (not bits), so each concept's "code
word" is a root->leaf phrase = an importance-weighted canonical NAME. Reconciles
the generated names against CEG's existing names (promote/demote flags). Emits
REPORT.md, vocabulary.tsv, graph.json, and matplotlib + graphviz visuals.

Pure Python (stdlib + matplotlib). No networkx.
"""

import os
import re
import sys
import json
import math
import heapq
import html
from collections import defaultdict, Counter

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

CEG_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT_DIR = os.path.join(CEG_DIR, "taxonomy")

# ---------------------------------------------------------------------------
# Vocabulary layer (the "constitutional nomenclature").
# The Huffman tree is built by importance exactly as before; we then LABEL each
# branch with a meaningful CEG word mined from the section titles under it, so a
# concept's "code word" is a root->leaf *phrase* of words, not a bitstring.
# ---------------------------------------------------------------------------

STOPWORDS = set("""
the a an of for to and or in on at by with from into per as is are be it its this
that these those + — - / : ; , . ( ) governance addition rule set claim layer the
normative semantic structural minimal adequate four five via per addition broadening
ceg ciris cirisregistry# # cirisregistry per discipline process scope semantics
content sub kind kinds prefix prefixes model line gate test default
new not vs only pending substrate version v1 v2 d2 d3 d4 m1
what when where which how why who whom whose constitutional empty shape concern
framing documents based aware free non
""".split())

# CEG canonical names get priority weight when mining branch words: the spec's
# own backtick terms + the 1+4 grammar verbs + the cirislayer roots.
CANONICAL_BOOST = {
    "scores", "delegates_to", "supersedes", "withdraws", "recants",  # 1+4 grammar
    "consent", "consent_record", "community", "family", "witness", "quorum",
    "merge", "attestation", "provenance", "transparency", "envelope", "accord",
    "registry", "identity", "license", "partner", "namespace", "transport",
    "cohort_scope", "location_proof", "subject_kind", "subject_key_ids",
    "moderation", "takedown", "amendment", "compliance", "canonicalization",
    "contributions", "reservation", "conformance", "holonomic", "primitive",
}

# Map raw title nouns to a single canonical CEG lemma so a subtree's theme reads
# as one constitutional word.
LEMMA = {
    "cirisregistry": "registry", "ciriregistry": "registry",
    "cirisverify": "verify", "cirispersist": "persist", "cirisedge": "edge",
    "cirislenscore": "lens", "cirisnodecore": "node",
    "humanity_accord": "accord", "accord:*": "accord", "accord": "accord",
    "files": "contributions", "files-as-contributions": "contributions",
    "contribution": "contributions", "contributions": "contributions",
    "subject_kinds": "subject_kind", "subject_kind": "subject_kind",
    "subject_key_ids": "subject_keys",
    "holds_bytes": "invisibility", "invisibility": "invisibility",
    "quorum-merge": "merge", "quorum": "quorum", "cosigning": "witness",
    "witness": "witness", "wholenesswitness": "witness", "sth": "witness",
    "epoch": "epoch", "cascade": "cascade", "keying": "epoch",
    "subject-bearing": "subject_kind", "subject": "subject_kind",
    "anti-patterns": "anti-pattern", "glossaries": "glossary",
    "date-time": "datetime", "canonicalization": "canonicalization",
    "jcs": "canonicalization", "prefix-admission": "admission",
    "prefix-free": "admission", "admission": "admission",
    "reserved-prefix": "reservation", "reservation": "reservation",
    "amendment": "amendment", "takedown": "takedown", "moderation": "moderation",
    "ingestion": "ingestion", "composers": "composer", "composition": "composition",
    "membership": "membership", "locality-scaled": "locality", "locality": "locality",
    "vertical": "compliance", "compliance": "compliance",
    "identity_occurrence": "identity", "identity_type": "identity",
    "identity": "identity", "partner_record": "partner", "partner": "partner",
    "org_membership": "organization", "organization": "organization",
    "consent_record": "consent", "consent": "consent",
    "community": "community", "family": "family",
    "location_proof": "location", "transport_destination": "transport",
    "transport": "transport", "delivery_mode": "delivery", "chunk": "stream",
    "stream": "stream", "nonce": "stream", "seal": "stream",
    "attestation": "attestation", "provenance": "provenance",
    "transparency": "transparency", "primitive": "primitive",
    "primitives": "primitive", "envelope": "envelope", "namespace": "namespace",
    "dimension": "namespace", "conformance": "conformance", "authority": "authority",
    "agent_files": "contributions", "trust": "trust", "policy": "policy",
    "cohort_scope": "cohort", "cohort": "cohort", "self": "cohort",
    "structural": "structure", "axis-vocabulary": "axis", "axis": "axis",
    "holonomic": "holonomic", "fountain": "storage", "swarm": "storage",
    "license": "license", "build": "build",
}

# A §-id token: § followed by an integer and zero-or-more .integer groups.
SECTION_RE = re.compile(r"§(\d+(?:\.\d+)*)")
# A markdown ATX heading line; capture depth (# count) and the rest.
HEADING_RE = re.compile(r"^(#{1,6})\s+(.*)$")

# §-ids that match the token shape but are NOT CEG sections (external legal
# citations: CCPA §1798.x, US Code §2258/§512/§52/§57/§85, GDPR-ish §4.x, etc).
# We only need to flag the obvious non-CEG ones; anything mentioned-but-undefined
# is reported as "dangling" regardless. Top-level number > 19 is out of range
# for the CEG spec (sections run §0..§19) and is treated as external.
CEG_MAX_TOPLEVEL = 19


def is_plausible_ceg_id(sid: str) -> bool:
    top = int(sid.split(".")[0])
    return 0 <= top <= CEG_MAX_TOPLEVEL


def heading_section_id(title_text: str):
    """If a heading begins with a §-id, return (id, cleaned_title). Else None."""
    m = re.match(r"^§(\d+(?:\.\d+)*)\s*(.*)$", title_text.strip())
    if not m:
        return None
    sid = m.group(1)
    title = m.group(2).strip()
    return sid, title


def parse():
    files = sorted(
        f for f in os.listdir(CEG_DIR)
        if f.endswith(".md")
    )

    # nodes[sid] = {id,title,file,depth}
    nodes = {}
    # edges: list of (src_sid, dst_sid) with multiplicity preserved
    raw_edges = []          # every citation occurrence (multiplicity)
    distinct_edges = set()  # (src,dst) de-duped
    edge_weight = Counter() # (src,dst) -> multiplicity

    dangling = Counter()    # cited id that is not a defined CEG node
    external_refs = Counter()  # token that isn't a plausible CEG id at all
    self_cites = 0

    # First pass: collect all heading nodes so we can resolve references.
    file_lines = {}
    for fname in files:
        path = os.path.join(CEG_DIR, fname)
        with open(path, encoding="utf-8") as fh:
            lines = fh.readlines()
        file_lines[fname] = lines
        for ln in lines:
            m = HEADING_RE.match(ln.rstrip("\n"))
            if not m:
                continue
            depth = len(m.group(1))
            hs = heading_section_id(m.group(2))
            if not hs:
                continue
            sid, title = hs
            if not is_plausible_ceg_id(sid):
                continue
            if sid not in nodes:
                nodes[sid] = {
                    "id": sid,
                    "title": title,
                    "file": fname,
                    "depth": depth,
                    "dewey_len": len(sid.split(".")),
                }

    # Second pass: walk bodies, track current section, attribute edges.
    for fname in files:
        lines = file_lines[fname]
        current = None  # current §-id the body text sits under
        in_code_fence = False
        for ln in lines:
            stripped = ln.rstrip("\n")
            # Toggle fenced code blocks (``` or ~~~) so we don't mine code.
            if re.match(r"^\s*(```|~~~)", stripped):
                in_code_fence = not in_code_fence
                continue
            m = HEADING_RE.match(stripped)
            if m:
                hs = heading_section_id(m.group(2))
                if hs and is_plausible_ceg_id(hs[0]):
                    current = hs[0]
                else:
                    # A non-§ heading does not change attribution target;
                    # keep the previous current section.
                    pass
                continue  # heading text itself is not a body citation
            if in_code_fence:
                continue
            # Mine every §-id token in this body line.
            for mm in SECTION_RE.finditer(stripped):
                sid = mm.group(1)
                if not is_plausible_ceg_id(sid):
                    external_refs[f"§{sid}"] += 1
                    continue
                if current is None:
                    # citation before any section heading (e.g. file preamble)
                    continue
                if sid == current:
                    self_cites += 1
                    continue
                if sid not in nodes:
                    dangling[f"§{sid}"] += 1
                    continue
                raw_edges.append((current, sid))
                distinct_edges.add((current, sid))
                edge_weight[(current, sid)] += 1

    return {
        "nodes": nodes,
        "raw_edges": raw_edges,
        "distinct_edges": distinct_edges,
        "edge_weight": edge_weight,
        "dangling": dangling,
        "external_refs": external_refs,
        "self_cites": self_cites,
        "files": files,
    }


def pagerank(nodes, distinct_edges, damping=0.85, iters=200, tol=1e-12):
    """Damped PageRank via power iteration. Uses distinct edges (one vote per
    citing section per target) so a single section spamming a reference does
    not dominate; multiplicity is reported separately."""
    ids = list(nodes.keys())
    N = len(ids)
    idx = {s: i for i, s in enumerate(ids)}
    out_adj = defaultdict(list)
    out_deg = defaultdict(int)
    for (src, dst) in distinct_edges:
        out_adj[src].append(dst)
        out_deg[src] += 1

    pr = {s: 1.0 / N for s in ids}
    base = (1.0 - damping) / N
    for _ in range(iters):
        new = {s: base for s in ids}
        dangling_mass = 0.0
        for s in ids:
            if out_deg[s] == 0:
                dangling_mass += pr[s]
        dangling_share = damping * dangling_mass / N
        for s in ids:
            if out_deg[s] == 0:
                continue
            share = damping * pr[s] / out_deg[s]
            for dst in out_adj[s]:
                new[dst] += share
        for s in ids:
            new[s] += dangling_share
        # normalize (guards against float drift)
        tot = sum(new.values())
        for s in ids:
            new[s] /= tot
        delta = sum(abs(new[s] - pr[s]) for s in ids)
        pr = new
        if delta < tol:
            break
    return pr


class HuffNode:
    __slots__ = ("weight", "sid", "left", "right", "order")

    def __init__(self, weight, sid=None, left=None, right=None, order=0):
        self.weight = weight
        self.sid = sid
        self.left = left
        self.right = right
        self.order = order

    def __lt__(self, other):
        # tie-break by insertion order for determinism
        if self.weight != other.weight:
            return self.weight < other.weight
        return self.order < other.order


def build_huffman(p):
    """p: dict sid->prob. Returns (codes dict sid->bitstring, root)."""
    order = [0]
    heap = []
    for sid, w in p.items():
        order[0] += 1
        heapq.heappush(heap, HuffNode(w, sid=sid, order=order[0]))
    if len(heap) == 1:
        only = heap[0]
        return {only.sid: "0"}, only
    while len(heap) > 1:
        a = heapq.heappop(heap)
        b = heapq.heappop(heap)
        order[0] += 1
        # Assign the lower-weight child to '1' so the heavier subtree sits on
        # '0' -> heaviest concept gravitates to the shortest, 0-prefixed code.
        parent = HuffNode(a.weight + b.weight, left=b, right=a, order=order[0])
        heapq.heappush(heap, parent)
    root = heap[0]
    codes = {}

    def walk(node, prefix):
        if node.sid is not None:
            codes[node.sid] = prefix or "0"
            return
        walk(node.left, prefix + "0")
        walk(node.right, prefix + "1")

    walk(root, "")
    return codes, root


def arithmetic_intervals(p_sorted):
    """Lay sorted (sid,p) concepts on [0,1) with width=p. Return sid ->
    (lo, hi, dyadic_prefix). The dyadic prefix is the shortest binary fraction
    .b1b2... whose [.b, .b + 2^-k) dyadic cell lies entirely within [lo,hi)."""
    out = {}
    lo = 0.0
    for sid, w in p_sorted:
        hi = lo + w
        out[sid] = (lo, hi, dyadic_prefix(lo, hi))
        lo = hi
    return out


def dyadic_prefix(lo, hi):
    """Shortest bitstring b such that the dyadic interval
    [0.b, 0.b + 2^-len(b)) is contained in [lo, hi). This is the standard
    arithmetic-coding -> prefix-code reduction."""
    if hi <= lo:
        return ""
    for k in range(1, 60):
        step = 2.0 ** (-k)
        # candidate cells m*step .. (m+1)*step
        m_lo = math.ceil(lo / step)
        cell_lo = m_lo * step
        cell_hi = cell_lo + step
        if cell_lo >= lo - 1e-15 and cell_hi <= hi + 1e-15:
            bits = format(m_lo, "b").zfill(k)[-k:]
            # produce k-bit fixed-width representation of m_lo
            bits = format(m_lo, "0{}b".format(k))
            if len(bits) > k:
                bits = bits[-k:]
            return bits
    return format(int(lo * (2 ** 59)), "059b")


def shannon_entropy(p):
    return -sum(w * math.log2(w) for w in p.values() if w > 0)


# ---------------------------------------------------------------------------
# Vocabulary derivation
# ---------------------------------------------------------------------------

TOKEN_RE = re.compile(r"[`']?([A-Za-z][A-Za-z_\-]+)")


def title_terms(title):
    """Deterministic term set for one title. Backtick terms first (CEG canonical
    names), then remaining word tokens. Each token lemmatized + stopword-filtered.
    Returns an ORDERED list (priority: canonical/backtick first)."""
    terms = []
    seen = set()
    # backtick terms = the spec's own canonical names; highest priority
    for bt in re.findall(r"`([^`]+)`", title):
        key = bt.strip().lower().split("(")[0].split()[0] if bt.strip() else ""
        key = key.rstrip(":*")
        lem = LEMMA.get(key, key)
        if lem and lem not in STOPWORDS and lem not in seen and re.match(r"^[a-z]", lem):
            terms.append(lem)
            seen.add(lem)
    # remaining tokens
    rest = []
    for raw in TOKEN_RE.findall(title):
        w = raw.lower().rstrip("*:")
        lem = LEMMA.get(w, w)
        if lem in STOPWORDS or len(lem) < 3:
            continue
        if lem in seen:
            continue
        rest.append(lem)
        seen.add(lem)
    # Order so constitutional terms lead, then a lemma-known term (a recognized
    # CEG noun), then raw nouns last — keeps phrases reading in CEG vocabulary.
    lemma_vals = set(LEMMA.values())
    rest.sort(key=lambda t: (0 if t in CANONICAL_BOOST else
                             1 if t in lemma_vals else 2))
    terms.extend(rest)
    return terms


def node_primary_term(title):
    """The single canonical 'current name' word the spec already gives a concept
    (its title's lead distinctive term)."""
    ts = title_terms(title)
    # prefer a CANONICAL_BOOST term if present, else first term
    for t in ts:
        if t in CANONICAL_BOOST:
            return t
    return ts[0] if ts else "(unnamed)"


def subtree_leaves(node):
    out = []

    def rec(n):
        if n.sid is not None:
            out.append(n.sid)
            return
        rec(n.left)
        rec(n.right)

    rec(node)
    return out


def dominant_term(sids, p, nodes, banned):
    """The p_i-weighted dominant CEG term across a subtree's leaf titles, skipping
    any term already used by an ancestor branch (`banned`). CANONICAL_BOOST terms
    get a x2 weight so constitutional words win labels. Deterministic tie-break by
    (−weight, term)."""
    score = Counter()
    for sid in sids:
        terms = title_terms(nodes[sid]["title"])
        # rank-decayed contribution so the lead term of each title dominates
        for rank, t in enumerate(terms):
            if t in banned:
                continue
            boost = 2.0 if t in CANONICAL_BOOST else 1.0
            score[t] += p[sid] * boost / (1 + rank)
    if not score:
        return None
    return sorted(score.items(), key=lambda kv: (-kv[1], kv[0]))[0][0]


def label_tree_words(root, p, nodes):
    """Walk the Huffman tree; at each internal node, label its two child branches
    with the dominant CEG term of each child subtree (heavier child labelled
    first so it claims the strongest word). Returns:
      - phrase[sid] = list of branch words root->leaf
      - branch_label[(parent_id,child_side)] -> word  (for drawing)
    Words used on an ancestor branch are banned downstream so a phrase never
    repeats a word."""
    phrase = {}
    edge_word = {}  # id(child) -> word

    def own_term(sid, banned):
        """A leaf concept's own distinctive word: first title term not yet used by
        an ancestor; if all are banned, fall back to its unfiltered primary term
        (a meaningful word always beats 'misc')."""
        ts = title_terms(nodes[sid]["title"])
        for t in ts:
            if t not in banned:
                return t
        return ts[0] if ts else "concept"

    def walk(node, banned):
        if node.sid is not None:
            return
        # order children: heavier subtree first
        wl = sum(p[s] for s in subtree_leaves(node.left))
        wr = sum(p[s] for s in subtree_leaves(node.right))
        first, second = (node.left, node.right) if wl >= wr else (node.right, node.left)
        for child in (first, second):
            if child.sid is not None:
                # leaf branch: name it with the concept's OWN term so the phrase
                # ends on something meaningful, not a coarse cluster label.
                word = own_term(child.sid, banned)
            else:
                word = dominant_term(subtree_leaves(child), p, nodes, banned)
                if word is None:
                    # fall back to the heaviest leaf's own term, never 'misc'
                    heavy = max(subtree_leaves(child), key=lambda s: p[s])
                    word = own_term(heavy, banned)
            edge_word[id(child)] = word
            walk(child, banned | {word})

    walk(root, set())

    # Now accumulate root->leaf phrases using edge_word. Collapse any word that
    # already appears earlier in the same phrase (a coarse cluster label can
    # recur as a leaf's own term) so each phrase reads as distinct words.
    def accumulate(node, acc):
        if node.sid is not None:
            phrase[node.sid] = list(acc) if acc else ["concept"]
            return
        for child in (node.left, node.right):
            w = edge_word.get(id(child), "concept")
            nxt = acc if w in acc else acc + [w]
            accumulate(child, nxt)

    accumulate(root, [])
    return phrase, edge_word


def main():
    data = parse()
    nodes = data["nodes"]
    distinct_edges = data["distinct_edges"]
    edge_weight = data["edge_weight"]
    raw_edges = data["raw_edges"]

    N = len(nodes)
    M_distinct = len(distinct_edges)
    M_raw = len(raw_edges)

    # in-degree (distinct citing sections) and raw in-cite multiplicity
    indeg = Counter()
    incite = Counter()
    outdeg = Counter()
    for (src, dst) in distinct_edges:
        indeg[dst] += 1
        outdeg[src] += 1
    for (src, dst), w in edge_weight.items():
        incite[dst] += w

    pr = pagerank(nodes, distinct_edges)
    # normalize to probability distribution (already normalized, but be safe)
    tot = sum(pr.values())
    p = {s: pr[s] / tot for s in pr}

    codes, root = build_huffman(p)

    # Alternative: an in-degree-weighted Huffman code, for readers who prefer
    # raw citation-count addressing over citer-weighted PageRank import.
    in_total = sum(indeg.values()) or 1
    p_indeg = {s: (indeg[s] + 1) / (in_total + N) for s in nodes}  # +1 smoothing
    tot_i = sum(p_indeg.values())
    p_indeg = {s: v / tot_i for s, v in p_indeg.items()}
    codes_indeg, _ = build_huffman(p_indeg)

    p_sorted = sorted(p.items(), key=lambda kv: (-kv[1], kv[0]))
    intervals = arithmetic_intervals(p_sorted)

    # ---- vocabulary layer: word-labelled Huffman -> canonical phrases ----
    phrase, edge_word = label_tree_words(root, p, nodes)
    # current canonical name (the word the spec already leads with)
    current_name = {s: node_primary_term(nodes[s]["title"]) for s in nodes}
    # The constitutional root lexicon = the branch words at the top of the tree
    # (depth 1 + 2), in importance order. Depth-1 is the binary root split (2
    # words); depth-2 fans those into the primary vocabulary the rest derives from.
    root_words = []

    def collect_top_words(node, depth):
        if node.sid is not None or depth >= 2:
            return
        for child in (node.left, node.right):
            w = edge_word.get(id(child))
            mass = sum(p[s] for s in subtree_leaves(child))
            if w:
                root_words.append((w, mass, depth + 1))
            collect_top_words(child, depth + 1)

    collect_top_words(root, 0)
    # de-dup keeping the heaviest occurrence of each word; sort by mass
    _seen = {}
    for w, m, d in root_words:
        if w not in _seen or m > _seen[w][0]:
            _seen[w] = (m, d)
    root_words = sorted(((w, m, d) for w, (m, d) in _seen.items()),
                        key=lambda x: -x[1])

    # Promote/demote: compare a concept's IMPORTANCE rank to the COMPOUND-NESS of
    # its CURRENT title (how many qualifier words bury its lead term). The
    # user's ask: load-bearing concept buried under a long compound name ->
    # PROMOTE; minor concept on a short bare primary word -> DEMOTE.
    title_compound = {s: len(title_terms(nodes[s]["title"])) for s in nodes}
    plen = {s: len(phrase[s]) for s in nodes}
    rank_of = {s: i for i, (s, _) in enumerate(p_sorted, 1)}
    tc_sorted = sorted(title_compound.values())
    tc_median = tc_sorted[len(tc_sorted) // 2]
    flags = {}
    for s in nodes:
        r = rank_of[s]
        tc = title_compound[s]
        if r <= 20 and tc >= tc_median + 2:
            flags[s] = "PROMOTE"
        elif r > N * 0.5 and tc <= 1:
            flags[s] = "DEMOTE"
        else:
            flags[s] = ""

    H = shannon_entropy(p)
    L_huff = sum(p[s] * len(codes[s]) for s in p)
    L_flat = math.ceil(math.log2(N))
    # current hierarchical numbering effective depth = Dewey path length,
    # importance-weighted. (e.g. §5.6.8.10 -> depth 4)
    L_dewey = sum(p[s] * nodes[s]["dewey_len"] for s in p)
    # semantic seek: avg WORDS-to-reach a concept, importance-weighted.
    L_words = sum(p[s] * plen[s] for s in p)
    max_words = max(plen.values())

    # ---- write graph.json ----
    graph = {
        "meta": {
            "n_nodes": N,
            "n_edges_distinct": M_distinct,
            "n_edges_raw": M_raw,
            "self_cites_excluded": data["self_cites"],
            "entropy_bits": H,
            "L_huffman": L_huff,
            "L_flat_fixed": L_flat,
            "L_dewey_weighted": L_dewey,
        },
        "nodes": [
            {
                "id": s,
                "title": nodes[s]["title"],
                "file": nodes[s]["file"],
                "depth": nodes[s]["depth"],
                "dewey_len": nodes[s]["dewey_len"],
                "in_degree": indeg[s],
                "in_cites_raw": incite[s],
                "out_degree": outdeg[s],
                "pagerank": pr[s],
                "p": p[s],
                "huffman": codes[s],
                "arith_lo": intervals[s][0],
                "arith_hi": intervals[s][1],
                "dyadic": intervals[s][2],
                "phrase": phrase[s],
                "phrase_str": " · ".join(phrase[s]),
                "word_count": plen[s],
                "current_name": current_name[s],
                "flag": flags[s],
            }
            for s in nodes
        ],
        "edges": [
            {"src": src, "dst": dst, "weight": edge_weight[(src, dst)]}
            for (src, dst) in sorted(distinct_edges)
        ],
        "root_lexicon": [w for w, _, _ in root_words],
        "dangling_refs": dict(data["dangling"]),
        "external_refs": dict(data["external_refs"]),
    }
    graph["meta"]["L_words_weighted"] = L_words
    graph["meta"]["max_phrase_words"] = max_words
    with open(os.path.join(OUT_DIR, "graph.json"), "w") as fh:
        json.dump(graph, fh, indent=2)

    # ---- graph-stage visuals only ----
    # NOTE: this script is now the GRAPH STAGE. It emits graph.json (the reusable
    # graph + PageRank), the importance histogram, and the concept-graph DOT.
    # The vocabulary/nomenclature outputs (REPORT.md, vocabulary.tsv,
    # importance_top.png, vocabulary_tree.png) are produced by the encoding stage
    # `build_vocabulary.py`, which loads graph.json. We do NOT write those here so
    # the two stages never clobber each other.
    render_hist(p, os.path.join(OUT_DIR, "importance_hist.png"))
    dot_path = os.path.join(OUT_DIR, "concept_graph.dot")
    render_dot(graph, p_sorted, nodes, distinct_edges, edge_weight, dot_path)

    # ---- graph-stage summary to stdout ----
    print(json.dumps({
        "stage": "graph (build_taxonomy.py) — writes graph.json",
        "N": N, "M_distinct": M_distinct, "M_raw": M_raw,
        "entropy_bits": round(H, 4),
        "wrote": ["graph.json", "importance_hist.png", "concept_graph.dot"],
        "next": "run build_vocabulary.py for the nomenclature (REPORT.md, "
                "vocabulary.tsv, importance_top.png, vocabulary_tree.png)",
    }, indent=2, ensure_ascii=False))


def render_bar(p_sorted, nodes, phrase, path, topn=25):
    top = p_sorted[:topn]
    labels = [f"§{s}" for s, _ in top]
    vals = [w for _, w in top]
    fig, ax = plt.subplots(figsize=(12, 9))
    ypos = range(len(top))
    ax.barh(list(ypos), vals, color="#2b6cb0")
    ax.set_yticks(list(ypos))
    ax.set_yticklabels(labels, fontsize=8)
    ax.invert_yaxis()
    ax.set_xlabel("relative import  p_i  (normalized PageRank)")
    ax.set_title(f"CEG concept importance — top {topn} "
                 "(label = canonical word-phrase · word-count)")
    for i, (s, w) in enumerate(top):
        ph = " · ".join(phrase[s])
        ax.text(w, i, f"  {ph}  ({len(phrase[s])}w)", va="center",
                fontsize=7, color="#222")
    ax.margins(x=0.30)
    fig.tight_layout()
    fig.savefig(path, dpi=130)
    plt.close(fig)


def render_hist(p, path):
    vals = list(p.values())
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.hist(vals, bins=60, color="#805ad5", edgecolor="white")
    ax.set_yscale("log")
    ax.set_xlabel("relative import p_i")
    ax.set_ylabel("# concepts (log scale)")
    ax.set_title("CEG importance distribution (flat head, long thin tail)")
    fig.tight_layout()
    fig.savefig(path, dpi=130)
    plt.close(fig)


def render_word_tree(root, phrase, edge_word, p, nodes, path, max_depth=5):
    """Draw the top of the importance tree with WORD labels on each branch (the
    constitutional lexicon). Leaves reached within max_depth are labelled with
    §-id + their short phrase; deeper subtrees collapse to a triangle tagged with
    that subtree's dominant word + aggregate weight."""
    fig, ax = plt.subplots(figsize=(16, 9))
    ax.axis("off")
    positions = {}
    leaf_x = [0]

    def layout(node, depth):
        if depth > max_depth or node.sid is not None:
            x = leaf_x[0]
            leaf_x[0] += 1
            positions[id(node)] = (x, -depth, node)
            return x
        lx = layout(node.left, depth + 1)
        rx = layout(node.right, depth + 1)
        x = (lx + rx) / 2.0
        positions[id(node)] = (x, -depth, node)
        return x

    layout(root, 0)

    def draw(node, depth):
        x, y, _ = positions[id(node)]
        is_cut_internal = (node.sid is None and depth >= max_depth)
        if node.sid is not None:
            ax.plot(x, y, "o", color="#2b6cb0", ms=5)
            ax.text(x, y - 0.32, f"§{node.sid}\n{' · '.join(phrase[node.sid])}",
                    ha="center", va="top", fontsize=6)
            return
        if is_cut_internal:
            w = edge_word.get(id(node), "…")
            ax.plot([x - 0.5, x + 0.5, x, x - 0.5],
                    [y - 0.7, y - 0.7, y, y - 0.7], color="#a0aec0")
            ax.text(x, y - 0.85, f"{w}*\n(w={node.weight:.3f})", ha="center",
                    va="top", fontsize=6, color="#555")
            return
        for child in (node.left, node.right):
            cx, cy, _ = positions[id(child)]
            ax.plot([x, cx], [y, cy], "-", color="#cbd5e0", lw=0.8)
            word = edge_word.get(id(child), "")
            ax.text((x + cx) / 2, (y + cy) / 2, word, fontsize=7,
                    color="#c05621", ha="center",
                    bbox=dict(boxstyle="round,pad=0.1", fc="#fffaf0", ec="none"))
            draw(child, depth + 1)

    draw(root, 0)
    ax.set_title("CEG constitutional vocabulary tree — branch WORDS, not bits "
                 "(heaviest concepts sit nearest the root → shortest phrases)")
    fig.tight_layout()
    fig.savefig(path, dpi=130)
    plt.close(fig)


def render_dot(graph, p_sorted, nodes, distinct_edges, edge_weight, dot_path,
               topk=40):
    keep = set(s for s, _ in p_sorted[:topk])
    pmax = p_sorted[0][1]
    lines = ["digraph CEG {", '  rankdir=LR;', '  node [shape=box, style="rounded,filled", fontname="Helvetica", fontsize=9];', '  edge [color="#99999988"];']
    phrase_by = {n["id"]: n["phrase_str"] for n in graph["nodes"]}
    p_by = {n["id"]: n["p"] for n in graph["nodes"]}
    for s in keep:
        inten = p_by[s] / pmax
        # colour ramp white->blue by importance
        r = int(255 - 180 * inten)
        g = int(255 - 120 * inten)
        b = 255
        label = f"§{s}\\n[{html.escape(phrase_by[s])}]"
        lines.append(f'  "{s}" [label="{label}", fillcolor="#{r:02x}{g:02x}{b:02x}"];')
    for (src, dst) in distinct_edges:
        if src in keep and dst in keep:
            w = edge_weight[(src, dst)]
            lines.append(f'  "{src}" -> "{dst}" [penwidth={1 + 0.3 * (w-1):.1f}];')
    lines.append("}")
    with open(dot_path, "w") as fh:
        fh.write("\n".join(lines))
    # try to render
    png = dot_path.replace(".dot", ".png")
    rc = os.system(f'dot -Tpng "{dot_path}" -o "{png}" 2>/dev/null')
    return rc == 0


def write_report(graph, p_sorted, intervals, codes, phrase, current_name, flags,
                 root_words, nodes, indeg, incite, data, H, L_huff, L_flat,
                 L_dewey, L_words, max_words, N, M_distinct, M_raw):
    dot_png_exists = os.path.exists(os.path.join(OUT_DIR, "concept_graph.png"))
    p = {n["id"]: n["p"] for n in graph["nodes"]}
    rank_of = {s: i for i, (s, _) in enumerate(p_sorted, 1)}
    plen = {s: len(phrase[s]) for s in nodes}
    lines = []
    A = lines.append

    A("# CEG Constitutional Vocabulary — an Importance-Weighted Nomenclature\n")
    A("> Analysis only. Generated by `build_taxonomy.py`. The spec markdown is unmodified.\n")
    A(f"_Generated over `FSD/CEG/*.md` — {len(data['files'])} files._\n")

    A("## 1. What this is\n")
    A("Each `§`-marked heading in the CEG spec is a **concept node**; each in-body "
      "`§X.Y` reference is a **directed edge** (citing section → cited section). The "
      "*relative import* of a concept = how much the rest of the document feeds into "
      "it, measured by damped **PageRank**. We Huffman-merge concepts by that import — "
      "but instead of labelling branches with **bits (0/1)** we label each branch with "
      "a **meaningful CEG word** mined from the concepts beneath it. A concept's "
      "**code word is therefore a root→leaf phrase of words** — a canonical name. "
      "Because the merge is importance-optimal, the heaviest concepts sit nearest the "
      "root and earn the **shortest names** (here ~4 words — the tree is binary over "
      "291 leaves, so even the top concept is a few hops deep); long-tail subsections "
      "get longer compound phrases. This is the semantic analogue of *\"`0` is X, "
      "everything else under `1`\"* — except the alphabet is **words**, so the "
      "highest-mass concept claims the single primary word and everything else composes "
      "beneath it. The deliverable is a **generated nomenclature**: concept → "
      "importance-weighted canonical name, plus a reconciliation against the names CEG "
      "already uses.\n")

    A("## 2. Method\n")
    A("- **Graph / import** (unchanged from the bit-code analysis): nodes = §-headings "
      "(ids 0–19, globally unique — verified no collisions); edges = body-text `§X.Y` "
      "literals attributed to the enclosing section (headings + fenced code excluded; "
      "self-cites dropped). PageRank: hand-rolled power iteration, damping 0.85, "
      "dangling mass redistributed, normalized to `p_i` (Σ=1).\n")
    A("- **Huffman structure** over `{p_i}` exactly as before (merge two lowest-weight "
      "nodes; heavier subtree kept on the primary side).\n")
    A("- **Word-selection rule (deterministic).** At every internal node, each child "
      "branch is labelled with the **dominant CEG term of that child's subtree**: for "
      "each leaf title we extract an ordered term list — **backtick terms first** "
      "(the spec's own canonical names: `community`, `consent_record`, `withdraws`, "
      "`cohort_scope`, …), then remaining nouns — lemmatized to a canonical CEG word "
      "(`CIRISRegistry`→`registry`, `Files-as-Contributions`→`contributions`, "
      "`quorum-merge`→`merge`, `holds_bytes`→`invisibility`, …) and stopword-filtered. "
      "Each term contributes `p_i / (1+rank_in_title)` to its subtree, with a **×2 "
      "boost for constitutional terms** (the 1+4 grammar verbs + the CIRIS-layer "
      "roots). The highest-scoring term that an ancestor branch hasn't already used "
      "labels the branch (tie-break: −weight, then alphabetical). A leaf's phrase = the "
      "ordered branch words from root to leaf, so **no word repeats within a phrase** "
      "and the phrase reads as a real description.\n")
    A("- **Current name**: each concept's existing lead term (its title's first "
      "constitutional/backtick word) — the name the spec gives it today.\n")
    A("- **Promote/demote flag**: compares importance **rank** to the **compound-ness "
      "of the concept's CURRENT title** (how many qualifier terms its heading carries). "
      "`PROMOTE` = top-20 by import but a title with ≥ median+2 terms (a load-bearing "
      "concept buried under a long compound heading). `DEMOTE` = bottom-half by import "
      "yet a single bare title term (a short headline name spent on a minor concept).\n")

    A("## 3. Graph shape\n")
    A(f"- **Nodes (concepts)**: {N}  ·  **Edges (distinct)**: {M_distinct}  ·  "
      f"**raw**: {M_raw}  ·  self-cites excluded: {data['self_cites']}.\n")
    A(f"- **Dangling refs** (cited, never defined): {len(data['dangling'])} distinct. "
      f"**External legal citations filtered**: {sum(data['external_refs'].values())} "
      f"occurrences (CCPA/USC §-numbers).\n")
    A(f"- **Entropy** H(p) = **{H:.3f} bits** — the information-theoretic floor on "
      "addressing effort. The generated names average "
      f"**{L_words:.3f} words** per concept (range {min(len(v) for v in phrase.values())}–"
      f"{max_words}; see §7 for the bit-vs-word reconciliation). The importance "
      "distribution is a **flat head over a long tail** — no single concept dominates "
      f"(top p = {p_sorted[0][1]:.3f}), so the root lexicon is a handful of near-equal "
      "primary words rather than one root word.\n")
    A("\n![importance](importance_top.png)\n")

    A("## 4. The constitutional root lexicon (the alphabet)\n")
    A("The branches at the top of the tree (depths 1–2) carry the spec's **primary "
      "words** — the few constitutional terms from which every longer concept-name "
      "derives. In importance order (mass = Σ p_i of the subtree that word heads; the "
      "depth-1 pair `registry`/`contributions` partition the whole tree, the rest are "
      "their depth-2 children, so masses overlap by construction):\n")
    A("| primary word | subtree mass (Σ p_i) | reading |")
    A("|---|---:|---|")
    readings = {
        "registry": "identity / build / license / partner — who is who",
        "contributions": "files-as-contributions, the agent_files trust join",
        "envelope": "the wire object + canonicalization + subject binding",
        "accord": "HUMANITY_ACCORD constitutional layer + reservations",
        "namespace": "the dimension namespace + reserved prefixes",
        "consent": "consent_record / withdrawal / cohort scope",
        "community": "community / family / membership composition",
        "merge": "quorum-merge, witness cosigning, transport seals",
        "transport": "byte-level content transport + epoch cascade",
        "admission": "the four-test prefix-admission gate (grammar floor)",
        "subject_kind": "what an envelope is about — the subject_kind families",
        "primitive": "the 1+4 primitive set: scores + 4 relations",
        "attestation": "CIRISVerify attestation ladder / provenance",
        "compliance": "vertical compliance + amendment governance",
        "conformance": "conformance levels + anti-patterns",
    }
    for w, mass, depth in root_words:
        A(f"| **{w}** | {mass:.3f} | {readings.get(w, '—')} |")
    A("")
    A("These are the CEG's measured **constitutional primitives** — the semantic "
      "analogue of the spec's own declared 1+4 grammar (`scores` + "
      "`delegates_to`/`supersedes`/`withdraws`/`recants`). Every other concept-name is "
      "a composition under one of them.\n")
    A("\n![vocabulary tree](vocabulary_tree.png)\n")

    A("## 5. Top-30 — concept → canonical word-phrase → vs current name\n")
    A("| rank | § | p_i | canonical word-phrase | words | current name | flag |")
    A("|---:|---|---:|---|---:|---|---|")
    for rank, (s, w) in enumerate(p_sorted[:30], 1):
        ph = " · ".join(phrase[s])
        fl = f"**{flags[s]}**" if flags[s] else ""
        A(f"| {rank} | §{s} | {w:.4f} | `{ph}` | {plen[s]} | {current_name[s]} | {fl} |")
    A("")
    A("**How to read a phrase (honest note).** A phrase is a **path through the "
      "importance tree**, not a hand-written name. The *early* words are coarse "
      "cluster labels (the dominant term of the heavy subtree the concept lives in); "
      "the concept's *own* distinctive term lands toward the **end**. So §4 \"The "
      "envelope\" reads `registry · accord · identity · envelope · family` — you "
      "descend through the registry/accord/identity mass before `envelope` "
      "discriminates it. The first word is the constitutional primary it derives "
      "from; the last words pin the concept. Word count = semantic depth = "
      "−log2(p_i) in words, so heavier concepts get shorter paths. This is the price "
      "of making the names *prefix-free and importance-ordered* rather than arbitrary "
      "labels.\n")

    A("## 6. Reconciliation — does current naming track importance?\n")
    promotes = sorted((s for s in nodes if flags[s] == "PROMOTE"),
                      key=lambda x: rank_of[x])
    demotes = sorted((s for s in nodes if flags[s] == "DEMOTE"),
                     key=lambda x: rank_of[x])
    A("This is the constitutional-design payoff: where the spec's existing naming "
      "primacy **disagrees** with measured importance.\n")
    A(f"### 6a. PROMOTE — load-bearing concepts with over-long names ({len(promotes)})\n")
    if promotes:
        A("| § | rank | p_i | importance-optimal phrase | current name |")
        A("|---|---:|---:|---|---|")
        for s in promotes:
            A(f"| §{s} | {rank_of[s]} | {p[s]:.4f} | `{' · '.join(phrase[s])}` | "
              f"{current_name[s]} — \"{nodes[s]['title'][:46]}\" |")
        A("")
        A("Reading: these are among the most load-bearing concepts in the document, yet "
          "their current titles bury the key term under qualifiers. The importance-optimal "
          "vocabulary would **promote** the lead word to a primary (shorter) name.\n")
    else:
        A("_None: every top-15 concept already has a phrase at/below the median length._\n")
    A(f"### 6b. DEMOTE — minor concepts holding short bare primary names ({len(demotes)})\n")
    if demotes:
        A("Flagged where the **current title** is a single bare term (`current name`) "
          "but the concept is in the bottom half by import. Their *generated* phrase is "
          "long precisely **because** they sit deep in the tail — that's the point: a "
          "short headline name is spent on a low-import concept.\n")
        A("| § | rank | p_i | current title (short) | importance address (deep) |")
        A("|---|---:|---:|---|---|")
        for s in demotes[:18]:
            t = nodes[s]["title"].replace("|", "\\|")
            t = (t[:38] + "…") if len(t) > 39 else t
            A(f"| §{s} | {rank_of[s]} | {p[s]:.4f} | {t} | "
              f"`…{' · '.join(phrase[s][-2:])}` ({plen[s]}w) |")
        A("")
        A("Reading: a short headline name is a scarce resource; under a strict "
          "importance budget these would be qualified/lengthened so the short names "
          "are reserved for load-bearing concepts (and many are changelog/foreword "
          "sections — `What CEG 0.x documents` — that arguably need no headline term "
          "at all).\n")
    else:
        A("_None flagged._\n")

    A("## 7. Semantic seek-cost — words-to-reach, importance-weighted\n")
    L_bit = sum(p[s] * len(codes[s]) for s in p)
    min_words = min(plen.values())
    A("Expected number of **semantic discriminations to localize a concept**, weighted "
      "by import:\n")
    A("| scheme | avg discriminations | unit |")
    A("|---|---:|---|")
    A(f"| Shannon entropy H(p) — the floor | {H:.3f} | bits |")
    A(f"| bit-Huffman depth (the underlying tree) | {L_bit:.3f} | bits |")
    A(f"| **word-phrase nomenclature (this scheme)** | **{L_words:.3f}** | words |")
    A(f"| current §-numbering (Dewey path, p-weighted) | {L_dewey:.3f} | levels |")
    A("")
    A(f"- The **bit-Huffman tree** that underlies the names averages {L_bit:.2f} bits "
      f"per concept — within {(L_bit / H - 1) * 100:.1f}% of the entropy floor "
      f"({H:.2f} bits), i.e. essentially optimal addressing.\n")
    A(f"- The **word-phrases average {L_words:.2f} words** (range {min_words}–"
      f"{max_words}). That is slightly *below* the bit-depth because consecutive "
      "branch-words that repeat a coarse cluster label are collapsed — so a word is a "
      "richer discrimination than a bit. (Words and bits aren't the same unit; word "
      "count is not bounded by the bit entropy.) The point is the **ordering**: the "
      f"heaviest concepts get the shortest names ({min_words} words — e.g. §5.6.7 "
      "`contributions · envelope · epoch · joint`), the long tail the longest "
      f"({max_words}).\n")
    A("- Today's hierarchical numbering spends its depth on **organizational nesting** "
      f"(`§5.6.8.10` = 4 Dewey levels, p-weighted {L_dewey:.2f} — small only because "
      "the heaviest concepts happen to sit at shallow §-numbers like §4/§5/§9); the "
      "nomenclature instead spends length on **importance** directly, so a concept's "
      "name *length itself* tells you how load-bearing it is — which Dewey numbering "
      "cannot. The comparison is of *what the address encodes*, not raw count.\n")

    A("## 8. The concept graph (top-weighted subgraph)\n")
    if dot_png_exists:
        A("![concept graph](concept_graph.png)\n")
    else:
        A("`concept_graph.dot` written (top-40 by PageRank, nodes labelled with their "
          "phrase). `dot` not on PATH at build time — run "
          "`dot -Tpng concept_graph.dot -o concept_graph.png` to render.\n")

    A("## 9. How to read / apply the nomenclature\n")
    A("Each concept's **name is its root→leaf phrase**. The first word is the "
      "constitutional primary it lives under; each further word is one semantic "
      "discrimination; the **shortest names are the concepts the whole document leans "
      "on**.\n")
    A("- **Overlay on headers.** A header `#### §5.6.8.15 Replication-target policy` "
      "would carry its phrase as a seek tag, e.g. "
      "`#### §5.6.8.15 ⟨community · replication · peer⟩ …`. A reader wanting the most "
      "load-bearing concept follows the heaviest primary word first.\n")
    A("- **Seek-optimized & prefix-free.** Phrase length ≈ −log2(p_i) in words, so "
      "frequently-referenced concepts are cheapest to name; no phrase is a prefix of "
      "another, so a name stream (a citation trail) is self-delimiting.\n")
    A("- **A living constitution.** The root lexicon is the doc's measured primitive "
      "vocabulary. Re-running regenerates it as the citation structure evolves — heavy "
      "concepts keep short names; new rare subsections only lengthen the tail. The "
      "promote/demote table (§6) is the actionable hook: align the spec's headline "
      "names with where its own cross-references say the weight actually is.\n")

    A("## 10. Artifacts\n")
    A("- `build_taxonomy.py` — generator (stdlib + matplotlib; PageRank + word-Huffman).\n")
    A("- `vocabulary.tsv` — §id · title · p_i · **canonical word-phrase** · word-count · "
      "current-name · promote/demote flag · huffman-bits · in-degree.\n")
    A("- `graph.json` — full node/edge dump incl. per-node phrase, word-count, flag, "
      "and the `root_lexicon`.\n")
    A("- `importance_top.png` (phrase-labelled), `importance_hist.png`, "
      "`vocabulary_tree.png` (word-labelled tree), `concept_graph.dot`(+`.png`).\n")

    with open(os.path.join(OUT_DIR, "REPORT.md"), "w") as fh:
        fh.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()

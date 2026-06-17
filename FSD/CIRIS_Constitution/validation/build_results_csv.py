#!/usr/bin/env python3
"""
Generate results.csv: one row per ORIGINAL source chapter (CEG §-file or Accord Book),
pre-populated with the CC decimal range + section count it maps to, and blank result
columns for the per-chapter validators to fill. Reads the authoritative ../toc.tsv.
"""
import csv, os, re
from collections import defaultdict

HERE = os.path.dirname(__file__)
TOC = os.path.join(HERE, "..", "toc.tsv")

# source-file map for CEG chapters (for the validator to open)
CEG_FILES = {
    "§0": "00_conformance.md", "§1": "01_foundation.md", "§2": "02_grammar.md",
    "§3": "03_primitives.md", "§4": "04_envelope.md", "§5": "05_namespace.md",
    "§6": "06_relations.md", "§7": "07_reserved.md", "§8": "08_composition.md",
    "§9": "09_humanity_accord.md", "§10": "10_endpoints.md", "§11": "11_governance.md",
    "§12": "12_translation.md", "§13": "13_anti_patterns.md", "§14": "14_glossaries.md",
    "§15": "15_gaps.md", "§16": "16_references.md", "§17": "17_cadence.md",
    "§18": "18_interop.md", "§19": "19_holonomic.md",
}
ACCORD_BOOK = {  # Accord book number -> readable name (source: CIRISAgent/ACCORD.md)
    "0": "Book 0 — Genesis / the parable", "1": "Book I — Becoming an Ethical Entity",
    "2": "Book II — PDMA / WBD / Wise Authorities", "3": "Book III — Case Studies",
    "4": "Book IV — Obligations", "5": "Book V — Ethical Becoming",
    "6": "Book VI — Creation Ethics / Stewardship Tiers", "7": "Book VII — Use-of-Force Ethics",
    "8": "Book VIII — Sunset / De-commissioning / Sentience Safeguards",
}
# Accord curated-concept ids -> conceptual home (no book number in legacy_ref)
ACCORD_CONCEPT_BUCKET = {
    "M-1": "Book I — Meta-Goal & Principles", "P.": "Book I — Meta-Goal & Principles",
    "PDMA": "Book II — PDMA / WBD / Wise Authorities", "WBD": "Book II — PDMA / WBD / Wise Authorities",
    "WA": "Book II — PDMA / WBD / Wise Authorities", "fail": "Book I — Meta-Goal & Principles",
    "J": "Book IX — Coherence Mathematics (pending)", "F": "Book IX — Coherence Mathematics (pending)",
    "sigma": "Book IX — Coherence Mathematics (pending)", "ratchet": "Book IX — Coherence Mathematics (pending)",
    "A0": "Book VIII — Autonomy Tiers", "A1": "Book VIII — Autonomy Tiers", "A2": "Book VIII — Autonomy Tiers",
    "A3": "Book VIII — Autonomy Tiers", "A4": "Book VIII — Autonomy Tiers", "DCP": "Book VIII — Sunset",
}

def chapter_of(legacy):
    if legacy.startswith("§"):
        top = re.match(r"§(\d+)", legacy).group(1)
        return "ceg", f"§{top}"
    # Accord
    body = legacy.replace("Accord ", "")
    m = re.match(r"(\d+)\.", body)
    if m:
        return "accord", ACCORD_BOOK.get(m.group(1), f"Book {m.group(1)}")
    for k, v in ACCORD_CONCEPT_BUCKET.items():
        if body.startswith(k):
            return "accord", v
    return "accord", "Book I — Meta-Goal & Principles"

rows = list(csv.DictReader(open(TOC), delimiter="\t"))
groups = defaultdict(lambda: {"origin": "", "decs": [], "n": 0})
for r in rows:
    origin, chap = chapter_of(r["legacy_ref"])
    g = groups[chap]; g["origin"] = origin
    g["decs"].append([int(x) for x in r["decimal_id"].split(".")]); g["n"] += 1

def dec_range(decs):
    decs = sorted(decs)
    lo = ".".join(str(x) for x in decs[0]); hi = ".".join(str(x) for x in decs[-1])
    return lo if lo == hi else f"{lo}–{hi}"

# order: CEG §0..§19 then Accord books
def sortkey(item):
    chap, g = item
    if g["origin"] == "ceg":
        return (0, int(re.match(r"§(\d+)", chap).group(1)))
    return (1, chap)

cols = ["source_chapter", "origin", "source_file", "cc_decimal_range", "n_cc_sections",
        "C0_fidelity(PASS/FAIL)", "C1_byte_exact_CEG(PASS/FAIL/NA)", "C2_superior(YES/NO)",
        "readability(1-5)", "verdict(ACCEPT/ACCEPT-WITH-FIXES/REJECT)", "n_issues", "reviewer", "notes"]
with open(os.path.join(HERE, "results.csv"), "w", newline="") as f:
    w = csv.writer(f); w.writerow(cols)
    for chap, g in sorted(groups.items(), key=sortkey):
        src = CEG_FILES.get(chap, "CIRISAgent/ACCORD.md") if g["origin"] == "ceg" else "CIRISAgent/ACCORD.md"
        w.writerow([chap, g["origin"], src, dec_range(g["decs"]), g["n"],
                    "", "NA" if g["origin"] == "accord" else "", "", "", "", "", "", ""])
n = len(groups)
print(f"results.csv: {n} source-chapter rows "
      f"({sum(1 for _,g in groups.items() if g['origin']=='ceg')} CEG + "
      f"{sum(1 for _,g in groups.items() if g['origin']=='accord')} Accord)")
for chap, g in sorted(groups.items(), key=sortkey):
    print(f"  {chap:<42} {g['origin']:<7} {g['n']:>3} CC sections")

#!/usr/bin/env python3
"""
Emit the CEG *reader edition* as clean MARKDOWN (one file per source chapter) into
FSD/CEG/reader-md/. Same DE-EDITORIALIZATION prefilter as build_reader_pdf.py — strips
version history, provenance parentheticals, refrain sentences, the README changelog wall,
the §15.6 RC audit register, and the §16 lineage file — but keeps ALL wire-normative
content (canonical encodings, schemas, exact rules). This is the source the CIRIS
Constitution copy-migration works from: human-readable, lineage-free, byte-normative-intact.
"""
import re, os
from pathlib import Path

SPEC = Path(__file__).resolve().parent.parent           # FSD/CEG
OUT = SPEC / "reader-md"
OUT.mkdir(exist_ok=True)

FILES = ["README.md"] + sorted(f.name for f in SPEC.glob("[0-9][0-9]_*.md") if not f.name.startswith("16_"))

EDITORIAL_LINE = re.compile(
    r"(independent path|[A-Za-z]+(th|teenth|nth) path\.)"
    r"|wire-format lockdown preserved|lockdown is preserved"
    r"|^>\s*\*?This section is \*\*informative", re.I)
REFRAIN_SENT = re.compile(
    r"\*?\*?1\+4 (wire-format )?lockdown preserved\*?\*?:?[^.]*\.\s*"
    r"|Zero new structural primitives\.\s*"
    r"|[A-Z][a-z]+(th|teenth) (independent )?path( on [^.]*)?\.\s*", )
PROVENANCE = re.compile(r"\s*\(per \[[^\]]*\]\([^)]*\)(?:\s*\+\s*\[[^\]]*\]\([^)]*\))*\)")

def prefilter(text, fname):
    if fname == "README.md":
        out, skip = [], False
        for ln in text.split("\n"):
            if re.match(r"^\*\*0\.\d+ (is |folds|carries|adds)", ln):
                skip = True
            elif ln.startswith("## ") or ln.startswith("| ") or ln.startswith("#"):
                skip = False
            if not skip:
                out.append(ln)
        text = "\n".join(out)
    if fname.startswith("15_"):
        text = re.split(r"\n##+ §15\.6", text)[0]
    if fname.startswith("01_"):
        text = re.sub(
            r"\n1\. \*\*PRIOR_ART_SCAN.*?(?=\n\*\*Future extensions)",
            "\n*Sixteen independent design exercises each composed without a new structural "
            "primitive; the enumeration lives in the canonical working draft.*\n\n",
            text, flags=re.S)
    text = PROVENANCE.sub("", text)
    text = REFRAIN_SENT.sub("", text)
    kept = []
    for ln in text.split("\n"):
        if EDITORIAL_LINE.search(ln) and not ln.startswith(("#", "|")):
            if len(EDITORIAL_LINE.findall(ln)) and ("path" in ln.lower() or "lockdown" in ln.lower() or "informative" in ln.lower()):
                continue
        kept.append(ln)
    # strip the nav header/footer lines (first/last "[← ...] | ... | [Next ...]")
    out = "\n".join(kept)
    out = re.sub(r"^\[←[^\n]*\n", "", out)
    out = re.sub(r"\n---\n+\[←[^\n]*\|[^\n]*$", "\n", out)
    return out

n = 0
for fn in FILES:
    src = (SPEC / fn).read_text(encoding="utf-8")
    (OUT / fn).write_text(prefilter(src, fn), encoding="utf-8")
    n += 1
print(f"wrote {n} reader-markdown files to {OUT} (§16 dropped; editorial stripped; wire-normative intact)")

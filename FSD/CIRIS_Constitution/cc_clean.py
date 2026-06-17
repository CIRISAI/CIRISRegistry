#!/usr/bin/env python3
"""
clean_history(text) — strip version-history, RC/version annotations, issue refs, and
provenance from CEG/Accord prose so the CIRIS Constitution reads as clean, positive,
present-tense 1.0 statements. Removes ONLY editorial/historical scaffolding — never
normative content (rules, schemas, tables, numbers, field names are untouched).

Shared by build_reader_md.py (clean source) and the assembly step (clean output),
so the produced document is uniformly history-free regardless of copy path.
"""
import re

def clean_history(t: str) -> str:
    # provenance links: (per [..](..)) and (per [..](..) + [..](..))
    t = re.sub(r"\s*\((?:per|see)\s+\[[^\]]*\]\([^)]*\)(?:\s*\+\s*\[[^\]]*\]\([^)]*\))*\)", "", t)
    # version-tag parentheticals: (CEG 0.6 addition) · (normative, 1.0-RC25 — resolves CIRISRegistry#95)
    #   · (1.0-RC14) · (CEG 0.7 retcon) · (RECOMMENDED default; 1.0-RCx) — no nested parens
    t = re.sub(r"\s*\((?:normative|informative|RECOMMENDED|optional)?[,;]?\s*"
               r"(?:CEG\s+\d+\.\d+|1\.0-RC\d+)[^()]*\)", "", t)
    t = re.sub(r"\s*\(CEG\s+\d+\.\d+[^()]*\)", "", t)                 # any leftover (CEG 0.x ...)
    # bold version lead-ins: **CEG 0.6 addition.** / **... 1.0-RC25 ...** headers
    t = re.sub(r"\*\*CEG\s+\d+\.\d+\s+(?:addition|retcon|change)\.?\*\*\s*[—-]?\s*", "", t)
    t = re.sub(r"\*\*[^*\n]*\b1\.0-RC\d+\b[^*\n]*\*\*\s*[—-]?\s*", "", t)
    # "RESOLVED at 1.0-RCx:" blockquote notes (whole line)
    t = re.sub(r"^>\s*\*\*RESOLVED[^\n]*\n", "", t, flags=re.M)
    # issue references: CIRISRegistry#83 / CIRISVerify#64 / (per #95) / (#71 C5)
    t = re.sub(r"\s*\(?(?:per\s+|resolves\s+|ref\.?\s+)?CIRIS[A-Za-z]+#\d+(?:\s+[A-Z]\d+)?\)?", "", t)
    t = re.sub(r"\s*\(#\d+(?:\s+[A-Z]\d+)?\)", "", t)
    # naked inline version stamps left dangling: ", CEG 0.8," / " — 1.0-RC1 — "
    t = re.sub(r"[,;]?\s*\b(?:CEG\s+\d+\.\d+|1\.0-RC\d+)\b\s*[—-]?\s*(?=[.,;:)])", "", t)
    # tidy: collapse spaces, fix orphaned punctuation/empty parens left behind
    t = re.sub(r"\(\s*[,;—-]?\s*\)", "", t)
    t = re.sub(r"[ \t]{2,}", " ", t)
    t = re.sub(r"\s+([.,;:])", r"\1", t)
    t = re.sub(r"\n{3,}", "\n\n", t)
    return t

if __name__ == "__main__":
    import sys
    sys.stdout.write(clean_history(sys.stdin.read()))

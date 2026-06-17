#!/usr/bin/env python3
r"""
Human-first reader edition of the CEG spec -> ceg-1.0-rc29-reader.pdf.

Reuses the markdown->LaTeX converter from build_pdf.py, but applies a
DE-EDITORIALIZATION prefilter (strip version history, per-path narrative,
"lockdown preserved" refrains, provenance cross-refs, RC1 audit register,
lineage tables) and a cleaner, human-reading preamble. Deduplicates by
dropping the README changelog wall and the §16 lineage file.

Goal: more humans than machines. The canonical working draft (with full
lineage) stays in the source tree and in pdf/ceg-1.0-rc29.pdf.
"""
import re
from pathlib import Path
import build_pdf as B   # convert(), inline(), esc(), code_ascii(), NUC

D = Path(__file__).parent
SPEC = D.parent
VERSION = "1.0-RC29"

# ---- which files, in reading order. Drop §16 (references+lineage: mostly history). ----
FILES = ["README.md"] + sorted(f.name for f in SPEC.glob("[0-9][0-9]_*.md") if not f.name.startswith("16_"))

# ---------------------------------------------------------------- de-editorialization
EDITORIAL_LINE = re.compile(
    r"(independent path|[A-Za-z]+(th|teenth|nth) path\.)"           # "...Ninth path." / "independent path"
    r"|wire-format lockdown preserved|lockdown is preserved"
    r"|^>\s*\*?This section is \*\*informative", re.I)

REFRAIN_SENT = re.compile(
    r"\*?\*?1\+4 (wire-format )?lockdown preserved\*?\*?:?[^.]*\.\s*"
    r"|Zero new structural primitives\.\s*"
    r"|[A-Z][a-z]+(th|teenth) (independent )?path( on [^.]*)?\.\s*", )

PROVENANCE = re.compile(r"\s*\(per \[[^\]]*\]\([^)]*\)(?:\s*\+\s*\[[^\]]*\]\([^)]*\))*\)")

def prefilter(text, fname):
    # 1) README: drop the per-version changelog wall, keep intro + section index + human note
    if fname == "README.md":
        out, skip = [], False
        for ln in text.split("\n"):
            if re.match(r"^\*\*0\.\d+ (is |folds|carries|adds)", ln):   # "**0.14 is additive...**" wall
                skip = True
            elif ln.startswith("## ") or ln.startswith("| ") or ln.startswith("#"):
                skip = False
            if not skip:
                out.append(ln)
        text = "\n".join(out)
    # 2) §15: cut the §15.6 RC1 audit register (and everything after)
    if fname.startswith("15_"):
        text = re.split(r"\n##+ §15\.6", text)[0]
    # 3) §1.4: collapse the numbered path enumeration to one line
    if fname.startswith("01_"):
        text = re.sub(
            r"\n1\. \*\*PRIOR_ART_SCAN.*?(?=\n\*\*Future extensions)",
            "\n*Sixteen independent design exercises (consent, families, communities, "
            "streaming, payments, DNS-free addressing, settlement, ...) each composed without a new "
            "structural primitive; the enumeration lives in the canonical working draft.*\n\n",
            text, flags=re.S)
    # 4) global: strip provenance parentheticals + refrain sentences; drop pure-editorial lines
    text = PROVENANCE.sub("", text)
    text = REFRAIN_SENT.sub("", text)
    kept = []
    for ln in text.split("\n"):
        if EDITORIAL_LINE.search(ln) and not ln.startswith(("#", "|")):
            # drop a line that is *primarily* editorial narrative
            if len(EDITORIAL_LINE.findall(ln)) and ("path" in ln.lower() or "lockdown" in ln.lower() or "informative" in ln.lower()):
                continue
        kept.append(ln)
    return "\n".join(kept)

# ---------------------------------------------------------------- human preamble
PREAMBLE = r'''\documentclass[11pt]{article}
\usepackage[a4paper,margin=2.4cm]{geometry}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\usepackage{newunicodechar}
\usepackage{amssymb,amsmath}
\usepackage{longtable,array,booktabs}
\usepackage{listings}
\usepackage{xcolor}
\definecolor{accent}{HTML}{2a4d69}
\usepackage[hidelinks]{hyperref}
\usepackage{titlesec}
\titleformat{\section}{\LARGE\bfseries\color{accent}}{}{0pt}{}[\vspace{2pt}\hrule]
\titleformat{\subsection}{\large\bfseries\color{accent!85}}{}{0pt}{}
\titleformat{\subsubsection}{\normalsize\bfseries}{}{0pt}{}
\titlespacing*{\section}{0pt}{16pt}{8pt}
\lstset{basicstyle=\ttfamily\footnotesize,breaklines=true,columns=fullflexible,
        frame=leftline,framerule=1pt,rulecolor=\color{accent!40},
        backgroundcolor=\color{accent!4},xleftmargin=8pt,aboveskip=6pt,belowskip=6pt}
\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}
\linespread{1.05}
\renewcommand{\arraystretch}{1.25}
\usepackage{sectsty}
''' + B.nuc_lines + r'''
\title{\vspace{-1cm}\Huge\bfseries\color{accent}CEG\\[6pt]\LARGE The CIRIS Epistemic Grammar\\[14pt]
\normalsize\mdseries Version ''' + VERSION + r''' --- Reader Edition\\[2pt]
\itshape a signed, compositional graph language for claims, relationships, authority,\\
membership, consent, governance, addressing, and settlement across a decentralized network}
\author{}\date{2026-06-10}
\begin{document}
\maketitle\thispagestyle{empty}
\begin{quote}\itshape This is the human-reading edition: version history, per-path narrative, and
provenance cross-references are omitted for readability. The canonical working draft
(with full lineage) is the markdown source and \texttt{ceg-1.0-rc29.pdf}. Conformance is
judged against the normative wire surface only (\S0.1.1).\end{quote}
\clearpage
\tableofcontents
\clearpage
'''

if __name__ == "__main__":
    body = [PREAMBLE]
    for fn in FILES:
        md = (SPEC/fn).read_text(encoding="utf-8")
        body.append(B.convert(prefilter(md, fn)))
        body.append(r"\clearpage")
    body.append(r"\end{document}")
    (D/"ceg-1.0-rc29-reader.tex").write_text("\n".join(body), encoding="utf-8")
    print(f"wrote ceg-1.0-rc29-reader.tex ({len(FILES)} files, §16 + RC1 register + changelog wall stripped)")

#!/usr/bin/env python3
"""
Build the CIRIS Constitution as a PDF: Foreword + an explicit chapter-level Contents +
Part I-VIII -> LaTeX -> pdflatex. Front matter is deliberately minimal: title, the
Accord foreword (which sets the tone), and the table of contents -- nothing else.
Reuses the markdown->LaTeX converter from FSD/CEG/pdf/build_pdf.py.
"""
import re, csv, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "CEG" / "pdf"))
import build_pdf as B   # convert(), inline(), esc(), code_ascii(), NUC  (guarded; import is safe)

B.NUC.update({"é": r"\'e", "↑": r"$\uparrow$", "↓": r"$\downarrow$",
              "¶": r"\P{}", "Δ": r"$\Delta$", "σ": r"$\sigma$",
              "₂": r"\textsubscript{2}", "⅔": r"$2/3$",
              "⟨": r"$\langle$", "⟩": r"$\rangle$",
              "├": "+", "└": "+", "⟶": r"$\longrightarrow$"})
B.CODE_ASCII.update({"├": "+", "└": "+", "⟶": "-->"})
nuc_lines = "\n".join(r"\newunicodechar{%s}{%s}" % (k, v) for k, v in B.NUC.items())

VERSION = "0.4"
PARTS = sorted(HERE.glob("part_*.md"), key=lambda p: int(re.match(r"part_(\d+)_", p.name).group(1)))
PART_TITLE = {1: "Foundation", 2: "The Grammar", 3: "The Namespace", 4: "Composition & Governance",
              5: "Transport & Substrate", 6: "The Coherence Mathematics", 7: "Lifecycle & Stewardship",
              8: "Appendices"}

def prefilter(md):
    md = re.sub(r"</?(sub|sup)>", "", md)
    md = re.sub(r"<br\s*/?>", "  ", md)
    md = (md.replace("“", '"').replace("”", '"').replace("‘", "'").replace("’", "'"))
    return md

def contents_md():
    """Explicit chapter-level TOC (decimal + semantic + title), grouped by Part."""
    rows = list(csv.DictReader(open(HERE / "toc.tsv"), delimiter="\t"))
    rows.sort(key=lambda r: [int(x) for x in r["decimal_id"].split(".")])
    out, cur = ["# Contents\n"], None
    for r in rows:
        dec = r["decimal_id"]; part = int(dec.split(".")[0])
        if dec == "1.14" or dec.startswith("1.14."):  # the parable is now the Foreword
            continue
        if part != cur:
            cur = part; out.append(f"\n## Part {part} — {PART_TITLE[part]}\n")
        if dec.count(".") == 1:                      # chapter level only
            out.append(f"- **{dec}** `{r['semantic_id']}` — {r['title'].strip()}")
    return "\n".join(out)

PREAMBLE = r"""\documentclass[11pt]{article}
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
\lstset{basicstyle=\ttfamily\scriptsize,breaklines=true,columns=fullflexible,
        keepspaces=true,frame=leftline,framerule=1pt,rulecolor=\color{accent!40},
        backgroundcolor=\color{accent!4},xleftmargin=8pt,aboveskip=6pt,belowskip=6pt}
\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}
\linespread{1.05}
\renewcommand{\arraystretch}{1.25}
""" + nuc_lines + r"""
\title{\vspace{-1cm}\Huge\bfseries\color{accent}The CIRIS Constitution\\[10pt]
\normalsize\mdseries Version """ + VERSION + r"""\\[6pt]\itshape Stewarded by Eric Moore}
\author{}\date{}
\begin{document}
\maketitle\thispagestyle{empty}
"""

body = [PREAMBLE]
body.append(B.convert(prefilter((HERE / "FOREWORD.md").read_text(encoding="utf-8"))))   # foreword sets the tone
body.append(r"\clearpage")
body.append(B.convert(prefilter((HERE / "SCOPE_AND_DISCLAIMERS.md").read_text(encoding="utf-8"))))  # scope + disclaimers
body.append(r"\clearpage")
body.append(B.convert(contents_md()))                                                   # explicit contents
body.append(r"\clearpage")
for p in PARTS:
    body.append(B.convert(prefilter(p.read_text(encoding="utf-8"))))
    body.append(r"\clearpage")
body.append(B.convert(prefilter((HERE / "STEWARDSHIP.md").read_text(encoding="utf-8"))))
body.append(r"\end{document}")

out = HERE / f"ciris-constitution-{VERSION}.tex"
out.write_text("\n".join(body), encoding="utf-8")
print(f"wrote {out.name} (foreword + contents + {len(PARTS)} parts)")

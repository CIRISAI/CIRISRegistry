#!/usr/bin/env python3
"""
Build the CIRIS Constitution as a PDF: README + Part I–VIII -> LaTeX -> pdflatex.
Reuses the markdown->LaTeX converter from FSD/CEG/pdf/build_pdf.py (convert/inline/
esc/code_ascii/NUC). Run: python3 build_cc_pdf.py  (then pdflatex x3 over the .tex).
"""
import re, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "CEG" / "pdf"))
import build_pdf as B   # convert(), inline(), esc(), code_ascii(), NUC, nuc_lines  (guarded; import is safe)

# extend the unicode map for glyphs the CC uses that CEG's map lacked
B.NUC.update({"é": r"\'e", "↑": r"$\uparrow$", "↓": r"$\downarrow$"})
nuc_lines = "\n".join(r"\newunicodechar{%s}{%s}" % (k, v) for k, v in B.NUC.items())

VERSION = "0.1.1"
FILES = [HERE / "README.md"] + sorted(HERE.glob("part_*.md"), key=lambda p: int(re.match(r"part_(\d+)_", p.name).group(1)))

def prefilter(md):
    md = re.sub(r"</?(sub|sup)>", "", md)            # drop inline HTML the converter doesn't handle
    md = re.sub(r"<br\s*/?>", "  ", md)
    md = (md.replace("“", '"').replace("”", '"')   # curly quotes -> straight (also fixes code)
            .replace("‘", "'").replace("’", "'"))
    return md

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
\normalsize\mdseries Version """ + VERSION + r""" --- Reader Edition\\[10pt]
\itshape a unified constitution for the CIRIS epistemic web: the meta-goal M-1 and the
federation that serves it --- woven from the CIRIS Accord and the CIRIS Epistemic Grammar,\\
byte-exact to the wire, faithful to the ethics}
\author{}\date{}
\begin{document}
\maketitle\thispagestyle{empty}
\begin{abstract}\noindent
One document unifying the ethical constitution (the CIRIS Accord) and the wire grammar (CEG).
Structure is importance-derived --- M-1 at the apex; every section carries a decimal address and a
semantic name. Every wire-normative element (canonical encodings, signing preimages, the 1+4
attestation surface) is preserved byte-for-byte; every ethical principle keeps its force. Built by
faithful copy-migration and adversarially-validated consolidation (0 REJECT; byte-exact; judged
clearer than either source).
\end{abstract}
\tableofcontents\newpage
"""

body = [PREAMBLE]
for p in FILES:
    body.append(B.convert(prefilter(p.read_text(encoding="utf-8"))))
    body.append(r"\clearpage")
body.append(r"\end{document}")
out = HERE / f"ciris-constitution-{VERSION}.tex"
out.write_text("\n".join(body), encoding="utf-8")
print(f"wrote {out.name} ({len(FILES)} files)")

#!/usr/bin/env python3
"""Assemble an exhaustively-complete CEG 0.14 PDF from the markdown spec.
No pandoc available -> a focused markdown->LaTeX converter for this spec's
subset (headings, pipe tables, code fences, lists, inline bold/italic/code/links,
blockquotes), pdflatex + newunicodechar for the 42 special glyphs.

Front matter: title + a hand-written PQC-streaming model section (clean LaTeX
with the matplotlib figures + a TikZ stack diagram). Then the full spec.
"""
import re, glob, sys
from pathlib import Path

D = Path(__file__).parent
SPEC = D.parent

# ---- spec file order: README (overview) then 00..17 ----
FILES = ["README.md"] + sorted(f.name for f in SPEC.glob("[0-9][0-9]_*.md"))

# ---------------------------------------------------------------- escaping
SPECIAL = {'\\':r'\textbackslash{}','&':r'\&','%':r'\%','$':r'\$','#':r'\#',
           '_':r'\_','{':r'\{','}':r'\}','~':r'\textasciitilde{}','^':r'\textasciicircum{}'}
def esc(s):
    return ''.join(SPECIAL.get(c, c) for c in s)

# ASCII fallbacks for unicode INSIDE code blocks (verbatim can't use newunicodechar)
CODE_ASCII = {'§':'S','—':'--','→':'->','←':'<-','≤':'<=','≥':'>=','∈':'in','‖':'||',
   '²':'^2','³':'^3','⁴':'^4','⁶':'^6','✅':'[x]','↔':'<->','·':'.','×':'x','–':'-',
   '∧':'/\\','≠':'!=','│':'|','…':'...','≡':'==','±':'+-','─':'-','𝒞':'C','−':'-',
   '⊇':'>=','∩':'^','┐':'+','┘':'+','✓':'v','á':'a','⚠':'(!)','️':'','∃':'E',
   'ρ':'rho','⌈':'|','⌉':'|','∪':'U','≫':'>>','≈':'~=','🔴':'(R)','┌':'+','┴':'+',
   '┬':'+','├':'+','┤':'+','┼':'+','╔':'+','╗':'+','╚':'+','╝':'+','║':'|','═':'='}
def code_ascii(s):
    return ''.join(CODE_ASCII.get(c, c if ord(c)<128 else '?') for c in s)

def inline(s):
    """Inline markdown -> LaTeX. Operates on already-escaped-safe segments."""
    # links [text](url) -> text (drop url to avoid overflow; refs are self-describing)
    s = re.sub(r'\[([^\]]+)\]\([^)]*\)', r'\1', s)
    # inline code `x` -> \texttt{escaped}
    def code_sub(m): return r'\texttt{' + esc(m.group(1)) + '}'
    parts = re.split(r'(`[^`]*`)', s)
    out = []
    for p in parts:
        if p.startswith('`') and p.endswith('`') and len(p) >= 2:
            out.append(r'\texttt{' + esc(p[1:-1]) + '}')
        else:
            t = esc(p)
            t = re.sub(r'\*\*([^*]+)\*\*', r'\\textbf{\1}', t)
            t = re.sub(r'(?<!\*)\*([^*]+)\*(?!\*)', r'\\textit{\1}', t)
            out.append(t)
    return ''.join(out)

NAV = re.compile(r'^\[.←.*\]\(.*\)\s*\|.*\|.*\)\s*$|^\[←|Next:.*→')
def is_nav(line):
    return bool(re.search(r'\]\(\d\d_.*\.md\)', line)) and '|' in line

HEAD = {1:r'\section*{%s}',2:r'\subsection*{%s}',3:r'\subsubsection*{%s}',
        4:r'\paragraph{%s}\mbox{}\\',5:r'\subparagraph{%s}\mbox{}\\',6:r'\subparagraph{%s}\mbox{}\\'}

def convert(md):
    out, i, lines = [], 0, md.split('\n')
    n = len(lines)
    while i < n:
        line = lines[i]
        # code fence
        if line.lstrip().startswith('```'):
            i += 1; buf = []
            while i < n and not lines[i].lstrip().startswith('```'):
                buf.append(code_ascii(lines[i])); i += 1
            i += 1
            out.append(r'\begin{lstlisting}')
            out.extend(buf)
            out.append(r'\end{lstlisting}')
            continue
        # nav line -> drop
        if is_nav(line):
            i += 1; continue
        # heading
        m = re.match(r'^(#{1,6})\s+(.*)$', line)
        if m:
            lvl = len(m.group(1)); txt = inline(m.group(2))
            out.append(HEAD[lvl] % txt); out.append(''); i += 1; continue
        # table: header row '|...|' followed by separator '|---|'
        if line.strip().startswith('|') and i+1 < n and re.match(r'^\s*\|[\s:|-]+\|\s*$', lines[i+1]):
            header = [c.strip() for c in line.strip().strip('|').split('|')]
            ncol = len(header); i += 2; rows = []
            while i < n and lines[i].strip().startswith('|'):
                cells = [c.strip() for c in lines[i].strip().strip('|').split('|')]
                cells = (cells + ['']*ncol)[:ncol]
                rows.append(cells); i += 1
            w = round(16.0/max(ncol,1), 2)
            colspec = '|' + '|'.join([f'p{{{w}cm}}']*ncol) + '|'
            sz = r'\scriptsize' if ncol >= 4 else r'\footnotesize'
            out.append(r'{%s' % sz)
            out.append(r'\begin{longtable}{%s}\hline' % colspec)
            out.append(' & '.join(r'\textbf{%s}' % inline(c) for c in header) + r' \\ \hline\endhead')
            for r in rows:
                out.append(' & '.join(inline(c) for c in r) + r' \\ \hline')
            out.append(r'\end{longtable}}'); out.append(''); continue
        # blockquote
        if line.startswith('>'):
            buf = []
            while i < n and lines[i].startswith('>'):
                buf.append(inline(lines[i].lstrip('>').strip())); i += 1
            out.append(r'\begin{quote}\itshape ' + ' '.join(buf) + r'\end{quote}'); out.append('')
            continue
        # hrule
        if re.match(r'^\s*---+\s*$', line) or re.match(r'^\s*===+\s*$', line):
            i += 1; continue
        # unordered list
        if re.match(r'^\s*[-*]\s+', line):
            out.append(r'\begin{itemize}\setlength\itemsep{0pt}')
            while i < n and re.match(r'^\s*[-*]\s+', lines[i]):
                out.append(r'\item ' + inline(re.sub(r'^\s*[-*]\s+','',lines[i]))); i += 1
            out.append(r'\end{itemize}'); out.append(''); continue
        # ordered list
        if re.match(r'^\s*\d+\.\s+', line):
            out.append(r'\begin{enumerate}\setlength\itemsep{0pt}')
            while i < n and re.match(r'^\s*\d+\.\s+', lines[i]):
                out.append(r'\item ' + inline(re.sub(r'^\s*\d+\.\s+','',lines[i]))); i += 1
            out.append(r'\end{enumerate}'); out.append(''); continue
        # blank
        if not line.strip():
            out.append(''); i += 1; continue
        # paragraph
        out.append(inline(line)); out.append(''); i += 1
    return '\n'.join(out)

# ---------------------------------------------------------------- preamble
NUC = {
 '§':r'\S','—':'---','→':r'$\rightarrow$','←':r'$\leftarrow$','≤':r'$\leq$','≥':r'$\geq$',
 '∈':r'$\in$','‖':r'$\|$','²':r'\textsuperscript{2}','³':r'\textsuperscript{3}',
 '⁴':r'\textsuperscript{4}','⁶':r'\textsuperscript{6}','✅':r'[\checkmark]','↔':r'$\leftrightarrow$',
 '·':r'\textperiodcentered{}','×':r'$\times$','–':'--','∧':r'$\wedge$','≠':r'$\neq$',
 '│':'|','…':r'\ldots{}','≡':r'$\equiv$','±':r'$\pm$','─':'-','𝒞':r'$\mathcal{C}$',
 '−':'-','⊇':r'$\supseteq$','∩':r'$\cap$','┐':'+','┘':'+','✓':r'\checkmark{}','á':r"\'a",
 '⚠':r'[!]','️':'','∃':r'$\exists$','ρ':r'$\rho$','⌈':r'$\lceil$','⌉':r'$\rceil$',
 '∪':r'$\cup$','≫':r'$\gg$','≈':r'$\approx$','🔴':r'[\textbullet]',
}
nuc_lines = '\n'.join(r'\newunicodechar{%s}{%s}' % (k, v) for k, v in NUC.items())

PREAMBLE = r'''\documentclass[10pt]{article}
\usepackage[a4paper,margin=2cm]{geometry}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\usepackage{newunicodechar}
\usepackage{amssymb,amsmath}
\usepackage{longtable}
\usepackage{array}
\usepackage{graphicx}
\usepackage{listings}
\usepackage{xcolor}
\usepackage[hidelinks]{hyperref}
\usepackage{tikz}
\usetikzlibrary{positioning,arrows.meta,fit,backgrounds}
\usepackage{titlesec}
\titleformat{\section}{\Large\bfseries\color{black!80}}{}{0pt}{}
\lstset{basicstyle=\ttfamily\scriptsize,breaklines=true,columns=fullflexible,
        frame=single,framerule=0.2pt,rulecolor=\color{black!25},
        backgroundcolor=\color{black!3},xleftmargin=4pt,aboveskip=4pt,belowskip=4pt}
\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}
\renewcommand{\arraystretch}{1.15}
''' + nuc_lines + r'''
\title{\textbf{CEG --- The CIRIS Epistemic Grammar}\\[4pt]\large Version 0.14 (Public Working Draft) --- Exhaustively Complete Reference\\[2pt]\normalsize with the PQC Streaming Bandwidth/Lag Model}
\author{CIRIS Federation --- generated from \texttt{FSD/CEG/}}
\date{2026-06-06}
\begin{document}
\maketitle
\begin{abstract}\noindent
This document is the complete CEG 0.14 wire-format specification (the 1+4
minimal-and-adequate attestation grammar), assembled from the 18-section source
plus the version-history overview. It opens with a quantitative model of
PQC-native streaming video --- bandwidth and lag --- under CEG \S10.5, isolating
the scale ``long tail'' (the per-epoch O(N) key cascade and the transport-bound
realtime lag), then presents the normative spec in full.
\end{abstract}
\tableofcontents
\clearpage
'''

# ---------------------------------------------------------------- model section (hand-written, clean)
MODEL = r'''
\section*{0. PQC Streaming Video --- Bandwidth/Lag Model (``the toy'')}
\addcontentsline{toc}{section}{0. PQC Streaming Bandwidth/Lag Model}

An analytical, parametric model of CEG \S10.5 streaming under the mandatory PQC
envelope (\S10.5.3 \texttt{wrap\_algorithm:v2} = X25519+ML-KEM-768; \S10.5.2
AES-256-GCM chunk seal; hybrid Ed25519+ML-DSA-65 signatures). Every constant is
sourced from the spec or the relevant FIPS standard. \textbf{The single free
parameter is Reticulum transport RTT/throughput} --- CEG defers transport to RET,
so this is the one empirical input that turns the parametric model into concrete
numbers.

\subsection*{0.1 The CEG/RET vs TCP/IP stack}
\begin{center}
\begin{tikzpicture}[node distance=2pt,
  layer/.style={draw,rounded corners,minimum width=5.4cm,minimum height=0.72cm,align=center,font=\footnotesize},
  ceg/.style={layer,fill=blue!8}, ip/.style={layer,fill=black!5}]
\node[ip] (app2) {Application (HTTP/SMTP/RTP\ldots)};
\node[ip,below=of app2] (tls) {TLS / PKI (X.509 CA)};
\node[ip,below=of tls] (tcp) {TCP / UDP};
\node[ip,below=of tcp] (ip) {IP / DNS / BGP};
\node[ip,below=of ip] (link2) {Link (Ethernet/Wi-Fi/\ldots)};
\node[font=\small\bfseries,above=6pt of app2] {TCP/IP stack};
\node[ceg,right=2.4cm of app2] (ceg) {CEG --- 1+4 attestation grammar\\(\S3 + \S5 namespace)};
\node[ceg,below=of ceg] (deliv) {Delivery axis \S10.5\\(stream/epoch DEK, STH, receipts)};
\node[ceg,below=of deliv] (res) {Resolution \S8.13.1.1\\(community $\rightarrow$ member $\rightarrow$ dest)};
\node[ceg,below=of res] (ret) {Reticulum (RET)\\dest-hash, announce, Links};
\node[ceg,below=of ret] (link3) {Link (LoRa/TCP/serial/\ldots)};
\node[font=\small\bfseries,above=6pt of ceg] {CEG/RET stack};
\draw[-Latex] (ceg)--(deliv); \draw[-Latex] (deliv)--(res); \draw[-Latex] (res)--(ret); \draw[-Latex] (ret)--(link3);
\end{tikzpicture}
\end{center}

\noindent Trust is intrinsic (signed quorum), not bolted on: \textbf{PKI/CA $\rightarrow$ hybrid
signatures + founder-quorum}; \textbf{DNS $\rightarrow$ \texttt{resolve\_community}};
\textbf{A-record $\rightarrow$ signed \texttt{transport\_destination}}; \textbf{IP routing
$\rightarrow$ RET announce/path-request}.

\subsection*{0.2 Per-message PQC sizes (derived)}
\begin{center}\footnotesize
\begin{tabular}{|l|r|l|}\hline
\textbf{Object} & \textbf{Bytes} & \textbf{Composition}\\\hline
Hybrid epoch-DEK grant (v2) & 4{,}669 & X25519 32 + ML-KEM-768 ct 1088 + wrapped DEK 48 + hybrid sig 3373 + env 128\\\hline
Per-stream STH & 3{,}485 & root 32 + sizes 16 + log\_id 64 + hybrid sig 3373\\\hline
AES-GCM tag / 1 MiB chunk & 16 & 0.0015\% content overhead\\\hline
ML-DSA-65 sig / Ed25519 sig & 3{,}309 / 64 & FIPS 204 / RFC 8032\\\hline
ML-KEM-768 ciphertext & 1{,}088 & FIPS 203\\\hline
\end{tabular}
\end{center}

\subsection*{0.3 Findings}
\begin{enumerate}\setlength\itemsep{2pt}
\item \textbf{PQC is not the bandwidth bottleneck --- content fan-out is.} The
AES-256-GCM content overhead is 0.0015\% at a 1 MiB chunk. At scale the dominant
cost is unicast content replication (e.g.\ $\sim$2 Tbps for 1M viewers at 2 Mbps),
which is the CDN / broadcast-relay-tree problem (1.x), independent of crypto.
\item \textbf{The PQC ``long tail'' is the per-epoch O(N) key cascade under churn.}
Each member-removal forces an epoch rotation (\S10.5.3 forward-secrecy); the flat
1.0 cascade re-wraps the DEK to all remaining members at 4.56 KiB each. At
N=1M, churn=3600/hr this is $\sim$37 Gbps of \emph{key} traffic --- large in
absolute terms, but still $<$2\% of the content fan-out. The MLS-style
O($\log^2$N) tree (1.x) collapses it to $\sim$15 Mbps. Fig.~\ref{fig:cascade}.
\item \textbf{Lag is transport-bound, not crypto-bound.} Per-chunk AES seal/open
is $\sim$$\mu$s; the ML-KEM-768 DEK decap is $\sim$30 $\mu$s \emph{once per epoch}.
Glass-to-glass lag is dominated by chunk/segment duration, STH cadence (T=2s for
accountable broadcast), and \textbf{Reticulum Link RTT}. Fig.~\ref{fig:lag}.
\item \textbf{Join cost is dominated by RNS path-setup, not the handshake.} The
hybrid KEX (ML-KEM encap+decap) + signature verify are sub-millisecond; RNS path
establishment is the variable. Fig.~\ref{fig:join}.
\end{enumerate}

\subsection*{0.4 Do we have enough detail? (yes, parametrically)}
\textbf{Sufficient and pinned:} all crypto sizes (FIPS 203/204, RFC 8032), the
AES-GCM chunk overhead, cadence constants (K=64, T=2s, MAX\_CHUNKS/epoch=$2^{24}$,
1 MiB chunk), and cascade complexity (flat O(N) vs tree O($\log^2$N)).
\textbf{Parametric (RC1-7 unratified):} K, T, MAX\_CHUNKS --- the model is a
function of them. \textbf{The one missing empirical input:} Reticulum transport
characteristics (Link RTT, throughput, MTU, path-setup time) per interface ---
CEG defers transport to RET, so the lag term is a free parameter. Over IP-backed
RNS, assume $\sim$internet RTT + small RNS overhead (realtime video feasible);
over LoRa, seconds (realtime infeasible; store-and-forward only). \textbf{To go
parametric $\rightarrow$ concrete, measure RNS Link RTT/throughput on the target
interfaces.} The O($\log$N) tree (1.x) and SFU relay (1.x) are modelled with
standard complexity, not yet spec-detailed.

\begin{figure}[h]\centering
\includegraphics[width=0.78\textwidth]{fig_cascade.pdf}
\caption{Per-epoch PQC key cascade vs subscribers --- flat O(N) (1.0) is the long tail; the MLS tree (1.x) is the scale lever.}\label{fig:cascade}
\end{figure}
\begin{figure}[h]\centering
\includegraphics[width=0.70\textwidth]{fig_overhead.pdf}
\caption{PQC key+STH overhead as a share of egress --- content dominates; the key tail bites only at low bitrate $\times$ high churn $\times$ huge N.}\label{fig:overhead}
\end{figure}
\begin{figure}[h]\centering
\includegraphics[width=0.78\textwidth]{fig_lag.pdf}
\caption{Glass-to-glass lag vs Reticulum Link RTT (the free parameter). PQC adds $\sim\mu$s; transport sets the regime.}\label{fig:lag}
\end{figure}
\begin{figure}[h]\centering
\includegraphics[width=0.70\textwidth]{fig_join.pdf}
\caption{Stream/call join one-time cost --- RNS path setup dominates; crypto is sub-millisecond.}\label{fig:join}
\end{figure}
\clearpage
'''

# ---------------------------------------------------------------- assemble
body = [PREAMBLE, MODEL]
for fn in FILES:
    md = (SPEC/fn).read_text(encoding='utf-8')
    body.append(convert(md))
    body.append(r'\clearpage')
body.append(r'\end{document}')
(D/'ceg-0.14.tex').write_text('\n'.join(body), encoding='utf-8')
print('wrote ceg-0.14.tex (%d files)' % len(FILES))

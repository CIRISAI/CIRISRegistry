# CEG 0.14 PDF build

Exhaustively-complete CEG 0.14 PDF (the full 18-section spec + version overview)
with a front-matter **PQC streaming bandwidth/lag model** ("the toy").

## Reproduce
```bash
python3 pqc_streaming_model.py   # -> fig_*.pdf (model figures) + printed worked points
python3 build_pdf.py             # md (../*.md) -> ceg-0.14.tex
pdflatex ceg-0.14.tex && pdflatex ceg-0.14.tex   # 2 passes (TOC + figure refs)
```
Toolchain: `pdflatex` (TeX Live), `python3` + `numpy` + `matplotlib`. No pandoc
(a focused markdown->LaTeX converter for this spec's subset lives in `build_pdf.py`;
the 42 non-ASCII glyphs are mapped via `newunicodechar`).

## Files
- `pqc_streaming_model.py` — analytical PQC-streaming model; emits `fig_*.pdf`.
- `build_pdf.py` — converter + assembler -> `ceg-0.14.tex`.
- `ceg-0.14.pdf` — the deliverable (118 pp).

## The model in one line
PQC is not the streaming bottleneck (content fan-out is); the PQC "long tail" is
the per-epoch O(N) key cascade under churn (→ O(log²N) tree in 1.x); lag is
transport-bound (Reticulum Link RTT), not crypto-bound. The one missing empirical
input is RNS transport RTT/throughput — measure it to go parametric → concrete.

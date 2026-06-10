# CEG 0.17 PDF builds

Two editions, both generated from the markdown spec (`../*.md`):

- **`ceg-0.17.pdf`** — *exhaustive / reference edition*. Full 18-section spec +
  version overview + the front-matter PQC streaming bandwidth/lag model ("the toy")
  with 4 figures + a TikZ stack diagram. For implementers/machines.
- **`ceg-0.17-reader.pdf`** — *human reading edition*. The same normative spec,
  **de-editorialized + deduplicated**: version history, per-path narrative,
  "lockdown preserved" refrains, provenance cross-refs, the §15.6 RC1 audit
  register, and the §16 lineage file are stripped; serif, generous spacing,
  one-line "what CEG is" framing up front. For people.

## Reproduce
```bash
python3 pqc_streaming_model.py        # -> fig_*.pdf (model figures)
python3 build_pdf.py        && pdflatex ceg-0.17.tex        && pdflatex ceg-0.17.tex
python3 build_reader_pdf.py && pdflatex ceg-0.17-reader.tex && pdflatex ceg-0.17-reader.tex
```
Toolchain: `pdflatex` (TeX Live), `python3` + `numpy` + `matplotlib`. No pandoc —
`build_pdf.py` is a focused markdown→LaTeX converter; `build_reader_pdf.py` reuses it
and adds the de-editorialization prefilter. 42 non-ASCII glyphs via `newunicodechar`.

## Helper tools
- `pqc_streaming_model.py` — analytical PQC-streaming model (the figures).
- `crypto_bench.py` — per-op crypto benchmark (classical/symmetric measured; PQC from liboqs).
- `rns_transport_bench.py` — RNS Link RTT/throughput/path-setup harness (run on a CIRISEdge node).
- `concretize.py` — feeds a transport profile into the model → concrete lag/bandwidth.

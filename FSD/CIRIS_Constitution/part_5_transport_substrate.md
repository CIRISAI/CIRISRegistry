# Part V — Transport & Substrate

**CC decimal range** `5.x` · **35 concepts** · **page budget 11.0pp** (∝ importance) · [← master index](README.md)

> Byte-level content transport, structural invisibility, epoch keying, and delivery — how content moves without leaking what it is.

> **Status:** scaffold — headings + budgets fixed by the importance graph; bodies woven from `legacy_ref` sources in Phase 4.

## 5.1 `epoch` — Epoch keying + cascade (normative — D2 / D3; substrate-pending #142)
<sub>budget 1.34pp · import #12 · from **§10.5.3** (ceg)</sub>

## 5.2 `family` — Structural invisibility — `holds_bytes:sha256:*` suppression for `cohort_scope: self | family`
<sub>budget 1.04pp · import #19 · from **§10.1.4** (ceg)</sub>

## 5.3 `endpoint` — Endpoint shapes
<sub>budget 0.22pp · import #132 · from **§10** (ceg)</sub>

### 5.3.1 `witness` — STH cosigning + witness directory
<sub>budget 0.89pp · import #25 · from **§10.3** (ceg)</sub>

#### 5.3.1.1 `consistency-proof` — Consistency-proof requirement (normative; addresses CEG 0.1 distsys review)
<sub>budget 0.26pp · import #112 · from **§10.3.1** (ceg)</sub>

### 5.3.2 `transport` — Transport substrate for byte-level content
<sub>budget 0.5pp · import #58 · from **§10.1** (ceg)</sub>

#### 5.3.2.1 `holder` — Holder directory TTL + ContentMiss feedback
<sub>budget 0.47pp · import #63 · from **§10.1.2** (ceg)</sub>

#### 5.3.2.2 `consent-revocations` — Consent revocations are NOT local-tier-eligible (CEG 0.6 addition)
<sub>budget 0.38pp · import #80 · from **§10.1.3** (ceg)</sub>

#### 5.3.2.3 `merge` — Cross-region merge intents — CEG-declared per subject_kind (normative; CEG 1.0-RC2 addition)
<sub>budget 0.23pp · import #123 · from **§10.1.6** (ceg)</sub>

#### 5.3.2.4 `attestation-tier` — The attestation tier model — local-tier write, query, promotion (normative)
<sub>budget 0.14pp · import #194 · from **§10.1.5** (ceg)</sub>

##### 5.3.2.4.1 `authority-local` — Local-tier eligibility — the discriminator is *revocation authority*, not subject-set emptiness
<sub>budget 0.18pp · import #158 · from **§10.1.5.2** (ceg)</sub>

##### 5.3.2.4.2 `local` — Promotion — `local → federation` (the deferred-signature moment)
<sub>budget 0.15pp · import #180 · from **§10.1.5.3** (ceg)</sub>

##### 5.3.2.4.3 `two` — The two tiers
<sub>budget 0.15pp · import #181 · from **§10.1.5.1** (ceg)</sub>

###### 5.3.2.4.3.1 `admission-pqc` — The PQC half is MANDATORY at admission — no classical-only, no hybrid-pending accommodation (normative, CEG 1.0)
<sub>budget 0.15pp · import #179 · from **§10.1.5.1.1** (ceg)</sub>

##### 5.3.2.4.4 `persist-query` — Query — open-prefix dimensions, bounded operators (normative; resolves CIRISPersist#172 OQ-2)
<sub>budget 0.11pp · import #239 · from **§10.1.5.4** (ceg)</sub>

##### 5.3.2.4.5 `holistic` — For holistic analysis + modeling (informative)
<sub>budget 0.11pp · import #240 · from **§10.1.5.5** (ceg)</sub>

#### 5.3.2.5 `full-sha` — Full-SHA verification before consumption (normative)
<sub>budget 0.11pp · import #224 · from **§10.1.1** (ceg)</sub>

### 5.3.3 `transport-streaming` — Streaming transport, per-stream logs & delivery receipts (CEG 0.10 addition)
<sub>budget 0.45pp · import #67 · from **§10.5** (ceg)</sub>

#### 5.3.3.1 `stream` — Chunk seal + STREAM nonce (normative — V2 lock)
<sub>budget 0.5pp · import #56 · from **§10.5.2** (ceg)</sub>

#### 5.3.3.2 `composition` — Realtime group communication — composition (CEG 0.13 addition)
<sub>budget 0.46pp · import #66 · from **§10.5.8** (ceg)</sub>

##### 5.3.3.2.1 `namespace-realtime` — `codec_id` namespace — realtime A/V chunk codec discriminator (normative, 1.0-RC9 — ratifies [CIRISRegistry#84](https://github.com/CIRISAI/CIRISRegistry/issues/84))
<sub>budget 0.23pp · import #125 · from **§10.5.8.2** (ceg)</sub>

##### 5.3.3.2.2 `realtime` — Realtime non-A/V data streams (normative scope boundary)
<sub>budget 0.16pp · import #175 · from **§10.5.8.1** (ceg)</sub>

##### 5.3.3.2.3 `registry-wire` — `SealedAvChunk` wire layout (normative, 1.0-RC10 — absorbs CIRISEdge v4.0.0 per [CIRISRegistry#85](https://github.com/CIRISAI/CIRISRegistry/issues/85) §N)
<sub>budget 0.14pp · import #195 · from **§10.5.8.3** (ceg)</sub>

##### 5.3.3.2.4 `chunklayer` — `ChunkLayer` + `ReceiverLayerPolicy` — SVC layer model (normative, 1.0-RC10 — §85 §N.2)
<sub>budget 0.14pp · import #197 · from **§10.5.8.4** (ceg)</sub>

##### 5.3.3.2.5 `stream-double` — Double-seal + deterministic nonce derivation (normative, 1.0-RC10 — §85 §N)
<sub>budget 0.11pp · import #243 · from **§10.5.8.5** (ceg)</sub>

#### 5.3.3.3 `per-stream` — Per-stream log + stream-root (normative — V1 lock)
<sub>budget 0.36pp · import #82 · from **§10.5.1** (ceg)</sub>

#### 5.3.3.4 `liveness` — D6 liveness invariant — entitled vs reachable (normative)
<sub>budget 0.35pp · import #84 · from **§10.5.6** (ceg)</sub>

#### 5.3.3.5 `transport-edge` — Transport — Edge layer (normative — E1–E4 lock per [§15.6.2](15_gaps.md))
<sub>budget 0.29pp · import #101 · from **§10.5.5** (ceg)</sub>

#### 5.3.3.6 `delivery` — Delivery receipts (normative — D5 / V3 lock)
<sub>budget 0.27pp · import #105 · from **§10.5.4** (ceg)</sub>

#### 5.3.3.7 `normative` — Framing (normative)
<sub>budget 0.11pp · import #241 · from **§10.5.0** (ceg)</sub>

#### 5.3.3.8 `documents-what` — What CEG 0.10 documents
<sub>budget 0.11pp · import #242 · from **§10.5.7** (ceg)</sub>

### 5.3.4 `multi-steward` — Multi-steward + accord-holder discovery
<sub>budget 0.32pp · import #94 · from **§10.2** (ceg)</sub>

### 5.3.5 `registry-other` — Other Registry endpoints
<sub>budget 0.13pp · import #207 · from **§10.4** (ceg)</sub>

### 5.3.6 `common` — Common response shape
<sub>budget 0.11pp · import #223 · from **§10.0** (ceg)</sub>

#### 5.3.6.1 `envelope-error` — Error envelope
<sub>budget 0.33pp · import #92 · from **§10.0.1** (ceg)</sub>

# Part IV — Composition & Governance

**CC decimal range** `4.x` · **96 concepts** · **page budget 26.2pp** (∝ importance) · [← master index](README.md)

> How attestations compose into trust, how the federation governs itself, the amendment process, moderation, and the human halt-authority.

> **Status:** scaffold — headings + budgets fixed by the importance graph; bodies woven from `legacy_ref` sources in Phase 4.

## 4.1 `anti-pattern` — Anti-patterns
<sub>budget 1.65pp · import #7 · from **§13** (ceg)</sub>

### 4.1.1 `anti-pattern-delegation` — Delegation-laundering anti-pattern
<sub>budget 0.19pp · import #155 · from **§13.3** (ceg)</sub>

### 4.1.2 `pattern` — Discipline pattern
<sub>budget 0.15pp · import #183 · from **§13.5** (ceg)</sub>

### 4.1.3 `already-rejected` — Already-rejected wire additions
<sub>budget 0.13pp · import #205 · from **§13.1** (ceg)</sub>

### 4.1.4 `withdraws-arbitrage` — `withdraws` arbitrage
<sub>budget 0.11pp · import #226 · from **§13.4** (ceg)</sub>

### 4.1.5 `registry-rejections` — CEG 0.1 rejections (from CIRISRegistry#30 stress test)
<sub>budget 0.11pp · import #231 · from **§13.2** (ceg)</sub>

## 4.2 `accord` — The HUMANITY_ACCORD constitutional layer
<sub>budget 1.4pp · import #10 · from **§9** (ceg)</sub>

### 4.2.1 `authority` — Authority scope
<sub>budget 0.63pp · import #40 · from **§9.2** (ceg)</sub>

#### 4.2.1.1 `invocation` — Invocation canonical bytes (anti-replay; 0.1 scaffold)
<sub>budget 0.19pp · import #154 · from **§9.2.1** (ceg)</sub>

#### 4.2.1.2 `notify` — `notify` vs `CONSTITUTIONAL` — consumer-UI requirement
<sub>budget 0.11pp · import #227 · from **§9.2.2** (ceg)</sub>

### 4.2.2 `hardware-class` — Hardware-class taxonomy
<sub>budget 0.33pp · import #89 · from **§9.4** (ceg)</sub>

#### 4.2.2.1 `hardware-class-hardware` — Hardware-class self-assertion gap (acknowledged)
<sub>budget 0.13pp · import #200 · from **§9.4.1** (ceg)</sub>

### 4.2.3 `accord-holder` — The accord-holder triple
<sub>budget 0.16pp · import #172 · from **§9.1** (ceg)</sub>

### 4.2.4 `policy-concern` — Concern split — key material vs role-recognition policy
<sub>budget 0.11pp · import #301 · from **§9.3** (ceg)</sub>

### 4.2.5 `isn` — Why this isn't a Golden-Rule violation
<sub>budget 0.11pp · import #302 · from **§9.5** (ceg)</sub>

## 4.3 `wise-authority` — Designated Wise Authorities
<sub>budget 0.88pp · import #26 · from **Accord WA** (accord)</sub>

## 4.4 `composition-policies` — Composition policies
<sub>budget 0.34pp · import #88 · from **§8** (ceg)</sub>

### 4.4.1 `frickerian` — Frickerian discipline — consumer-policy norms
<sub>budget 0.35pp · import #86 · from **§8.3** (ceg)</sub>

### 4.4.2 `aggregation` — Aggregation semantics — opinionated defaults
<sub>budget 0.24pp · import #119 · from **§8.2** (ceg)</sub>

### 4.4.3 `reference` — Reference policies
<sub>budget 0.23pp · import #127 · from **§8.1** (ceg)</sub>

#### 4.4.3.1 `quorum` — Policy E — Locality-scaled quorum
<sub>budget 0.72pp · import #31 · from **§8.1.5** (ceg)</sub>

##### 4.4.3.1.1 `sub-quorum` — Sub-quorum fallback (0.1 scaffold; addresses CEG 0.1 distsys review)
<sub>budget 0.48pp · import #61 · from **§8.1.5.1** (ceg)</sub>

#### 4.4.3.2 `community-policy` — Policy M — Community membership composition
<sub>budget 0.65pp · import #36 · from **§8.1.13** (ceg)</sub>

##### 4.4.3.2.1 `community-three` — The three crypto tiers + the Community DEK cascade (normative — supersedes the 0.8 "no cascade" reasoning)
<sub>budget 0.45pp · import #68 · from **§8.1.13.3** (ceg)</sub>

##### 4.4.3.2.2 `community-forward` — Forward secrecy on community member removal (Option A — now applies)
<sub>budget 0.21pp · import #142 · from **§8.1.13.4** (ceg)</sub>

##### 4.4.3.2.3 `community-admission` — Community admission per `consensus_protocol` + `cohort_subkind`
<sub>budget 0.18pp · import #160 · from **§8.1.13.2** (ceg)</sub>

##### 4.4.3.2.4 `community-membership` — Community membership resolution
<sub>budget 0.18pp · import #163 · from **§8.1.13.1** (ceg)</sub>

###### 4.4.3.2.4.1 `deterministic` — Deterministic resolution + member→address resolution (NORMATIVE)
<sub>budget 0.31pp · import #95 · from **§8.1.13.1.1** (ceg)</sub>

##### 4.4.3.2.5 `admission-geographic` — Geographic-community admission flow (worked example)
<sub>budget 0.15pp · import #184 · from **§8.1.13.5** (ceg)</sub>

##### 4.4.3.2.6 `delivery-extension` — Delivery extension — `delivery_mode` × Policy M
<sub>budget 0.12pp · import #212 · from **§8.1.13.7** (ceg)</sub>

##### 4.4.3.2.7 `composition-8-1-13-6` — Composition with CEG 0.6 + 0.7
<sub>budget 0.11pp · import #298 · from **§8.1.13.6** (ceg)</sub>

#### 4.4.3.3 `policy` — Policy H — Tiered-Scope Composition (LIVE)
<sub>budget 0.55pp · import #47 · from **§8.1.8** (ceg)</sub>

##### 4.4.3.3.1 `supersedes` — Promotion via `supersedes` (worked pattern)
<sub>budget 0.45pp · import #69 · from **§8.1.8.1** (ceg)</sub>

#### 4.4.3.4 `family-policy` — Policy L — Self/family membership composition (CEG 0.7 addition)
<sub>budget 0.54pp · import #49 · from **§8.1.12** (ceg)</sub>

##### 4.4.3.4.1 `cascade` — Key-grant cascade (the at-rest encryption flow)
<sub>budget 0.48pp · import #60 · from **§8.1.12.4** (ceg)</sub>

##### 4.4.3.4.2 `admission-membership` — Membership-change admission per consensus_protocol
<sub>budget 0.26pp · import #107 · from **§8.1.12.3** (ceg)</sub>

###### 4.4.3.4.2.1 `quorum-absolute` — `quorum:M/N` is absolute-M (normative; per CIRISRegistry#52 + NodeCore#30)
<sub>budget 0.13pp · import #204 · from **§8.1.12.3.1** (ceg)</sub>

##### 4.4.3.4.3 `cohort` — The "Self at login" — app + agent co-self + partnered delegation (CEG 0.15, normative composition)
<sub>budget 0.26pp · import #108 · from **§8.1.12.7** (ceg)</sub>

###### 4.4.3.4.3.1 `canonicalization-signing` — Signing member sets (normative — the JCS contract Verify hybrid-signs; resolves CIRISVerify#63)
<sub>budget 0.23pp · import #124 · from **§8.1.12.7.1** (ceg)</sub>

##### 4.4.3.4.4 `family-membership` — Family membership resolution
<sub>budget 0.26pp · import #110 · from **§8.1.12.2** (ceg)</sub>

##### 4.4.3.4.5 `forward-secrecy` — Forward secrecy on member removal (Option A, recommended for v1)
<sub>budget 0.2pp · import #149 · from **§8.1.12.5** (ceg)</sub>

##### 4.4.3.4.6 `self-collective` — Self-collective resolution
<sub>budget 0.11pp · import #296 · from **§8.1.12.1** (ceg)</sub>

##### 4.4.3.4.7 `composition-subject` — Composition with CEG 0.6 subject_key_ids[]
<sub>budget 0.11pp · import #297 · from **§8.1.12.6** (ceg)</sub>

#### 4.4.3.5 `policy-cem` — Policy K — CEM composition
<sub>budget 0.53pp · import #50 · from **§8.1.11** (ceg)</sub>

##### 4.4.3.5.1 `composition-decay` — Decay-protocol stage composition (CIRISAgent CEM ANONYMOUS)
<sub>budget 0.29pp · import #102 · from **§8.1.11.5** (ceg)</sub>

##### 4.4.3.5.2 `deletion-sla` — Deletion-SLA watcher (substrate emission)
<sub>budget 0.27pp · import #106 · from **§8.1.11.3** (ceg)</sub>

##### 4.4.3.5.3 `composition-bilateral` — Bilateral pair composition (PARTNERED ceremony)
<sub>budget 0.21pp · import #133 · from **§8.1.11.4** (ceg)</sub>

##### 4.4.3.5.4 `multi-subject` — Multi-subject revocation (any-subject-binding)
<sub>budget 0.17pp · import #167 · from **§8.1.11.2** (ceg)</sub>

##### 4.4.3.5.5 `consent-effective` — Effective consent resolution (read path)
<sub>budget 0.16pp · import #176 · from **§8.1.11.1** (ceg)</sub>

##### 4.4.3.5.6 `policy-what` — What Policy K composes
<sub>budget 0.11pp · import #295 · from **§8.1.11.6** (ceg)</sub>

#### 4.4.3.6 `policy-attestation` — Policy I — Attestation-Ladder Composition
<sub>budget 0.41pp · import #73 · from **§8.1.9** (ceg)</sub>

#### 4.4.3.7 `contributions-policy` — Policy F — `agent_files` trust composition
<sub>budget 0.3pp · import #99 · from **§8.1.6** (ceg)</sub>

#### 4.4.3.8 `policy-direct` — Policy A — direct trust
<sub>budget 0.24pp · import #118 · from **§8.1.1** (ceg)</sub>

#### 4.4.3.9 `policy-lexical` — Policy D — Lexical-vulnerability-priority
<sub>budget 0.21pp · import #138 · from **§8.1.4** (ceg)</sub>

#### 4.4.3.10 `policy-trusted` — Policy J — Trusted-Publisher composition
<sub>budget 0.14pp · import #193 · from **§8.1.10** (ceg)</sub>

#### 4.4.3.11 `policy-trust` — Policy G — Trust-Fresh / Lighthouse
<sub>budget 0.11pp · import #228 · from **§8.1.7** (ceg)</sub>

#### 4.4.3.12 `policy-one` — Policy B — one-hop transitive
<sub>budget 0.11pp · import #299 · from **§8.1.2** (ceg)</sub>

#### 4.4.3.13 `policy-weighted` — Policy C — weighted graph (EigenTrust-style)
<sub>budget 0.11pp · import #300 · from **§8.1.3** (ceg)</sub>

### 4.4.4 `sovereign-registered` — Sovereign-Registered equivalence (wire-symmetric, policy-differentiated)
<sub>budget 0.2pp · import #146 · from **§8.4** (ceg)</sub>

## 4.5 `discipline` — Governance discipline
<sub>budget 0.21pp · import #140 · from **§11** (ceg)</sub>

### 4.5.1 `amendment` — Amendment process — federation Contribution + WA quorum + 1-of-6 sign-off
<sub>budget 0.85pp · import #27 · from **§11.2** (ceg)</sub>

#### 4.5.1.1 `axis` — Axis-vocabulary discipline
<sub>budget 0.7pp · import #32 · from **§11.2.1** (ceg)</sub>

#### 4.5.1.2 `meta-amendment` — Meta-amendment + entrenchment
<sub>budget 0.29pp · import #103 · from **§11.2.3** (ceg)</sub>

#### 4.5.1.3 `open-vocabulary` — Open-vocabulary collision rule
<sub>budget 0.11pp · import #225 · from **§11.2.2** (ceg)</sub>

### 4.5.2 `compliance` — Vertical compliance + subject-bearing dimension governance (CEG 0.6 addition; per CIRISRegistry#45)
<sub>budget 0.6pp · import #43 · from **§11.6** (ceg)</sub>

#### 4.5.2.1 `subject_kind-subject-3` — Subject-bearing dimension governance (normative)
<sub>budget 0.15pp · import #177 · from **§11.6.2** (ceg)</sub>

#### 4.5.2.2 `compliance-vertical` — Vertical compliance mapping (informational)
<sub>budget 0.11pp · import #247 · from **§11.6.1** (ceg)</sub>

#### 4.5.2.3 `documents-what-3` — What CEG 0.6 documents
<sub>budget 0.11pp · import #248 · from **§11.6.3** (ceg)</sub>

### 4.5.3 `takedown` — Fast-path takedown coordination (CEG 0.3 addition; per CIRISRegistry#37 + #38)
<sub>budget 0.55pp · import #46 · from **§11.4** (ceg)</sub>

### 4.5.4 `registry-named` — Named-moderator existence invariant + merit auto-promotion (1.0-RC21; per [CIRISRegistry#93](https://github.com/CIRISAI/CIRISRegistry/issues/93))
<sub>budget 0.42pp · import #72 · from **§11.11** (ceg)</sub>

### 4.5.5 `takedown-moderation` — Moderation as a delegable duty — `moderate` / `takedown` / `review` (1.0-RC19; per [CIRISRegistry#90](https://github.com/CIRISAI/CIRISRegistry/issues/90))
<sub>budget 0.26pp · import #109 · from **§11.10** (ceg)</sub>

### 4.5.6 `admission-operational` — Operational-language gate at admission
<sub>budget 0.2pp · import #151 · from **§11.1** (ceg)</sub>

### 4.5.7 `registry-watchlist` — Watchlist auto-detection — opt-in, per-group, separation-of-powers (1.0-RC23; per [CIRISRegistry#94](https://github.com/CIRISAI/CIRISRegistry/issues/94))
<sub>budget 0.14pp · import #185 · from **§11.12** (ceg)</sub>

### 4.5.8 `identity-set-2` — `identity_type` as a set — single-key role cohabitation (CEG 0.9 addition; per CIRISRegistry#49 + CIRISAgent#856)
<sub>budget 0.14pp · import #189 · from **§11.9** (ceg)</sub>

#### 4.5.8.1 `cohabitation` — Cohabitation discipline for constitutional + substrate roles
<sub>budget 0.12pp · import #217 · from **§11.9.3** (ceg)</sub>

#### 4.5.8.2 `amendment-what` — What the amendment changes — and what it deliberately does not
<sub>budget 0.11pp · import #254 · from **§11.9.1** (ceg)</sub>

#### 4.5.8.3 `settled` — Settled in CIRISAgent#856, carried as-is
<sub>budget 0.11pp · import #255 · from **§11.9.2** (ceg)</sub>

#### 4.5.8.4 `documents-what-6` — What CEG 0.9 documents
<sub>budget 0.11pp · import #256 · from **§11.9.4** (ceg)</sub>

### 4.5.9 `registry-geographic` — Geographic-community privacy invariant (CEG 0.8 addition; per CIRISRegistry#48)
<sub>budget 0.13pp · import #201 · from **§11.8** (ceg)</sub>

#### 4.5.9.1 `location-joining` — Joining is opt-in — substrate does NOT solicit location
<sub>budget 0.23pp · import #122 · from **§11.8.3** (ceg)</sub>

#### 4.5.9.2 `rough-only` — Rough-only is wire-format-enforced
<sub>budget 0.13pp · import #199 · from **§11.8.1** (ceg)</sub>

#### 4.5.9.3 `leaving` — Leaving is forward-only — the audit chain preserves the historical claim
<sub>budget 0.11pp · import #252 · from **§11.8.2** (ceg)</sub>

#### 4.5.9.4 `documents-what-5` — What CEG 0.8 documents
<sub>budget 0.11pp · import #253 · from **§11.8.4** (ceg)</sub>

### 4.5.10 `registry-hash` — Hash-database operator policy (CEG 0.3 addition; per CIRISRegistry#39)
<sub>budget 0.12pp · import #220 · from **§11.5** (ceg)</sub>

#### 4.5.10.1 `hash-database` — Hash-database access landscape
<sub>budget 0.12pp · import #214 · from **§11.5.1** (ceg)</sub>

#### 4.5.10.2 `registry-operator` — Operator path (CEG 0.3 default — option (a) per CIRISRegistry#39)
<sub>budget 0.11pp · import #244 · from **§11.5.2** (ceg)</sub>

#### 4.5.10.3 `future` — Future hash-coalition path (deferred; awaits CIRIS hash-coalition emergence)
<sub>budget 0.11pp · import #245 · from **§11.5.3** (ceg)</sub>

#### 4.5.10.4 `documents-what-2` — What CEG 0.3 documents
<sub>budget 0.11pp · import #246 · from **§11.5.4** (ceg)</sub>

### 4.5.11 `bootstrap-content` — Bootstrap-content pattern
<sub>budget 0.12pp · import #221 · from **§11.3** (ceg)</sub>

### 4.5.12 `family-self-2` — Self/family membership governance (CEG 0.7 addition; per CIRISRegistry#47)
<sub>budget 0.11pp · import #230 · from **§11.7** (ceg)</sub>

#### 4.5.12.1 `forward` — Forward secrecy on member departure — Option A (CEG 0.7 default)
<sub>budget 0.51pp · import #55 · from **§11.7.1** (ceg)</sub>

#### 4.5.12.2 `envelope-multi` — Multi-family membership — envelope `family_id` (CEG 0.7 default)
<sub>budget 0.15pp · import #178 · from **§11.7.2** (ceg)</sub>

#### 4.5.12.3 `admission-self` — Self-occurrence admission — single-vouch (CEG 0.7 default)
<sub>budget 0.12pp · import #222 · from **§11.7.4** (ceg)</sub>

#### 4.5.12.4 `reservation-reserved` — Reserved-prefix substrate emissions — locked in §7.7
<sub>budget 0.11pp · import #249 · from **§11.7.3** (ceg)</sub>

#### 4.5.12.5 `family-admission` — Family admission — consensus_protocol (CEG 0.7 normative)
<sub>budget 0.11pp · import #250 · from **§11.7.5** (ceg)</sub>

#### 4.5.12.6 `documents-what-4` — What CEG 0.7 documents
<sub>budget 0.11pp · import #251 · from **§11.7.6** (ceg)</sub>

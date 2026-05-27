# Prior Art Scan — FSD-002 federation surface vs the design space

**Companion to**: [`FSD-002_FEDERATION_SURFACE.md`](FSD-002_FEDERATION_SURFACE.md) v1.2; [`SOTA_SCAN.md`](SOTA_SCAN.md).
**Purpose**: Put FSD-002's distinctive design shapes in dialogue with prior-art systems so external reviewers can locate the work in the broader design space — what's a rediscovery, what's a genuine novel composition, what's borrowable. Scoped by structural shape, not by named system; systems are grouped by which distinctive shape they most challenge.
**Method**: 5 buckets, ~14 systems. Each bucket asks: *where does FSD-002 sit in the design space this set of systems spans?*
**Last updated**: 2026-05-27.

---

## Bucket 1: Single attestation primitive + 4 structural composers

### CIRIS distinctive shape (1 sentence)
FSD-002 collapses all federation claims into one `scores` attestation primitive (scalar `score`/`confidence` on an open-vocabulary `dimension` string) plus four graph-operating primitives — `delegates_to`, `supersedes`, `withdraws`, `recants` — for a total wire surface of five.

### Prior art table

| System | What's similar | What's distinct | What's borrowable into FSD-002 |
|---|---|---|---|
| PGP web of trust | One attestation type (a key signature) with a numeric trust level (marginal/full) carried as a scalar — close kin to `score`+`confidence` on a dimension | Single fixed dimension ("this key belongs to this UID"); revocation only, no `supersedes`/`withdraws`/`recants` distinction; no `delegates_to` (introducers are emergent, not declared) | Sequoia's flow-network model for resolving conflicting trust paths ([sequoia-wot](https://sequoia-pgp.gitlab.io/sequoia-wot/)); the lesson that distinguishing "I revoke" from "I was wrong" matters for downstream auditors |
| SPKI/SDSI | Authorization-as-certificate with explicit delegation bit (`propagate`) — the direct ancestor of `delegates_to` with scope | Boolean grants only (no scalar score/confidence); local-name namespaces are closed-vocabulary per principal; no notion of confidence-weighted aggregation | The 5-tuple `(issuer, subject, delegation, authorization, validity)` model maps almost 1:1 onto `(signer, subject, delegates_to/scope, dimension, valid_until)`; certificate-chain-discovery algorithms (Clarke et al.) for resolving delegation graphs |
| W3C VC/VP 2.0 | `credentialSubject` + claim graph + `proof` is structurally a `scores`-shaped attestation; `RefreshService` ≈ `supersedes`, `StatusList2021` ≈ `withdraws` | Claim payload is open-schema JSON-LD (not a single scalar); revocation is bitstring-encoded, not a typed primitive; no `recants` (admitting prior error is out of scope); delegation handled out-of-band via DID controllers | VC 2.0's `validFrom`/`validUntil` is wire-compatible with FSD-002's `valid_until`; Bitstring Status List ([w3c spec](https://w3c.github.io/vc-bitstring-status-list/)) as a deployment template for compact `withdraws`/`recants` distribution; DID method registry as a model for the open `dimension` namespace |

### Prior art synthesis (2-3 sentences)
FSD-002 sits at an unusual intersection: SPKI/SDSI's delegation/scope model + PGP's scalar-trust calculus + VC's open-claim graph — but reduced to a fixed 5-primitive surface no predecessor matched. The closest prior reach is VC + Bitstring Status List + RefreshService, which composes to roughly four primitives but never typed `recants` separately from `withdraws` (the epistemic-error vs. retraction distinction is genuinely new). SPKI/SDSI proved delegation-with-scope works as a primitive; CIRIS's contribution is treating it as one of exactly four graph operators on a uniform scalar substrate.

---

## Bucket 2: Rules-crowdsourced / verdicts-machined separation

### CIRIS distinctive shape (1 sentence)
FSD-002 splits trust into three layers — crowdsourced *rules* (signed Contribution + WA quorum), deterministic *verdicts* (machine-computed against a version-pinned rule package), and *adjudication* (WA quorum reading verdicts as evidence) — with appeals routed to a fresh-quorum recusal so original adjudicators cannot re-confirm.

### Prior art table

| System | What's similar | What's distinct | What's borrowable into FSD-002 |
|---|---|---|---|
| Birdwatch / Community Notes | Bridging algorithm requires agreement across historically-divergent rater profiles before a note publishes — separates "who writes" (crowd) from "what shows" (deterministic scoring against a published `scoring.py`) ([X repo / arxiv](https://arxiv.org/html/2510.09585v3)) | Single platform-owned rule code; no quorum amendment path; no appeals layer (notes appear/disappear based on continuous rescoring); rater "identity" is pseudonymous behavioral, not keyed | T1/T3 in practice: Twitter open-sources the scorer and pins note-state to a dated algorithm version — direct precedent for `rules.pkg@version` in `evidence_refs` |
| Pol.is | Surfaces *consensus statements* by clustering opinion across factions rather than majority vote; output is descriptive (machine-derived from raw votes), input (statements) is crowdsourced ([compdemocracy.org](https://compdemocracy.org/case-studies/)) | No adjudication or consequence layer — Pol.is produces a report, humans (e.g. vTaiwan officials) decide. No formal rule-package versioning; statement moderation is operator-side | T4 by construction: Pol.is outputs *cannot* directly cause standing-change — the consensus map is evidence for a separate deliberative body, exactly FSD-002's attestation→adjudication firewall |
| Kleros | Pseudonymous jurors stake PNK and vote on disputes against a per-subcourt *policy document* (the "rules"); appeals draw a fresh, larger jury ([docs.kleros.io](https://docs.kleros.io/kleros-faq)) | Jury *is* the verdict-producer (no machine layer between evidence and ruling); rule changes are token-weighted governance, not WA quorum; appeals are larger-N, not fresh-recusal | Fresh-jury appeal mechanic — Kleros doubles jury size each appeal round; FSD-002 could specify minimum non-overlap percentage rather than just "fresh quorum" |

### Prior art synthesis (2-3 sentences)
Birdwatch is the clearest precedent for FSD-002's T1/T2/T3: a published scoring algorithm pinned by version, with prefix-style helpfulness labels describing a mechanism ("rated helpful by raters from different viewpoints") not a subjective quality. Pol.is independently arrived at T4 — outputs are evidence for downstream deciders, never consequence themselves. Kleros is the *counterexample*: it collapses verdicts and adjudication into one juror vote, and the [documented 51%-attack by a co-founder](https://deepfivalue.substack.com/p/the-kleros-experiment-has-failed) is exactly the failure mode FSD-002's three-layer separation is designed to make structurally impossible (rule-amendment quorum ≠ verdict-computation ≠ adjudicating quorum ≠ appeal quorum).

---

## Bucket 3: Relational-anthropology substrate (Ubuntu-primary)

### CIRIS distinctive shape (1 sentence)
FSD-002 explicitly names *Umuntu ngumuntu ngabantu* as the wire-format substrate, so attestations are constitutive (not observational), correlated-action detection brings patterns into morally-real existence, and no principal — including the registry itself — is exempt from the recursive Golden Rule it imposes on others.

### Prior art table

| System | What's similar | What's distinct | What's borrowable into FSD-002 |
|---|---|---|---|
| Spritely Goblins / OCapN ([spritely.institute/goblins](https://spritely.institute/goblins/), [ocapn/ocapn](https://github.com/ocapn/ocapn)) | Capability references are unforgeable, transferable, and *relational* — you only have power over what someone introduced you to; "introductions" are first-class | Atomic-agent ontology inherited from E/KeyKOS: each ocap is its own monad; identity = unforgeable reference, not constituted by a graph; no anthropological framing | "Introduction ceremony" semantics for cross-attestation: peer A vouches B to C is a primitive, not a derived fact |
| Holochain ([holochain.org](https://www.holochain.org/)) | "Agent-centric" architecture — each agent has its own source-chain, validation is performed by neighbors; explicitly post-blockchain and explicitly rejects single-truth ledgers | Agent-centric != relational-constitutive: each agent is still epistemically prior to its peers; DHT validators check rules, not constitute personhood; framing is Western-individualist with P2P plumbing | Source-chain + neighborhood-validation pattern as the F-3 correlated-action substrate — local chain is per-org, validation graph is federation-shared |

### Prior art synthesis (2-3 sentences)
Holochain's "agent-centric" is the closest CS-side neighbor but still treats the agent as ontologically prior; CIRIS goes further by making the cross-attestation graph *constitutive* of standing — an org with zero peer attestations is not just unknown, it has reduced moral reality in the federation. Explicit Ubuntu grounding in decentralized-identity literature appears absent; the closest precedent is the **CARE Principles for Indigenous Data Governance** ([Carroll et al. 2020](https://datascience.codata.org/articles/10.5334/dsj-2020-043), [Jennings et al. 2024](https://datascience.codata.org/articles/10.5334/dsj-2024-032)), which center *relational accountability* and collective authority over data — and African-philosophy work by Menkiti and Metz arguing personhood is achieved, not given. FSD-002 appears to be the first decentralized-identity wire format to name Ubuntu as load-bearing substrate rather than as cultural flavoring.

---

## Bucket 4: Federation governance via Contribution + WA quorum

### CIRIS distinctive shape (1 sentence)
FSD-002 §4.9.2 routes rule-layer changes through P5 Contribution envelopes weighted by *earned* Credits/Expertise (P2/P3), adjudicated by a recused-from-self P8 WA quorum with N=3 witness diversity, where slashing (P9) is explicitly decoupled from disagreement — Goal/Approach/Measure disputes route to P11 Reconsideration instead of penalty.

### Prior art table

| System | What's similar | What's distinct | What's borrowable into FSD-002 |
|---|---|---|---|
| Aragon DAOs ([OSx framework](https://daotimes.com/aragon-dao-tool-report-for-2025/)) | Signed proposal envelopes, plugin-modular adjudication, on-chain audit trail of governance actions, supports diverse voting plugins | Default voting weight is ERC-20 token balance (plutocratic); no native concept of "earned standing" decoupled from purchasable token; adjudicator and proposer pool overlap (no recusal); slashing not a primitive | Plugin/template separation (FSD-002 could model the calibration-package transition as a named plugin); the multichain ([zkSync/LayerZero](https://www.prnewswire.com/news-releases/aragon-brings-multichain-governance-to-daos-with-zksync-and-layerzero-301983483.html)) treasury-action pattern parallels cross-cell witness diversity |
| Conviction voting ([1Hive Gardens](https://github.com/1Hive/conviction-voting-app), [TEC Disputable CV](https://token-engineering-commons.gitbook.io/tec-handbook/governance/voting-tools-and-methods/conviction-voting)) | Time-integrated commitment as a weight (analogous to Credits accrued over time); continuous rather than time-boxed; resists last-minute capture | Still token-weighted at the base — conviction is `tokens × time`, not earned-merit × time; no separate adjudicator quorum (the conviction crossing the threshold IS the decision); TEC's Celeste integration is the closest analogue to a recused adjudicator but operates as dispute layer not primary | Time-integration of standing (Credits could accrue conviction-style); TEC's "Disputable" pattern (Celeste challenge layer) is a near-twin of P11 Reconsideration with a separate quorum |

### Prior art synthesis (2-3 sentences)
Both systems compose well at the *envelope/proposal* layer (Aragon's plugin model and CV's signed staking actions resemble P5 Contributions) but collapse into token plutocracy at the *weight* layer — Aragon defaults to 1-token-1-vote and CV's "conviction" is still token-denominated, so both end up with the top 10% controlling [76.2% of voting power](https://patentpc.com/blog/dao-growth-stats-treasury-sizes-governance-votes-activity) at federation scale. Coordinape and (defunct) SourceCred separately tried to build *earned reputation* as a governance input ([SourceCred PageRank, Coordinape GIVE](https://medium.com/sourcecred/the-dao-missing-link-reputation-protocols-8e141355cef2)), but neither produced a quorum-adjudication layer on top — they remained signaling/payment systems. CIRIS's full shape — earned Credits/Expertise + witness-diversity bar + recused WA quorum + slashing-decoupled-from-disagreement — has no direct prior art; TEC + Celeste is the closest *partial* analogue but still rests on token-weighted base conviction.

---

## Bucket 5: Versioned hash-pinned calibration with delegates_to rename chain

### CIRIS distinctive shape (1 sentence)
FSD-002 treats RATCHET rule packages as versioned, hash-pinned, evidence-cited artifacts whose renames propagate via the wire format's own `delegates_to` primitive — applying supply-chain attestation discipline (version + sha256 + chain-of-custody) to policy-rule attestation, and amending them via federation WA quorum rather than a single-author release loop.

### Prior art table

| System | What's similar | What's distinct | What's borrowable into FSD-002 |
|---|---|---|---|
| Sigstore / rekor | Append-only Merkle log binds (artifact-hash, signer-identity, timestamp); consumers re-verify against the exact entry; OIDC-keyless workflow means signing identity is a structured claim, not just a key | Logs *signing events*, not *semantic versions of policy rules*; rename of a signing identity is out-of-band (revoke + re-issue), no in-protocol `delegates_to` | Inclusion-proof model for evidence_refs; Rekor v2 tile-based storage as a cheap backend for calibration-history retention ([blog.sigstore.dev/rekor-v2-ga](https://blog.sigstore.dev/rekor-v2-ga/)) |
| Binary transparency (Trillian) | Same Merkle-log shape applied to OS-update binaries; "no party, including Google, can modify what was authorized for release without a public record"; covers Play Services + Mainline modules ([blog.google](https://blog.google/security/bringing-binary-transparency-to-the-android-ecosystem/)) | Single-publisher trust root (Google signs, public verifies); no quorum-amendment path; renames handled by package-name aliases at the distribution layer, not in the log entry itself | The "verifiable transport" framing — a separate adjudication layer (researchers/auditors) consumes the log and challenges discrepancies, mirroring CIRIS's separation of computation from verdict |
| SLSA / in-toto / SPDX | Predicate-typed attestations with explicit `predicateType` URI versioning; in-toto's versioning spec mandates that any minor-version parse must be semantically equivalent ([in-toto/attestation/spec/versioning.md](https://github.com/in-toto/attestation/blob/main/spec/versioning.md)); URI rename precedent exists (`in-toto.io/Provenance/v0.1` → `slsa.dev/provenance`) | The rename was a one-time spec-committee decision communicated out-of-band, not a structural primitive emitted in subsequent attestations; no `delegates_to` field exists in the predicate itself | Predicate-type URI + monotonic-minor-version rule as the schema spine for `correlated_action_v{N}`; VSA (Verification Summary Attestation) as the model for compressing "I checked this against rule vN" into a re-verifiable claim |

### Prior art synthesis (2-3 sentences)
The structural symmetry holds cleanly on the wire-format axis: hash-pinned + version-tagged + signature-bound + append-logged is genuinely the same shape for "this binary was built from these sources" and "this verdict was computed against this rule." It breaks on the *governance* axis — the supply-chain world assumes a single authoritative publisher per artifact (Google for Play Services, the maintainer for an npm package, the build platform for SLSA provenance), while FSD-002 assumes federated quorum-amendment of the rule itself. No one in the supply-chain ecosystem has explicitly named this unity; SLSA's "attestation" vocabulary stops at build provenance and VSAs, and the trust-rule world (OPA/Rego, policy-as-code) has stayed conceptually separate, treating policies as code-to-be-built rather than as attestable artifacts in their own right. FSD-002's `delegates_to` rename primitive appears to be genuinely novel — neither rekor nor in-toto has an in-band rename channel.

---

## Cross-bucket synthesis

Across the five buckets, FSD-002's distinctive contribution is **composition discipline**, not novel atomic primitives. Almost every individual primitive has a predecessor — SPKI/SDSI's delegation-with-scope (Bucket 1), Birdwatch's bridging-as-rule (Bucket 2), Holochain's agent-centric DHT (Bucket 3), TEC's Celeste-as-recused-adjudicator (Bucket 4), Sigstore's hash-pinned attestation (Bucket 5). What no prior system did is compose all five in one wire-format-locked specification with the recursive Golden Rule binding the framework's own authoring constraints.

The three genuinely novel atomic contributions, if the comparison holds:

1. **`recants` as a distinct structural primitive** (Bucket 1) — the epistemic-error-versus-retraction distinction has no prior in PGP, SPKI/SDSI, or VC.
2. **Explicit Ubuntu-grounded anthropology in a wire-format specification** (Bucket 3) — Indigenous-data-governance and African-philosophy precedents exist as ethical frameworks; none have shipped as a load-bearing substrate in a decentralized-identity protocol.
3. **`delegates_to` rename chains for policy-rule artifacts** (Bucket 5) — supply-chain attestation never made the rename mechanism in-protocol; FSD-002 is first.

For external reviewers: the right read is not "FSD-002 invents the federation surface from scratch" — it's "FSD-002 is the first system to compose the design space's tested primitives under a single specification with an explicit anthropological substrate and a Recursive-Golden-Rule binding."

---

## References

- [W3C Verifiable Credentials 2.0 (Recommendation, May 2025)](https://www.w3.org/TR/vc-data-model-2.0/)
- [W3C Bitstring Status List](https://w3c.github.io/vc-bitstring-status-list/)
- [Sequoia OpenPGP Web of Trust](https://sequoia-pgp.gitlab.io/sequoia-wot/)
- [RFC 2693 SPKI Certificate Theory](https://datatracker.ietf.org/doc/html/rfc2693)
- [UCAN spec (SPKI/SDSI descendant)](https://github.com/ucan-wg/spec)
- [Macaroons paper (Google Research)](https://research.google/pubs/pub41892/)
- [CRSet: Private VC Revocation (arXiv 2501.17089)](https://arxiv.org/html/2501.17089v2)
- [Community Notes algorithm sustainability (arXiv 2510.00650)](https://arxiv.org/pdf/2510.00650)
- [vTaiwan + Pol.is case study (peoplepowered)](https://www.peoplepowered.org/news-content/digital-participation-case-study-taiwan)
- [Kleros FAQ](https://docs.kleros.io/kleros-faq)
- [Spritely Goblins](https://spritely.institute/goblins/)
- [OCapN protocol](https://github.com/ocapn/ocapn)
- [Holochain.org](https://www.holochain.org/)
- [CARE Principles (Carroll et al. 2020)](https://datascience.codata.org/articles/10.5334/dsj-2020-043)
- [CARE Principles operationalization (Jennings et al. 2024)](https://datascience.codata.org/articles/10.5334/dsj-2024-032)
- [Aragon OSx framework](https://daotimes.com/aragon-dao-tool-report-for-2025/)
- [Conviction Voting (1Hive)](https://github.com/1Hive/conviction-voting-app)
- [TEC Disputable Conviction Voting handbook](https://token-engineering-commons.gitbook.io/tec-handbook/governance/voting-tools-and-methods/conviction-voting)
- [SourceCred PageRank / Coordinape GIVE (Medium)](https://medium.com/sourcecred/the-dao-missing-link-reputation-protocols-8e141355cef2)
- [Sigstore rekor v2 GA](https://blog.sigstore.dev/rekor-v2-ga/)
- [Android Binary Transparency (blog.google)](https://blog.google/security/bringing-binary-transparency-to-the-android-ecosystem/)
- [in-toto attestation versioning spec](https://github.com/in-toto/attestation/blob/main/spec/versioning.md)

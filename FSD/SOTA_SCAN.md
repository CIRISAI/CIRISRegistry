# State of the Art Scan — FSD-002 federation surface vs production-validated systems

**Companion to**: [`FSD-002_FEDERATION_SURFACE.md`](FSD-002_FEDERATION_SURFACE.md) v1.2; [`PRIOR_ART_SCAN.md`](PRIOR_ART_SCAN.md).
**Purpose**: Where `PRIOR_ART_SCAN.md` asks "where does FSD-002 sit in the design space?", this document asks "what production load has each shape actually carried, and where is FSD-002 inheriting validated patterns vs venturing into unvalidated frontier?" The same 14 systems, organized by which distinctive shape FSD-002 borrows from them, with deployment state, scale evidence, and known limitations as of 2026-05.
**Method**: Each bucket gives a SOTA table + a 2-3 sentence reading of what the production record means for FSD-002's claims.
**Last updated**: 2026-05-27.

---

## Bucket 1: Single attestation primitive + 4 structural composers

### SOTA table

| System | Deployment state (2026) | Scale evidence | Known limitations relevant to CIRIS |
|---|---|---|---|
| PGP web of trust | Niche / expert-only; SKS keyserver network deprecated, replaced by [keys.openpgp.org](https://keys.openpgp.org/) which strips third-party signatures, effectively neutering classic WoT. Sequoia-WoT actively maintained (Rust, 2026) but for small expert groups | Tens of thousands of active WoT participants; trust-path computation expensive at scale ([Axelspire 2026](https://axelspire.com/vault/foundations/trust-models/)) | Scalability collapse; no native "I was wrong" semantics; majority of users default to TOFU |
| SPKI/SDSI | Abandoned at IETF experimental status (RFC 2693/2692, never advanced past Experimental); no production deployments | Zero direct production scale; lives on as design DNA in Macaroons, Biscuit, UCAN ([UCAN spec](https://github.com/ucan-wg/spec)) | Cautionary tale: minimal-primitive elegance ≠ adoption; needs a forcing function (CIRIS has one: licensure) |
| W3C VC 2.0 | W3C Recommendation since May 2025 ([W3C press](https://www.w3.org/press-releases/2025/verifiable-credentials-2-0/)); EU Digital Identity Wallet rollout 2026-2027 driving tens-of-millions-scale deployment | Tens of millions of holders across government issuers; multiple production stacks (Microsoft Entra Verified ID, walt.id, IOTA Identity, EBSI) | Open-schema JSON-LD payloads — each issuer reinvents the dimension; no scalar/confidence convention; revocation privacy leaks via Bitstring Status List ([arxiv 2501.17089](https://arxiv.org/html/2501.17089v2)) |

### SOTA reading (2-3 sentences)
Only W3C VC 2.0 has production validation at the scale CIRIS would need (tens of millions of credentials, government issuers, post-2025 recommendation status). PGP's WoT is a long, slow abandonment of the classic model — preserved technically by Sequoia but operationally hollowed by keys.openpgp.org stripping third-party sigs — and SPKI/SDSI is the canonical "elegant minimum that never shipped." The deployment record argues the 1+4 shape is viable only if CIRIS pairs it with a forcing function VC has (regulatory mandate) and PGP/SPKI lacked: the licensure dimension and Registry-mediated revocation are exactly that lever.

---

## Bucket 2: Rules-crowdsourced / verdicts-machined separation

### SOTA table

| System | Deployment state (2026) | Scale evidence | Known limitations relevant to CIRIS |
|---|---|---|---|
| Birdwatch / Community Notes | Production on X (200 countries by 2025); Meta adopted the model Jan 2025 for Facebook/Instagram; AI Notes Writer launched July 2025 ([heyorca](https://www.heyorca.com/blog/x-twitter-social-news), [CBS](https://www.cbsnews.com/news/what-is-community-notes-twitter-x-facebook-instagram/)) | ~1M contributors by 2025; 930K had rated ≥1 note by Dec 2024 ([Wikipedia](https://en.wikipedia.org/wiki/Community_Notes)) | Sustainability paper ([arxiv 2510.00650](https://arxiv.org/pdf/2510.00650)) documents declining note-publication rates in 2025 and contributor churn — bridging works but is fragile under coordinated rating drift; NBC reports usage "plummeted" in 2025 ([NBC](https://www.nbcnews.com/tech/social-media/x-twitter-community-notes-disappear-data-rcna210710)) |
| Pol.is | Active; vTaiwan + TWNIC ran AI-governance roundtable Dec 2024, presented to Taiwan NHRC March 2025 ([peoplepowered.org](https://www.peoplepowered.org/news-content/digital-participation-case-study-taiwan), [freiheit.org](https://www.freiheit.org/taiwan/how-public-participation-can-improve-ai-governance-vtaiwans-initiatives)) | Hundreds of public deliberations globally; vTaiwan governance influenced multiple Taiwan statutes | No adversarial-load testing comparable to X — most deployments are operator-curated participant pools, so resistance to Sybil flooding is unproven at open-internet scale |
| Kleros | Kleros 2.0 Beta on Arbitrum One Nov 2024; Court V2 in Certora audit through 2026 ([blog.kleros.io 2026](https://blog.kleros.io/kleros-project-update-2026/), [Dec 2025](https://blog.kleros.io/kleros-development-update-december-2025/)) | ~1,200 disputes through 2024; usage well below initial projections | Documented co-founder-executed 51% attack that denied insurance claims; token-weighted juror selection means rule-layer capture and verdict-layer capture share the same attack surface — exactly the collapse FSD-002 separates |

### SOTA reading (2-3 sentences)
Only Birdwatch has been validated under genuine adversarial production load (1M raters, political actors actively trying to manipulate), and its bridging-based separation has held against per-note manipulation while showing erosion at the rules-layer (Musk-era contributor churn, AI-author influx) — confirming FSD-002's instinct that the rules layer needs WA-quorum gating, not open contribution. Pol.is's separation holds because consequence is always offboard (proving T4 is achievable but not stress-testing it), while Kleros's collapse of layers produced the precise failure mode — a single-actor capture of both rule application and consequence — that FSD-002's three-quorum architecture is designed to prevent. The distinctive contribution is not the bridging insight (Birdwatch has it) or the consequence-firewall (Pol.is has it), but compounding both with cryptographic version-pinning and fresh-quorum appeal recusal.

---

## Bucket 3: Relational-anthropology substrate (Ubuntu-primary)

### SOTA table

| System | Deployment state (2026) | Scale evidence | Known limitations relevant to CIRIS |
|---|---|---|---|
| Spritely Goblins | Pre-1.0; Guile + Racket impls; OCapN/CapTP interop work ongoing with Agoric SwingSet & Cap'n Proto ([Spritely QCon London Mar 2026](https://www.infoq.com/news/2026/03/spritely-infrastructure/), [Agoric forum](https://community.agoric.com/t/ocapn-captp-interop-with-spritely-networked-communities-capn-proto/198)) | No production deployments; explicit "not production-ready" guidance from upstream | 25+ years of ocap research has not yet produced a deployed federation; suggests anthropological framing alone doesn't drive adoption — needs a forcing function (CIRIS has licensing/liability) |
| Holochain | Released Jan 27 2026; Wind Tunnel stress-testing infra Mar 2026; ~140 repos in ecosystem ([Happenings 2025 review](https://happeningscommunity.substack.com/p/the-holochain-ecosystem-in-2025-a), [Holochain roadmap](https://www.holochain.org/roadmap/)) | Real apps: NY Carbon Farm Network (hREA supply chain), Volla messaging, kando, talking-stickies; community estimate "1-2 year" production horizon | DHT validation is rule-based, not norm-constitutive; no built-in notion that detection-creates-moral-fact; per-app DNAs balkanize the validation graph |

### SOTA reading (2-3 sentences)
Agent-centric architecture has shipped (Holochain, 2026) but only at hobbyist + small-supply-chain scale; capability-secure relational architecture (Spritely) has not shipped after a quarter-century. The deployment record suggests technical relationality without an explicit anthropological commitment drifts toward individualist defaults (Holochain's marketing is "you own your data" — possessive-individualist framing for a relational substrate), validating FSD-002 §1.10's claim that *naming the substrate* is the discipline that prevents drift. CIRIS's distinguishing bet is that pairing Ubuntu-grounded semantics with a hard forcing function (licensing, liability, professional accountability) is what carries relational architecture across the production threshold.

---

## Bucket 4: Federation governance via Contribution + WA quorum

### SOTA table

| System | Deployment state (2026) | Scale evidence | Known limitations relevant to CIRIS |
|---|---|---|---|
| Aragon DAOs | Active; OSx framework live; >7,000 DAOs deployed; major tenants Lido (Dual Governance + stVaults Q2 2026), Decentraland, API3 ([recap Feb 2026](https://blog.lido.fi/recap-lido-tokenholder-update-february-2026/)) | ~$6B active treasury across platform, ~$12B historical secured value; Lido Earn 61K ETH TVL ([DAO stats](https://patentpc.com/blog/dao-growth-stats-treasury-sizes-governance-votes-activity)) | <10% turnout typical; 350–500 voters per proposal even at Uniswap scale; top 10% holders = 76.2% voting power → whale capture is the steady state, not the failure mode |
| Conviction voting | 1Hive Gardens production on Gnosis Chain since 2020; TEC active with Disputable CV + Celeste; Polkadot OpenGov uses time-locked variant ([TEC handbook](https://token-engineering-commons.gitbook.io/tec-handbook/governance/voting-tools-and-methods/conviction-voting)) | Order-of-magnitude smaller than Aragon-mainnet; TEC + 1Hive treasuries in low-millions USD range; Polkadot OpenGov is the largest deployment by capital | Resists *flash* capture but not *sustained* whale capture — a large holder simply stakes early and waits; UX is opaque ([Gardens issue #340](https://github.com/1Hive/gardens/issues/340)); no native recusal for the parties accruing conviction |

### SOTA reading (2-3 sentences)
The deployment record is unambiguous: token-weighted governance scales operationally (Aragon clears billions in treasury) but does *not* scale past whale-capture — sub-10% turnout and 76%-top-decile concentration are the equilibrium, not transient pathologies. Conviction voting demonstrably reduces *flash* capture and is production-proven at small-to-mid scale, but its token base means it inherits the same plutocratic ceiling at federation size, and the TEC/Celeste pattern proves a *separate* adjudicator quorum is implementable in production. The empirical gap CIRIS is filling — Contribution-weight derived from truth-grounding (P2 Credits) rather than purchase, with a structurally recused WA quorum — has no scaled deployment to point to, which is both the risk (unproven at scale) and the thesis (the failure modes of existing scaled systems are exactly the ones the Credits/recusal/decoupled-slashing shape is designed against).

---

## Bucket 5: Versioned hash-pinned calibration with delegates_to rename chain

### SOTA table

| System | Deployment state (2026) | Scale evidence | Known limitations relevant to CIRIS |
|---|---|---|---|
| Sigstore / rekor | Rekor v2 GA on tile-based Trillian-Tessera, 99.5% availability SLO; public BigQuery research dataset announced Oct 2025 ([openssf.org](https://openssf.org/blog/2025/10/15/announcing-the-sigstore-transparency-log-research-dataset/)) | Backs GitHub Actions OIDC-keyless signing for thousands of OSS releases; cosign v3.0.1+/v2.6.0+ produces bundles by default | **May 2026 Shai-Hulud npm worm forged valid Sigstore provenance** via ephemeral keys + compromised CI OIDC tokens — 633 malicious package versions passed verification ([snyk.io](https://snyk.io/blog/tanstack-npm-packages-compromised/)); a green badge is not legitimacy proof |
| Binary transparency (Trillian) | Android Binary Transparency expanded May 1, 2026 to all production Google apps + Play Services + Mainline modules ([helpnetsecurity.com](https://www.helpnetsecurity.com/2026/05/06/google-android-binary-transparency/), [thehackernews.com](https://thehackernews.com/2026/05/android-apps-get-public-verification.html)) | Billions of Android devices in scope; verification tooling open-sourced | Single-publisher; verification adoption by end-users is unmeasured — log existence ≠ log monitoring |
| SLSA / in-toto / SPDX | SLSA L2 is the practical industry sweet spot; L3 reference generators for GitHub Actions emit sigstore-signed in-toto provenance in one workflow line | TanStack May 11 2026 incident produced **valid SLSA Build L3 provenance for malicious packages** — first documented case ([snyk.io](https://snyk.io/blog/tanstack-npm-packages-compromised/)) | Provenance attests "who built it where" not "is it benign"; rename governance is committee-out-of-band, not in-protocol |

### SOTA reading (2-3 sentences)
Versioning + transparency-log + hash-pinning have been validated at billions-of-devices scale (Android BT) and thousands-of-publishers scale (Sigstore-backed OSS), and the underlying Merkle-log mechanics are production-hardened. But the May 2026 Shai-Hulud / TanStack incidents proved that *attestation validity is not verdict validity*: a cryptographically perfect SLSA L3 + Sigstore bundle was produced for malicious code because the build environment itself was compromised. This is exactly the gap FSD-002's separation of computation-from-adjudication is designed to close — the supply-chain world's deployment record validates the transport (logs scale, signatures hold) but actively demonstrates that an additional independent-adjudication layer is required, which is what `delegates_to` + WA-quorum-amendment provides and what SLSA/Sigstore structurally cannot.

---

## Cross-bucket synthesis: validated patterns vs unvalidated frontier

The five SOTA readings sort cleanly into three categories:

### Validated at production load — FSD-002 inherits these

| Pattern | Validated by | What CIRIS can claim |
|---|---|---|
| Open-vocabulary attestation graph + revocation list | W3C VC 2.0 + Bitstring Status List (tens of millions of holders, EU DI rollout) | The 1+4 wire shape with `valid_until` + `withdraws` is operationally sound; CIRIS extends rather than invents |
| Rules-as-version-pinned-code in production with adversarial readers | Birdwatch (1M contributors, hostile political climate) | Rule-layer versioning + machine-applied verdicts is production-hardened |
| Transparency-log mechanics (Merkle inclusion, append-only) | Sigstore + Android Binary Transparency (billions of devices) | Hash-pinned `evidence_refs` + retrievable proofs scales; the transport layer is solved |
| Adjudication via separate quorum from rule-amendment | TEC + Celeste (small-mid scale); Pol.is + vTaiwan (offboard consequence) | The separation pattern is implementable; just not yet at federation scale |

### Demonstrated failure modes — FSD-002 is structurally designed to avoid these

| Failure mode | Demonstrated in | FSD-002's structural answer |
|---|---|---|
| Rule-layer capture by single actor | Kleros (co-founder-executed 51%) | §4.9.2 WA-quorum amendment + witness diversity |
| Whale-capture at governance equilibrium | Aragon (top-10% controls 76%) | Credits/Expertise weight is earned, not purchasable |
| Attestation validity ≠ verdict validity | Shai-Hulud / TanStack (May 2026) | Adjudication layer (WA quorum) cannot be skipped by valid signature alone |
| Possessive-individualist drift in relational systems | Holochain "you own your data" framing | §1.10 explicit Ubuntu naming as drift-prevention discipline |

### Unvalidated frontier — CIRIS is the first attempt

| Pattern | Why it's unvalidated | Risk |
|---|---|---|
| Earned-Credits-weighted governance at federation scale | No prior system separates earned standing from purchasable token at scale | Adoption gap — without a forcing function, the system is "elegant minimum that never shipped" (SPKI/SDSI failure mode) |
| Ubuntu-grounded relational-anthropology substrate as wire-format substrate | CARE Principles + African philosophy exist as frameworks, never as protocol substrate | First-adopter risk — no precedent for how the discipline interacts with engineering trade-offs at scale |
| In-protocol `delegates_to` rename chain for policy artifacts | Supply-chain attestation handles renames out-of-band (committee decisions); no in-band mechanism exists | Coordination cost — every consumer pinning old names must implement the chain-following logic; no reference impl to inherit |
| `recants` as a structural primitive distinct from `withdraws` | PGP/VC/SPKI all collapse retraction and error-admission | Adoption ambiguity — consumers may treat them identically until governance disputes force the distinction |

### Reading for external reviewers

FSD-002 inherits production-validated patterns at its transport layer (Bucket 1 + Bucket 5) and at its operational separation (Bucket 2's bridging principle, Bucket 4's separated adjudication quorum). The unvalidated frontier is the **composition + governance shape**: earned-Credits-weighted federation governance + Ubuntu-grounded substrate + recursive-Golden-Rule binding. The risk profile is exactly the opposite of "untested cryptography on a known governance shape" — the cryptographic primitives are stable (Ed25519+ML-DSA-65, sigstore-style transparency, W3C VC envelopes), and the unknowns are at the social-architecture layer where deployment is the only test.

The forcing function CIRIS provides — licensure attestation backed by professional liability (medical/legal/financial regulatory frameworks) — is the structural reason FSD-002 has a path past the SPKI/SDSI adoption gap. Without that forcing function, the same 1+4 + Ubuntu + Credits shape would be another elegant minimum that never shipped. With it, the licensure dimension is the lever that pulls every other primitive into production use.

---

## What this scan does NOT establish

- **Empirical performance.** No claim that FSD-002's primitives are faster, more accurate, or cheaper than predecessors. The scan is about structural shape, not runtime.
- **Adoption forecast.** No claim that the forcing function will actually drive adoption — that's an open empirical question, not a comparison artifact.
- **Comprehensive coverage.** 14 systems is a deliberate scope-limit; systems not covered (Sovrin, Hyperledger Indy, RChain, Secure Scuttlebutt, Nostr, Bluesky AT Protocol) may surface additional convergence or distinction patterns. Future revisions of this scan should add a sixth bucket if a missing system challenges a distinctive shape not currently bucketed.
- **Position relative to non-decentralized prior art.** Centralized reputation systems (Stack Overflow, Wikipedia citations, academic peer review) embody similar separations (crowdsourced rules, machined verdicts via centralized rule application) but are out of scope for this scan because they don't address the federated-substrate question.

---

## References

See [`PRIOR_ART_SCAN.md` References section](PRIOR_ART_SCAN.md#references) for the full citation list. Sources unique to deployment evidence in this document:

- [Sigstore Transparency Log Research Dataset (OpenSSF, Oct 2025)](https://openssf.org/blog/2025/10/15/announcing-the-sigstore-transparency-log-research-dataset/)
- [Android Binary Transparency expansion (helpnetsecurity, May 2026)](https://www.helpnetsecurity.com/2026/05/06/google-android-binary-transparency/)
- [Shai-Hulud + TanStack analysis (Snyk, May 2026)](https://snyk.io/blog/tanstack-npm-packages-compromised/)
- [Kleros 2.0 Beta (Nov 2024)](https://blog.kleros.io/kleros-project-update-2026/)
- [Lido Tokenholder Update (Feb 2026)](https://blog.lido.fi/recap-lido-tokenholder-update-february-2026/)
- [Holochain ecosystem 2025 review (Happenings)](https://happeningscommunity.substack.com/p/the-holochain-ecosystem-in-2025-a)
- [Spritely Infrastructure (InfoQ, Mar 2026)](https://www.infoq.com/news/2026/03/spritely-infrastructure/)
- [Community Notes plummet (NBC, 2025)](https://www.nbcnews.com/tech/social-media/x-twitter-community-notes-disappear-data-rcna210710)
- [DAO Growth Stats (patentpc)](https://patentpc.com/blog/dao-growth-stats-treasury-sizes-governance-votes-activity)

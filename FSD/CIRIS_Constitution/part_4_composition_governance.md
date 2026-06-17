# Part IV — Composition & Governance

**CC decimal range** `4.x` · **96 concepts** · **page budget 26.2pp** (∝ importance) · [← master index](README.md)

> How attestations compose into trust, how the federation governs itself, and — at the root of all of
> it — the one place a human hand can stop the whole machine. This is the largest Part because it
> carries the most: the discipline that keeps the wire honest (the anti-patterns), the human-held
> halt-authority that lives *outside* the federation (the HUMANITY_ACCORD), the Wise Authorities a
> system defers to, the dozen composition policies that turn raw attestations into verdicts, and the
> amendment / moderation machinery the federation governs itself with. Read it as the answer to one
> question: **once the grammar can say things and the namespace says what they're about, how does the
> federation decide what to believe, who may change the rules, and who — outside the system — can pull
> the plug?**
>
> Its heaviest concept is placed first not by section number but by weight: the **anti-patterns**
> (4.1) and the **HUMANITY_ACCORD** (4.2) are where the federation's deepest commitments — *the wire
> cannot be used to brand a person*, and *consent requires a stop-button the system cannot reach* —
> become mechanical. Everything downstream binds back to those two and, through them, to M-1.

---

## 4.1 `anti-pattern` — Anti-patterns
<sub>budget 1.65pp · import #7 · from **CEG §13** · semantic id `anti-pattern`</sub>

The anti-patterns are the federation's negative space — the wire-format reaches that *look* reasonable
and are *deliberately refused*. They are recorded so the next author finds the discipline before the
wire format does. Each one is individually reachable; none is necessary; and the recurring reason they
fail is the same reason the [admission gate's T2](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate)
exists: **the grammar must measure mechanisms, never brand persons.** A CEG-Conforming Producer (CCP)
SHOULD NOT emit attestations matching any pattern below.

The deepest of them all is named at 4.1.2 — so it is stated first here, because it explains every other:

### 4.1.2 `pattern` — The discipline pattern
<sub>budget 0.15pp · import #183 · from **CEG §13.5** · semantic id `pattern`</sub>

Read across almost every rejected addition and one shape repeats: **extending the wire so a single
attester can pre-declare its own state more richly.** A new `epistemic_mode` so a lone subject can
announce its inner certainty; a `transparency:*` prefix so a producer can *claim* disclosure; a
`score:trustworthiness:*` so an entity can carry a self-summary. Each is reachable; none is necessary.

The cut is the [Ubuntu commitment](part_1_foundation.md#1131-ubuntu--the-ubuntu-commitment-informative)
made operational: **standing is constituted relationally, through attestation by others — not through
self-declaration.** The wire format should *resist* primitives that let a single key announce its own
state without external composition. The substrate is austere *by design* so that consumers compose
verdicts; richer narrative belongs in `context:`, in `evidence_refs[]`, and in downstream witness
attestations — never in new envelope enum members or new self-attestation prefix families. This is the
[minimal-and-adequate claim](part_1_foundation.md#17-minimal-and-adequate--the-14-claim) defended at
its boundary: the 1+4 surface stays small because the temptation to grow it is, on inspection, always
the temptation to let a key vouch for itself.

### 4.1.1 `anti-pattern-delegation` — Delegation-laundering
<sub>budget 0.19pp · import #155 · from **CEG §13.3** · semantic id `anti-pattern-delegation`</sub>

The most dangerous anti-pattern because every individual step is *valid*. A chain
`delegates_to → delegates_to → … → attacker` routes trust, one well-formed hop at a time, to a terminal
attester the original delegator would never have approved. The grammar's one delegation primitive
([2.4.1](part_2_the_grammar.md#24-primitive--the-primitive-set-14)) is powerful precisely because it
chains — so the discipline lives in consumer policy, with three normative guards:

- **Depth cap.** Consumer policy MUST cap `delegates_to` traversal at **5 hops** by default
  (configurable); a chain longer than the cap is treated as `attestation:self_verify` only — no
  transitive trust survives the overreach.
- **Cycle rejection.** The substrate MUST detect cycles on the `delegates_to` graph (A → B → A) and
  reject the cycle-closing emission.
- **Weight concentration.** Consumer policy SHOULD cap the trust weight any single terminal delegate
  can accumulate from one root at **0.5 × root_trust** by default.

These three caps are load-bearing far beyond this section: the [moderation chain](#4510-takedown-moderation--moderation-as-a-delegable-duty)
(4.5.10) and the [Self-at-login agency grant](#4434-policy-l--selffamily-membership-composition)
(4.4.3.4) both inherit the 5-hop cap as their structural revocability bound.

### 4.1.4 `withdraws-arbitrage` — `withdraws` arbitrage
<sub>budget 0.11pp · import #226 · from **CEG §13.4** · semantic id `withdraws-arbitrage`</sub>

The grammar distinguishes `withdraws` ("I no longer assert this") from `recants` ("this was false") —
and the [coherence ratchet](part_6_the_coherence_mathematics.md) penalises acknowledged error more than
plain retraction. A misattester can therefore *arbitrage*: always `withdraws`, never `recants`, to dodge
the penalty. The mitigation is behavioural, not wire-format: consumer policy MUST track each attester's
rolling `withdraws:recants` ratio and downweight any attester past a configured threshold (default 5:1)
regardless of which primitive they reach for. The distinction in the grammar still matters; the
anti-arbitrage countermeasure is consumer-side analysis layered over it.

### 4.1.3 `already-rejected` — Already-rejected wire additions
<sub>budget 0.13pp · import #205 · from **CEG §13.1** · semantic id `already-rejected`</sub>

A standing table of additions that have been proposed and refused, each smuggling a *verdict* where a
*mechanism* belongs — and the correct expression for each:

| Rejected | What it smuggles | Correct expression |
|---|---|---|
| `detection:emergent_deception:{axis}` | a moral verdict ("deception") in the prefix | `detection:correlated_action:{axis}` — mechanism-descriptive |
| `attestation:l{N}:*` | a ladder *position* (verdict-shape) on the wire | bare mechanism (`self_verify` / `hardware_rooted` / `registry_consensus` / …); the L1–L5 ladder is composed by [Policy I](#4436-policy-i--attestation-ladder-composition) |
| `score:trustworthiness:{entity}` | meta-judgement as its own prefix | compose downstream from `licensure:*` / `capacity:*` / `provenance:*` |
| `flag:bad_actor:{axis}` | pejorative wire vocabulary | low-confidence `provenance:*` / `coherence_standing:*` scores; adjudicate via quorum |
| `grounding:{tradition}:{principle}` | interpretive "tradition" claims as if mechanism | reuse the `delegates_to` primitive |

The thread is T2 again: a prefix may name a correlation, a count, a time-window, a schema-check; it may
not name a deception, a harm, a virtue, or a sin.

### 4.1.5 `registry-rejections` — Stress-test rejections (CIRISRegistry#30)
<sub>budget 0.11pp · import #231 · from **CEG §13.2** · semantic id `registry-rejections`</sub>

A second rejection table, harvested from a 30-story substrate stress test, catching the *Cartesian
shortcut* — the lone-subject-declares-its-own-interior move — in its many disguises:
`epistemic_mode: introspection` (self-knowledge is not standing without external witness →
`witness_relation: self` + `confidence < 1.0` + pending composition); `epistemic_mode: testimony`
(reducible to `external` + `witness_relation: external`); standalone `transparency:{kind}` (disclosure
is constituted by *reception*, not announcement → `evidence_refs[]` + a witness's
`transparency_log:inclusion`); the `stake: civic|epistemic|dignitary` triple (each composes from
existing axes — e.g. dignity-harm belongs on `harm_class:dignity_harm`, what the *attested* loses, not
on `stake`, what the *attester* loses); `oversight_mode` synonyms (all map to existing HITL/HOTL/HOOTL);
`provenance_walk` as a wire primitive (a UX concern, composed consumer-side); and the recurring pull to
rename canonical capacity factors to "kid-friendly" labels (the canonical names map to a worked-out
ethical lattice that accessibility renames quietly degrade — translation belongs in a glossary, version-
pinned). *The detailed rejection tables (4.1.3 / 4.1.5) are migrated verbatim in Phase 4 with their
`legacy_ref` provenance ([`toc.tsv`](toc.tsv)).*

---

## 4.2 `accord` — The HUMANITY_ACCORD constitutional layer
<sub>budget 1.4pp · import #10 · from **CEG §9** · semantic id `accord`</sub>

This is the **single wire-format asymmetry in the entire federation**, and the structural root of
[autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy). Everywhere else, the
[Recursive Golden Rule](part_1_foundation.md#1132-structure-recursive--the-recursive-golden-rule-structural)
binds every participant symmetrically — a steward is bound by the rotation it imposes; revocation rules
apply to the steward's own records. Here, and *only* here, the constitution holds open one door it
cannot itself reach through.

**The asymmetry, stated plainly.** Three named human key holders hold authority to invoke a full
constitutional halt — `EmergencyShutdown CONSTITUTIONAL` — that **no federation-side authority can grant
itself, revoke, override, or decay.** Not `SYSTEM_ADMIN`, not a `WISE_AUTHORITY` quorum, not the
regional stewards. There is *no federation-internal protocol path to that signature*. The reason is the
load-bearing argument of the whole document, [carried from M-1](part_1_foundation.md#14-autonomy--respect-for-autonomy):
consent is M-1's deepest property; consent is only real if it is *revocable*; and revocability requires
a halt-authority that lives **outside the system being halted.** The federation cannot deny humans the
right to stop it, because it is built so the off-switch is not wired into it. (Why this is not a
Golden-Rule violation is settled at [4.2.5](#425-isn--why-this-isnt-a-golden-rule-violation).)

### 4.2.3 `accord-holder` — The accord-holder triple
<sub>budget 0.16pp · import #172 · from **CEG §9.1** · semantic id `accord-holder`</sub>

Three named human key holders, hardware-attested, at federation genesis:

| Position | Holder | Threshold |
|---|---|---|
| 1 | Eric Moore | 2-of-3 |
| 2 | Eric Kudzin | 2-of-3 |
| 3 | Haley Bradley | 2-of-3 |

Permanent — no automatic decay; replacement requires an out-of-band CIRIS L3C process
(FEDERATION_ANNOUNCEMENT.md §4.5.3). Structurally, the triple **is** the canonical *entrenched `family`*
([2.x family subject_kind](part_2_the_grammar.md)): `family_key_id: "humanity-accord"`,
`consensus_protocol: "quorum:2/3"`, `consensus_protocol_entrenched: true`. The 2-of-3 multi-sig verifier
([4.2.1.1](#4211-invocation--invocation-canonical-bytes)) *is* that consensus protocol's enforcement;
entrenchment is what stops any federation-internal authority from amending it. The constitutional
asymmetry is therefore not a one-off primitive but a precise, reusable shape: **an entrenched family,
wire-scope-isolated to halt authority.** Other entrenched-family instances (a national-emergency triple,
a court-ordered preservation triple) MAY appear in operator deployments; HUMANITY_ACCORD is the one
CIRIS L3C ships at genesis.

**The correlated-failure geometry, named honestly.** Two of the three holders share a household, so the
2-of-3 quorum is physically assemblable from one street address — a real correlated-compromise/coercion
surface that entrenchment makes *harder*, not easier, to correct later. This is not softened: the
authority at stake is the full constitutional kill, not a recoverable pause. What scope isolation
([4.2.1](#421-authority--authority-scope)) *does* guarantee is that a compromise cannot escalate beyond
the halt — accord keys cannot sign grants, licences, or amendments. **The standing mitigation is to
diversify the holder set** — finding new holders via the out-of-band process so that no household, and
ultimately no single jurisdiction, can assemble the quorum. That is an active obligation on CIRIS L3C,
recorded here as a duty, not a deferred nicety.

### 4.2.1 `authority` — Authority scope
<sub>budget 0.63pp · import #40 · from **CEG §9.2** · semantic id `authority`</sub>

The halt-authority is doubly fenced — **wire-isolated AND scope-isolated.** `HUMANITY_ACCORD`
signatures are valid *only* on:

- `EmergencyShutdown CONSTITUTIONAL` (`IncidentSeverity::INCIDENT_CONSTITUTIONAL = 5`),
- `accord:invoke:notify:{notify_id}`, `accord:invoke:drill:{drill_id}`, `accord:lifecycle:active`, and
- the corresponding `FederationAnnouncement` priority `AccordCarrier`.

Any announcement of any *other* priority signed by an accord-holder key is **rejected at admission, out
of role.** And the fence runs both ways: federation-side authority cannot sign `AccordCarrier`;
humanity-accord authority cannot sign anything else. The halt-authority is thereby the narrowest
possible blade — it can stop everything and *do* nothing else. This is the [fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)
posture applied to power itself: the most dangerous key in the system is the one with the smallest
reachable surface.

#### 4.2.1.1 `invocation` — Invocation canonical bytes (anti-replay)
<sub>budget 0.19pp · import #154 · from **CEG §9.2.1** · semantic id `invocation`</sub>

**This is genesis-critical normative wire content — rendered here exactly.** Every `accord:invoke:*`
Contribution signs the following canonical bytes, binding **both** a discriminator **and** a
per-invocation nonce into the signed payload, so that a CONSTITUTIONAL signature can never be replayed as
a `notify`, nor a `drill` as a CONSTITUTIONAL (the cross-invocation-replay hole CEG 0.1's red-team review
found):

```
canonical = sha256(
    "ciris.accord_invoke.v1\n" ||
    "invocation_kind=" || ("CONSTITUTIONAL" | "notify" | "drill") || "\n" ||
    "invocation_id="   || halt_id_or_notify_id_or_drill_id || "\n" ||
    "nonce="           || base64url(rand_32_bytes) || "\n" ||
    "asserted_at="     || rfc3339_canonical || "\n" ||
    "valid_until="     || rfc3339_canonical || "\n" ||
    "payload_sha256="  || sha256_hex_lowercase_of_payload
)
```

Each of the 2-of-3 holders signs `canonical` independently with a hybrid Ed25519 + ML-DSA-65
bound-payload signature; a consumer verifies all three against the *same* `canonical` bytes and counts
≥ 2 valid. The substrate MUST reject duplicate `invocation_id` values within the `valid_until` window
(per-kind unique). This encoding is *deliberately not migrated* to the [namespace's JCS redesign](part_3_the_namespace.md):
its preimage is closed-vocabulary (discriminator + nonce + enum fields, no attacker-controlled free
text), so the injection surface that redesign closes is unreachable here — and genesis-critical bytes
stay stable. (Normative: CEG §9.2.1.)

#### 4.2.1.2 `notify` — `notify` vs `CONSTITUTIONAL` — the consumer-UI requirement
<sub>budget 0.11pp · import #227 · from **CEG §9.2.2** · semantic id `notify`</sub>

Wire-isolation alone does not close the *social*-engineering risk. A CEG-Conforming Consumer presenting
accord invocations to humans MUST visually distinguish the three kinds: **`CONSTITUTIONAL`** (kill-switch;
an unambiguous emergency banner), **`notify`** (federation-wide accord-holder communication — MUST NOT be
visually conflated with CONSTITUTIONAL), and **`drill`** (an exercise — MUST be marked, e.g. an explicit
`[DRILL]` prefix). This UI rule is the load-bearing safeguard against accord-holders being socially
pressured into emitting a `notify` that carries CONSTITUTIONAL *social* weight without CONSTITUTIONAL
*substrate* weight. (Normative: CEG §9.2.2.)

### 4.2.2 `hardware-class` — Hardware-class taxonomy
<sub>budget 0.33pp · import #89 · from **CEG §9.4** · semantic id `hardware-class`</sub>

The halt-authority is only as strong as the hardware its keys live in, so each accord-holder (and
production steward) key declares a `hardware_class`, with a recommended consumer trust-multiplier:

| Value | Use | Multiplier |
|---|---|---|
| `HSM_FIPS_140_3_L3` | production stewards (US / EU / APAC) | 1.0 |
| `Apple_Secure_Enclave` | accord-holders on iOS/macOS | 0.95 |
| `YubiKey_5_FIPS` | accord-holders preferring portable tokens | 0.95 |
| `TPM_2_0` | accord-holders on Linux/Windows desktops | 0.9 |
| `placeholder_pending_provisioning` | interim, pre-provisioning | **0.0** |
| `software_hsm_development` | development only | **0.0** (federation-scope MUST reject) |

The two `0.0` rows are the fail-secure floor: an unprovisioned or software-only key carries *no* trust
weight for federation-scope verification.

#### 4.2.2.1 `hardware-class-hardware` — The self-assertion gap (acknowledged)
<sub>budget 0.13pp · import #200 · from **CEG §9.4.1** · semantic id `hardware-class-hardware`</sub>

Honestly stated, in the spirit of [integrity](part_1_foundation.md#18-integrity--integrity): the
`hardware_class` field is *currently a self-asserted string*. CEG has no normative mechanism (TPM quote
chain, Apple attestation, FIDO attestation) for a verifier to independently corroborate it, so consumer
policy MUST treat it as a **producer claim, not a cryptographically-attested fact** — the trust-
multipliers above bind only as guidance until a planned roadmap item adds per-platform attestation-chain
verification. Stating the limit is itself the discipline.

### 4.2.4 `policy-concern` — Concern split: key material vs role-recognition policy
<sub>budget 0.11pp · import #301 · from **CEG §9.3** · semantic id `policy-concern`</sub>

A clean separation keeps the layer auditable: the **key material** (Ed25519 + ML-DSA-65 pubkeys for the
three holders) lives in the CIRISPersist substrate as `federation_keys` rows with
`identity_type="accord_holder"`, self-signed at provisioning and cross-attested by all three regional
stewards; the **role-recognition policy + verifier logic** (the 2-of-3 multi-sig check, the
`EmergencyShutdown CONSTITUTIONAL` admin RPC, the audit hooks) lives in `ciris-registry-core`. What the
keys *are* and what they're *recognised to do* are governed in different places, by design.

### 4.2.5 `isn` — Why this isn't a Golden-Rule violation
<sub>budget 0.11pp · import #302 · from **CEG §9.5** · semantic id `isn`</sub>

The [Recursive Golden Rule](part_1_foundation.md#1132-structure-recursive--the-recursive-golden-rule-structural)
binds *participants in the federation* to one another. Humanity-as-such occupies a position **outside**
the participant set, by design — it is not a peer the federation may bind. The three holders carry an
authority no federation-side class can grant itself, revoke, override, or decay; that is not an
*exemption* from the standard, but the recognition that **consent requires a halt-authority outside the
system being halted.** The one deliberate asymmetry is not a flaw in the symmetry — it is the condition
that makes the symmetry trustworthy.

---

## 4.3 `wise-authority` — Designated Wise Authorities
<sub>budget 0.88pp · import #26 · from **Accord (WA)** · semantic id `wise-authority`</sub>

Where the [HUMANITY_ACCORD](#42-accord--the-humanity_accord-constitutional-layer) is the *human* hand
that stops the machine, the **Wise Authorities** are the human judgement the machine *defers to* before
it ever reaches that extremity. They are the destination of [Wisdom-Based Deferral](part_1_foundation.md#19-deferral--wisdom-based-deferral-wbd)
(1.9) and the recipient of the [PDMA's seventh step](part_1_foundation.md#13-pdma--the-principled-decision-making-algorithm)
("Feedback to Governance"): when uncertainty exceeds threshold, when a dilemma is novel or
precedent-less, or when severe harm is possible with ambiguous mitigation, a CIRIS system does not
improvise — it routes the decision to designated Wise Authorities for adjudication.

A WA is a *role*, not a person, and it is held under the same discipline as every other authority in the
federation: granted (never assumed), scoped, hardware-attested, owner-bound to an accountable human, and
revocable. WA adjudication is the **primary substantive review** in the [amendment process](#451-amendment--amendment-process)
(4.5.1) and the moderation appeal path; it is structurally separated from the parties it judges by
**fresh-quorum recusal** — a reviewer who has standing in a matter is recused, and the quorum is drawn
fresh, so judgement does not flow back to the judged. That recusal is what the
[locality-scaled quorum](#4431-quorum--policy-e--locality-scaled-quorum) (4.4.3.1) is engineered to keep
*feasible* even in small cells.

WBD is humility made procedural — the operational face of the Accord's **Incompleteness Awareness.** It
is precisely *because* the systems pursuing M-1 are fallible that the federation routes their hardest
calls to a human authority that is itself bounded, recused, and accountable, rather than letting a system
pretend to a certainty it does not have. *The Accord's long-form Wise-Authority chapters (Book II §III)
are migrated verbatim in Phase 4 with their `legacy_ref` provenance ([`toc.tsv`](toc.tsv)); this section
is their distilled constitutional treatment.*

---

## 4.4 `composition-policies` — Composition policies
<sub>budget 0.34pp · import #88 · from **CEG §8** · semantic id `composition-policies`</sub>

Here the document turns from *what the federation refuses* to *how it decides what to believe.* The
division of labour is the spine of the whole system: **the substrate carries edges (attestations);
consumers compose traversals (verdicts).** The wire stays neutral — it never tells a consumer *what to
believe* — and CEG supplies a *library* of named, reference composition policies a consumer may pick
from. A CEG-Conforming Consumer MUST implement at least [Policy A](#4438-policy-a--direct-trust); the
rest are RECOMMENDED for richer compositions. This neutrality is [integrity](part_1_foundation.md#18-integrity--integrity)
at the verdict layer: two consumers reading the same edges may *legitimately* reach different verdicts,
because they declared different policies — and both are checkable.

### 4.4.1 `frickerian` — Frickerian discipline (consumer-policy norms)
<sub>budget 0.35pp · import #86 · from **CEG §8.3** · semantic id `frickerian`</sub>

A direct expression of [justice](part_1_foundation.md#112-justice--justice) at the composition layer.
After Miranda Fricker's *epistemic injustice*, consumers SHOULD apply identity-prejudice-resistant
weighting: **do not** downweight `testimonial_witness:*` from cohorts with low overall attestation
density (preserving that testimony is exactly what corrects for the low density); **do not** downweight
a `non_maleficence:*` claim about a partner merely because the partner has a long `partner_role:*` track
record (the long record may *be* the harm); and apply [Policy D](#4439-policy-d--lexical-vulnerability-priority)
lexical-vulnerability-priority in ties involving small cohorts. The discipline is honest about its own
limits — it is consumer-policy-only, an adversary can craft `testimonial_witness:victim_of_my_competitor`
to exploit the non-downweighting rule, so it binds **after** the structural safeguards (testimonial
witness is never sole evidence for `slashing:*`; self-relation claims are weighted against the attester's
track record), never before them.

### 4.4.2 `aggregation` — Aggregation semantics (opinionated defaults)
<sub>budget 0.24pp · import #119 · from **CEG §8.2** · semantic id `aggregation`</sub>

Per `(dimension, attested_key_id)`, the default aggregation is chosen by the dimension's polarity, and the
defaults are *fail-secure on purpose*: `signed` → **mean** of `score × confidence`; `boolean-via-score` →
**min** (any negative trumps positive — hard constraints like `prohibited:*` fail closed);
`positive-only` → **max** (any positive is conclusive); `-1.0 only` → **min**; `enumerated` →
**most-recent** from an authorised emitter; and detector dimensions
(`detection:correlated_action:*`, `ratchet:flag:*`) → **median**, which resists a single captured
detector pulling the mean. Consumers MAY override per dimension; these are the Conforming-Consumer
minimum.

### 4.4.3 `reference` — Reference policies
<sub>budget 0.23pp · import #127 · from **CEG §8.1** · semantic id `reference`</sub>

The named policy library itself. The heaviest members — the quorum rule, the membership policies (M / H /
L / K) — get real treatment below; the thinner direct-trust and graph policies are framed and migrated
in Phase 4. Read them as a menu ordered from cheapest-and-narrowest to richest-and-broadest.

#### 4.4.3.1 `quorum` — Policy E — Locality-scaled quorum
<sub>budget 0.72pp · import #31 · from **CEG §8.1.5** · semantic id `quorum`</sub>

The rule that makes [Wise-Authority recusal](#43-wise-authority--designated-wise-authorities) *feasible*
rather than merely *required*, and the highest-weighted composition policy in the Part. It scales quorum
size to decision locality — `local → 2`, `regional → 3`, `national → 4`, `federation → 6` (the reference
function is policy-tunable) — and pins the pool a *fresh-quorum recusal* needs:
`min_pool(scale) = quorum_size(scale) × 2`. Recusal is feasible when `cell_pool ≥ min_pool(S)`; an
attempt to decide above a cell's weight surfaces as a named *locality mismatch* failure rather than
silently over-reaching. Decision-scale matching becomes structurally enforced — a small cell cannot quietly
arrogate a federation-scale call.

##### 4.4.3.1.1 `sub-quorum` — Sub-quorum fallback
<sub>budget 0.48pp · import #61 · from **CEG §8.1.5.1** · semantic id `sub-quorum`</sub>

When `cell_pool < min_pool(S)` there is **no implicit fallback** — the consumer MUST take one explicit,
*observable* path: **scale-down** (re-attest at the next-lower locality where the smaller quorum is met,
emitting `hard_case:locality_scale_down`); **escalate** (emit `hard_case:locality_underpopulated`, route
to the federation cell, which by definition has the largest pool); or **liveness-defer** (emit
`hard_case:locality_quorum_unreachable`, defer until the pool grows, with the deferred state itself
reviewable). The recursion-safety floor is the constitutional tie-in: the [amendment process](#451-amendment--amendment-process)
routes through `locality:decision:federation`, whose pool is sized so `cell_pool ≥ min_pool(federation)`
always holds at genesis — and *if it ever falls below*, the entire amendment surface is in a
constitutional-crisis state that **only the HUMANITY_ACCORD CONSTITUTIONAL halt can resolve.** Even the
quorum mathematics, at its limit, hands back to the human halt-authority.

#### 4.4.3.2 `community-policy` — Policy M — Community membership composition
<sub>budget 0.65pp · import #36 · from **CEG §8.1.13** · semantic id `community-policy`</sub>

How a *community* resolves its current member set and gates `cohort_scope: community` content. The
membership resolution walks the latest non-superseded `community` Contribution under the community's
declared `consensus_protocol`; the result is a per-region materialised view of a signed, replicated
membership stream. Policy M is the sibling of [Policy L](#4434-policy-l--selffamily-membership-composition)
(self/family) but with a stronger confidentiality default, set out in its normative core below. *Member →
reachable-address resolution (the DNS-free `resolve_member_transport`), the geographic worked example, and
the delivery-mode extension are migrated verbatim in Phase 4 ([`toc.tsv`](toc.tsv)); their constitutional
weight is carried by the three sub-rules below.*

##### 4.4.3.2.1 `community-three` — The three crypto tiers + the Community DEK cascade (normative)
<sub>budget 0.45pp · import #68 · from **CEG §8.1.13.3** · semantic id `community-three`</sub>

The load-bearing privacy ruling of Policy M, and a direct service to
[non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence): the line between encrypted
and plaintext is drawn at **"does it have a bounded membership roster?"** — yes → encrypt, no →
plaintext:

| Tier | `cohort_scope` | At-rest | Reader |
|---|---|---|---|
| **self / family** | `self`, `family` | encrypted, per-write DEK; no wire discovery (structural invisibility) | occurrences / family members |
| **Community** | `community`, `affiliations` | **encrypted under the community DEK** + `holds_bytes:*` with cleartext provenance | community members (DEK cascade) |
| **Commons** | `species`, `biosphere`, `federation` | plaintext | anyone |

The **Community DEK cascade is MANDATORY, not opt-in** — *a persecuted community is protected by being a
community, not by remembering to set a flag.* One DEK is shared across the community's emissions
(per-emission cost O(1), not O(members)), wrapped to each member on admission, with **`wrap_algorithm: v2`
(hybrid X25519 + ML-KEM-768 PQC) MANDATORY** against harvest-now-decrypt-later. The
**holder-inspectability principle** anchors the shape: nothing a host holds above local tier is ever a
forced, unattributable opaque blob — the holder either inspects the *bytes* (Commons, plaintext) or
inspects the *provenance* (Community: the cleartext `attesting_key_id` + `community_id` + reason on an
otherwise-encrypted blob) and chooses to hold ciphertext for a community it trusts. The one exception is
`cohort_subkind: infrastructure` (e.g. `ciris-canonical`), which opts *out* to Commons-tier plaintext —
the trust root's whole purpose is public auditability, so it must not be opaque. (Normative: CEG §8.1.13.3.)

##### 4.4.3.2.2 `community-forward` — Forward secrecy on member removal (Option A)
<sub>budget 0.21pp · import #142 · from **CEG §8.1.13.4** · semantic id `community-forward`</sub>

Because a community now *has* a DEK, removing a member raises the can-still-decrypt concern — answered by
the federation's uniform **Option A** forward-secrecy posture: on removal, the substrate **rotates the
community DEK**, so subsequent emissions are sealed under a key the removed member lacks. "Once shared,
always shared" — content received during membership stays in the ex-member's cache (no post-compromise
re-keying of history); they receive no *new* community content. The same forward-only discipline as
`withdraws`, takedown, and consent-decay — one posture across every surface.

##### 4.4.3.2.3–4.4.3.2.7 — community admission, resolution, and composition *(page-thin tail)*

The remaining Policy-M sub-rules are framed here and migrated verbatim in Phase 4: **community admission**
per `consensus_protocol` + `cohort_subkind` (4.4.3.2.3); **membership resolution** (4.4.3.2.4) and its
**deterministic-resolution + member→address** normative pins (4.4.3.2.4.1 — identical resolution across
implementations is a 1.0 interop requirement, with reachability never standing in for trust); the
**geographic-community admission worked example** (4.4.3.2.5); the **`delivery_mode` × Policy M**
extension (4.4.3.2.6 — "subscribe = join the community"); and **composition with CEG 0.6 + 0.7**
(4.4.3.2.7). Each carries its `legacy_ref` in [`toc.tsv`](toc.tsv).

#### 4.4.3.3 `policy` — Policy H — Tiered-Scope Composition (LIVE)
<sub>budget 0.55pp · import #47 · from **CEG §8.1.8** · semantic id `policy`</sub>

The three-tier feed model: a **local_feed** (`cohort_scope: self` only; self-attested, no peer
weighting), a **community_feed** (`{family, community, affiliations}`; expertise *within* cohort matters,
cross-cohort downweighted unless invited), and a **global_feed** (`{species, biosphere, federation}`; full
federation weighting, fact-checkers carry weight, [Frickerian discipline](#441-frickerian--frickerian-discipline-consumer-policy-norms)
applied). All four `external_content` sub_kinds share one envelope shape, so they compose across the tiers
naturally; a Contribution is *promoted* from a narrower scope to a wider one not by a new `promote`
primitive but by [`supersedes`](part_2_the_grammar.md#24-primitive--the-primitive-set-14) (4.4.3.3.1) —
re-using a structural composer rather than growing the surface, with the promotion lineage walkable via
`references_attestation_id`.

##### 4.4.3.3.1 `supersedes` — Promotion via `supersedes`
<sub>budget 0.45pp · import #69 · from **CEG §8.1.8.1** · semantic id `supersedes`</sub>

The worked promotion pattern: emit a `supersedes` against the prior attestation with
`differs_in: ["cohort_scope", "sub_kind?"]`, reusing the prior `content_sha256` (no body re-upload) and a
wider `cohort_scope` (e.g. `self → community`, optionally morphing `sub_kind`, e.g. a private note → a
published encyclopedia entry). Wire-format clean, lineage preserved — the canonical demonstration that the
[1+4 set](part_1_foundation.md#17-minimal-and-adequate--the-14-claim) is *adequate*: a real operation
("widen the audience for this") needs no new primitive.

#### 4.4.3.4 `family-policy` — Policy L — Self/family membership composition
<sub>budget 0.54pp · import #49 · from **CEG §8.1.12** · semantic id `family-policy`</sub>

How the substrate resolves an identity's **self-collective** (its admitted occurrences — phone, laptop,
agent) or a **family**, and gates the at-rest DEK wrapping that lets self/family content reach its members
without ever emitting a public `holds_bytes:*`. Its constitutional centre of gravity is the
**Self-at-login composition** (4.4.3.4.3): a person's *app* and their *agent* are two occurrences of one
identity sharing one Self DEK, and at login the agent is *partnered + delegated* to act as the user — a
shape that composes `identity_occurrence` + Policy L + `consent:partnered` + `delegates_to` with **no new
structural primitive**, and whose two layers (co-self *visibility* vs. agency *act-on-behalf*) are
**independently revocable.** The key-grant cascade (4.4.3.4.1) is the wire mechanism — `wrap_algorithm: v2`
hybrid-PQC MANDATORY for the user's longest-lived data — and the membership-change admission (4.4.3.4.2)
runs the family's `consensus_protocol`, where `quorum:M/N` is **absolute-M** (4.4.3.4.2.1; the `N` is
documentary, never rebased against the live roster — so the gate is an unambiguous `count ≥ M`). A crucial
sub-ruling keeps [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) honest:
**infrastructure must not have agency** — a fabric/server node may receive partnership (`infra:*` scopes —
identity + membership standing under the user) but a verifier MUST reject any `infra`-only key presenting an
`agency:*` scope, making "infrastructure has no agency" a wire-checkable invariant. *The remaining Policy-L
sub-rules — self-collective / family resolution, forward secrecy (Option A again), the signing-member-set
JCS contract, and CEG 0.6 composition — are migrated verbatim in Phase 4 ([`toc.tsv`](toc.tsv)).*

#### 4.4.3.5 `policy-cem` — Policy K — CEM (consent) composition
<sub>budget 0.53pp · import #50 · from **CEG §8.1.11** · semantic id `policy-cem`</sub>

How consent state — the wire root of [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) —
is *resolved over time.* For any Contribution naming `subject_key_ids`, a consumer resolves each subject's
**effective consent** by walking that subject's latest non-superseded `consent:state:*` emission, gated by
`valid_until` (granted / revoked / expired / unspecified). The load-bearing ruling is **any-subject-binding
multi-subject revocation** (4.4.3.5.4): when a Contribution names several subjects, *each is an independent
revocation authority* — any one subject's admitted `withdraws` evicts the whole Contribution for everyone,
with no "majority-rules" softening. (A group photo: any one subject revokes → it is evicted.) This is the
subject-as-individual principle pushed to the wire, and it needs no new primitive — the withdrawal *is* the
eviction. The **deletion-SLA watcher** (4.4.3.5.2) gives Article-17-style erasure an observability
primitive: on revocation against a producer who committed a `consent:deletion_sla`, the substrate emits
`hard_case:consent_sla_breach` if the deadline passes uncompleted. *The bilateral PARTNERED ceremony, the
decay-protocol stage composition, the effective-consent read path, and the stream catalogue are migrated
verbatim in Phase 4 ([`toc.tsv`](toc.tsv)).*

#### 4.4.3.6–4.4.3.13 — the remaining reference policies *(page-thin tail)*

The thinner reference policies are framed here and migrated verbatim in Phase 4 with their `legacy_ref`
([`toc.tsv`](toc.tsv)): **Policy I** — Attestation-Ladder composition (4.4.3.6; the L1–L5 ladder is a
consumer-rendered verdict-shape over the bare *mechanism* prefixes — the rung number is composition, never
wire, exactly the [4.1.3](#413-already-rejected--already-rejected-wire-additions) `attestation:l{N}` lesson);
**Policy F** — `agent_files` trust (4.4.3.7; three layers — canonical / open-contribution / vote-then-trust —
with the anti-tricking guarantee that newcomers' *default* path is always the steward-attested canonical one,
binding CIRIS L3C to its own rule); **Policy A** — direct trust (4.4.3.8; the one mandatory policy: trust an
attester in the pinned bootstrap set); **Policy D** — lexical-vulnerability-priority (4.4.3.9; tie-break toward
the more-affected cohort); **Policy J** — Trusted-Publisher (4.4.3.10); **Policy G** — Trust-Fresh / Lighthouse
(4.4.3.11); **Policy B** — one-hop transitive (4.4.3.12); and **Policy C** — weighted-graph / EigenTrust-style
(4.4.3.13).

### 4.4.4 `sovereign-registered` — Sovereign-Registered equivalence
<sub>budget 0.2pp · import #146 · from **CEG §8.4** · semantic id `sovereign-registered`</sub>

The structural expression of [justice](part_1_foundation.md#112-justice--justice): a Sovereign agent
scoring `licensure:CA_medical_board: +1.0` is **wire-format identical** to a Registry-steward scoring the
same — the substrate is source-neutral; consumer policy weights by attester source. Both paths produce
federation membership; neither is a *gate*. What differs is only the *attestation surface* — the kind of
claim the federation can compose about *why* a participant is trustworthy. M-1's symmetry — standing
earnable without the steward's permission — is structural here, not bolted on.

---

## 4.5 `discipline` — Governance discipline
<sub>budget 0.21pp · import #140 · from **CEG §11** · semantic id `discipline`</sub>

The federation's self-governance: how the rules change, who may moderate, and how the dangerous power to
*remove* content is kept from becoming the power to *seize* it. The unifying thread is the
[operational-language gate](#456-admission-operational--operational-language-gate-at-admission): every
governance act enforces *published, mechanically-checkable rules*, never contested judgements of meaning —
the line that lets the federation be **safe without becoming a censor.**

### 4.5.1 `amendment` — Amendment process — Contribution + WA quorum + 1-of-6 sign-off
<sub>budget 0.85pp · import #27 · from **CEG §11.2** · semantic id `amendment`</sub>

How the rule layer itself changes — new prefixes, new envelope fields, new policies, calibration-version
transitions — and the highest-weighted governance concept. The path is five steps, defence-in-depth by
design: (1) a **proposed amendment** filed as a Contribution; (2) **witness diversity** (N=3 default); (3)
**WA quorum adjudication** — the *primary* substantive review by [Wise Authorities](#43-wise-authority--designated-wise-authorities);
(4) **reconsideration** with fresh-quorum recusal (per the [locality-scaled quorum](#4431-quorum--policy-e--locality-scaled-quorum));
and (5) a **1-of-6 accord-holder OR steward sign-off** — the *secondary* check, where **any single signer
can VETO by refusing to sign.** Step 5 reduces the attack surface from "produce N Sybils" to "compromise one
of six specific hardware-attested keys."

The deepest property here is **entrenchment** (4.5.1.2): the amendment process *itself*, the
[T1–T4 admission gate](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate), and the
[HUMANITY_ACCORD layer](#42-accord--the-humanity_accord-constitutional-layer) are entrenched — changing any
of these three surfaces requires a MAJOR version bump **AND a separate, dedicated 2-of-3 HUMANITY_ACCORD
ratification** (distinct from the step-5 sign-off). *Without this, a single quorum could rewrite the gate
admitting the next quorum* — entrenchment is the lock that stops the rules-of-rule-change from being captured
in one move.

#### 4.5.1.1 `axis` — Axis-vocabulary discipline
<sub>budget 0.7pp · import #32 · from **CEG §11.2.1** · semantic id `axis`</sub>

The discipline that keeps open vocabularies *meaningful*. Every `{axis}` value emittable under an
open-vocabulary prefix MUST carry either an **operational definition** (where the prefix has a
RATCHET-calibration package — measurement procedure, threshold function, statistical floor, evidence-shape
requirement, polarity semantics, all version-pinned via `evidence_refs[]`) **or** a **documented
convention** (for documentation-only vocabularies, discoverable in non-normative registries that need no
spec amendment to extend). This is [integrity](part_1_foundation.md#18-integrity--integrity) applied to
vocabulary growth: the namespace may grow at its open edges, but never into ungrounded terms.

#### 4.5.1.2 `meta-amendment` — Meta-amendment + entrenchment
<sub>budget 0.29pp · import #103 · from **CEG §11.2.3** · semantic id `meta-amendment`</sub>

The normative statement of the entrenchment ruling summarised at [4.5.1](#451-amendment--amendment-process):
the three entrenched surfaces (amendment process, admission gate, HUMANITY_ACCORD) require MAJOR-version
bump **plus** a dedicated 2-of-3 accord ratification to change. The structural root of why the rules of
rule-change cannot be quietly rewritten from inside.

#### 4.5.1.3 `open-vocabulary` — Open-vocabulary collision rule
<sub>budget 0.11pp · import #225 · from **CEG §11.2.2** · semantic id `open-vocabulary`</sub>

When two parties independently register confusingly-similar `{kind}`/`{axis}` values: **first-registered
wins** the canonical name (earlier `signed_at` holds it; the later registration disambiguates); a
**Levenshtein-distance guard** (≤ 2 of an existing value) returns an *advisory* `409 IDEMPOTENT_CONFLICT`,
not a hard reject (intentional near-collisions like `commonsense` vs `commonsense_hard` are allowed to
proceed); and **no squatting** — a `{kind}` registered but unused for 90 days MAY be reclaimed via the
amendment process.

### 4.5.10 `takedown-moderation` — Moderation as a delegable duty — `moderate` / `takedown` / `review`
<sub>budget 0.26pp · import #109 · from **CEG §11.10** · semantic id `takedown-moderation`</sub>

The constitutional heart of the moderation chain — placed ahead of its lighter siblings because it carries
the most weight. **Moderation is a delegable *duty*, not a platform- or fabric-assigned role.** A
participant exercises a `moderate` / `takedown` / `review` duty *as themselves*, or delegates it — to their
*agent* (AI on-behalf-of) or to *any trusted party* — via [`delegates_to`](part_2_the_grammar.md#24-primitive--the-primitive-set-14),
with **no new structural primitive.** The grammar spans two layers and unifies them:

- **Open labeling — anyone, no authority.** Anyone MAY file a `scores` Contribution against anything they
  see (an *opinion*, not an action); consumers compose **filters** over the score graph (hide / blur /
  annotate / down-rank) as pure consumer policy. Stackable, swappable, subscribed by choice.
- **Authoritative action — the enforced duty.** Hiding-for-yourself needs no authority; *acting on a
  group's behalf* (a takedown, an authoritative ModerationEvent, an appeal ruling) requires the delegated
  duty.

The **enforced-admission rule** (normative wire content, rendered faithfully) is the load-bearing safeguard:
a moderation action is admitted **iff** it holds *positively* under either **(a) as-self** — `attesting_key_id`
itself holds the duty over the target (the content's own subject, or the target community's named-moderator) —
or **(b) delegated** — a live `delegates_to` chain `root →* attesting_key_id` where *every edge bears the
matching scope*, the *root holds the duty over the target* and is *owner-bound* (an accountable human), depth
≤ 5, and *no edge is `withdraws`-revoked*. Otherwise **REJECT. Absence of a principal field is NEVER an admit
condition** — there is deliberately *no `on_behalf_of` envelope field*: a side-field would both break the 1+4
lockdown and open a bypass (omit-the-field ⇒ admitted-as-self would make the gate a no-op exactly where it is
load-bearing). The principal is **discovered by walking the chain upward from `attesting_key_id`**, never read
from a payload. Subject authority is likewise resolved from the content's **signed establishing attestation**,
never the action payload's self-declared `subject_key_ids` (fail-secure: if the establishing attestation is not
locally held, the subject-self clause *fails* — only the named-moderator path remains).

Sub-delegation **attenuates, never expands** (`child.scope ⊆ parent.scope`; constraints may be added, never
removed — the capability rule shared by UCAN / macaroons / SPKI-SDSI), is permitted only if the delegator
granted `sub_delegation`, and is revocable at any link (a `withdraws` against any edge invalidates everything
downstream). This is the **"takedown-isn't-a-coup" property made structural:** every action is delegate-signed,
delegator-traceable, owner-bound at the root, and revocable — *coordinated + attributable + revocable*, never a
unilateral seizure. A no-authority actor, or a state actor demanding removal of `federation_keys` for whole
classes of dissenters, **fails the gate and escalates to the [HUMANITY_ACCORD](#42-accord--the-humanity_accord-constitutional-layer)**
([4.5.3](#453-takedown--fast-path-takedown-coordination)). The dangerous power to remove is fenced exactly as
the dangerous power to halt is. (Normative: CEG §11.10.)

### 4.5.4 `registry-named` — Named-moderator existence invariant + merit auto-promotion
<sub>budget 0.42pp · import #72 · from **CEG §11.11** · semantic id `registry-named`</sub>

**No unmoderated multi-party space, ever.** A `community` operates and federates *only while it has ≥1
active holder of its `moderate` duty* — the moderation analogue of owner-binding, and a direct service to
[non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence) (it closes the
unmoderated-space-for-predators gap of relay / immutable-store / lax-instance models). Three normative
rules: (1) an **existence gate** — a community with no active owner-bound `moderate`-holder is
non-conformant, and the creator names one at creation; (2) **merit auto-promotion** — when the named
moderator lapses (revoked or inactive past the freshness window), the member with the highest
`moderation_track_record` is *automatically* granted the duty, by deterministic selection (track-record,
then earliest membership, then lexicographic `key_id`, so every peer auto-promotes the *same* member — one
code path, resolvable identically to a hand-named appointment); and (3) **fail-secure** — if no eligible,
consenting, owner-bound member exists, the community **fails-secure and MUST NOT federate at moderated
capability.** *Better no group than an unmoderated one* — degrade, never escalate. And merit grants the
*seat*, not *fiat*: every action the auto-promoted moderator takes is still constrained by the
[operational-language gate](#456-admission-operational--operational-language-gate-at-admission) and the
recused-reviewer appeals path, and the duty itself remains revocable and re-auto-promotes on lapse — so
capture is bounded. **No new structural primitive** — the invariant rides existing `delegates_to` +
`scores`.

### 4.5.3 `takedown` — Fast-path takedown coordination
<sub>budget 0.55pp · import #46 · from **CEG §11.4** · semantic id `takedown`</sub>

Some harms cannot wait for the [amendment process](#451-amendment--amendment-process). For
`takedown_notice` Contributions in the **immediate-removal** category (`TvecTerrorist` / `NcmecCsam` /
`GifctCip` / `PerceptualHashCsam` / `CourtOrder`), CEG carves out a fast-path: **speed at the action layer,
authority checked at the delegation layer.** The substrate accepts the signed notice *without* quorum, emits
a `withdraws` against the matching `holds_bytes:*` directory entry (holders see the eviction and SHOULD cease
serving), and dispatches per legal basis (TVEC's 1-hour obligation; GIFCT's CIP channel; NCMEC's CyberTipline
report with hash-only retention, *no content retention*; a court's stated timeline). Every fast-path takedown
enters a `hard_case:fast_path_takedown` Contribution for downstream review, and immediate-removal bases bypass
counter-notice by design (the `expeditious-with-counter-notice` bases route through the standard amendment path
on counter-notice).

The constitutional guarantee is the **"takedown-isn't-a-coup" property**: the fast-path coordinates *removal*,
it does not grant *seizure*. A `takedown_notice` targeting the substrate itself — a state actor demanding
takedown of `federation_keys` for whole categories of dissenting participants — does **not** propagate the same
way; substrate-protective discipline and [HUMANITY_ACCORD](#42-accord--the-humanity_accord-constitutional-layer)
veto authority intersect at the substrate level, and operators facing this conflict escalate to the accord
triple per [4.2.1.1](#4211-invocation--invocation-canonical-bytes). The speed serves victims; the halt-authority
guards against the speed being turned into a weapon.

### 4.5.7 `registry-watchlist` — Watchlist auto-detection — opt-in, per-group, separation-of-powers
<sub>budget 0.14pp · import #185 · from **CEG §11.12** · semantic id `registry-watchlist`</sub>

Auto-detection done *without* becoming surveillance. A `moderate`-scope holder may **optionally** enable a
watchlist for a group they moderate; the fabric fires the matcher at the publish/share seam and auto-fires
the action (CSAM → `takedown_notice{PerceptualHashCsam}`; other → `detection:*` + a ModerationEvent). The
discipline is three normative invariants: **opt-in, per-group, NEVER global** (a "scan everything" config is
non-conformant — that is the bulk-surveillance posture the framework refuses); **separation of powers** (no
single party does all three — the *fabric* holds the mechanism but cannot provision the hash-DB or enable it;
the *operator* holds the licensed hash-DB + the report obligation but cannot turn it on for a group; the
*authority* holds the opt-in but cannot run the match or access the list); and **audit, never silent** —
enablement and *every* match emit `hard_case:watchlist_*`, and *disabling a CSAM list MUST be an audited act*
(silent removal of CSAM detection is barred, so a predator-operator cannot quietly switch it off). Honest
scope: detection reaches only the publish/share seam of enabled groups — never self/family private content (the
universal E2EE limit, not claimed solved), and no client-side scanning is mandated. **No new structural
primitive.**

### 4.5.2 `compliance` — Vertical compliance + subject-bearing dimension governance
<sub>budget 0.6pp · import #43 · from **CEG §11.6** · semantic id `compliance`</sub>

How the domain-agnostic wire primitives — `subject_key_ids`, the `consent:*` family, the
[deletion-SLA watcher](#4435-policy-k--cem-consent-composition) — *compose into* regulatory compliance
without the wire ever learning a regulation's name. CEG documents the mappings as **informational** (GDPR
Articles 7 / 9 / 17 / 20, HIPAA, FERPA, CCPA, the EU AI Act's training-data opt-out all express as
compositions of existing primitives) and is explicit that it does **not** prescribe which framework an
operator must follow — operators pin compliance mappings as *configuration above* the wire, never as new wire
shapes. The one **normative** rule (4.5.2.1) is the [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy)
default-leak gate: a dimension whose namespace pattern *names a subject* (e.g. `observed:user:{key}:*`,
`epistemic:about:{key}:*`) MUST carry `subject_key_ids` containing that subject — closing the failure mode
where subject-bearing content publishes without wire-level subject authority. The same "isn't-a-coup" parallel
applies: a state actor publishing `observed:user:dissenter:*` with empty `subject_key_ids` does not get
admitted; the gate binds uniformly. *The informational mapping table and the CEG-0.6 documentation notes are
migrated verbatim in Phase 4 ([`toc.tsv`](toc.tsv)).*

### 4.5.6 `admission-operational` — Operational-language gate at admission
<sub>budget 0.2pp · import #151 · from **CEG §11.1** · semantic id `admission-operational`</sub>

The gate every governance act passes through, and the line that separates *safety* from *censorship.* Every
new prefix admitted to the namespace passes the
[four-test T1–T4 admission gate](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate);
failed admissions are revised (mechanism-descriptive reframe) or rejected. Moderation, takedown, and
watchlist enforcement act *only* on mechanically-checkable, publicly-proposed rules — never on contested
judgements of meaning. **Safety enforces published mechanism; censorship enforces opinion** — and this gate is
where the federation stays on the right side of that line.

### 4.5.5 `bootstrap-content` — Bootstrap-content pattern
<sub>budget 0.12pp · import #221 · from **CEG §11.3** · semantic id `bootstrap-content`</sub>

After genesis, a curated batch of Contributions is admitted through the amendment flow to seed the
federation's substantive content surface with high-quality ethical-framework material — **content-neutral by
design**: any sufficiently substantive source can serve, and the admission gate ensures prefix names don't
import source-tradition vocabulary. The first deployment maps the *Magnifica Humanitas* encyclical;
subsequent batches draw from CARE Principles (Indigenous data governance), Buddhist economic-justice
scholarship, secular humanist instruments, and African philosophy of personhood — *multi-traditional by
design*, each through the same gate. This is M-1's "diverse sentient beings" honoured at the content layer:
the framework's ethics are seeded from many traditions, none privileged in the wire.

### 4.5.8–4.5.12 — the governance tail *(page-thin tail)*

The remaining governance sections are framed here and migrated verbatim in Phase 4 with their `legacy_ref`
provenance ([`toc.tsv`](toc.tsv)):

- **4.5.8 `identity-set-2`** — `identity_type` as a *set*, so one key may cohabit several roles
  (`X ∈ identity_type`); semantically null for legacy single-role keys, with defence-in-depth discipline that
  an `accord_holder` key cohabiting other roles draws elevated scrutiny (the halt-authority is strongest when
  single-purpose) and substrate roles keep their steward cross-attestation (4.5.8.1–4.5.8.4).
- **4.5.9 `registry-geographic`** — the geographic-community privacy invariant: joining is a *one-way*
  disclosure — **rough-only** (`cell_resolution ≤ 7`, wire-format-enforced — a malformed UI cannot publish
  precise location), **opt-in** (the substrate never solicits location; only the subject's signature mints a
  `location_proof`), and **forward-only on leaving** (4.5.9.1–4.5.9.4). A state actor demanding involuntary
  location disclosure is the substrate-protective case the HUMANITY_ACCORD exists to address.
- **4.5.10 `registry-hash`** — hash-database operator policy: the self-hosted-PDQ default path, the access-
  gated landscape (PhotoDNA / Arachnid / GIFCT), and the deferred future hash-coalition slot
  (4.5.10.1–4.5.10.4).
- **4.5.11 `bootstrap-content`** — *(rendered above at 4.5.5 by importance; see [4.5.5](#455-bootstrap-content--bootstrap-content-pattern)).*
- **4.5.12 `family-self-2`** — self/family membership governance: forward-secrecy Option A on member
  departure, multi-family `family_id` routing, single-vouch self-occurrence admission, and the
  consensus-protocol-governed family-admission rules (4.5.12.1–4.5.12.6).

---

*Part IV is the federation's hinge. Read its weight honestly: the federation's two deepest commitments are
the [anti-patterns](#41-anti-pattern--anti-patterns) (the wire cannot brand a person) and the
[HUMANITY_ACCORD](#42-accord--the-humanity_accord-constitutional-layer) (consent requires a stop-button the
system cannot reach) — and everything between them, the composition policies and the governance machinery,
exists to make trust **composable, attributable, and revocable** without ever letting the power to remove or
the power to change become the power to capture. Its deep tail — the per-policy pseudocode, the worked
admission flows, the regulatory mapping tables — is migrated verbatim in Phase 4 with full `legacy_ref`
provenance ([`toc.tsv`](toc.tsv)); the importance graph keeps it page-thin here because the federation leans
on the halt-authority and the enforced-admission rule, while the mechanism minutiae are consulted, not read.*

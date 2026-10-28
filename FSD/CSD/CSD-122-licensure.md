# CSD-122 — Licensure (any authority, under its own key, says who may practise)

**CSD**: CSD-122 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) + CSD/4 (`CIRISServer/FSD/CSD4.md`, catalogue entry T6, `csd:tests` in §3a) · **Origin**: the registry fold, FSD-005; CIRISRegistry#139, CIRISConstitution#100, #162, #164; the CC ruling on cirisconstitution-32 (2026-10-28)
**Flow**: unwritten
**Reads with**: CSD-120 (the authority is an affiliation), CSD-124 (a `license`-scoped delegate issues on its behalf; the worked chain to `ciris-canonical` is there), CSD-123 (the recognition that sits beside a licence), CSD-052 (the attestation ladder — not this), `COMMON_CARDS_ANALYSIS.md` (KEEP)

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**An authority — an organization or a single key — can issue a licence to a
subject, put it on probation or restrict it, suspend and reinstate it, revoke it,
record that it lapsed, and attach the duties that come with it; a subject can
see every licence held under every authority with every live status; and a
reader anywhere can look a key up and get the status set, never a blended
scalar.** Serves **Fidelity** and **Justice**, on CC 2.4.1.2.1 and 3.1.1:
`licensure:{authority_id}` is open-emitter — "anyone may be an authority under
their own key" — and `suspended` is reversible while `revoked` is terminal, "a
status a consumer MUST NOT infer from the other."

**Nobody sells a licence, and CIRIS least of all (steward, 2026-10-28).** The
Portal issued CIRIS licences for a fee by tier (Community, Professional,
Enterprise, Safety-Critical) through Stripe; CIRIS is no longer selling licences
and portal.ciris.ai is being decommissioned. The Portal screens this card stood
in for — `RegisterPartner`'s licence half, `RevokeEntity(license)`,
`ListExpiringLicenses` — are **retired, not migrated**. The model is fully
decentralised: anyone can be their own licensor; it only matters who trusts
them. A reader chooses how it trusts (CC 4.4.3.8), and the shipped default root
is trusted by default, which is how the accord's own licences work out of the
box.

**A CIRIS licence is a row under the root community (CC ruling,
cirisconstitution-32, citing existing text).** The CIRIS L3C licensing authority
is the humanity accord trust root acting through the root community it founded,
`ciris-canonical`, whose founders are the accord holders A1/B1/C1 (CC 3.2,
`CLM-canonical-founders-accord`). So the dimension is
**`licensure:ciris-canonical:v1`** — the root community's `community_key_id`,
NOT `licensure:humanity-accord:v1`: the accord family is a witnessed lineage and
a kill switch, not a licensor. The row is a **cosigned row** admitted under the
cosigned-row rule (CC 2.1: "the family declares its `consensus_protocol`
source … for `licensure:{community_key_id}` the authority named in the
dimension") at its signed `asserted_at`, cosigned by that community's quorum
(`quorum:2/3`, a strict majority of three founders) **or** signed by a key
holding a plain `license`-scoped `delegates_to` from `ciris-canonical`, itself
cosigned under that quorum (CC 2.4.1.2.1, 3.3.9, 4.4.3.4.3). **There is no
`infra:license` and there must be none**: `license` is an agency scope in the
CC 4.4.3.4.3 table, and a node key (identity_type ⊇ node) may carry only
`infra:*` scopes, so a licence can never be issued by an install. The `license`
delegation is an ordinary cosigned row the reader walks, never a member of the
GenesisBundle (CC 3.2 T5, 5.3.4: the bundle carries the charter, the holders,
the serve nodes and the birth record, and nothing else is a root).

**Any affiliation is its own licensor, on the same rule.** A state board, a
clinic, a guild issues `licensure:{its community_key_id}:v1` cosigned under its
own `consensus_protocol`. A reader honours it only if its own root's trust
reaches that authority (CC 4.4.3.8: the pinned trust set is a graph whose roots
are pluggable; the default pin is `ciris-canonical` + `humanity-accord`). A
Sovereign key scoring `licensure:CA_medical_board` is wire-identical to a
steward scoring it (CC 4.4.4); the reader's policy weights it.

**A licence does not chain.** Holding one confers no power to issue one. The only
way a key other than the authority issues `licensure:A` is a `license`-scoped
delegation from A (CSD-124), refused at admission otherwise
(`licensure_delegator_not_authority`). The card never offers "delegate this
licence"; it offers "delegate the right to issue".

**The authority of an organization is its affiliation id** (CC 3.3.9 rc12:
"a partner is `licensure:{community_key_id}:v1` with `partner_role:{role}` for
the tier and `duty:{kind}` for obligations"). The Portal's `license_type` enum
is gone from the wire: the practised domain is the authority, the obligations are
`duty:{kind}`, and the typed licence payload persist v60 carries
(`partner_record` as an envelope member on the `licensure:` row, with the
`revision` anti-rollback against the authority's own held licences) replaces the
Portal's `PartnerRecord`.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:licensure
screen: proposed:Licensure
```

Placement: Communities and Businesses › Rules for the issuing side (an
affiliation's licences), and My things › Devices & keys for the holding side (my
licences). One screen, two scopes.

```yaml csd:shows
registry_sha256: d587156605dd8de7c0c56e271625293eb05b3f9b5debd44cf8bc822d4e701c5b
fields:
  - ceg: "licensure:{authority_id}"
    bind: {authority_id: ciris-canonical}
    use: emit
    type: "enum[issued,probation,restricted,suspended,revoked,lapsed,surrendered,reduced]"
    example: "issued"
    renders: "one row per (subject, authority, status): 'Issued by ciris-canonical on {when}, until {valid_until}'; several statuses may be live at once and each is its own row; the bind is the worked example (a CIRIS licence), any affiliation's id goes in its place"
    tag: "proposed:row_licence_*"
    assert:
      one_of: {"proposed:row_licence_*": [issued, probation, restricted, suspended, revoked, lapsed, surrendered, reduced]}
  - ceg: x_private:licence_status_set
    use: display-only
    type: "list[string]"
    example: ["issued", "probation"]
    renders: "'Standing: issued, on probation' — the SET, composed (persist `licensure::status_set_for`); never 'active' as a scalar; `revoked` present ⇒ terminal, shown first"
    tag: "proposed:txt_licence_standing_*"
  - ceg: x_private:authority_id
    use: display-only
    type: string
    example: "ciris-canonical"
    renders: "'Authority' — the affiliation (named) or the key; the licence's provenance (CC 2.4.1.2.1); for ciris-canonical the card says 'the root community, founded by the accord holders' and nothing grander"
    tag: "proposed:txt_licence_authority_*"
  - ceg: x_private:cosigned_under
    use: display-only
    type: string
    example: "quorum:2/3 — A1, B1 signed"
    renders: "'Signed by' — the scrub set that met the authority's protocol at asserted_at (persist v60 item 1: the full scrub set ∩ the roster at the instant, evaluated under the community's own protocol); one member's signature alone reads 'testimony, not a licence'"
    tag: "proposed:txt_licence_cosigned_*"
  - ceg: x_private:issued_under_delegation
    use: display-only
    type: string
    example: "att-7f3a…"
    renders: "'Issued by {key}, under a licence-scoped delegation from {authority}' when the issuer is not the authority; the delegation's receipt opens (CSD-124)"
    tag: "proposed:txt_licence_issued_under_*"
  - ceg: x_private:asserted_at
    use: display-only
    type: timestamp
    example: "2026-10-28T09:00:00Z"
    renders: "'As of' — authority is judged at the row's signed asserted_at (CC#162 as landed; persist v60 item 2); a later withdrawal of the issuer's delegation stops future issuance and does not reclassify this row; a row signed more than 72 h ago whose standing no longer holds at receipt is `asserted_at_stale`, shown as refused"
    tag: "proposed:txt_licence_as_of_*"
  - ceg: "duty:{kind}"
    bind: {kind: inform}
    use: emit
    type: "list[string]"
    example: ["inform", "review_policy"]
    renders: "'Comes with' — the duties attached (ODRL kinds: compensate, attribute, inform, obtain_consent, ensure_exclusivity, delete, next_policy, anonymize, review_policy); supervision-required becomes `duty:review_policy` with the supervisor named"
    tag: "proposed:list_licence_duties_*"
  - ceg: x_private:valid_until
    use: emit
    type: timestamp
    example: "2027-10-01T00:00:00Z"
    renders: "'Until' on the row; past it, the row is not live (CC 2.1) and the card says 'expired' — NOT `lapsed`, which only the authority can say"
    tag: "proposed:txt_licence_until_*"
  - ceg: x_private:reinstate
    use: emit
    type: bool
    example: true
    renders: "'Reinstate' on a suspended row — a cosigned `withdraws` on the suspended row, never a new `issued`; absent on a revoked row"
    tag: "proposed:btn_licence_reinstate_*"
  - ceg: x_private:licence_audience
    use: display-only
    type: "enum[federation,affiliations,named_readers]"
    example: "federation"
    renders: "'Who can verify this' — the cohort_scope the licence's CLASS declares in the charter (`licence_audiences`, CSD-120), the smallest that fits (CC 1.13.3.4); a practice licence meant for strangers is federation-scope by its nature, an internal credential is not. There is no public default. The authority head (roster, protocol, `license` edges) travels on its own planes at a scope a verifier can reach"
    tag: "proposed:txt_licence_visibility"
  - ceg: x_private:re_emitted_to
    use: emit
    type: "list[string]"
    example: ["wa-state-board-verifier-3c0e"]
    renders: "'Readers' — present only for a class not declared public; the re-emission is a NEW cosigned row (`reemission_of` = the held licence's id) at the same scope with the readers in subject_key_ids, payload and status unchanged (CC#162 as landed; ruling #163; persist v60 item 3), never a carriage of the public row; refusals shown by name: `re_emission_names_no_reader`, `re_emission_target_unknown`, `re_emission_diverges`"
    tag: "proposed:list_licence_readers"
  - ceg: x_private:subject_view
    use: display-only
    type: "list[string]"
    example: ["ciris-canonical: issued", "aff-state-board-77c1: suspended"]
    renders: "'Licences about you' — every licence row this node holds whose subject is me, by authority, with its status set (CC#164 as landed: a subject can see every licence about them); a claim about a person they cannot see fails ciris.ai/values (informed agency)"
    tag: "proposed:list_my_licences"
  - ceg: x_private:appeal
    use: emit
    type: string
    example: "reconsideration:suspension_unfounded"
    renders: "'Contest' on a row about me — files a `reconsideration:{grounds}` (CC 3.1.9.2; CC#164) addressed to the authority; a subject's own `withdraws` against a licence is refused (persist v60 `SUBJECT_WITHDRAWS_CARVE_OUT`); the row stays as it is until the authority's set, plus any `review`-scoped chain from it, acts"
    tag: "proposed:btn_licence_contest_*"
  - ceg: "attestation:license_validity"
    use: display-only
    type: "enum[verified,single_source,unknown]"
    example: "single_source"
    renders: "'Verify has not co-signed this licence (confidence ≤ 0.5)' — CC 3.4.9 composition for a licence under the default root only; absent for every other authority"
    tag: "proposed:chip_licence_cosigned_*"
```

**No card ranks authorities.** A reader decides which authorities to trust (CC 4.4.4;
ciris.ai/constitutional-mesh: "every client chooses which roots to trust"). The card
lists what is held and who said it; it never sorts a state board above a sole
practitioner, and never sorts `ciris-canonical` above either.

**`lapsed` and `expired` are different words on purpose.** A row whose
`valid_until` has passed is no longer live; `lapsed` is the authority saying so
on the record. The card never fabricates a `lapsed` or a `revoked` from a date
(FSD-004 §4.3).

**Nothing on this card has a price.** No fee, no bond, no tier, no "contact
sales"; a licence class is a line in the charter, and whether anyone honours it
is the reader's trust.

```yaml csd:states
populated: {tag: "proposed:list_licences"}
empty:     {tag: "proposed:txt_licences_empty", renders: "No licences issued by this organization yet. / You hold no licences this node knows of."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_licences_error", renders: "Couldn't read licences from this node. This is not a report that none are held."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| issue / change status (write a `licensure:{authority_id}:v1` scores row about the subject) as an act of the affiliation under its current `consensus_protocol` — the `/changes/{envelope,cosign,assemble}` pattern; one signer only where the protocol says so | none today | CIRISServer | `blocked_by: CIRISServer#752` (the doors) |
| re-emit a licence to named readers (`reemission_of`, `subject_key_ids`) | none | CIRISServer | `blocked_by: CIRISServer#752`; admission confirmed (persist v60 item 3); carriage `blocked_by: CIRISEdge#895` (`CLM-licence-audience` staged there) |
| reinstate (a cosigned `withdraws` on the suspended row) | none | CIRISServer | `blocked_by: CIRISServer#752` |
| attach a duty (`duty:{kind}:v1` row referencing the licence) | none | CIRISServer | `blocked_by: CIRISServer#752` |
| read a subject's licences per authority, as a status set; list by authority | none (FSD-004 proposed `GET /v1/licensure/{key_id}?authority=`) | CIRISServer | `blocked_by: CIRISServer#752`; the persist read and list-by-authority shipped (CIRISPersist#1035) |
| admission refuses an issuer that is neither the authority's quorum nor a `license`-scoped delegate (`licensure_delegator_not_authority`); authority judged at `asserted_at`; `asserted_at_stale` past 72 h | CIRISPersist | CIRISPersist | confirmed — v60.0.0 §8 items 1–2 (CIRISPersist#1036) |
| the authority of an affiliation resolves to whatever its active `consensus_protocol` admits | `admission::licence_authority_set` | CIRISPersist | confirmed — v60.0.0 §8 item 1 |
| the typed licence payload (`partner_record` member), `revision` anti-rollback, skew bound | CIRISPersist | CIRISPersist | confirmed — v60.0.0 §8 item 4 |
| a subject's `withdraws` against a licence refused; `reconsideration:{grounds}` is the contest | CIRISPersist | CIRISPersist | confirmed — v60.0.0 §8 item 5 (CC#164) |
| the `license` delegation from `ciris-canonical` to an issuing key (the CIRIS worked chain) | CSD-124 | CIRISServer | `blocked_by: CIRISServer#752`; no genesis-bundle change is asked and none is wanted |

## 3a. Topology (CSD/4)

T6 is the entry: its `authority` relation ("a licensing authority is affiliated",
`authority.licensing`) is the one CSD4.md §5 says waits on this card set.
`requirements.yaml` carries no CSD-122 yet; the block is the entry to add.

```yaml csd:tests
CSD-122:
  catalogue_version: 3
  topology: T6
  tests:
    issued_by_the_authority:
      at: [authority.licensing, community.member]
    issued_by_a_delegate:
      at: [authority.licensing, delegation.chain]
```

What this card adds to the T6 layer graph (CSD-120 §3a):

```yaml
  relations:
    # PENDING (builder refuses `authority` by name until a licence has a route — CSD4.md §5):
    - {rel: authority, community: AFF, by: authority}            # authority.licensing: AFF issues licensure:AFF:v1
    # PENDING: a licence ROW as a relation — licence(authority: AFF, subject: practitioner, status: issued, cosigned: founder_only)
    #          and its status transitions (suspend → reinstate → revoke); TOPOLOGY.md has no row-level relation for scores rows
    - {rel: delegation, from: authority, to: practitioner, scopes: [license], depth: 1, over: AFF, term_days: 90}
  checks:
    relations:
      - authority: {AFF: {dimension: "licensure:AFF:v1", protocol: founder_only}}
      - licence:   {subject: practitioner, status_set: [issued], asserted_at: "≤ now", cosigned: true}
    actor: [reads: "licences of practitioner under AFF" → status set [issued], issued_under: the founder]
  negatives:
    # PENDING: cannot_act — a member without `license` issuing licensure:AFF is refused licensure_delegator_not_authority
    - {check: cannot_act, person: practitioner, op: "issue licensure:AFF:v1", reason: licensure_delegator_not_authority}
```

**The CIRIS case in this vocabulary** needs an `infrastructure` root with
seated founders (`roots[].kind: infrastructure`, `founders.seated: 3`,
`quorum: 2`), which TOPOLOGY.md §2.1 types and the builder refuses until the
ceremony can mint it; T8 builds the accord halt on that root but not a licence.
The card therefore tests the CIRIS chain on T6's key root with `AFF` standing
for `ciris-canonical` and `founder_only` standing for `quorum:2/3`, and says so.

## 4. Flow

Unwritten at `envisioned`. The chain: the affiliation issues → suspends → the
status set reads `issued, suspended` → reinstates (withdraws) → reads `issued` →
a `license`-scoped delegate issues a second licence and the row names the
delegation → a member without the scope is refused at the door → the subject
contests one row and the row is unchanged → the authority revokes and the row
is terminal.

## 5. Acceptance

Unsigned. Not tested here: Verify's co-signature of a licence under the default
root (CC 3.4.9), the attestation ladder (CSD-052), the accord's halt (CSD-129).

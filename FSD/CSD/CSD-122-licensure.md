# CSD-122 — Licensure (any authority, under its own key, says who may practise)

**CSD**: CSD-122 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-005; CIRISRegistry#139, CIRISConstitution#100
**Flow**: unwritten
**Reads with**: CSD-120 (the authority is an affiliation), CSD-124 (a `license`-scoped delegate issues on its behalf), CSD-123 (the partner tier that sits beside a licence), CSD-052 (the attestation ladder — not this)

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

**A licence does not chain.** Holding one confers no power to issue one. The only
way a key other than the authority issues `licensure:A` is a `license`-scoped
delegation from A (CSD-124), refused at admission otherwise. The card never
offers "delegate this licence"; it offers "delegate the right to issue".

**The authority of an organization is its affiliation id** (CC 3.3.9 ruling on
CIRISPersist#814, re-based by FSD-005: `authority_id` = `community_key_id`). The
Portal's `license_type` enum is gone from the wire: the practised domain is the
authority, the tier is `partner_role:{role}` (CSD-123), the obligations are
`duty:{kind}`.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:licensure
screen: proposed:Licensure
```

Placement: Communities and Businesses › Rules for the issuing side (an
affiliation's licences), and My things › Devices & keys for the holding side (my
licences). One screen, two scopes.

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
fields:
  - ceg: "licensure:{authority_id}"
    bind: {authority_id: aff-ciris-l3c-9f2a}
    use: emit
    type: "enum[issued,probation,restricted,suspended,revoked,lapsed,surrendered,reduced]"
    example: "issued"
    renders: "one row per (subject, authority, status): 'Issued by {authority} on {when}, until {valid_until}'; several statuses may be live at once and each is its own row"
    tag: "proposed:row_licence_*"
    assert:
      one_of: {"proposed:row_licence_*": [issued, probation, restricted, suspended, revoked, lapsed, surrendered, reduced]}
  - ceg: x_private:licence_status_set
    use: display-only
    type: "list[string]"
    example: ["issued", "probation"]
    renders: "'Standing: issued, on probation' — the SET, composed; never 'active' as a scalar; `revoked` present ⇒ terminal, shown first"
    tag: "proposed:txt_licence_standing_*"
  - ceg: x_private:authority_id
    use: display-only
    type: string
    example: "aff-ciris-l3c-9f2a"
    renders: "'Authority' — the affiliation (named) or the key; the licence's provenance (CC 2.4.1.2.1)"
    tag: "proposed:txt_licence_authority_*"
  - ceg: x_private:issued_under_delegation
    use: display-only
    type: string
    example: "att-7f3a…"
    renders: "'Issued by {key}, under a licence-scoped delegation from {authority}' when the issuer is not the authority; the delegation's receipt opens"
    tag: "proposed:txt_licence_issued_under_*"
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
    renders: "'Reinstate' on a suspended row — a `withdraws` on the suspended row, never a new `issued`; absent on a revoked row"
    tag: "proposed:btn_licence_reinstate_*"
  - ceg: x_private:licence_visibility
    use: display-only
    type: "enum[federation,affiliations,named_readers]"
    example: "federation"
    renders: "'Who can verify this' — the scope the licence's CLASS declares in the charter (CSD-120), the smallest that fits (CC 1.13.3.4); a practice licence meant for strangers is federation-scope by its nature, an internal credential is not. There is no public default. The authority head travels with the licence at this scope"
    tag: "proposed:txt_licence_visibility"
  - ceg: x_private:re_stamped_readers
    use: emit
    type: "list[string]"
    example: ["wa-state-board-verifier-3c0e"]
    renders: "'Readers' — present only for a class scoped to named readers; the re-emission is NEW cosigned rows (the licence and the authority head) with the readers in subject_key_ids (CIRISConstitution#162 as landed), never a carriage of the public rows"
    tag: "proposed:list_licence_readers"
  - ceg: x_private:subject_view
    use: display-only
    type: "list[string]"
    example: ["aff-ciris-l3c-9f2a: issued", "aff-state-board-77c1: suspended"]
    renders: "'Licences about you' — every licence row this node holds whose subject is me, by authority, with its status set; a claim about a person they cannot see fails ciris.ai/values (informed agency)"
    tag: "proposed:list_my_licences"
  - ceg: x_private:appeal
    use: emit
    type: string
    example: "reconsideration:suspension_unfounded"
    renders: "'Contest' on a row about me — files a `reconsideration:{grounds}` (CC 3.1.9.2) addressed to the authority; the row stays as it is until the authority acts (ciris.ai/constitutional-mesh: authority is 'open to appeal')"
    tag: "proposed:btn_licence_contest_*"
  - ceg: "attestation:license_validity"
    use: display-only
    type: "enum[verified,single_source,unknown]"
    example: "single_source"
    renders: "'Verify has not co-signed this licence (confidence ≤ 0.5)' — CC 3.4.9 composition for the CIRIS-issued licence only; absent for every other authority"
    tag: "proposed:chip_licence_cosigned_*"
```

**No card ranks authorities.** A reader decides which authorities to trust (CC 4.4.4;
ciris.ai/constitutional-mesh: "every client chooses which roots to trust"). The card
lists what is held and who said it; it never sorts a state board above a sole
practitioner.

**`lapsed` and `expired` are different words on purpose.** A row whose
`valid_until` has passed is no longer live; `lapsed` is the authority saying so
on the record. The card never fabricates a `lapsed` or a `revoked` from a date
(FSD-004 §4.3).

```yaml csd:states
populated: {tag: "proposed:list_licences"}
empty:     {tag: "proposed:txt_licences_empty", renders: "No licences issued by this organization yet. / You hold no licences this node knows of."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_licences_error", renders: "Couldn't read licences from this node. This is not a report that none are held."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| issue / change status (write a `licensure:{authority_id}:v1` scores row about the subject) as an act of the affiliation under its current `consensus_protocol` — the `/changes/{envelope,cosign,assemble}` pattern; one signer only where the protocol says so | none today | CIRISServer | `blocked_by: FSD-005 §7` |
| re-stamp a licence and the authority head to named readers | none | CIRISServer, CIRISPersist | `blocked_by: FSD-005 §7` |
| reinstate (a `withdraws` on the suspended row) | none | CIRISServer | `blocked_by: FSD-005 §7` |
| attach a duty (`duty:{kind}:v1` row referencing the licence) | none | CIRISServer | `blocked_by: FSD-005 §7` |
| read a subject's licences per authority, as a status set | none (FSD-004 proposed `GET /v1/licensure/{key_id}?authority=`) | CIRISServer | `blocked_by: FSD-005 §7` |
| admission refuses an issuer that is neither the authority nor a `license`-scoped delegate (`licensure_delegator_not_authority`) | CIRISPersist | CIRISPersist | unconfirmed — FSD-005 §7 |
| the authority of an affiliation resolves to whatever its active `consensus_protocol` admits, selectable at founding and changeable by the active quorum | CIRISPersist | CIRISPersist | unconfirmed — FSD-005 §7 |

## 4. Flow

Unwritten at `envisioned`. The chain: the affiliation issues → suspends → the
status set reads `issued, suspended` → reinstates (withdraws) → reads `issued` →
a `license`-scoped delegate issues a second licence and the row names the
delegation → a member without the scope is refused at the door → the authority
revokes and the row is terminal.

## 5. Acceptance

Unsigned. Not tested here: Verify's co-signature of the CIRIS-issued licence
(CC 3.4.9), the attestation ladder (CSD-052).

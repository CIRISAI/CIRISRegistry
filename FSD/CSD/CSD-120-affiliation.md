# CSD-120 — Affiliation (an organization is a community gathered by necessity)

**CSD**: CSD-120 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-005
**Flow**: unwritten
**Reads with**: CSD-102 (founding and governance of any community — this card is the affiliation-specific charter on top of it), CSD-121 (who is in it), CSD-124 (the hierarchy under it)

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**A person can found an organization under their own key, declare what kind of
institution it is and how it handles what it holds, see those declarations as
every member sees them on joining, nest it under a parent, and dissolve it — and
no administrator anywhere is asked.** Serves **Justice** and **Contextual
Integrity**, on CC 4.4.3.2.8: an affiliation gathers by necessity, shares the
community machinery, and adds institutional governance "through one declared
config record … visible to members on joining — there are no undeclared
institutional powers."

**An organization IS this card.** The Portal's `CreateOrganization` was a
`SYSTEM_ADMIN` act writing an `organization` row; FSD-005 §1 rules the
organization is the affiliation. Founding is self-founding (CC 3.2: a community
is born by its founders' signatures), and `org_id` is the affiliation's
`community_key_id`, which is also the `authority_id` of every licence it issues
(CSD-122).

**Mandatory-with-default, not mandatory-to-author.** CC 4.4.3.2.8 ships a no-op
default and archetype presets, so a five-person committee sets one field
(`affiliation_archetype: informal_adhoc`) and is served by construction. The card
must make the small case small: a founder who picks an archetype and nothing
else has a complete charter.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:affiliation-charter
screen: proposed:AffiliationCharter
```

Placement: Communities and Businesses › Rules, mounted on the hub beside
CSD-102's governance section (the community IS the circle's group; the charter
is the affiliation's addition). The founding flow is CSD-102's founding flow with
`cohort_scope: affiliations` and one more step, the charter.

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
fields:
  - ceg: x_private:community_key_id
    use: display-only
    type: string
    example: "aff-ciris-l3c-9f2a"
    renders: "the affiliation's id, shown as the id its licences will carry: 'Licences you issue are signed as licensure:aff-ciris-l3c-9f2a'"
    tag: "proposed:txt_affiliation_id"
  - ceg: x_private:affiliation_archetype
    use: emit
    type: "enum[informal_adhoc,nonprofit_board,healthcare_provider,legal_practice,gov_body,igo,custom]"
    example: "healthcare_provider"
    renders: "'What kind of institution' — a preset that pre-fills everything below; `custom` opens the matrix"
    tag: "proposed:select_affiliation_archetype"
  - ceg: x_private:membership_basis
    use: emit
    type: "enum[voluntary-total,ascriptive,role-assigned,employment-contract]"
    example: "employment-contract"
    renders: "'Why people belong' — the necessity axis, orthogonal to the tier (the kibbutz catch)"
    tag: "proposed:select_membership_basis"
  - ceg: x_private:classification_scheme
    use: emit
    type: "list[string]"
    example: ["internal"]
    renders: "'Classes of what we hold' — default one class, `internal`; each class binds a crypto tier, retention, disclosure bar"
    tag: "proposed:list_classification"
  - ceg: x_private:retention_policy
    use: emit
    type: "list[string]"
    example: ["internal: rotate-forward"]
    renders: "'How long, per class' — floor and ceiling per class; the default rides the noise floor unchanged"
    tag: "proposed:list_retention"
  - ceg: x_private:disclosure_posture
    use: emit
    type: "enum[privacy-seeking,transparency-seeking]"
    example: "privacy-seeking"
    renders: "'Who can read what we publish' — whole cohort under the DEK, or per-class promotion to Commons"
    tag: "proposed:select_disclosure"
  - ceg: x_private:consensus_protocol
    use: emit
    type: string
    example: "quorum:2/3"
    renders: "'How this organization decides' — the full range of persist's quorum models, chosen at founding (CSD-102 offers it for any community) and changeable later by a decision of the active quorum; the same rule signs a licence, a role, a charter change"
    tag: "proposed:select_consensus_protocol"
  - ceg: x_private:parent_affiliation
    use: emit
    type: string
    example: "aff-ciris-l3c-9f2a"
    renders: "'Part of' — nests this affiliation under a parent (CC 4.4.3.2.8 C: hierarchy may nest sub-affiliations); replaces parent_org_id"
    tag: "proposed:input_parent_affiliation"
  - ceg: x_private:dissolves_at
    use: emit
    type: timestamp
    example: "2027-01-01T00:00:00Z"
    renders: "'Winds down on' — optional; a committee that ends with its event leaves no zombie cohort"
    tag: "proposed:input_dissolves_at"
  - ceg: x_private:charter_version
    use: display-only
    type: int
    example: 3
    renders: "'Charter version 3 — changed by {who} on {when}'; a change is a supersession of the record, never an edit"
    tag: "proposed:txt_charter_version"
  - ceg: "membership:{stage}"
    bind: {stage: acceptance}
    use: display-only
    type: int
    example: 12
    renders: "'12 members have accepted this charter' — a member sees the declarations on joining; their acceptance row is the evidence"
    tag: "proposed:txt_charter_accepted_by"
```

**Every emitted field is a member of one signed record.** The charter is the
affiliation's config record, superseded as a whole (CC 4.4.3.2.8: "DECLARED at
creation"). A card that saved fields one at a time would produce N versions for
one edit and N receipts for one act.

```yaml csd:states
populated: {tag: "proposed:card_affiliation_charter"}
empty:     {tag: "proposed:txt_charter_none", renders: "This organization has declared nothing beyond the default — one class, rotate-forward, privacy-seeking. That is a complete charter."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_charter_error", renders: "Couldn't read this organization's charter from this node. This is not a report that it has none."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| found an affiliation | `POST /v1/communities` with `cohort_scope: affiliations` | CIRISServer | unconfirmed — see FSD-005 §7 |
| write / supersede the charter (config record) | none today | CIRISServer, CIRISPersist | `blocked_by: CIRISServer#649` |
| read the charter | none today | CIRISServer | `blocked_by: CIRISServer#649` |
| nest under a parent | charter member `hierarchy` | CIRISPersist | unconfirmed |
| dissolve | CSD-102's dissolve | CIRISServer | unconfirmed |

## 4. Flow

Unwritten at `envisioned`. The steps a flow will drive: found with an archetype →
read back the charter → supersede one field → the version increments and the
previous version is still readable → a second person joins and the acceptance row
names the charter version they accepted.

## 5. Acceptance

Unsigned. What this card will not test: the DEK cascade and forward secrecy on
removal (CSD-102's), lawful-access declarations (CC 4.4.3.2.8 D — a later card,
never covert, always `hard_case:*`).

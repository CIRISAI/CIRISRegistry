# CSD-123 — Partners (recognition, tier, forums: the half of a partner that is not a licence)

**CSD**: CSD-123 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-004 §4.2 / FSD-005
**Flow**: unwritten
**Reads with**: CSD-122 (the licence half), CSD-120 (both sides are affiliations), CSD-124 (what the partner may do on the org's behalf is a delegation, never a partner field)

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**An organization can recognise another organization or key as a partner at a
named tier, state the forums they share, see every partner it recognises and
every organization that recognises it, and end a recognition — and a reader can
tell recognition apart from licence.** Serves **Fidelity**: CC 2.4.1.2.1 keeps
"who grants the licence", "who vouches for the attester" and "what external force
it carries" apart, and FSD-004 §4.2 split `RegisterPartner` into a recognition
half and a licence half for the same reason.

**A partner is three families, none of them `partner_record`.** The Portal's
`partner_record` bundled recognition (`partner_id`, `org_id`), standing
(`license_type`, `status`), capability (`capabilities_granted/denied`,
`max_autonomy_tier`) and obligation (`requires_supervisor`, limits) in one
quorum-signed row. On rc7 those are: the tier, `partner_role:{role}:v1`; the
standing, a licence (CSD-122); the obligations, `duty:{kind}:v1` on the licence;
the capabilities, the `scope` of a `delegates_to` where they are agency (CSD-124)
and the licence where they are standing; forum participation,
`multilateral_participation:{forum}:{kind}:v1`. The monotonic `revision` the
record carried is replaced by the row plane's own rules: a status change is a new
row, a withdrawal is forward-only.

**"Anyone can define their own partners."** Recognition is a `scores` row by the
recognising affiliation about the partner's key. It needs no steward quorum
outside the affiliation's own, and no registry. Whether a reader weights it is
consumer policy (CC 4.4.4).

## 2. Surface (what)

```yaml csd:surface
surface: proposed:partners
screen: proposed:Partners
```

Placement: Communities and Businesses › People (partners are the organizations
this one works with) with the issuing controls behind the affiliation's role
gate.

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
fields:
  - ceg: "partner_role:{role}"
    bind: {role: professional_medical}
    use: emit
    type: "enum[community,community_plus,professional_medical,professional_legal,professional_financial,professional_full]"
    example: "professional_medical"
    renders: "one row per partner: '{name} — professional_medical, recognised {when}'; the six canonical tiers, open vocabulary beyond them"
    tag: "proposed:row_partner_*"
  - ceg: x_private:partner_key_id
    use: emit
    type: string
    example: "aff-mercy-clinic-41d0"
    renders: "'Partner' — the other affiliation's id or a key; added by code or id, never by email"
    tag: "proposed:input_partner_key"
  - ceg: x_private:recognised_by
    use: display-only
    type: string
    example: "aff-ciris-l3c-9f2a"
    renders: "'Recognised by' — the row's attester, so an inbound recognition reads as THEIR claim about us"
    tag: "proposed:txt_partner_recognised_by_*"
  - ceg: "multilateral_participation:{forum}:{kind}"
    bind: {forum: ciris-medical-compact, kind: voting}
    use: emit
    type: "list[string]"
    example: ["ciris-medical-compact:voting"]
    renders: "'Forums' — membership / voting / proposal_filing / observer_status per named body"
    tag: "proposed:list_partner_forums_*"
  - ceg: x_private:licence_summary
    use: display-only
    type: "list[string]"
    example: ["issued"]
    renders: "'Licence: issued' — CSD-122's status set for this partner under THIS authority, opened from the row; absent when no licence is held, which is not an error"
    tag: "proposed:txt_partner_licence_*"
  - ceg: x_private:recognition_direction
    use: display-only
    type: "enum[outbound,inbound,mutual]"
    example: "mutual"
    renders: "'Mutual' when both affiliations recognise each other; CC 2.3.2.4: bilateral ratification is consumer policy, so this is a display fold, not a wire state"
    tag: "proposed:chip_partner_direction_*"
  - ceg: "revocation:{entity_type}:{reason}"
    bind: {entity_type: partner, reason: ended}
    use: emit
    type: string
    example: "revocation:partner:ended:v1"
    renders: "'End recognition' — a withdraws on the recognition row, plus this row when the reason is to be stated; forward-only, never a resurrect"
    tag: "proposed:btn_partner_end_*"
```

```yaml csd:states
populated: {tag: "proposed:list_partners"}
empty:     {tag: "proposed:txt_partners_empty", renders: "No partners recognised yet, and no organization has recognised this one that this node knows of."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_partners_error", renders: "Couldn't read partners from this node, so this list is not complete."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| recognise (write `partner_role:{role}:v1` about the partner, signed by the affiliation) | none today | CIRISServer | `blocked_by: FSD-005 §7` |
| forums (`multilateral_participation:*` rows) | none | CIRISServer | `blocked_by: FSD-005 §7` |
| read outbound and inbound recognitions | none (FSD-004 proposed reads over `partner_record`, now retired) | CIRISServer | `blocked_by: FSD-005 §7` |
| end (withdraws + optional `revocation:partner:*`) | none | CIRISServer | `blocked_by: FSD-005 §7` |
| the `partner_record` subject kind retired or marked transitional | CIRISConstitution, CIRISPersist | — | FSD-005 §6 |

## 4. Flow

Unwritten at `envisioned`. The chain: A recognises B at a tier → B's card shows
an inbound recognition → B recognises A → both read `mutual` → A issues B a
licence (CSD-122) and B's row shows it → A ends the recognition and the licence
is untouched, because they are two things.

## 5. Acceptance

Unsigned. Not tested here: the licence itself (CSD-122), capability intersection
for a deployment (`VerifyDeployment`, FSD-004 §6.3, still open).

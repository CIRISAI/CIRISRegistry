# CSD-124 — Authority delegations (the right to issue, handed down one attenuating step at a time)

**CSD**: CSD-124 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-005 §4–5; CIRISConstitution#100
**Flow**: unwritten
**Reads with**: CSD-055 (the same primitive for a person's devices and agents; this card adds the two emission scopes and the chain), CSD-121 (roles are these edges), CSD-122 / CSD-125 (what a `license` / `grant` scope lets a delegate issue), `formal/authority_tree/`

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**A principal — an organization's founders, or anyone holding `sub_delegation`
from them — can hand a member the right to issue licences or grants on the
organization's behalf, bounded to a subset of what the principal holds, for a
term, with or without the right to hand it on; everyone involved can see the
whole chain above any edge and how deep it is; and ending an edge shows, before
the act, every edge and every issuance right below it that ends with it.**
Serves **Core Identity** and **Integrity**, on the one rule FSD-005 §5 settles:
**only delegation chains**. A licence and a grant are leaves; the right to issue
them is the delegable thing, and it rides delegation's rules — attenuation
(CC 4.5), the depth cap of five (CC 4.1.1), cycle rejection, revocation cascade,
and `sub_delegation` as the only way a chain grows (CC 4.4.3.4.3).

**The root is a human, and authority decays.** Every chain this card shows ends at
an accountable person: the founders of an affiliation, or the person themselves
(CC 3.4.7.3; ciris.ai/constitutional-mesh). An edge defaults to a one-year term and
is renewed by re-conferral; the card shows an edge's remaining term beside its depth.

**Why this is not CSD-055.** CSD-055 is the device-code flow: a person offers a
code, a device approves it, the scopes are a person's own verbs. This card is
the same `delegates_to` primitive with two scopes CSD-055 does not offer —
`license` and `grant` — a delegator that is an affiliation rather than a person,
and the chain view CSD-055 marks `blocked_by: CIRISServer#663`. The two should
become one card when the chain read exists; until then this document carries the
affiliation half so the Portal's org-admin acts have a home.

**The enforced-admission rule is the card's whole contract.** CC 4.4.3.4.3: "an
issuance whose delegator does not itself hold the underlying authority is
refused at admission, not merely unweighted." The card therefore never lets a
delegator pick a scope it does not hold (TIGHTEN-ONLY, as CSD-055 already
implements for its scopes), and shows a refusal from the door as a refusal, by
its token, never as a success with a weaker row.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:authority-delegations
screen: proposed:AuthorityDelegations
```

Placement: Communities and Businesses › Rules (the affiliation's chain), and the
same card under My things for a person's own `license`/`grant` delegations
(a sole practitioner is their own authority).

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
fields:
  - ceg: x_private:delegated_scope
    use: emit
    type: "list[string]"
    example: ["license", "sub_delegation"]
    renders: "'May issue' — license (licences under this authority), grant (grants over assets this principal owns or stewards), sub_delegation (hand this on, one step deeper); each glossed in CC 4.4.3.4.3's words; TIGHTEN-ONLY against the delegator's own effective scope"
    tag: "proposed:list_delegation_scope_*"
  - ceg: x_private:delegate_key_id
    use: emit
    type: string
    example: "wa-registrar-3e90"
    renders: "'To' — a member's key (must be on the roster for an affiliation's edge)"
    tag: "proposed:input_delegate_key"
  - ceg: x_private:delegation_depth
    use: display-only
    type: int
    example: 3
    renders: "'3 of 5' — this edge's depth below the root; at 5 the card offers no sub_delegation, because the door will refuse it (CC 4.1.1)"
    tag: "proposed:txt_delegation_depth_*"
    assert:
      number: {"proposed:txt_delegation_depth_*": {min: 1, max: 5}}
  - ceg: x_private:delegation_chain
    use: display-only
    type: "list[string]"
    example: ["founders", "wa-admin-2b1c", "wa-registrar-3e90"]
    renders: "'From the founders, through {admin}, to {registrar}' — the chain above this edge, each link's receipt openable; the effective scope is the INTERSECTION along it, shown as such"
    tag: "proposed:list_delegation_chain_*"
    assert:
      count: {of: "proposed:list_delegation_chain_*", max: 6}
  - ceg: x_private:effective_scope
    use: display-only
    type: "list[string]"
    example: ["license"]
    renders: "'Effective: license' — what the delegate can actually issue after attenuation along the chain, which may be less than this edge's own scope says"
    tag: "proposed:txt_delegation_effective_*"
  - ceg: x_private:delegation_valid_until
    use: emit
    type: timestamp
    example: "2027-06-30T00:00:00Z"
    renders: "'Until' — defaults to one year; an expired edge confers nothing and the card says 'expired', never hides it"
    tag: "proposed:input_delegation_until"
  - ceg: x_private:revoke_cascade
    use: display-only
    type: "list[string]"
    example: ["wa-clerk-77a0 (license)", "3 licences issued under this edge stay issued"]
    renders: "in the revoke confirm: every edge below this one that ends with it, AND the statement that issuances already made under it stay (a licence is revoked by its authority, not by the delegate's fall — CC 2.4.1.2.1)"
    tag: "proposed:sheet_delegation_revoke_cascade_*"
  - ceg: x_private:admission_refusal
    use: display-only
    type: "enum[licensure_delegator_not_authority,scope_not_held,depth_exceeded,cycle,not_on_roster]"
    example: "scope_not_held"
    renders: "'The node refused this: you do not hold `grant` to hand on.' — the door's own token, shown as a refusal"
    tag: "proposed:txt_delegation_refused"
```

**Issued things outlive the edge they were issued under.** Withdrawing a
`license`-scoped edge stops the delegate issuing; it does not un-issue the
licences already admitted (their authority revokes them, CSD-122). The cascade
sheet says both, because an admin who thinks revoking a registrar revokes the
registrar's licences will act on the wrong belief.

```yaml csd:states
populated: {tag: "proposed:list_authority_delegations"}
empty:     {tag: "proposed:txt_authority_delegations_empty", renders: "Nobody has been delegated the right to issue for this organization. Only the founders can issue."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_authority_delegations_error", renders: "Couldn't read delegations from this node, so this list is not complete."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| write a `delegates_to` with scope ⊆ {license, grant, sub_delegation, …} from a person or an affiliation's quorum | none for `license`/`grant` scopes today | CIRISServer | `blocked_by: FSD-005 §7` |
| read inbound and outbound edges with chain and depth | none (`GET /v1/auth/device/grants` is flat) | CIRISServer | `blocked_by: CIRISServer#663` |
| withdraw an edge; the read reflects the cascade | partial (device grants revoke) | CIRISServer | `blocked_by: CIRISServer#663` |
| admission enforces attenuation, depth 5, cycle rejection, `sub_delegation`, and the `license`/`grant` underlying-authority rule | CIRISPersist | CIRISPersist | unconfirmed — FSD-005 §7 |
| the affiliation's quorum as the chain's root | CIRISPersist | CIRISPersist | unconfirmed — FSD-005 §7 |

## 4. Flow

Unwritten at `envisioned`. The chain `formal/authority_tree/` enumerates: root →
A (license, sub) → B (license) → B tries to issue to C and is refused (no
sub_delegation) → A tries to hand `grant` on and is refused (not held) → root
withdraws A → B's edge is dead, the depth read says so, B's earlier licence
issuance stands.

## 5. Acceptance

Unsigned. Not tested here: the device-code offer/approve flow (CSD-055), moderation
duties (CSD-090).

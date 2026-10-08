# CSD-121 — Members and roles (a role is a delegation, a member is a person who said yes)

**CSD**: CSD-121 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-005
**Flow**: unwritten
**Reads with**: CSD-103 (the roster as every community has one), CSD-106 (invites), CSD-124 (the delegation a role is), CSD-043 (users on a node — not this)

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**An organization's admin can invite a person, see who has accepted, give a
member a role, narrow or end it, and see — for every role — the delegation it
is, who conferred it, and what it lets the member do; and a member can see the
same about themselves.** Serves **Autonomy** (nobody joins without their own
signed acceptance, CC 3.1.3.2) and **Justice** (a role is a scoped, attenuated,
revocable delegation, CC 4.4.3.2.8 C and CC 4.5, never a flag an administrator
sets on a user row).

**The Portal's `org_membership.role` enum becomes four presets over delegation
scopes** (FSD-005 §4): `OrgAdmin` carries `sub_delegation` + `license` + `grant`
+ the affiliation's admin verbs; `KeyManager` the key-binding right;
`Operator` the operational verbs; `Viewer` nothing but membership. The card
offers the presets; the wire carries the scopes; the receipt shows the scopes.
An admin who wants a role no preset names picks the scopes.

**A member is a self, human or agent; the root is a human.** An agent joins through
its partnership (CC 4.4.3.4.3) and may hold a role (ciris.ai/philosophy: one
Constitution for NHI). The chain every role hangs from roots in the founders, who are
people (CC 3.4.7.3); a node is never a member of record and never a root.

**A role has a term.** The presets default `delegation_valid_until` to one year;
re-conferral renews it (ciris.ai/constitutional-mesh: authority "decays like everyone
else's"). A perpetual role is a founder's explicit choice.

**Membership and role are two rows and two lists.** Membership is the
`membership:{stage}` sequence onto the roster (CSD-103 renders it). A role is a
`delegates_to` edge from the affiliation's root, or from a holder of
`sub_delegation`, to the member. Removing a role leaves the membership;
removing the membership (CSD-103) cascades every role, because the roster no
longer names the key the edges point at.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:affiliation-roles
screen: proposed:AffiliationRoles
```

Placement: Communities and Businesses › People, beside CSD-103's roster
(`AffiliationsRoster`). One row per member; the role chips on the row open this
card's role sheet.

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
fields:
  - ceg: "membership:{stage}"
    bind: {stage: acceptance}
    use: display-only
    type: timestamp
    example: "2026-10-08T14:02:11Z"
    renders: "'Member since' — the instant of THEIR acceptance row, never the inviter's proposal"
    tag: "proposed:txt_member_since_*"
  - ceg: x_private:role_preset
    use: emit
    type: "enum[OrgAdmin,KeyManager,Operator,Viewer,custom]"
    example: "KeyManager"
    renders: "the preset chip; picking one fills the scope set below and shows it before signing"
    tag: "proposed:select_role_preset_*"
  - ceg: x_private:delegated_scope
    use: emit
    type: "list[string]"
    example: ["sub_delegation", "license"]
    renders: "'May do' — the scope set on the edge, each scope glossed in the CC 4.4.3.4.3 words; TIGHTEN-ONLY against the conferrer's own scope (CC 4.5)"
    tag: "proposed:list_role_scope_*"
  - ceg: x_private:conferred_by
    use: display-only
    type: string
    example: "wa-admin-2b1c"
    renders: "'Conferred by' — the delegator's key; 'the founders' when the edge is from the quorum"
    tag: "proposed:txt_role_conferred_by_*"
  - ceg: x_private:delegation_depth
    use: display-only
    type: int
    example: 2
    renders: "'2 steps below the founders' — the chain above this role, capped at 5 (CC 4.1.1); an admin approving a role sees whether it is direct or a sub-delegation"
    tag: "proposed:txt_role_depth_*"
    assert:
      number: {"proposed:txt_role_depth_*": {min: 1, max: 5}}
  - ceg: x_private:delegation_valid_until
    use: emit
    type: timestamp
    example: "2027-06-30T00:00:00Z"
    renders: "'Until' — defaults to one year from conferral; a founder may clear it; an expired role confers nothing and reads 'expired — renew?'"
    tag: "proposed:input_role_until_*"
  - ceg: x_private:role_revoke_cascade
    use: display-only
    type: "list[string]"
    example: ["wa-clerk-77a0", "wa-clerk-9c13"]
    renders: "in the revoke confirm: 'Ending this role also ends 2 roles it conferred' — the subtree a withdrawal severs (CC 4.5), named before the act"
    tag: "proposed:sheet_role_revoke_cascade_*"
```

**The depth row is the one CSD-055 could not fill** (`delegation_depth:
unconfirmed, blocked_by CIRISServer#663`). This card has the same dependency and
says so in §3; it cannot pass `building` until a chain read exists.

```yaml csd:states
populated: {tag: "proposed:list_affiliation_roles"}
empty:     {tag: "proposed:txt_roles_empty", renders: "No roles conferred yet — every member is a member only. The founders hold the root."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_roles_error", renders: "Couldn't read roles from this node, so this list is not complete."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| invite / accept / remove a member | CSD-106 / CSD-103 routes (`/v1/communities/{id}/…`) | CIRISServer | unconfirmed |
| confer a role (write a `delegates_to` from a member with `sub_delegation`, or from the founders' quorum) | none at affiliation scope today | CIRISServer | `blocked_by: FSD-005 §7` |
| read roles with chain and depth | none (`GET /v1/auth/device/grants` is a flat list of device grants) | CIRISServer | `blocked_by: CIRISServer#663` |
| end a role (a `withdraws` on the edge) | none | CIRISServer | `blocked_by: FSD-005 §7` |
| scope attenuation and depth cap enforced at admission | CIRISPersist | CIRISPersist | unconfirmed — FSD-005 §7 |

## 4. Flow

Unwritten at `envisioned`. The chain a flow will drive: founders confer
`OrgAdmin` on A → A confers `KeyManager` on B → A tries to confer a scope A does
not hold and is refused → the founders end A's role → B's role is gone, by
cascade, and the card showed that it would be before the act.

## 5. Acceptance

Unsigned. Not tested here: the roster's DEK rotation on removal (CSD-102), the
user accounts on a node (CSD-043, a different thing: a login, not a membership).

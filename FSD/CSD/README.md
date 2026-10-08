# CSDs for the Portal surface, on CEG rc7 — the registry's hand-off to the fold

**Status:** DRAFT, 2026-10-08. Stage `envisioned` (steward pen) under CSD/3
(`CIRISClient/CSD.md`). These documents are written here, in the repo that is folding, so
the Portal's surface is specified before the Portal is gone. On adoption they move to
`CIRISClient/FSD/CSD/` and keep their numbers (120–126 are free there; the client's
set ends at 110).

**Reads with:** `FSD/FSD-005_ORG_AS_AFFILIATION.md` (the translation these cards
render), `formal/authority_tree/` (the state tree the Delegations and Grants cards must
honour), `FSD/FSD-004_POST_FOLD_SURFACE.md` §4.3 (the PortalService methods each card
replaces).

**Registry pin:** CC 1.0-rc7 `registry_sha256`
`796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464` (168 families).

## The set

| CSD | Card | Replaces (PortalService / Portal screen) | Serves |
|---|---|---|---|
| CSD-120 | **Affiliation** (found, charter, dissolve) | CreateOrganization, UpdateOrganization, GetOrganization*, ListChildOrganizations | Justice, Contextual Integrity |
| CSD-121 | **Members and roles** | *OrgUser*, AddUserToOrg, RemoveUserFromOrg, UpdateUserOrgRole, ListOrgMembers | Autonomy, Justice |
| CSD-122 | **Licensure** (issue, suspend, reinstate, revoke, lapse) | RegisterPartner (licence half), RevokeEntity(license), ListExpiringLicenses | Fidelity, Justice |
| CSD-123 | **Partners** (the recognition half: role, duties, forums) | RegisterPartner (recognition half), UpgradeToPartner, GetPartnerActivity | Fidelity |
| CSD-124 | **Authority delegations** (`license` / `grant` / `sub_delegation`) | the org-admin half of every write above; extends CSD-055 | Core Identity, Integrity |
| CSD-125 | **Grants** (access to a thing) | — (new; the CIRISBilling resource-grant shape) | Contextual Integrity |
| CSD-126 | **Affiliation keys** (register, rotate, revoke, escrow) | GenerateKeyPair (dropped), ActivateKey, RotateKey, RevokeKey, Register*Key, RequestKeyEscrow/Recovery | Integrity |

Not in this set, because the client already has the card: audit (CSD-071), the device
delegation flow (CSD-055), community founding and governance (CSD-102/103; CSD-120 is
the affiliation-specific charter on top of it), key verification (CSD-104), users on a
node (CSD-043).

## The three rules every card in the set follows

1. **The signer is named before the act.** Every write is a signed row by a person's or
   affiliation's key; the confirm sheet says who signs, what row it is, and who can read
   it, as CSD-102 does. No card calls a route whose own auth is the authority.
2. **Only delegation chains.** A card never offers to delegate a licence or a grant; it
   offers to delegate the *right to issue* one (CSD-124), and the Licensure and Grants
   cards show which delegation an issuance was made under.
3. **Error is not empty.** A read the node could not serve says so; a licence that is
   not held is not "no licence".

## Stage and ownership

All seven are `envisioned`: §1 is falsifiable and every `ceg:` family resolves in the
rc7 registry. §2 surfaces are proposed (`nav_map` cannot resolve them until the client
places them); §3 contracts are `unconfirmed` with `blocked_by` naming the Server, Persist
or Edge gap, from the three repo audits recorded in FSD-005 §7.

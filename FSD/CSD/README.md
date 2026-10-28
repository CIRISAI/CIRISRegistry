# CSDs for the former Portal surface, on CEG rc12 — the registry's hand-off to the fold

**Status:** DRAFT, 2026-10-08; caught up to CSD/4, CC 1.0-rc12 and the 2026-10-28 rulings
on 2026-10-28. Stage `envisioned` (steward pen) under CSD/3 (`CIRISClient/CSD.md`), with
the CSD/4 `csd:tests` block (`CIRISServer/FSD/CSD4.md`, LOCKED catalogue). These documents
are written here, in the repo that is folding, so the Portal's surface is specified before
the Portal is gone. On adoption they move to `CIRISClient/FSD/CSD/` and keep their numbers
(120–126 are reserved there on CIRISClient#206; `numbers.tsv`).

**Reads with:** `FSD/FSD-005_ORG_AS_AFFILIATION.md` (the translation these cards
render; §9.4 and §9.6 carry the rulings), `formal/authority_tree/` (the state tree the
Delegations and Grants cards must honour), `FSD/FSD-004_POST_FOLD_SURFACE.md` §4.3 (the
PortalService methods each card stood in for), `COMMON_CARDS_ANALYSIS.md` (which client
cards already carry parts of this set, and what to fold where).

**Registry pin:** CC 1.0-rc12 `registry_sha256`
`d587156605dd8de7c0c56e271625293eb05b3f9b5debd44cf8bc822d4e701c5b` (188 families; the
grammar hash, `_meta.registry_sha256`). Every `ceg:` family the seven cards name resolves
in it (checked programmatically 2026-10-28; none was renamed between rc7 and rc12).

**Catalogue pin:** CSD/4 catalogue version 3, the client's `packaging/csd4/PIN.md`
(CIRISServer v0.5.233); the server's main is at version 5, and T6/T18/T19, the entries
these cards name, are unchanged across 3–5.

## The Portal is being decommissioned (steward, 2026-10-28)

CIRIS is no longer selling licences; portal.ciris.ai is being decommissioned. The CIRIS
L3C licensing authority is the humanity accord trust root acting through the root
community it founded, `ciris-canonical`; the model is fully decentralised — anyone can be
their own licensor, it only matters who trusts them; a reader chooses how it trusts, and
the shipped default root is trusted by default. So, in this set:

- Where a card replaced a Portal screen, **the screen is retired, not migrated**. The
  "Replaces" column below is historical: it records which PortalService method a card's
  act corresponds to, so FSD-004 §4.3 stays traceable.
- Every billing, Stripe, fee, bond, tier-price and "contact sales" dependency is gone
  from the cards. `partner_role:{role}` names a role, never a price tier (CSD-123); the
  Portal's six `license_type` tiers are retired; `CIRISBilling` resource grants are off
  CSD-125 and its `CIRISRegistry#51` blocker with them; the per-key issuance fee and
  bond are not mentioned except to say they went with the Portal.
- The CIRIS licence is `licensure:ciris-canonical:v1` (not `licensure:humanity-accord:v1`),
  a cosigned row under the root community's `quorum:2/3` of founders A1/B1/C1, or signed
  by a key holding a plain `license`-scoped `delegates_to` from `ciris-canonical` under that
  quorum (CC 2.4.1.2.1, 3.3.9, 4.4.3.4.3; the CC ruling on cirisconstitution-32,
  2026-10-28, citing existing text). There is no `infra:license` and there must be none;
  the `license` delegation is an ordinary cosigned row the reader walks, never a
  genesis-bundle member (CC 3.2 T5, 5.3.4). CSD-122 carries the ruling; CSD-124 carries the
  worked chain.

## The set

| CSD | Card | Stood in for (PortalService / Portal screen, now retired) | Serves | CSD/4 entry |
|---|---|---|---|---|
| CSD-120 | **Affiliation** (found, charter, dissolve) | CreateOrganization, UpdateOrganization, GetOrganization*, ListChildOrganizations | Justice, Contextual Integrity | T6 |
| CSD-121 | **Members and roles** | *OrgUser*, AddUserToOrg, RemoveUserFromOrg, UpdateUserOrgRole, ListOrgMembers | Autonomy, Justice | T6 (T18 for the lapse) |
| CSD-122 | **Licensure** (issue, suspend, reinstate, revoke, lapse, re-emit, contest) | RegisterPartner (licence half), RevokeEntity(license), ListExpiringLicenses | Fidelity, Justice | T6 |
| CSD-123 | **Partners** (the recognition half: role, forums) | RegisterPartner (recognition half), UpgradeToPartner, GetPartnerActivity | Fidelity | T6 |
| CSD-124 | **Authority delegations** (`license` / `grant` / `sub_delegation`; the `ciris-canonical` chain) | the org-admin half of every write above; extends CSD-055 | Core Identity, Integrity | T6 (T18 for the endings) |
| CSD-125 | **Grants** (access to a thing) | — (new) | Contextual Integrity | T18 |
| CSD-126 | **Affiliation keys** (register, rotate, revoke, escrow) | GenerateKeyPair (dropped), ActivateKey, RotateKey, RevokeKey, Register*Key, RequestKeyEscrow/Recovery | Integrity | T6 (T19 for revocation) |

Not in this set, because the client already has the card: audit (CSD-071), the device
delegation flow (CSD-055), community founding and governance (CSD-102/103; CSD-120 is
the affiliation-specific charter on top of it), key verification (CSD-104), the self
roster (CSD-095), users on a node (CSD-043). `COMMON_CARDS_ANALYSIS.md` recommends that
on adoption CSD-120 EXTENDs CSD-102, CSD-121 FOLDs into CSD-103 + CSD-055, CSD-124
EXTENDs CSD-055, CSD-126 FOLDs into CSD-037 + CSD-055, and CSD-122, CSD-123, CSD-125 are
KEPT as their own cards; until the client decides, all seven stay whole here.

## The three rules every card in the set follows

1. **The signer is named before the act.** Every write is a signed row by a person's or
   affiliation's key; the confirm sheet says who signs, what row it is, and who can read
   it, as CSD-102 does. An affiliation's act is a cosigned row under its
   `consensus_protocol` (CC 2.1); one member's signature is testimony. No card calls a
   route whose own auth is the authority.
2. **Only delegation chains.** A card never offers to delegate a licence or a grant; it
   offers to delegate the *right to issue* one (CSD-124), and the Licensure and Grants
   cards show which delegation an issuance was made under. Every edge has a term
   (`delegation_term_required`, CC#163 landed; persist v60 enforces it).
3. **Error is not empty.** A read the node could not serve says so; a licence that is
   not held is not "no licence".

## CSD/4 form, and what the checker says

CSD/4 is CSD/3 plus a `csd:tests` block naming ONE catalogue entry and the atoms each
test needs at one instant (`CIRISServer/FSD/CSD4.md` §1, LOCKED 2026-10-22). The
`csd:topology` layer graph of CSD-107 is the pre-catalogue form; a card carrying both is
refused by `packaging/check_csd4.py`. Each card's §3a therefore carries the typed
`csd:tests` block, and beside it the layer graph the lead asked for — as an untyped
`yaml` fence the checker ignores — with per-layer `checks:` and `# PENDING` notes for the
relations the vocabulary lacks (`authority`, `delegation`, a scores-row relation, `pre:`/
`post:` transitions, `cannot_act`), per `CSD4_EVALUATION.md` §4 and CSD4.md §5.

Run on 2026-10-28 with the client's `packaging/check_csd_v3.py` and `check_csd4.py`
(origin/main) against the rc12 registry and the client's pinned catalogue (v3):

| card | `check_csd_v3` | `check_csd4` (`csd:tests`) |
|---|---|---|
| CSD-120 | pin OK, every `ceg:` resolves, states OK; `surface:` cannot derive the nav map outside the client tree (expected until placed) | OK — matches the server's `requirements.yaml` entry for CSD-120 |
| CSD-121–126 | same as CSD-120 | FAIL by name: `requirements.yaml` has no entry for the card. The entries are in each card's §3a; adding them to `harness/native/topologies/catalog/requirements.yaml` is the ask on CIRISServer. Notes report where the planner would reselect T18/T19 |

The verbatim output is in the catch-up commit message. The `surface:` failure is the
same for every card: `nav_map` lives in the client tree and the surfaces are `proposed:`.

## Stage and ownership

All seven stay `envisioned`. CSD/3 requires `sketched` to have real screens/tags and a
parsing flow with a client pen; nothing here has a Kotlin screen, every tag is
`proposed:`, and no flow is written. `building` would additionally need every §3 row
answered: the persist half is now answered (v60.0.0, CIRISPersist#1036 and #1031–#1035,
#1048, #1062 shipped), but every Server door is `blocked_by: CIRISServer#752` (open) and
the chain read by CIRISServer#663, and the catalogue's `authority` and `delegation`
relations are refused by the builder until those doors exist. §1 is falsifiable and every
`ceg:` resolves in rc12, which is all `envisioned` asks.

## Adopter status (2026-10-28)

| ticket | what | state |
|---|---|---|
| CIRISPersist#1036 | authority-set arm, `as_of` lens, audience, the kinds' move | **shipped** in persist v60.0.0 (2026-10-26; CHANGELOG §8 items 1–5; migrations V202–V205). Cited as evidence, no longer a blocker |
| CIRISPersist#1031–#1035, #1048, #1062 | cycle refusal, term lens, `grant` arm, typed config, licensure gate/fold + public read, term gate, org on the bond | **shipped** (persist CHANGELOG) |
| CIRISServer#752 | the doors (found with charter, roles, licences, grants, delegations, keys) | **open** per the lead's check; `CLM-delegation-term` is staged on it in CC `claims.tsv` |
| CIRISEdge#852 | carriage of the authority head | split into **CIRISEdge#895** (licence audience carriage; `CLM-licence-audience` staged there) and **CIRISEdge#896** (delegation term; `CLM-delegation-term` staged there) |

`gh api` answered 404 for every CIRISAI issue during the catch-up session (a known
intermittent auth failure on this machine); the states above come from the lead's
check, persist's CHANGELOG and CC's `claims.tsv`, and should be re-read once.

## Unsettled

- CC 3.3.9 (rc12) still says the three operational kinds "remain registered and in force
  until an amendment retires them", while persist v60.0.0 V205 has dropped their tables
  and refuses the replication kinds. A CC text amendment closing 3.3.9 onto its first
  paragraph is owed; the cards cite both.
- CIRISEdge#851 (depth 4 → 5) and CIRISEdge#808 (per-recipient `key_grant` carriage)
  were not re-checked.
- The server's `requirements.yaml` entries for CSD-121–126 (above).

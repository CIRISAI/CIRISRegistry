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
(CIRISServer v0.5.233, 7bcbee42); the server's main is at version 5. T6 and T18, the
entries these cards name, differ between the pinned files and main only in their own
`catalogue_version` field (diffed 2026-10-28); versions 4 and 5 added the durability and
agent-plane atoms, none of which these cards use.

**CC rc13.** CC branch `rc13` (b70b614, 2026-10-28) lands "Who I trust (normative)" at
CC 4.4.3.8 and the deltas note `review/notes/affiliation_vs_community_deltas_2026-10-28.md`.
Its registry grammar hash is the same `d587156605dd…`, so the pin above holds; the cards
cite `1.0-rc13` where they cite Who I trust and rc12 elsewhere.

## The Portal is being decommissioned (steward, 2026-10-28)

CIRIS is no longer selling licences; portal.ciris.ai is being decommissioned. The CIRIS
L3C licensing authority is the humanity accord trust root acting through the root
community it founded, `ciris-canonical`; the model is fully decentralised — anyone can be
their own licensor, it only matters who trusts them; a reader chooses how it trusts, and
the shipped default root is trusted by default — through the reader's own **Who I
trust** set (CC 4.4.3.8 rc13), in which the root is one default, removable entry and
never the only trust knob. So, in this set:

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
- **The reader half is the reader's own.** A licence is honoured iff the reader's
  class-filtered, depth-bounded walk over its own `trust:accepts:v1` edges reaches the
  issuing authority and that authority's charter declares the class (CC 4.4.3.8 rc13;
  `formal/trust_set/TrustSet.lean`). Licensors-of-licensors is that walk over the
  authorities' own acceptance edges, not a `license` delegation chain. Removing the root
  removes exactly what reached through it and un-federates the node until another root
  is attached (items 4–5): licensor trust is not root trust. Wire members
  `trust_classes` / `trust_depth`: CIRISPersist#1142 (open). The surface for the set is
  the Who-I-trust extension of CSD-105 (`COMMON_CARDS_ANALYSIS.md`).

## The set is the affiliations tier of the circles model

Steward direction (2026-10-28): "this should fill out the affiliations tier entirely
in the circles model." The circle is **Communities and Businesses**
(`layer-global-communities`, whose hub already reads `tier: affiliations`; CSD-050 §1),
seven tabs (`CirclesNav.kt`). `COMMON_CARDS_ANALYSIS.md` places every slot and
recommends the tier stay that circle, made explicit as `cohort_scope: affiliations`,
rather than a sixth circle (a circle IS a `cohort_scope`, and "global communities" is
not one of CC's seven; scope order `pending: CC ruling 11`).

| tab › row | card | stood in for (PortalService, now retired) | CSD/4 entry | verdict on adoption |
|---|---|---|---|---|
| Rules › hub (record + **charter**) | **CSD-120 Affiliation** | CreateOrganization, UpdateOrganization, GetOrganization*, ListChildOrganizations | T6 | EXTEND CSD-102 §2.3 |
| People › roster; Rules › Delegations (roles) | **CSD-121 Members and roles** | *OrgUser*, AddUserToOrg, RemoveUserFromOrg, UpdateUserOrgRole, ListOrgMembers | T6 (T18 lapse) | FOLD INTO CSD-103 + CSD-055 §7 |
| Rules › **Licensure** | **CSD-122 Licensure** (issue, status set, duties, re-emission, contest; the reader's Who-I-trust verdict) | RegisterPartner (licence half), RevokeEntity(license), ListExpiringLicenses | T6 | KEEP |
| People › **Partners** | **CSD-123 Partners** (recognition, role, forums) | RegisterPartner (recognition half), UpgradeToPartner, GetPartnerActivity | T6 | KEEP |
| Rules › Delegations (the chain) | **CSD-124 Authority delegations** (`license` / `grant` / `sub_delegation`; the `ciris-canonical` chain) | the org-admin half of every write above | T6 (T18 endings) | EXTEND CSD-055 §7 |
| Rules › **Grants**; Files door | **CSD-125 Grants** | — (new) | T18 | KEEP |
| My things › My Identity; Rules › Delegations (key filter) | **CSD-126 Affiliation keys** | GenerateKeyPair (dropped), ActivateKey, RotateKey, RevokeKey, Register*Key, RequestKeyEscrow/Recovery | T6 (T19 revocation) | FOLD INTO CSD-037 + CSD-055 §7 |
| Rules › Trust (**Who I trust**) | reader side; not one of the seven | — | — | EXTEND CSD-105 §2.2; the Rules › Trust row on this circle is its door |
| Safety › lawful access, transparency obligations | — | — | — | GAP → CSD-130 |
| Decisions › pending cosignatures | — | — | — | GAP → CSD-131 |
| Record › records duties (retention, legal hold, erasure, disclosure accounting) | — | — | — | GAP → CSD-132 |
| People / Rules › public face of an affiliation | — | — | — | GAP → CSD-133, held on CC ruling 8 |
| Files, Chats, Contacts, Moderation, ChildSafety, Consent, Health & Reputation, Audit | CSD-007, 110, 005, 065, 066, 053/054, 044, 071 | — | — | existing, unchanged |
| Rules › Billing; People › Users | CSD-056, CSD-043 | — | — | leave the tier |

**Why the tier is distinct** (CC rc13 deltas note, the why column): the data-handling
and retention duties are affiliation-only in the text (C1, TEXT SAYS: classification,
retention floors and ceilings, `legal_hold`, `erasure_policy`, `designated_officials`,
`archive_custody`); the interest/necessity split is the stated motive (A1); wider
moderation latitude is SILENT, and where the text speaks it tightens affiliations
(B1: every role edge termed, the organisation on the bond). The tier shares the
community's crypto tier and feed band (8a).

Not in this set, because the client already has the card: audit (CSD-071), the device
delegation flow (CSD-055), community founding and governance (CSD-102/103; CSD-120 is
the affiliation-specific charter on top of it), key verification (CSD-104), the self
roster (CSD-095), users on a node (CSD-043), the trust root (CSD-105, which Who I trust
extends). Until the client decides, all seven stay whole here.

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
`csd:topology` layer graph of CSD-107 is the retired pre-catalogue form; a card carrying
both is refused by `packaging/check_csd4.py`, and this set carries only the locked form.
Each card's §3a carries the typed `csd:tests` block and a short prose note naming the
atoms and relations the vocabulary lacks; those are collected below as the asks.

Run on 2026-10-28 with the client's `packaging/check_csd_v3.py` and `check_csd4.py`
(origin/main) against the rc12 registry and the client's pinned catalogue (v3):

| card | `check_csd_v3` | `check_csd4` (`csd:tests`) |
|---|---|---|
| CSD-120 | pin OK, every `ceg:` resolves, states OK; `surface:` cannot derive the nav map outside the client tree (expected until placed) | OK — matches the server's `requirements.yaml` entry for CSD-120 |
| CSD-121–126 | same as CSD-120 | FAIL by name: `requirements.yaml` has no entry for the card. The entries are in each card's §3a; adding them to `harness/native/topologies/catalog/requirements.yaml` is the ask on CIRISServer. Notes report where the planner would reselect T18/T19 |

The verbatim output is in the catch-up commit messages. The `surface:` failure is the
same for every card: `nav_map` lives in the client tree and the surfaces are `proposed:`.

## Asks of CIRISServer (one issue)

1. **`requirements.yaml` rows for CSD-121–126** — the `csd:tests` block of each card,
   verbatim (§3a): CSD-121 on T6 (`roster`, `role_is_a_chain`, `term_lapsed`); CSD-122
   on T6 (`issued_by_the_authority`, `issued_by_a_delegate`); CSD-123 on T6
   (`recognised`, `licence_beside_it`); CSD-124 on T6 (`chain`, `withdrawn`, `expired`);
   CSD-125 on T18 (`held`, `issued_by_a_delegate`); CSD-126 on T6 (`bound`,
   `revoked_after`, `revoked_all`). Until they exist `check_csd4` fails each card by name.
2. **Lift the T6 refusals** once the doors exist (CIRISServer#752): `authority` (a
   licensing-authority affiliation: no route) and `delegation` (a chain needs the
   delegate's own act; today `POST /v1/auth/device/delegate` keeps one scope and a 600 s
   term, CIRISServer#751).
3. **Atoms the cards need and the vocabulary lacks** (a new atom is a CIRISConstitution
   PR to `manifests/topology_vocabulary.json` first, then the server moves the pin,
   CSD4.md §4; a catalogue version bump): `affiliation.charter.declared`,
   `affiliation.charter.superseded`; `licence.issued`, `licence.status.suspended`,
   `licence.status.revoked`, `licence.honoured`, `licence.not_trusted`;
   `recognition.outbound`, `recognition.inbound`; `delegation.scope.license`,
   `delegation.scope.grant`, `delegation.refused`; `grant.axis.content`,
   `grant.onward_refused`, `grant.ended_by_rotation`; `key.successor_committed`,
   `key.rotated`, `key.escrowed`; `trust.entry.licensor`, `trust.entry.root_removed`.
4. **Relations / negatives the builder lacks**: a scores-row relation for a licence and
   a recognition; a second `community` at the affiliations tier on T6; `key_grant` with
   an axis and a window; a `trust_accepts` relation with `trust_classes` / `trust_depth`
   for the reader; `cannot_act(person, op, reason)` as a negative (CSD4_EVALUATION §4.2);
   a cosigned charter supersession (`quorum_change` + `cosign`) at the affiliations tier.
5. **The Who-I-trust door**: a route that adds and removes a reader's `trust:accepts:v1`
   entry targeting any authority (not only a root bundle), carrying `trust_classes` and
   `trust_depth` (CIRISPersist#1142), with the CC 4.4.3.8 items 4–5 consequence reported
   on removing the last root. `POST /v1/trust-root/import` / `DELETE /v1/trust-root/{id}`
   cover roots only.

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
- The server's `requirements.yaml` entries for CSD-121–126 (the asks above).
- The eleven rulings of the CC rc13 deltas note that the cards mark `pending: CC ruling
  <n> (affiliation_vs_community_deltas_2026-10-28)`: 1 discriminator on the record, 2
  charter defaults, 3 admin scope set, 4 protocol floor and the one-person affiliation,
  5 succession, 6 moderator duty for institutions, 7 a social community as licensor, 8
  the public face, 10 lifecycle, 11 scope order. Ruling 9 (the reader's edge) is settled:
  Who I trust, with CIRISPersist#1142 for the wire shape.

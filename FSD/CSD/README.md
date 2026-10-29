# CSDs for the former Portal surface, on CEG — the registry's hand-off to the fold

**Status:** DRAFT. Stage `envisioned` (steward pen) under CSD/3 (`CIRISClient/CSD.md`),
with the CSD/4 `csd:tests` block (`CIRISServer/FSD/CSD4.md`, LOCKED catalogue). These
documents are written here, in the repo that is folding, so the Portal's surface is
specified before the Portal is gone. On adoption they move to `CIRISClient/FSD/CSD/` and
keep their numbers (120–126 are reserved there on CIRISClient#206; `numbers.tsv`).

**Reads with:** `FSD/FSD-005_ORG_AS_AFFILIATION.md` (the translation these cards
render; its §9 lists the CC sections), `formal/authority_tree/` (the state tree the
Delegations and Grants cards must honour), `FSD/FSD-004_POST_FOLD_SURFACE.md` §4.3 (the
PortalService methods each card stood in for), `COMMON_CARDS_ANALYSIS.md` (which client
cards already carry parts of this set, and what to fold where).

**Registry pin:** `registry_sha256`
`df79a1dde30a380380da83f0d776737e580705b6305741f037a5a78c2bef9898` (189 families; the
grammar hash, `_meta.registry_sha256`), CC 1.0-rc14. Every
`ceg:` family the seven cards name resolves in it. The cards cite CC 1.0-rc14 text.

**Catalogue pin:** CSD/4 catalogue version 3, the client's `packaging/csd4/PIN.md`
(CIRISServer v0.5.233); the server's main is at version 5. T6 and T18, the entries these
cards name, differ between the pinned files and main only in their own
`catalogue_version` field; versions 4 and 5 added the durability and agent-plane atoms,
none of which these cards use.

## The Portal is decommissioned

CIRIS sells no licences; portal.ciris.ai is decommissioned. The CIRIS L3C licensing
authority is the humanity accord trust root acting through the root community it
founded, `ciris-canonical`; the model is fully decentralised — anyone can be their own
licensor, it only matters who trusts them; a reader chooses how it trusts, and the
shipped default root is trusted by default — through the reader's own **Who I trust**
set (CC 4.4.3.8 "Who I trust"), in which the root is one default, removable entry and
never the only trust knob. So, in this set:

- Where a card replaced a Portal screen, **the screen is retired, not migrated**. The
  "stood in for" column below is historical: it records which PortalService method a
  card's act corresponds to, so FSD-004 §4.3 stays traceable.
- Every billing, Stripe, fee, bond, tier-price and "contact sales" dependency is gone
  from the cards. `partner_role:{role}` names a role, never a price tier (CSD-123); the
  Portal's six `license_type` tiers are retired; `CIRISBilling` resource grants are off
  CSD-125 and its `CIRISRegistry#51` blocker with them; the per-key issuance fee and
  bond are not mentioned except to say they went with the Portal.
- The CIRIS licence is `licensure:ciris-canonical:v1` (not `licensure:humanity-accord:v1`),
  a cosigned row under the root community's `quorum:2/3` of founders A1/B1/C1, or signed
  by a key holding a plain `license`-scoped `delegates_to` from `ciris-canonical` under that
  quorum (CC 2.4.1.2.1, 3.3.9, 4.4.3.4.3). There is no `infra:license` and there must be
  none; the `license` delegation is an ordinary cosigned row the reader walks, never a
  genesis-bundle member (CC 3.2 T5, 5.3.4). CSD-122 carries the rule; CSD-124 carries the
  worked chain.
- **The reader half is the reader's own.** A licence is honoured iff the reader's
  class-filtered, depth-bounded walk over its own `trust:accepts:v1` edges reaches the
  issuing authority and that authority's charter declares the class (CC 4.4.3.8 "Who I
  trust"; `formal/trust_set/TrustSet.lean`). Licensors-of-licensors is that walk over the
  authorities' own acceptance edges, not a `license` delegation chain. Removing the root
  removes exactly what reached through it and un-federates the node until another root
  is attached (CC 4.4.3.8 items 4–5): licensor trust is not root trust. Wire members
  `trust_classes` / `trust_depth`: CIRISPersist#1142. The surface for the set is the
  Who-I-trust extension of CSD-105 (`COMMON_CARDS_ANALYSIS.md`).

## The set is the affiliations tier of the circles model

This set fills out the affiliations tier of the circles model entirely. The circle is
**Communities and Businesses** (`layer-global-communities`, whose hub already reads
`tier: affiliations`; CSD-050 §1), seven tabs (`CirclesNav.kt`).
`COMMON_CARDS_ANALYSIS.md` places every slot and recommends the tier stay that circle,
made explicit as `cohort_scope: affiliations`, rather than a sixth circle (a circle IS a
`cohort_scope`, and "global communities" is not one of CC's seven; `affiliations` sits
beside `community`, neither wider nor narrower, the two circles siblings in the order of
widening, CC 4.4.3.2.8 "The scopes").

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
| People / Rules › public face of an affiliation | CSD-120 (`declared_cohort`, `public_face`, `listing_row`) | — | T6 | part of CSD-120 (CC 4.4.3.2.8 "Public face": the four things are members of the record the card renders) |
| Files, Chats, Contacts, Moderation, ChildSafety, Consent, Health & Reputation, Audit | CSD-007, 110, 005, 065, 066, 053/054, 044, 071 | — | — | existing, unchanged |
| Rules › Billing; People › Users | CSD-056, CSD-043 | — | — | leave the tier |

**Why the tier is distinct** — CC 4.4.3.2.8 "The assumption each tier carries": "A
community is presumed pro-social: its members gather as peers, so the floors of CC 4.5.13
and CC 4.5.4 (no single vote admits or removes, a named moderator, removal on a harm
report by more than one voice) protect them from one another and from one voice. An
affiliation carries no such presumption, and the law does not grant it one: it holds
other people's data because its work requires it, so the limbs of this record
(classification, retention, legal hold, erasure, disclosure, custody, designated
officials) bind an affiliation and bind no community." Hence the split ("One member is
enough", "The floors bind power over members, not the count of voices"): the data regime
attaches to the record at any size, so a sole practitioner is an affiliation; the floors
bind power over members, not the count of voices (appointment is two consents; a
one-seat body acts against a member only with an outside countersign, else it is a
CC 4.5.5 proposal; the designated official discharges the named-moderator duty; two or
more seats decide as a community). The tier shares the community's crypto tier and feed
band (CC 4.4.3.2.1; CC 4.4.3.3), and sits beside `community` in the order of widening,
neither wider nor narrower ("The scopes"). The line between the tiers is the licence
class, not the label: anyone may license, a community included, and a class with legal
weight brings the affiliation's duties with it whatever the record's `declared_cohort`
says ("Who may license").

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
   (`delegation_term_required`, CC 4.4.3.2.8 C; persist v60 enforces it).
3. **Error is not empty.** A read the node could not serve says so; a licence that is
   not held is not "no licence".

## CSD/4 form, and what the checker says

CSD/4 is CSD/3 plus a `csd:tests` block naming ONE catalogue entry and the atoms each
test needs at one instant (`CIRISServer/FSD/CSD4.md` §1, LOCKED). The `csd:topology`
layer graph of CSD-107 is the retired pre-catalogue form; a card carrying both is
refused by `packaging/check_csd4.py`, and this set carries only the locked form. Each
card's §3a carries the typed `csd:tests` block and a short prose note naming the atoms
and relations the vocabulary lacks; those are collected below as the asks.

Run with the client's `packaging/check_csd_v3.py` and `check_csd4.py` (origin/main)
against the pinned registry and the client's pinned catalogue (v3):

| card | `check_csd_v3` | `check_csd4` (`csd:tests`) |
|---|---|---|
| CSD-120 | pin OK, every `ceg:` resolves, states OK; `surface:` cannot derive the nav map outside the client tree (expected until placed) | OK — matches the server's `requirements.yaml` entry for CSD-120 |
| CSD-121–126 | same as CSD-120 | FAIL by name: `requirements.yaml` has no entry for the card. The entries are in each card's §3a; adding them to `harness/native/topologies/catalog/requirements.yaml` is the ask on CIRISServer. Notes report where the planner would reselect T18/T19 |

The `surface:` failure is the same for every card: `nav_map` lives in the client tree
and the surfaces are `proposed:`.

## Asks of CIRISServer

Tracked as CIRISServer#831.

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
answered: the persist half is answered (persist v60), but every Server door is
`blocked_by: CIRISServer#752` and the chain read by CIRISServer#663, and the catalogue's
`authority` and `delegation` relations are refused by the builder until those doors
exist. §1 is falsifiable and every `ceg:` resolves in the pinned registry, which is all
`envisioned` asks.

## What exists

- **Persist v60** carries the authority-set arm, the `as_of` lens, licence audiences
  and re-emission, the typed `partner_record` payload, V205 retiring the three
  operational kinds, cycle refusal, the term lens, the `grant` arm, the typed
  affiliation config, the licensure gate/fold agreement and public read, the term gate,
  and the organisation on the bond. In the cards these are `confirmed — v60` evidence
  pointers, never a `blocked_by`.
- **Open on persist**: `trust_classes` / `trust_depth` on `trust:accepts:v1`
  (CIRISPersist#1142); the one-voice countersign, the public face and dissolution
  (CIRISPersist#1143, where `CLM-affiliation-one-voice-countersign`,
  `CLM-affiliation-public-face` and `CLM-affiliation-dissolution` are staged).
- **Open on Server**: the doors (CIRISServer#752, where `CLM-delegation-term` is
  staged); the chain read (CIRISServer#663).
- **Open on Edge**: licence audience carriage (CIRISEdge#895, `CLM-licence-audience`);
  delegation term carriage (CIRISEdge#896); default depth 4 → 5 (CIRISEdge#851);
  per-recipient `key_grant` carriage (CIRISEdge#808).

## Unsettled

- The server's `requirements.yaml` entries for CSD-121–126 (the asks above).
- The substrate tickets under "What exists": CIRISPersist#1142, #1143, CIRISServer#752,
  #663, CIRISEdge#895, #896, #851, #808.

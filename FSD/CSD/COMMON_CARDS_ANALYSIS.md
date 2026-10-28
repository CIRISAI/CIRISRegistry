# Common-cards analysis: which of CSD-120..126 the client already carries

**Status:** DRAFT, 2026-10-28. Read against CIRISClient `origin/main` (the cards
under `FSD/CSD/`, `CSD.md` CSD/3, `packaging/check_csd_v3.py`) and our seven drafts
in this directory after the 2026-10-28 catch-up (CSD/4, rc12, the steward's
decommission of portal.ciris.ai, the licensure ruling on cirisconstitution-32).

**The rule being applied** is the client's own: CSD.md §4.1, "two cards newly
calling the same mutating route fail the gate: that is the same card under two
names, or one action with two doors", and the precedent of CSD-001 folded into
CSD-055 §6, CSD-034 into CSD-046, CSD-106 written as "no new card" over four hubs.
A card of ours survives only if no client card can carry its rows without
becoming a second door on a route the client already calls.

**Three recommendations are used, and only three.**

* **FOLD INTO** a card: our card is deleted; the rows named move into the
  sections named; the rows named as duplicates are dropped.
* **EXTEND** a card: our card is deleted; the client card gains one new,
  named section carrying our content as a block.
* **KEEP**: our card is adopted as its own file under its reserved number
  (`numbers.tsv`, CIRISClient#206).

A blocker already on a client card is cited, never re-declared: `blocked_by`
on a field that a sibling row already carries is the drift CSD.md §1 warns of.

---

## CSD-120 Affiliation (found, charter, dissolve)

### What the client already covers

**CSD-102** is the card, almost entirely.

* **§1** names CC 4.4.3.2.8 in the same words our §1 does ("an *affiliation* is
  the same machinery gathered by necessity, with one declared config record
  visible to members on joining") and records that the record "has no route
  (CIRISServer#649), and the card says so instead of implying there is nothing
  to declare". Its "Placement, and why there is no new Rules card" paragraph
  mounts the governance section on the circle hub, `tier: affiliations` being
  Communities and Businesses, which is exactly where our §2 places the charter.
* **§2 `csd:surface`**: `layer-local-community` with `scopes:
  [layer-local-community, layer-global-communities]`. Our
  `proposed:affiliation-charter` is a second surface on the same hub.
* **§2 `shows`**, rows that are ours by another name:
  * `x_private:cohort_scope` (`enum[community,affiliations]`, emit: "the circle
    you found it from IS the tier") covers our founding "with `cohort_scope:
    affiliations`".
  * `x_private:consensus_protocol` (emit, `"quorum:2/3"`, "four choices at
    founding") is our `x_private:consensus_protocol` row, same example, same
    mutability.
  * `x_private:affiliations_declared_record` (`type: unconfirmed`,
    `blocked_by: CIRISServer#649`, tag `txt_affiliations_terms_unavailable`,
    example `"affiliation_archetype: informal_adhoc"`) is the placeholder for
    our whole `shows` block; its example is our first field.
* **§3**: `POST /v1/communities {name, members?, tier?, consensus_protocol?}`
  (found), `DELETE /v1/communities/{id}` (dissolve), the held-change trio
  `…/changes/{envelope,cosign,assemble}` and 409 `community.quorum_pending`.
  Our §3 cites the same `POST /v1/communities` and "CSD-102's dissolve"; the
  duplicate-route gate would fail the pair.
* **§3.1**: "The affiliations record (CC 4.4.3.2.8) has no route,
  CIRISServer#649." **§5 Review**: stage held at `building` for that one row.

**CSD-106** covers founding alone (`community_create_found_alone`) and the
invitee's inbox on the hub, so our flow's "a second person joins and the
acceptance row names the charter version" is CSD-106 §2's accept confirm plus
one more fact.

### What is genuinely new in ours

The charter itself, as twelve typed fields that CSD-102 only names as
"unavailable": `affiliation_archetype`, `membership_basis`,
`classification_scheme`, `retention_policy`, `disclosure_posture`, `role_term`,
`licence_audiences`, `parent_affiliation`, `dissolves_at`, `charter_version`,
the `membership:{stage}` acceptance count against a charter version, and the
display of `community_key_id` **as the licensure authority id** ("Licences you
issue are signed as `licensure:{id}`"). Also the two rules: the charter is one
record superseded as a whole, never field by field; and a founder who picks an
archetype and nothing else has a complete charter (mandatory-with-default).

### Recommendation: EXTEND CSD-102

Add to CSD-102 a section **§2.3 "The affiliation charter (the declared config
record)"**, affiliations tier only, carrying our `shows` rows as the expansion
of `x_private:affiliations_declared_record`. Concretely:

* Our `x_private:consensus_protocol` row is **dropped** (CSD-102 has it; our
  `renders` text about "the same rule signs a licence, a role, a charter
  change" moves into CSD-102's `renders` for that row).
* Our §3 rows "found" and "dissolve" are **dropped** (CSD-102 §3 has both;
  note on the found row that the wire field is `tier`, not `cohort_scope`).
* Our §3 rows "write / supersede the charter" and "read the charter" become
  CSD-102 §3 rows under the existing `blocked_by: CIRISServer#649`; no second
  blocker. "Nest under a parent" (charter member `hierarchy`, CIRISPersist) is
  a new row.
* The remaining eleven fields, the supersession rule, the archetype default,
  and our four `states` (the `empty` sentence "has declared nothing beyond the
  default, and that is a complete charter" is the one CSD-102 lacks) move in.
* Our `csd:topology` block is CSD-102's own (CSD4_EVALUATION §4.1 drafts the
  community block for CSD-102, not for us); the charter adds no layer.

Why not KEEP: CSD-102 §1 already argues "why there is no new Rules card" for
the community half, and that argument holds for the charter half, which is one
more section of the same record. Why EXTEND rather than FOLD: the charter is a
coherent block with its own empty state and its own supersession rule, and
dispersing twelve fields through CSD-102's existing rows would hide them.

---

## CSD-121 Members and roles

### What the client already covers

**CSD-103** (the roster) and **CSD-106** (invitations) between them carry the
whole membership half:

* CSD-103 §2: `x_private:community_member_key_id` (one row per member),
  `x_private:community_member_role` (the role chip), `community_member_joined_at`
  (read against `founded_at` for the late flag), `community_add_member`,
  `community_remove_member` (behind the three-fact ConfirmSheet: "removed, key
  rotated, nothing new reaches them, what they had stays"),
  `x_private:membership_invitation` (Invite someone),
  `x_private:community_group_book` (`unconfirmed`, `blocked_by: CIRISServer#650`:
  who admitted whom, when).
* CSD-103 §3: `GET /v1/communities` (the fold), `POST /v1/communities/{id}/members`,
  `POST/GET/DELETE /v1/communities/{community_id}/invites[...]`,
  `DELETE /v1/communities/{id}/members/{key_id}`.
* CSD-106 §2: `membership_invitation`, `membership_acceptance` (Accept, "signed
  with the invitee's own key", notice "Accepted. You're not in it until the
  group seats you"), `membership_decline`, `membership_invitation_pending`,
  `membership_invitation_expires_at`, `membership_invite_withdraw`. §3 cites
  the rows themselves: `membership:proposal:v1`, `membership:acceptance:v1`,
  `membership:decline:v1` (CIRISPersist#955, v52), so our "Member since: the
  instant of THEIR acceptance row" is CSD-106's acceptance row read back on
  CSD-103's `joined_at`.

Our §3 row "invite / accept / remove a member" already says "CSD-106 / CSD-103
routes"; it cites nothing of its own.

**CSD-102** carries the roster-record role: `x_private:community_member_role`
(emit, example `"treasurer"`, "a 'Change role' chip; the role body says a role
is not a duty") over `POST /v1/communities/{id}/members/{key_id}/role {role}`.
That is the role **word** on the roster record, the one persist's `tally`
reads (`founder_only` admits a founder). Our card's role is a different object:
a `delegates_to` edge with scopes. Both are real and the client card should say
they are two things; today CSD-102 says only "a role is not a duty".

**CSD-055** carries every one of our role-sheet rows for a person's own
delegations: `delegated_scope` (`list[string]`, TIGHTEN-ONLY on approve),
`delegate_key_id`, `delegation_attestation_id` ("the signed record it is"),
`delegation_depth` (`unconfirmed`, `blocked_by: CIRISServer#663`, "Who granted
this before you, the chain above this grant, capped at 5"), and
`inbound_delegations` (`unconfirmed`, `blocked_by: CIRISServer#663`). Our
`delegation_depth` row names the same blocker in prose; the fold must not add a
second `blocked_by: CIRISServer#663`.

**CSD-043** does not overlap, and says so itself: §1 "That sentence is the
mission of the screen that exists. It is not the mission of the card the Locked
Spec placed in Communities › People, which would be *the people in the
affiliations I belong to*"; §3 "an affiliation's roster: missing everywhere";
§6 recommends moving Users to My things › This node. Our "Reads with" line is
right to list it as "not this". Nothing of ours moves there.

### What is genuinely new in ours

* The four presets (`OrgAdmin`, `KeyManager`, `Operator`, `Viewer`) as bundles
  of delegation scopes, with "the card offers the presets; the wire carries the
  scopes; the receipt shows the scopes".
* `conferred_by` ("the founders" when the edge is from the quorum).
* `delegation_valid_until` as **required**, from the charter's `role_term`,
  ceiling one year, refused at the door otherwise (`delegation_term_required`);
  and the CC 4.5.4 consequence that a lapsed `moderate` role auto-promotes or
  fails secure in the same step.
* `role_revoke_cascade`: the subtree a withdrawal severs, named before the act.
* The rule that removing a membership cascades every role (the roster no longer
  names the key the edges point at).

### Recommendation: FOLD INTO CSD-103 and CSD-055

The membership half has no content the client lacks; the role half is the
delegation card's. Split:

* **Into CSD-103 §2**: on `community_member_joined_at`, the note that the
  instant is the member's own `membership:acceptance` row (CSD-106 §3), never
  the inviter's proposal; on `community_remove_member`'s confirm, a fourth
  fact for the affiliations tier, "ends every role this member holds". The
  role chip on the row opens the delegation sheet (next bullet). Our `empty`
  sentence ("every member is a member only; the founders hold the root") is
  the affiliations-tier wording of CSD-103's roster empty state. Our
  `membership:{stage}` rows are **dropped** (CSD-106 emits them; CSD-103
  reads them). Our invite / accept / remove §3 row is **dropped**.
* **Into CSD-055**, alongside the CSD-124 extension below (one section, not
  two): `role_preset` (as a preset picker over the scope set, affiliations
  only), `conferred_by`, `delegation_valid_until` (required, with the term
  rule and the `delegation_term_required` token), `role_revoke_cascade` (the
  same sheet as CSD-124's `revoke_cascade`; one field). Our
  `delegated_scope` and `delegation_depth` are **dropped**: CSD-055 has both,
  and keeps its `blocked_by: CIRISServer#663` on depth.
* **Into CSD-102 §2**: one sentence on `community_member_member_role`
  distinguishing the roster record's role word (what `tally` reads) from a
  scoped delegation (what a member may do), with a pointer to CSD-055.

Why not KEEP: our §3 has no route of its own; every row is "CSD-106 / CSD-103
routes" or "none, FSD-005 §7". A card with no door of its own is a view, and
the client's rule files views under the card whose routes they read.

---

## CSD-122 Licensure

### What the client already covers

Nothing renders a `licensure:*` row. The family is named three times, each
time to say the card in hand is not it:

* **CSD-052 §1**: CC "keeps three trust vocabularies apart on purpose:
  attestation/provenance for a binary's integrity, `trust:{job}:{version}` for
  which root a node obeys, and `licensure:*` / `capacity:*` for a person's or
  agent's standing. This card renders the first and is named for the second, in
  a tab where a person is looking for the third." §2 adds that
  `attestation:license_validity` "is rendered nowhere" and that CIRISVerify
  emits it (`ciris-verify-core/src/federation_provenance.rs:107`). That is the
  one row our card binds for the CIRIS-issued licence
  (`attestation:license_validity`, "Verify has not co-signed this licence").
* **CSD-104 §1**: the SAS ceremony "confers no `trust:{job}:{version}`, no
  `licensure:*` and no `capacity:*`."
* **CSD-090 §3**: `ACCORD_CANNOT_CONFER = consent_revocation, license, grant`,
  "the CC 4.2.1 wall: accord authority cannot reach the consent or licensure
  planes". The `license` scope is in the node's served vocabulary
  (`GET /v1/vocabulary → delegation_scope.moderation`, seven at persist v48,
  including `license` and `grant`), and the one card that reads that vocabulary
  refuses to offer it, correctly: the accord is not a licensing authority.

Partnership "licences" on **CSD-054 §7** (`consent:partnership_grant`,
`GET /v1/partnership/*`) are a person partnering with an agent on the agent's
consent plane, not a `licensure:*` row; the word collides, the object does not.

### What is genuinely new in ours

All of it: the status **set** (`enum[issued,probation,restricted,suspended,
revoked,lapsed,surrendered,reduced]`, several live at once, `revoked`
terminal), the authority as an affiliation (`licensure:{community_key_id}:v1`)
or a key, the `license`-scoped delegate as the only other issuer, the
`duty:{kind}` attachments, `lapsed` versus `expired`, reinstate as a
`withdraws` on the suspended row, per-class visibility from the charter, the
re-emission to named readers via `subject_key_ids`, the subject's own view and
`reconsideration:{grounds}` contest, and, after the 2026-10-28 ruling, the CIRIS
licence as `licensure:ciris-canonical:v1` cosigned by the root community's
quorum with no sale behind it.

### Recommendation: KEEP

No client card renders, emits or reads a `licensure:*` row; two say in terms
they must not. The nearest surfaces are CSD-052 (a binary's attestation ladder,
`attestation:license_validity` unrendered) and CSD-090 (a duty conferral the
accord may not extend to `license`). Neither can carry an issuing side without
becoming a card about a different plane. One link to add: CSD-052's
`attestation:license_validity` row, when it is drawn, is the co-signature chip
our card shows on the CIRIS-issued licence only; the two cards should cite each
other rather than each bind the family alone.

---

## CSD-123 Partners

### What the client already covers

* **CSD-043 §2** binds `partner_role:{role}` with `bind: {role: community}` as
  the API-role chip (`OBSERVER / ADMIN / AUTHORITY / SYSTEM_ADMIN /
  SERVICE_ACCOUNT`), and says in the paragraph under the block that the bind is
  "deliberately not the values on screen": "the nearest registered family",
  named "to force the mismatch into view". It is the only client use of the
  family, and it is a placeholder that CSD-043 §6 itself wants retired ("a
  CC-rejected concept rendered as a chip").
* **CSD-104 §2** lists "partners" among the federation directory's key kinds on
  the peer list (`GET /v1/federation/peers`: "nodes, stewards, accord holders,
  wise authorities, partners, witnesses and agents"). That is a directory row's
  kind, read-only, with no recognising affiliation named.
* No client card names `multilateral_participation:*` or
  `revocation:partner:*`. `revocation:{entity_type}:{reason}` is bound on
  CSD-065, CSD-087 (`entity_type: agent`) and CSD-108, none for a partner.

### What is genuinely new in ours

Recognition as a `scores` row by one affiliation about another's key, at a
named `partner_role:{role}`, with forum participation
(`multilateral_participation:{forum}:{kind}`), the inbound / outbound / mutual
display fold (CC 2.3.2.4), and the separation of recognition from licence
(CSD-122) and from agency (CSD-124). After the catch-up `partner_role:{role}`
is open vocabulary naming a role, with no tier, price or billing attached.

### Recommendation: KEEP

The one existing bind of `partner_role:{role}` is a placeholder on a card that
wants it gone, and the directory kind on CSD-104 is a read of someone else's
claim with no issuing side. A partners card is the issuing side. On adoption,
CSD-043's `partner_role` row should be rewritten to point here (the API role is
not a partner role, and §6 of that card already says so).

---

## CSD-124 Authority delegations

### What the client already covers

**CSD-055** is the delegation card, and ours says so: §1 "Why this is not
CSD-055 … The two should become one card when the chain read exists". The
overlap is most of our `shows` block:

| ours | CSD-055 §2 | note |
|---|---|---|
| `x_private:delegated_scope` (`["license","sub_delegation"]`, TIGHTEN-ONLY) | `x_private:delegated_scope` (`["read","write"]`, TIGHTEN-ONLY on approve) | same row; ours adds three scope tokens |
| `x_private:delegate_key_id` | `x_private:delegate_key_id` | identical |
| `x_private:delegation_depth` (`"3 of 5"`, assert 1..5) | `x_private:delegation_depth` (`unconfirmed`, `blocked_by: CIRISServer#663`) | same gap, same blocker |
| `x_private:delegation_chain` | `proposed:text_delegation_chain` (the tag on the depth row: "Who granted this before you") | the same element |
| `x_private:delegation_valid_until` | §3's note: "There is no `expires_at` on the wire … although every edge carries `delegation_valid_until` = issue + `GRANT_TTL_SECS` (600 s); the list cannot say when a grant lapses" | CSD-055 has found the gap our row depends on |
| `x_private:revoke_cascade` | §5 "That a revoke cascades. CC 4.5 says it must; the client cannot see the subtree, so it cannot assert it" | the sheet we specify is the one CSD-055 says it cannot draw without #663 |

CSD-055 §2 `surface: delegations` is placed in **every** circle's Rules tab
(`Placement(NavSurface.Delegations, Tab.RULES, ALL)`), so "Communities and
Businesses › Rules (the affiliation's chain)" and "the same card under My
things" in our §2 are both already that screen, scoped by circle.

CSD-055 §3 routes: `GET /v1/auth/device/grants` (flat), `POST
/v1/auth/device/{delegate,code,approve,deny,revoke}`, and the row "the chain
above a grant: missing … Blocks `building` for `text_delegation_chain`". Our §3
cites the same `GET /v1/auth/device/grants` as the read that is flat.

Two other client cards touch the same primitive:

* **CSD-090** (duty conferral) carries `x_private:delegated_scope_set` over the
  node's served vocabulary (`license` and `grant` included),
  `x_private:sub_delegation_depth` (the `(sub_delegation, depth)` pair: "Leaf,
  may act, may not delegate further … 5 further hops"), and the
  `ACCORD_CANNOT_CONFER` wall for `license` / `grant`. That is the conferral
  **ceremony** for the accord's root (two hardware holders); the scope picker
  and the depth pair are reusable UI, the root is not ours.
* **CSD-065 §3** reads `GET /v1/admin/self → owner_delegations[]` (0.5.218,
  CIRISServer#676): the owner's inbound delegations **with their scopes**. That
  is a partial answer to CSD-055's `inbound_delegations` gap and to our
  "read inbound edges" row; it carries no parent and no depth, so #663 stands.

### What is genuinely new in ours

The two emission scopes `license` and `grant` as the right to **issue** (never
a licence or grant itself), the affiliation's quorum as a delegator, the
roster precondition (`not_on_roster`), `effective_scope` as the intersection
along the chain, the required term under the charter's `role_term` with the
one-year ceiling and the `delegation_term_required` refusal, the six admission
refusal tokens shown as refusals, and the cascade sheet's second sentence:
issuances already made under an edge survive its withdrawal (a licence is
revoked by its authority, CC 2.4.1.2.1). After the catch-up, the worked
example is the `license` chain from `ciris-canonical` under its `quorum:2/3`.

### Recommendation: EXTEND CSD-055

Add to CSD-055 a section **§7 "Authority scopes: `license`, `grant`,
`sub_delegation`, and the chain"**, carrying:

* the three scope tokens added to `delegated_scope`'s vocabulary, each glossed
  in CC 4.4.3.4.3's words, with the rule that holding a licence or grant
  confers no right to issue one;
* a delegator that may be an affiliation's quorum (the `/changes/{envelope,
  cosign,assemble}` pattern from CSD-102 §3), with the roster precondition;
* our `delegation_chain`, `effective_scope`, `delegation_valid_until`
  (required; CSD-055 §3's 600 s TTL note becomes the stated gap: the wire must
  carry the term), `revoke_cascade` (one sheet, shared with the CSD-121 fold),
  `admission_refusal`;
* the worked `license` chain to `ciris-canonical` and the `formal/authority_tree/`
  flow (root → A (license, sub) → B (license) → refusals → cascade).

Dropped as duplicates: our `delegated_scope`, `delegate_key_id`,
`delegation_depth` (CSD-055 keeps its row and its single `blocked_by:
CIRISServer#663`; our §3 "read inbound and outbound edges with chain and
depth" row is folded into CSD-055's existing "the chain above a grant: missing"
row). CSD-055's `csd:topology` block is the one from CSD4_EVALUATION §4.2
(delegation chains, `cannot_act`), with our affiliation root added as a
`community` relation.

Why not KEEP: our own §1 says the two become one card; the only reason given
for a separate document was that the Portal's org-admin acts needed a home
before the chain read existed. With the Portal retired that reason is gone.

---

## CSD-125 Grants

### What the client already covers

No client card names `key_grant:{axis}`. The nearby surfaces each carry a
different "grant":

* **CSD-053** (Manage Consent): the peering grant, `consent:{kind}`,
  `POST /v1/federation/peering` → `grant_attestation_id`, and its withdrawal
  `POST /v1/federation/peering/revoke` (0.5.218, CIRISServer#657) with
  `remaining_grants`. A consent grant is **withdrawable**; a `key_grant` is not
  (CC 3.1.3), which is the single most important sentence on our card. The two
  must not share a row.
* **CSD-054** (Consent, the subject's half): `consent:{kind}` streams on the
  agent (`granted / revoked / expired`), `consent:partnership_grant`. Our card
  reads `consent:scope:{kind}` "in the same list" and says CSD-053/054 emit
  it; CSD-006 §2 confirms `consent:scope` is the one family leaf on the receipt.
* **CSD-107** (file custody) parses and does not render
  `access[{person_key_id, devices, via}]` on `GET /v1/files/{id}/custody`
  ("Read and not rendered: `announced_holders`, `access`, `tier`…"). That is
  the read side of "grants I issued over this file", already on the wire and
  unshown.
* **CSD-007** (Files) reads `bytes: here | not_fetched | not_granted` per drive
  row: the recipient side of a missing grant, as a state, not a list.
* **CSD-056** (Billing) is CIRISAgent's credits and Google Play; its
  "start a non-Play purchase" row (`POST /v1/api/billing/purchase/initiate`,
  "the Portal/Stripe leg; nothing reaches it") is the leg the Portal
  decommission retires. No metered-resource **grant** exists there, and after
  the catch-up our card no longer reserves one: the CIRISBilling row and its
  `CIRISRegistry#51` blocker are gone from CSD-125 (CIRIS sells nothing that
  would be metered), so nothing on CSD-056 touches it.

### What is genuinely new in ours

A list of grants over a thing, both directions (issued / held), with the
non-transferability rendered as an absence ("no share control at all"), end of
access as rotation and exclusion ("never 'revoke'"), the `grant`-scoped
delegate as the only other issuer, the v2 wrap requirement
(`wrap_algorithm`), and `key_validity_window`. Nothing in the client lists
grants; CSD-107 holds the only read of them and does not draw it.

### Recommendation: KEEP

No existing card can carry it without contradiction: CSD-053/054 are about
withdrawable consent grants and would make "revoke" the natural verb on a row
where CC forbids it; CSD-107 is a per-file custody sheet and §6 says "a fourth
file surface adds a door, not a card", which is what a grants list is. The
right joins are doors, not sections: CSD-107's unrendered `access[]` opens this
card's per-file view; CSD-007's `not_granted` state names it as the remedy.
The `consent:scope:*` row stays read-only here with CSD-053/054 as the emitters.

---

## CSD-126 Affiliation keys

### What the client already covers

After the registry fold "the organization's key" is a member's self-custodied
key bound by a delegation, and the client already has every primitive for the
member's side:

* **CSD-037** (My Identity): the person's device roster (`GET
  /v1/self/occurrences`), add (`POST /v1/self/occurrence`, with its signing-path
  gap), evict (`POST /v1/self/occurrence/revoke`, 0.5.218, CIRISServer#700), label,
  release, associate with `device_custody: tpm | software`,
  `hardware_custody:{platform}` (`unconfirmed`, `blocked_by:
  CIRISConstitution#107`), and the same `ownership:{relation}:{target_kind}:
  {version}` bind our card uses for the node's responsible party. Its §1 is the
  honesty rule our rotation row needs: eviction stops new wraps, "No DEK
  rotation; no re-encryption" (CC 4.5.12.1).
* **CSD-095** (the self-key roster's held changes): from two or more devices an
  add or an evict is a **proposal** (`GET /v1/self/roster/proposals`,
  `…/assent`, `…/object`, CIRISServer#802, persist v59). Our "register a key"
  and "revoke" on a member's own roster go through this path; the affiliation
  adds nothing to it.
* **CSD-104** (verify someone's key): the SAS ceremony over two Ed25519 keys,
  recorded as a self-scoped `config:*` row. Our "Reads with" cites it; nothing
  of ours overlaps it (it verifies, it does not register, rotate or escrow).
* **CSD-105** (trust root) and **CSD-067** (Accord): the only client rows for
  **supersession to a successor** are the accord's (`POST
  /v1/accord/canonical/supersede {old_key_id, new_record, proposal_digest}`,
  CSD-067 §3) and the root's recovery commitment leg (CSD-105
  `charter_has_recovery`, CC 3.2 T3). A pre-rotation commitment for a person's
  or affiliation's key has no client row.
* **CSD-085** (claim a node) is where `ownership:responsible_party:node:v1` is
  written; CSD-035 §2 reads it from `owned-nodes`, "NOT from an ownership:*
  row". Our `ownership:responsible_party:node:v1` display row is that read,
  already on two cards.
* No client card names `key:revocation` as a drawn row except CSD-108 (the
  purge ledger's count, "not a family in the vendored registry" at its pin; it
  is in rc12), and none names escrow or recovery.

The affiliation **binding** itself (a `KeyManager`-scoped `delegates_to` from
the founders' quorum to the member) is a CSD-055 / CSD-124 edge; our §3 row
says so ("the affiliation binding is CSD-124's edge").

### What is genuinely new in ours

The successor commitment and the rotation that honours it (`pre_rotation_
commitment`, `rotation`: "a supersession, the old key's rows stay verifiable"),
escrow with a named custodian and the recovery request as the custodian's own
signed act (CIRISPersist#752's custody index, the `registry_key_escrows` table
that survives the fold), the `held_by` rule ("never 'the registry'"), and the
list "keys that speak for this organization", which is the roster's members
filtered to holders of a key-binding scope.

### Recommendation: FOLD INTO CSD-037 and CSD-055

* **Into CSD-037 §2**, as rows on the person's own key: `pre_rotation_commitment`
  ("Successor committed"), `rotation` ("Rotate to", the old key's rows stay
  verifiable, CC 4.5.12.1's no-re-encryption sentence already on that card),
  `escrow_custodian`, `recovery_request`, and `key:revocation` drawn beside
  the occurrence eviction as the two families they are (an occurrence
  revocation is `identity:occurrence_revocation`; a key revocation is
  revoker-signed and judged without the subject's record, CC 3.1.3). Into
  CSD-037 §3: the escrow / recovery rows under `blocked_by: CIRISPersist#752`,
  and rotation under the same FSD-005 §7 gap. CSD-095 is untouched: a held
  add or evict is already how a multi-device person's roster changes.
* **Into CSD-055 §7** (the CSD-124 extension): the affiliation's key list is
  its outbound edges filtered to the key-binding scope, with `held_by` as the
  delegate and the `empty` sentence "No keys registered to this organization
  beyond its founders'". Our `key_id` and `held_by` rows become that filter;
  no new route (our §3 "list keys that speak for an affiliation" is the
  chain read, CIRISServer#663, already CSD-055's blocker).
* **Dropped**: `ownership:responsible_party:node:v1` (CSD-035 / CSD-037 /
  CSD-085 carry it); `register a key` (CSD-037's add plus CSD-055's edge);
  `revoke` (CSD-037's evict for the occurrence, the new `key:revocation` row
  for the key). The CIRISRegistry#65 and #66 citations go with the rows they
  blocked.

Why not KEEP: once the registry custodies nothing, an "affiliation keys" card
is a person's key card viewed through a delegation filter. Keeping it would
put register, rotate and revoke behind two doors (My Identity and Communities
› Rules), which is the two-doors failure CSD.md §4.1 fails the gate on.

---

## CSD-071 Audit: nothing of ours moves there, and one dependency

None of the seven cards duplicates CSD-071 (our README already excludes
audit). One dependency is worth recording: every "the receipt opens" and
"issued under {delegation}, the delegation's receipt opens" in our cards is a
read of a signed row by id, which is CSD-006's receipt over the feed CSD-071 §3
asks CIRISServer for (`GET /v1/attestations?cohort=…&since=…`, "serve the
corpus as a record feed"). The licensure, partners and grants cards inherit
that ask rather than filing their own.

---

## Final table

| our card | overlaps | recommendation | moves to / stays |
|---|---|---|---|
| CSD-120 Affiliation | CSD-102 §1 (CC 4.4.3.2.8, #649), §2 `cohort_scope`, `consensus_protocol`, `affiliations_declared_record`; §3 `POST /v1/communities`, `DELETE /v1/communities/{id}`; CSD-106 (found alone, inbox) | **EXTEND CSD-102** | new CSD-102 §2.3 "The affiliation charter": eleven charter fields, supersession rule, archetype default, empty state; §3 rows under the existing `blocked_by: CIRISServer#649` |
| CSD-121 Members and roles | CSD-103 §2/§3 (roster, add/remove, group book #650), CSD-106 §2/§3 (`membership:{proposal,acceptance,decline}`), CSD-102 §2 (`community_member_role`), CSD-055 §2 (`delegated_scope`, `delegation_depth` #663); CSD-043 explicitly not this | **FOLD INTO CSD-103 + CSD-055** | member-since-from-acceptance and the removal cascade fact → CSD-103; presets, `conferred_by`, required term, cascade sheet → CSD-055 §7; one sentence on role-word vs delegation → CSD-102 |
| CSD-122 Licensure | none renders `licensure:*`; CSD-052 §1/§2 (`attestation:license_validity` unrendered), CSD-104 §1, CSD-090 §3 (`ACCORD_CANNOT_CONFER` license) all disclaim it | **KEEP** | own file; cross-cite CSD-052 for the co-signature chip |
| CSD-123 Partners | CSD-043 §2 placeholder bind of `partner_role:{role}` (to be retired per its §6); CSD-104 §2 directory kind "partners" | **KEEP** | own file; CSD-043's `partner_role` row rewritten to point here |
| CSD-124 Authority delegations | CSD-055 §1 ("the only delegation card"), §2 (`delegated_scope`, `delegate_key_id`, `delegation_depth` #663, `text_delegation_chain`), §3 (`GET /v1/auth/device/grants` flat; TTL note), §5; CSD-090 (scope picker, `sub_delegation_depth`, accord wall); CSD-065 §3 (`owner_delegations[]`) | **EXTEND CSD-055** | new CSD-055 §7 "Authority scopes and the chain": `license`/`grant`/`sub_delegation`, quorum delegator, `delegation_chain`, `effective_scope`, required `valid_until`, `revoke_cascade`, `admission_refusal`, the `ciris-canonical` worked chain |
| CSD-125 Grants | none names `key_grant:{axis}`; CSD-053 (withdrawable peering grant), CSD-054 (`consent:scope`), CSD-107 §3 (`access[]` parsed, unrendered), CSD-007 (`not_granted`), CSD-056 (Stripe leg retired) | **KEEP** | own file; CSD-107's `access[]` and CSD-007's `not_granted` become doors to it |
| CSD-126 Affiliation keys | CSD-037 §1–§3 (roster, add, evict, custody, `ownership:*`), CSD-095 (held roster changes), CSD-067 §3 / CSD-105 (supersession, recovery commitment, roots only), CSD-085/035 (`responsible_party`), CSD-104 (not this) | **FOLD INTO CSD-037 + CSD-055** | successor commitment, rotate, escrow, recovery, `key:revocation` → CSD-037; "keys that speak for the affiliation" = CSD-055 §7 outbound edges filtered to the key-binding scope |

## The net set

After the recommendations the client maintains **three new cards and four
extended ones**; no new number is consumed beyond 122, 123 and 125, and 120,
121, 124, 126 are retired into their carriers (`numbers.tsv` gains four
`retired` rows naming the carrier, as CSD-001 and CSD-034 did).

**New files (adopted under their reserved numbers):**

* **CSD-122 Licensure**: the only card on the `licensure:*` plane; any
  authority under its own key, `ciris-canonical` as one of them, status sets,
  duties, re-emission to named readers.
* **CSD-123 Partners**: recognition at a `partner_role` with forums, apart from
  licence and from agency; retires CSD-043's placeholder bind.
* **CSD-125 Grants**: non-transferable access to a thing, ended by rotation;
  the list CSD-107 reads and does not draw.

**Client cards that gain a section:**

* **CSD-102** gains §2.3 "The affiliation charter" (from CSD-120) and one
  sentence on `community_member_role` (from CSD-121). Its `blocked_by:
  CIRISServer#649` now gates twelve rows instead of one placeholder.
* **CSD-103** gains the acceptance-row provenance on `joined_at`, the
  removal-cascade fact, and the affiliations-tier empty sentence (from
  CSD-121).
* **CSD-055** gains §7 "Authority scopes and the chain" (from CSD-124, with the
  role presets and required term from CSD-121 and the affiliation key filter
  from CSD-126). Its `blocked_by: CIRISServer#663` stays the single blocker for
  depth, chain, inbound edges and the affiliation key list.
* **CSD-037** gains the key lifecycle rows: successor commitment, rotation,
  escrow custodian, recovery request, `key:revocation` beside eviction (from
  CSD-126), with `blocked_by: CIRISPersist#752` on escrow.

**Unchanged by this set:** CSD-043 (its own §6 move to This node stands;
one row rewritten), CSD-071, CSD-095, CSD-104, CSD-105, CSD-106.

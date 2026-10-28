# Common-cards analysis: which of CSD-120..126 the client already carries

**Status:** DRAFT. Read against CIRISClient `origin/main` (the cards under
`FSD/CSD/`, `CSD.md` CSD/3, `packaging/check_csd_v3.py`, `CirclesNav.kt`) and the seven
cards in this directory (CSD/4; the Portal decommissioned; the CIRIS licence a row under
`ciris-canonical`; CC 4.4.3.8 "Who I trust").

**The organising frame: this set fills out the affiliations tier of the circles model
entirely.** The circles model is the client's layer hub
(CSD-050: `layer-agent`, `layer-family`, `layer-local-community`,
`layer-global-communities`; CSD-051: `layer-global-commons`), five circles by seven
tabs (`CirclesNav.kt`: Files, Chats, People, Safety, Rules, Decisions, Record). The
per-card sections below are the overlap check; the two sections after them place
every slot of the affiliations tier and recommend where the tier sits.

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
* Our `csd:tests` block (T6) stays on the extended CSD-102 beside its own T5
  entry only if the server's `requirements.yaml` keys tests per card; otherwise
  the three CSD-120 tests join CSD-102's entry (one card, one entry, CSD4.md §1).

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
`reconsideration:{grounds}` contest, and the CIRIS licence as
`licensure:ciris-canonical:v1` cosigned by the root community's quorum with no sale
behind it.

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
(CSD-122) and from agency (CSD-124). `partner_role:{role}` is open vocabulary
naming a role, with no tier, price or billing attached.

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
revoked by its authority, CC 2.4.1.2.1). The worked example is the `license`
chain from `ciris-canonical` under its `quorum:2/3`.

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
row). CSD-055 gains a `csd:tests` entry on T6 / T18 (our CSD-124 block), since
`delegation.chain` and `delegation.link.*` are the atoms its chain view needs.

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
  decommission retires. No metered-resource **grant** exists there, and our
  card reserves none: CSD-125 carries no CIRISBilling row (CIRIS sells nothing
  that would be metered), so nothing on CSD-056 touches it.

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

## Who I trust: the reader-side card

CC 4.4.3.8 "Who I trust" adds a surface none of our seven cards carried: the
reader's own trust set. Each entry is the reader's own `delegates_to(reader → authority)`
bearing `trust:accepts:v1`, naming the authority as a licensor for every class its
charter declares or for a listed class set (`trust_classes`), with a depth
(`trust_depth`, 0..5: licensors of licensors over the authorities' own acceptance
edges). The shipped root is one default, removable entry; removing it removes
exactly what reached through it (`removal_only_removes`, `TrustSet.lean`), keeps the
identity floor, and un-federates the node at the server tier until another root is
attached (CC 4.4.3.8 items 4–5). Wire members: CIRISPersist#1142.

### Is CSD-105 already that surface?

**CSD-105** ("This node's trust root: posture, adopt a seed, un-trust") is the
closest card, and it is half of it:

* **§1**: its mission is CC 3.2's re-root MUST ("untrust the canonical group, pin a
  different … community instead or in addition, or run with none"), and T3's one
  row: "remove the acceptance edge and every downstream gate fails closed". That
  row is the same `delegates_to(node → root)` bearing `trust:accepts:v1` the
  Who-I-trust set is made of (CSD-105 §2: "The one CEG object underneath, the
  node's own `delegates_to(node → root)` bearing `trust:accepts:v1` (CC 3.1.1), is
  what `DELETE` removes and `import` writes").
* **§2**: `surface: null`, `flow_only: true`, a leaf of the Accord card under
  Everyone › Safety; one posture card, one card per ROOT the node lists, one
  adopt-a-seed card. `x_private:node_accepts_root`, `root_valid`, `charter_legs`,
  `charter_quorum`, `hardware_custody:*` per seated holder, `drill_band`,
  `accord:halt_status` are all verdicts about a **root** (`TrustRootVerdict`).
* **§2.1**: the two confirms (adopt, un-trust) with three facts each; the un-trust
  sheet already says "when it is the last accepted root, every root-requiring gate
  fails closed and no `trace:*` row is served".
* **§3**: `GET /v1/trust-root`, `POST /v1/trust-root/import {bundle}`,
  `DELETE /v1/trust-root/{root_key_id}`, all `require_loopback`; the entry targets
  a root with a charter, holders and a lineage head (CIRISServer#693, #404 open).

What CSD-105 does not have: an entry that targets an **authority that is not a
root** (a hospital board, a guild); a class list and a depth on the entry
(`trust_classes` / `trust_depth`, CC 4.4.3.8 "Who I trust"); a list of
entries that is not a list of roots; and the warning that removing the root with
licensor entries remaining un-federates without un-trusting them. Its routes write
and remove a ROOT's edge from a bundle; no route adds a licensor entry with classes
and a depth.

### Recommendation: EXTEND CSD-105 ("Who I trust"), with a door on the affiliations tier

Add to CSD-105 a section **§2.2 "Who I trust: licensors, classes, depth"**:

* one row per acceptance edge the node holds, roots and licensors alike
  (`trust:{job}:{version}` bind `accepts`/`v1`, display-only; the root rows keep
  their posture cards, a licensor row shows its charter's declared classes and the
  entry's `trust_classes` ∩ that list, and its `trust_depth`);
* add an entry: pick an authority (an affiliation's id, a key, or a class of
  licensor — "every medical board that {authority} trusts" is an entry on that
  authority with `trust_classes: [medical]` and `trust_depth: n`), a class set or
  "everything its charter declares", a depth 0..5; the confirm's three facts: which
  node signs, what the edge says, what will now be honoured that was not;
* remove an entry, the root included: the confirm states what stops being honoured
  (`removal_only_removes`) and, for the last root, **CC 4.4.3.8 items 4–5 as the
  warning**: "This node keeps its identity and the licensors you added, but is no
  longer federated until it trusts another root. Licensor trust is not root trust.";
* `empty`: "You trust nobody. No licence is honoured." (`empty_set_honours_nothing`),
  distinct from the error block as CSD-105 already separates four blocks;
* §3 rows: the two existing routes for roots; a new door for a licensor entry
  (`blocked_by`: the README's asks of CIRISServer); `trust_classes` /
  `trust_depth` `blocked_by: CIRISPersist#1142`.

**Why EXTEND and not a new card.** The entry is the same row CSD-105 already
writes and removes, through the same two confirms; a second card would be a second
door on `DELETE /v1/trust-root/{id}` and `POST /v1/trust-root/import`, which the
route gate fails (CSD.md §4.1). The one real difference, root versus licensor, is a
column on the row, not a card.

**The door on the affiliations tier.** CSD-105 is a leaf under Everyone › Safety ›
Accord and loopback-only, which is right for a root and wrong for a person who
wants to say "I trust the state medical board" from the circle where licences are
read. The affiliations tier's Rules tab already carries a row named **Trust**
(`NavSurface.Trust`, CSD-052), which CSD-052 §1 says is "misnamed and misplaced":
named for `trust:{job}:{version}`, rendering the attestation ladder, "in a tab where
a person is looking for" licensure standing. That row is where Who I trust belongs:
the recommendation is that CSD-052's content moves to My things › This node (its
own §1 implies it) and the Rules › Trust row on Communities and Businesses opens the
CSD-105 extension (one card, two doors, as CSD-107 is reached from three surfaces).
The loopback gate stays the node's; off the node's machine the row renders CSD-105's
`trust_root_loopback_only` sentence, never an empty list.

**No card may say the root is the only trust knob.** CSD-122 §1 and §2 say the
reader half in these terms; CSD-120 and the README say the root is a default entry.

---

## The affiliations tier, slot by slot

The circle is **Communities and Businesses** (`GLOBAL_COMMUNITIES`), whose hub
already reads the community section at `tier: affiliations` (CSD-050 §1 table). The
seven tabs are `CirclesNav.kt`'s; the "today" column is the placement set on
`origin/main`; the "fill" column is the card that serves the slot for an affiliation
after this set is adopted.

| tab | row (NavSurface) | today | fill on the affiliations tier | verdict |
|---|---|---|---|---|
| Files | Files (CSD-007) | placed on four circles | the affiliation's drive at the affiliations cohort; a `not_granted` row opens Grants | existing; CSD-125 adds a door |
| Chats | AffiliationsChats (CSD-110) | placed | the affiliation's rooms | existing |
| People | Contacts (CSD-005) | all circles | contacts, unchanged | existing |
| People | Users (CSD-043) | `GLOBAL_COMMUNITIES` only | not an affiliation thing: CSD-043 §6 moves it to My things › This node | leaves the tier (CSD-043's own ask) |
| People | AffiliationsRoster (CSD-103) | placed | the roster, with the acceptance-row provenance, the removal cascade fact and the role chip opening the delegation sheet | FOLD CSD-121 into CSD-103 (+ CSD-055 §7) |
| People | **Partners** (CSD-123) | none | recognitions out and in, forums, the licence summary beside a partner | KEEP: new row |
| Safety | Moderation (CSD-065) | `NEIGHBOURS_OUT` | the moderator duty; for an affiliation the designated official discharges it (CC 4.4.3.2.8 "The floors bind power over members, not the count of voices"), and a one-seat body's act against a member needs an outside countersign (CSD-121's sheet) | existing; CSD-065 gains the affiliation sentence |
| Safety | ChildSafety (CSD-066) | all circles | unchanged | existing |
| Safety | *(none)* | — | **lawful access and transparency obligations** (CC 4.4.3.2.8 D: `lawful_access` typed, never covert, always `hard_case:*`; `transparency_obligations[]`); CSD-120 §5 defers it to "a later card" | **GAP → CSD-130** |
| Rules | LayerGlobalCommunities hub (CSD-050 + CSD-102) | placed | the community record and governance, plus the charter | EXTEND CSD-102 §2.3 (from CSD-120) |
| Rules | Trust (CSD-052) | all circles | **Who I trust**: licensors, classes, depth, the root as one removable entry | EXTEND CSD-105; this row becomes its door; CSD-052's ladder moves to This node |
| Rules | ManageConsent (CSD-053), Consent (CSD-054) | all circles | consent grants, unchanged; CSD-125 reads `consent:scope:*` from them | existing |
| Rules | Delegations (CSD-055) | all circles | the affiliation's chain: `license` / `grant` / `sub_delegation`, the quorum delegator, roles as presets, the key-binding filter | EXTEND CSD-055 §7 (from CSD-124; CSD-121's role half; CSD-126's key list) |
| Rules | **Licensure** (CSD-122) | none | issue, status set, duties, re-emission, the reader's verdict | KEEP: new row |
| Rules | **Grants** (CSD-125) | none | grants over the affiliation's assets (the same card under My things › Files for a person's own) | KEEP: new row |
| Rules | Billing (CSD-056) | `GLOBAL_COMMUNITIES` only | nothing: the Portal/Stripe leg is retired; agent credits are not an affiliation matter | leaves the tier (placement removed; CSD-056 itself is CIRISAgent's) |
| Rules | Wallet (CSD-057) | `GLOBAL_COMMUNITIES` only | out of this set's scope; its own card says what it is bound to | untouched |
| Rules | *(Affiliation keys, CSD-126)* | none | no row: a person's key card (CSD-037) viewed through the key-binding filter on CSD-055 §7 | FOLD CSD-126 into CSD-037 + CSD-055 |
| Decisions | HealthReputation (CSD-044) | `NEIGHBOURS_OUT` | unchanged | existing |
| Decisions | *(none)* | — | **pending cosignatures**: the affiliation's held changes of every kind (charter supersession, roster, licence issue/suspend/revoke, a root delegation edge), each a `/changes/{envelope,cosign,assemble}` 409 `quorum_pending` that CSD-102 renders only for roster changes on the Rules hub | **GAP → CSD-131** (or EXTEND CSD-102's held-change rendering with a Decisions door; a number is proposed so the slot is not empty) |
| Record | Audit (CSD-071) | all circles | the record feed; every "the receipt opens" in this set reads it; the Portal's compliance export is retired, not migrated | existing |
| Record | *(none)* | — | **records duties**: retention floors and ceilings per class, `legal_hold`, `erasure_policy` (Art 17(3) refused-with-reason), `disclosure_accounting`, `breach_notification`, `archive_custody` (CC 4.4.3.2.8 A, B and D: affiliation-only) | **GAP → CSD-132** |
| People / Rules | *(none)* | — | **the public face of an affiliation**: existence, `community_key_id`, officers' roles, declared licence classes, Commons-visible; a non-public affiliation's `community:listing:v1` row naming those four and nothing else (CC 4.4.3.2.8 "Public face") | part of CSD-120: the four things are members of the record it renders, and the listing row is signed at its found door |

**Numbers.** `numbers.tsv` on the client reserves CSD-118 and CSD-119 on #179
("Never allowed", "Guardian bond"), so they are not free; CSD-129 is in the server's
`requirements.yaml` (the halt card). The roster's unassigned list is 009, 017–019,
038, 058–064, 072–079, 088–089, 097–099 and everything above 128; this note proposes
**CSD-130, 131, 132** as new numbers above the reserved range, to be added to
`numbers.tsv` as `reserved` rows naming this file.

## Own scope, or inside `layer-global-communities`?

**Recommendation: the affiliations tier is the circle that exists, made explicit —
not a sixth circle, and not "beside global communities".** In the circles model's
own terms:

1. **A circle IS a `cohort_scope`** (CSD-050 §1: "a circle IS a `cohort_scope`, so
   the card that names the scope is the card that says what may cross out of it"),
   and `affiliations` is one of CC 2.1's closed set of seven. "Global communities" is
   not a CC kind: scale is a `cohort_subkind` and roster matter (CC 3.2), "global" is
   the Commons (CC 4.4.3.2.1).
2. **The hub already reads the tier.** `LayerGlobalCommunities` reads the community
   section at `tier: affiliations` (CSD-050 §1 table); `AffiliationsRoster` and
   `AffiliationsChats` are its People and Chats rows. The circle has been the
   affiliations scope under another label since CSD-102 mounted it.
3. **Order.** Every CC list writes `self | family | community | affiliations |
   species | biosphere | federation`, and the circles run Just me, Family,
   Neighbours, Communities and Businesses, Everyone: the affiliations circle sits
   between the local community and the Commons already. Whether `affiliations` is
   *wider* than `community` is settled by CC 4.4.3.2.8 "The scopes":
   "`affiliations` sits beside `community`, neither wider nor narrower, so a re-sign
   between them is a new placement, never a promotion and never the narrowing a
   placement-touching re-sign may make; the order of widening is `self`, `family`,
   then `community` and `affiliations` side by side, then `species`, `biosphere` and
   `federation`." The two circles are siblings, and no arrow runs between them.
4. **Why the tier is distinct at all** (CC 4.4.3.2.8 "The assumption each tier
   carries"): a community is presumed pro-social and the floors protect peers from one
   another and from one voice; an affiliation carries no such presumption and the data
   limbs bind it alone. And the line between the two is the licence class, not the
   label ("Who may license"): anyone may license, a community included, and "a
   community whose charter declares a class with legal weight has taken on the
   affiliation's duties for it". So the tier exists to carry duties, sharing the
   community's crypto tier and feed band (CC 4.4.3.2.1; CC 4.4.3.3).

So: keep the `layer-global-communities` nav id (renaming it churns every tag the
flows name), declare `cohort_scope: affiliations` on its row in CSD-050 §2, and let
the client decide the label ("Businesses and institutions" says what CC says;
interest communities of any scale belong to Neighbours, `cohort_scope: community`).
A separate `layer-affiliations` circle beside this one would be two circles for one
scope; a circle for interest communities "beyond local" would be a circle for a
scope CC does not have.

## Final table: the affiliations tier, fully covered

| slot (tab › row) | our card | existing client card(s) | verdict | where it lives after adoption |
|---|---|---|---|---|
| Rules › hub (the record and charter) | CSD-120 | CSD-102 §1–§3 (`affiliations_declared_record` #649), CSD-050 | **EXTEND CSD-102** | CSD-102 §2.3 "The affiliation charter"; CSD-120 retired into it |
| People › roster | CSD-121 (membership half) | CSD-103 §2/§3, CSD-106 §2/§3 | **FOLD INTO CSD-103** | acceptance-row provenance, removal cascade, affiliations empty sentence |
| Rules › Delegations (roles) | CSD-121 (role half), CSD-124, CSD-126 (key list) | CSD-055 §1–§3 (#663), CSD-090, CSD-065 §3 | **EXTEND CSD-055** | CSD-055 §7 "Authority scopes and the chain": presets, term, cascade, `license`/`grant`, the `ciris-canonical` chain, the key-binding filter |
| Rules › Licensure | CSD-122 | none renders `licensure:*` (CSD-052/104/090 disclaim it) | **KEEP** | CSD-122, new row on Rules; reads the Who-I-trust verdict |
| People › Partners | CSD-123 | CSD-043 §2 placeholder bind, CSD-104 §2 kind | **KEEP** | CSD-123, new row on People |
| Rules › Grants (and Files door) | CSD-125 | CSD-053/054 (withdrawable consent), CSD-107 §3 `access[]`, CSD-007 `not_granted` | **KEEP** | CSD-125, new row on Rules; doors from CSD-107 and CSD-007 |
| My things › My Identity (keys) | CSD-126 (lifecycle) | CSD-037 §1–§3, CSD-095, CSD-067/105 | **FOLD INTO CSD-037** | successor commitment, rotation, escrow, recovery, `key:revocation` beside eviction |
| Rules › Trust (Who I trust) | — (new, reader side) | CSD-105 §1–§3 (root edges, two confirms), CSD-052 (the misplaced row) | **EXTEND CSD-105** | CSD-105 §2.2 "Who I trust"; the Rules › Trust row on this circle is its door; CSD-052 moves to This node |
| Safety › lawful access, transparency obligations | — | none | **GAP → CSD-130** | new card; CC 4.4.3.2.8 D |
| Decisions › pending cosignatures | — | CSD-102 (roster held changes only) | **GAP → CSD-131** | new card, or CSD-102's held-change rendering with a Decisions door |
| Record › records duties | — | CSD-071 (the feed only) | **GAP → CSD-132** | new card; CC 4.4.3.2.8 A |
| People / Rules › public face | CSD-120 | none; CSD-102's public rooms are a different thing | **part of CSD-120** | CSD-120 §2: `declared_cohort`, `public_face`, `listing_row`; CC 4.4.3.2.8 "Public face" |
| Files, Chats, Contacts, Moderation, ChildSafety, Consent ×2, HealthReputation, Audit | — | CSD-007, 110, 005, 065, 066, 053, 054, 044, 071 | **existing** | unchanged (Moderation: the designated official discharges the duty for an affiliation, CC 4.4.3.2.8 "The floors bind power over members, not the count of voices") |
| Rules › Billing; People › Users | — | CSD-056, CSD-043 | **leave the tier** | Billing's Portal leg is retired; Users moves per its own §6 |

## The net set

After the recommendations the client maintains, for the affiliations tier:

* **three new cards from this set** under their reserved numbers: CSD-122
  Licensure, CSD-123 Partners, CSD-125 Grants;
* **five extended cards**: CSD-102 (§2.3 charter; the role-word sentence), CSD-103
  (acceptance provenance, removal cascade, empty sentence), CSD-055 (§7 authority
  scopes and the chain), CSD-037 (key lifecycle), CSD-105 (§2.2 Who I trust, with
  its door on this circle's Rules › Trust row);
* **four retired numbers**: 120, 121, 124, 126, each a `retired` row in
  `numbers.tsv` naming its carrier (as CSD-001 and CSD-034);
* **three gaps with proposed numbers**: CSD-130 lawful access and transparency
  obligations, CSD-131 pending cosignatures, CSD-132 records duties (the public face
  is not a gap: CC 4.4.3.2.8 "Public face" makes it four members of the record
  CSD-120 renders, so it is CSD-120 §2, not a card);
* **two rows that leave the tier**: Billing (CSD-056 placement) and Users (CSD-043,
  per its own §6);
* **unchanged**: CSD-007, 110, 005, 065, 066, 053, 054, 044, 071, 095, 104, 106,
  057.

Every slot the layer hub gives a scope is then filled or named as a gap with a
number; no slot of the affiliations tier is left to a card that does not exist.

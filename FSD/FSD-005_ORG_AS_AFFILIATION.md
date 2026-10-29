# FSD-005 — An organization is an affiliation: the Portal's concepts on CEG

**Status:** DRAFT. An org is an affiliation in CEG; this document completes that
translation and models the full state tree for grants and delegations so it closes for
the enumerable depths.
**Reads with:** FSD-004 §4.3 (the PortalService disposition table this supersedes for
org data), `formal/authority_tree/` (the state tree), the CSD set under `FSD/CSD/`.
**Constitution:** CC 1.0-rc14. The registry grammar is pinned at `registry_sha256`
`df79a1dde30a380380da83f0d776737e580705b6305741f037a5a78c2bef9898` (189 families); the CSD set under `FSD/CSD/` carries the same pin.

---

## 1. The one sentence

**The Portal's `Organization` is a community at `cohort_scope: affiliations`
(CC 4.4.3.2.8); its members, roles, partners, licences, grants and delegations are the
rows CEG already has for an affiliation, and nothing about an organization is a record
kind of its own.** Anyone can found one under their own key. There is no system
administrator who creates organizations, and no roster of permitted licensing
authorities. CIRIS L3C is one such affiliation and uses the same cards; its licences are
`licensure:ciris-canonical:v1` — the root community the accord holders founded is the
affiliation, and its `community_key_id` is the authority (CC 3.3.9; CC 2.4.1.2.1;
ciris.ai/philosophy: universal application of the rules to CIRIS itself). CIRIS sells
none of them.

## 2. Why the translation is owed

CC 3.3.9 carried three operational subject kinds — `organization`, `org_membership`,
`partner_record` — written when the Registry was a central service federating a
"trust/authz-minimal projection" of its Portal tables. They were the Portal's tables
with the PII removed.

CC 4.4.3.2.8 defines the affiliation: the institutional cohort, gathered by necessity,
with a declared config record (archetype, classification, retention, hierarchy,
designated officials, lawful access) and the whole community machinery (roster, DEK
cascade, `consensus_protocol` admission, `membership:{stage}` rows). An organization has
every property an affiliation has, and the affiliation has the governance an
organization needs that the operational kinds never had.

Two records for one thing is the drift CC 3.1.7 R2 forbids. The affiliation survives:
CC 3.3.9 opens with "an organisation is an affiliation" and retires the three kinds by
amendment (CC 4.5.1), their names kept so citations resolve and no substrate admitting
them as record kinds; persist v60 (migration V205) drops their tables.

## 3. The translation, concept by concept

| Portal / FSD-004 concept | On CEG | Signed by | Where it lives |
|---|---|---|---|
| **Organization** (`org_id`, name, type, parent, status) | an affiliation: a `community` record with `declared_cohort: affiliations` (CC 4.4.3.2.8 "The record"), `community_key_id` = the former `org_id`; `name`; `cohort_subkind` as today; `parent_org_id` → `hierarchy` nesting (CC 4.4.3.2.8 C: "may nest federated sub-affiliations"); `org_type` → `affiliation_archetype` + `membership_basis` | founders (CC 3.2 T6: the head is the signed roster record at a version) | community record + its config record |
| **Org user** (a person in the org) | a **member**: `membership:{stage}` rows (proposal → acceptance → widening) onto the roster; the person's fed-ID is the member, their devices follow by owner-binding | inviter + invitee (CC 3.1.3.2: nobody joins without their own signed acceptance) | roster record versions |
| **Org role** (`OrgAdmin` / `KeyManager` / `Operator` / `Viewer`) | a **`delegates_to` edge in the affiliation's `hierarchy`** (CC 4.4.3.2.8 C), from the affiliation's root (its founders' quorum, or a `sub_delegation` holder) to the member, carrying the scopes the role bundles (§4). The four names survive as **presets**, not as a wire enum (CC 3.3.9) | the delegator, within its own scope | `delegates_to` rows at the affiliation's scope |
| **Partner** (`partner_record`: licence type, capabilities, limits, status, revision) | the affiliation **as authority** issuing `licensure:{community_key_id}:v1` rows about the partner's key(s) with the status set, plus `partner_role:{role}:v1` for the role, `duty:{kind}:v1` for obligations (supervision, limits), `multilateral_participation:*` for forum standing. Capabilities ride `scope` on a `delegates_to` where they are agency, and the licence where they are standing (§4) | the affiliation's quorum, or a `license`-scoped delegate | scores rows at federation scope |
| **Licence** (issue / suspend / revoke / expire / renew) | `licensure:{authority_id}:v1` with `authority_id` = the affiliation's `community_key_id` (CC 3.3.9: `org_id` is the affiliation's `community_key_id`). `suspended` is reversible (reinstate = `withdraws` on the suspended row); `revoked` is terminal; `lapsed` is the authority saying expiry happened; several statuses may be live | as above | scores rows |
| **Delegation** (act on the org's behalf) | `delegates_to` with the CC 4.4.3.4.3 scopes; `license` and `grant` are the two that convey issuance rights; chains need `sub_delegation`; depth ≤ 5; attenuation; withdrawal cascades | the principal | `delegates_to` rows |
| **Grant** (access to a thing) | `key_grant:{axis}:v1` per recipient (content/epoch/stream) and `consent:scope:{kind}` grants; issued by the asset's owner or steward, or a `grant`-scoped delegate; **non-transferable**; never withdrawn (rotation is the revocation) | the sealer / owner | scores rows at the content's scope |
| **Keys** (register, rotate, revoke, escrow) | a member's own `SignedKeyRecord` (self-custody; the Registry never holds a partner's private key again — FSD-004 §6.1); rotation by supersession with a pre-rotation commitment (CC 3.2 T3); revocation `key:revocation`; escrow as a node-local custody index (CIRISPersist#752) | the key's owner | key plane |
| **Audit** | persist audit over the rows above; every act above IS its own receipt | — | — |
| **Org PII** (emails, tax id, OAuth, billing) | **off the wire, as before** (CC 3.3.9 governing principle). It lives on the node the affiliation is administered from, under the affiliation's DEK, or nowhere | — | node-local, affiliations tier |

### 3.1 What does not translate, and is dropped

- `CreateOrganization` by `SYSTEM_ADMIN`: no such door. Founding is self-founding
  (CC 3.2: a community is born by its founders' signatures).
- `GenerateKeyPair` / `RequestSignature` (the Registry holding partner keys): delegation
  laundering under CC 4.1.1; dropped in FSD-004 and stays dropped.
- `GetOrgUserByEmail`: an email is not a federation identifier (FSD-004 §4.3).
- `license_type` as an enum on the wire: the practised domain is the `authority_id`; the
  role is `partner_role:{role}`; obligations are `duty:{kind}`.
- The `organization` / `org_membership` / `partner_record` subject kinds: retired by
  amendment (CC 3.3.9, CC 4.5.1), not transitional. Their names stay in CC 3.3.9 so
  citations resolve; persist v60 (V205) drops the tables and the replication kinds;
  everything they carried lives on the affiliation record.

## 4. Roles as presets over delegation scopes

An `org_membership.role` was a four-value enum the Registry's JWT layer checked. On the
affiliation it is a `delegates_to` edge whose `scope` set names what the member may do
for the affiliation. The four names survive as presets a card offers; the wire carries
the scopes.

| Preset | Scopes on the edge (CC 4.4.3.4.3 table) | Who may confer it |
|---|---|---|
| `OrgAdmin` | `sub_delegation`, `license`, `grant`, plus the affiliation's declared admin verbs (roster `membership:proposal`, config supersession) | the founders' quorum, or an OrgAdmin (sub-delegation, attenuated) |
| `KeyManager` | the key-registration binding for the affiliation's own keys; no `license`, no `grant` | OrgAdmin |
| `Operator` | `act_on_behalf`-class verbs the affiliation declares (sign operational rows), no issuance rights | OrgAdmin |
| `Viewer` | none; membership alone (reads ride the roster) | — |

**The root is a human.** An affiliation's chain roots in its founders' keys, and a
founder is a person: a node has no agency and cannot found, hold a role, or be a root
(CC 3.4.7.3; ciris.ai/constitutional-mesh: standing is "traceable through a live chain
to an accountable human"). An agent may be a member and hold a role through its
partnership (CC 4.4.3.4.3); it is never the root.

**Authority decays, and a perpetual edge is refused.** Every role edge carries a term
(`delegation_valid_until`) and is renewed by re-conferral; an edge with no term, or a
term past the ceiling, is refused at the door (ciris.ai/constitutional-mesh: "It decays
like everyone else's, seniority included. It never becomes permanent sovereignty").
**The one-year ceiling is constitutional; the term itself is a charter parameter**
(CC 4.4.3.2.8 C; CSD-120), so a small affiliation may choose ninety days. This costs
nothing structurally: the founders are the roster root, not an edge, so when every
admin's term lapses the quorum chosen at founding still re-confers, and the subtree
below a lapsed edge lapses with it. **A lapsed `moderate` edge is a moderator lapse
under CC 4.5.4**: auto-promotion or fail-secure in the same step, never an unmoderated
window.

The rule on the wire: `delegation_valid_until` is REQUIRED on any edge carrying
`sub_delegation`, `license`, `grant`, `moderate`, `takedown`, `review` or `slash`, or
sitting in an affiliation's `hierarchy` (CC 4.4.3.2.8 C; CC 2.4.1), at most 365 days
after `delegation_valid_from` and within the charter's `role_term` (integer days
1..=365; an absent `role_term` means the ceiling, and a charter may lower it and never
raise it, CC 4.4.3.2.8 "Charter defaults"). The refusal is `delegation_term_required`
(persist v60: `federation_delegation_term_required`, rules `term_missing` /
`term_unreadable` / `over_ceiling` / `over_role_term`). Register: `CLM-delegation-term`.

Everything below the root is attenuated: an OrgAdmin created by sub-delegation holds a
subset of what their delegator held, never more, and a withdrawal at any link removes
every role beneath it (CC 4.5). A role is a leaf of the one tree, with two kinds of leaf
(a licence, a grant) hanging off the same tree. The default depth cap is 5 (CC 4.1.1,
"configurable"), and CC 4.4.3.2.8 C lets an affiliation declare a deeper cap for a deep
secretariat; this document takes the default and treats a deeper cap as a charter
declaration (CSD-120), never a silent widening.

*What the Constitution says for this section.* The seat is by two consents in both
kinds and a delegated admin never stands in for the protocol (CC 3.1.3.2; CC 3.3.9); the
four names are non-normative presets over scope sets, and the admin presets are data on
the family model manifest, not a closed set in the text (CC 3.3.9; CC 4.4.3.2.8 "The
record"); in an affiliation every role edge is termed where a community terms only
duty-bearing edges (CC 2.4.1; CC 4.4.3.2.8 C); an organisation on the bond is reached by
`slash` and `quarantine` (CC 3.2, canonical `cohort_subkind: infrastructure`, "The
organisation is on the bond"); succession is the charter's: a successor seat or
`dissolves_at`, else dissolution at the last seat's lapse (CC 4.4.3.2.8 "Succession and
dissolution"). The cards cite the same paragraphs.

*The protocol floor.* CC 4.4.3.2.8 carries three normative paragraphs on it. "The
assumption each tier carries": a community is presumed pro-social and the CC 4.5.13 /
4.5.4 floors protect peers from one another and from one voice; an affiliation carries
no such presumption and the data limbs bind it alone. "One member is enough": a sole
practitioner is an affiliation, holds and licenses under the limbs from creation, and
members hold no governance vote unless the charter grants one. "The floors bind power
over members, not the count of voices": appointment is the protocol signature plus the
appointee's `membership:acceptance`; a one-seat body's act against a member — removal,
quarantine, slash, takedown — is admitted only countersigned by a key outside the seat,
the designated official or a review-holder of the overseeing body, else it is a CC 4.5.5
reconsideration proposal; the designated official discharges the CC 4.5.4 duty; two or
more seats decide under their own protocol as a community. CC 4.5.12.5 admits
`founder_only` for an affiliation "only under the one-voice countersign rule of
CC 4.4.3.2.8". Register: governance row `affiliation_member_action` (cosigned 2/2);
model `formal/affiliation/Affiliation.lean`; `CLM-affiliation-one-voice-countersign`
staged on CIRISPersist#1143. So one founder suffices to issue, under `founder_only`;
never alone for an act against a member.

## 5. The authority tree, and that it closes

`formal/authority_tree/authority_tree.py` models the triad as one state machine and
enumerates every reachable state of a small universe (153,856 states with every action
interleaved at two links, 2,218 delegation-only states at three, the depth chain closing
at five with 4,096 refusals past the cap; 0 violations; 27 s; in CI), checking nine
invariants in each: attenuation, the depth cap, revocation cascade, licence closure
(holding a licence never confers issuance), grant closure (holding a grant never confers
re-granting), enforced admission, the `sub_delegation` requirement, acyclicity, and
history-independence of the verdicts. A separate chain universe checks closure at the
cap: the resolver confers nothing past five links and a sixth link is refused at the
door.

The answer to "should all three be delegable, or should the two ride the one": **the two
ride the one.** Only delegation chains. The right to issue a licence or a grant is
delegated; the licence and the grant are leaves.

## 6. What each repo carries

| Repo | What it carries | Why |
|---|---|---|
| CIRISConstitution | CC 3.3.9: an organisation is an affiliation; the three kinds retired by amendment; `authority_id` is the `community_key_id`; the role presets are non-normative presets over scope sets. CC 2.4.1.2.1: the authority set is whatever the affiliation's `consensus_protocol` admits; a licence is a cosigned row judged at its signed `asserted_at`; no public default, scope per licence class in the charter (`licence_audiences`); re-emission to named readers is new cosigned rows with the readers in `subject_key_ids`. CC 4.4.3.2.8 C: the term is required, ceiling 365 days. CC 4.5.5 and 2.4.1.1: a licensee's contest path is `reconsideration:{grounds}`, never withdrawal, and a subject can see every licence about them. CC 4.4.3.8 "Who I trust": the reader's trust set | the rules the rows below implement |
| CIRISPersist | §7.2: the affiliation authority-set arm for licensure; the `as_of` lens; cycle refusal at admission; typed affiliation config + discriminator; licensure public read; the `grant` gate; per-recipient `key_grant`; the three kinds retired (V205); the term gate. Open: `trust_classes` / `trust_depth` on `trust:accepts:v1` (CIRISPersist#1142); the one-voice countersign, the public face and dissolution (CIRISPersist#1143) | storage + admission of the rows above |
| CIRISEdge | §7.3: a commons reach for registry families on a conferred node; subject-pull by non-subjects; consent-prefix coverage; depth default 5 (CIRISEdge#851); licence audience carriage (CIRISEdge#895); delegation term carriage (CIRISEdge#896); per-recipient `key_grant` carriage (CIRISEdge#808) | carriage + audience of the rows above |
| CIRISServer | §7.4: every write door (found an affiliation with its charter, roles as `delegates_to`, licences, grants, `license`/`grant` delegations, keys: CIRISServer#752) and the public read tier a registry node serves; the chain read (CIRISServer#663); registering every kind (CIRISServer#646) | the cards |
| CIRISClient | the CSD set under `FSD/CSD/` here, to be adopted into `CIRISClient/FSD/CSD/` | the cards |

## 7. What the substrate provides and lacks

Three read-only audits, one per substrate repo, against the product decision in §1 and
the authority model in §5. The sizes are the auditors' estimates.

### 7.1 The four requirements across repos

1. **An affiliation is a licensing authority (Persist).** The licensure admission
   predicate originally accepted only an attester whose key *is* `authority_id`, or a
   `license` chain walked out of that key; a `community_key_id` is keyless (no
   `federation_keys` row, never signs, emits no edges), so every licence issued under an
   affiliation was refused (`licensure_delegator_not_authority`) or admitted as
   testimony. The rule (CC 2.4.1.2.1): the full range of persist's quorum models is
   selectable when the affiliation is founded (`consensus_protocol`: `founder_only`,
   `unanimous`, `majority`, `quorum:M/N`, …) and may be changed later by a decision of
   the active quorum on the active roster. A licence is an act of the affiliation under
   its current `consensus_protocol`, the same way a roster change is: one founder
   suffices only where the protocol says so. The persist arm is "authority set =
   whatever the affiliation's active protocol admits" (persist v60
   `admission::licence_authority_set`), and Server's door is the existing
   `/changes/{envelope,cosign,assemble}` pattern.
2. **Outsiders can verify an affiliation's licence (Persist + Edge).** Community
   records, rosters and the internal `delegates_to` hierarchy replicate only to member
   nodes, so a registry node holding a `licensure:{community_key_id}` row could read the
   row as testimony but not fold it as the authority's licensure. The rule
   (CC 2.4.1.2.1; CC 1.13.3.4): a licence is a claim about a person, so **there is no
   public default**; the scope of a licence is the authority's declared choice per
   licence class, the smallest that fits (`licence_audiences` in the charter,
   CC 4.4.3.2.8 "Charter defaults"). A practice licence meant to be checked by strangers
   is public by its nature; many licences are not. Whatever the class's scope, the
   licence **and the authority head a verifier needs** travel together at that scope,
   re-emitted to named readers where the class is not public (new cosigned rows with the
   readers in `subject_key_ids`, persist v60 `reemission_of`), never by widening the
   roster. Carriage: CIRISEdge#895.
3. **A registry node serves rows to "any client" (Edge).** A peer is served a
   federation-scope row only if the serving node has authored a `consent:replication`
   grant naming it; otherwise it gets first-contact reach, which carries only the
   server's own allegiance rows. Edge needs a `Reach::Commons` for registry families on
   a conferred node, gated by Rooted and quarantine, plus subject-pull on the
   Attestation plane for readers other than the subject. Until then "is X licensed?" is
   answerable over replication only by X, and everyone else goes through Server's HTTP
   read.
4. **Changing an affiliation works on Postgres and SQLite (Persist).**
   `federation_group_versions.cohort` was `CHECK (cohort IN ('family','community'))`
   (V089) while `supersede_affiliations` inserted `'affiliations'`. Fixed in persist v60
   (CIRISPersist#1034).

### 7.2 Persist

Persist v60 carries: the affiliation authority-set arm, `asserted_at` with the 72 h
window and `asserted_at_stale`, `licence_audiences` and re-emission, the typed
`partner_record` payload on the licence, V205 dropping the three operational tables, and
the `reconsideration` contest (CIRISPersist#1036); cycle refusal at admission
(CIRISPersist#1031); the term lens with `as_of` (CIRISPersist#1032); the `grant` arm
(CIRISPersist#1033); the typed `AffiliationConfig` with `declared_cohort` and the V089
fix (CIRISPersist#1034); licensure gate/fold agreement, the public read and
list-by-authority (CIRISPersist#1035); the term gate `federation_delegation_term_required`
(CIRISPersist#1048); the organisation on the bond (CIRISPersist#1062); per-recipient
`key_grant` rows (CIRISPersist#989); the custody index `registry_key_escrows`
(CIRISPersist#752).

Open: `trust_classes` / `trust_depth` inside the reader's signature on a
`trust:accepts:v1` edge, and the class-filtered, depth-bounded walk at read
(CIRISPersist#1142); the one-voice countersign (`affiliation_member_action`), the public
face and the listing row, succession and dissolution as the record's `withdraws`
(CIRISPersist#1143).

### 7.3 Edge

| Area | Status | Finding |
|---|---|---|
| Carriers | mostly exists | 19 `EnvelopeKind`s, all served. `delegates_to`, `licensure:*`, `duty:*`, `membership:*`, `consent:scope:*` ride the Attestation plane; affiliations ride Community. `licensure:` classifies `Unknown` → no family gates → Cohort projection, advertised. `key_grant` per-recipient: CIRISEdge#808. The three operational kinds were served to every peer unfiltered; they are retired (CC 3.3.9) and nothing may emit them. |
| Host side | gap | CIRISServer#646: the host registers 6 of the kinds, dropping Family, membership revocations/widenings, Revocation, IdentityOccurrenceRevocation. |
| Audience | the main gap | Requirement 3. Consent prefixes do **not** filter per recipient; they decide promotion of local-tier rows to federation tier. If Server emits at local tier, the prefixes must cover `licensure:`, `duty:`, `partner_role:`, `revocation:`, `consent:scope:`, `key_grant:`, `membership:` and the hierarchy's `delegates_to` dimension; edge's defaults cover `capacity: chat: ownership: self:delegates_to: trace:` only. A dimension-less `delegates_to` can never be promoted and must be emitted at `tier: federation`. Licence audience carriage: CIRISEdge#895; delegation term carriage: CIRISEdge#896. |
| Rooted | exists | The per-peer-pair floor compares the two nodes' owners, never the row's author, so thousands of affiliations add nothing. A read-only node must still be claimed, accept a common root, and publish its key/occurrence/transport rows. |
| Blobs | nothing new | Affiliation documents are CommunityDek blobs held by members only; what outsiders must read goes at federation scope. Per-class transparency promotion (CC 4.4.3.2.8 A) is not in edge. |
| Delegation evaluation | none | Edge knows none of `license` / `grant` / `sub_delegation`; persist owns depth, attenuation and cascade. The one edge-side walk (`message_io`) is off by default with depth 4, not 5 (CIRISEdge#851). |

What edge must provide: `Reach::Commons` for registry families on a conferred node;
subject-pull of registry rows by non-subjects; the licensing affiliation's authority
head carried with its licences at the class's scope (CIRISEdge#895); the registry
families in the consent-prefix lists, and refusal of a dimension-less `delegates_to` at
local tier; default depth 4 → 5 (CIRISEdge#851); release of parked licence rows when
their delegation lands.

### 7.4 Server

Server has **no route at all** for licensure, partners, organizations, key rotation,
escrow or an audit read. Affiliations exist only as a tier label on a room. Delegation
exists only as the device-grant flow. Persist's licensure fold, delegation reads,
key-grant listing, escrow table and typed affiliation config have **zero callers** in
Server. The doors are tracked as CIRISServer#752.

| Area | Status | Finding |
|---|---|---|
| Affiliations | partial | `POST /v1/communities {tier: affiliations}` is accepted and writes only `policy_blob = {cohort_scope: affiliations}`; messages are still placed at the community tier. Members are added by key (must already be a contact) with free-string roles; only `founder` has meaning; roster roles are not `delegates_to`. The roster read is members-only (non-member → 404). No config record (CIRISServer#649), no cohort-scoped roster read (CIRISServer#662), no public listing of `listed: public` members. |
| Delegations | partial | `GET /v1/auth/device/grants` is outbound-only, skips `infra:*`, has no depth, parent, `valid_until` or inbound read (CIRISServer#663); persist's `Engine::delegations_to` is unused. `POST /v1/auth/device/delegate` accepts any scope string but **keeps only the first scope**, binds no `authority_id`/`community_id`, and sets `valid_until` to the device-code TTL, **600 s**, so every edge from this flow dies in ten minutes (CIRISServer#751). Server's walk cap is 4, not 5. An admin act needs a `delegation_id` no route returns (CIRISServer#676). Chains can only be founder-rooted today. |
| Licensure | missing | Only consumer composition exists (`compose_policy.rs` `licensure_cap`, the CC 3.4.9 ≤ 0.5 rule). No writer, no reader; nothing issues `duty:{kind}` on a licence; `partner_role:{role}` has no writer or reader. |
| Grants | missing for clients | `key_grant` is emitted only internally; `consent:scope:*` appears only in `safety/infohazard.rs`; `POST /v1/auth/consent` writes a self-tier row with no `scope` member (not a CC consent grant) and needs client-side crypto; no list of peering grants (CIRISServer#680), no consent-grant withdraw (CIRISServer#657). |
| Keys | mostly missing | Device-occurrence add/revoke/label and portable associate exist. No identity-key rotation, no key history, no KeyManager binding, no public key reads, and `registry_key_escrows` is referenced nowhere. |
| Public read tier | partial | `compose_registry` gates and logs but mounts nothing; registry-core's fold router is mounted outside the gate as tier-P (CIRISServer#669). The default consent prefixes (`capacity: chat: ownership: self:delegates_to: trace:`) cover none of the registry families, so a conferred node never receives other authorities' licences and cannot answer for them. |

What Server must provide: a public listed roster (waits on CIRISPersist#912); durable
scoped delegation issue/withdraw with an attenuation precheck, fixing the first-scope
truncation and the depth constant (CIRISServer#751); the affiliation as delegation root;
licensure issue/suspend/revoke (single-key and quorum); licensure read per subject per
authority; `partner_role` and `duty` writer/reader; grants list and issue; identity-key
rotation and key history; escrow; a registry-slice replication policy (with edge);
`compose_registry` mounting the reads (CIRISServer#669). Buildable in Server alone, now:
the delegation door and reads, single-key licensure write and read, partner/duty, grants
list, key rotation, the read mounts.

## 8. Alignment check against ciris.ai (values, mdd, constitutional-mesh, philosophy)

Each row is a stated commitment, where the translation meets it, and what had to
change.

| Source | Commitment (their words) | Where it lands | Change made |
|---|---|---|---|
| values | "Consequential actions travel with signed documentation including: actor identity, authority verification, consent records, completed checks, unalterable records, and human stop-authority" | every card's confirm sheet names the signer; the Licensure read returns the standing block (which delegation, which authority); rows are append-only on the row plane | **unalterable records**: the standalone registry's manifest upsert (CIRISRegistry#144) is the one place this fails and is why the fold retires it |
| values | Respect for Autonomy: "Uphold informed agency" | a member joins by their own signed acceptance (CSD-121); a delegate sees the chain above them (CSD-124) | **added**: the subject of a licence sees every licence about them and can file a `reconsideration:{grounds}` appeal (CSD-122 §2; CC 4.5.5 "Subject standing"); the Constitution does not consent-gate licensure (CC 3.4.5 binds `capacity:*`), but a claim about a person they cannot see or contest fails this value |
| values | Justice: "Distribute benefits equitably. Detect and mitigate bias" | anyone may be an authority under their own key (CC 2.4.1.2.1); no roster of permitted authorities; the smallest affiliation is served by one field (CSD-120) | none |
| values | Fidelity & Transparency: "Clearly communicate uncertainty" | a single-source CIRIS licence shows confidence ≤ 0.5 (CSD-122 chip); error is never rendered as empty (every card §2 states) | none |
| values | "No principle grants license to violate another" | the model's refusals: a `license`-scoped delegate cannot widen scope in Beneficence's name; admission refuses, it does not weigh | none |
| constitutional-mesh | "Standing comes from being answerable, traceable through a live chain to an accountable human, not from wealth" | the root of every affiliation's chain is its founders' keys, which are human-held (CC 3.2, CC 3.4.7.3: a node has no agency and cannot found or hold a role); a delegation chain therefore always resolves to a person | **added** to §4 and CSD-124: the chain's root is a human; a node or agent is never the root of an authority tree |
| constitutional-mesh | "earned authority stays bounded, revocable, and open to appeal. It decays like everyone else's, seniority included. It never becomes permanent sovereignty" | bounded: attenuation; revocable: withdrawal cascade; appeal: `reconsideration` (`review` scope, CC 4.4.3.4.3) | a perpetual delegation edge is refused; every role edge carries a term of at most one year and is renewed by re-conferral (CSD-121, CSD-124; CC 4.4.3.2.8 C) |
| constitutional-mesh | "the admission quorums that matter count founders, not the crowd" | founding is the founders' quorum (CSD-120 via CSD-102); roles are conferred from that root | none |
| constitutional-mesh | "The CIRIS root is the shipped default, never the only option … every client chooses which roots to trust" | licensure is open-emitter; a reader honours a licence through its own Who-I-trust set (CC 4.4.3.8), in which the shipped root is one removable entry | **stated** in CSD-122/123: no card ranks authorities; the reader's trust does |
| philosophy | "The floor is the test, not the ceiling" | a five-person committee sets one field and has a complete charter (CSD-120) | none |
| philosophy | "The least of us. Not the least of them." / one Constitution for NHI | a member of an affiliation is a self, human or agent: an agent joins through its partnership (CC 4.4.3.4.3) and may hold a role; only the ROOT must be human | **added** to CSD-121 |
| philosophy | "Truth is composed, never declared … Multiple copies of one voice count as one voice" | the licence status set is composed, never a scalar; mutual recognition is a display fold over two independent rows, not a wire state; co-stewarded licences count the second voice only when it is a second key | none |
| philosophy | "Universal application of rules to CIRIS itself" | CIRIS L3C is an affiliation like any other; its licences are `licensure:ciris-canonical:v1`; it uses the same seven cards | **stated** in §1 |
| mdd | "Why are we building it, and does this choice serve that purpose?" | each CSD §1 names the principle it serves and the sentence that falsifies it; FSD-005 §2 states why the translation is owed | none |
| mdd | "Tests that the system refuses to cross ethical boundaries" | `formal/authority_tree/` checks refusals as properties (I4–I7), and every CSD §4 flow includes a refusal step | none |

## 9. References

**The Constitution sections that govern each translation.**

| Translation | CC |
|---|---|
| An organisation is an affiliation; the three operational kinds retired; `authority_id` is the `community_key_id`; roles are presets over scope sets; a delegated holder never stands in for the protocol | 3.3.9 |
| The affiliation record: `declared_cohort`, the config record, charter defaults (`role_term`, licence classes), the public face and `community:listing:v1`, succession and dissolution, who may license, the scopes | 4.4.3.2.8 "The record", "Charter defaults", "Public face", "Succession and dissolution", "Who may license", "The scopes" |
| The protocol floor: the assumption each tier carries; one member is enough; the floors bind power over members | 4.4.3.2.8 "The assumption each tier carries", "One member is enough", "The floors bind power over members, not the count of voices"; 4.5.12.5; 4.5.4; 4.5.13 |
| The charter's limbs: classification and retention, compartments and `archive_custody`, hierarchy and designated officials, lawful access and transparency, `dissolves_at` | 4.4.3.2.8 A, B, C, D, "Lifecycle" |
| Only delegation chains; `license` / `grant` convey the right to issue; attenuation; the enforced-admission rule | 2.4.1.2.1; 4.4.3.4.3; 4.5 |
| Depth cap 5 by default, configurable; cycle refusal | 4.1.1 |
| The term on a delegation edge; the 365-day ceiling | 2.4.1; 4.4.3.2.8 C |
| Who may be a licensing authority; the authority set under the `consensus_protocol`; authority judged at `asserted_at`; no public default; re-emission to named readers | 2.4.1.2.1 "Who may be a licensing authority, and what admits a licence"; 1.13.3.4 |
| A licensee's contest is `reconsideration:{grounds}`; a subject sees every licence about them | 2.4.1.1; 4.5.5 "Subject standing" |
| The reader's trust set; licensors of licensors; removal removes exactly what reached through the entry; the federation floor on removing the last root | 4.4.3.8 "Who I trust", items 4–5 |
| Membership is two consents | 3.1.3.2 |
| The organisation on the bond; `slash` and `quarantine` reach its keys | 3.2, canonical `cohort_subkind: infrastructure` |
| Per-recipient `key_grant`; a grant is never withdrawn | 3.3.2; 3.1.3 |
| The crypto tier and feed band affiliations share with communities | 4.4.3.2.1; 4.4.3.3 |
| The GenesisBundle carries no `license` edge | 3.2 T5; 5.3.4 |

**Register rows.** `CLM-authority-triad`, `CLM-issuance-scope`, `CLM-scope-attenuation`,
`CLM-delegation-depth-default`, `CLM-key-grant-rows`, `CLM-delegation-term`,
`CLM-licence-audience`, `CLM-canonical-founders-accord`, `CLM-affiliation-config-record`,
`CLM-affiliation-one-voice-countersign`, `CLM-affiliation-public-face`,
`CLM-affiliation-dissolution`; governance row `affiliation_member_action`. Models:
`formal/affiliation/Affiliation.lean`, `formal/trust_set/TrustSet.lean`.

**Substrate tickets cited as evidence of a door or as a `blocked_by`.** CIRISPersist#1142
(`trust_classes` / `trust_depth`), CIRISPersist#1143 (one-voice countersign, public face,
dissolution), CIRISPersist#752 (the custody index); CIRISServer#752 (the doors),
CIRISServer#663 (the chain read), CIRISServer#751 (walk cap and the durable scoped
door), CIRISServer#649 (the config record), CIRISServer#646 (register every kind);
CIRISEdge#895 (licence audience carriage), CIRISEdge#896 (delegation term carriage),
CIRISEdge#851 (depth default), CIRISEdge#808 (per-recipient `key_grant` carriage).

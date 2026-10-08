# FSD-005 — An organization is an affiliation: the Portal's concepts on CEG rc7

**Status:** DRAFT, 2026-10-08. Direction from the maintainer: *an org is an affiliation in
CEG; complete that translation; model the full state tree for grants/delegations so it
closes for the enumerable depths.*
**Reads with:** FSD-004 §4.3 (the PortalService disposition table this supersedes for
org data), `formal/authority_tree/` (the state tree), the CSD set under `FSD/CSD/`.
**Constitution:** 1.0-rc7 (`manifests/namespace_registry.json`
`796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464`, 168 families).

---

## 1. The one sentence

**The Portal's `Organization` is a community at `cohort_scope: affiliations`
(CC 4.4.3.2.8); its members, roles, partners, licences, grants and delegations are the
rows CEG already has for an affiliation, and nothing about an organization is a record
kind of its own.** Anyone can found one under their own key. There is no system
administrator who creates organizations, and no roster of permitted licensing
authorities. CIRIS L3C is one such affiliation and uses the same cards; its licences are
`licensure:{its community_key_id}` (ciris.ai/philosophy: universal application of the
rules to CIRIS itself).

## 2. Why the translation is owed

CC 3.3.9 carries three operational subject kinds — `organization`, `org_membership`,
`partner_record` — written when the Registry was a central service federating a
"trust/authz-minimal projection" of its Portal tables. Persist v48 built them
(`federation/operational.rs`: LWW + withdrawal-forward-only for the first two,
monotonic-quorum for the third). They are the Portal's tables with the PII removed.

CC 4.4.3.2.8 then defined the affiliation: the institutional cohort, gathered by
necessity, with a declared config record (archetype, classification, retention,
hierarchy, designated officials, lawful access) and the whole community machinery
(roster, DEK cascade, `consensus_protocol` admission, `membership:{stage}` rows). An
organization has every property an affiliation has, and the affiliation has the
governance an organization needs that the operational kinds never had.

Two records for one thing is the drift CC 3.1.7 R2 forbids. The maintainer's ruling
settles which survives: the affiliation.

## 3. The translation, concept by concept

| Portal / FSD-004 concept | On CEG rc7 | Signed by | Where it lives |
|---|---|---|---|
| **Organization** (`org_id`, name, type, parent, status) | an affiliation: a `community` record with `cohort_scope: affiliations`, `community_key_id` = the former `org_id`; `name`; `cohort_subkind` as today; `parent_org_id` → `hierarchy` nesting (CC 4.4.3.2.8 C: "may nest federated sub-affiliations"); `org_type` → `affiliation_archetype` + `membership_basis` | founders (CC 3.2 T6: the head is the signed roster record at a version) | community record + its config record |
| **Org user** (a person in the org) | a **member**: `membership:{stage}` rows (proposal → acceptance → widening) onto the roster; the person's fed-ID is the member, their devices follow by owner-binding | inviter + invitee (CC 3.1.3.2: nobody joins without their own signed acceptance) | roster record versions |
| **Org role** (`OrgAdmin` / `KeyManager` / `Operator` / `Viewer`) | a **`delegates_to` edge in the affiliation's `hierarchy`** (CC 4.4.3.2.8 C), from the affiliation's root (its founders' quorum, or a `sub_delegation` holder) to the member, carrying the scopes the role bundles (§4). The four names survive as **presets**, not as a wire enum | the delegator, within its own scope | `delegates_to` rows at the affiliation's scope |
| **Partner** (`partner_record`: licence type, capabilities, limits, status, revision) | the affiliation **as authority** issuing `licensure:{community_key_id}:v1` rows about the partner's key(s) with the status set, plus `partner_role:{role}:v1` for the tier, `duty:{kind}:v1` for obligations (supervision, limits), `multilateral_participation:*` for forum standing. Capabilities ride `scope` on a `delegates_to` where they are agency, and the licence where they are standing (§4) | the affiliation's quorum, or a `license`-scoped delegate | scores rows at federation scope |
| **Licence** (issue / suspend / revoke / expire / renew) | `licensure:{authority_id}:v1` with `authority_id` = the affiliation's `community_key_id` (CC 3.3.9: "an organisation names itself as an authority by `org_id`"; the id is now the community's). `suspended` is reversible (reinstate = `withdraws` on the suspended row); `revoked` is terminal; `lapsed` is the authority saying expiry happened; several statuses may be live | as above | scores rows |
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
  tier is `partner_role:{role}`; obligations are `duty:{kind}`.
- The `organization` / `org_membership` / `partner_record` subject kinds: **not**
  "transitional with a named exit". The steward rejected that shape for key grants on
  2026-10-05 (#143: "if it is moving to scores it needs to move"). They move on the
  release that adopts ruling A, with the Portal's data migrated in the same cut; until
  that release CC 3.3.9 stands as it is.

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

**Authority decays, and a perpetual edge is refused (maintainer ruling, 2026-10-08).**
Every role edge carries a term (`delegation_valid_until`) and is renewed by
re-conferral; an edge with no term, or a term past the ceiling, is refused at the door
(ciris.ai/constitutional-mesh: "It decays like everyone else's, seniority included. It
never becomes permanent sovereignty"). **The one-year ceiling is constitutional; the
term itself is a charter parameter** (CSD-120), so a small affiliation may choose
ninety days. This costs nothing structurally: the founders are the roster root, not an
edge, so when every admin's term lapses the quorum chosen in §7.1 still re-confers, and
the subtree below a lapsed edge lapses with it. **A lapsed `moderate` edge is a
moderator lapse under CC 4.5.4**: auto-promotion or fail-secure in the same step, never
an unmoderated window. CC 4.4.3.2.8 C makes the term optional; this is an ask on the
Constitution to make it required for delegation edges (§6).

Everything below the root is attenuated: an OrgAdmin created by sub-delegation holds a
subset of what their delegator held, never more, and a withdrawal at any link removes
every role beneath it (CC 4.5). A role is a leaf of the one tree, with two kinds of leaf
(a licence, a grant) hanging off the same tree. The default depth cap is 5 (CC 4.1.1,
"configurable"), and CC 4.4.3.2.8 C lets an affiliation declare a deeper cap for a deep
secretariat; this document takes the default and treats a deeper cap as a charter
declaration (CSD-120), never a silent widening.

## 5. The authority tree, and that it closes

`formal/authority_tree/authority_tree.py` models the triad as one state machine and
enumerates every reachable state of a small universe (measured 2026-10-08: 153,856
states with every action interleaved at two links, 2,218 delegation-only states at
three, the depth chain closing at five with 4,096 refusals past the cap; 0 violations;
27 s; in CI), checking nine invariants in each:
attenuation, the depth cap, revocation cascade, licence closure (holding a licence never
confers issuance), grant closure (holding a grant never confers re-granting), enforced
admission, the `sub_delegation` requirement, acyclicity, and history-independence of
the verdicts. A separate chain universe checks closure at the cap: the resolver confers
nothing past five links and a sixth link is refused at the door.

The answer to "should all three be delegable, or should the two ride the one": **the two
ride the one.** Only delegation chains. The right to issue a licence or a grant is
delegated; the licence and the grant are leaves.

## 6. What this asks of other repos

| Repo | Ask | Why |
|---|---|---|
| CIRISConstitution | Re-base CC 3.3.9 on CC 4.4.3.2.8: an organization is an affiliation; `authority_id` for an organisation is its `community_key_id`; retire the three operational subject kinds or mark them transitional; state the role presets as presets, not a wire enum | two records for one thing |
| CIRISConstitution (second ask) | Rule how a licensing affiliation's authority set reaches non-member readers (§7.1 blocker 2), and whether a licence needs the affiliation's quorum or one founder (§7.1 blocker 1) | both are rulings, not code |
| CIRISPersist | §7.2: the affiliation authority-set arm for licensure; the V089 CHECK fix; licensure gate/fold agreement with an `as_of` lens; cycle refusal at admission; chain-explain read; typed affiliation config + discriminator; `role_of`; licensure public read; the `grant` gate; rc7 `key_grant`; the 3.3.9 deprecation plan | storage + admission of the rows above |
| CIRISEdge | §7.3: a commons reach for registry families on a conferred node; subject-pull by non-subjects; consent-prefix coverage; depth default 5 | carriage + audience of the rows above |
| CIRISServer | §7.4: every write door (found an affiliation with its charter, roles as `delegates_to`, licences, grants, `license`/`grant` delegations, keys) and the public read tier a registry node serves; #646 (register every kind) | the cards |
| CIRISClient | the CSD set under `FSD/CSD/` here, to be adopted into `CIRISClient/FSD/CSD/` | the cards |

## 7. Audit results (2026-10-08, read-only, against each repo's `origin/main`)

Three audits were run in parallel, one per substrate repo, against the product decision
in §1 and the authority model in §5. Nothing was filed from them yet. The sizes are the
auditors' estimates.

### 7.1 The blockers, across repos

1. **An affiliation cannot be a licensing authority today (Persist, M).** The licensure
   admission predicate accepts only an attester whose key *is* `authority_id`, or a
   `license` chain walked out of that key (`admission.rs:3657-3677`). A
   `community_key_id` is keyless: it has no `federation_keys` row (V060), never signs,
   and emits no edges. So every licence issued under an affiliation is refused
   (`licensure_delegator_not_authority`) or admitted as testimony. The fix is an
   authority-set arm: `authority_id` names a community ⇒ the authority set is its
   active founders (or whatever its `consensus_protocol` admits), and the `license`
   walk roots there. Server's audit confirms from its side: a room's id comes from
   `ciris_edge::chat::new_room_community_key_id()` (`communities.rs:1485`), a random
   id, not a key. **Design decision needed:** is one founder enough to issue, or must
   the affiliation's quorum co-sign a licence? CC 3.3.9 says "by quorum".
   **Ruled (maintainer, 2026-10-08):** the full range of persist's quorum models is
   selectable when the affiliation is founded (`consensus_protocol`: founder-only,
   everyone, majority, `quorum:M/N`, …) and may be changed later by a decision of the
   active quorum on the active roster. Persist handles all of that. A licence is
   therefore an act of the affiliation under its current `consensus_protocol`, the same
   way a roster change is: one founder suffices only where the protocol says so. The
   persist arm is "authority set = whatever the affiliation's active protocol admits",
   and Server's door is the existing `/changes/{envelope,cosign,assemble}` pattern.
2. **Outsiders cannot verify an affiliation's licence (Persist + Edge, CC ruling).**
   Community records, rosters and the internal `delegates_to` hierarchy replicate only
   to member nodes (persist `replication_audience.rs:603-640`; edge `is_public_group`
   covers infrastructure/accord/WA groups only). A registry node holding a
   `licensure:{community_key_id}` row cannot see the quorum or chain behind it, so it
   can read the row as testimony but not fold it as the authority's licensure. Options:
   enrol the registry node as a member of every licensing affiliation (does not scale),
   or publish a licensing affiliation's authority head (founders + `consensus_protocol`
   + `license`-scoped edges) at federation scope, the way `infrastructure` does. The
   second conflicts with CC's "public = replicated on join"; it needs a ruling.
   **Ruled (maintainer, 2026-10-08; corrected by the Constitution session's review of
   the same day):** a licence is a claim about a person, so CC 1.13.3.4 applies and
   **there is no public default**: the scope of a licence is the authority's declared
   choice per licence class, the smallest that fits. A practice licence meant to be
   checked by strangers is public by its nature; many licences are not. Whatever the
   class's scope, the licence **and the authority head a verifier needs** travel
   together at that scope, re-stamped to named readers where the class is not public,
   never by widening the roster. The CC ask: the authority head of a licensing
   affiliation travels with its licences at whatever audience the licence has, and the
   affiliation's charter declares the scope per licence class.
3. **A registry node cannot serve rows to "any client" (Edge, M).** A peer is served a
   federation-scope row only if the serving node has authored a `consent:replication`
   grant naming it (`resolved_state.rs:76-110`); otherwise it gets first-contact reach,
   which carries only the server's own allegiance rows. Edge proposes a
   `Reach::Commons` for registry families on a conferred node, gated by Rooted and
   quarantine (sibling of CIRISEdge#837), plus subject-pull on the Attestation plane
   for readers other than the subject (analogue of #552, rate-limited per #844). Until
   then "is X licensed?" is answerable over replication only by X, and everyone else
   goes through Server's HTTP read.
4. **Probable bug: changing an affiliation fails on Postgres and SQLite (Persist, S).**
   `federation_group_versions.cohort` is `CHECK (cohort IN ('family','community'))`
   (V089) while `supersede_affiliations` inserts `'affiliations'` (`postgres.rs:9276`,
   `sqlite.rs:7913`). Only the memory backend is tested. Found by reading, not run.

### 7.2 Persist (v53.1.8 at `1e596708`)

| Area | Status | Finding |
|---|---|---|
| Affiliations | partial | An affiliation is a community record addressed by `Cohort::Affiliations`; quorum supersede exists. **No stored discriminator** says a record is an affiliation (`Community` has no cohort field; only the version history carries one). **No storage or typing** for the CC 4.4.3.2.8 config record: the only home is the opaque `policy_blob`. `cohort_subkind: public` / `admission: open` not honoured. |
| Roles | partial | `CommunityMember.role` is an open string; only `founder` is interpreted (`community_authority_set_for`). `delegation_purpose` knows only `responsible_for`. No "role of K in C" read; no mapping for the four presets. |
| Delegation plane | partial | The scoped walk (`scoped_delegation_reach_at`) does attenuation, `sub_delegation` + depth budget, retracted-edge skip (cascade), depth default 5, visited-set guard; `license` and `grant` use it. **Missing:** cycle-closing edge refused at admission (CC 4.1.1 MUST, nothing does it); write-time attenuation/sub_delegation checks (non-conformant edges are stored and confer nothing); **expiry in the license walk** (default lens admits expired and future edges, so term-bound officers never lapse); a chain-explain read; the outbound graph read traverses past withdrawn edges. |
| Licensure | partial | The 8-status fold exists (`licensure::status_set_for`, revoked absorbing, withdraws/recants/supersedes/expiry handled) and the door gate is wired. **Gaps:** blocker 1; the gate admits multi-hop chains but the fold requires a direct edge from the authority (`admission.rs:3627-3636`); the gate discards the `delegation_id` it requires (`:3735`); the fold checks the *current* graph, so a later withdrawal reclassifies history; no public Engine/FFI read and no "licences under authority A" index; no typed licence payload (nowhere for `partner_record`'s capability/tier/limit fields). |
| Grants | partial | `DELEGATION_SCOPE_GRANT` is declared and no gate reads it; a `grant`-scoped delegate is refused. `key_grant` is still the rc6 N-wrap set row; the rc7 per-recipient shape (CIRISPersist#989) is not on `main`. `consent:scope:*` admission is a humans-only rule with no asset-ownership check. |
| Retiring 3.3.9 | needs CC | `operational.rs` (7.5k lines), V071/V072, three `EnvelopeKind`s, the verify `operational_admit` dependency. CC 3.3.9 is still normative in rc7. Worth porting: `revision` anti-rollback, the ±5 min skew bound, payment-id rejection, the binding checks, and `partner_record`'s field set as the typed licence payload. |
| Replication | partial | Licence rows at `federation` reach everyone; affiliation-scoped rows reach members only; blocker 2. A licence placed at `affiliations` would reach only members, and nothing pins the scope. |

Persist's proposed issues (12): the V089 CHECK fix; the affiliation authority-set arm;
licensure gate/fold agreement (depth, `delegation_id`, `as_of`); cycle refusal at
admission; chain-explain read + cascade-correct outbound graph; typed affiliation config
+ discriminator; `role_of` read + role vocabulary; licensure public read + list-by-
authority + typed payload; the `grant` admission gate; rc7 per-recipient `key_grant`;
the non-member authority-visibility design; the 3.3.9 deprecation plan.

### 7.3 Edge (v40.1.0 at `7057fc2`, pins persist v53.1.4)

| Area | Status | Finding |
|---|---|---|
| Carriers | mostly exists | 19 `EnvelopeKind`s, all served. `delegates_to`, `licensure:*`, `duty:*`, `membership:*`, `consent:scope:*` ride the Attestation plane; affiliations ride Community. `licensure:` classifies `Unknown` → no family gates → Cohort projection, advertised. `key_grant` per-recipient: CIRISEdge#808. The three operational kinds are served **to every peer unfiltered**, which under org = affiliation would publish role bindings: freeze as migration-only. |
| Host side | gap | CIRISServer#646: the host registers 6 of the kinds, dropping Family, membership revocations/widenings, Revocation, IdentityOccurrenceRevocation and the operational kinds. |
| Audience | the main gap | Blocker 3. Consent prefixes do **not** filter per recipient; they decide promotion of local-tier rows to federation tier. If Server emits at local tier, the prefixes must cover `licensure:`, `duty:`, `partner_role:`, `revocation:`, `consent:scope:`, `key_grant:`, `membership:` and the hierarchy's `delegates_to` dimension; edge's defaults cover `capacity: chat: ownership: self:delegates_to: trace:` only. A dimension-less `delegates_to` can never be promoted and must be emitted at `tier: federation`. |
| Rooted | exists | The per-peer-pair floor (#659) compares the two nodes' owners, never the row's author, so thousands of affiliations add nothing. A read-only node must still be claimed, accept a common root, and publish its key/occurrence/transport rows. |
| Blobs | nothing new | Affiliation documents are CommunityDek blobs held by members only; what outsiders must read goes at federation scope, under #785 and #843. Per-class transparency promotion (CC 4.4.3.2.8 A) is not in edge. |
| Delegation evaluation | none | Edge knows none of `license` / `grant` / `sub_delegation`; persist owns depth, attenuation and cascade. The one edge-side walk (`message_io`) is off by default with depth 4, not 5. |

Edge's proposed issues (6): `Reach::Commons` for registry families on a conferred node
(M); subject-pull of registry rows by non-subjects (M); licensing affiliations publish
their authority head at federation scope (S, CC-led); the registry families in the
consent-prefix lists, and refuse dimension-less `delegates_to` at local tier (S);
default depth 4 → 5 (S); release parked licence rows when their delegation lands (S).

### 7.4 Server (0.5.224 at `origin/main`; the #442 branch for the fold)

Server has **no route at all** for licensure, partners, organizations, key rotation,
escrow or an audit read. Affiliations exist only as a tier label on a room. Delegation
exists only as the device-grant flow. Persist's licensure fold, delegation reads,
key-grant listing, escrow table and operational doors have **zero callers** in Server.

| Area | Status | Finding |
|---|---|---|
| Affiliations | partial | `POST /v1/communities {tier: affiliations}` is accepted and writes only `policy_blob = {cohort_scope: affiliations}` (`communities.rs:1409-1497`); messages are still placed at the community tier. Members are added by key (must already be a contact) with free-string roles; only `founder` has meaning; roster roles are not `delegates_to`. The roster read is members-only (non-member → 404). No config record (#649), no cohort-scoped roster read (#662), no public listing of `listed: public` members. |
| Delegations | partial | `GET /v1/auth/device/grants` is outbound-only, skips `infra:*`, has no depth, parent, `valid_until` or inbound read (#663); persist's `Engine::delegations_to` is unused. `POST /v1/auth/device/delegate` accepts any scope string but **keeps only the first scope** (`device_grant.rs:527`), binds no `authority_id`/`community_id`, and sets `valid_until` to the device-code TTL, **600 s**, so every edge from this flow dies in ten minutes. Server's walk cap is 4, not 5. An admin act needs a `delegation_id` no route returns (#676). Chains can only be founder-rooted today. |
| Licensure | missing | Only consumer composition exists (`compose_policy.rs` `licensure_cap`, the CC 3.4.9 ≤ 0.5 rule). No writer, no reader; nothing issues `duty:{kind}` on a licence; `partner_role:{role}` has no writer or reader. |
| Grants | missing for clients | `key_grant` is emitted only internally; `consent:scope:*` appears only in `safety/infohazard.rs`; `POST /v1/auth/consent` writes a self-tier row with no `scope` member (not a CC consent grant) and needs client-side crypto; no list of peering grants (#680), no consent-grant withdraw (#657). |
| Keys | mostly missing | Device-occurrence add/revoke/label and portable associate exist. No identity-key rotation, no key history, no KeyManager binding, no public key reads, and `registry_key_escrows` is referenced nowhere. |
| Public read tier | partial (branch) | On #442, `compose_registry` gates and logs but mounts nothing; registry-core's fold router is mounted outside the gate as tier-P. The default consent prefixes (`capacity: chat: ownership: self:delegates_to: trace:`) cover none of the registry families, so a conferred node never receives other authorities' licences and cannot answer for them. |

Server's proposed issues (11): a public listed roster (S, waits on CIRISPersist#912);
durable scoped delegation issue/withdraw with an attenuation precheck, fixing the
first-scope truncation and the depth constant (M); affiliation as delegation root (L,
persist); licensure issue/suspend/revoke (M single-key, L quorum); licensure read per
subject per authority (S–M); partner_role and duty writer/reader (S); grants list and
issue (M); identity-key rotation and key history (M); escrow (M, product decision
first); a registry-slice replication policy (L, edge); `compose_registry` mounting the
reads (S, closes #669). Buildable in Server alone, now: the delegation door and reads,
single-key licensure write and read, partner/duty, grants list, key rotation, the
read mounts.

## 8. Alignment check against ciris.ai (values, mdd, constitutional-mesh, philosophy)

Read on 2026-10-08. Each row is a stated commitment, where the translation meets
it, and what had to change.

| Source | Commitment (their words) | Where it lands | Change made |
|---|---|---|---|
| values | "Consequential actions travel with signed documentation including: actor identity, authority verification, consent records, completed checks, unalterable records, and human stop-authority" | every card's confirm sheet names the signer; the Licensure read returns the standing block (which delegation, which authority); rows are append-only on the row plane | **unalterable records**: the standalone registry's manifest upsert (CIRISRegistry#144) is the one place this fails today and is why the fold retires it |
| values | Respect for Autonomy: "Uphold informed agency" | a member joins by their own signed acceptance (CSD-121); a delegate sees the chain above them (CSD-124) | **added**: the subject of a licence sees every licence about them and can file a `reconsideration:{grounds}` appeal (CSD-122 §2); the Constitution does not consent-gate licensure (CC 3.4.5 binds `capacity:*`), but a claim about a person they cannot see or contest fails this value |
| values | Justice: "Distribute benefits equitably. Detect and mitigate bias" | anyone may be an authority under their own key (CC 2.4.1.2.1); no roster of permitted authorities; the smallest affiliation is served by one field (CSD-120) | none |
| values | Fidelity & Transparency: "Clearly communicate uncertainty" | a single-source CIRIS licence shows confidence ≤ 0.5 (CSD-122 chip); error is never rendered as empty (every card §2 states) | none |
| values | "No principle grants license to violate another" | the model's refusals: a `license`-scoped delegate cannot widen scope in Beneficence's name; admission refuses, it does not weigh | none |
| constitutional-mesh | "Standing comes from being answerable, traceable through a live chain to an accountable human, not from wealth" | the root of every affiliation's chain is its founders' keys, which are human-held (CC 3.2, CC 3.4.7.3: a node has no agency and cannot found or hold a role); a delegation chain therefore always resolves to a person | **added** to §4 and CSD-124: the chain's root is a human; a node or agent is never the root of an authority tree |
| constitutional-mesh | "earned authority stays bounded, revocable, and open to appeal. It decays like everyone else's, seniority included. It never becomes permanent sovereignty" | bounded: attenuation; revocable: withdrawal cascade; appeal: `reconsideration` (`review` scope, CC 4.4.3.4.3) | **ruled**: a perpetual delegation edge is refused; every role edge carries a term of at most one year and is renewed by re-conferral (CSD-121, CSD-124). CC makes the term optional; the ask to make it required for delegation edges goes to the Constitution |
| constitutional-mesh | "the admission quorums that matter count founders, not the crowd" | founding is the founders' quorum (CSD-120 via CSD-102); roles are conferred from that root | none |
| constitutional-mesh | "The CIRIS root is the shipped default, never the only option … every client chooses which roots to trust" | licensure is open-emitter; a reader weights an authority by consumer policy (CC 4.4.4) | **stated** in CSD-122/123: no card ranks authorities; the reader's trust does |
| philosophy | "The floor is the test, not the ceiling" | a five-person committee sets one field and has a complete charter (CSD-120) | none |
| philosophy | "The least of us. Not the least of them." / one Constitution for NHI | a member of an affiliation is a self, human or agent: an agent joins through its partnership (CC 4.4.3.4.3) and may hold a role; only the ROOT must be human | **added** to CSD-121 |
| philosophy | "Truth is composed, never declared … Multiple copies of one voice count as one voice" | the licence status set is composed, never a scalar; mutual recognition is a display fold over two independent rows, not a wire state; co-stewarded licences count the second voice only when it is a second key | none |
| philosophy | "Universal application of rules to CIRIS itself" | CIRIS L3C is an affiliation like any other; its licences are `licensure:{its community_key_id}`; it uses the same seven cards | **stated** in §1 |
| mdd | "Why are we building it, and does this choice serve that purpose?" | each CSD §1 names the principle it serves and the sentence that falsifies it; FSD-005 §2 states why the translation is owed | none |
| mdd | "Tests that the system refuses to cross ethical boundaries" | `formal/authority_tree/` checks refusals as properties (I4–I7), and every CSD §4 flow includes a refusal step | none |

## 9. How the changes enter: the filing plan (approved and filed 2026-10-08)

The Constitution's own process (STEWARDSHIP.md; README "Versioning"; `EVIDENCE.md`;
`tools/check_claims.py`): the steward rules and lands text; every new MUST gets a
`claims.tsv` row; a row is `staged` on an **open** ticket in the implementing repo and
becomes `established` when that repo publishes a matching `evidence/cc_impl.tsv` row
that CC pins and re-reads at the same decimal. So: **a Constitution ask first for
anything the text does not yet say; a substrate issue citing the section for anything
it already says MUST; nothing re-asked that rc7 already rules.** Coverage was checked
ask by ask against rc7's text and register.

### 9.1 Already ruled — cite, do not ask

| Ask | Where | Register |
|---|---|---|
| Only delegation chains; `license` / `grant` convey the right to issue; attenuation | CC 2.4.1.2.1, 4.4.3.4.3 (#100, rc5) | `CLM-authority-triad`, `CLM-issuance-scope`, `CLM-scope-attenuation` — established on persist |
| Depth cap 5 by default, configurable | CC 4.1.1 | `CLM-delegation-depth-default` — established |
| The quorum model is chosen at founding and amended under its own rule | CC 3.3.4, 4.4.3.4.2 | — |
| Per-recipient `key_grant` | CC 3.3.2 (#143) | `CLM-key-grant-rows` — staged on CIRISPersist#989; CIRISEdge#808 |

### 9.2 Needs a Constitution ruling — four asks on CIRISConstitution, filed first (#161, #162, #163, #164)

| Draft | Rules | Covers FSD-005 |
|---|---|---|
| **A** = CIRISConstitution#161 | An organization is an affiliation; `authority_id` is the `community_key_id`; the three CC 3.3.9 kinds **move on the release that adopts A** (no "transitional with a named exit": #143 rejected that shape), their anti-rollback, skew bound, payment-id rejection and field set ported; roles are non-normative presets over scope sets, the `role:` enum struck. Sub-answers as reviewed: admin scopes are open vocabulary with a closed canonical set; a delegate never stands in for the protocol on a roster change; a keyless affiliation's key manager holds the registration binding under the quorum's cosignature. | §1, §3, §4; CIRISRegistry#112's counterpart under #115 |
| **B** = CIRISConstitution#162 | A community-named authority's set is whatever its current `consensus_protocol` admits; a licence is a cosigned row whose protocol source is the authority named in the dimension; authority is judged **at issuance**: the row's signed `asserted_at` under the cosigned-row rule, never receipt, because this is a shared verdict every node must compute identically (unlike the halt fuse), so a later withdrawal stops issuance without reclassifying history; **no public default** (CC 1.13.3.4): the scope is the authority's declared choice per licence class, and the authority head travels with the licence at that scope, re-stamped to named readers where the class is not public. Asks whether re-stamping is re-emission or carriage, and which member names the readers. | §7.1 blockers 1–2; the two maintainer rulings |
| **C** = CIRISConstitution#163 | A `delegates_to` carrying `sub_delegation`, `license`, `grant`, `moderate`, `takedown`, `review` or `slash`, or sitting in an affiliation's `hierarchy`, MUST carry `delegation_valid_until`; the **one-year ceiling is constitutional, the term a charter parameter**; renewed only by fresh conferral; a perpetual edge refused at admission (token to be named); a lapsed `moderate` edge is a CC 4.5.4 moderator lapse (auto-promote or fail-secure in the same step). Owner-binding out of scope. Precedents: the 30-day cap on a membership proposal, the teen pre-authorisation ceiling. | §4 ruling B; closes the remainder of CIRISRegistry#128 |
| **D** = CIRISConstitution#164 | A licensee's contest path is `reconsideration:{grounds}`, never withdrawal: extend the CC 2.4.1.1 carve-out to `licensure:*` and `revocation:*` (a single-signer licence is otherwise withdrawable by its subject under rule 2); the duty-holders for a reconsideration against `licensure:{A}` are A's authority set plus any `review`-scoped chain from it; and one sentence that a licence subject can see every licence about them (a read over rows they are party to; the sentence is what makes the executive summary's redress promise checkable). | §8 Respect for Autonomy |

### 9.3 Substrate-only — the text already says MUST; file on the implementing repo citing the §

| Repo | Issue | Cites | Register action |
|---|---|---|---|
| CIRISPersist#1031 | Refuse the cycle-closing `delegates_to` at admission | CC 4.1.1 "MUST detect cycles … and reject the cycle-closing emission" | the Constitution session re-stages `CLM-anti-pattern-delegation` here on receipt of the issue number (it sits on CIRISServer#536, a manifest ticket) |
| CIRISPersist#1032 | The `license`/`grant` walk honours `delegation_valid_until` and `as_of` | CC 2.1 `valid_until`; 4.5.5 "live chain" | — |
| CIRISPersist#1033 | The positive `grant` arm: a `grant`-scoped delegate may issue; `consent:scope:*` ownership check | CC 2.4.1.2.1, 4.4.3.4.3 | note that `CLM-issuance-scope` overstates what is built |
| CIRISPersist#1034 | Typed CC 4.4.3.2.8 config record + affiliation discriminator at founding + the V089 `CHECK` fix | CC 4.4.3.2.8 | ask CC for a `CLM-affiliation-config-record` row staged here; link CIRISServer#649 |
| CIRISPersist#1035 | Licensure gate/fold agreement (chain depth, the discarded `delegation_id`) and a public read + list-by-authority | CC 2.4.1.2.1 "what the fold keys on" | — |
| CIRISEdge#851 | Default delegation depth 4 → 5 | CC 4.1.1 | — |
| CIRISServer#751 | Walk cap 4 → 5; the durable scoped delegation door (one scope kept, 600 s term today) | CC 4.1.1, 4.4.3.4.3 | — |

### 9.4 Landed and adopted

#161–#164 landed at CIRISConstitution `c3a0a13` on `rc8` with the steward's go
(2026-10-08). Rulings as landed: `licensure:{community_key_id}` is a cosigned row with the
authority named in the dimension as the **third protocol source**; authority judged at the
signed `asserted_at` (`as_of` lens), withdrawal stops future issuance only; no public
default, scope per licence class in the charter; **re-emission to named readers is new
cosigned rows with the readers in `subject_key_ids`**; the refusal token for a termless
edge is **`delegation_term_required`**; a lapsed `moderate` edge is CC 4.5.4. Adopter
tickets, on which the CC rows are staged: **CIRISPersist#1036** (authority-set arm, `as_of`
lens, audience), **CIRISServer#752** (the doors), **CIRISEdge#852** (carriage of the
authority head). CIRISRegistry#112 and #128 close on #161 and #163.

### 9.5 Housekeeping

CIRISRegistry#142: the registry is in neither `evidence_pins.tsv` nor the checker's
manifest table; the row graduates on Server's manifest after the fold, as #142 itself
offers. CIRISConstitution#115: the Constitution session is doing the intake of the
sixteen `fold:move-to-constitution` issues; #112 is held a day for draft A.

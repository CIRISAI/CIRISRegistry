# FSD-004 — The Post-Fold Registry Surface

**Status:** DRAFT for decision (2026-09-26, revised for CC 1.0-rc5). No NEW route or
capability name in this document is on the wire yet; §6 lists the decisions it needs
first. What HAS landed is §5's shape for the routes already marked KEEP:
`ciris_registry_core::fold::router` (a build without default features carries no sqlx,
tonic or persist-postgres).

**Constitution pin:** CC 1.0-rc5 at `CIRISConstitution@44ae7b2` (the `rc5` branch tip
of 2026-09-19, the same commit CIRISClient pins in `client/ceg/README.md`). Rc5's
CC 2.4.1.2.1 (#100, with the CIRISPersist#814 ruling) is what §1 rule 4 and §4.5 apply.

**Scope:** every route and RPC `ciris-registry` serves today, and what each one
becomes once `ciris-registry-core` folds into CIRISServer as the registry slice
(`compose_registry`, CIRISServer `src/compose.rs`). For each: the post-fold path, the
substrate call behind it, and **who may call it** by caller tier (§2) and by node
posture (§3).

**Why this exists:** no document in either repo specifies it. FSD-002 §8 maps the gRPC
methods to backends but assumes a singleton host with HS256 JWTs. CIRISServer's
`FSD/REGISTRY_FOLD_DERISK.md` §4 item 7 leaves the registry-specific surface as one
line. CIRISClient already calls a route (`GET /v1/registry/lookup`) that matches
nothing in either repo (CIRISClient `FSD/AGENTS_AND_NODE_TIER.md`).

**Format precedent:** CIRISServer `src/auth/ABSORPTION.md`. That is the same table,
built for the agent's auth fold: route → substrate call → fabric home → status →
conflict.

**Companions:** FSD-002 §8–§9 (the pre-fold backend map), CIRISServer
`FSD/REGISTRY_FOLD_DERISK.md`, `FSD/TRUST_ROOT_CAPABILITY_GATE.md`,
`FSD/NAMING_THE_TRUST_ROOT.md`, `MISSION.md` §1.5, CC 2.4.1.2.1 / 3.3.9 / 3.4.9 (rc5),
CIRISClient `CSD.md` (CSD/3, the form each Portal surface becomes). Issues: #41 (handler cutover),
#58 (Spock removal), #62 (fold epic), #65/#66 (key-registration routes), #133
(registry becomes a conferred canonical server), CIRISServer#442 (registry-slice role
gate), CIRISServer#499 (capability declaration).

---

## 1. The four rules this surface is derived from

These are not new policy. Each is already ratified or shipped somewhere; this FSD
applies them to the registry's surface.

1. **A node holds `infra:*` and nothing else.** CC 4.4.3.4.3 conformance, as
   CIRISServer `FSD/TRUST_ROOT_CAPABILITY_GATE.md` puts it: "judgment roles (steward,
   moderator, founder-authority) … are never bestowable on a pure node … the node
   holds your standing; you (or your agent) wield the judgment."
   **Consequence:** every registry *decision* (register an agent, grant a license,
   revoke, halt) is a signature by a human-held key. The node admits, stores and
   serves it. No registry route lets a node, including a canonical one, decide on its
   own key's authority. **Nor does a node vote** (ruling of 2026-09-22 on
   CIRISServer#537): infrastructure holds no agency, so the `ciris-canonical`
   community is a roster the accord admits installs into, not a body that votes.
   Its founders are accord-conferred, human-rooted steward keys; its admission
   quorum is the accord's; a canonical node's *owner* configures it (CC 3.4.5
   `config:*` is self-or-owner). A node gains neither a vote nor a verdict.
2. **Authority is conferred, not self-published (#133, settled).** Post-fold the
   registry is an accord-scrub-conferred canonical server. It does not publish its own
   trust root. `REGISTRY_ADMIN_TOKEN`, the SYSTEM_ADMIN JWT god-mode and
   `trusted_primitive_keys` are all forms of "the registry operator says so". Each
   is replaced by a conferred key: the CI key via `/v1/accord/ci-key/*`, canonical
   servers via `/v1/accord/canonical/*`, and holder quorum via the accord.
3. **Signed objects authenticate themselves; the route does not.** A read route
   serving a signed CEG envelope needs no caller auth, because the consumer
   re-verifies the signature. This is already the shape of `/v1/steward-key`
   (#133: "signing the wrapper would prove only that the relay said so") and of
   the #138 provenance walk. What a route cannot make self-authenticating is
   **absence**: "no record" is only as good as the serving node's replica. That is
   why §3 separates serving from answering negatively.
4. **Licensure, grant and delegation are three objects, and only delegation chains
   (CC 2.4.1.2.1, rc5).** A *licence* is standing to practise, given by an
   **authority**, carried as a `licensure:{authority_id}` score; it ends when the
   authority suspends or revokes it. A *grant* is access to one asset, given by the
   asset's **owner**, carried as `subject_kind: key_grant` or `consent:scope:*`; it
   ends by rotation, expiry or exhaustion. A *delegation* is agency to act for a
   **principal**, carried as `attestation_type: delegates_to`; it attenuates, is
   depth-capped and is withdrawn per link. Holding a licence confers no power to issue
   one; receiving a grant confers no power to re-grant. Only `delegates_to` gets a
   graph walk. **Consequence:** the registry's `LicenseType` / `partner_record` path
   folded three of these into one "license", and the pre-fold Community tier sold
   standing (licensure) as if it were access (a grant) from a single central
   authority. Post-fold each leg has its own route, its own signer and its own
   revocation path (§4.5). Anyone may be a licensing authority under their own key;
   the registry is one `authority_id` among any, co-stewarding only the CIRIS-issued
   licence with CIRISVerify (CC 3.4.9).

## 2. Caller tiers

These are the post-fold tiers. Each maps to a mechanism CIRISServer already ships;
none is invented here. A route declares its tier the way the server does: a
router-level layer for loopback, and an in-handler check for the others.

| Tier | Who | Mechanism (CIRISServer) | Replaces (registry today) |
|---|---|---|---|
| **P — public** | anyone | open route, rate-limited; the payload is self-authenticating | public HTTP group; gRPC `RegistryService` |
| **S — signed caller** | any key registered in the federation directory | federation-signed `x-ciris-*` request headers (`auth::verify`); "signature is the gate" | witness cosign signature; signed-body POSTs |
| **M — org member** | a user key holding an `org_membership` role in the target org | an S-tier signature, plus the role read from persist's `org_membership` LWW state (§5.6.8.13) | HS256 JWT + `authorize_org_access(OrgRole)` |
| **O — owner** | the responsible party bound to *this* node | `require_owner_bound` / `require_owner_session` (`auth/gate.rs`, `federation_peers.rs`) | nothing (the registry has no node-owner concept) |
| **L — loopback** | an operator on the node's host | `require_loopback` layer (`auth/loopback.rs`) | nothing |
| **A — licensing authority** | the key that *is* an `authority_id`, or a key on a live `delegates_to` chain bearing the `license` scope that resolves to it (the `license` scope in CC 4.4.3.4.3's emission-authority table). An org-named authority resolves through its `OrgAdmin` / `KeyManager` members, and its `authority_id` is the **org UUID**, never the registration number (CC 3.3.9, #139) | persist's delegation resolver (`federation/admission.rs`) with the one refusal rc5 adds, `licensure_delegator_not_authority`; the emitter must resolve to the authority for the row to enter that authority's fold | `RegisterPartner` / `RevokeEntity(license)` under SYSTEM_ADMIN |
| **B — blessed key** | a key whose record carries an accord-conferred `infra:attest` (or `infra:serve`); `infra:attest` MAY be attenuated to one family, `infra:attest:licensure:{authority_id}` (CC 4.4.3.4.3, rc5) | `mesh_genesis::carries_scope` over the scrub-signed `registration_envelope.roles`; the #138 walk does exactly this. **Sub-scope matching is directional**: a parent token satisfies a check for its child, a child never satisfies its parent, and an unknown caveat fails closed. A `starts_with("infra:attest")` test is a defect | `REGISTRY_ADMIN_TOKEN` + `trusted_primitive_keys` |
| **Q — accord quorum** | ≥2 of 3 accord holders (A1/B1/C1), hardware-held | the propose/cosign ceremonies (`accord_provision.rs`), `accord/halt` | SYSTEM_ADMIN JWT (`RegistryAdminService`) |

**Retired outright, with no post-fold equivalent:**
- SYSTEM_ADMIN god-mode;
- the static `REGISTRY_ADMIN_TOKEN` bearer;
- HS256 JWTs for registry authority;
- the dead `ROLE_SYSTEM_AUDITOR` / `ROLE_WISE_AUTHORITY` constants.

`ROLE_HUMANITY_ACCORD=4`, already documented as "never granted via JWT alone", becomes
tier Q. Q is walled off from tiers A and M on purpose: CC 4.2.1 "accord keys cannot sign
grants or licenses". Nothing in §4 lets a Q signature stand in for an A one.

## 3. Node postures — who serves what

"Blessed" is used exactly as CIRISServer and CIRISClient use it: **the node's key
record carries an `infra:*` scope conferred by accord co-scrub** (`NAMING_THE_TRUST_ROOT.md`
`ConferralPlane::AccordCoScrub`, `mesh_genesis::carries_infra_serve`). A node is
unblessed otherwise, whether or not it is owned (`auth/gate.rs`: "TRUST ≠ JOIN. SERVE ≠
OWN.").

| Posture (`AgentMode` + conferral) | Registry routes it serves | Negative answers ("not found", "not revoked") | Admits registry writes |
|---|---|---|---|
| **client** | none; it consumes | — | no |
| **proxy** | none of its own; relays per the closed allow-list (`RNS_CONTROL_RELAY.md` §"No open HTTP proxy") | — | no |
| **server, unblessed** (owned or not) | P-tier reads of signed objects it has replicated | **no**: 404 means "this node holds no copy", and it must say so (§4.1) | stores and forwards envelopes it receives; never the admitting node |
| **server, blessed `infra:serve`** | all P-tier reads | **yes**: it is the canonical replica | yes, as one admitting node; admission still requires the envelope's own human signatures |
| **server, blessed `infra:attest`** | same as above | same | additionally *emits* `provenance:*` Contributions (the CI key is this posture, §4.3) |

A key point for CIRISClient: `found: false` is only actionable from a blessed node.
The client already keeps "answered no" apart from "could not ask". This adds a third
case, "answered but not authoritative", which the response must carry (§4.1).

Licensure adds a fourth axis, **per authority**. "Not licensed under A" is the fold of
`licensure:{A}` rows whose emitter resolves to A (rule 4). A row under `licensure:{A}`
signed by anyone else is admitted as *testimony* and stays out of the fold, so a
stranger's `revoked` binds nobody. A negative licensure answer therefore names the
authority it is about, and a node's replica of A's rows is what makes it authoritative
for A, not the node's own blessing.

## 4. The route map

Dispositions:
- **KEEP**: same path, now served by the registry slice's router.
- **SERVER**: CIRISServer already serves it; the registry's copy is dropped.
- **REPLACE**: a CEG envelope or ceremony supersedes it.
- **DROP**: no post-fold role.
- **NEW**: a route that does not exist today.
- **OPEN**: needs a §6 decision.

### 4.1 Public reads (tier P)

| Today | Post-fold | Tier | Serves | Substrate call | Disposition |
|---|---|---|---|---|---|
| `GET /health` `/ready` `/live` `/metrics` | server's own health + `/v1/system/verify-status` | P | all | — | SERVER |
| `GET /v1/identity` | server `compose.rs` (six-key `local_identity_aggregate`) | P | all | `Engine::local_identity_aggregate` | SERVER |
| `GET /v1/steward-key`, `GET /v1/trust-root/bundle` | same paths | P | all | baked `GenesisBundle` + `trust_root_valid` | KEEP. The server has no GET on `/v1/trust-root/bundle`, and its `/v1/trust-root/{id}` is a loopback DELETE, so the two coexist. Target: the server's `BundleBroadcast`, same schema |
| `GET /v1/accord-holders`, `/v1/accord/holders` | server `accord.rs` `/v1/accord-holders` | P | all | accord roster | SERVER. The registry's copy is a static `provisioned:false` placeholder and must not survive |
| gRPC `LookupAgent`, `BatchLookupAgents`; CIRISClient's `GET /v1/registry/lookup?agent_hash=` | `GET /v1/registry/lookup` | P | blessed `infra:serve` answers authoritatively; unblessed returns found records only (see note) | `lookup_keys_for_identity` (identity_type `agent`) + `list_signed_revocations_since` | NEW (client-expected). Response carries `authoritative: bool` |
| gRPC `LookupPartner`; `GET /v1/partner/{key_id}` | `GET /v1/partner/{key_id}` | P | as above | `list_partner_records_for` (monotonic-quorum merge) for the org's recognition; the licence itself comes from the row below | KEEP path, backend → persist. The `license_type` / `capabilities_*` fields stop being the licence of record (§4.5) |
| (no equivalent; `LookupPartner.license_status` was a scalar) | `GET /v1/licensure/{key_id}?authority={authority_id}` | P | blessed answers authoritatively **for the authorities it replicates**; others return rows only | `list_attestations_for` filtered to `licensure:{authority_id}`, folded per rule 4: emitter resolves to the authority; `withdraws` forward-only; the **status set** (`issued` · `probation` · `restricted` · `suspended` · `revoked` · `lapsed` · `surrendered` · `reduced`) | NEW. Returns a set, never a scalar: `suspended` is reversible, `revoked` is terminal, and both may be live at once. Omitting `authority` lists every authority with rows for the key, each folded separately |
| gRPC `VerifyDeployment` | `GET /v1/registry/verify-deployment` | P | as above | agent read + licensure fold + `key_grant` reads, capability intersection | OPEN (§6.3). The intersection's licence leg reads the licensure fold, not `partner_record.license_type` |
| gRPC `GetRevocationList`; `GET /v1/revocation/{target_id}` | `GET /v1/revocation/{target_id}`, `GET /v1/revocation?since=` | P | as above | `list_signed_revocations_since` | KEEP path, backend → `federation_revocations` |
| gRPC `GetPublicKeys`; `GET /v1/verify/key/{fingerprint}` | `GET /v1/verify/key/{fingerprint}` | P | all | `lookup_public_key` | KEEP path, backend → `federation_keys` |
| `GET /v1/rotation-history` | `GET /v1/keys/{key_id}/history` | P | all | `list_key_registration_history` (persist V148) | REPLACE. Registry steward rotation is retired; per-key history is persist's |
| `GET /v1/agent_files/{kind}` | same | P | all | `list_attestations_for` | KEEP (already persist-only) |
| `GET /v1/builds/{version}`, `/v1/builds/hash/{h}`; `GET /v1/verify/{binary,function,build}-manifest*` | same paths | P | all | manifest **bytes**: OPEN (§6.2). Provenance: #138 walk (`list_attestations_for`, blessed `infra:attest`) | KEEP paths; storage OPEN |
| gRPC `GetBuildAttestation` | folded into `/v1/builds/*` provenance | P | all | as above | REPLACE |
| gRPC `GetEmergencyStatus` | server `GET /v1/accord/halt-status` | P | all | accord halt state | SERVER |
| gRPC `GetOfflinePackage`, `GetOfflineDelta` | OPEN | P | blessed | a `list_signed_*_since` bundle | OPEN (§6.4): the CEG anti-entropy plane may make it redundant |
| `GET /v1/transparency/witnesses`, `/v1/transparency/sth/{n}/witnesses` | same paths | P | all | `federation_keys` identity_type `witness` + `transparency_log:cosigned:{n}` attestations | KEEP paths. Blocked on CIRISPersist#102 (witness vocabulary); until then the per-region PG tables stay |
| gRPC `HealthCheck`, `GetCapabilities`, `GetMetrics` | server `/v1/federation/conformance` capabilities | P | all | — | SERVER |

### 4.2 Registry decisions (tier Q, A or M, human-signed)

Rule 1 in practice: each of these becomes a **signed CEG envelope** authored by a human
key and **admitted** by blessed nodes. The post-fold transport for the envelope is the
anti-entropy plane (#58), not an HTTP write. Where an HTTP door exists, it is an
S-tier "submit this signed envelope" route; the route's own auth proves nothing.

| Today | Post-fold | Tier (of the envelope's signer) | Substrate | Disposition |
|---|---|---|---|---|
| `RegisterAgent`, `BatchRegisterAgents` | `SignedKeyRecord` identity_type `agent` | Q; the delegation depth is OPEN (§6.5) | `put_public_key` | REPLACE |
| `RegisterPartner`, `UpgradeToPartner`, `CreateLicenseeOrganization` — the **recognition** half | `partner_record` envelope (the org is a recognised partner; its steward quorum) | Q-style M-of-N steward quorum over identical JCS bytes (CC 3.3.9), monotonic on `revision` | `put_partner_record` | REPLACE. This is the org's standing as a partner, not a licence; #139: any authority the org names itself as uses its org UUID |
| `RegisterPartner` (`license_type`, `capabilities_granted`), `ListExpiringLicenses` renewal — the **licence** half | one `licensure:{authority_id}` `scores` row per status, on the subject key, signed by the authority or its `license`-scoped delegate | **A** | `put_attestation`; admission refuses `licensure_delegator_not_authority` | REPLACE. The registry signs as the CIRIS authority (`authority_id` = the CIRIS org UUID) and CIRISVerify co-signs (CC 3.4.9; single-source rows compose at confidence ≤ 0.5). No `professional_*` enum survives on the wire: the practised domain is the `authority_id`, e.g. a medical board, and the obligations ride `duty:{kind}` |
| `RevokeEntity(license)`, `MassRevoke(license)` | a `licensure:{authority_id}` row carrying `suspended` (reversible) or `revoked` (terminal), or `lapsed` / `surrendered` / `reduced` | **A**, the same authority or its delegate | `put_attestation` | REPLACE. Not a `SignedRevocation`: a licence ends by its authority, and a consumer MUST NOT infer `revoked` from `suspended` (CC 3.1.1). Reinstating is a `withdraws` on the `suspended` row |
| `RevokeEntity(agent)`, `RevokeEntity(partner)`, `MassRevoke` of keys | `SignedRevocation` | Q for authority keys; M (OrgAdmin) for an org's own keys | `put_revocation` | REPLACE |
| `SetEmergencyShutdown`, `ClearEmergencyShutdown` | server `POST /v1/accord/halt` (+ clear) | Q | accord halt | SERVER |
| `RotateSigningKey`, `GetActiveSigningKey`, `ListSigningKeys` | none: the node key is per-node and conferred | — | — | DROP |
| `RegisterTrustedPrimitiveKey`, `List…`, `Revoke…` | server `/v1/accord/ci-key/{propose,cosign}` | Q | accord co-scrub → `infra:attest` | SERVER |
| `RegisterBuildAttestation`, gRPC `RegisterBuild` | pipeline-signed `provenance:build_manifest:{target}` Contribution | B (`infra:attest`) | `put_attestation` | REPLACE (#138's consumer side already reads this) |
| `POST /v1/builds`, `POST /v1/verify/build-manifest` (admin bearer + signed body) | same paths, **S-tier submit** | B: the signature must be an `infra:attest` key; the bearer is dropped | Contribution + manifest bytes (§6.2) | KEEP paths, auth REPLACED |
| `POST /v1/verify/binary-manifest`, `POST /v1/verify/function-manifest` (admin bearer; **the registry signs**) | none | — | — | DROP. "The registry signs what the operator uploaded" is exactly the self-published authority #133 retired; use the B-tier routes above |
| `POST /v1/transparency/witnesses` (admin bearer) | witness `SignedKeyRecord` | Q | `put_public_key` | REPLACE (CIRISPersist#102) |
| `POST /v1/transparency/sth/cosign` | same path | S (the witness's own hybrid signature; this is unchanged) | cosign attestation | KEEP |

### 4.3 Org and Portal operations (tier M, O or L)

The operational planes already exist in persist v48 (`federation/operational.rs`:
`organization` / `org_membership` LWW, `partner_record` monotonic-quorum; CIRISRegistry#70).
The authority for an org action moves **from a JWT's claimed role to a signature by a
key whose `org_membership` role is sufficient**. The `OrgRole` ladder
(`OrgAdmin=1 … Viewer=4`) survives unchanged as the value of `org_membership.role`.

| Today (PortalService) | Post-fold | Tier | Substrate | Disposition |
|---|---|---|---|---|
| `CreateOrganization`, `BatchCreate…` (SYSTEM_ADMIN) | signed `organization` genesis by its first OrgAdmin | S (self-founding) | `put_organization` | REPLACE. God-mode creation is gone; OPEN whether an org needs a Q co-sign to be *recognised* (§6.5) |
| `UpdateOrganization` | signed `organization` supersession | M (OrgAdmin) | `put_organization` | REPLACE |
| `Get/ListOrganizations`, `GetOrganizationHierarchy`, `ListChildOrganizations` | `GET /v1/orgs/…` | P for public org facts; M (Viewer) for member lists | `list_organizations_*`, `list_org_memberships_for` | OPEN (§6.6: which org facts are public) |
| `AddUserToOrg`, `RemoveUserFromOrg`, `UpdateUserOrgRole`, `CreateOrgUser`, `BatchCreateOrgUsers`, `CreateUserWithMembership` | signed `org_membership` envelopes | M (OrgAdmin) | `put_org_membership` | REPLACE |
| `GetOrgUser`, `ListOrgUsers`, `ListOrgMembers`, `GetOrgUserByEmail` | reads over `org_membership` | M (Viewer) | `list_org_memberships_for` | REPLACE. **Email lookup does not survive:** an email is not a federation identifier and the operational planes refuse off-federation identifiers |
| `GetUser`, `GetUserByEmail`, `CreateUser`, `*SystemUser*`, `*OAuth*` | server auth (`wa_cert`, `/v1/auth/*` session + OAuth) | server's own | `WaCertService` | SERVER. User accounts are the node's login surface, not registry data |
| `GenerateKeyPair`, `RequestSignature` (the registry **holds** partner private keys) | none | — | — | DROP (§6.1, settled by rule 4). The registry signing *as* a partner with no `delegates_to` edge is delegation laundering (CC 4.1.1) and judgment on a node key (rule 1) |
| `ActivateKey`, `RotateKey`, `RevokeKey`, `GetRegistrationChallenge`, `RegisterPublicKey`, `Activate/RotateSelfCustodyKey` | the #65/#66 key-registration flow: a self-signed `SignedKeyRecord` + an OrgAdmin `org_membership` binding | S (self) + M (KeyManager) | `put_public_key`, `list_key_registration_history` | REPLACE (#65) |
| `RequestKeyEscrow`, `RequestKeyRecovery`, `ListKeyEscrows` | node-local working index | O (the node is the custodian) + M (OrgAdmin) to request | `registry_key_escrows` (CIRISPersist#752, built for this fold) | KEEP semantics, backend → persist |
| `GetAuditLog`, `ExportAuditLog`, `CreateAuditEntry` | persist audit (`cirisaudit`) | M (Viewer read / Operator write) | persist audit | REPLACE |
| `GenerateComplianceReport` | derived from the audit reads | M (OrgAdmin) | — | OPEN: Portal-side rendering |
| `RegisterWebhook`, `ListWebhooks`, `DeleteWebhook` | node-local | O | node config | OPEN: likely DROP in favour of the event stream |
| `ListExpiringLicenses` | the licensure fold with `valid_until` inside the window, per authority | A (an authority listing its own issuances) or P for a subject's own | `list_attestations_for` | REPLACE. A row whose `valid_until` has passed is no longer live (CC 2.1); `lapsed` is the authority saying so on the record. A consumer reads either and fabricates neither a `lapsed` nor a `revoked` |
| `GetPartnerActivity` | reads over `partner_record` | P | `list_partner_records_since` | REPLACE |
| `CleanupTestRecords` | none | — | — | DROP |

### 4.4 Device integrity

| Today | Post-fold | Tier | Disposition |
|---|---|---|---|
| `GET /v1/integrity/nonce`, `POST /v1/integrity/verify`, `/v1/integrity/ios/{nonce,verify,assert}` | OPEN | P, rate-limited | OPEN (§6.7). The server has no device-integrity surface: `/v1/auth/attestation` is CEG attestation emission, not device integrity. CIRISVerify ships Android Key Attestation and App Attest validators against pinned vendor roots |
| `POST /v1/integrity/auth` | none | — | DROP. It returns `authenticated: true, authorized: true` for any `Bearer ` prefix without validating the token (`api/http.rs` `integrity_auth`). It must not be ported, and should be fixed or removed before the fold (§7) |

### 4.5 The three legs, and what happens to the Community tier

Rule 4 applied to what the registry sells and stores today.

| Today | Which leg it really is | Post-fold object | Signer | Ends by |
|---|---|---|---|---|
| `LicenseType::PROFESSIONAL_*` with `capabilities_granted` / `denied`, `max_autonomy_tier`, `requires_supervisor` | **licensure** (standing to practise) plus the obligations attached to it | `licensure:{authority_id}` status rows on the practitioner or agent key; obligations as `duty:{kind}` rows (CC 3.1.1, rc5) | tier A: the authority (a board, a regulator, the CIRIS org) or its `license`-scoped delegate | the authority: `suspended` / `revoked` / `lapsed` / `surrendered` / `reduced` |
| `deployment_limit`, `allowed_identity_templates`, `geographic_restrictions`, per-asset access | **grant** (access to a specific thing) | `key_grant` / `consent:scope:*` from the holder of the asset | the asset's owner; non-transferable, an onward grant is a *new* grant by a key-holder | rotation, expiry, exhaustion |
| `OrgRole` (`OrgAdmin` … `Viewer`), registrar, the CI key, `requires_supervisor`'s supervising relation | **delegation** (agency to act for a principal) | `delegates_to` with a scope: `org_membership` role chain, `license` / `grant` issuance scopes, `infra:attest[:licensure:{A}]` | the principal | `withdraws`, attenuation per link, depth cap |
| `LicenseType::COMMUNITY` / `COMMUNITY_PLUS`, the issuance fee, `bond_posted` | **none of the three** | see below | — | — |

**There is no community licence to buy.** Under CC 3.2 T1, first run writes
`attestation(user → user)`: the user is their own root and the node inherits it. That
is the whole of "community standing", and no authority issues it. Federated capability
comes from accepting a trust root (`trust:accepts:v1`, the delegation plane), not from a
licence. Licensure only means something under a named authority for a practised domain.
The bond survives as what CLAUDE.md already calls it, a Sybil-resistance deposit that is
refundable and waivable on the Sovereign path, and it stays off-wire (CC 3.3.10: no
payment-processor data in any envelope). So the post-fold registry **issues nothing for
the Community tier**, `LicenseType::COMMUNITY` is never emitted as a `licensure:*` row,
and CIRISPortal's purchase flow has no registry counterpart (§6.9). What the Constitution
still carries that contradicts this is §6.10.

**CIRISPortal becomes CIRISClient CSDs.** Each row above is a screen in the client,
keyed to the CC family it renders, signing through the user's own node (CIRISClient
`CSD.md`: "the app holds no keys and does no crypto"). Delegation is covered today
(CSD-055 delegations, CSD-085 claim node, CSD-086 add federation ID, CSD-090 duty
conferral). Licensure and grants are not: no CSD yet issues, suspends or revokes a
licence under an authority the user holds, lists the licences a key holds per authority,
or shows the `key_grant`s a key holds or has issued. Those are the Portal remainder, and
they are client work against the routes in §4.1 and §4.2. Billing (CSD-056) stays
off-wire and is the only Portal function that does not become a CEG object.

## 5. What the registry slice hands the server

This mirrors the lens precedent: `LensCore::attach_handler(&edge, engine)` plus
`read_api*(…) -> Router`, merged in `compose.rs`.

```rust
// ciris-registry-core, default-features = false (no sqlx, no tonic, no persist-postgres)
pub fn router(engine: Arc<Engine>, node_key_id: String) -> axum::Router;
```

Landed with the three routes that already ran on persist alone (`/v1/steward-key`,
`/v1/trust-root/bundle`, `/v1/agent_files/{kind}`). Each further KEEP / REPLACE route
moves into this router as its backend moves to persist. The router applies **no rate
limit**: the composition root owns that layer, as it does for lens.

- **No own storage.** The slice never opens a pool. Everything in §4 marked KEEP /
  REPLACE reads and writes the shared `Engine`.
- **No own identity or Edge.** `/v1/identity` and the Reticulum transport are the
  server's. `edge_runtime` stays in the standalone binary only.
- **No gRPC.** The server has no tonic. RegistryService's reads become the P-tier HTTP
  routes in §4.1; its writes become envelopes (§4.2). The standalone binary keeps
  tonic during the transition, behind the `standalone` feature, which is on by default.
- **Declared capabilities.** Covered by §6.8.

**Rule-4 obligations on the slice's code**, so they are not rediscovered at review:

- The licensure fold keys on **emitter resolves to authority**. Filter to rows whose
  `attesting_key_id` is the `authority_id` or reaches it over a `license`-scoped
  `delegates_to` chain; every other row is testimony and is returned, if at all, under
  a separate `testimony` member at consumer confidence.
- Statuses are a **set**. Never collapse to one scalar; never derive `revoked` from
  `suspended`; an expired `valid_until` makes a row not-live (CC 2.1); it does not make the licence `revoked`.
- Scope matching is **directional and exact**. `infra:attest:licensure:{A}` satisfies a
  check for `infra:attest:licensure:{A}` only; `infra:attest` satisfies both; an
  unrecognised sub-scope fails closed. No prefix tests.
- No graph walk over licences or grants. Only `delegates_to` is walked, with the
  CC 4.1.1 depth cap and cycle rejection persist already enforces.
- `authority_id` for an org is its **org UUID** (CC 3.3.9). `PartnerRecord.organization_id`
  stays region-local and never appears in a dimension.

## 6. Decisions needed

Each item carries a recommendation, but none of them is settled by this document.

1. **Custodial keys (`GenerateKeyPair`, `RequestSignature`).** SETTLED by rule 4 and
   CC 4.1.1: the registry signing as a partner with no `delegates_to` edge is delegation
   laundering, and judgment on a node key (rule 1). DROP at the fold. Partners hold
   their own keys (the self-custody RPCs already exist), and escrow covers recovery
   (CIRISPersist#752, CC `archive_custody`). What remains open is only the migration:
   each custodied key's owner must register a self-custody key before the cutover.
2. **Manifest bytes.** `builds` / `binary_manifests` / `function_manifests` are the
   last registry-owned blobs. #41 says to keep them local; no persist table holds them.
   *Recommend:* ask persist for a consumer table like `registry_key_escrows`
   (the #751 pattern), keyed by `(project, version, target, manifest_hash)`. Serving
   bytes is P-tier, and the Contribution carries the authority.
3. **`VerifyDeployment`.** Keep a combined read route, or let clients compose agent +
   partner reads? *Recommend:* keep it. The capability intersection
   `agent ∩ partner.granted − partner.denied` is the one place fail-secure logic lives.
4. **Offline package / delta.** Is it still needed once anti-entropy replicates the
   signed stream? *Recommend:* keep as P-tier `GET` bundles of `list_signed_*_since`
   for air-gapped verifiers (the 72-hour grace), not as a registry snapshot table.
5. **Who may author an agent registration or an org's recognition.** Q (accord
   holders) for everything does not scale, and CC 4.2.1 forbids Q on the licensure and
   consent planes anyway. For **licences** rc5 answers it: the CIRIS authority key
   confers a `license`-scoped `delegates_to` on a human registrar (or attenuates
   `infra:attest:licensure:ciris` onto a pipeline key), and admission refuses an
   issuance whose chain does not resolve to the authority. For **agent key records**
   and **org recognition** the same shape applies with the accord's own delegation
   (#128). *Recommend:* delegated registrar, never a node key; the `license` scope for
   licences, a registrar scope for key records, both attenuated per CC 4.5.
6. **Which org facts are public.** Org name and partner status are needed by verifiers;
   member lists are not. *Recommend:* `organization` + `partner_record` are P; the
   `org_membership` detail is M (Viewer).
7. **Device integrity home.** Registry-slice route, server `/v1/auth/*`, or a
   verify-owned surface? It needs Google service-account credentials, which is node
   config. *Recommend:* server `/v1/auth/device-integrity/*`, validating with
   CIRISVerify's pinned-root validators.
8. **The `registry:lookup` capability name.** CIRISClient gates on
   `registry:lookup`, but the server's `/v1/federation/conformance` capabilities list
   is the **conferred scopes on the node key**, and rule 1 says a node key carries
   only `infra:*`. A `registry:*` scope therefore cannot be conferred on a node. The
   choices:
   - (a) the client gates lookup on `infra:serve` plus the slice being mounted;
   - (b) conformance gains a separate `surfaces` list (what this build mounts) beside
     `capabilities` (what the key was conferred);
   - (c) accept a `registry:*` scope family on node keys.

   *Recommend:* (b). It keeps "what I can do" apart from "what I was granted", which is
   the same honesty rule the client's `UNDECLARED`/`UNDETERMINED` states exist for.
   This needs a CIRISServer#499 and CIRISClient change.
9. **Portal.** SETTLED in direction, open in scope. CIRISPortal becomes CIRISClient
   CSDs (§4.5): a client that signs envelopes with its users' keys through their node.
   A central front-end driving a canonical node with an owner session would reintroduce
   god-mode through the O tier. *Open:* which CSDs, and in what order. The licensure
   and grant screens have no CSD yet; the `partner_record` steward-quorum signing has
   none either. Those need to be filed in CIRISClient at stage `envisioned` against
   this FSD's routes before the fold removes the Portal RPCs they replace.
10. **The Constitution still carries a community licence.** Rc5's CC 3.1.1 keeps
    `partner_role:{role}` with `community` / `community_plus` values, and CC 3.3.9's
    `partner_record.license_type` still enumerates `community | community_plus |
    professional_*`. Both contradict rule 4 read with CC 3.2 T1 and CC 8.3.5's F1 first-adopter
    exposure ("separates earned standing from purchasable token"). *Recommend:* file against
    CIRISConstitution: drop the community values from `partner_role`, and re-state
    `partner_record` as recognition only (no `license_type`, no `capabilities_*`),
    with the licence carried by `licensure:{authority_id}` and the obligations by
    `duty:{kind}`. Until ruled, the registry emits no `community` rows (§4.5) and
    treats `license_type` as a legacy projection.
11. **`partner_record` versus the licensure fold, during transition.** Verify reads
    `attestation:license_validity` today from `partner_record`. *Recommend:* the
    registry emits both for one release: the `partner_record` (recognition, unchanged
    bytes for Verify) and the `licensure:ciris` row set. Verify moves its L4 read to
    the fold, then `license_type` is dropped from the envelope (decision 10).

## 7. Before the fold (pre-existing gaps found while mapping)

- `PortalService.get_user` / `get_user_by_email` performed **no authorization**: any
  valid JWT could read any user and all of that user's memberships. **Fixed alongside
  this FSD:** the caller must be the user, a SYSTEM_ADMIN, or hold at least Viewer in
  an org the target belongs to. That is the same visibility `list_org_members` already
  grants.
- `POST /v1/integrity/auth` reports `authenticated`/`authorized` from the mere presence
  of a `Bearer ` prefix (§4.4). **Not changed here:** CIRISVerify and the mobile
  client may consume those fields, so the fix (validate, or stop claiming) needs a
  consumer check first.

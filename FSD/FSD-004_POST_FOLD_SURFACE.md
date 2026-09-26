# FSD-004 — The Post-Fold Registry Surface

**Status:** DRAFT for decision (2026-09-26). No NEW route or capability name in this
document is on the wire yet; §6 lists the decisions it needs first. What HAS landed is
§5's shape for the routes already marked KEEP: `ciris_registry_core::fold::router` (a
build without default features carries no sqlx, tonic or persist-postgres).

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
`FSD/NAMING_THE_TRUST_ROOT.md`, `MISSION.md` §1.5. Issues: #41 (handler cutover),
#58 (Spock removal), #62 (fold epic), #65/#66 (key-registration routes), #133
(registry becomes a conferred canonical server), CIRISServer#442 (registry-slice role
gate), CIRISServer#499 (capability declaration).

---

## 1. The three rules this surface is derived from

These are not new policy. Each is already ratified or shipped somewhere; this FSD
applies them to the registry's surface.

1. **A node holds `infra:*` and nothing else.** CC 4.4.3.4.3 conformance, as
   CIRISServer `FSD/TRUST_ROOT_CAPABILITY_GATE.md` puts it: "judgment roles (steward,
   moderator, founder-authority) … are never bestowable on a pure node … the node
   holds your standing; you (or your agent) wield the judgment."
   **Consequence:** every registry *decision* (register an agent, grant a license,
   revoke, halt) is a signature by a human-held key. The node admits, stores and
   serves it. No registry route lets a node, including a canonical one, decide on its
   own key's authority. This is the route-level form of MISSION §1.5: "a node
   running the registry slice gains a *vote*, never a *verdict*."
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
| **B — blessed key** | a key whose record carries an accord-conferred `infra:attest` (or `infra:serve`) | `mesh_genesis::carries_scope` over the scrub-signed `registration_envelope.roles`; the #138 walk does exactly this | `REGISTRY_ADMIN_TOKEN` + `trusted_primitive_keys` |
| **Q — accord quorum** | ≥2 of 3 accord holders (A1/B1/C1), hardware-held | the propose/cosign ceremonies (`accord_provision.rs`), `accord/halt` | SYSTEM_ADMIN JWT (`RegistryAdminService`) |

**Retired outright, with no post-fold equivalent:**
- SYSTEM_ADMIN god-mode;
- the static `REGISTRY_ADMIN_TOKEN` bearer;
- HS256 JWTs for registry authority;
- the dead `ROLE_SYSTEM_AUDITOR` / `ROLE_WISE_AUTHORITY` constants.

`ROLE_HUMANITY_ACCORD=4`, already documented as "never granted via JWT alone", becomes
tier Q.

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
| gRPC `LookupPartner`; `GET /v1/partner/{key_id}` | `GET /v1/partner/{key_id}` | P | as above | `list_partner_records_for` (monotonic-quorum merge) | KEEP path, backend → persist |
| gRPC `VerifyDeployment` | `GET /v1/registry/verify-deployment` | P | as above | agent + partner reads, capability intersection | OPEN (§6.3) |
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

### 4.2 Registry decisions (tier Q or M, human-signed)

Rule 1 in practice: each of these becomes a **signed CEG envelope** authored by a human
key and **admitted** by blessed nodes. The post-fold transport for the envelope is the
anti-entropy plane (#58), not an HTTP write. Where an HTTP door exists, it is an
S-tier "submit this signed envelope" route; the route's own auth proves nothing.

| Today | Post-fold | Tier (of the envelope's signer) | Substrate | Disposition |
|---|---|---|---|---|
| `RegisterAgent`, `BatchRegisterAgents` | `SignedKeyRecord` identity_type `agent` | Q; the delegation depth is OPEN (§6.5) | `put_public_key` | REPLACE |
| `RegisterPartner`, `UpgradeToPartner`, `CreateLicenseeOrganization` | `partner_record` envelope | Q, monotonic-quorum merge | `put_partner_record` | REPLACE. Note #139: the licensure `authority_id` is the org_id |
| `RevokeEntity`, `MassRevoke` | `SignedRevocation` | Q for authority keys; M (OrgAdmin) for an org's own keys | `put_revocation` | REPLACE |
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
| `GenerateKeyPair`, `RequestSignature` (the registry **holds** partner private keys) | none on a fabric node | — | — | OPEN (§6.1). Custodial signing conflicts with rule 1 |
| `ActivateKey`, `RotateKey`, `RevokeKey`, `GetRegistrationChallenge`, `RegisterPublicKey`, `Activate/RotateSelfCustodyKey` | the #65/#66 key-registration flow: a self-signed `SignedKeyRecord` + an OrgAdmin `org_membership` binding | S (self) + M (KeyManager) | `put_public_key`, `list_key_registration_history` | REPLACE (#65) |
| `RequestKeyEscrow`, `RequestKeyRecovery`, `ListKeyEscrows` | node-local working index | O (the node is the custodian) + M (OrgAdmin) to request | `registry_key_escrows` (CIRISPersist#752, built for this fold) | KEEP semantics, backend → persist |
| `GetAuditLog`, `ExportAuditLog`, `CreateAuditEntry` | persist audit (`cirisaudit`) | M (Viewer read / Operator write) | persist audit | REPLACE |
| `GenerateComplianceReport` | derived from the audit reads | M (OrgAdmin) | — | OPEN: Portal-side rendering |
| `RegisterWebhook`, `ListWebhooks`, `DeleteWebhook` | node-local | O | node config | OPEN: likely DROP in favour of the event stream |
| `ListExpiringLicenses`, `GetPartnerActivity` | reads over `partner_record` | P | `list_partner_records_since` | REPLACE |
| `CleanupTestRecords` | none | — | — | DROP |

### 4.4 Device integrity

| Today | Post-fold | Tier | Disposition |
|---|---|---|---|
| `GET /v1/integrity/nonce`, `POST /v1/integrity/verify`, `/v1/integrity/ios/{nonce,verify,assert}` | OPEN | P, rate-limited | OPEN (§6.7). The server has no device-integrity surface: `/v1/auth/attestation` is CEG attestation emission, not device integrity. CIRISVerify ships Android Key Attestation and App Attest validators against pinned vendor roots |
| `POST /v1/integrity/auth` | none | — | DROP. It returns `authenticated: true, authorized: true` for any `Bearer ` prefix without validating the token (`api/http.rs` `integrity_auth`). It must not be ported, and should be fixed or removed before the fold (§7) |

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

## 6. Decisions needed

Each item carries a recommendation, but none of them is settled by this document.

1. **Custodial keys (`GenerateKeyPair`, `RequestSignature`).** The registry today
   generates and holds partner private keys and signs on request. A fabric node doing
   this signs *as* a partner, which is judgment on a node key (rule 1).
   *Recommend:* retire custodial signing at the fold. Partners hold their own keys
   (the self-custody RPCs already exist), and escrow covers recovery (CIRISPersist#752,
   CC `archive_custody`).
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
   holders) for everything does not scale. The alternative is a delegation from the
   accord to a registrar role held by a *human* key (CC 2.4.1.2 `delegates_to`, #128).
   *Recommend:* delegated registrar, never a node key.
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
9. **Portal.** Does CIRISPortal stay a central front-end, driving a canonical node with
   an owner session, or become a client that signs envelopes with its users' keys?
   §4.3 assumes the latter; the former reintroduces god-mode through the O tier.

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

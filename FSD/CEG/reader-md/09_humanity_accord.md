
---

# §9 The HUMANITY_ACCORD constitutional layer

The single wire-format asymmetry in the federation.

## §9.1 The accord-holder triple

Three named human key holders. Initial state at federation genesis:

| Position | Holder | Threshold |
|---|---|---|
| 1 | Eric Moore | 2-of-3 |
| 2 | Eric Kudzin | 2-of-3 |
| 3 | Haley Bradley | 2-of-3 |

Hardware-attested (per [§9.4](#94-hardware-class-taxonomy) hardware_class taxonomy). Permanent: no automatic decay; replacement requires out-of-band CIRIS L3C process per FEDERATION_ANNOUNCEMENT.md §4.5.3.

**Correlated-failure geometry (named honestly — 1.0-RC1, [#71](https://github.com/CIRISAI/CIRISRegistry/issues/71) C5):** two of the three holders share a household, so the 2-of-3 quorum is physically achievable from one street address — a correlated compromise/coercion surface that entrenchment makes harder to correct later. The authority at stake is the **full constitutional kill** (`EmergencyShutdown CONSTITUTIONAL` — not a recoverable pause), so the exposure is real and is not softened here; what scope isolation ([§9.2](#92-authority-scope)) does guarantee is that compromise cannot escalate *beyond* the kill — accord keys cannot sign grants, licenses, or amendments. **The mitigant is diversifying the holder set: finding new holders** (via the out-of-band replacement process, FEDERATION_ANNOUNCEMENT.md §4.5.3) so that no household — and ultimately no single jurisdiction — can assemble the quorum. This is an active obligation on CIRIS L3C, not a deferred nice-to-have.

**The HUMANITY_ACCORD triple is the canonical entrenched-`family` instance (CEG 0.7 retcon).** Per [§5.6.8.9](05_namespace.md), the accord-holder triple structurally IS a `family` subject_kind with:

```
family {
    family_key_id:                   "humanity-accord",
    family_name:                     "Humanity Accord",
    members: [
        {key_id: <eric-moore-key>,      role: founder},
        {key_id: <eric-kudzin-key>,     role: founder},
        {key_id: <haley-bradley-key>,   role: founder}
    ],
    consensus_protocol:              "quorum:2/3",
    consensus_protocol_entrenched:   true   // replacement requires §9.2 / FEDERATION_ANNOUNCEMENT.md §4.5.3 ceremony
}
```

The 2-of-3 multi-sig verifier at [§9.2.1](#921-invocation-canonical-bytes-anti-replay-01-scaffold) is the `quorum:2/3` consensus_protocol enforcement; the entrenchment property is what prevents any federation-internal authority from amending the protocol. §9 remains load-bearing for the **role-recognition policy** (which dimensions accord-holders may emit — only `accord:*` per [§7.1](07_reserved.md)) and the **scope-isolation** discipline (only `EmergencyShutdown CONSTITUTIONAL` per [§9.2](#92-authority-scope)). CEG 0.7 makes the structural shape explicit — the constitutional asymmetry is "an entrenched family that is wire-scope-isolated to halt authority," not a one-off primitive. Other entrenched-family instances (a national-emergency triple, an international body, a court-ordered preservation triple) MAY appear in operator deployments; HUMANITY_ACCORD is the one CIRIS L3C deployments ship at genesis.

## §9.2 Authority scope

`HUMANITY_ACCORD` signatures are valid only on `EmergencyShutdown CONSTITUTIONAL` (`IncidentSeverity::INCIDENT_CONSTITUTIONAL = 5`), `accord:invoke:notify:{notify_id}`, `accord:invoke:drill:{drill_id}`, `accord:lifecycle:active`, and the corresponding `FederationAnnouncement` priority `AccordCarrier`. Announcements of any other priority signed by accord-holder keys are rejected at admission (out of role). Federation-side authority cannot sign `AccordCarrier`; humanity-accord authority cannot sign anything else. **Wire-isolated AND scope-isolated.**

### §9.2.1 Invocation canonical bytes (anti-replay; 0.1 scaffold)

> **RESOLVED at 1.0-RC1**: the discriminator + nonce binding below addresses the cross-invocation-replay hole identified by CEG 0.1 cryptographic + red-team review. The anticipated [§5.2.1](05_namespace.md) refinement landed as the **JCS redesign** (TupleHash128 retired — see §5.2.1); this invocation encoding is **intentionally NOT migrated**: its preimage is closed-vocabulary (discriminator + nonce + enum fields, no attacker-controlled free text), so the injection surface the §5.2.1 redesign closes is not reachable here, and genesis-critical bytes stay stable.

Every `accord:invoke:*` Contribution signs the following canonical bytes (BOTH the discriminator AND a per-invocation nonce are in the signed payload — preventing CONSTITUTIONAL ↔ notify ↔ drill cross-replay):

```
canonical = sha256(
    "ciris.accord_invoke.v1\n" ||
    "invocation_kind=" || ("CONSTITUTIONAL" | "notify" | "drill") || "\n" ||
    "invocation_id=" || halt_id_or_notify_id_or_drill_id || "\n" ||
    "nonce=" || base64url(rand_32_bytes) || "\n" ||
    "asserted_at=" || rfc3339_canonical || "\n" ||      // per §0.5
    "valid_until=" || rfc3339_canonical || "\n" ||
    "payload_sha256=" || sha256_hex_lowercase_of_payload // per §0.6
)
```

Hybrid signature per [§5.2.1](05_namespace.md): Ed25519 + ML-DSA-65 bound-payload. Each of the 2-of-3 holders signs `canonical` independently; consumer verifies all three signatures against the same `canonical` bytes and counts ≥ 2 valid.

The substrate MUST reject duplicate `invocation_id` values within the `valid_until` window (per-kind unique).

### §9.2.2 `notify` vs `CONSTITUTIONAL` — consumer-UI requirement

A CEG-Conforming Consumer (CCC) presenting accord invocations to humans MUST visually distinguish the three kinds:

- **`CONSTITUTIONAL`** — kill-switch authority; full halt; visible as an unambiguous emergency banner.
- **`notify`** — federation-wide accord-holder communication; MUST NOT be visually conflated with CONSTITUTIONAL.
- **`drill`** — accord-holder exercise; MUST be visually marked as a drill (e.g., explicit "[DRILL]" prefix on any human-visible surface).

Wire-format isolation alone does not close the social-engineering risk that downstream UI conflates the three; the consumer-UI requirement above is the load-bearing safeguard against accord-holders being socially pressured into emitting a `notify` that carries CONSTITUTIONAL social weight without CONSTITUTIONAL substrate weight.

## §9.3 Concern split — key material vs role-recognition policy

**Key material** (Ed25519 + ML-DSA-65 pubkeys for the three holders) lives in **CIRISPersist substrate**: `federation_keys` rows with `identity_type="accord_holder"`, self-signed at provisioning, cross-attested by all three regional stewards.

**Role-recognition policy + verifier logic** lives in **`ciris-registry-core`**: the 2-of-3 multi-sig verification, the `EmergencyShutdown CONSTITUTIONAL` admin RPC, the audit hooks.

## §9.4 Hardware-class taxonomy

| Value | Use |
|---|---|
| `HSM_FIPS_140_3_L3` | Production stewards (US / EU / APAC) |
| `Apple_Secure_Enclave` | Accord-holders on iOS/macOS |
| `YubiKey_5_FIPS` | Accord-holders preferring portable hardware tokens |
| `TPM_2_0` | Accord-holders on Linux/Windows desktops |
| `placeholder_pending_provisioning` | Interim value before actual hardware provisioning. Consumers MUST treat as `0.0` trust weight |
| `software_hsm_development` | DEVELOPMENT ONLY; consumer policy MUST reject for federation-scope verification |

Per-class recommended trust-multipliers: `HSM_FIPS_140_3_L3` = 1.0; `Apple_Secure_Enclave` = 0.95; `YubiKey_5_FIPS` = 0.95; `TPM_2_0` = 0.9; `placeholder_pending_provisioning` = 0.0; `software_hsm_development` = 0.0.

### §9.4.1 Hardware-class self-assertion gap (acknowledged)

The `hardware_class` field is currently a self-asserted string on each `federation_keys` row. CEG 0.1 has no normative mechanism (TPM quote chain, Apple attestation, FIDO attestation) for a verifier to independently corroborate the claim. Per [§15.2](15_gaps.md) **R5** (acknowledged risk): consumer policy MUST treat the `hardware_class` field as a producer claim, not a cryptographically-attested fact. A planned 0.x → 1.x roadmap item closes this via per-platform attestation-chain verification; until then the trust-multipliers in §9.4 above bind only as guidance.

## §9.5 Why this isn't a Golden-Rule violation

Per [§1.5](01_foundation.md): the Recursive Golden Rule binds *participants in the federation* to each other. Humanity-as-such occupies a position outside the federation's participant set, by design. The three named human key holders hold `AccordCarrier` authority that no federation-side authority class (including `SYSTEM_ADMIN` / `WISE_AUTHORITY` / per-install stewards) can grant itself, revoke, override, or decay. This is not a Golden-Rule exemption; it is the recognition that consent (M-1's load-bearing property) requires revocability, and revocability requires a halt-authority that lives outside the system being halted. The federation cannot deny humans the right to halt it, because no federation-internal protocol path to that signature exists.



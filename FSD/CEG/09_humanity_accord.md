[← §8 Composition](08_composition.md) | **§9 HUMANITY_ACCORD** | [Next: §10 Endpoints →](10_endpoints.md)

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

## §9.2 Authority scope

`HUMANITY_ACCORD` signatures are valid only on `EmergencyShutdown CONSTITUTIONAL` (`IncidentSeverity::INCIDENT_CONSTITUTIONAL = 5`), `accord:invoke:notify:{notify_id}`, `accord:invoke:drill:{drill_id}`, `accord:lifecycle:active`, and the corresponding `FederationAnnouncement` priority `AccordCarrier`. Announcements of any other priority signed by accord-holder keys are rejected at admission (out of role). Federation-side authority cannot sign `AccordCarrier`; humanity-accord authority cannot sign anything else. **Wire-isolated AND scope-isolated.**

### §9.2.1 Invocation canonical bytes (anti-replay; 0.1 scaffold)

> **0.1 SCAFFOLD NOTE**: The discriminator + nonce binding below addresses the cross-invocation-replay hole identified by CEG 0.1 cryptographic + red-team review. 0.2 may refine the encoding when [§5.2.1](05_namespace.md) canonical-bytes redesigns to TupleHash128.

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

---

[← §8 Composition](08_composition.md) | **§9 HUMANITY_ACCORD** | [Next: §10 Endpoints →](10_endpoints.md)

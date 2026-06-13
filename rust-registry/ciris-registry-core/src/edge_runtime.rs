//! Edge transport-identity runtime — the native-Rust mirror of CIRISLens's
//! `api/edge_runtime.py` (CIRISLens#20).
//!
//! Brings up a `ciris-edge` Reticulum transport identity purely to expose
//! the server's **RET-transport** pubkeys — the third keypair role of the
//! §5.6.8.8.2 six-key `LocalIdentityAggregate` (the registry already owns
//! the signing Ed25519 + ML-DSA-65 and the content-KEM X25519 + ML-KEM-768
//! roles via persist). With this, `GET /v1/identity` emits the full 6-of-6
//! federation identity — the form the `ciris-canonical` community
//! enrollment (CIRISRegistry#56) needs to resolve a member: WHO (federation
//! key) + the transport identity that backs the signed `transport_destination`
//! binding (§5.6.8.8.1).
//!
//! This does NOT federate (no bootstrap peers, no announce loop kept
//! running) — identity exposure only, exactly the lens posture. Actual
//! Reticulum peering is deeper scope (cf. CIRISLens#18 §2 / CIRISRegistry#62).
//!
//! Gated on `CIRIS_REGISTRY_EDGE_IDENTITY_PATH`:
//! - unset/empty → no-op; `/v1/identity` emits the 4-of-6-key bundle without
//!   the RET-transport pubkeys (non-fatal, the registry serves normally).
//! - set → Edge mints (mode 0600) the identity at that path on first run and
//!   reloads it thereafter, stable across restarts. The transport identity is
//!   a **distinct** dual-key (X25519 + Ed25519) keypair, never the federation
//!   signing key (AV-17).
//!
//! Init failure is non-fatal (logged WARN); the registry serves normally.

use std::path::PathBuf;

use ciris_edge::transport::reticulum::{ReticulumAuth, ReticulumTransport, ReticulumTransportConfig};

/// The env var naming the persisted transport-identity file. Mirrors the
/// lens `CIRISLENS_EDGE_IDENTITY_PATH`; prod sets it alongside the steward
/// keyring (e.g. `/var/lib/ciris-registry/keyring/registry-edge.identity`).
pub const IDENTITY_PATH_ENV: &str = "CIRIS_REGISTRY_EDGE_IDENTITY_PATH";

/// Bring up the Reticulum transport identity and return its
/// `(x25519_pub_base64, ed25519_pub_base64)` — base64-standard of the raw
/// 32-byte halves — or `None` when the identity path is unset or init fails.
///
/// `local_key_id` is the registry's federation `key_id`, used to label the
/// transport config. We construct the transport with `signer: None`: the
/// federation signer is only used to self-sign the RNS *announce*
/// attestation (CIRISEdge#15), which matters only for self-authenticating
/// peer discovery over a running announce loop — not for *exposing* the
/// identity. The durable, trusted source of truth for the `key_id →
/// transport` binding is the signed `identity_occurrence.transport_destination`
/// envelope (§5.6.8.8.1) the registry emits via persist, not the announce
/// app-data. Signing the announce + running the loop is the deeper-scope
/// federation step (cf. CIRISLens#18 §2 / CIRISRegistry#62), matching the
/// lens's current non-federating posture.
///
/// The constructed transport is dropped after the pubkeys are read: we only
/// expose the identity here. The identity file on disk keeps the
/// destination stable across restarts.
pub async fn init_transport_identity(local_key_id: &str) -> Option<(String, String)> {
    let identity_path = match std::env::var(IDENTITY_PATH_ENV) {
        Ok(p) if !p.is_empty() => p,
        _ => {
            tracing::info!(
                "Edge transport identity not configured ({} unset); GET /v1/identity \
                 will emit a 4-of-6-key bundle without the Reticulum transport pubkeys. \
                 Set the env var to enable.",
                IDENTITY_PATH_ENV,
            );
            return None;
        }
    };

    let config = ReticulumTransportConfig::new(PathBuf::from(&identity_path), local_key_id.to_string());
    let auth = ReticulumAuth {
        signer: None,
        ..Default::default()
    };

    let transport = match ReticulumTransport::new(config, auth).await {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!(
                "Edge transport identity init failed ({}): {e}; serving without RET-transport pubkeys.",
                identity_path,
            );
            return None;
        }
    };

    // [u8; 64] = x25519_pub(32) || ed25519_pub(32), per the dual-key
    // Reticulum transport identity (CIRISEdge transport/reticulum.rs).
    let pubkey = transport.local_transport_pubkey();
    let b64 = base64::engine::general_purpose::STANDARD;
    use base64::Engine as _;
    let x25519_b64 = b64.encode(&pubkey[..32]);
    let ed25519_b64 = b64.encode(&pubkey[32..64]);

    tracing::info!(
        key_id = local_key_id,
        identity_path = %identity_path,
        x25519_prefix = &x25519_b64[..x25519_b64.len().min(16)],
        ed25519_prefix = &ed25519_b64[..ed25519_b64.len().min(16)],
        "Edge transport identity ready; folding RET-transport role into /v1/identity",
    );

    Some((x25519_b64, ed25519_b64))
}

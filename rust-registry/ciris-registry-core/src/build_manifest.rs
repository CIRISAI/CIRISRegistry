//! Vendored `BuildManifest` type for inbound POST validation.
//!
//! Mirrors `ciris-verify-core` v1.8.0 wire format
//! (`docs/BUILD_MANIFEST.md` upstream). We vendor instead of importing
//! `ciris-verify-core` because that crate has a hard `rusqlite`
//! dependency for its license cache that conflicts with `sqlx-sqlite`
//! at the `libsqlite3-sys` linker level.
//!
//! **Wire-format parity contract**: any change to `BuildManifest` /
//! `BuildPrimitive` / `ManifestSignature` / `CanonicalBuildManifest`
//! must match the upstream v1.8.0 (or later) shape exactly. The
//! `canonical_bytes()` output must be byte-identical to upstream's.
//! Integration test in Phase A (Phase A.1+) round-trips a manifest
//! signed by `ciris-build-sign` against this module's verify path.
//!
//! Closes THREAT_MODEL.md AV-26 (uploaded-manifest hybrid-sig
//! verification).

use base64::{engine::general_purpose::STANDARD, Engine};
use ciris_crypto::{ClassicalVerifier, Ed25519Verifier, MlDsa65Verifier, PqcVerifier};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Which CIRIS primitive a build manifest describes.
///
/// Wire format: snake_case strings (`"verify"`, `"agent"`, etc.). The
/// Rust enum uses PascalCase variants. Mirrors
/// `ciris_verify_core::security::build_manifest::BuildPrimitive`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildPrimitive {
    Verify,
    Agent,
    Lens,
    Persist,
    Registry,
    /// Forward-compat for primitives invented after this enum version.
    /// Production primitives should add named variants instead.
    Other(String),
}

impl BuildPrimitive {
    /// Canonical string form used by the trusted-key DB lookup
    /// (`trusted_primitive_keys.project`).
    pub fn project_name(&self) -> String {
        match self {
            BuildPrimitive::Verify => "ciris-verify".to_string(),
            BuildPrimitive::Agent => "ciris-agent".to_string(),
            BuildPrimitive::Lens => "ciris-lens".to_string(),
            BuildPrimitive::Persist => "ciris-persist".to_string(),
            BuildPrimitive::Registry => "ciris-registry".to_string(),
            BuildPrimitive::Other(name) => name.clone(),
        }
    }

    /// Inverse: parse a project string back into a typed variant.
    /// Unknown names map to `Other(...)`.
    pub fn from_project_name(name: &str) -> Self {
        match name {
            "ciris-verify" => BuildPrimitive::Verify,
            "ciris-agent" => BuildPrimitive::Agent,
            "ciris-lens" => BuildPrimitive::Lens,
            "ciris-persist" => BuildPrimitive::Persist,
            "ciris-registry" => BuildPrimitive::Registry,
            other => BuildPrimitive::Other(other.to_string()),
        }
    }
}

/// Hybrid signature carried inside a `BuildManifest`. Mirrors
/// `ciris_verify_core::security::function_integrity::ManifestSignature`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSignature {
    pub classical: String,
    pub classical_algorithm: String,
    pub pqc: String,
    pub pqc_algorithm: String,
    pub key_id: String,
}

/// `BuildManifest` describing a single build of a CIRIS PoB primitive.
///
/// Wire format is canonicalized JSON (see `canonical_bytes`). Both
/// signatures (Ed25519 + ML-DSA-65) must verify against the canonical
/// bytes for the manifest to be accepted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    pub manifest_schema_version: String,
    pub primitive: BuildPrimitive,
    pub build_id: String,
    pub target: String,
    pub binary_hash: String,
    pub binary_version: String,
    pub generated_at: String,
    pub manifest_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
    pub signature: ManifestSignature,
}

impl BuildManifest {
    /// Canonical byte representation for signing / verification.
    /// Excludes the `signature` field. Field order is fixed by the
    /// `CanonicalBuildManifest` struct definition.
    ///
    /// Must be byte-identical to upstream's `BuildManifest::canonical_bytes`
    /// for cross-implementation signature compatibility.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let canonical = CanonicalBuildManifest {
            manifest_schema_version: &self.manifest_schema_version,
            primitive: &self.primitive,
            build_id: &self.build_id,
            target: &self.target,
            binary_hash: &self.binary_hash,
            binary_version: &self.binary_version,
            generated_at: &self.generated_at,
            manifest_hash: &self.manifest_hash,
            extras: &self.extras,
        };
        serde_json::to_vec(&canonical).unwrap_or_default()
    }
}

/// Canonical form (excludes signature, matches upstream field order).
#[derive(Serialize)]
struct CanonicalBuildManifest<'a> {
    manifest_schema_version: &'a str,
    primitive: &'a BuildPrimitive,
    build_id: &'a str,
    target: &'a str,
    binary_hash: &'a str,
    binary_version: &'a str,
    generated_at: &'a str,
    manifest_hash: &'a str,
    extras: &'a Option<serde_json::Value>,
}

/// Errors returned from `verify_uploaded_manifest`.
#[derive(Debug, Error)]
pub enum BuildManifestError {
    #[error("BuildManifest parse failed: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("BuildManifest primitive mismatch: expected {expected:?}, got {got:?}")]
    PrimitiveMismatch {
        expected: BuildPrimitive,
        got: BuildPrimitive,
    },
    #[error("Invalid signature base64 ({field}): {source}")]
    Base64 {
        field: &'static str,
        source: base64::DecodeError,
    },
    #[error("Signature verification failed ({algorithm}): {message}")]
    Verify {
        algorithm: &'static str,
        message: String,
    },
    #[error("No trusted key registered for project {0}")]
    UnknownProject(String),
}

/// Verify a hybrid-signed BuildManifest payload against the trusted
/// public keys for its primitive.
///
/// Steps:
/// 1. Parse the JSON manifest.
/// 2. Reject if `manifest.primitive` does not match `expected_primitive`.
/// 3. Compute `manifest.canonical_bytes()`.
/// 4. Verify the Ed25519 signature against `ed25519_pk` over
///    `canonical_bytes`.
/// 5. Verify the ML-DSA-65 signature against `mldsa_pk` over the bound
///    payload `canonical_bytes || classical_signature`.
///
/// Returns the parsed `BuildManifest` on success. Note: this does NOT
/// dispatch primitive-specific extras validators (the registry doesn't
/// register any — that's verifier-side concern). Callers needing
/// extras validation should consume the returned manifest.
pub fn verify_uploaded_manifest(
    bytes: &[u8],
    expected_primitive: BuildPrimitive,
    ed25519_pk: &[u8],
    mldsa_pk: &[u8],
) -> Result<BuildManifest, BuildManifestError> {
    let manifest: BuildManifest = serde_json::from_slice(bytes)?;

    if manifest.primitive != expected_primitive {
        return Err(BuildManifestError::PrimitiveMismatch {
            expected: expected_primitive,
            got: manifest.primitive,
        });
    }

    let canonical = manifest.canonical_bytes();

    let classical_sig = STANDARD
        .decode(&manifest.signature.classical)
        .map_err(|e| BuildManifestError::Base64 {
            field: "classical",
            source: e,
        })?;

    let ed_verifier = Ed25519Verifier::new();
    let classical_ok = ed_verifier
        .verify(ed25519_pk, &canonical, &classical_sig)
        .map_err(|e| BuildManifestError::Verify {
            algorithm: "Ed25519",
            message: e.to_string(),
        })?;
    if !classical_ok {
        return Err(BuildManifestError::Verify {
            algorithm: "Ed25519",
            message: "signature did not verify".to_string(),
        });
    }

    let pqc_sig = STANDARD
        .decode(&manifest.signature.pqc)
        .map_err(|e| BuildManifestError::Base64 {
            field: "pqc",
            source: e,
        })?;

    // PQC signature covers (canonical || classical_sig) — bound signature
    let mut bound = Vec::with_capacity(canonical.len() + classical_sig.len());
    bound.extend_from_slice(&canonical);
    bound.extend_from_slice(&classical_sig);

    let pqc_verifier = MlDsa65Verifier::new();
    let pqc_ok = pqc_verifier
        .verify(mldsa_pk, &bound, &pqc_sig)
        .map_err(|e| BuildManifestError::Verify {
            algorithm: "ML-DSA-65",
            message: e.to_string(),
        })?;
    if !pqc_ok {
        return Err(BuildManifestError::Verify {
            algorithm: "ML-DSA-65",
            message: "signature did not verify".to_string(),
        });
    }

    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_serde_snake_case() {
        let json = r#""ciris-persist isn't this discriminator""#;
        // Wire format uses bare "persist", not "ciris-persist"
        let p: BuildPrimitive = serde_json::from_str(r#""persist""#).unwrap();
        assert_eq!(p, BuildPrimitive::Persist);
        let json = serde_json::to_string(&BuildPrimitive::Registry).unwrap();
        assert_eq!(json, r#""registry""#);
        // confirm we recognized the trick line above isn't a real assertion
        let _ = json;
    }

    #[test]
    fn project_name_roundtrip() {
        for p in [
            BuildPrimitive::Verify,
            BuildPrimitive::Agent,
            BuildPrimitive::Lens,
            BuildPrimitive::Persist,
            BuildPrimitive::Registry,
        ] {
            let name = p.project_name();
            assert_eq!(BuildPrimitive::from_project_name(&name), p);
        }
    }

    #[test]
    fn other_project_name_roundtrip() {
        let other = BuildPrimitive::Other("ciris-future".to_string());
        let name = other.project_name();
        assert_eq!(name, "ciris-future");
        assert_eq!(
            BuildPrimitive::from_project_name(&name),
            BuildPrimitive::Other("ciris-future".to_string())
        );
    }

    #[test]
    fn canonical_bytes_excludes_signature() {
        let manifest = BuildManifest {
            manifest_schema_version: "1.0".to_string(),
            primitive: BuildPrimitive::Registry,
            build_id: "abc123".to_string(),
            target: "x86_64-unknown-linux-gnu".to_string(),
            binary_hash: "sha256:deadbeef".to_string(),
            binary_version: "1.2.0".to_string(),
            generated_at: "2026-05-01T20:00:00Z".to_string(),
            manifest_hash: "sha256:cafebabe".to_string(),
            extras: None,
            signature: ManifestSignature {
                classical: "AAAA".to_string(),
                classical_algorithm: "Ed25519".to_string(),
                pqc: "BBBB".to_string(),
                pqc_algorithm: "ML-DSA-65".to_string(),
                key_id: "test-key".to_string(),
            },
        };
        let bytes = manifest.canonical_bytes();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert!(s.contains(r#""build_id":"abc123""#));
        assert!(!s.contains("signature"));
        assert!(!s.contains("AAAA"));
    }

    #[test]
    fn verify_rejects_primitive_mismatch() {
        // Construct a manifest claiming Persist; verify against expected Registry.
        let manifest = BuildManifest {
            manifest_schema_version: "1.0".to_string(),
            primitive: BuildPrimitive::Persist,
            build_id: "x".to_string(),
            target: "x".to_string(),
            binary_hash: "x".to_string(),
            binary_version: "x".to_string(),
            generated_at: "x".to_string(),
            manifest_hash: "x".to_string(),
            extras: None,
            signature: ManifestSignature {
                classical: "".to_string(),
                classical_algorithm: "".to_string(),
                pqc: "".to_string(),
                pqc_algorithm: "".to_string(),
                key_id: "".to_string(),
            },
        };
        let bytes = serde_json::to_vec(&manifest).unwrap();
        let err =
            verify_uploaded_manifest(&bytes, BuildPrimitive::Registry, &[0u8; 32], &[0u8; 1952])
                .unwrap_err();
        assert!(matches!(err, BuildManifestError::PrimitiveMismatch { .. }));
    }

    #[test]
    fn verify_rejects_garbage_json() {
        let err = verify_uploaded_manifest(
            b"not json",
            BuildPrimitive::Registry,
            &[0u8; 32],
            &[0u8; 1952],
        )
        .unwrap_err();
        assert!(matches!(err, BuildManifestError::Parse(_)));
    }

    #[test]
    fn verify_signs_and_verifies_roundtrip() {
        use ciris_crypto::{ClassicalSigner, Ed25519Signer, MlDsa65Signer, PqcSigner};

        // v13.3.1: `random()` is fallible — it fails secure when the RNG health
        // check has marked the source failed (ciris-crypto rng_health).
        let ed_signer = Ed25519Signer::random().unwrap();
        let pqc_signer = MlDsa65Signer::new().unwrap();
        let ed_pk = ed_signer.public_key().unwrap();
        let pqc_pk = pqc_signer.public_key().unwrap();

        let manifest_unsigned = BuildManifest {
            manifest_schema_version: "1.0".to_string(),
            primitive: BuildPrimitive::Registry,
            build_id: "abc".to_string(),
            target: "x86_64-unknown-linux-gnu".to_string(),
            binary_hash: "sha256:1234".to_string(),
            binary_version: "1.0.0".to_string(),
            generated_at: "2026-05-01T00:00:00Z".to_string(),
            manifest_hash: "sha256:5678".to_string(),
            extras: None,
            signature: ManifestSignature {
                classical: String::new(),
                classical_algorithm: "Ed25519".to_string(),
                pqc: String::new(),
                pqc_algorithm: "ML-DSA-65".to_string(),
                key_id: "registry-test".to_string(),
            },
        };

        let canonical = manifest_unsigned.canonical_bytes();
        let classical_sig = ed_signer.sign(&canonical).unwrap();
        let mut bound = canonical.clone();
        bound.extend_from_slice(&classical_sig);
        let pqc_sig = pqc_signer.sign(&bound).unwrap();

        let mut signed = manifest_unsigned;
        signed.signature.classical = STANDARD.encode(&classical_sig);
        signed.signature.pqc = STANDARD.encode(&pqc_sig);

        let bytes = serde_json::to_vec(&signed).unwrap();
        let verified =
            verify_uploaded_manifest(&bytes, BuildPrimitive::Registry, &ed_pk, &pqc_pk).unwrap();
        assert_eq!(verified.build_id, "abc");
    }
}

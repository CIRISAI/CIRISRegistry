//! Full SHA-256 verification of received content bytes — CEG 0.2 §10.1.1.
//!
//! Per CEG 0.2 §10.1.1:
//!
//! > A CEG-Conforming Consumer (CCC) MUST verify the full SHA-256 of
//! > received bytes against the value in `evidence_refs[]` BEFORE handing
//! > the bytes to any consumer (Agent loader, Portal renderer, etc.).
//! > The `holds_bytes:sha256:{prefix}` directory carries only a short
//! > prefix for index efficiency; the consumer MUST NOT short-circuit
//! > verification to the prefix.
//!
//! This module ships the helper. It rejects bytes whose full hash does
//! not match, and is deliberately written so a caller cannot
//! accidentally pass only a prefix.

use sha2::{Digest, Sha256};
use thiserror::Error;

/// Error returned when a `ContentBody`'s SHA-256 does not match the
/// expected value from `evidence_refs[]`.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ContentBodyVerifyError {
    /// The received bytes hash to a different SHA-256 than the
    /// expected one. Caller MUST discard the bytes and SHOULD emit a
    /// `withdraws` against the holder's `holds_bytes` attestation per
    /// §10.1.2 ContentMiss-feedback discipline.
    #[error("SHA-256 mismatch: expected {expected_hex}, got {actual_hex}")]
    Sha256Mismatch {
        /// Hex (lowercase, per CEG §0.6) of the expected SHA-256.
        expected_hex: String,
        /// Hex (lowercase) of the actual SHA-256 the bytes hashed to.
        actual_hex: String,
    },
}

/// Verify that the SHA-256 of `bytes` matches `expected_sha256`.
///
/// Returns `Ok(())` on match, `Err(ContentBodyVerifyError::Sha256Mismatch)`
/// on mismatch.
///
/// The expected hash is a full 32-byte SHA-256 — the type signature
/// rejects shorter prefixes at the call site, structurally preventing
/// the CEG §10.1.1 "short-circuit-to-prefix" anti-pattern.
pub fn verify_content_body_sha256(
    expected_sha256: &[u8; 32],
    bytes: &[u8],
) -> Result<(), ContentBodyVerifyError> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let actual: [u8; 32] = hasher.finalize().into();

    if actual == *expected_sha256 {
        Ok(())
    } else {
        Err(ContentBodyVerifyError::Sha256Mismatch {
            expected_hex: hex::encode(expected_sha256),
            actual_hex: hex::encode(actual),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sha256_of(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }

    #[test]
    fn matching_sha256_verifies_ok() {
        let bytes = b"hello, world";
        let expected = sha256_of(bytes);
        assert!(verify_content_body_sha256(&expected, bytes).is_ok());
    }

    #[test]
    fn mismatched_sha256_returns_typed_error() {
        let bytes = b"hello, world";
        let wrong_hash = sha256_of(b"goodbye, world");
        let result = verify_content_body_sha256(&wrong_hash, bytes);

        assert!(matches!(
            result,
            Err(ContentBodyVerifyError::Sha256Mismatch { .. })
        ));

        // Verify the error carries hex-encoded values per CEG §0.6 (lowercase).
        if let Err(ContentBodyVerifyError::Sha256Mismatch {
            expected_hex,
            actual_hex,
        }) = result
        {
            assert_eq!(expected_hex.len(), 64, "SHA-256 hex is 64 chars");
            assert_eq!(actual_hex.len(), 64);
            assert!(expected_hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
            assert!(actual_hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
        }
    }

    #[test]
    fn empty_bytes_verifies_against_empty_sha() {
        // SHA-256 of "" is e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let empty_sha = sha256_of(b"");
        assert!(verify_content_body_sha256(&empty_sha, b"").is_ok());
    }

    #[test]
    fn type_signature_rejects_prefix() {
        // Documentation test: the function signature takes &[u8; 32], NOT
        // &[u8]. A caller cannot pass an 8-byte hex prefix here — the
        // type system structurally prevents the §10.1.1 short-circuit
        // anti-pattern.
        let bytes = b"test";
        let prefix_array: [u8; 32] = [0u8; 32]; // padded; not a real prefix
        assert!(verify_content_body_sha256(&prefix_array, bytes).is_err());
    }
}

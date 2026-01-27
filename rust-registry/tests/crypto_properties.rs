//! Property-based tests for cryptographic operations
//!
//! These tests verify mathematical invariants using proptest.
//! The actual crypto module tests are in src/crypto/mod.rs (unit tests).
//! These integration tests verify properties that don't require internal access.

mod common;

use proptest::prelude::*;
use common::strategies::*;

// Cryptographic constant tests
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 100,
        ..ProptestConfig::default()
    })]

    /// Property: SHA-256 hashes are always 32 bytes
    #[test]
    fn sha256_produces_32_bytes(data in binary_data_strategy(0, 10000)) {
        use sha2::{Sha256, Digest};
        let hash = Sha256::digest(&data);
        prop_assert_eq!(hash.len(), 32);
    }

    /// Property: SHA-256 is deterministic
    #[test]
    fn sha256_is_deterministic(data in binary_data_strategy(0, 1000)) {
        use sha2::{Sha256, Digest};
        let hash1 = Sha256::digest(&data);
        let hash2 = Sha256::digest(&data);
        prop_assert_eq!(hash1.as_slice(), hash2.as_slice());
    }

    /// Property: Different inputs produce different hashes (with high probability)
    #[test]
    fn sha256_collision_resistant(
        data1 in binary_data_strategy(1, 1000),
        data2 in binary_data_strategy(1, 1000)
    ) {
        use sha2::{Sha256, Digest};
        prop_assume!(data1 != data2);

        let hash1 = Sha256::digest(&data1);
        let hash2 = Sha256::digest(&data2);
        prop_assert_ne!(hash1.as_slice(), hash2.as_slice());
    }

    /// Property: Hex encoding produces expected length
    #[test]
    fn hex_encode_length(data in binary_data_strategy(0, 100)) {
        let encoded = hex::encode(&data);
        prop_assert_eq!(encoded.len(), data.len() * 2);
    }

    /// Property: Hex encoding is reversible
    #[test]
    fn hex_roundtrip(data in binary_data_strategy(0, 100)) {
        let encoded = hex::encode(&data);
        let decoded = hex::decode(&encoded).unwrap();
        prop_assert_eq!(decoded, data);
    }

    /// Property: Base64 encoding is reversible
    #[test]
    fn base64_roundtrip(data in binary_data_strategy(0, 100)) {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        let encoded = STANDARD.encode(&data);
        let decoded = STANDARD.decode(&encoded).unwrap();
        prop_assert_eq!(decoded, data);
    }
}

/// Ed25519 signature tests (using ed25519-dalek directly)
mod ed25519_tests {
    use super::*;
    use ed25519_dalek::{SigningKey, Signer, Verifier};
    use rand::rngs::OsRng;

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 50,  // Crypto operations are slower
            ..ProptestConfig::default()
        })]

        /// Property: Ed25519 sign-verify roundtrip succeeds
        #[test]
        fn ed25519_sign_verify_roundtrip(data in binary_data_strategy(0, 10000)) {
            let signing_key = SigningKey::generate(&mut OsRng);
            let verifying_key = signing_key.verifying_key();

            let signature = signing_key.sign(&data);
            let result = verifying_key.verify(&data, &signature);

            prop_assert!(result.is_ok(), "Verification should succeed");
        }

        /// Property: Ed25519 signatures are 64 bytes
        #[test]
        fn ed25519_signature_length(data in binary_data_strategy(1, 100)) {
            let signing_key = SigningKey::generate(&mut OsRng);
            let signature = signing_key.sign(&data);

            prop_assert_eq!(signature.to_bytes().len(), 64);
        }

        /// Property: Ed25519 public keys are 32 bytes
        #[test]
        fn ed25519_pubkey_length(_seed in any::<u64>()) {
            let signing_key = SigningKey::generate(&mut OsRng);
            let verifying_key = signing_key.verifying_key();

            prop_assert_eq!(verifying_key.to_bytes().len(), 32);
        }

        /// Property: Tampered data fails verification
        #[test]
        fn ed25519_tampered_fails(
            data in binary_data_strategy(1, 1000),
            flip_byte in any::<usize>()
        ) {
            let signing_key = SigningKey::generate(&mut OsRng);
            let verifying_key = signing_key.verifying_key();
            let signature = signing_key.sign(&data);

            // Tamper with data
            let mut tampered = data.clone();
            let idx = flip_byte % tampered.len();
            tampered[idx] ^= 0x01;

            let result = verifying_key.verify(&tampered, &signature);
            prop_assert!(result.is_err(), "Tampered data should fail verification");
        }

        /// Property: Different keypairs produce different signatures
        #[test]
        fn ed25519_different_keys_different_sigs(data in binary_data_strategy(1, 100)) {
            let key1 = SigningKey::generate(&mut OsRng);
            let key2 = SigningKey::generate(&mut OsRng);

            let sig1 = key1.sign(&data);
            let sig2 = key2.sign(&data);

            prop_assert_ne!(sig1.to_bytes(), sig2.to_bytes());
        }
    }
}

/// Nonce generation tests
mod nonce_tests {
    use super::*;
    use rand::RngCore;

    proptest! {
        /// Property: Generated nonces are unique (probabilistic)
        #[test]
        fn nonces_unique(count in 2usize..50) {
            let mut rng = rand::thread_rng();
            let nonces: Vec<Vec<u8>> = (0..count)
                .map(|_| {
                    let mut nonce = vec![0u8; 32];
                    rng.fill_bytes(&mut nonce);
                    nonce
                })
                .collect();

            let unique: std::collections::HashSet<_> = nonces.iter().collect();
            prop_assert_eq!(unique.len(), nonces.len(), "All nonces should be unique");
        }

        /// Property: Nonces have correct length
        #[test]
        fn nonce_length(size in 16usize..64) {
            let mut rng = rand::thread_rng();
            let mut nonce = vec![0u8; size];
            rng.fill_bytes(&mut nonce);

            prop_assert_eq!(nonce.len(), size);
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use sha2::{Sha256, Digest};

    #[test]
    fn test_sha256_empty() {
        let hash = Sha256::digest(b"");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_sha256_hello_world() {
        let hash = Sha256::digest(b"Hello, World!");
        let hex = hex::encode(hash);
        // Known SHA-256 of "Hello, World!"
        assert_eq!(hex, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }

    #[test]
    fn test_hex_encode_decode() {
        let data = b"test data";
        let encoded = hex::encode(data);
        let decoded = hex::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}

//! RFC 6962 §2.1.2 Merkle consistency-proof verification.
//!
//! Vendored from `ciris-verify-core::transparency` (the vetted, round-trip-
//! tested implementation Persist consumes) — Registry deliberately does NOT
//! pull `ciris-verify-core` (its rusqlite dep conflicts with sqlx-sqlite at
//! the libsqlite3-sys linker level; see `Cargo.toml`), so the small,
//! self-contained consistency verifier is vendored here under the same
//! precedent as `security::build_manifest`. Wire-format parity is held by
//! the known-answer test vectors below, generated independently (Python
//! `hashlib`) per RFC 6962 and anchored against the well-known empty-tree
//! MTH `e3b0c4…b855` and the canonical single-leaf vector `96a296d2…`.
//!
//! Used by `POST /v1/transparency/sth/cosign` to enforce CEG 0.2 §10.3.1:
//! a witness cosigning an STH MUST supply a consistency proof from the
//! prior STH it cosigned, and the Registry rejects the cosign if the proof
//! is absent or does not verify — making `witness_quorum_met` "quorum on
//! log consistency," not "quorum on a string."

use sha2::{Digest, Sha256};

/// Outcome of a consistency check, so the caller can map to the right
/// §10.0.1 error code.
#[derive(Debug, PartialEq, Eq)]
pub enum ConsistencyOutcome {
    /// Proof verifies: the new tree is an append-only extension of the old.
    Valid,
    /// Proof is well-formed in shape but does not reconstruct both roots.
    Invalid,
    /// Range is malformed (`old_size == 0`, or `old_size > new_size`).
    MalformedRange,
}

/// Hash leaf bytes under the RFC 6962 §2.1 leaf prefix (`0x00`).
pub fn hash_leaf(canonical: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update([0x00]);
    h.update(canonical);
    h.finalize().into()
}

/// Hash two node hashes under the RFC 6962 §2.1 internal-node prefix (`0x01`).
pub fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update([0x01]);
    h.update(left);
    h.update(right);
    h.finalize().into()
}

/// Largest power of 2 strictly less than `n` (`n > 1`).
fn largest_pow2_lt(n: usize) -> usize {
    debug_assert!(n > 1);
    let mut k = 1;
    while k * 2 < n {
        k *= 2;
    }
    k
}

/// A well-formed RFC 6962 consistency proof has at most ⌈log2(n)⌉+1 hashes;
/// `new_size` is u64 so 65 is a hard upper bound. Reject longer before
/// recursing so a hostile oversized proof can't drive work/allocation.
const MAX_CONSISTENCY_PROOF_HASHES: usize = 65;

/// Verify an RFC 6962 consistency proof that `(old_size, old_root)` is a
/// prefix of `(new_size, new_root)`.
///
/// Returns [`ConsistencyOutcome`]. `old_size == new_size` requires an empty
/// proof and `old_root == new_root`. `old_size == 0` / `old_size > new_size`
/// → [`ConsistencyOutcome::MalformedRange`].
pub fn verify_consistency(
    old_root: &[u8; 32],
    old_size: u64,
    new_root: &[u8; 32],
    new_size: u64,
    proof: &[[u8; 32]],
) -> ConsistencyOutcome {
    if old_size == 0 || old_size > new_size {
        return ConsistencyOutcome::MalformedRange;
    }
    if old_size == new_size {
        return if proof.is_empty() && old_root == new_root {
            ConsistencyOutcome::Valid
        } else {
            ConsistencyOutcome::Invalid
        };
    }
    if proof.len() > MAX_CONSISTENCY_PROOF_HASHES {
        return ConsistencyOutcome::Invalid;
    }

    let (m, n) = (old_size as usize, new_size as usize);

    // RFC 6962 §2.1.2: when `m` is a power of 2 the old tree is a complete
    // subtree of the new tree, so the SUBPROOF recursion bottoms out with an
    // empty leftmost element — seed that slot with `old_root`. When `m` is
    // not a power of 2 the proof carries everything.
    let seeded = if m.is_power_of_two() {
        Some(*old_root)
    } else {
        None
    };

    match reconstruct_roots(m, n, proof, seeded) {
        Some((computed_old, computed_new))
            if &computed_old == old_root && &computed_new == new_root =>
        {
            ConsistencyOutcome::Valid
        }
        _ => ConsistencyOutcome::Invalid,
    }
}

fn reconstruct_roots(
    m: usize,
    n: usize,
    proof: &[[u8; 32]],
    seeded_old_root: Option<[u8; 32]>,
) -> Option<([u8; 32], [u8; 32])> {
    let mut full: Vec<[u8; 32]> = Vec::with_capacity(proof.len() + 1);
    if let Some(seed) = seeded_old_root {
        full.push(seed);
    }
    full.extend_from_slice(proof);
    let mut it = full.into_iter();
    let (old_h, new_h) = reconstruct_recursive(m, n, &mut it)?;
    // leftover hashes => malformed proof
    if it.next().is_some() {
        return None;
    }
    Some((old_h, new_h))
}

fn reconstruct_recursive(
    m: usize,
    n: usize,
    hashes: &mut impl Iterator<Item = [u8; 32]>,
) -> Option<([u8; 32], [u8; 32])> {
    if m == n {
        let h = hashes.next()?;
        return Some((h, h));
    }
    let k = largest_pow2_lt(n);
    if m <= k {
        let (old_h, new_left) = reconstruct_recursive(m, k, hashes)?;
        let right = hashes.next()?;
        Some((old_h, hash_node(&new_left, &right)))
    } else {
        let (old_right, new_right) = reconstruct_recursive(m - k, n - k, hashes)?;
        let left = hashes.next()?;
        Some((hash_node(&left, &old_right), hash_node(&left, &new_right)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hx(s: &str) -> [u8; 32] {
        let v = hex::decode(s).unwrap();
        let mut a = [0u8; 32];
        a.copy_from_slice(&v);
        a
    }

    // ── Anchors: RFC 6962 hashing domain separation (independent of the
    //    consistency algorithm) ──────────────────────────────────────────
    #[test]
    fn anchor_single_leaf_mth() {
        // leaf(0x00) = SHA256(0x00 || 0x00) — the canonical CT vector.
        assert_eq!(
            hash_leaf(&[0u8]),
            hx("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7")
        );
    }

    // RFC 6962 MTH roots for the canonical 8-leaf tree (leaves = single
    // bytes 0x00..0x07), computed independently via Python hashlib.
    const ROOTS: &[(u64, &str)] = &[
        (1, "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        (2, "a20bf9a7cc2dc8a08f5f415a71b19f6ac427bab54d24eec868b5d3103449953a"),
        (3, "3b6cccd7e3e023ff393006f030315ee7ad9eb111b022b41fba7e5b7a3973f688"),
        (4, "9bcd51240af4005168f033121ba85be5a6ed4f0e6a5fac262066729b8fbfdecb"),
        (5, "b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f"),
        (6, "bb36e7d3d4cee5720cbd323d02fab15962e2ba1dadf5f8fc6eeef4fd6ad056a8"),
        (7, "3560191803028444b232018ac047fdb561c09c23a7a6876c85e08b5e4d48e9f3"),
        (8, "ef7f49b620f6c7ea9b963a214da34b5021c6ded8ed57734380a311ab726aa907"),
    ];

    fn root(n: u64) -> [u8; 32] {
        hx(ROOTS.iter().find(|(s, _)| *s == n).unwrap().1)
    }

    // (m, n, proof_hashes) — authoritative RFC 6962 consistency proofs,
    // generated independently (Python hashlib SUBPROOF).
    const PROOFS: &[(u64, u64, &[&str])] = &[
        (1, 2, &["b413f47d13ee2fe6c845b2ee141af81de858df4ec549a58b7970bb96645bc8d2"]),
        (2, 3, &["fcf0a6c700dd13e274b6fba8deea8dd9b26e4eedde3495717cac8408c9c5177f"]),
        (3, 4, &[
            "fcf0a6c700dd13e274b6fba8deea8dd9b26e4eedde3495717cac8408c9c5177f",
            "583c7dfb7b3055d99465544032a571e10a134b1b6f769422bbb71fd7fa167a5d",
            "a20bf9a7cc2dc8a08f5f415a71b19f6ac427bab54d24eec868b5d3103449953a",
        ]),
        (3, 7, &[
            "fcf0a6c700dd13e274b6fba8deea8dd9b26e4eedde3495717cac8408c9c5177f",
            "583c7dfb7b3055d99465544032a571e10a134b1b6f769422bbb71fd7fa167a5d",
            "a20bf9a7cc2dc8a08f5f415a71b19f6ac427bab54d24eec868b5d3103449953a",
            "89c929834ed1459b07f65b5e1a2143a8cf5d8efdf30f49ffffa328bb1d9133bb",
        ]),
        (4, 8, &["c1fe42b33ebb8e8a7e4a90abc481c7434e2be02cff2f6a18d7ffab4f1e25891b"]),
        (6, 8, &[
            "4b8c129ed14cce2c08cfc6766db7f8cdb133b5f698b8de3d5890ea7ff7f0a8d1",
            "bbb0feb32f648c73fe170518bcec1f675af1b780dc23d6fbf30b745c1ca5fa11",
            "9bcd51240af4005168f033121ba85be5a6ed4f0e6a5fac262066729b8fbfdecb",
        ]),
        (2, 5, &[
            "52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825",
            "4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4",
        ]),
        (1, 8, &[
            "b413f47d13ee2fe6c845b2ee141af81de858df4ec549a58b7970bb96645bc8d2",
            "52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825",
            "c1fe42b33ebb8e8a7e4a90abc481c7434e2be02cff2f6a18d7ffab4f1e25891b",
        ]),
        (7, 8, &[
            "40d88127d4d31a3891f41598eeed41174e5bc89b1eb9bbd66a8cbfc09956a3fd",
            "2ecd8a6b7d2845546659ad4cf443533cf921b19dc81fa83934e83821b4dfdcb7",
            "4b8c129ed14cce2c08cfc6766db7f8cdb133b5f698b8de3d5890ea7ff7f0a8d1",
            "9bcd51240af4005168f033121ba85be5a6ed4f0e6a5fac262066729b8fbfdecb",
        ]),
    ];

    fn proof_of(m: u64, n: u64) -> Vec<[u8; 32]> {
        PROOFS
            .iter()
            .find(|(a, b, _)| *a == m && *b == n)
            .unwrap()
            .2
            .iter()
            .map(|s| hx(s))
            .collect()
    }

    #[test]
    fn accepts_all_authoritative_proofs() {
        for &(m, n, _) in PROOFS {
            let p = proof_of(m, n);
            assert_eq!(
                verify_consistency(&root(m), m, &root(n), n, &p),
                ConsistencyOutcome::Valid,
                "({m},{n}) authoritative proof must verify"
            );
        }
    }

    #[test]
    fn rejects_tampered_proof_hash() {
        for &(m, n, _) in PROOFS {
            let mut p = proof_of(m, n);
            if p.is_empty() {
                continue;
            }
            p[0][0] ^= 0xff; // flip a bit in the first proof hash
            assert_eq!(
                verify_consistency(&root(m), m, &root(n), n, &p),
                ConsistencyOutcome::Invalid,
                "({m},{n}) tampered proof must be rejected"
            );
        }
    }

    #[test]
    fn rejects_wrong_new_root() {
        let p = proof_of(3, 7);
        // claim the proof proves consistency to root(8), not root(7)
        assert_eq!(
            verify_consistency(&root(3), 3, &root(8), 7, &p),
            ConsistencyOutcome::Invalid
        );
    }

    #[test]
    fn rejects_wrong_old_root() {
        let p = proof_of(3, 7);
        assert_eq!(
            verify_consistency(&root(4), 3, &root(7), 7, &p),
            ConsistencyOutcome::Invalid
        );
    }

    #[test]
    fn rejects_leftover_hashes() {
        let mut p = proof_of(4, 8);
        p.push([0x11u8; 32]); // extra trailing hash
        assert_eq!(
            verify_consistency(&root(4), 4, &root(8), 8, &p),
            ConsistencyOutcome::Invalid
        );
    }

    #[test]
    fn rejects_too_short_proof() {
        let mut p = proof_of(3, 7);
        p.pop();
        assert_eq!(
            verify_consistency(&root(3), 3, &root(7), 7, &p),
            ConsistencyOutcome::Invalid
        );
    }

    #[test]
    fn equal_sizes_require_empty_proof_and_equal_roots() {
        assert_eq!(
            verify_consistency(&root(5), 5, &root(5), 5, &[]),
            ConsistencyOutcome::Valid
        );
        assert_eq!(
            verify_consistency(&root(5), 5, &root(6), 5, &[]),
            ConsistencyOutcome::Invalid
        );
        assert_eq!(
            verify_consistency(&root(5), 5, &root(5), 5, &[[0u8; 32]]),
            ConsistencyOutcome::Invalid
        );
    }

    #[test]
    fn malformed_range() {
        assert_eq!(
            verify_consistency(&root(1), 0, &root(8), 8, &[]),
            ConsistencyOutcome::MalformedRange
        );
        assert_eq!(
            verify_consistency(&root(8), 8, &root(3), 3, &[]),
            ConsistencyOutcome::MalformedRange
        );
    }

    #[test]
    fn rejects_oversized_proof() {
        let big = vec![[0u8; 32]; MAX_CONSISTENCY_PROOF_HASHES + 1];
        assert_eq!(
            verify_consistency(&root(3), 3, &root(7), 7, &big),
            ConsistencyOutcome::Invalid
        );
    }
}

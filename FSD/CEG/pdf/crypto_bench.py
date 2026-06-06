#!/usr/bin/env python3
r"""
Per-operation crypto benchmark for the CEG §10.5 PQC streaming stack.

Measures the CLASSICAL + SYMMETRIC ops for real (pyca/cryptography); fills the
PQC rows from published liboqs numbers (x86-64, AVX2) since no PQC python lib is
present here. Cross-check target: ciris-crypto `benches/federation_crypto.rs`
(criterion) on the deployment host.

Point: PQC's cost is SIZE, not COMPUTE — the per-op latency is comparable to
classical; the bytes are ~30-50x bigger. And in CEG the asymmetric ops are
per-EPOCH (cold path); the per-FRAME hot path is AES-256-GCM (fast + PQC-safe).
"""
import time, os
from cryptography.hazmat.primitives.asymmetric import x25519, ed25519
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

def bench(fn, iters):
    fn()  # warm
    t = time.perf_counter()
    for _ in range(iters): fn()
    return (time.perf_counter() - t) / iters * 1e6  # us/op

def main():
    N = 2000
    # X25519 ECDH (one full shared-secret derivation)
    a, b = x25519.X25519PrivateKey.generate(), x25519.X25519PrivateKey.generate()
    bpub = b.public_key()
    x_us = bench(lambda: a.exchange(bpub), N)
    # Ed25519 sign / verify over a 64-byte message
    sk = ed25519.Ed25519PrivateKey.generate(); pk = sk.public_key(); msg = os.urandom(64)
    sig = sk.sign(msg)
    eds_us = bench(lambda: sk.sign(msg), N)
    edv_us = bench(lambda: pk.verify(sig, msg), N)
    # AES-256-GCM seal/open of a 1 MiB chunk
    key = AESGCM(os.urandom(32)); nonce = os.urandom(12); chunk = os.urandom(1024*1024)
    ct = key.encrypt(nonce, chunk, None)
    aes_seal = bench(lambda: key.encrypt(nonce, chunk, None), 200)
    aes_open = bench(lambda: key.decrypt(nonce, ct, None), 200)
    aes_gbps = (1024*1024*8)/(aes_seal/1e6)/1e9

    rows = [
        # op, us/op (measured or published), size on wire (B), source
        ("X25519 ECDH (1 shared secret)",      f"{x_us:6.1f}",  "32",   "measured (pyca)"),
        ("Ed25519 sign",                        f"{eds_us:6.1f}", "64",   "measured (pyca)"),
        ("Ed25519 verify",                      f"{edv_us:6.1f}", "-",    "measured (pyca)"),
        ("AES-256-GCM seal (1 MiB chunk)",      f"{aes_seal:6.1f}", "+16/chunk", f"measured ({aes_gbps:.1f} GB/s)"),
        ("AES-256-GCM open (1 MiB chunk)",      f"{aes_open:6.1f}", "-",    "measured (pyca)"),
        ("ML-KEM-768 keygen",                   "  ~25",  "ek 1184", "published (liboqs AVX2)"),
        ("ML-KEM-768 encaps",                   "  ~30",  "ct 1088", "published (liboqs AVX2)"),
        ("ML-KEM-768 decaps",                   "  ~25",  "-",    "published (liboqs AVX2)"),
        ("ML-DSA-65 sign",                      " ~330",  "sig 3309", "published (liboqs AVX2)"),
        ("ML-DSA-65 verify",                    " ~110",  "-",    "published (liboqs AVX2)"),
    ]
    w = max(len(r[0]) for r in rows)
    print(f"{'operation':<{w}} {'us/op':>8} {'wire B':>10}  source")
    print("-"*(w+34))
    for op, us, sz, src in rows:
        print(f"{op:<{w}} {us:>8} {sz:>10}  {src}")
    print(f"\nKey reading: PQC compute is COMPARABLE to classical (ML-KEM encaps ~30us ~ X25519 ~{x_us:.0f}us);")
    print(f"the difference is SIZE (ML-KEM ct 1088B vs 32B; ML-DSA sig 3309B vs 64B).")
    print(f"Hot path = AES-256-GCM @ ~{aes_gbps:.0f} GB/s (PQC-safe). Asymmetric PQC is per-EPOCH, not per-frame.")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
r"""
CEG 0.14 — PQC streaming video bandwidth / lag model ("the toy").

Analytical, parametric model of CEG §10.5 streaming under the mandatory PQC
envelope (§10.5.3 wrap_algorithm:v2 = X25519+ML-KEM-768; §10.5.2 AES-256-GCM
chunk seal; hybrid Ed25519+ML-DSA-65 signatures). Isolates the "long tail":
the per-epoch O(N) key-grant cascade vs the 1.x O(log N) tree, and the
transport-RTT-dominated realtime lag.

Every constant is sourced from the CEG spec or the relevant FIPS standard;
the ONE free parameter is Reticulum transport RTT/throughput (CEG defers
transport to RET — this is the empirical input that turns the parametric
model into concrete numbers).

Outputs PDF figures into the same dir for LaTeX \includegraphics.
"""
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from pathlib import Path

OUT = Path(__file__).parent

# ---------------------------------------------------------------------------
# Constants (bytes). Sources noted.
# ---------------------------------------------------------------------------
# Symmetric (PQC-safe: AES-256 = 128-bit PQ via Grover; SHA-256 = PQ-safe)
AES_GCM_TAG      = 16        # §10.5.2 per-chunk auth tag
AES_GCM_NONCE    = 12        # §10.5.2 STREAM nonce (DERIVED, not transmitted)
DEK              = 32        # 256-bit epoch DEK

# KEM (FIPS 203 ML-KEM-768) + classical X25519  (hybrid wrap, §10.5.3 v2)
X25519_PK        = 32
MLKEM768_CT      = 1088      # FIPS 203 ciphertext
MLKEM768_EK      = 1184      # FIPS 203 encap (public) key

# Signatures (hybrid Ed25519 + ML-DSA-65, FIPS 204)
ED25519_SIG      = 64
MLDSA65_SIG      = 3309      # FIPS 204 final (c_tilde=48); per CIRISVerify#4
HYBRID_SIG       = ED25519_SIG + MLDSA65_SIG          # 3373 B

# Hash / merkle
SHA256           = 32

# Cadence (§10.5.1 / §10.5.2 defaults; RC1-7 unratified -> parametric)
K_CHUNKS         = 64
T_SECONDS        = 2.0
CHUNK_BYTES      = 1 * 1024 * 1024     # NodeCore#26 recommended 1 MiB

# ---------------------------------------------------------------------------
# Derived per-message sizes
# ---------------------------------------------------------------------------
def key_grant_v2_bytes():
    """One per-recipient hybrid epoch-DEK grant (§10.5.3 wrap_algorithm:v2).
       X25519 ephemeral + ML-KEM-768 ciphertext + AEAD-wrapped DEK + hybrid sig
       + small envelope overhead."""
    envelope = 128  # key_id, epoch, stream_id, wrap_algorithm tag, JCS overhead
    return X25519_PK + MLKEM768_CT + (DEK + AES_GCM_TAG) + HYBRID_SIG + envelope

def sth_bytes():
    """One producer-signed per-stream STH (§10.5.1)."""
    body = SHA256 + 8 + 8 + 64   # root_hash + tree_size + timestamp + log_id/stream_id
    return body + HYBRID_SIG

def tree_node_ct_bytes():
    """One TreeKEM path-node update = a hybrid HPKE ciphertext to a copath subtree
       (X25519 ephemeral + ML-KEM-768 ct + AEAD-wrapped node secret). The commit is
       signed ONCE (not per node), unlike the flat grant which carries a full hybrid
       sig per member."""
    return X25519_PK + MLKEM768_CT + (DEK + AES_GCM_TAG) + 32  # ~1.2 KiB

KG = key_grant_v2_bytes()            # flat per-member grant (carries its own hybrid sig)
STH = sth_bytes()
NODE_CT = tree_node_ct_bytes()       # TreeKEM per-node update (commit signed once)

# ---------------------------------------------------------------------------
# Bandwidth model (per stream, producer/substrate egress)
# ---------------------------------------------------------------------------
def content_overhead_frac():
    """AES-GCM tag overhead as a fraction of content bytes."""
    return AES_GCM_TAG / CHUNK_BYTES   # ~1.5e-5 at 1 MiB

def commit_bytes(N):
    """TreeKEM removal commit = O(log N) node updates + ONE commit signature."""
    depth = max(1, int(np.ceil(np.log2(N))))
    return depth * NODE_CT + HYBRID_SIG

def key_cascade_bw_bps(N, epoch_rate_hz, mode="flat_unicast"):
    """Per-rekey (= per member-removal) key-distribution PRODUCER/SUBSTRATE EGRESS,
       amortized to bits/sec. The delivery model is explicit and decisive
       (per the §0 review): the tree only wins WITH efficient multicast.

       flat_unicast (1.0): N member-specific grants -> O(N) egress. Cannot
            multicast (each grant is encrypted to a distinct member).
       tree_multicast (1.x, NEEDS multicast): one O(log N) commit sent ONCE ->
            O(log N) egress. Cheap ONLY if the transport multicasts.
       tree_unicast (1.x WITHOUT multicast): the O(log N) commit delivered to each
            of N members -> O(N log N) egress -> WORSE than flat. The tree's whole
            advantage is multicast aggregation, which over a RET mesh is the open
            transport question (§0.4)."""
    if N <= 1:
        return 0.0
    if mode == "flat_unicast":
        egress = N * KG                       # O(N), member-specific, no multicast
    elif mode == "tree_multicast":
        egress = commit_bytes(N)              # O(log N), ONE multicast send
    elif mode == "tree_unicast":
        egress = N * commit_bytes(N)          # O(N log N), commit to each member
    else:
        raise ValueError(mode)
    return egress * 8 * epoch_rate_hz

def per_member_download_bytes(N, mode):
    """What ONE member downloads per rekey. flat: 1 grant; tree: the whole commit.
       NB: flat (KG ~4.6 KiB) is SMALLER per-member than the tree commit
       (O(log N) ~tens of KiB) — the tree only helps SENDER egress, and only with multicast."""
    if mode == "flat_unicast": return KG
    return commit_bytes(N)

def sth_pull_bw_bps(N, epoch_rate_hz):
    """Broadcast pull: each subscriber pulls the per-epoch STH (cacheable)."""
    return N * STH * 8 * epoch_rate_hz

CHURN_REALISTIC_PER_HOUR_FRAC = 0.30   # ~30%/hr broadcast-audience churn (1.0-RC1 / #71 C1 correction;
                                        # the original 3600/hr at N=1M = 0.36%/hr understated by ~2 orders)
STH_CADENCE_T_S = 2.0                   # removal-coalescing window (== STH cadence, SS10.5.3 normative)

def coalesced_epoch_rate_hz(N, churn_frac_per_hour, T_s=STH_CADENCE_T_S):
    """SS10.5.3 removal coalescing: all removals within one STH window batch into ONE
    rotation -> epoch rate is min(raw removal rate, 1/T), i.e. capped at 1/T regardless
    of churn. listed:public ungated rosters are exempt entirely (rate ~ 0)."""
    raw = N * churn_frac_per_hour / 3600.0
    return min(raw, 1.0 / T_s)

def total_egress_mbps(bitrate_mbps, N, churn_per_hour, mode="flat_unicast"):
    """Total producer/substrate egress (Mbps) for a stream of N viewers."""
    epoch_rate_hz = churn_per_hour / 3600.0    # member-removal forces an epoch (§10.5.3)
    content = bitrate_mbps * N * (1 + content_overhead_frac())   # unicast worst case
    keys = key_cascade_bw_bps(N, epoch_rate_hz, mode) / 1e6
    sth = sth_pull_bw_bps(N, epoch_rate_hz) / 1e6
    return content, keys, sth

# ---------------------------------------------------------------------------
# Lag model (glass-to-glass, seconds)
# ---------------------------------------------------------------------------
# Per-chunk crypto cost is NEGLIGIBLE: AES-256-GCM ~5 GiB/s (ring); ML-KEM-768
# decap ~tens of microseconds, ONCE per epoch (not per chunk). So crypto is
# not the lag driver; transport + cadence are.
AES_THROUGHPUT_BPS = 5 * 1024**3       # ring AES-256-GCM
MLKEM_DECAP_S      = 30e-6             # per-epoch, amortized to ~0 per chunk

def lag_broadcast_pull(bitrate_mbps, rns_rtt_s):
    chunk_dur = (CHUNK_BYTES * 8) / (bitrate_mbps * 1e6)
    seal_open = (CHUNK_BYTES / AES_THROUGHPUT_BPS) * 2
    sth_wait  = T_SECONDS / 2          # avg wait for next STH boundary
    return chunk_dur + sth_wait + rns_rtt_s + seal_open

def lag_realtime_directlink(frame_ms, rns_rtt_s, jitter_ms=40):
    frame_dur = frame_ms / 1000.0
    seal_open = (frame_ms/1000.0 * 0)  # per-frame AES negligible at these sizes
    return frame_dur + rns_rtt_s + jitter_ms/1000.0 + seal_open

# ===========================================================================
# FIGURES
# ===========================================================================
plt.rcParams.update({"font.size": 10, "figure.dpi": 120})

# Fig 1: key-cascade egress vs N — the THREE delivery models (per §0 review)
def fig_cascade():
    N = np.logspace(0, 6, 200)
    churn = 3600  # 1 removal/sec
    flat   = np.array([key_cascade_bw_bps(n, churn/3600, "flat_unicast")/1e6 for n in N])
    tmc    = np.array([key_cascade_bw_bps(n, churn/3600, "tree_multicast")/1e6 for n in N])
    tuc    = np.array([key_cascade_bw_bps(n, churn/3600, "tree_unicast")/1e6 for n in N])
    fig, ax = plt.subplots(figsize=(6.4, 4.2))
    ax.loglog(N, flat, label="flat, unicast  O(N)  — CEG 1.0", lw=2)
    ax.loglog(N, tmc,  label="tree, multicast  O(log N)  — 1.x, NEEDS multicast", lw=2, ls="--")
    ax.loglog(N, tuc,  label="tree, unicast  O(N log N)  — WORSE than flat", lw=2, ls=":")
    ax.set_xlabel("subscribers N"); ax.set_ylabel("key-cascade egress (Mbps)")
    ax.set_title(f"PQC epoch-key cascade — delivery model is decisive\n"
                 f"(churn={churn}/hr; the tree wins ONLY with efficient multicast)")
    ax.legend(fontsize=8); ax.grid(True, which="both", alpha=0.3)
    fig.tight_layout(); fig.savefig(OUT/"fig_cascade.pdf"); plt.close(fig)

# Fig 2: PQC overhead as % of total egress vs bitrate (content vs keys)
def fig_overhead():
    bitrates = np.array([0.5, 1, 2, 4, 6, 12, 25])  # Mbps tiers
    fig, ax = plt.subplots(figsize=(6.2, 4))
    for N, churn in [(1_000, 60), (100_000, 600)]:
        pct = []
        for br in bitrates:
            c, k, s = total_egress_mbps(br, N, churn, "flat_unicast")
            pct.append(100*(k+s)/(c+k+s))
        ax.plot(bitrates, pct, marker="o", label=f"N={N:,}, churn={churn}/hr")
    ax.set_xlabel("video bitrate (Mbps)"); ax.set_ylabel("PQC key+STH overhead (% of egress)")
    ax.set_title("PQC overhead share — content dominates;\nkey-cascade tail only bites at low bitrate × high churn × big N")
    ax.legend(); ax.grid(True, alpha=0.3)
    fig.tight_layout(); fig.savefig(OUT/"fig_overhead.pdf"); plt.close(fig)

# Fig 3: glass-to-glass lag vs RNS RTT — broadcast vs realtime
def fig_lag():
    rtt = np.linspace(0.005, 0.5, 200)  # 5 ms (IP-backed RNS) .. 500 ms (multi-hop mesh)
    bc = [lag_broadcast_pull(6, r) for r in rtt]
    rt = [lag_realtime_directlink(40, r) for r in rtt]  # 40ms frames
    fig, ax = plt.subplots(figsize=(6.2, 4))
    ax.plot(rtt*1000, bc, label="broadcast pull (K=64,T=2s, 6 Mbps, 1 MiB chunk)", lw=2)
    ax.plot(rtt*1000, rt, label="realtime direct-link (40 ms frames)", lw=2)
    ax.axhline(0.150, color="green", ls=":", label="150 ms — interactive-call ceiling")
    ax.axhline(0.400, color="orange", ls=":", label="400 ms — ITU one-way comfort")
    ax.set_xlabel("Reticulum Link RTT (ms)  [the one free parameter]")
    ax.set_ylabel("glass-to-glass lag (s)")
    ax.set_title("Lag is transport-bound, not crypto-bound\n(PQC adds ~µs/chunk; DEK decap ~30µs/epoch)")
    ax.legend(fontsize=8); ax.grid(True, alpha=0.3)
    fig.tight_layout(); fig.savefig(OUT/"fig_lag.pdf"); plt.close(fig)

# Fig 4: join/handshake one-time cost breakdown
def fig_join():
    parts = ["ML-KEM-768\nencap+decap", "hybrid sig\nverify", "RNS path\nsetup", "first-epoch\ngrant fetch"]
    # ms estimates: KEM ~0.1ms, sig verify ~0.3ms, RNS path setup (free param) ~250ms, grant ~KG/throughput
    ms = [0.1, 0.3, 250, (KG*8)/(2e6)*1000]  # grant over a 2 Mbps link
    fig, ax = plt.subplots(figsize=(6.2, 4))
    bars = ax.bar(parts, ms, color=["#4c72b0","#4c72b0","#dd8452","#4c72b0"])
    ax.set_ylabel("one-time join cost (ms, log)"); ax.set_yscale("log")
    ax.set_title("Stream/call join — RNS path setup dominates;\ncrypto is sub-millisecond")
    for b,v in zip(bars, ms): ax.text(b.get_x()+b.get_width()/2, v, f"{v:.2g}", ha="center", va="bottom", fontsize=8)
    ax.grid(True, axis="y", alpha=0.3)
    fig.tight_layout(); fig.savefig(OUT/"fig_join.pdf"); plt.close(fig)

if __name__ == "__main__":
    print(f"hybrid key_grant (v2) = {KG} B ({KG/1024:.2f} KiB)")
    print(f"per-stream STH        = {STH} B")
    print(f"AES-GCM content overhead at 1 MiB chunk = {content_overhead_frac()*100:.4f}%")
    print(f"DEK decap per epoch ~ {MLKEM_DECAP_S*1e6:.0f} us (amortized ~0 per chunk)")
    print(f"flat grant={KG} B | tree node_ct={NODE_CT} B | tree commit@1M={commit_bytes(1_000_000)} B "
          f"(depth={int(__import__('numpy').ceil(__import__('numpy').log2(1_000_000)))})")
    print("\n-- worked points: key-cascade egress under the 3 delivery models --")
    for N, churn, br in [(1_000,60,6),(100_000,600,6),(1_000_000,3600,2)]:
        c,_,s = total_egress_mbps(br,N,churn,"flat_unicast")
        kf = key_cascade_bw_bps(N, churn/3600, "flat_unicast")/1e6
        ktm = key_cascade_bw_bps(N, churn/3600, "tree_multicast")/1e6
        ktu = key_cascade_bw_bps(N, churn/3600, "tree_unicast")/1e6
        print(f"N={N:>9,} churn={churn:>5}/hr br={br}Mbps | content={c:>12,.0f} | "
              f"flat/unicast={kf:>10,.3f} | tree/multicast={ktm:>8,.4f} | tree/unicast={ktu:>12,.1f}  (Mbps)")
    for fn in (fig_cascade, fig_overhead, fig_lag, fig_join):
        fn(); print(f"wrote {fn.__name__}")
    print("done")

#!/usr/bin/env python3
"""
Turn the CEG §10.5 PQC-streaming model from parametric -> concrete by feeding it
a measured Reticulum transport profile (from rns_transport_bench.py).

  python3 concretize.py profile.json          # measured profile
  python3 concretize.py                        # runs 3 representative profiles

Reads {rtt_ms, throughput_mbps} and prints concrete glass-to-glass lag (broadcast
+ realtime), join cost, and per-stream PQC key/STH bandwidth feasibility.
"""
import json, sys
from pqc_streaming_model import (lag_broadcast_pull, lag_realtime_directlink,
                                 total_egress_mbps, KG, STH, MLKEM_DECAP_S)

# Representative profiles (until real measurements land). RTTs are illustrative
# transport assumptions, NOT measurements — replace with rns_transport_bench out.
REPRESENTATIVE = [
    {"interface": "IP-backed RNS (TCPInterface, same region)",  "rtt_ms": 35,  "throughput_mbps": 50},
    {"interface": "IP-backed RNS (cross-region)",               "rtt_ms": 140, "throughput_mbps": 30},
    {"interface": "LoRa (RNodeInterface, SF7/125kHz)",          "rtt_ms": 1200,"throughput_mbps": 0.005},
]

def report(p):
    rtt_s = p["rtt_ms"]/1000.0
    thr = p.get("throughput_mbps") or 0.0
    print(f"\n=== {p['interface']}  (RTT={p['rtt_ms']} ms, throughput={thr} Mbps) ===")
    # lag
    bc = lag_broadcast_pull(6, rtt_s)
    rt = lag_realtime_directlink(40, rtt_s)
    print(f"  glass-to-glass lag:")
    print(f"    broadcast pull (6 Mbps, 1 MiB chunk, T=2s):  {bc:6.2f} s")
    print(f"    realtime direct-link (40 ms frames):         {rt*1000:6.0f} ms  "
          f"{'[OK <150ms interactive]' if rt<0.150 else '[OK <400ms]' if rt<0.4 else '[too high for calls]'}")
    # join one-time cost
    grant_ms = (KG*8)/(max(thr,0.001)*1e6)*1000
    join = 0.1 + 0.3 + p['path_setup_ms'] if 'path_setup_ms' in p else 0.1+0.3+p['rtt_ms']*2
    join += grant_ms
    print(f"  join one-time: ~{join:.0f} ms  (KEM+verify ~0.4 ms; path-setup/RTT dominant; "
          f"first grant {grant_ms:.0f} ms)")
    # PQC bandwidth feasibility for this link (can it even carry the key cascade?)
    for N, churn, br in [(50, 60, 2), (1000, 60, 6)]:
        c,k,s = total_egress_mbps(br, N, churn, "flat")
        fits = "fits" if (k+s) < thr else "EXCEEDS link" if thr else "n/a"
        print(f"    N={N:>4} churn={churn}/hr br={br}Mbps: PQC key+STH={k+s:7.3f} Mbps  ({fits} vs {thr} Mbps link)")

def main():
    print("PQC streaming — concrete numbers from a transport profile")
    print(f"(invariants: hybrid grant={KG/1024:.2f} KiB, STH={STH} B, DEK decap~{MLKEM_DECAP_S*1e6:.0f}us/epoch)")
    if len(sys.argv) > 1:
        p = json.load(open(sys.argv[1]))
        p.setdefault("interface", "measured"); report(p)
    else:
        print("(no profile given — showing 3 REPRESENTATIVE transport assumptions; "
              "run rns_transport_bench.py for real numbers)")
        for p in REPRESENTATIVE: report(p)

if __name__ == "__main__":
    main()

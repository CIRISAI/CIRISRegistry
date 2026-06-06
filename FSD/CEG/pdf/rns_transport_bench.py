#!/usr/bin/env python3
"""
RNS transport benchmark — measures the ONE free parameter of the CEG §10.5
PQC-streaming model: Reticulum Link RTT / throughput / path-setup.

CEG defers transport to RET, so the streaming lag/bandwidth model is parametric
on these until measured. Run this on the target deployment interface(s) (a
CIRISEdge node) to turn the model parametric -> concrete. Output is a JSON
profile consumable by `concretize.py`.

Modes
-----
  loopback : single process, two local destinations — measures the RNS stack +
             Link + Resource FLOOR (no wire latency). Good for a quick sanity
             floor and CI.
  server   : run on host A; prints its destination hash and serves echo + a
             throughput sink. Keep running.
  client   : run on host B with the server's hash — measures real over-the-wire
             path-setup, Link RTT, and Resource throughput across whatever RNS
             interface connects them (TCP, I2P, LoRa, packet radio, ...).

Usage
-----
  python3 rns_transport_bench.py --mode loopback [--mib 8] [--out profile.json]
  python3 rns_transport_bench.py --mode server   [--config ~/.reticulum]
  python3 rns_transport_bench.py --mode client --peer <DEST_HASH_HEX> --mib 8 --out profile.json

Requires: pip install rns   (https://reticulum.network)
"""
import argparse, json, sys, time, threading

APP = "ciris.ceg.bench"

def _need_rns():
    try:
        import RNS  # noqa
        return RNS
    except Exception:
        sys.stderr.write(
            "Reticulum (rns) not installed on this host.\n"
            "  pip install rns   # then run on a node attached to the target interface\n"
            "This harness measures the real transport; it must run where RNS + the\n"
            "interface (TCP/LoRa/...) live — typically a CIRISEdge node.\n")
        sys.exit(2)

def _now():  # monotonic ms
    return time.monotonic() * 1000.0

# --------------------------------------------------------------------------
def run_loopback(mib, out):
    RNS = _need_rns()
    r = RNS.Reticulum()                      # local instance (default config)
    server_id = RNS.Identity()
    server = RNS.Destination(server_id, RNS.Destination.IN, RNS.Destination.SINGLE, APP, "echo")

    established = {"t": None, "link": None}
    def on_link(link):
        established["t"] = _now(); established["link"] = link
        link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
    server.set_link_established_callback(on_link)

    # client links to the (locally-known) server destination
    t0 = _now()
    link = RNS.Link(server)
    # wait for establishment
    for _ in range(200):
        if link.status == RNS.Link.ACTIVE: break
        time.sleep(0.02)
    if link.status != RNS.Link.ACTIVE:
        print(json.dumps({"error": "link did not establish (loopback)"})); return
    path_setup_ms = _now() - t0
    # RNS measures RTT during establishment
    time.sleep(0.2)
    rtt_ms = (link.rtt or 0) * 1000.0

    # throughput via a Resource transfer of `mib` MiB
    payload = b"\x00" * (mib * 1024 * 1024)
    done = {"t": None}
    def on_done(res): done["t"] = _now()
    ts = _now()
    res = RNS.Resource(payload, link, callback=on_done)
    for _ in range(2000):
        if done["t"]: break
        time.sleep(0.01)
    thr_mbps = (mib * 8) / (((done["t"] or _now()) - ts) / 1000.0) if done["t"] else None

    profile = {"mode": "loopback", "interface": "local (RNS stack floor)",
               "rtt_ms": round(rtt_ms, 3), "path_setup_ms": round(path_setup_ms, 1),
               "throughput_mbps": round(thr_mbps, 2) if thr_mbps else None,
               "note": "FLOOR only — no wire latency; add the real interface RTT for deployment numbers"}
    print(json.dumps(profile, indent=2))
    if out: open(out, "w").write(json.dumps(profile, indent=2))

def run_server(config):
    RNS = _need_rns()
    RNS.Reticulum(config)
    sid = RNS.Identity()
    dest = RNS.Destination(sid, RNS.Destination.IN, RNS.Destination.SINGLE, APP, "echo")
    def on_link(link):
        link.set_resource_strategy(RNS.Link.ACCEPT_ALL)
        link.set_packet_callback(lambda data, pkt: RNS.Packet(link, data).send())  # echo for RTT
    dest.set_link_established_callback(on_link)
    dest.announce()
    print(f"server destination hash: {RNS.prettyhexrep(dest.hash)}")
    print("serving (echo + resource sink). Ctrl-C to stop.")
    while True: time.sleep(1)

def run_client(peer_hex, mib, out, config):
    RNS = _need_rns()
    RNS.Reticulum(config)
    peer = bytes.fromhex(peer_hex.replace("<","").replace(">",""))
    if not RNS.Transport.has_path(peer):
        RNS.Transport.request_path(peer)
        t0 = _now()
        while not RNS.Transport.has_path(peer) and _now()-t0 < 15000: time.sleep(0.05)
    sid = RNS.Identity.recall(peer)
    sdest = RNS.Destination(sid, RNS.Destination.OUT, RNS.Destination.SINGLE, APP, "echo")
    t0 = _now(); link = RNS.Link(sdest)
    while link.status != RNS.Link.ACTIVE and _now()-t0 < 15000: time.sleep(0.02)
    path_setup_ms = _now() - t0
    time.sleep(0.3); rtt_ms = (link.rtt or 0)*1000.0
    payload = b"\x00"*(mib*1024*1024); done={"t":None}
    ts=_now(); RNS.Resource(payload, link, callback=lambda r: done.__setitem__("t",_now()))
    while not done["t"] and _now()-ts < 120000: time.sleep(0.01)
    thr = (mib*8)/(((done["t"] or _now())-ts)/1000.0) if done["t"] else None
    profile={"mode":"client","interface":"over-the-wire (deployment)","peer":peer_hex,
             "rtt_ms":round(rtt_ms,3),"path_setup_ms":round(path_setup_ms,1),
             "throughput_mbps":round(thr,2) if thr else None}
    print(json.dumps(profile, indent=2))
    if out: open(out,"w").write(json.dumps(profile, indent=2))

if __name__ == "__main__":
    ap = argparse.ArgumentParser(description="RNS transport benchmark for the CEG streaming model")
    ap.add_argument("--mode", choices=["loopback","server","client"], required=True)
    ap.add_argument("--peer", help="server destination hash (client mode)")
    ap.add_argument("--mib", type=int, default=8, help="payload size for throughput (MiB)")
    ap.add_argument("--config", default=None, help="RNS config dir")
    ap.add_argument("--out", default=None, help="write JSON profile to this path")
    a = ap.parse_args()
    if a.mode == "loopback": run_loopback(a.mib, a.out)
    elif a.mode == "server": run_server(a.config)
    else:
        if not a.peer: ap.error("client mode needs --peer <DEST_HASH>")
        run_client(a.peer, a.mib, a.out, a.config)

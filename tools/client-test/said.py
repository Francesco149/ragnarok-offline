#!/usr/bin/env python3
"""Decode what NPCs told the client from the packet log (dev.toml: packet_log).

    said.py [N [FILE]]   the last N NPC-related packets of logs/packets.log (default 30)
    said.py --state      what the NPC window waits for: next, close, menu, number, text, idle

Shows dialogue text (0x00b4), next/close buttons, menus, input boxes, cut-ins,
map changes, item gains and broadcasts: enough to follow a quest without a
screenshot of every page.
"""
import struct, sys

def z(b):
    return b.split(b"\0")[0].decode("cp1252", "replace")

def show(op, d):
    if op == 0x00b4: return "  | " + z(d[8:])
    if op == 0x00b5: return "  [next]"
    if op == 0x00b6: return "  [close]"
    if op == 0x00b7: return "  MENU: " + " / ".join(x for x in z(d[8:]).split(":") if x)
    if op == 0x00b8: return f"  > picked {d[6]}"
    if op == 0x00b9: return "  > next"
    if op == 0x0146: return "  > close"
    if op == 0x0142: return "  [input number]"
    if op == 0x01d4: return "  [input text]"
    if op == 0x0143: return f"  > number {struct.unpack_from('<I', d, 6)[0]}"
    if op == 0x01d5: return "  > text " + z(d[8:])
    if op == 0x0145: return f"  cutin {z(d[2:18])!r} {d[18]}"
    if op == 0x0091: return "  MAP -> %s %d,%d" % (z(d[2:18]), *struct.unpack_from("<HH", d, 18))
    if op == 0x00a0:
        idx, amount, nameid = struct.unpack_from("<HHH", d, 2)
        return f"  +item {nameid} x{amount}" + (" (failed)" if d[22] else "")
    if op == 0x00af: return "  -item idx %d x%d" % struct.unpack_from("<HH", d, 2)
    if op == 0x009a: return "  BROADCAST: " + z(d[4:])
    if op == 0x008e: return "  SELF: " + z(d[4:])
    if op == 0x00c0: return "  emotion %d" % d[6]
    if op == 0x0090: return "> talk"
    if op == 0x00c4: return "  [shop: buy or sell]"
    if op == 0x00b1:
        t, v = struct.unpack_from("<Hi", d, 2)
        return {20: f"  zeny {v}"}.get(t)
    if op == 0x00c3 and d[6] == 0: return None
    return None

def state():
    """What the client's NPC window waits for: next, close, menu, number, text, or idle."""
    st = "idle"
    for line in open("logs/packets.log", errors="replace"):
        p = line.split()
        if len(p) < 5 or p[1] not in "<>":
            continue
        op = int(p[2], 16)
        if p[1] == ">":
            st = {0xb5: "next", 0xb6: "close", 0xb7: "menu", 0x142: "number", 0x1d4: "text"}.get(op, "idle" if op == 0x91 else st)
        elif op in (0xb8, 0xb9, 0x146, 0x143, 0x1d5):
            st = "idle"
    print(st)

def main():
    if sys.argv[1:] == ["--state"]:
        return state()
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 30
    out = []
    for line in open(sys.argv[2] if len(sys.argv) > 2 else "logs/packets.log", errors="replace"):
        p = line.split()
        if len(p) < 5 or p[1] not in "<>":
            continue
        try:
            s = show(int(p[2], 16), bytes.fromhex(p[4]))
        except Exception:
            continue
        if s:
            out.append(s)
    print("\n".join(out[-n:]))

main()

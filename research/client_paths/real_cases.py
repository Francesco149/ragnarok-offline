# real_cases.py GRF OUT.json SEED -> windows of real maps (uncommitted: Gravity's data)
import json, random, struct, sys, collections
sys.path.insert(0, __file__.rsplit("/", 2)[0])
import grf_peek, zlib
grf, out, seed = sys.argv[1], sys.argv[2], int(sys.argv[3])
R = random.Random(seed)
gats = {}
odd = collections.Counter()
with open(grf, "rb") as f:
    for n, cs, csa, rs, flags, pos in grf_peek.entries(grf):
        if n.lower().endswith(".gat") and not flags & 6:
            f.seek(pos); gats[n] = zlib.decompress(f.read(csa))[:rs]
print("gat files", len(gats))
for n, b in gats.items():
    w, h = struct.unpack_from("<II", b, 6)
    for i in range(w * h):
        t = struct.unpack_from("<I", b, 14 + i * 20 + 16)[0]
        if t not in (0, 1, 2, 3, 4, 5, 6) and (t & 0xff) in (1, 5):
            odd[(n, t)] += 1
print("types whose low byte is 1 or 5 but are not:", dict(list(odd.items())[:10]), "total", sum(odd.values()))
names = sorted(gats)
cases = []
for k in range(160):
    n = R.choice(names); b = gats[n]
    w, h = struct.unpack_from("<II", b, 6)
    W, H = min(w, R.randint(30, 64)), min(h, R.randint(30, 64))
    ox, oy = R.randint(0, w - W), R.randint(0, h - H)
    cells = []
    for y in range(oy, oy + H):
        for x in range(ox, ox + W):
            t = struct.unpack_from("<I", b, 14 + (y * w + x) * 20 + 16)[0]
            cells.append("1" if t in (1, 5) else "0")
    free = [i for i, c in enumerate(cells) if c == "0"]
    if len(free) < 20: continue
    pairs = []
    for _ in range(40):
        s, t = R.choice(free), R.choice(free)
        pairs.append([s % W, s // W, t % W, t // W])
    cases.append({"map": n, "w": W, "h": H, "cells": "".join(cells), "pairs": pairs})
json.dump(cases, open(out, "w"))
print("cases", len(cases))

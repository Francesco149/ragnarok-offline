# to_fixture.py EMU_OUT.json FIXTURE.json -> compact form for crates/ro-game/tests/client_paths.rs
# Rows: [sx, sy, tx, ty, dirs] or, for pairs that carry a sub-cell, speed and
# start tick, [sx, sy, tx, ty, subx, suby, speed, start, dirs, [cell times]].
import json, sys
c = json.load(open(sys.argv[1]))
out = []
for k in c:
    rows = []
    for pair, p in zip(k["pairs"], k["paths"]):
        dirs = None if p is None else "".join(str(e[2]) for e in p[1:])
        if len(pair) >= 8:
            rows.append(list(pair[:8]) + [dirs, None if p is None else [e[3] for e in p]])
        else:
            rows.append(list(pair[:4]) + [dirs])
    out.append({"w": k["w"], "h": k["h"], "cells": k["cells"], "pairs": rows})
with open(sys.argv[2], "w") as f:
    f.write("[\n" + ",\n".join(json.dumps(x, separators=(",", ":")) for x in out) + "\n]\n")

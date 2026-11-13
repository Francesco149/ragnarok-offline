# to_fixture.py EMU_OUT.json FIXTURE.json -> compact form for crates/ro-game/tests/client_paths.rs
import json, sys
c = json.load(open(sys.argv[1]))
out = []
for k in c:
    rows = []
    for (sx, sy, tx, ty), p in zip(k["pairs"], k["paths"]):
        rows.append([sx, sy, tx, ty, None if p is None else "".join(str(e[2]) for e in p[1:])])
    out.append({"w": k["w"], "h": k["h"], "cells": k["cells"], "pairs": rows})
with open(sys.argv[2], "w") as f:
    f.write("[\n" + ",\n".join(json.dumps(x, separators=(",", ":")) for x in out) + "\n]\n")

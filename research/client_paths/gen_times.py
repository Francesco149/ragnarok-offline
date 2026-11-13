# gen_times.py CASES.json OUT.json SEED  -> the same grids and pairs, each pair
# given a random sub-cell, speed and start tick for emu_path.py, so the times
# the client gives a path's cells (0x4d2cf0) can be checked.
import json, math, random, struct, sys
cases, out, seed = json.load(open(sys.argv[1])), sys.argv[2], int(sys.argv[3])
R = random.Random(seed)
SPEEDS = [150, 200, 100, 75, 50, 300, 400, 112, 165, 125, 60, 1000]
for c in cases:
    rows = []
    for p in c["pairs"]:
        sub = (8, 8) if R.random() < 0.3 else (R.randrange(16), R.randrange(16))
        speed = R.choice(SPEEDS) if R.random() < 0.5 else R.randint(1, 1000)
        start = R.choice((0, R.randrange(1 << 32), (1 << 32) - R.randint(1, 5000)))
        rows.append(list(p[:4]) + [sub[0], sub[1], speed, start])
    c["pairs"] = rows

# An open grid with the first steps where a float sqrt (sqrtf at 0x4085a0),
# a double one, or the product rounded to a float (the x87 at 24 bits) give
# different times, so the fixture tells them apart.
f = lambda v: struct.unpack("<f", struct.pack("<f", v))[0]
edge = []
for sx, sy in ((x, y) for x in range(16) for y in range(16)):
    for dx, dy in ((1, 1), (-1, 1), (1, 0), (0, -1), (-1, -1)):
        ex, ey = f(6 + dx - f(5.5 + sx / 16)), f(6 + dy - f(5.5 + sy / 16))
        d2 = f(ex * ex + ey * ey)
        for speed in range(1, 1001):
            t = {int(speed * f(math.sqrt(d2))), int(speed * math.sqrt(d2)), int(f(speed * f(math.sqrt(d2))))}
            if len(t) > 1:
                edge.append([6, 6, 6 + 4 * dx, 6 + 4 * dy, sx, sy, speed, 0])
cases.append({"w": 13, "h": 13, "cells": "0" * 169, "pairs": edge})
json.dump(cases, open(out, "w"))

# gen_cases.py OUT.json SEED N  -> synthetic grids with start/target pairs
import json, random, sys
out, seed, n = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
R = random.Random(seed)
cases = []
for k in range(n):
    w, h = R.randint(16, 48), R.randint(16, 48)
    kind = k % 4
    g = [[0] * w for _ in range(h)]
    if kind == 0:   # scattered blocks
        d = R.uniform(0.05, 0.4)
        for y in range(h):
            for x in range(w):
                g[y][x] = 1 if R.random() < d else 0
    elif kind == 1:  # walls with gaps
        for _ in range(R.randint(2, 8)):
            if R.random() < 0.5:
                x = R.randrange(w); y0 = R.randrange(h); y1 = min(h, y0 + R.randint(3, h))
                for y in range(y0, y1): g[y][x] = 1
            else:
                y = R.randrange(h); x0 = R.randrange(w); x1 = min(w, x0 + R.randint(3, w))
                for x in range(x0, x1): g[y][x] = 1
    elif kind == 2:  # rectangles (buildings) and diagonal lines
        for _ in range(R.randint(2, 10)):
            x0, y0 = R.randrange(w), R.randrange(h)
            for y in range(y0, min(h, y0 + R.randint(1, 8))):
                for x in range(x0, min(w, x0 + R.randint(1, 8))): g[y][x] = 1
        for _ in range(R.randint(0, 3)):
            x, y, sx, sy = R.randrange(w), R.randrange(h), R.choice((-1, 1)), R.choice((-1, 1))
            for _ in range(R.randint(3, 15)):
                if 0 <= x < w and 0 <= y < h: g[y][x] = 1
                x += sx; y += sy
    else:            # U-shaped traps and open field
        for _ in range(R.randint(1, 4)):
            cx, cy, r = R.randrange(w), R.randrange(h), R.randint(2, 7)
            for i in range(-r, r + 1):
                for (x, y) in ((cx + i, cy + r), (cx - r, cy + i), (cx + r, cy + i)):
                    if 0 <= x < w and 0 <= y < h: g[y][x] = 1
    free = [(x, y) for y in range(h) for x in range(w) if g[y][x] == 0]
    pairs = []
    for _ in range(40):
        sx, sy = R.choice(free)
        if R.random() < 0.85:
            tx, ty = R.choice(free)
        else:  # sometimes a blocked target or the same cell
            tx, ty = (sx, sy) if R.random() < 0.3 else (R.randrange(w), R.randrange(h))
        pairs.append([sx, sy, tx, ty])
    cases.append({"w": w, "h": h, "cells": "".join("".join(map(str, row)) for row in g), "pairs": pairs})
json.dump(cases, open(out, "w"))

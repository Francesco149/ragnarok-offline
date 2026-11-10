#!/usr/bin/env python3
import struct
import zlib
import sys

sys.path.insert(0, "/work/research")
import grf_peek

GRF = "/data/play/RagnarokOnline/data.grf.original"

data = None
for n, cs, csa, rs, flags, pos in grf_peek.entries(GRF):
    if n.lower() == "data\\job_wizard.gat":
        with open(GRF, "rb") as f:
            f.seek(pos)
            data = zlib.decompress(f.read(csa))[:rs]
        break

assert data and data[:4] == b"GRAT"
w, h = struct.unpack_from("<ii", data, 6)

def kind(x, y):
    return struct.unpack_from("<I", data, 14 + (y * w + x) * 20 + 16)[0]

def ok(x, y):
    return 0 <= x < w and 0 <= y < h and kind(x, y) not in (1, 5)

# Find connected components of walkable cells
visited = set()
components = []

for y in range(h):
    for x in range(w):
        if ok(x, y) and (x, y) not in visited:
            comp = []
            queue = [(x, y)]
            visited.add((x, y))
            while queue:
                cx, cy = queue.pop()
                comp.append((cx, cy))
                for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                    nx, ny = cx + dx, cy + dy
                    if ok(nx, ny) and (nx, ny) not in visited:
                        visited.add((nx, ny))
                        queue.append((nx, ny))
            components.append(comp)

components.sort(key=lambda c: len(c), reverse=True)
print(f"Total walkable components: {len(components)}")
for i, c in enumerate(components[:10]):
    min_x = min(x for x, y in c)
    max_x = max(x for x, y in c)
    min_y = min(y for x, y in c)
    max_y = max(y for x, y in c)
    center_x = sum(x for x, y in c) // len(c)
    center_y = sum(y for x, y in c) // len(c)
    print(f"Room {i+1}: {len(c)} cells, bounds X:[{min_x}..{max_x}] Y:[{min_y}..{max_y}], center: ({center_x}, {center_y})")

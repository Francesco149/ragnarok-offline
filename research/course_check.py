#!/usr/bin/env python3
"""Can the Swordman test's course be walked on THIS client's map?

  course_check.py <data.grf>

The job-quest test (crates/ro-game/tests/job_quests.rs) plays on all-walkable
synthetic maps, so it cannot tell. This reads job_sword1.gat from the GRF and
the pit cells from the imported script (scripts/import/npc/jobs/swordsman.lua:
the `sprite = 139` touch NPCs, and the four in scripts/npc/swordman_course.lua), and searches each of the three stretches for
a way from its entrance to the warp tile at its end that steps on no pit
(eight directions, no cutting corners), and from there through the checkpoint
room. It also checks that every place a pit drops you has a way
to one of the cave's exits.
"""
import re
import struct
import sys
import zlib
from collections import deque
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "research"))
import grf_peek  # noqa: E402


def gat(grf, name):
    for n, cs, csa, rs, flags, pos in grf_peek.entries(grf):
        if n.lower() == f"data\\{name}.gat":
            with open(grf, "rb") as f:
                f.seek(pos)
                d = zlib.decompress(f.read(csa))[:rs]
            w, h = struct.unpack_from("<ii", d, 6)
            return w, h, lambda x, y: 0 <= x < w and 0 <= y < h and struct.unpack_from("<I", d, 14 + (y * w + x) * 20 + 16)[0] not in (1, 5)
    sys.exit(f"{name}.gat: not in {grf}")


def reach(ok, start, goal, avoid=frozenset()):
    """Steps from start to any cell of goal, or None."""
    seen = {start: 0}
    q = deque([start])
    while q:
        x, y = q.popleft()
        if (x, y) in goal:
            return seen[(x, y)]
        for dx in (-1, 0, 1):
            for dy in (-1, 0, 1):
                n = (x + dx, y + dy)
                if n in seen or n in avoid or not ok(*n):
                    continue
                if dx and dy and not (ok(x + dx, y) and ok(x, y + dy)):
                    continue
                seen[n] = seen[(x, y)] + 1
                q.append(n)
    return None


def box(x, y, rx, ry):
    return {(x + i, y + j) for i in range(-rx, rx + 1) for j in range(-ry, ry + 1)}


def main():
    w, h, ok = gat(sys.argv[1], "job_sword1")
    src = (ROOT / "scripts/import/npc/jobs/swordsman.lua").read_text(encoding="utf-8")
    src += (ROOT / "scripts/npc/swordman_course.lua").read_text(encoding="utf-8")  # four pits 817 lacks
    pits, drops = set(), set()
    for m in re.finditer(r'map = "job_sword1", x = (\d+), y = (\d+), dir = \d+, sprite = 139, name = "[^"]*",\s*\n\s*touch = \{ (\d+), (\d+) \}', src):
        x, y, rx, ry = map(int, m.groups())
        pits |= box(x, y, rx, ry)
    for m in re.finditer(r'warp\("job_sword1", (\d+), (\d+)\)', src):
        drops.add((int(m.group(1)), int(m.group(2))))
    start = (10, 245)
    drops.discard(start)
    print(f"job_sword1 {w}x{h}: {len(pits)} pit cells ({sum(ok(*c) for c in pits)} walkable), {len(drops)} places a pit drops you")
    bad = 0
    # each stretch ends on a warp tile (jobs/swordsman.txt:9-11) into its checkpoint room, whose
    # own warp (npc_warp.txt:1623-1624) starts the next; the third room has the examiner
    for name, a, goal in [("1st stretch", start, box(193, 244, 1, 1)), ("1st room", (214, 243), box(223, 243, 1, 1)),
                          ("2nd stretch", (11, 206), box(193, 206, 1, 1)), ("2nd room", (214, 205), box(223, 205, 1, 1)),
                          ("3rd stretch", (11, 168), box(193, 168, 1, 1)), ("3rd room", (214, 167), box(223, 167, 1, 1))]:
        n = reach(ok, a, goal, pits)
        free = reach(ok, a, goal)
        print(f"  {name}: from {a}: {'no way past the pits' if n is None else f'{n} steps past the pits'}"
              f" ({'no way at all' if free is None else f'{free} with no pits'})")
        bad += n is None
    exits = box(65, 117, 1, 1) | box(98, 27, 1, 1) | box(161, 27, 1, 1) | box(239, 117, 1, 1)
    for d in sorted(drops):
        n = reach(ok, d, exits)
        print(f"  dropped at {d}: {'blocked cell' if not ok(*d) else 'no way out' if n is None else f'{n} steps to an exit'}")
        bad += n is None
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()

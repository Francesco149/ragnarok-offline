#!/usr/bin/env python3
"""Look for the slips 817's NPC scripts are known to have, in the imported Lua.

  npc_lint.py [file ...]          default: scripts/import/npc/**/*.lua

Two patterns, each found by playing the game first (THREADS.md #6, #27):
  missing   a jump to a label the source does not have (the script just ends)
  hollow    `if X goto L;` directly followed by `L:`: the test decides nothing,
            usually because the line that should refuse was lost
(A third, `or` where `and` was meant, needs reading: Kay's "no box" test.)
Per NPC: its place, its `src`, and the Lua line of each hit. NPCs that a
hand-written file replaces (`replaces = "<src>"` under scripts/npc/) are left
out; `wrap_npc` ones are marked, since a wrapper may or may not cover the hit.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
files = [Path(a) for a in sys.argv[1:]] or sorted((ROOT / "scripts/import/npc").rglob("*.lua"))
hand = "\n".join(p.read_text(errors="replace") for p in (ROOT / "scripts/npc").glob("*.lua"))
replaced = set(re.findall(r'replaces\s*=\s*"([^"]+)"', hand))
# kafra.lua and others build their lists as `src = "817:..."` entries of a table
replaced |= set(re.findall(r'\{\s*src\s*=\s*"(817:[^"]+)"', hand))
wrapped = set(re.findall(r'wrap_npc\(\s*"([^"]+)"', hand)) | set(re.findall(r'basic_skill_9\(\s*"([^"]+)"', hand))

total = 0
for f in files:
    lines = f.read_text(errors="replace").split("\n")
    npc = None
    hits = {}
    for i, line in enumerate(lines):
        m = re.match(r'\s*map = "([^"]+)", x = (\d+), y = (\d+).*name = "([^"]*)"', line)
        if m:
            npc = [f"{m.group(1)} {m.group(2)},{m.group(3)} {m.group(4)}", "?"]
        m = re.match(r'\s*src = "([^"]+)"', line)
        if m and npc:
            npc[1] = m.group(1)
        if not npc:
            continue
        key = tuple(npc)
        if "no such label in the source" in line:
            hits.setdefault(key, []).append((i + 1, "missing", line.strip()[:110]))
        m = re.match(r"\s*if (.*) then goto (\w+) end\s*$", line)
        # (a menu's `if c == 1 then goto A elseif ...` falling into `::A::` is how menus are translated)
        if m and "elseif" not in line and i + 1 < len(lines) and lines[i + 1].strip() == f"::{m.group(2)}::":
            hits.setdefault(key, []).append((i + 1, "hollow", line.strip()[:110]))
    for (where, src), hs in hits.items():
        if src in replaced:
            continue
        total += len(hs)
        print(f"{where}  [{src}]{'  (wrapped)' if src in wrapped else ''}")
        for n, kind, text in hs:
            print(f"  {f.relative_to(ROOT)}:{n} {kind}: {text}")
print(f"{total} hits")

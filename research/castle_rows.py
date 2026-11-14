#!/usr/bin/env python3
"""Each guild castle's NPC cells, as the period emulators place them.

  castle_rows.py E817 E052 JATHENA_TBOX

E817:  athena-817-src/athena (eAthena 817, 2004-02-19)
E052:  eathena-v0.5.2/athena (2004-05-20)
JATHENA_TBOX: jAthena //0974 (2004-05-16) script/npc/gvg/test/tbox.txt,
       `git show d6aa96f:script/npc/gvg/test/tbox.txt > FILE`

One line per castle: the name (817 db/castle_db.txt), the Emperium (817
npc/guild/ev_agit_*.txt), 817's Kafra and her one destination, 817's two
levers ("Switch" back from the master room, "Switch 2" into the guild
dungeon) with their targets, 0.5.2's steward (name, cell, and the cell
"Enter Master's room" warps to), jAthena's treasure room rectangle and its
two boxes. scripts/npc/castle_lord.lua is built from this output, read by
hand against each castle's cells (research/gat_cells.py).
"""
import re, sys
from pathlib import Path

e817, e052, tbox = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
rd = lambda p: p.read_bytes().decode('latin1')
names = {}
for l in rd(e817 / 'db/castle_db.txt').splitlines():
    f = l.split(',')
    if len(f) >= 3 and f[0].isdigit():
        names[f[1].replace('.gat', '')] = f[2]

emp = {}
for p in sorted((e817 / 'npc/guild').glob('ev_agit_*.txt')):
    for m in re.finditer(r'Monster "(\w+)\.gat",(\d+),(\d+),"EMPELIUM"', rd(p)):
        emp.setdefault(m.group(1), (int(m.group(2)), int(m.group(3))))

# NPC blocks: header line, then the body up to the closing brace at line start
HDR = re.compile(r'^(//)?(\w+)\.gat,(\d+),(\d+),(\d+)\tscript\t([^\t]+)\t(-?\d+),\{', re.M)
def blocks(text):
    hs = list(HDR.finditer(text))
    for i, h in enumerate(hs):
        end = hs[i + 1].start() if i + 1 < len(hs) else len(text)
        yield h, text[h.end():end]

WARP = re.compile(r'warp "(\w+?)(?:\.gat)?",(\d+),(\d+)')
tb = {}
t = tbox.read_bytes().decode('cp932', 'replace')
for m in re.finditer(r'areamonster "(\w+)\.gat",(\d+),(\d+),(\d+),(\d+),"[^"]*",(\d+)', t):
    tb.setdefault(m.group(1), [tuple(int(m.group(i)) for i in range(2, 6)), []])[1].append(int(m.group(6)))

for cas in sorted(names):
    row = {'name': names[cas], 'emp': emp.get(cas)}
    for h, body in blocks(rd(e817 / f'npc/guild/{cas}.txt')):
        if h.group(1) or h.group(2) != cas: continue
        nm, x, y = h.group(6), int(h.group(3)), int(h.group(4))
        w = WARP.findall(body)
        if nm.startswith('Kafra'):
            row['kafra'] = (x, y, int(h.group(5)), [m for m in w])
        elif nm == 'Switch':
            row['lever_out'] = (x, y, w)
        elif nm == 'Switch 2':
            row['lever_dun'] = (x, y, w)
    for h, body in blocks(rd(e052 / f'npc/guild/{cas}.txt')):
        if h.group(1) or h.group(2) != cas or h.group(7) != '55': continue
        row['steward'] = (h.group(6), int(h.group(3)), int(h.group(4)), int(h.group(5)),
                          [m for m in WARP.findall(body) if m[0] == cas])
    row['tbox'] = tb.get(cas)
    print(cas, row)

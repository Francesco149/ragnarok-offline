#!/usr/bin/env python3
"""NPCs on the same cell in mod0631 and in our scripts (817 import, minus what scripts/npc/ replaces):
the item ids each counts, takes and gives, where they differ. Runs in the workbench (/data, /work).
Found the Clock Tower keys and Aldebaran's Trader (THREADS.md #27)."""
import re, glob, sys
from pathlib import Path
M = Path('/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/conf')
def m631():
    out = {}
    for f in M.glob('*.txt'):
        t = f.read_bytes().decode('cp932', 'replace')
        for m in re.finditer(r'^(\w[\w-]*)\.gat,(\d+),(\d+),\d+\tscript\t([^\t]+)\t[-\d]+,\{(.*?)^\}', t, re.S | re.M):
            body = m.group(5)
            k = (m.group(1), int(m.group(2)), int(m.group(3)))
            out[k] = (f.name, m.group(4), {
                'count': set(map(int, re.findall(r'countitem\((\d+)\)', body))),
                'take': set(map(int, re.findall(r'delitem\s+(\d+)', body))),
                'give': set(map(int, re.findall(r'getitem\s+(\d+)', body)))})
    return out
def ours():
    hand = {}
    for f in glob.glob('/work/scripts/npc/*.lua'):
        t = open(f).read()
        for r in re.findall(r'replaces\s*=\s*"([^"]+)"', t): hand[r] = f
    out = {}
    for f in glob.glob('/work/scripts/import/npc/**/*.lua', recursive=True):
        t = open(f).read()
        for blk in re.split(r'\nnpc \{', t)[1:]:
            m = re.search(r'map = "([^"]+)", x = (\d+), y = (\d+).*name = "([^"]*)"', blk)
            s = re.search(r'src = "([^"]+)"', blk)
            if not m or (s and s.group(1) in hand): continue
            k = (m.group(1), int(m.group(2)), int(m.group(3)))
            out[k] = (f.split('/npc/')[-1], m.group(4), {
                'count': set(map(int, re.findall(r'count_item\((\d+)', blk))),
                'take': set(map(int, re.findall(r'take_item\((\d+)', blk))),
                'give': set(map(int, re.findall(r'give_item\((\d+)', blk)))})
    return out
a, b = m631(), ours()
for k in sorted(set(a) & set(b)):
    (fa, na, ia), (fb, nb, ib) = a[k], b[k]
    d = {x: (sorted(ia[x] - ib[x]), sorted(ib[x] - ia[x])) for x in ia if ia[x] != ib[x]}
    if d and any(ia.values()) and any(ib.values()):
        print(f"{k[0]} {k[1]},{k[2]}  817 {fb} {nb!r} / 631 {fa}")
        for x, (only631, only817) in d.items():
            print(f"   {x}: mod0631 only {only631}  817 only {only817}")

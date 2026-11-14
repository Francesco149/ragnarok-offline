#!/usr/bin/env python3
"""Monster drops mod0631 leaves as placeholders and eAthena 817 fills.

mod0631's db/mob_db.txt (jRO, 2003-12) writes `512,0` (an Apple at rate 0) in
drop slots it had no item for. 817's (2004-01) has a real item and rate in many
of those very slots. This lists each such slot and whether RODataZone's iRO
monster table of 2004-02 (data:refs/web/rodatazone-2004/*_monsters_monsters.js,
names only, no rates) names that item for that monster.

  research/mob_drop_holes.py REFS RDZ_MONSTERS_JS     (run from the repo root)
  REFS = data:refs/eathena-archive/x

Used for tools/import/athena_import.py's MOB_DROP_HOLES (research/THREADS.md #32).
"""
import re
import sys
import tomllib

refs, rdz = sys.argv[1:3]


def load(path):
    d = {}
    for line in open(path, "rb").read().decode("latin-1").split("\n"):
        c = [x.strip() for x in line.split(",")]
        if c and c[0].isdigit() and len(c) > 45 and c[3]:
            d[int(c[0])] = c
    return d


m631 = load(f"{refs}/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/db/mob_db.txt")
e817 = load(f"{refs}/athena-817-src/athena/db/mob_db.txt")
items = {i["id"]: i for i in tomllib.load(open("data/db/items.toml", "rb"))["item"] if "id" in i}
mobs = {m["id"]: m for m in next(v for v in tomllib.load(open("data/db/mobs.toml", "rb")).values()
                                  if isinstance(v, list))}


def norm(s):
    s = s.lower().replace("'", "").replace("-", " ")
    s = re.sub(r"\s*\(\d\)|\s*\[\d\]", "", s)
    return re.sub(r"[^a-z0-9 ]", "", s).strip()


# RODataZone: MONSTERS[n] = new Array('Name', level, ..., 'drops, comma separated', ...)
rdz_drops = {}
for m in re.finditer(r"MONSTERS\[\d+\] = new Array\((.*)\);", open(rdz, encoding="latin-1").read()):
    cols = re.findall(r"'((?:[^'\\]|\\.)*)'", m.group(1))
    if len(cols) > 20:
        rdz_drops[norm(cols[0].rstrip("*"))] = {norm(x) for x in cols[20].replace("\\'", "'").split(",")}

n = agree = 0
for mid in sorted(m631):
    if mid not in e817:
        continue
    a, b = m631[mid], e817[mid]
    mob = mobs.get(mid, {})
    seen = rdz_drops.get(norm(mob.get("display", "")))
    for s in range(8):
        i = 29 + 2 * s
        if a[i] == "512" and a[i + 1] == "0" and b[i] != "512" and b[i + 1] not in ("", "0"):
            iid, rate = int(b[i]), int(b[i + 1])
            name = items.get(iid, {}).get("display", "?")
            if seen is None:
                verdict = "rdz: no such monster"
            elif norm(name) in seen:
                verdict = "rdz: yes"
                agree += 1
            else:
                verdict = "rdz: no"
            n += 1
            print(f"{mid}\t{a[1]}\tslot {s}\t{iid}\t{rate}\t{name}\t{verdict}")
print(f"# {n} slots; RODataZone names the item in {agree}", file=sys.stderr)

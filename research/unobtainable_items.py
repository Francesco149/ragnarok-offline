#!/usr/bin/env python3
"""Items the client knows (description or resource name) that nothing in the game gives.

  ragnarok-offline grf cat DATA.GRF 'data\\idnum2itemdesctable.txt' > desc.txt   (and the
  displayname and resname tables likewise), then
  unobtainable_items.py desc.txt names.txt res.txt     (run from the repo root)

"Gives" is generous: any monster drop, any id in data/db/random.toml or pets.toml,
any 3-5 digit number or item db name in scripts/. What is left is a lead for cut
content (docs/DEVIATIONS.md, "Content the client ships"), research/THREADS.md #31.
"""
import glob
import re
import sys
import tomllib


def ids(path):
    t = open(path, "rb").read().decode("latin-1")
    return {int(m.group(1)) for m in re.finditer(r"^(\d+)#", t, re.M)}


desc, names, res = (ids(p) for p in sys.argv[1:4])
items = {i["id"]: i for i in tomllib.load(open("data/db/items.toml", "rb"))["item"] if "id" in i}
byname = {i["name"]: k for k, i in items.items() if "name" in i}
got = set()
mobs = tomllib.load(open("data/db/mobs.toml", "rb"))
for mo in next(v for v in mobs.values() if isinstance(v, list)):
    for d in mo.get("drops", []):
        got.add(d.get("item") if isinstance(d, dict) else d[0])
for f in ["data/db/random.toml", "data/db/pets.toml"]:
    got |= {int(x) for x in re.findall(r"\b(\d{3,5})\b", open(f).read())}
for f in glob.glob("scripts/**/*.lua", recursive=True):
    t = open(f, encoding="utf-8", errors="replace").read()
    got |= {int(x) for x in re.findall(r"\b(\d{3,5})\b", t)}
    got |= {byname[n] for n in re.findall(r'"([A-Za-z_\'0-9]+)"', t) if n in byname}
for i in sorted((desc | res) - got):
    it = items.get(i, {})
    print(i, it.get("name", "-"), "|", it.get("display", "-"),
          "| named" if i in names else "| no name", "| desc" if i in desc else "")

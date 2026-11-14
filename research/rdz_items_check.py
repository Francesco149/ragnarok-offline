#!/usr/bin/env python3
"""RODataZone's item tables (iRO, early 2004) against data/db/items.toml and the client's text.

  rdz_items_check.py DIR [DESC.txt]

DIR holds RODataZone's `items/*.js` as saved from Wayback (data:refs/web/rodatazone-2004/,
`<capture>_items_weapons.js`, `_armor.js`, `_consumables.js`, `_common.js`). DESC.txt is the
client's `data\\idnum2itemdesctable.txt` (optional: then the client's figure is shown too).
Items are matched by display name (lower case), so a name RODataZone spells otherwise is
listed as unmatched. Prints every field where RODataZone and our table differ:
id, name, field, RODataZone, ours, client text.
"""
import re, sys, tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
d = Path(sys.argv[1])
items = tomllib.load(open(ROOT / 'data/db/items.toml', 'rb'))['item']
byname = {}
for it in items:
    byname.setdefault(str(it.get('display', '')).lower().replace("'", ''), it)

client = {}
if len(sys.argv) > 2:
    text = open(sys.argv[2], 'rb').read().decode('latin-1')
    rx = {'weight': r'Weight\s*:\s*(\d+)', 'atk': r'Attack Strength\s*:\s*(\d+)', 'def': r'Defen[cs]e Rate\s*:\s*(\d+)',
          'equip_level': r'Required Level\s*:\s*(\d+)', 'weapon_level': r'Weapon Level\s*:\s*(\d+)'}
    for m in re.finditer(r'(?m)^(\d+)#\r?\n(.*?)^#', text, re.S):
        body = re.sub(r'\^[0-9a-fA-F]{6}', '', m.group(2))
        client[int(m.group(1))] = {k: int(f.group(1)) for k, r in rx.items() if (f := re.search(r, body))}


def rows(pattern, var):
    f = next(iter(sorted(d.glob(pattern))), None)
    if not f:
        return []
    js = f.read_text(encoding='latin-1')
    out = []
    for m in re.finditer(rf"^{var}\[\d+\] = new Array\((.*)\);", js, re.M):
        out.append([x.replace("\\'", "'") for x in re.findall(r"'((?:[^'\\]|\\.)*)'", m.group(1))])
    return out


def num(s):
    s = s.strip()
    return int(s) if s.isdigit() else None


# field name -> column, per table
TABLES = [
    ('*_items_weapons.js', 'WEAPONS', 1, {'weight': 3, 'equip_level': 4, 'weapon_level': 5, 'atk': 6, 'slots': 7}),
    ('*_items_armor.js', 'ARMOR', 1, {'weight': 3, 'equip_level': 4, 'def': 5, 'slots': 6}),
    ('*_items_armor.js', 'HEADGEAR', 1, {'weight': 3, 'equip_level': 4, 'def': 5, 'slots': 6}),
    ('*_items_consumables.js', 'CONSUMABLES', 1, {'weight': 3}),
    ('*_items_common.js', 'COMMON', 0, {'weight': 1}),
]
unmatched, n = [], 0
for pattern, var, name_col, fields in TABLES:
    for r in rows(pattern, var):
        it = byname.get(r[name_col].lower().replace("'", ''))
        if not it:
            unmatched.append(r[name_col])
            continue
        for field, col in fields.items():
            if col >= len(r) or (rdz := num(r[col])) is None:
                continue
            ours = it.get(field) or 0
            if field == 'weight':
                ours //= 10
            if rdz != ours:
                n += 1
                cl = client.get(it['id'], {}).get(field, '')
                print(f"{it['id']}\t{it.get('display')}\t{field}\trdz {rdz}\tours {ours}\tclient {cl}")
print(f"# {n} differences; {len(unmatched)} names unmatched: {', '.join(unmatched[:30])}", file=sys.stderr)

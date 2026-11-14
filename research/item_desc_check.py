#!/usr/bin/env python3
"""Compare the item numbers the client's own descriptions state with data/db/items.toml.

  item_desc_check.py DESC.txt      (data\\idnum2itemdesctable.txt of the played client, e.g.
                                    `ragnarok-offline grf cat <data.grf> "data\\idnum2itemdesctable.txt"`)

The client (tier client) says, per item: Attack Strength, Defence Rate, Weight (whole
units; ours are tenths), Weapon Level, Required Level. Prints every disagreement.
"""
import re, sys, tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
text = open(sys.argv[1], 'rb').read().decode('latin-1')
items = {i['id']: i for i in tomllib.load(open(ROOT / 'data/db/items.toml', 'rb'))['item']}
FIELDS = {
    'atk': r'Attack Strength\s*:\s*\^?[0-9a-fA-F]{0,6}\s*(\d+)',
    'def': r'Defen[cs]e Rate\s*:\s*\^?[0-9a-fA-F]{0,6}\s*(\d+)',
    'weight': r'Weight\s*:\s*\^?[0-9a-fA-F]{0,6}\s*(\d+)',
    'weapon_level': r'Weapon Level\s*:\s*\^?[0-9a-fA-F]{0,6}\s*(\d+)',
    'equip_level': r'Required Level\s*:\s*\^?[0-9a-fA-F]{0,6}\s*(\d+)',
}
n = 0
for m in re.finditer(r'(?m)^(\d+)#\r?\n(.*?)^#', text, re.S):
    iid, body = int(m.group(1)), re.sub(r'\^[0-9a-fA-F]{6}', '', m.group(2))
    it = items.get(iid)
    if not it:
        continue
    for k, rx in FIELDS.items():
        f = re.search(rx.replace(r'\^?[0-9a-fA-F]{0,6}\s*', r'\s*'), body)
        if not f:
            continue
        said = int(f.group(1))
        ours = it.get(k) or 0
        if k == 'weight':
            ours = ours // 10
        if said != ours:
            n += 1
            print(f"{iid}\t{it.get('display')}\t{k}\tclient {said}\tours {ours}")
print(f"# {n} disagreements", file=sys.stderr)

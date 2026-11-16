#!/usr/bin/env python3
"""Items whose effect the period sources call "not implemented", beside what ours does.

  nyi_items.py DESC.txt RDZ_DIR

DESC.txt is the client's `data\\idnum2itemdesctable.txt` (tier client, text); RDZ_DIR holds
RODataZone's `items/*.js` (iRO, captures of 2004-02; data:refs/web/rodatazone-2004/). An item
is listed when either source marks an effect "Not Implemented", "not yet implemented" or "NYI".
Columns: id, name, client says NYI (C), RODataZone says NYI (R), our script, the two lines.

The rule (THREADS.md R6): the client's text alone is an older state and loses to mod0631; the
text with a second, independent period witness (RODataZone) wins. So C+R = the effect was not
in the game; C alone = keep mod0631's.
"""
import re, sys, tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
NYI = re.compile(r'not\s+(yet\s+)?implement|\bNYI\b', re.I)
items = {i['id']: i for i in tomllib.load(open(ROOT / 'data/db/items.toml', 'rb'))['item']}
byname = {}
for it in items.values():
    byname.setdefault(re.sub(r"[^a-z0-9]", "", str(it.get('display', '')).lower()), it['id'])

client = {}
text = open(sys.argv[1], 'rb').read().decode('latin-1')
for m in re.finditer(r'(?m)^(\d+)#\r?\n(.*?)^#', text, re.S):
    body = re.sub(r'\^[0-9a-fA-F]{6}', '', m.group(2)).replace('\r', '')
    lines = [l.strip() for l in body.split('\n') if NYI.search(l)]
    client[int(m.group(1))] = lines

# RODataZone: every quoted field of each row; the effect is whichever field says NYI
ALIAS = {'cobold card': 'kobold card', 'freezing arrow': 'freeze arrow', 'orc warrier card': 'orc warrior card',
         'mjolnir': 'mjollnir', 'brisingamen': 'brysinggamen', 'megingard': 'magingiorde'}
rdz = {}
unmatched = []
for f in sorted(Path(sys.argv[2]).glob('*_items_*.js')):
    for row in re.findall(r"new Array\((.*?)\);", f.read_text(encoding='latin-1')):
        fields = [x.replace("\\'", "'") for x in re.findall(r"'((?:[^'\\]|\\.)*)'", row)]
        hits = [x for x in fields if NYI.search(x)]
        if not hits:
            continue
        def key(x):
            k = x.lower().rstrip('*').strip()
            return re.sub(r"[^a-z0-9]", "", ALIAS.get(k, k))
        # the name is the first of the leading fields that names an item (weapon rows lead with
        # their class, "Dagger", "Axe", which are item names too: so prefer the second field)
        cands = [x for x in fields[:3] if key(x) and key(x) in byname]
        name = cands[-1] if len(cands) > 1 and fields[0] in cands else (cands[0] if cands else fields[1])
        iid = byname.get(key(name)) if key(name) else None
        if iid is None:
            unmatched.append((f.name[:14], name, hits[0]))
            continue
        rdz[iid] = (f.name[:14], hits)

for iid in sorted(set(i for i, l in client.items() if l) | set(rdz)):
    it = items.get(iid, {})
    c, r = client.get(iid, []), rdz.get(iid, (None, []))[1]
    ours = it.get('athena_equip') or it.get('use_script') or ''
    print(f"{iid}\t{it.get('display')}\t{'C' if c else '-'}{'R' if r else '-'}\t{ours}\t{' / '.join(c)}\t{' / '.join(r)}")
for u in unmatched:
    print('unmatched RODataZone row:', *u, sep='\t')

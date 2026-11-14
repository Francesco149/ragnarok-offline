#!/usr/bin/env python3
"""Ragu Memo's castle treasure table (jRO, 2004-02-06) against mod0631's boxes.

  ragmemo_gvg_box.py PAGE ITEM_DB MOB_DB

PAGE: data:refs/web/ragmemo-2003/ragmemo_gvg_box_20040206.html (capture
20040206224529 of ragmemo.s33.xrea.com/gvg_box.html). ITEM_DB, MOB_DB:
mod0631's db/item_db.txt and db/mob_db.txt (Shift-JIS; names are matched
by the JName column). One line per castle: each item of the page as
name=id, its rate as Athena's per-10000 ("rm"), and the box's rate for the
same id ("box", "-" when the box lacks it); then what the box has that the
page does not. "s" before a name is the slotted item (the next id of the
same name). The castle's box is the one holding its God-item material.
Round 18 review, research/notes/round18-review.md.
"""
import re, html, sys
page, itemdb, mobdb = sys.argv[1:4]
t = open(page, 'rb').read().decode('cp932', 'replace')
t = re.sub(r'<BR>', '\n', t); t = html.unescape(re.sub(r'<[^>]+>', '', t))
lines = [l.strip() for l in t.split('\n') if l.strip()]
names = {}
byid = {}
for l in open(itemdb, 'rb').read().decode('cp932', 'replace').split('\n'):
    f = l.split(',')
    if len(f) > 3 and f[0].strip().isdigit():
        names.setdefault(f[2], int(f[0])); byid[int(f[0])] = f[2]
boxes = {}
for l in open(mobdb, 'rb').read().decode('cp932', 'replace').split('\n'):
    f = l.split(',')
    if len(f) > 40 and f[0].isdigit() and 1324 <= int(f[0]) <= 1363:
        d = f[29:47]
        boxes[int(f[0])] = {int(d[i]): int(d[i + 1]) for i in range(0, len(d) - 1, 2) if d[i].strip().isdigit() and int(d[i]) > 0}
sec, cur, out = None, None, {}
for l in lines:
    if l in ('砦共通',): cur = 'common'; out[cur] = []; continue
    if l == '砦固有': continue
    if l.startswith('[ 神器'): break
    m = re.match(r'(.+)\(1/([\d.]+)\)$', l)
    if m and cur:
        out[cur].append((m.group(1), round(10000 / float(m.group(2)))))
    elif re.match(r'^[A-Z][\w -]+ \d$', l):
        cur = l; out[cur] = []
def find(n):
    if n in names: return names[n]
    if n.startswith('s') and n[1:] in names:
        b = names[n[1:]]
        return b + 1 if byid.get(b + 1) == n[1:] else ('s', b)
    return None
for c, items in out.items():
    ids = [(n, r, find(n)) for n, r in items]
    plain = [i for _, _, i in ids if isinstance(i, int)]
    mat = [i for i in plain if 7073 <= i <= 7092]
    best = next((b for b in boxes if mat and mat[0] in boxes[b]), None) or max(boxes, key=lambda b: sum(1 for i in plain if i in boxes[b]))
    extra = {i: r for i, r in boxes[best].items() if i not in plain}
    row = []
    for n, r, i in ids:
        if i is None: row.append(f'{n}?{r}')
        elif isinstance(i, tuple): row.append(f'{n}[slotted {i[1]}] rm{r} box{boxes[best].get(i[1], "-")}')
        else: row.append(f'{n}={i} rm{r} box{boxes[best].get(i, "-")}')
    print(f'{c:22} box {best}:', '; '.join(row), '| box only:', extra)

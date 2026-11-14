#!/usr/bin/env python3
"""Ragu Memo's dungeon spawn pages (jRO, 2003) against our spawns.

  ragmemo_spawns.py MOD0631_MOB_DB PAGE.html... > out.txt     (run from the repo root)

PAGEs are Ragu Memo's `map_*.html` (Shift-JIS), saved in data:refs/web/ragmemo-2003/
as ragmemo_map_<area>_<capture>.html. Each floor is a block headed by a `MapLavel` cell
(Japanese floor name) with a map image; each monster row gives name, level, and a count:
"15", "定点 6" (fixed spot), "部屋 15" (in rooms), "1 (30min)" (respawn), "準Boss" or
"Boss" (1 when no number), several areas in one cell ("部屋 30 部屋 10 定点 1": summed, the
parenthesised respawn times left out); a cell with no number at all ("定点", "-") is not
counted and shows as such. Names go to ids through mod0631's JName column (the site and mod0631
both write jRO's names); a name spelt otherwise goes by its level and base EXP when exactly
one monster of mod0631 has both (printed with "~").

The page does not name the map, so each block is paired with the map of
scripts/import/spawns.lua whose monster ids overlap it most (Jaccard); the pairing is
printed with its score so a wrong one shows. Then, per block: every monster whose count
differs from ours, every monster only on the page or only in ours. research/THREADS.md #10.
"""
import re
import sys
import html
from collections import defaultdict
from pathlib import Path


# Page spellings mod0631's JName lacks, checked by level and EXP against mod0631's row.
ALIAS = {'エクスキュージョナー': 1205, 'オーク(弓)': 1189, 'クッキー(緑)': 1265,
         'クッキー(赤)': 1246, '白連玉': 1138, 'ゴデウィスパー': 1186, 'ファミリア': 1005,
         'ドラインリアー': 1111, 'カナトゥス': 1067, 'マリンスピアー': 1142}
# Plants and mushrooms: the page lists none of them on any floor.
PLANTS = set(range(1078, 1086))
# Floors the overlap pairs wrongly, by the page's image name.
PIN = {'map_comodof2.jpg': 'cmd_fild02'}


def jnames(path):
    d, by_stats = {}, defaultdict(list)
    for line in Path(path).read_bytes().decode('cp932', 'replace').splitlines():
        f = line.split(',')
        if len(f) > 7 and f[0].isdigit() and f[3].strip().isdigit() and f[6].strip().isdigit():
            d.setdefault(f[2].strip(), int(f[0]))
            by_stats[(int(f[3]), int(f[6]))].append(int(f[0]))
    return d, by_stats


def blocks(path):
    s = Path(path).read_bytes().decode('cp932', 'replace')
    parts = re.split(r'class="MapLavel">', s)
    for part in parts[1:]:
        label = html.unescape(re.sub(r'\s+', ' ', part.split('</TD>', 1)[0])).strip()
        img = re.search(r'<IMG src="img/([^"]+)"', part)
        mobs = []
        for cell, cnt in re.findall(
                r'<TD class="MapMob">(.*?)</TD>\s*<TD class="MapMob">(.*?)</TD>', part, re.S):
            text = html.unescape(re.sub(r'<[^>]+>', ' ', cell))
            name = re.match(r'\s*(.*?) Lv\.(\d+)', text)
            exp = re.search(r'Exp\.(\d+)', text)
            c = html.unescape(re.sub(r'<[^>]+>', ' ', cnt))
            c = re.sub(r'\s+', ' ', c).strip()
            ns = [int(x) for x in re.findall(r'\d+', re.sub(r'\(.*?\)', '', c))]
            num = sum(ns) if ns else 1 if 'Boss' in c else None
            if name:
                mobs.append((name.group(1).strip(), int(name.group(2)),
                             int(exp.group(1)) if exp else -1, num, c))
        yield label, img.group(1) if img else '', mobs


def ours(path):
    d = defaultdict(lambda: defaultdict(int))
    for line in open(path, encoding='utf-8'):
        m = re.search(r'map = "(\w+)".*mob = (\d+), count = (\d+)', line)
        if m:
            d[m.group(1)][int(m.group(2))] += int(m.group(3))
    return d


def main():
    jn, by_stats = jnames(sys.argv[1])
    sp = ours('scripts/import/spawns.lua')
    totals = defaultdict(int)
    for page in sys.argv[2:]:
        print(f'## {Path(page).name}')
        for label, img, mobs in blocks(page):
            ids, raws = {}, {}
            unknown = []
            guessed = []
            for name, lv, exp, num, raw in mobs:
                i = ALIAS.get(name) or jn.get(name)
                if i is None and len(by_stats.get((lv, exp), [])) == 1:
                    i = by_stats[(lv, exp)][0]
                    guessed.append(f'~ {name} = {i}')
                if i is None:
                    unknown.append(f'{name} Lv.{lv} ({raw})')
                else:
                    raws.setdefault(i, []).append(raw)
                    if num is None or ids.get(i, 0) is None:
                        ids[i] = None
                    else:
                        ids[i] = ids.get(i, 0) + num
            best = PIN.get(img)
            score = 1.0 if best else 0.0
            for m, mm in sp.items() if best is None else ():
                a, b = set(ids), set(mm) - PLANTS
                j = len(a & b) / len(a | b) if a | b else 0
                if j > score:
                    best, score = m, j
            o = {i: c for i, c in sp.get(best, {}).items() if i not in PLANTS}
            diff = [(i, ids.get(i, 0), o.get(i, 0)) for i in sorted(set(ids) | set(o))
                    if ids.get(i, 0) != o.get(i, 0)]
            totals['uncounted'] += sum(1 for i in ids if ids[i] is None)
            totals['blocks'] += 1
            totals['groups'] += len(ids)
            totals['same'] += sum(1 for i in ids if ids[i] == o.get(i))
            print(f'{label} [{img}] -> {best} ({score:.2f}): {len(ids)} monsters, '
                  f'{len(diff)} differ')
            for i, p, q in diff:
                name = next((n for n, v in jn.items() if v == i), str(i))
                shown = ' + '.join(raws.get(i, [])) or '-'
                print(f'    {i} {name}: page {shown}, ours {q}')
            for g in guessed:
                print(f'    {g}')
            for u in unknown:
                print(f'    ? {u}')
    print(f'# {totals["blocks"]} floors, {totals["groups"]} page rows, '
          f'{totals["same"]} with our count, {totals["uncounted"]} without a number',
          file=sys.stderr)


if __name__ == '__main__':
    main()

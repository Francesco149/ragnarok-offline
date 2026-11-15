#!/usr/bin/env python3
"""Ragu Memo's monsters by element (jRO, at_monster.html, captured 2003-12-13) against ours.

  ragmemo_elements.py PAGE.html MOD0631_MOB_DB     (run from the repo root)

The page has one table per element; its header row names the element levels ("無1".."無4")
and each level's cell lists "name(LvN,race)". Names are matched by mod0631's JName column.
Prints every monster whose element, element level, race or level differs from
data/db/mobs.toml (element = level*20 + element, races in Athena's order), and counts.
"""
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# Page spellings that differ from mod0631's JName (as research/ragmemo_mob_magic.py's, and more).
MOBS = {'地プティ': [1155], '空プティ': [1156], 'プティット(地)': [1155], 'プティット(空)': [1156],
        'クッキー(緑)': [1265], 'クッキー': [1246], 'エドガー': [1115], 'マヤ': [1147], 'キャットナインテール': [1307],
        'ブラッドナイト': [1268], 'プリオニ': [1159]}
ELEMENTS = {'無': 0, '水': 1, '地': 2, '火': 3, '風': 4, '毒': 5, '聖': 6, '闇': 7, '念': 8, '不死': 9, '死': 9}
RACES = {'無形': 0, '不死': 1, '動物': 2, '植物': 3, '昆虫': 4, '魚貝': 5, '水棲': 5, '悪魔': 6, '人間': 7,
         '天使': 8, '竜': 9}


def main():
    s = Path(sys.argv[1]).read_bytes().decode('cp932', 'replace')
    jname = {}
    for line in Path(sys.argv[2]).read_bytes().decode('cp932', 'replace').splitlines():
        f = line.split(',')
        if len(f) > 3 and f[0].isdigit():
            jname.setdefault(f[2], []).append(int(f[0]))
    mobs = {m['id']: m for m in tomllib.load(open(ROOT / 'data/db/mobs.toml', 'rb'))['mob']}
    n = {'monsters': 0, 'agree': 0, 'differ': 0, 'unknown': 0}
    for table in re.findall(r'<TABLE.*?</TABLE>', s, re.S):
        heads = re.findall(r'class="ATMonsterLavel"[^>]*>(\D+)(\d)</TD>', table)
        cells = re.findall(r'class="ATMonsterText"[^>]*>(.*?)</TD>', table, re.S)
        for (el, lv), cell in zip(heads, cells):
            for name, mlv, race in re.findall(r'([^<>\n]+?)\(Lv(\d+),([^)]+)\)', re.sub(r'<[^>]+>', '\n', cell)):
                name = name.strip()
                n['monsters'] += 1
                want = (int(lv) * 20 + ELEMENTS[el], RACES.get(race), int(mlv))
                ids = [i for i in MOBS.get(name) or jname.get(name, []) if i in mobs]
                if not ids:
                    n['unknown'] += 1
                    print(f'?\t{name}\t{el}{lv} {race} Lv{mlv}')
                    continue
                got = [(mobs[i].get('element', 0), mobs[i].get('race', 0), mobs[i].get('level', 0)) for i in ids]
                if want in got:
                    n['agree'] += 1
                else:
                    n['differ'] += 1
                    print(f'{ids}\t{name}\tpage {el}{lv} {race} Lv{mlv} = {want}\tours {got}')
    print(' '.join(f'{k} {v}' for k, v in n.items()), file=sys.stderr)


main()

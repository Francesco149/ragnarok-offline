#!/usr/bin/env python3
"""Ragu Memo's item pages (jRO, Wayback captures of 2003-12) against our monster drops.

  ragmemo_drops.py MOD0631_DB_DIR PAGE.html... > out.tsv     (run from the repo root)

MOD0631_DB_DIR is mod0631's db/ (item_db.txt and mob_db.txt, whose JName columns name items
and monsters as the pages do), in data:refs/eathena-archive/x/, beside athena-817-src/ whose
rates fill the `817` column. The pages are Ragu Memo's armorN/weaponN/itemN/card.html,
Shift-JIS, saved at the capture nearest 2003-12-18 (data:refs/web/ragmemo-2003/; found from
research round 24, which quoted ten of their rows). Each item has an `ItemName` cell and
then an `ItemDrop` row: "monster(1/N) , monster(1/N)".

Our rates (data/db/mobs.toml, as mod0631's and 817's: chance in 10000) are set against the
page's 1/N as 10000/N. mod0631's small rates sit one below that (1/5000 is 1, 1/1666 is 5,
1/625 is 15; 1/200 is 50): whoever turned such a table into rates cut differently. A pair
"agrees" when ours is within 1.6 (or 2.5%) of 10000/N.
The importer marks the slots taken from 817 where mod0631 had `512,0`
(`# drops ...: eAthena 817` comments): those rows are the point (research/THREADS.md #32).
Output columns: item id, item, monster id, monster, page 1/N, as rate, ours, from (mod0631, or
817 for a filled hole; "mvp" for an MVP slot), 817's rate, verdict, and a call: keep; page (817
slot) where the page differs from the 817 rate we took; page+817 where both differ from mod0631
alike; keep (817 = ours); page alone; missing: page+817 / page alone where we lack the drop.
Summary counts on stderr.
"""
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def jnames(path, col=2):
    d = {}
    for line in Path(path).read_bytes().decode('cp932', 'replace').splitlines():
        f = line.split(',')
        if len(f) > col and f[0].isdigit():
            d.setdefault(f[col].strip(), []).append(int(f[0]))
    return d


# Page spellings of monsters that differ from mod0631's JName, by id (checked by English name).
MOBS = {'エクスキュージョナー': [1205], 'ミスティルテイン': [1203], 'マヤ': [1147],
        'ブラッドナイト': [1268], 'クリーミーフィア': [1293], 'マリンスピアー': [1142],
        'バフォメット.Jr': [1101], 'ステムウォーム': [1215], 'カナトゥス': [1067], 'プリオニ': [1159],
        'ファミリア': [1005], 'ファーミリア': [1005], 'ビッグフット': [1060], 'ドラップス': [1113],
        'エドガー': [1115], 'ウインドゴースト': [1263], '黄金盗蟲': [1086],
        'ゾンビプリゾナー': [1197], 'スケルプリゾナー': [1196], 'スケルトンプリゾナー': [1196],
        'ゴデウィスパー': [1186], 'キャットナインテール': [1307], '三葉虫': [1279],
        'レイドックアーチャー': [1276], 'ドラインリアー': [1111], 'シェルフィッシュ': [1074],
        'サヴィッジ': [1166], 'コボルトアーチャー': [1282], 'エンジェルリング': [1096],
        'アムマット': [1301]}


def page_items(path):
    s = Path(path).read_bytes().decode('cp932', 'replace')
    name = None
    for m in re.finditer(r'class="ItemName"[^>]*>(.*?)</TD>|class="ItemDrop"[^>]*>(.*?)</TD>', s, re.S):
        if m.group(1) is not None:
            name = re.sub(r'<[^>]+>', '', m.group(1)).strip()
            name = re.sub(r'\s*\[\d\]$', '', name)
        elif name:
            drops = re.findall(r'([^,()<>]+?)\(<FONT class="Drop">1/(\d+)</FONT>\)', m.group(2))
            yield name, [(mn.strip(), int(n)) for mn, n in drops]
            name = None


def main():
    db = Path(sys.argv[1])
    items_j = jnames(db / 'item_db.txt')
    mobs_j = jnames(db / 'mob_db.txt')
    mobs = {m['id']: m for m in tomllib.load(open(ROOT / 'data/db/mobs.toml', 'rb'))['mob']}
    # Which slots came from 817: the importer's comment under each [[mob]] block.
    from817 = {}
    cur = None
    for line in open(ROOT / 'data/db/mobs.toml', encoding='utf-8'):
        if line.startswith('id = '):
            cur = int(line.split('=')[1])
        m = re.match(r'# drops ([\d, ]+): eAthena 817', line)
        if m and cur:
            from817[cur] = {int(x) for x in m.group(1).split(',')}
    n = {k: 0 for k in ('pairs', 'agree', 'differ', 'missing', 'noitem', 'nomob',
                        '817 agree', '817 differ', '817 missing')}
    print('item\titem name\tmob\tmob name\tpage 1/N\tpage rate\tours\tfrom\t817\tverdict\tcall')
    # 817's db/mob_db.txt: (mob, item) -> rate, drops then MVP slots
    e817 = {}
    for line in Path(sys.argv[1], '../../../athena-817-src/athena/db/mob_db.txt').resolve().read_bytes().decode('latin-1').splitlines():
        f = [x.strip() for x in line.split(',')]
        if len(f) > 54 and f[0].isdigit():
            for i in list(range(29, 45, 2)) + list(range(49, 55, 2)):
                if f[i].isdigit() and f[i + 1].isdigit():
                    e817.setdefault((int(f[0]), int(f[i])), int(f[i + 1]))
    for page in sys.argv[2:]:
        for iname, drops in page_items(page):
            iids = items_j.get(iname, [])
            if not iids:
                n['noitem'] += 1
                print(f'?\t{iname}\t\t\t\t\t\t\titem?')
                continue
            for mname, nn in drops:
                n['pairs'] += 1
                mids = MOBS.get(mname) or mobs_j.get(mname, [])
                if not mids:
                    n['nomob'] += 1
                    print(f'{iids[0]}\t{iname}\t?\t{mname}\t1/{nn}\t\t\t\tmonster?')
                    continue
                best = None
                for mid in mids:
                    for kind in ('drops', 'mvp_drops'):
                        for d in mobs.get(mid, {}).get(kind, []):
                            if d['item'] in iids:
                                best = (mid, d, kind)
                prate = round(10000 / nn, 1)
                near = lambda r: r != '' and abs(prate - r) <= max(1.6, 0.025 * r)
                if best:
                    mid, item = best[0], best[1]['item']
                else:  # where 817 has it, if anywhere
                    mid, item = next(((m, i) for m in mids for i in iids if (m, i) in e817),
                                     (mids[0], iids[0]))
                r817 = e817.get((mid, item), '')
                src = '817' if best and item in from817.get(mid, ()) else 'mod0631'
                if best and best[2] == 'mvp_drops':
                    src += ' mvp'
                if not best:
                    verdict, ours, src = 'missing', '', ''
                    call = 'missing: page+817' if near(r817) else 'missing: page alone'
                else:
                    ours = best[1]['rate']
                    # mod0631's small rates sit one below 10000/N (1/1666 is 5, 1/5000 is 1)
                    verdict = 'agree' if near(ours) else 'differ'
                    if verdict == 'agree':
                        call = 'keep'
                    elif src == '817':
                        call = 'page (817 slot)'
                    elif near(r817):
                        call = 'page+817'
                    elif r817 == ours:
                        call = 'keep (817 = ours)'
                    else:
                        call = 'page alone'
                n[verdict] += 1
                if src == '817':
                    n['817 ' + verdict] += 1
                n[call] = n.get(call, 0) + 1
                print(f'{item}\t{iname}\t{mid}\t{mname}\t1/{nn}\t{prate}\t{ours}\t{src}\t{r817}\t{verdict}\t{call}')
    print(' '.join(f'{k} {v}' for k, v in n.items()), file=sys.stderr)


main()

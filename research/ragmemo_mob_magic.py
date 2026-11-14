#!/usr/bin/env python3
"""Ragu Memo's table of monster attack magic (jRO, at_monskill.html, captured 2003-12-05)
against our monster skills.

  ragmemo_mob_magic.py PAGE.html MOD0631_MOB_DB > out.tsv     (run from the repo root)

PAGE is the saved page (Shift-JIS), data:refs/web/ragmemo-2003/ragmemo_at_monskill_20031205.html
(found by research round 16); MOD0631_MOB_DB is mod0631's db/mob_db.txt, whose JName
column names the monsters as the page does. Each of the page's rows gives monster, skill and
level, a rate in percent ("10-30%": the second figure once the monster's HP is under 30%,
the page's own note; a third is mod0631's "attackpcge 2" row, or jAthena's
"longrangeattacked" for the Mutant Dragon), cast and wait in ms, and "不可" where the
cast cannot be interrupted. Rows with no rate are bosses the page has no numbers for.

For every row it prints what data/db/mob_skills.toml and mob_skills_faq.toml hold for that
monster and skill: our rate as stored (rolled against 1000, research/THREADS.md #23), cast,
delay, cancel, condition. A row of ours "matches" when rate/10, cast and delay equal the
page's. Summary counts on stderr.
"""
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SKILLS = {'ナパームビート': 11, 'ソウルストライク': 13, 'ファイヤーボルト': 19, 'ファイヤーウォール': 18,
          'ファイヤーボール': 17, 'マグナムブレイク': 7, 'メテオストーム': 83, 'コールドボルト': 14,
          'フロストダイバ': 15, 'ウォーターボール': 86, 'ストームガスト': 89, 'フロストノヴァ': 88,
          'ライトニングボルト': 20, 'ユピテルサンダー': 84, 'サンダーストーム': 21,
          'ロードオブヴァーミリオン': 85, 'ヘブンズドライブ': 91, 'アーススパイク': 90,
          '砂かけ': 149, 'ホーリーライト': 156, 'インベノム': 52}
# Page spellings that differ from mod0631's JName, by id.
MOBS = {'地プティ': [1155], '空プティ': [1156], '緑クッキー': [1265], '赤クッキー': [1246],
        'アナコンダーク': [1030], 'エドガー': [1115], 'マヤ': [1147], 'キャットナインテール': [1307],
        'ブラッドナイト': [1268], 'プリオニ': [1159]}


def page_rows(path):
    s = Path(path).read_bytes().decode('cp932', 'replace')
    mon = None
    for tr in re.findall(r'<TR>(.*?)</TR>', s, re.S):
        names = [re.sub('<[^>]+>', '', x).strip()
                 for x in re.findall(r'class="MonsterSkillName"[^>]*>(.*?)</TD>', tr, re.S)]
        params = [re.sub('<[^>]+>', '', x).strip()
                  for x in re.findall(r'class="MonsterSkillParam"[^>]*>(.*?)</TD>', tr, re.S)]
        if not names:
            continue
        if len(names) == 2 and names[0]:
            mon = names[0]
        sk = names[-1]
        m = re.match(r'(.*?)(?:Lv(\d+))?$', sk)
        yield mon, m.group(1), int(m.group(2) or 0), params


def main():
    jname = {}
    for line in Path(sys.argv[2]).read_bytes().decode('cp932', 'replace').splitlines():
        f = line.split(',')
        if len(f) > 3 and f[0].isdigit():
            jname.setdefault(f[2], []).append(int(f[0]))
    ours = []
    for name in ('mob_skills.toml', 'mob_skills_faq.toml'):
        for r in tomllib.load(open(ROOT / 'data/db' / name, 'rb'))['skill']:
            r['file'] = name.split('.')[0]
            ours.append(r)
    n = {'rows': 0, 'numbers': 0, 'match': 0, 'differ': 0, 'missing': 0, 'nomob': 0}
    print('monster\tids\tskill\tlv\tpage rate\tcast\twait\tinterrupt\tours (file lv rate cast delay cancel cond)\tverdict')
    for mon, sk, lv, p in page_rows(sys.argv[1]):
        n['rows'] += 1
        ids = MOBS.get(mon) or jname.get(mon, [])
        sid = SKILLS.get(sk)
        rate, cast, wait, intr = (p + ['', '', '', ''])[:4]
        have = [r for r in ours if r['mob'] in ids and r['skill'] == sid]
        desc = '; '.join(f"{r['file']} {r['lv']} {r['rate']} {r['cast']} {r['delay']} "
                         f"{'yes' if r['cancel'] else 'no'} {r['cond']}{'=' + str(r['cond_val']) if r['cond_val'] else ''}"
                         for r in have)
        if not ids:
            verdict = 'monster?'
            n['nomob'] += 1
        elif not have:
            verdict = 'missing'
            n['missing'] += 1
        elif not rate:
            verdict = '(no numbers)'
        else:
            n['numbers'] += 1
            pct = [int(x) for x in rate.rstrip('%').split('-')]
            ok = all(any(r['rate'] == q * 10 for r in have) for q in pct) and \
                any(str(r['cast']) == cast and str(r['delay']) == wait.split('-')[0] for r in have)
            verdict = 'match' if ok else 'differ'
            n[verdict] += 1
        print('\t'.join([mon, ','.join(map(str, ids)), sk, str(lv), rate, cast, wait, intr, desc, verdict]))
    print(' '.join(f'{k} {v}' for k, v in n.items()), file=sys.stderr)


main()

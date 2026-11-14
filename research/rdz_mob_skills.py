#!/usr/bin/env python3
"""RODataZone's iRO monster table (monsters.js, captured 2004-02-11) against our monster skills.

  rdz_mob_skills.py MONSTERS.js > out.tsv

Maps RODataZone's skill names to ours with tools/import/mob_skills_faq.tsv (the FAQ's names,
which RODataZone mostly shares) and a few more below, matches monsters by display name, and
writes every (monster, skill) pair RODataZone gives that data/db/mob_skills.toml and
mob_skills_faq.toml lack. Unmapped names and monsters are counted at the end (stderr).
"""
import collections, re, sys, tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
js = open(sys.argv[1], encoding='latin-1').read()
rows = [[x.replace("\\'", "'") for x in re.findall(r"'((?:[^'\\]|\\.)*)'", m.group(2))]
        for m in re.finditer(r"MONSTERS\[(\d+)\] = new Array\((.*)\);", js) if m.group(1) != '0']
sk = {s['name']: s['id'] for s in tomllib.load(open(ROOT / 'data/db/skills.toml', 'rb'))['skill']}
names = {}
for line in open(ROOT / 'tools/import/mob_skills_faq.tsv', encoding='utf-8'):
    if line.startswith('#') or not line.strip():
        continue
    f = line.rstrip('\n').split('\t')
    names.setdefault(f[0].lower(), [n for n in f[2].split('|') if n in sk])
MORE = {'emotion': ['NPC_EMOTION'], 'summon minions': ['NPC_SUMMONSLAVE'], 'heal': ['AL_HEAL'],
        'teleport': ['AL_TELEPORT'], 'two-hand quicken': ['KN_TWOHANDQUICKEN'], 'increase agi': ['AL_INCAGI'],
        'decrease agi': ['AL_DECAGI'], 'maximise power': ['BS_MAXIMIZE'], 'adrenaline rush': ['BS_ADRENALINE'],
        'magnum break': ['SM_MAGNUM'], 'bash': ['SM_BASH'], 'lightning bolt': ['MG_LIGHTNINGBOLT'],
        "heaven's drive": ['WZ_HEAVENDRIVE'], 'fire bolt': ['MG_FIREBOLT'], 'cold bolt': ['MG_COLDBOLT'],
        'frost driver': ['MG_FROSTDIVER'], 'stone curse': ['MG_STONECURSE'], 'sonic blow': ['AS_SONICBLOW'],
        'envenom': ['TF_POISON'], 'hiding': ['AS_HIDING', 'TF_HIDING'], 'cloaking': ['AS_CLOAKING'],
        'lex divina': ['PR_LEXDIVINA'], 'safety wall': ['MG_SAFETYWALL'], 'fire wall': ['MG_FIREWALL'],
        'napalm beat': ['MG_NAPALMBEAT'], 'jupitel thunder': ['WZ_JUPITEL'], 'waterball': ['WZ_WATERBALL'],
        'quagmire': ['WZ_QUAGMIRE'], 'meteor storm': ['WZ_METEOR'], 'earth spike': ['WZ_EARTHSPIKE'],
        'fire ball': ['MG_FIREBALL'], 'spear stab': ['KN_SPEARSTAB'], 'brandish spear': ['KN_BRANDISHSPEAR'],
        'spear boomerang': ['KN_SPEARBOOMERANG'], 'pierce': ['KN_PIERCE'], 'charge arrow': ['AC_CHARGEARROW'],
        'double strafe': ['AC_DOUBLE'], 'venom dust': ['AS_VENOMDUST'], 'provoke': ['SM_PROVOKE'],
        'mammonite': ['MC_MAMMONITE'], 'counter attack': ['KN_AUTOCOUNTER'], 'back slide': ['TF_BACKSLIDING'],
        'stone throw': ['TF_THROWSTONE'], 'sand attack': ['TF_SPRINKLESAND'], 'recovery': ['PR_STRECOVERY'],
        'endure': ['SM_ENDURE']}
for k, v in MORE.items():
    names.setdefault(k, [n for n in v if n in sk])
have = collections.defaultdict(set)
for f in ('mob_skills.toml', 'mob_skills_faq.toml'):
    for r in tomllib.load(open(ROOT / 'data/db' / f, 'rb'))['skill']:
        have[r['mob']].add(r['skill'])
mobs = {}
for m in tomllib.load(open(ROOT / 'data/db/mobs.toml', 'rb'))['mob']:
    if m.get('base_exp'):
        mobs.setdefault(re.sub(r'[^a-z]', '', m['display'].lower()), m['id'])
unmapped, unmatched, n_have, missing = collections.Counter(), [], 0, []
for r in rows:
    mid = mobs.get(re.sub(r'[^a-z]', '', r[0].lower()))
    if not mid:
        unmatched.append(r[0])
        continue
    for s in [x.strip() for x in r[22].split(',') if x.strip() and x.strip() != '-']:
        base = re.sub(r'\s*\(.*\)$', '', s).lower()
        ids = [sk[n] for n in names.get(base, [])]
        if not ids:
            unmapped[base] += 1
            continue
        if any(i in have[mid] for i in ids):
            n_have += 1
        else:
            missing.append((r[0], mid, s))
print('# RODataZone monster\tour id\tits skill name (pairs our tables lack)')
for m in missing:
    print('\t'.join(map(str, m)))
print(f"# {n_have} pairs present, {len(missing)} missing; {sum(unmapped.values())} unmapped "
      f"({len(unmapped)} names: {', '.join(n for n, _ in unmapped.most_common(25))}); "
      f"{len(unmatched)} monsters unmatched", file=sys.stderr)

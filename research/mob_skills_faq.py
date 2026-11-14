#!/usr/bin/env python3
"""Compare a period monster-skill list with data/db/mob_skills.toml.

unknown13's "Monster Skill FAQ" (GameFAQs 29608, v1.0 of 2004-04, iRO data written up by a
pRO player; saved by research round 9 under research/agy-threads/r9/pages/). Prints, for
each (skill, monster) pair the FAQ lists, whether mod0631's table has it.

  python3 -I research/mob_skills_faq.py PAGE REPO_ROOT
"""
import re, sys, html, tomllib, unicodedata
page, root = sys.argv[1], sys.argv[2]
t = open(page, encoding='utf-8', errors='replace').read()
t = html.unescape(re.sub(r'<[^>]+>', '', t)).replace('–', '-').replace('�', '-')
L = t.split('\n')
start = next(i for i, l in enumerate(L) if 'MONSTER SKILLS' in l)
end = next(i for i, l in enumerate(L) if 'THAT\'S ALL' in l)
SK = {
 'Decrease Agility': 'AL_DECAGI', 'Heal': 'AL_HEAL', 'Holy Light': 'AL_HOLYLIGHT', 'Increase Agility': 'AL_INCAGI',
 'Lex Divina': 'PR_LEXDIVINA', 'Lex Aeterna': 'PR_LEXAETERNA', 'Pneuma': 'AL_PNEUMA', 'Status Recovery': 'PR_STRECOVERY',
 'Safety Wall': 'MG_SAFETYWALL', 'Teleport': 'AL_TELEPORT', 'Ankle Snare': 'HT_ANKLESNARE', 'Arrow Repel': 'AC_CHARGEARROW',
 'Arrow Shower': 'AC_SHOWER', 'Claymore Trap': 'HT_CLAYMORETRAP', 'Double Strafe': 'AC_DOUBLE', 'Freezing Trap': 'HT_FREEZINGTRAP',
 'Sandman': 'HT_SANDMAN', 'Shockwave Trap': 'HT_SHOCKWAVE', 'Skid Trap': 'HT_SKIDTRAP', 'Cold Bolt': 'MG_COLDBOLT',
 'Earth Spike': 'WZ_EARTHSPIKE', 'Fire Ball': 'MG_FIREBALL', 'Fire Bolt': 'MG_FIREBOLT', 'Fire Wall': 'MG_FIREWALL',
 'Frost Diver': 'MG_FROSTDIVER', "Heaven`s Drive": 'WZ_HEAVENDRIVE', 'Heavens Drive': 'WZ_HEAVENDRIVE', 'Jupital Thunder': 'WZ_JUPITEL',
 'Lightening Bolt': 'MG_LIGHTNINGBOLT', 'Lightning Bolt': 'MG_LIGHTNINGBOLT', 'Meteor Storm': 'WZ_METEOR', 'Napalm Beat': 'MG_NAPALMBEAT',
 'Quagmire': 'WZ_QUAGMIRE', 'Sight': 'MG_SIGHT', 'Soul Strike': 'MG_SOULSTRIKE', 'Water Ball': 'WZ_WATERBALL',
 'Adrenaline Rush': 'BS_ADRENALINE', 'Hammer Fall': 'BS_HAMMERFALL', 'Mammonite': 'MC_MAMMONITE', 'Maximize Power': 'BS_MAXIMIZE',
 'Bash': 'SM_BASH', 'Brandish Spear': 'KN_BRANDISHSPEAR', 'Counter Attack': 'KN_AUTOCOUNTER', 'Endure': 'SM_ENDURE',
 'Magnum Break': 'SM_MAGNUM', 'Pierce': 'KN_PIERCE', 'Provoke': 'SM_PROVOKE', 'Spear Boomerang': 'KN_SPEARBOOMERANG',
 'Back Sliding': 'TF_BACKSLIDING', 'Cloaking': 'AS_CLOAKING', 'Detoxify': 'TF_DETOXIFY', 'Envenom': 'TF_POISON',
 'Grimtooth': 'AS_GRIMTOOTH', 'Hiding': 'TF_HIDING', 'Sand Attack': 'TF_SPRINKLESAND', 'Sonic Blow': 'AS_SONICBLOW',
 'Stone Fling': 'TF_THROWSTONE', 'Poison Attack': 'NPC_POISON', 'Venom Dust': 'AS_VENOMDUST', 'Changes to a Random Attribute': 'NPC_ATTRICHANGE',
 'Changes to Shadow Attribute': 'NPC_CHANGEDARKNESS', 'Changes to Fire Attribute': 'NPC_CHANGEFIRE',
 'Changes to Ghost Attribute': 'NPC_CHANGETELEKINESIS', 'Changes to Water Attribute': 'NPC_CHANGEWATER',
 'Silence Attack': 'NPC_SILENCEATTACK', 'Sleep Attack': 'NPC_SLEEPATTACK', 'Stun Attack': 'NPC_STUNATTACK',
 'Blind Attack': 'NPC_BLINDATTACK', 'Blind': 'NPC_BLINDATTACK', 'Suicide': 'NPC_SUICIDE', 'Self-destruction': 'NPC_SELFDESTRUCTION',
 'Consecutive Attack': 'NPC_COMBOATTACK', 'Critical Slash': 'NPC_CRITICALSLASH', 'Guided Attack': 'NPC_GUIDEDATTACK',
 'Splash Attack': 'NPC_SPLASHATTACK', 'Magical Attack': 'NPC_DARKNESSATTACK|NPC_TELEKINESISATTACK|NPC_MAGICALATTACK',
 'Piercing Attack': 'NPC_PIERCINGATT', 'Mental Breaker': 'NPC_MENTALBREAKER', 'Stone Curse': 'NPC_PETRIFYATTACK|MG_STONECURSE',
 'Lazy Attack': 'NPC_SLOWCAST', 'Lord of Vermilion': 'WZ_VERMILION', 'Curse': 'NPC_CURSEATTACK', 'Spear Stab': 'KN_SPEARSTAB',
 'Thunder Storm': 'MG_THUNDERSTORM', 'Storm Gust': 'WZ_STORMGUST',
}
sk = tomllib.load(open(f'{root}/data/db/skills.toml', 'rb'))
sid = {s['name']: s['id'] for s in sk['skill']}
mobs = tomllib.load(open(f'{root}/data/db/mobs.toml', 'rb'))['mob']
ms = tomllib.load(open(f'{root}/data/db/mob_skills.toml', 'rb'))['skill']
have = {}
for r in ms: have.setdefault(r['mob'], set()).add(r['skill'])
def norm(s): return re.sub(r'[^a-z]', '', s.lower())
ALIAS = {'clocktowerkeeper': 'clocktowermanager', 'savageworm': 'sageworm?', 'earthpetite': 'petit', 'skypetite': 'skypetit',
         'desertwolfbaby': 'desertwolfpuppy', 'abysmalknight': 'abyssknight', 'cobold': 'kobold', 'cobaldarcher': 'koboldarcher',
         'cobaldarcher': 'koboldarcher', 'koboldarcher': 'koboldarcher', 'axegoblin': 'goblin', 'axecobold': 'kobold', 'axekobold': 'kobold',
         'zealotus': 'zealotus?', 'jealotus': 'zealotus?', 'jealotous': 'zealotus?', 'malethiefbug': 'thiefbugmale', 'pestraggler': 'pest',
         'ridingword': 'ridingword?', 'grimtooth': 'grimtooth?', 'bigfoot': 'bigfoot', 'stalacticgolem': 'stalactitegolem', 'megalice': 'megalith',
         'hatii': 'garm', 'goldthiefbug': 'goldenthiefbug', 'dopplegangger': 'doppelganger', 'doppleganger': 'doppelganger', 'bloodyknight': 'bloodknight',
         'fest': 'fest?', 'angelring': 'angeling', 'anacondaq': 'anacondaq', 'jealotous': 'zealotus', 'jealotus': 'zealotus', 'matyr': 'matyr',
         'cobold': 'kobold', 'giearth': 'giearth', 'dopplegangger': 'doppelganger', 'doppleganger': 'doppelganger',
         'goldthiefbug': 'goldenthiefbug', 'riding word': 'ridingword', 'farmiliar': 'familiar', 'skelprisoner': 'skelprisoner'}
byname = {}
for m in mobs:
    for n in (m.get('display') or '', m.get('name') or ''):
        byname.setdefault(norm(n), []).append(m['id'])
def mob_ids(name):
    n = norm(re.sub(r'\(.*?\)', '', name)); n = ALIAS.get(n, n)
    if n.endswith('?'): return None
    return byname.get(n)
pairs = []  # (skill text, mob text)
cur = None; boss = False; secB = False
for l in L[start:end]:
    if re.match(r'\s*2\. Bosses', l): boss = True
    if l.strip().startswith('*') or not l.strip(): cur = None if not boss else cur; continue
    if boss:
        m = re.match(r'\s+[A-O]\. (.+?) - (.+)', l)
        if m: cur = m.group(1); items = m.group(2)
        elif cur and l.startswith('     '): items = l
        else: continue
        for s in items.split(','):
            s = s.strip()
            if s: pairs.append((s, cur))
        continue
    if re.match(r'\s+B\. Original', l): secB = True
    m = re.match(r'\s+(?:B\.\d )?\s*([A-Z][^-]+?) - (.+)', l)
    if m and m.group(1).strip() in SK:
        cur = m.group(1).strip(); rest = m.group(2)
        if secB and cur == 'Venom Dust': cur = 'Poison Attack'
    elif cur and re.match(r'\s{10,}\S', l): rest = l
    else: continue
    for mob in rest.split(','):
        mob = mob.strip()
        if mob: pairs.append((cur, mob))
ok = miss = unk = 0; out = []
for s, mob in pairs:
    ids = mob_ids(mob)
    names = SK.get(s)
    if not ids or not names: out.append(f'?  {s} / {mob} (unmatched {"mob" if not ids else "skill"})'); unk += 1; continue
    want = [sid[n] for n in names.split('|') if n in sid]
    if any(w in have.get(i, ()) for i in ids for w in want): ok += 1
    else: miss += 1; out.append(f'NO {s} / {mob} {ids}')
print(f'{len(pairs)} pairs: {ok} in ours, {miss} not, {unk} unmatched')
print('\n'.join(out))

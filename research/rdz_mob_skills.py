#!/usr/bin/env python3
"""RODataZone's iRO monster table (monsters.js, captured 2004-02-11) against our monster skills.

  rdz_mob_skills.py MONSTERS.js > out.tsv
  rdz_mob_skills.py --tsv MONSTERS.js > tools/import/mob_skills_rdz.tsv

--tsv writes the missing pairs in mob_skills_faq.tsv's columns, for tools/import/mob_skills_faq.py.

Maps RODataZone's skill names to ours with tools/import/mob_skills_faq.tsv (the FAQ's names,
which RODataZone mostly shares) and a few more below, matches monsters by display name, and
writes every (monster, skill) pair RODataZone gives that data/db/mob_skills.toml and
mob_skills_faq.toml lack. Unmapped names and monsters are counted at the end (stderr).
"""
import collections, re, sys, tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TSV = sys.argv[1] == '--tsv'
js = open(sys.argv[-1], encoding='latin-1').read()
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
        'endure': ['SM_ENDURE'],
        # the elemental attacks (184-191: their elements checked in skills.toml, 190 Shadow, 191 Ghost)
        'water element attack': ['NPC_WATERATTACK'], 'earth element attack': ['NPC_GROUNDATTACK'],
        'fire element attack': ['NPC_FIREATTACK'], 'wind element attack': ['NPC_WINDATTACK'],
        'wind attribute attack': ['NPC_WINDATTACK'], 'poison element attack': ['NPC_POISONATTACK'],
        'holy element attack': ['NPC_HOLYATTACK'], 'dark element attack': ['NPC_DARKNESSATTACK'],
        'darkness element attack': ['NPC_DARKNESSATTACK'], 'darkness attack': ['NPC_DARKNESSATTACK'],
        'ghost element attack': ['NPC_TELEKINESISATTACK'], 'random element attack': ['NPC_RANDOMATTACK'],
        'multi attack': ['NPC_COMBOATTACK'], 'curse attack': ['NPC_CURSEATTACK'], 'curse': ['NPC_CURSEATTACK'],
        'life drain': ['NPC_BLOODDRAIN'],
        # jRO's HP吸収(魔法打撃) is Energy Drain (mod0631 gives it to exactly RODataZone's six
        # "HP Absorption" monsters), and the SP drain is Mental Breaker (jAthena //0970, 7189b01,
        # 2004-05-14, gives it to exactly the three with "SP Decrease Attack"; so does the FAQ)
        'hp absorption': ['NPC_ENERGYDRAIN'],
        'sp decrease attack': ['NPC_MENTALBREAKER'], 'magic attack': ['NPC_MAGICALATTACK'],
        'critical attack': ['NPC_CRITICALSLASH'], 'critical slash': ['NPC_CRITICALSLASH'],
        'self-destruct': ['NPC_SELFDESTRUCTION'], 'suicide': ['NPC_SUICIDE', 'NPC_SELFDESTRUCTION'],
        'summon followers': ['NPC_SUMMONSLAVE'], 'summon follower': ['NPC_SUMMONSLAVE'],
        'summon monster': ['NPC_SUMMONMONSTER', 'NPC_SUMMONSLAVE'], 'throw stone': ['TF_THROWSTONE'],
        'keeping': ['NPC_KEEPING'], 'smoking': ['NPC_SMOKING'], 'hallucination': ['NPC_HALLUCINATION'],
        'hullucination': ['NPC_HALLUCINATION'], 'metamorphosis': ['NPC_METAMORPHOSIS'], 'lick': ['NPC_LICK'],
        'mental breaker': ['NPC_MENTALBREAKER'], 'pierce attack': ['NPC_PIERCINGATT'],
        'fire element change': ['NPC_CHANGEFIRE'], 'water element change': ['NPC_CHANGEWATER'],
        'random element change': ['NPC_ATTRICHANGE'], 'agi up': ['AL_INCAGI'], 'soul strike': ['MG_SOULSTRIKE'],
        'sight': ['MG_SIGHT'], 'pneuma': ['AL_PNEUMA'], 'detoxify': ['TF_DETOXIFY'], 'holy light': ['AL_HOLYLIGHT'],
        'lord of vermillion': ['WZ_VERMILION'], 'storm gust': ['WZ_STORMGUST'], 'frost nova': ['WZ_FROSTNOVA'],
        'thunder storm': ['MG_THUNDERSTORM'], 'sandman': ['HT_SANDMAN'], 'flasher': ['HT_FLASHER'],
        'freeze trap': ['HT_FREEZINGTRAP'], 'skid trap': ['HT_SKIDTRAP'], 'claymore trap': ['HT_CLAYMORETRAP'],
        'shockwave trap': ['HT_SHOCKWAVE'], 'ankle snare': ['HT_ANKLESNARE'], 'arrow shower': ['AC_SHOWER'],
        'hammerfall': ['BS_HAMMERFALL'], 'hammer': ['BS_HAMMERFALL'], 'firewall': ['MG_FIREWALL'],
        'firebolt': ['MG_FIREBOLT'], 'lex aeterna': ['PR_LEXAETERNA'], 'grimtooth': ['AS_GRIMTOOTH']}
# not skills: pet tricks are emotes the client plays, "plantlike" is the monster's mode
SKIP = {'pet trick', 'pet tricks', 'plantlike', '?', 'etc.', 'etc'}
for k, v in MORE.items():
    names.setdefault(k, [n for n in v if n in sk])
have = collections.defaultdict(set)
for f in ('mob_skills.toml', 'mob_skills_faq.toml'):
    for r in tomllib.load(open(ROOT / 'data/db' / f, 'rb'))['skill']:
        if '(RODataZone' not in r.get('faq', ''):  # rows taken from this script's own pairs
            have[r['mob']].add(r['skill'])
mobs = {}
for m in tomllib.load(open(ROOT / 'data/db/mobs.toml', 'rb'))['mob']:
    if m.get('base_exp'):
        mobs.setdefault(re.sub(r'[^a-z]', '', m['display'].lower()), m['id'])
for m in tomllib.load(open(ROOT / 'data/db/mobs.toml', 'rb'))['mob']:
    if m.get('base_exp'):  # Athena's own names: ASSULTER, TIRFING, PERMETER, SOLIDER...
        mobs.setdefault(re.sub(r'[^a-z]', '', m['name'].lower()), m['id'])
# RODataZone's names for the rest. Goblins and Kobolds by weapon as tools/import/mob_skills_faq.tsv
# reads them from the client's sprites (goblin_1 knife, _2 flail, _3 axe, _4 hammer, _5 morning
# star; kobold_1 axe, _2 hammer, _3 morning star); the Tower Keeper is the Clock Tower Manager
# (THREADS.md #32). Its training-ground Fabre is not a monster of ours.
ALIAS = {'Abyssal Knight': 1219, 'Bloody Knight': 1268, 'Creamy Fire*': 1293, 'Desert Wolf (Baby)': 1107,
         'General Skeleton': 1290, 'Goblin (Dagger)': 1122, 'Goblin (Chain)': 1123, 'Goblin (Axe)': 1124,
         'Goblin (Hammer)': 1125, 'Goblin (Morningstar)': 1126, 'Goblin (Xmas)': 1245, 'Kobold (Axe)': 1133,
         'Kobold (Hammer)': 1134, 'Kobold (Morningstar)': 1135, 'Hatii': 1252, 'Leid Olmai': 1306,
         'Moonlight Flower': 1150, 'Mutant Dragonoid': 1262, 'Petite (Ground)': 1155, 'Petite (Sky)': 1156,
         'Picky (Egg)': 1050, 'Pierre': 1160, 'Reydric': 1163, 'Reydric Archer': 1276, 'Savage Bebe': 1167,
         'Seal': 1317, 'Tower Keeper': 1270, 'Zealotus': 1200, 'Arclouze': 1194}
for k, v in ALIAS.items():
    mobs[re.sub(r'[^a-z]', '', k.lower())] = v
unmapped, unmatched, n_have, missing = collections.Counter(), [], 0, []
for r in rows:
    mid = mobs.get(re.sub(r'[^a-z]', '', r[0].lower()))
    if not mid:
        unmatched.append(r[0])
        continue
    for s in [x.strip() for x in r[22].split(',') if x.strip() and x.strip() != '-']:
        base = re.sub(r'\s*\(.*$', '', s).lower().rstrip('?')
        if base in SKIP or not base or base[0] in '0123456789' or ')' in base:
            continue  # a pet trick, or the tail of a note RODataZone split at its comma
        ids = [sk[n] for n in names.get(base, [])]
        if not ids:
            unmapped[base] += 1
            continue
        if any(i in have[mid] for i in ids):
            n_have += 1
        else:
            missing.append((r[0], mid, s, '|'.join(names[base])))
if TSV:
    print("# RODataZone's iRO monster table (monsters/monsters.js, Wayback 20040211150750), the (skill,\n"
          "# monster) pairs mod0631 and the FAQ lack. Written by research/rdz_mob_skills.py --tsv; read by\n"
          "# tools/import/mob_skills_faq.py. Columns as mob_skills_faq.tsv: its skill, its monster, our\n"
          "# skill names, our monster id, note.")
    for name, mid, s, ours in missing:
        print(f"{s}\t{name}\t{ours}\t{mid}\tRODataZone")
else:
    print('# RODataZone monster\tour id\tits skill name (pairs our tables lack)')
    for m in missing:
        print('\t'.join(map(str, m[:3])))
print(f"# {n_have} pairs present, {len(missing)} missing; {sum(unmapped.values())} unmapped "
      f"({len(unmapped)} names: {', '.join(n for n, _ in unmapped.most_common(25))}); "
      f"{len(unmatched)} monsters unmatched", file=sys.stderr)

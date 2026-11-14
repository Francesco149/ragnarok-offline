#!/usr/bin/env python3
"""Monster skills that iRO had in April 2004 and mod0631's table lacks.

mod0631's db/mob_skill_db.txt (December 2003, data/db/mob_skills.toml) is jRO's monster skills
as its authors knew them. unknown13's Monster Skill FAQ (GameFAQs 29608, April 2004) lists
what iRO's monsters used then; tools/import/mob_skills_faq.tsv holds its pairs. For every
pair mod0631 lacks, this takes the rows of the first table below that has that skill on one
of the pair's candidate monsters:

  1. jAthena //0953 (2004-04-27; its table says 2004/04/28): Kalen's monster skill database,
     merged 2004-02-20 (//0822). Tier athena2003.
  2. jAthena //1046 (2004-09-12). Tier later.
  3. rAthena's pre-renewal table (refs/rathena HEAD). Tier later.

Rates: mod0631 rolls rand()%1000 against a row's rate (map/mob.c:2168); from //0822 on
jAthena rolls rand()%10000 with the same numbers (THREADS.md #23). The game keeps mod0631's
roll, so jAthena's numbers go in unchanged, on the scale of the rows they sit beside.
rAthena's are five times jAthena's for the rows both have (the median), so they are divided
by five to land on that scale.

A pair that no table has (the later tables were reworked: rAthena gives Golem Auto Guard where
the FAQ has Endure) gets the commonest row of that skill on other monsters in the first table
that has the skill at all, tier guess: the FAQ says the monster used it, the row says how
monsters used that skill. Pairs whose monster or skill the game lacks are listed at the end of
the output file as comments.

The boss rule. The FAQ ends with a part it says it took from iRO: "All boss monsters will use the
Two-Hand Quicken, Increase Agility, Guided Attack, and Piercing Attack depending on the
probability" and "All boss monsters will Teleport if they are attacked from beyond their attacking
range". A boss is an MVP (a monster with MVP EXP): jAthena //0953 gives Two-Hand Quicken to exactly
those, Lord of Death aside. The four skills become pairs for every MVP and go through the same
search. Teleport gets one row each, on "rudeattacked": a hit from beyond the monster's attack range
from someone it cannot walk to (rAthena's reading of the condition, src/map/mob.cpp:1936-1976, tier
later; the server raises it in combat.rs damage_mob). The FAQ names no chance for it, so it always
fires.

  python3 tools/import/mob_skills_faq.py REFS REPO_ROOT
writes data/db/mob_skills_faq.toml.
"""
import statistics
from collections import Counter
import subprocess
import sys
import tomllib
from pathlib import Path

REFS, ROOT = Path(sys.argv[1]), Path(sys.argv[2])

SOURCES = [
    ("jathena", "8e060a8", "db/mob_skill_db.txt", "athena2003",
     "jAthena //0953 (2004-04-27) db/mob_skill_db.txt", 1.0),
    ("jathena", "6332410", "db/mob_skill_db.txt", "later",
     "jAthena //1046 (2004-09-12) db/mob_skill_db.txt", 1.0),
    ("rathena", "HEAD", "db/pre-re/mob_skill_db.txt", "later",
     "rAthena pre-renewal db/mob_skill_db.txt", None),  # scale worked out below
]
# rAthena's states and targets that mod0631's AI has under another name
STATE = {"follow": "chase", "angry": "attack", "anytarget": "any"}
TARGET = {"randomtarget": "target"}


def table(repo, rev, path):
    raw = subprocess.run(["git", "-C", str(REFS / repo), "show", f"{rev}:{path}"],
                         check=True, capture_output=True).stdout
    rows = []
    for line in raw.decode("cp932", "replace").splitlines():
        line = line.split("//")[0] if line.lstrip().startswith("//") else line
        c = [x.strip() for x in line.split(",")]
        if len(c) < 12 or not c[0].isdigit() or not c[3].isdigit():
            continue
        rows.append(c)
    return rows


def names_of(skills):
    return "/".join(n for n, i in sid.items() if i in skills)


def num(s):
    try:
        return int(s)
    except ValueError:
        return 0


ours = tomllib.load(open(ROOT / "data/db/mob_skills.toml", "rb"))["skill"]
have = {(r["mob"], r["skill"]) for r in ours}
mod_rate = {(r["mob"], r["state"], r["skill"]): r["rate"] for r in ours}
sid = {s["name"]: s["id"] for s in tomllib.load(open(ROOT / "data/db/skills.toml", "rb"))["skill"]}
mobdb = tomllib.load(open(ROOT / "data/db/mobs.toml", "rb"))["mob"]
mobs = {m["id"] for m in mobdb}
mvps = sorted((m["id"], m["name"]) for m in mobdb if m.get("mvp_exp", 0) > 0)
BOSS_SKILLS = [("Two-Hand Quicken", "KN_TWOHANDQUICKEN"), ("Increase Agility", "AL_INCAGI"),
               ("Guided Attack", "NPC_GUIDEDATTACK"), ("Piercing Attack", "NPC_PIERCINGATT")]
BOSS_SRC = ("unknown13's Monster Skill FAQ (GameFAQs 29608, 2004-04) quoting iRO: All boss monsters "
            "will Teleport if they are attacked from beyond their attacking range; the condition as "
            "rAthena reads it (src/map/mob.cpp:1936-1976)")

tables = []
for repo, rev, path, tier, src, scale in SOURCES:
    rows = table(repo, rev, path)
    if scale is None:
        first = {(num(c[0]), c[2], num(c[3])): num(c[5]) for c in tables[0][0]}
        ratios = [num(c[5]) / first[k] for c in rows
                  if (k := (num(c[0]), c[2], num(c[3]))) in first and first[k]]
        scale = statistics.median(ratios)
    tables.append((rows, tier, src, scale))

pairs = []
for line in open(ROOT / "tools/import/mob_skills_faq.tsv", encoding="utf-8"):
    if line.startswith("#") or not line.strip():
        continue
    f = line.rstrip("\n").split("\t")
    faq_skill, faq_mob, names, ids = f[0], f[1], f[2], f[3]
    skills = [sid[n] for n in names.split("|") if n in sid]
    cands = [int(i) for i in ids.split(",") if i and int(i) in mobs]
    pairs.append((faq_skill, faq_mob, skills, cands))
for faq_skill, name in BOSS_SKILLS:
    for mid, mname in mvps:
        pairs.append((faq_skill, f"{mname} (every boss)", [sid[name]], [mid]))
# The client's own monster action table (data\monsterskillinfo.xml, Atbash-encoded, shipped with
# the Turtle Island patch of 2003-04-30; tier client) gives these monsters an animation for these
# skills, so Gravity's monsters used them. Only pairs with a row in a period table are taken here;
# NPC_PROVOCATION (a taunt, on 28 monsters there) is left out: this client shouts "Unknown Skill !!"
# for it (THREADS.md #23).
CLIENT_PAIRS = [("Adrenaline Rush", "Hunter Fly (client action table)", "BS_ADRENALINE", 1035),
                ("Stone Fling", "Choco (client action table)", "TF_THROWSTONE", 1214)]
for faq_skill, faq_mob, name, mid in CLIENT_PAIRS:
    pairs.append((faq_skill, faq_mob, [sid[name]], [mid]))

out, missing, seen, nfound = [], [], set(), 0
for faq_skill, faq_mob, skills, cands in pairs:
    if not cands or not skills:
        missing.append(f"{faq_skill} / {faq_mob}: no monster or skill of that name")
        continue
    if any((m, s) in have for m in cands for s in skills):
        continue  # mod0631 has it
    for rows, tier, src, scale in tables:
        got = [c for c in rows if num(c[0]) in cands and num(c[3]) in skills]
        if got:
            break
    else:
        got = []
        for rows, tier, src, scale in tables:
            same = [tuple(c[2:12]) for c in rows if num(c[3]) in skills]
            if same:
                shape = Counter(same).most_common(1)[0][0]
                got = [[str(m), ""] + list(shape) for m in cands]
                tier, src = "guess", f"the commonest of {len(same)} {names_of(skills)} rows in {src}"
                break
        if not got:
            missing.append(f"{faq_skill} / {faq_mob}: no table has the skill")
            continue
    nfound += 1
    for c in got:
        key = (num(c[0]), c[2], num(c[3]), c[5], c[10], c[11])
        if key in seen:
            continue
        seen.add(key)
        rate = round(num(c[5]) / scale)
        rec = [f"mob = {num(c[0])}", f'state = "{STATE.get(c[2], c[2])}"', f"skill = {num(c[3])}",
               f"lv = {num(c[4])}", f"rate = {max(rate, 1)}", f"cast = {num(c[6])}", f"delay = {num(c[7])}",
               f"cancel = {'true' if c[8] in ('yes', '1') else 'false'}",
               f'target = "{TARGET.get(c[9], c[9])}"', f'cond = "{c[10]}"', f"cond_val = {num(c[11])}"]
        if len(c) > 12 and num(c[12]):
            rec.append(f"val = {num(c[12])}")
        rec += [f'tier = "{tier}"', f'source = "{src}"', f'faq = "{faq_skill} / {faq_mob}"']
        out.append("[[skill]]\n" + "\n".join(rec))

# Ragu Memo's table of monster attack magic (jRO, http://ragmemo.s33.xrea.com/at_monskill.html,
# Wayback 20031205093859; tier period) gives rate %, cast and wait for 76 (monster, skill) rows
# with numbers. 71 are mod0631's rows exactly, rate/10 = the page's percent
# (research/ragmemo_mob_magic.py). "10-30%" is the rate, then the rate once HP is under 30%; a
# third figure is mod0631's "attackpcge 2" row, or (Mutant Dragon) jAthena's "longrangeattacked".
# These are the rows the page has and our tables lack; where jAthena //0953 has the same row,
# its state is used.
RAGMEMO_SRC = "Ragu Memo at_monskill.html (jRO, Wayback 2003-12-05)"
RAGMEMO = [
    (1140, "attack", "MG_FIREBOLT", 5, 300, 4000, 10000, True, "myhpltmaxrate", 30,
     "Marduk Fire Bolt Lv5 10-30%; mod0631 has the 10% row, this is the 30% one as its others"),
    (1262, "chase", "MG_FIREBALL", 7, 800, 0, 0, True, "longrangeattacked", 0,
     "Mutant Dragon Fire Ball Lv7 10-30-80%; the 80% row as jAthena //0953 has it"),
    (1263, "chase", "WZ_JUPITEL", 3, 300, 4000, 10000, False, "myhpltmaxrate", 30,
     "Wind Ghost Jupitel Thunder Lv3 10-30%, uninterruptible under 30% HP; as jAthena //0953"),
    (1309, "chase", "SM_MAGNUM", 10, 400, 2000, 0, False, "always", 0,
     "Gajomart Magnum Break Lv10 40-100%; the 40% row as jAthena //0953 has it"),
    (1297, "attack", "TF_POISON", 10, 250, 1500, 0, False, "always", 0,
     "Ancient Mummy Envenom Lv10 25%, cast 1500, wait 0, uninterruptible"),
]
# The page's cast time where a row taken above says otherwise.
RAGMEMO_CAST = {(1033, sid["MG_NAPALMBEAT"]): 1000}  # Elder Willow Napalm Beat: jAthena 500
for i, rec in enumerate(out):
    m = rec.split("\n")
    mob, skill = int(m[1].split(" = ")[1]), int(m[3].split(" = ")[1])
    if (mob, skill) in RAGMEMO_CAST:
        out[i] = rec.replace(m[6], f"cast = {RAGMEMO_CAST[mob, skill]}") + \
            f'\nnote = "cast time from {RAGMEMO_SRC}"'
for mob, state, name, lv, rate, cast, delay, cancel, cond, cond_val, how in RAGMEMO:
    out.append("[[skill]]\n" + "\n".join([
        f"mob = {mob}", f'state = "{state}"', f"skill = {sid[name]}", f"lv = {lv}", f"rate = {rate}",
        f"cast = {cast}", f"delay = {delay}", f"cancel = {'true' if cancel else 'false'}",
        'target = "target"', f'cond = "{cond}"', f"cond_val = {cond_val}", 'tier = "period"',
        f'source = "{RAGMEMO_SRC}"', f'faq = "{how}"']))

nboss = 0
for mid, mname in mvps:
    nboss += 1
    out.append("[[skill]]\n" + "\n".join([
        f"mob = {mid}", 'state = "any"', f"skill = {sid['AL_TELEPORT']}", "lv = 1", "rate = 1000",
        "cast = 0", "delay = 0", "cancel = true", 'target = "self"', 'cond = "rudeattacked"',
        "cond_val = 0", 'tier = "later"', f'source = "{BOSS_SRC}"',
        f'faq = "Teleport / {mname} (every boss)"']))

hdr = f'''# Generated by tools/import/mob_skills_faq.py: do not edit; change the script or
# tools/import/mob_skills_faq.tsv and run it again.
[meta]
tier = "athena2003"
source = "unknown13's Monster Skill FAQ (GameFAQs 29608, 2004-04) for which monster has which skill; rows from {SOURCES[0][4]}, else {SOURCES[1][4]}, else {SOURCES[2][4]} (rate divided by {tables[2][3]:g})"
note = "monster skills iRO had in April 2004 that mod0631 lacks; read after mob_skills.toml, same fields; each row says its own tier and source"

'''
foot = "\n# FAQ pairs left out:\n" + "".join(f"#   {m}\n" for m in missing)
(ROOT / "data/db/mob_skills_faq.toml").write_text(hdr + "\n\n".join(out) + "\n" + foot, encoding="utf-8")
print(f"{nfound} FAQ pairs added as {len(out)} rows (with {nboss} boss Teleports); {len(missing)} left out")

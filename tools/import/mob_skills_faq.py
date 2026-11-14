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
mobs = {m["id"] for m in tomllib.load(open(ROOT / "data/db/mobs.toml", "rb"))["mob"]}

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

hdr = f'''# Generated by tools/import/mob_skills_faq.py: do not edit; change the script or
# tools/import/mob_skills_faq.tsv and run it again.
[meta]
tier = "athena2003"
source = "unknown13's Monster Skill FAQ (GameFAQs 29608, 2004-04) for which monster has which skill; rows from {SOURCES[0][4]}, else {SOURCES[1][4]}, else {SOURCES[2][4]} (rate divided by {tables[2][3]:g})"
note = "monster skills iRO had in April 2004 that mod0631 lacks; read after mob_skills.toml, same fields; each row says its own tier and source"

'''
foot = "\n# FAQ pairs left out:\n" + "".join(f"#   {m}\n" for m in missing)
(ROOT / "data/db/mob_skills_faq.toml").write_text(hdr + "\n\n".join(out) + "\n" + foot, encoding="utf-8")
print(f"{nfound} FAQ pairs added as {len(out)} rows; {len(missing)} left out")

#!/usr/bin/env python3
import sys

p_m631 = "/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/db/mob_skill_db.txt"
p_817 = "/data/refs/eathena-archive/x/athena-817-src/athena/db/mob_skill_db.txt"

def load_skills(p):
    skills = []
    with open(p, "rb") as f:
        text = f.read().decode("latin1", errors="replace")
    for line in text.splitlines():
        line = line.strip()
        if not line or line.startswith("//"):
            continue
        parts = [x.strip() for x in line.split(",")]
        skills.append(parts)
    return skills

s_m631 = load_skills(p_m631)
s_817 = load_skills(p_817)

print(f"mod0631 mob skills count: {len(s_m631)}")
print(f"817 mob skills count: {len(s_817)}")

# Count unique mobs
mobs_m631 = set(s[0] for s in s_m631 if s)
mobs_817 = set(s[0] for s in s_817 if s)

print(f"Unique mobs in mod0631: {len(mobs_m631)}")
print(f"Unique mobs in 817: {len(mobs_817)}")

# Are they identical?
diff_m631_only = len(set(tuple(s) for s in s_m631) - set(tuple(s) for s in s_817))
diff_817_only = len(set(tuple(s) for s in s_817) - set(tuple(s) for s in s_m631))

print(f"Skills in mod0631 not in 817: {diff_m631_only}")
print(f"Skills in 817 not in mod0631: {diff_817_only}")

print("\nSample mod0631 skill entries:")
for s in s_m631[:5]:
    print(" ", s)

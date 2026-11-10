import subprocess
import re
import os

REPO = "/data/refs/jathena"

def run_git(args):
    cmd = ["git", "-C", REPO] + args
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return res.stdout

# Let's search git log for job quests and 2-1 jobs:
# Knight, Priest, Wizard, Blacksmith, Hunter, Assassin
jobs = ["knight", "priest", "wizard", "blacksmith", "hunter", "assassin", "2nd"]

print("=== Git commits mentioning 2-1 jobs in commit message ===")
out = run_git(["log", "--since=2003-10-01", "--until=2004-07-01", "--date=short", "--pretty=format:%h|%ad|%an|%s"])
lines = out.decode("cp932", errors="replace").splitlines()

job_kws = ["騎士", "プリースト", "ウィザード", "ブラックスミス", "ハンター", "アサシン", "転職", "knight", "priest", "wizard", "blacksmith", "hunter", "assassin", "job_2nd"]
for l in lines:
    parts = l.split("|")
    if len(parts) >= 4:
        h, d, a, s = parts[0], parts[1], parts[2], "|".join(parts[3:])
        for kw in job_kws:
            if kw.lower() in s.lower():
                print(f"{h} | {d} | {a} | {s}")
                break

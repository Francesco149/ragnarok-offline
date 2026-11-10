import os
import re
import subprocess
import csv

REPO = "/data/refs/jathena"

# Target paths: files under conf/, conf/npc/, conf/gvg/, script/npc/
# We can find all commits affecting files matching these paths up to 2004-06-30.

def run_git(args):
    cmd = ["git", "-C", REPO] + args
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return res.stdout

# Get all commits up to 2004-06-30
# git log --since=2003-10-01 --until=2004-06-30 --name-only --pretty=format:"COMMIT_START|%h|%ad|%an|%s%n%b%nFILES_START" --date=short
out = run_git(["log", "--since=2003-10-01", "--until=2004-07-01", "--name-only", "--pretty=format:COMMIT_START|%h|%ad|%an|%s%n%b%nFILES_START", "--date=short"])
text = out.decode("cp932", errors="replace")

commits = text.split("COMMIT_START|")[1:]
print(f"Total commits in range: {len(commits)}")

records = []

OFFICIAL_KEYWORDS = [
    ("凸さんのログ", "Made from 凸's logs"),
    ("本鯖", "Official server"),
    ("実鯖", "Real server"),
    ("Aegis", "Aegis"),
    ("オフィシャル", "Official"),
    ("公式", "Official"),
    ("ログを基に", "Based on logs"),
    ("ログ", "Logs"),
]

def check_official(msg):
    hits = []
    for kw, en in OFFICIAL_KEYWORDS:
        if kw in msg:
            hits.append(f"{kw} ({en})")
    if hits:
        # Check if it's explicitly from Aegis or official logs
        return "yes", "; ".join(hits)
    return "no", ""

print("Parsed commits successfully.")

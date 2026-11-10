#!/usr/bin/env python3
import subprocess
import re
import csv
import os

REPO = "/data/refs/jathena"
OUT_TSV = "/work/research/agy-threads/r2/t2-jathena-history.tsv"

def run_git(args):
    cmd = ["git", "-C", REPO] + args
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return res.stdout.decode("utf-8", errors="replace")

# Get list of commits from 2003-10-01 to 2004-07-01
# git log --since=2003-10-01 --until=2004-07-01 --reverse --pretty=format:"COMMIT|%H|%h|%ad|%an|%s" --date=short
raw_log = run_git(["log", "--since=2003-10-01", "--until=2004-07-01", "--reverse", "--pretty=format:COMMIT|%H|%h|%ad|%an|%s", "--date=short"])

commits = []
for line in raw_log.splitlines():
    if line.startswith("COMMIT|"):
        parts = line.split("|")
        commits.append({
            "full_hash": parts[1],
            "hash": parts[2],
            "date": parts[3],
            "author": parts[4],
            "subject": parts[5]
        })

print(f"Total commits in timeframe: {len(commits)}")

# Keywords indicating official logs / server
OFFICIAL_KW = [
    (r"凸さんのログ", "凸's logs"),
    (r"本鯖", "official server"),
    (r"実鯖", "real server"),
    (r"Aegis", "Aegis"),
    (r"オフィシャル", "official"),
    (r"公式", "official"),
    (r"ログを基に", "based on logs"),
    (r"ログより", "from logs"),
    (r"本鯖仕様", "official server specs"),
    (r"本鯖通り", "per official server"),
    (r"本鯖準拠", "per official server"),
]

def check_official_log(msg):
    hits = []
    for pat, desc in OFFICIAL_KW:
        if re.search(pat, msg):
            hits.append(desc)
    if hits:
        if any("Aegis" in h for h in hits):
            return "yes (Aegis Chinese-big5 leak)", "; ".join(hits)
        return "yes", "; ".join(hits)
    return "no", ""

# Simple dictionary for common Japanese commit phrase translations
JA_EN_PHRASES = {
    "追加": "Add",
    "修正": "Fix",
    "変更": "Change",
    "削除": "Delete",
    "更新": "Update",
    "対応": "Support",
    "新規": "New",
    "作成": "Create",
    "転職": "Job change",
    "本鯖": "Official server",
    "仕様": "Specification",
    "台詞": "Dialogue",
    "ワープ": "Warp",
    "スキル": "Skill",
}

def translate_message(msg):
    # Short summary translation
    lines = [l.strip() for l in msg.splitlines() if l.strip()]
    if not lines:
        return ""
    trans_lines = []
    for l in lines[:3]: # First 3 lines
        tl = l
        for j, e in JA_EN_PHRASES.items():
            tl = tl.replace(j, e)
        trans_lines.append(tl)
    return " / ".join(trans_lines)

rows = []

# Regex for target paths: conf/npc_*.txt, conf/npc/*.txt, conf/gvg/*.txt, conf/agit/*.txt, script/npc/*.txt
TARGET_PATH_PAT = re.compile(r"^(conf/npc_.*\.txt|conf/npc/.*\.txt|conf/gvg/.*\.txt|conf/agit/.*\.txt|script/npc/.*\.txt)$")

for c in commits:
    chash = c["full_hash"]
    # Get full commit message and modified files
    c_details = run_git(["show", "--name-only", "--pretty=format:%B---FILES---", chash])
    parts = c_details.split("---FILES---")
    body = parts[0].strip()
    files = parts[1].strip().splitlines() if len(parts) > 1 else []

    # Extract revision tag //NNNN from subject or body
    rev_m = re.search(r"//(\d+)", c["subject"]) or re.search(r"//(\d+)", body)
    rev = f"//{rev_m.group(1)}" if rev_m else c["hash"]

    # Check official logs
    from_official, kw_hits = check_official_log(body)

    for f in files:
        f = f.strip()
        if TARGET_PATH_PAT.match(f):
            # Extract relevant portion of commit message for this file if possible
            msg_ja = c["subject"]
            # Look for lines mentioning the file or category
            relevant_lines = [l.strip() for l in body.splitlines() if os.path.basename(f) in l or "npc" in l.lower()]
            if relevant_lines:
                msg_ja = " | ".join(relevant_lines)
            elif body:
                msg_ja = body.replace("\n", " ")

            msg_en = translate_message(msg_ja)
            official_field = f"{from_official} [{kw_hits}]" if from_official.startswith("yes") else "no"

            rows.append([
                f, rev, c["date"], c["author"], msg_ja, msg_en, official_field
            ])

print(f"Total TSV rows: {len(rows)}")

headers = ["path", "rev", "date", "author", "message_ja", "message_en", "from_official_logs"]
with open(OUT_TSV, "w", encoding="utf-8", newline="") as fh:
    writer = csv.writer(fh, delimiter="\t")
    writer.writerow(headers)
    for r in rows:
        writer.writerow(r)

print(f"Written to {OUT_TSV} successfully.")

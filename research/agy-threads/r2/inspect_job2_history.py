#!/usr/bin/env python3
import subprocess
import re
import sys

REPO = "/data/refs/jathena"

def run_git(args):
    cmd = ["git", "-C", REPO] + args
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return res.stdout.decode("utf-8", errors="replace")

# Let's search for commits touching job_2nd or individual job files
# Let's see commits modifying conf/npc_job_2nd.txt, conf/npc/npc_job_2nd.txt, or specific job files
log_job2 = run_git(["log", "--since=2003-10-01", "--until=2004-07-01", "--reverse", "--pretty=format:%h|%ad|%an|%s", "--", "conf/npc_job_2nd.txt", "conf/npc/npc_job_2nd.txt", "script/npc/job/npc_job_2nd.txt"])

print("=== Commits touching job_2nd ===")
for l in log_job2.splitlines():
    print(l)

# Let's see what each commit changed in the 2-1 jobs!
# Specifically Knight, Priest, Wizard, Blacksmith, Hunter, Assassin
EOF

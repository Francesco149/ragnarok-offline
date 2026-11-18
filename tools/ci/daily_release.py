#!/usr/bin/env python3
"""Keep the rolling `nightly` release no more than a day behind master.

Owner rule (2026-11-18): when we work on the project, a release that is out of
date for more than a day is rebuilt and pushed. "Out of date": master has
commits the release was not built from. "More than a day": the last release
build started over 24 hours ago (so a busy day gives one build, not one per
commit). Then this pushes master (fast-forward only) and dispatches
.github/workflows/release.yml, which builds on GitHub and replaces the
release's files.

Called by tools/git-hooks/post-commit in the background after every commit
(log: .git/daily-release.log) and at session start (AGENTS.md). Cheap and
idempotent when there is nothing to do.

  python3 tools/ci/daily_release.py            # check, act if due
  python3 tools/ci/daily_release.py --dry-run  # say what it would do
  python3 tools/ci/daily_release.py --force    # ignore the 24 h wait

Rules:
  - a build queued or running = nothing to do;
  - the last successful build is of master's head = up to date;
  - the last build (any outcome) started less than 24 h ago = not due yet;
  - a build of master's head that failed is not retried for the same commit
    (gh run view <id> --log-failed);
  - only a clean fast-forward push of `master`, from a checkout of master:
    never force, never another branch;
  - a work in progress that leaves the game broken lives on a branch
    (AGENTS.md), so master is always fit to release.
"""
import argparse
import datetime as dt
import json
import os
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
WORKFLOW = "release.yml"
DUE_AFTER = dt.timedelta(hours=24)


def sh(*args, check=True):
    r = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
    if check and r.returncode != 0:
        raise SystemExit(f"daily-release: {' '.join(args)} failed: {r.stderr.strip() or r.stdout.strip()}")
    return r


def log(msg):
    stamp = dt.datetime.now().astimezone().strftime("%Y-%m-%d %H:%M:%S %z")
    print(f"[{stamp}] daily-release: {msg}", flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--force", action="store_true")
    a = ap.parse_args()

    if sh("git", "rev-parse", "--abbrev-ref", "HEAD").stdout.strip() != "master":
        log("not on master; skipped")
        return 0
    head = sh("git", "rev-parse", "master").stdout.strip()
    now = dt.datetime.now(dt.timezone.utc)

    r = sh("gh", "run", "list", "--workflow", WORKFLOW, "-L", "20",
           "--json", "createdAt,headSha,status,conclusion,databaseId", check=False)
    if r.returncode != 0 and "not found on the default branch" in r.stderr:
        runs = []  # the workflow is not on GitHub yet: never built
    elif r.returncode != 0:
        raise SystemExit(f"daily-release: gh run list failed: {r.stderr.strip()}")
    else:
        runs = json.loads(r.stdout or "[]")
    for r in runs:
        r["created"] = dt.datetime.fromisoformat(r["createdAt"].replace("Z", "+00:00"))
    active = [r for r in runs if r["status"] != "completed"]
    if active:
        log(f"build {active[0]['databaseId']} ({active[0]['headSha'][:8]}) is {active[0]['status']}; nothing to do")
        return 0
    ok = [r for r in runs if r["conclusion"] == "success"]
    if ok and ok[0]["headSha"] == head:
        log(f"the release is of master's head ({head[:8]}); up to date")
        return 0
    if runs and not a.force:
        last = runs[0]
        age = now - last["created"]
        if age < DUE_AFTER:
            left = DUE_AFTER - age
            log(f"last build {last['databaseId']} started {age.total_seconds() / 3600:.1f} h ago; "
                f"next one due in {left.total_seconds() / 3600:.1f} h")
            return 0
        if last["headSha"] == head and last["conclusion"] != "success":
            log(f"build {last['databaseId']} of {head[:8]} failed; not retrying the same commit "
                f"(gh run view {last['databaseId']} --log-failed)")
            return 1

    sh("git", "fetch", "-q", "origin", "master")
    if sh("git", "merge-base", "--is-ancestor", "origin/master", "master", check=False).returncode != 0:
        log("origin/master is not an ancestor of master (diverged); not pushing: needs a human")
        return 1
    ahead = int(sh("git", "rev-list", "--count", "origin/master..master").stdout.strip())

    if a.dry_run:
        log(f"would push master ({ahead} commits ahead, {head[:8]}) and dispatch {WORKFLOW}")
        return 0
    if ahead:
        sh("git", "push", "origin", "master:master")
        log(f"pushed master ({ahead} commits, {head[:8]})")
    # a workflow pushed for the first time takes GitHub a moment to register
    for attempt in range(7):
        r = sh("gh", "workflow", "run", WORKFLOW, "--ref", "master", check=attempt == 6)
        if r.returncode == 0:
            break
        time.sleep(10)
    log(f"dispatched {WORKFLOW} for {head[:8]}; release: "
        "https://github.com/Francesco149/ragnarok-offline/releases/tag/nightly")
    return 0


if __name__ == "__main__":
    sys.exit(main())

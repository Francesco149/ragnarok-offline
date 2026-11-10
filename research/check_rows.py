#!/usr/bin/env python3
"""Check research rows against the sources they cite. No row is believed until this passes it.

  check_rows.py rows.jsonl [more.jsonl ...]     check; web rows against the saved page
  check_rows.py --fetch rows.jsonl              also load each web source again and compare
  check_rows.py --pages DIR ...                 where saved pages live (default: next to the rows, pages/)

A row is one JSON object per line (research/agy-threads/HANDOFF-r3.md has the rules):

  {"id": "W2-014", "claim": "...", "value": "10000", "grade": "B", "quote": "set Zeny,Zeny-10000;",
   "source": {"kind": "file", "path": "data:refs/.../conf/gvg/x.txt", "line": 81}}

Source kinds, and what is checked for each:

  file   path [, line] [, encoding]     the file exists; the quote is in it; near `line` if one is given
  git    rev, path [, repo] [, line]    the revision exists in the repo (default data:refs/jathena); the quote is
                                        in that file at that revision
  grf    name [, grf]                   the quote is in that file inside the client's GRF
                                        (default data:play/RagnarokOnline/data.grf.original)
  web    url, page                      `page` is the copy you saved under pages/; the quote is in it. With
                                        --fetch the URL is loaded again: the quote must be in what comes back, and
                                        for a Wayback URL the capture served must be the one cited
  none   searched                       a negative result ("looked here, it is not there"). Not checked; counted

Paths: `repo:x` is this repository, `data:x` is ~/data/ragnarok-cd; /work/x and /data/x (the workbench's
names) mean the same. Text is compared after removing markup and collapsing whitespace, in the file's own
encoding (utf-8, cp932, cp949 and latin-1 are tried). A quote must be at least 8 characters.

Also checked: the id is unique; grades A and B carry a quote that contains `value` (set "value_in_quote":
false to say it does not, and then the grade must be C or D).

Result per row: PASS, WARN (it is there, but something is off: other line, other capture, value not in the
quote) or FAIL. Exit status 1 if any row fails. A PASS says the words are in the source. Whether they mean
what the claim says is still for a reader to judge.
"""
import html
import json
import os
import re
import subprocess
import sys
import unicodedata
import urllib.request

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DATA = os.path.expanduser("~/data/ragnarok-cd")
ENCODINGS = ("utf-8", "cp932", "cp949", "latin-1")
NEAR = 40  # lines either side of a cited line


def real(path):
    for prefix, root in (("repo:", REPO), ("data:", DATA), ("/work/", REPO), ("/data/", DATA)):
        if path.startswith(prefix):
            mapped = os.path.join(root, path[len(prefix):])
            if prefix.startswith("/") and os.path.exists(path):
                return path
            return mapped
    return os.path.expanduser(path)


def norm(text):
    text = re.sub(r"(?is)<(script|style)\b.*?</\1>", " ", text)
    text = re.sub(r"<[^>]{0,2000}>", " ", text)
    text = unicodedata.normalize("NFKC", html.unescape(text))
    return re.sub(r"\s+", " ", text).strip().casefold()


def decodings(raw, encoding=None):
    for enc in ([encoding] if encoding else ENCODINGS):
        try:
            yield raw.decode(enc)
        except (UnicodeDecodeError, LookupError):
            continue


def find(raw, quote, line=None, encoding=None):
    """-> (found, note). Looks for the quote in every decoding that works."""
    q = norm(quote)
    seen_any = False
    for text in decodings(raw, encoding):
        if q not in norm(text):
            continue
        seen_any = True
        if line is None:
            return True, ""
        lines = text.splitlines()
        lo, hi = max(0, line - 1 - NEAR), min(len(lines), line + NEAR)
        if q in norm("\n".join(lines[lo:hi])):
            return True, ""
        first = norm(quote.strip().splitlines()[0])
        at = next((i + 1 for i, l in enumerate(lines) if first and first in norm(l)), None)
        return True, f"not near line {line}" + (f"; it is at line {at}" if at else "")
    return seen_any, "" if seen_any else "quote not in source"


def grf_cat(grf, name):
    sys.path.insert(0, os.path.join(REPO, "research"))
    import gat_cells
    return gat_cells.cat(grf, name)


def wayback_ts(url):
    m = re.search(r"web\.archive\.org/web/(\d{14})", url)
    return m.group(1) if m else None


def fetch(url):
    """-> (bytes, final_url). Wayback URLs are loaded in their id_ form: the page as it was."""
    raw_url = re.sub(r"(web\.archive\.org/web/\d{14})(?:[a-z]{2}_)?/", r"\1id_/", url)
    req = urllib.request.Request(raw_url, headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) check-rows"})
    with urllib.request.urlopen(req, timeout=90) as r:
        return r.read(), r.geturl()


def check(row, pages, do_fetch):
    """-> (status, note)"""
    src = row.get("source") or {}
    kind = src.get("kind")
    quote = row.get("quote") or ""
    if kind == "none":
        return ("NONE", "") if src.get("searched") else ("FAIL", "a negative result must say where it searched")
    if len(norm(quote)) < 8:
        return "FAIL", "no quote, or one shorter than 8 characters"
    warn = []
    grade = str(row.get("grade", "")).upper()
    value = str(row.get("value", ""))
    squash = lambda s: re.sub(r"[\s,.]", "", norm(s))
    if row.get("value_in_quote") is False:
        if grade in ("A", "B"):
            return "FAIL", "grade A/B needs the value in the quote"
    elif value and grade in ("A", "B") and squash(value) not in squash(quote):
        warn.append("value is not in the quote")

    if kind == "file":
        path = real(src.get("path", ""))
        if not os.path.isfile(path):
            return "FAIL", f"no such file: {src.get('path')}"
        ok, note = find(open(path, "rb").read(), quote, src.get("line"), src.get("encoding"))
    elif kind == "git":
        repo = real(src.get("repo", "data:refs/jathena"))
        rev = str(src.get("rev", ""))
        if subprocess.run(["git", "-C", repo, "cat-file", "-e", rev + "^{commit}"], capture_output=True).returncode:
            return "FAIL", f"no such revision: {rev}"
        if src.get("path"):
            blob = subprocess.run(["git", "-C", repo, "show", f"{rev}:{src['path']}"], capture_output=True)
            if blob.returncode:
                return "FAIL", f"{src['path']} does not exist at {rev}"
            ok, note = find(blob.stdout, quote, src.get("line"), src.get("encoding"))
        else:  # the commit message
            msg = subprocess.run(["git", "-C", repo, "log", "-1", "--format=%an%n%ad%n%B", "--date=short", rev],
                                 capture_output=True).stdout
            ok, note = find(msg, quote)
    elif kind == "grf":
        grf = real(src.get("grf", "data:play/RagnarokOnline/data.grf.original"))
        try:
            raw = grf_cat(grf, src.get("name", ""))
        except SystemExit as e:
            return "FAIL", str(e)
        if raw is None:
            return "FAIL", f"{src.get('name')} is not in {os.path.basename(grf)}"
        ok, note = find(raw, quote, src.get("line"), src.get("encoding"))
    elif kind == "web":
        url = src.get("url", "")
        if not re.match(r"https?://", url):
            return "FAIL", "no URL"
        page = os.path.join(pages, os.path.basename(src.get("page", "") or "-"))
        if not os.path.isfile(page):
            return "FAIL", f"saved page missing: {src.get('page')}"
        ok, note = find(open(page, "rb").read(), quote)
        if ok and do_fetch:
            try:
                raw, final = fetch(url)
            except Exception as e:  # noqa: BLE001 - any network failure is the same finding
                warn.append(f"could not load it again ({type(e).__name__}: {e})")
            else:
                again, _ = find(raw, quote)
                if not again:
                    return "FAIL", "quote is in the saved page but not in what the URL serves"
                cited, served = wayback_ts(url), wayback_ts(final)
                if cited and served and cited != served:
                    warn.append(f"Wayback serves capture {served}, not {cited}")
    else:
        return "FAIL", f"unknown source kind: {kind!r}"

    if not ok:
        return "FAIL", note
    if note:
        warn.append(note)
    return ("WARN", "; ".join(warn)) if warn else ("PASS", "")


def main():
    args = sys.argv[1:]
    do_fetch = "--fetch" in args
    args = [a for a in args if a != "--fetch"]
    pages = None
    if "--pages" in args:
        i = args.index("--pages")
        pages = args[i + 1]
        del args[i:i + 2]
    if not args:
        sys.exit(__doc__)
    counts, seen = {}, set()
    for path in args:
        pdir = pages or os.path.join(os.path.dirname(os.path.abspath(path)), "pages")
        for n, line in enumerate(open(path, encoding="utf-8"), 1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
                rid = str(row["id"])
            except (ValueError, KeyError, TypeError) as e:
                status, note, rid = "FAIL", f"not a row: {e}", f"{os.path.basename(path)}:{n}"
            else:
                if rid in seen:
                    status, note = "FAIL", "duplicate id"
                else:
                    seen.add(rid)
                    status, note = check(row, pdir, do_fetch)
            counts[status] = counts.get(status, 0) + 1
            print(f"{status}\t{rid}\t{note}")
    total = sum(counts.values())
    print("# " + ", ".join(f"{counts.get(k, 0)} {k}" for k in ("PASS", "WARN", "FAIL", "NONE")) + f" of {total} rows",
          file=sys.stderr)
    sys.exit(1 if counts.get("FAIL") else 0)


if __name__ == "__main__":
    main()

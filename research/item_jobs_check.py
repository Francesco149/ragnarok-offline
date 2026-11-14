#!/usr/bin/env python3
"""Compare who may equip an item, as the client's descriptions say, with data/db/items.toml.

  ragnarok-offline grf cat DATA.GRF 'data\\idnum2itemdesctable.txt' > desc.txt
  item_jobs_check.py desc.txt [--all]     (run from the repo root)

The client (tier client) writes "Applicable Job :" (gear) or "Available Job :" (hats)
per item. Lists wrap onto indented lines. Where the client names only first jobs
(and the Novice), only those bits are compared: "Archer Thief" for a bow says nothing of
the Assassin. "Every Job" or "All" is all, "Except Novice" all but the Novice. Our `jobs` is the
athena job mask; bits 13 and 21 (riding Knight and Crusader), 22 (wedding) and 23 (Super
Novice, not in this client) are left out of the comparison. Prints one line per item that
disagrees: what the client allows that we do not (+) and what we allow that the client
does not (-). --all also prints the lines it cannot read.

  item_jobs_check.py desc.txt --rdz DIR

DIR holds RODataZone's iRO item tables of 2004-02 (data:refs/web/rodatazone-2004/, as for
research/rdz_items_check.py; matched by display name). Prints instead the `CLIENT_IRO_JOBS`
table of tools/import/athena_import.py: a job changes where the client and RODataZone both
disagree with ours on it. A first job dropped takes its second jobs with it unless
RODataZone still lists them; a first job added brings the second jobs RODataZone lists;
a 2-2 job follows its 2-1 sibling (Crusader the Knight, Monk the Priest, Sage the Wizard,
Rogue the Assassin, Alchemist the Blacksmith, Bard and Dancer the Hunter). Where the client
names second jobs (knuckles: "Monk"), those are compared too. It compares with the
items.toml on disk, which has the table applied: to rebuild the table, regenerate with
CLIENT_IRO_JOBS empty first (mod0631's masks). With it applied, --rdz prints nothing.
research/THREADS.md #31.
"""
import re
import sys
import tomllib

JOBS = ['Novice', 'Swordman', 'Mage', 'Archer', 'Acolyte', 'Merchant', 'Thief',
        'Knight', 'Priest', 'Wizard', 'Blacksmith', 'Hunter', 'Assassin', None,
        'Crusader', 'Monk', 'Sage', 'Rogue', 'Alchemist', 'Bard', 'Dancer']
BIT = {n: i for i, n in enumerate(JOBS) if n}
BIT['Theif'] = BIT['Thief']          # the client's spelling once
TREE = {1: [7, 14], 2: [9, 16], 3: [11, 19, 20], 4: [8, 15], 5: [10, 18], 6: [12, 17]}
COMPARED = {i for i, n in enumerate(JOBS) if n}
ALL = set(COMPARED)
FIRST = set(range(7))


def client_set(v):
    """The set of job bits a description line allows, or None if it cannot be read."""
    v = v.strip()
    low = v.lower()
    if low.startswith('available job'):            # "Available Job : Available Job : ..."
        v = v.split(':', 1)[1].strip()
        low = v.lower()
    if low in ('every job', 'all'):
        return ALL
    if 'except novice' in low:
        return ALL - {0}
    if low == '2nd class or above':
        return set(range(7, 21)) & COMPARED
    words = v.split()
    if not words or any(w not in BIT for w in words):
        return None
    s = set()
    for w in words:
        b = BIT[w]
        s.add(b)
        s.update(TREE.get(b, []))
    return s


RDZ_WORD = dict(BIT, Swordsman=1, Magician=2)
SIBLING = {14: 7, 15: 8, 16: 9, 17: 12, 18: 10, 19: 11, 20: 11}


def rdz_set(v):
    """RODataZone's "Used By" as job bits, or None."""
    low = v.strip().lower()
    if low == 'all':
        return ALL
    if low == 'all 2nd jobs':
        return set(range(7, 21)) & COMPARED
    if low == 'all except novice':
        return ALL - {0}
    if low == 'all but acolyte and priest':
        return ALL - {4, 8, 15}
    words = [w.strip() for w in v.split(',')]
    if any(w not in RDZ_WORD for w in words):
        return None
    return {RDZ_WORD[w] for w in words}


# RODataZone's spelling -> our display name (lower case, no apostrophes)
RDZ_ALIAS = {'waghnak': 'waghnakh', 'cutlus': 'cutlas', 'fireblend': 'fire brand',
             'ice falchion': 'ice falchon', 'high heels': 'highheals', 'goggles': 'goggle',
             'helm of the solar god': 'solar god helm', 'panda hat': 'panda cap',
             # paired by weight and DEF/ATK (2026-11-14); not "Kapra Band" (weight 10, ours 50)
             # nor "Mage Hat" (DEF 1, ours 5); not the Assassin Dagger either: both sources give
             # it the generic dagger list, a default rather than a statement about it
             'apple oarcher': 'apple o archer', 'brisingamen': 'brysinggamen',
             'bulb band': 'lightbulb hairband', 'diver goggles': 'divers goggles',
             'ear muffs': 'ear mufs', 'fashionable glasses': 'fashion sunglasses',
             'gangster mask': 'ganster mask', 'grampa beard': 'granpa beard', 'hallberd': 'halberd',
             'hat of petite diablo': 'deviruchi hat', 'iron cain': 'iron cane',
             'khatzbalger': 'katzbalger', 'magician hat': 'magic hat', 'megingard': 'magingiorde',
             'mjolnir': 'mjollnir', 'nut shell': 'apple hat', 'poll axe': 'pole axe',
             'purple sunglasses': 'purple glasses', 'raccon hat': 'smokie hat',
             'ranbow eggshell': 'spotted eggshell', 'santa hat': 'santas hat',
             'shaprpened legbone of ghoul': 'sharppened legbone of gh', 'stilleto': 'stiletto',
             'stripe hairband': 'striped bandana', 'blue hairband': 'blue bandana',
             'belt': 'ancient belt', 'jacket': 'leather jacket'}


def rdz_tables(d):
    import glob
    out = {}
    for f in sorted(glob.glob(d + '/*_items_*.js')):
        t = open(f, encoding='latin-1').read()
        for m in re.finditer(r"new Array\('([^']*)','((?:[^'\\]|\\.)*)','([^']*)'", t):
            name = m.group(2).replace("\\'", "'").lower().replace("'", '').rstrip('*').strip()
            a = re.match(r'(.*?)\s*\(ancient\)$', name)
            name = RDZ_ALIAS.get(name, 'ancient ' + a.group(1) if a else name)
            out.setdefault(name, m.group(3))
    return out


def merged(said, rdz, ours, second_named, named):
    """Our bits with the jobs the client and RODataZone agree on changed."""
    new = set(ours)
    for b in sorted(FIRST):
        if (b in said) == (b in rdz) != (b in ours):
            tree = [t for t in TREE.get(b, []) if t not in SIBLING]
            if b in said:
                new |= {b} | {t for t in tree if t in rdz}
            else:
                new -= {b} | {t for t in tree if t not in rdz}
    if second_named:
        for b in range(7, 21):
            if b in COMPARED and b not in SIBLING and (b in said) == (b in rdz) != (b in ours):
                new ^= {b}
    for b, sib in SIBLING.items():
        if b in named:
            if (b in said) == (b in rdz) != (b in ours):
                new ^= {b}
        elif (sib in new) != (sib in ours):
            new = new | {b} if sib in new else new - {b}
    return new


def names(bits):
    return ' '.join(JOBS[b] for b in sorted(bits))


def main():
    text = open(sys.argv[1], 'rb').read().decode('latin-1')
    show_all = '--all' in sys.argv
    rdz = rdz_tables(sys.argv[sys.argv.index('--rdz') + 1]) if '--rdz' in sys.argv else None
    items = {i['id']: i for i in tomllib.load(open('data/db/items.toml', 'rb'))['item']}
    n = bad = 0
    for m in re.finditer(r'(?m)^(\d+)#\r?\n(.*?)^#', text, re.S):
        iid, body = int(m.group(1)), re.sub(r'\^[0-9a-fA-F]{6}', '', m.group(2))
        f = re.search(r'(?im)^\s*(?:Applicable|Available) Jobs?\s*:\s*(.*?)\s*$((?:\r?\n[ \t]+\S.*$)*)',
                      body)
        it = items.get(iid)
        if not f or not it:
            continue
        line = ' '.join((f.group(1) + ' ' + f.group(2)).split())
        said = client_set(line)
        if said is None:
            if show_all:
                print(f"{iid}\t{it.get('display')}\tunread\t{line!r}")
            continue
        ours = {b for b in COMPARED if it.get('jobs', 0) >> b & 1}
        named = {BIT[w] for w in line.split() if w in BIT}
        second_named = bool(named - FIRST)
        if rdz is not None:
            r = rdz.get(str(it.get('display', '')).lower().replace("'", ''))
            r = r and rdz_set(r)
            if r is None:
                continue
            new = merged(said, r, ours, second_named, named)
            if new != ours:
                bad += 1
                mask = it.get('jobs', 0) & ~sum(1 << b for b in COMPARED) | sum(1 << b for b in new)
                plus, minus = new - ours, ours - new
                print(f"    {iid}: {mask:#x},  # {it.get('display')}"
                      + (f": + {names(plus)}" if plus else '') + (f"; - {names(minus)}" if minus else ''))
            n += 1
            continue
        if not second_named and said not in (ALL, ALL - {0}):
            # only first jobs named: the client says nothing exact of their second jobs
            said, ours = said & FIRST, ours & FIRST
        n += 1
        if said != ours:
            bad += 1
            plus, minus = said - ours, ours - said
            print(f"{iid}\t{it.get('display')}\tclient {line!r}"
                  + (f"\t+ {names(plus)}" if plus else '')
                  + (f"\t- {names(minus)}" if minus else ''))
    print(f"# {bad} of {n} items disagree", file=sys.stderr)


main()

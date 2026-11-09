#!/usr/bin/env python3
"""Import era Athena data into ragnarok-offline's own formats.

Numeric data comes from athena-dev-2.1.1-mod0631 (archived 2003-12-18, the same
week as the client); English display names from eAthena 0.5.2 (2004-05-20),
whose numbers are only used for records mod0631 lacks. Everything written is
tagged tier "athena2003" with the exact source file. See docs/PROVENANCE.md.

  athena_import.py <refs/eathena-archive/x> <repo root>

Writes data/db/{items,mobs,exp,jobs}.toml and scripts/import/{warps,spawns,shops}.lua.
The Athena sources are GPL-2.0-or-later; so are the generated files.
"""
import json
import re
import sys
from pathlib import Path

REFS = Path(sys.argv[1])
ROOT = Path(sys.argv[2])
M631 = REFS / "athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631"
E052 = REFS / "eathena-v0.5.2/athena"
SRC631 = "athena-dev-2.1.1-mod0631 (eathena.org/jathena, archived 2003-12-18)"
SRC052 = "eathena-v0.5.2 (eathena.org/eathena/archive, 2004-05-20)"


def read(p):
    return p.read_bytes().decode("cp932", "replace")


def csv_records(p):
    for line in read(p).splitlines():
        line = line.strip()
        if not line or line.startswith("//"):
            continue
        yield line


def split_item(line):
    """item_db: 17 plain columns then {use},{equip}."""
    m = re.match(r"^([^{]*?),(\{.*\}),(\{.*\})\s*$", line)
    if not m:
        return None
    cols = m.group(1).split(",")
    return cols, m.group(2)[1:-1].strip(), m.group(3)[1:-1].strip()


def num(s, default=0):
    s = s.strip()
    if not s:
        return default
    try:
        return int(s)
    except ValueError:
        return int(float(s))


def q(s):
    return json.dumps(s, ensure_ascii=False)


def lua_expr(a):
    a = a.strip()
    m = re.fullmatch(r"rand\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)", a)
    if m:
        return f"rand({m.group(1)}, {m.group(2)})"
    if re.fullmatch(r"-?\d+", a):
        return a
    return None


def split_args(s):
    """Split on commas outside parentheses."""
    parts, depth, cur = [], 0, ""
    for ch in s:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return [p.strip() for p in parts]


_SC = None


def sc_const(name):
    """SC_* number from mod0631 db/const.txt (case-insensitive, as Athena)."""
    global _SC
    if _SC is None:
        _SC = {}
        for line in csv_records(M631 / "db/const.txt"):
            m = re.match(r"(SC_\w+)[\s,]+(\d+)", line)
            if m:
                _SC[m.group(1).lower()] = int(m.group(2))
    return _SC.get(name.lower())


def translate_use(script):
    """Translate the common item-use statements; None if anything is unknown."""
    out = []
    for stmt in [s.strip() for s in script.split(";") if s.strip()]:
        m = re.fullmatch(r"(itemheal|percentheal)\s+(.+)", stmt)
        if m and len(split_args(m.group(2))) == 2:
            a, b = (lua_expr(x) for x in split_args(m.group(2)))
            if a is None or b is None:
                return None
            fn = "heal" if m.group(1) == "itemheal" else "percent_heal"
            out.append(f"{fn}({a}, {b})")
            continue
        m = re.fullmatch(r"sc_end\s+(SC_\w+)", stmt)
        if m and sc_const(m.group(1)) is not None:
            out.append(f"sc_end({sc_const(m.group(1))}) --[[{m.group(1)}]]")
            continue
        m = re.fullmatch(r"sc_start\s+(SC_\w+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)", stmt)
        if m and sc_const(m.group(1)) is not None:
            out.append(f"sc_start({sc_const(m.group(1))}, {m.group(2)}, {m.group(3)}) --[[{m.group(1)}]]")
            continue
        m = re.fullmatch(r'warp\s+"(\w+)"\s*,\s*(\d+)\s*,\s*(\d+)', stmt)  # Fly Wing, Butterfly Wing
        if m:
            out.append(f'return warp("{m.group(1)}", {m.group(2)}, {m.group(3)})')  # false: refused, not used up
            continue
        m = re.fullmatch(r"getitem\s+-(\d)\s*,\s*1", stmt)  # Old Blue Box & co
        if m:
            out.append(f"item_box({m.group(1)})")
            continue
        if re.fullmatch(r'monster\s+"this"\s*,\s*0\s*,\s*0\s*,\s*"[^"]*"\s*,\s*0\s*,\s*1\s*,\s*""', stmt):  # Dead Branch
            out.append("return dead_branch()")
            continue
        m = re.fullmatch(r'itemskill\s+(\d+)\s*,\s*(\d+)\s*,\s*"[^"]*"', stmt)
        if m:
            out.append(f"item_skill({m.group(1)}, {m.group(2)})")
            continue
        m = re.fullmatch(r"(pet|produce)\s+(\d+)", stmt)
        if m:
            out.append(f"{'catch_pet' if m.group(1) == 'pet' else 'produce'}({m.group(2)})")
            continue
        if stmt == "bpet":
            out.append("birth_pet()")
            continue
        return None
    return "; ".join(out)


def toml_kv(d):
    parts = []
    for k, v in d.items():
        if v is None or v == "" or v == 0 and k not in ("id",) and not k.startswith("rate"):
            continue
        if isinstance(v, str):
            parts.append(f"{k} = {q(v)}")
        elif isinstance(v, list):
            parts.append(f"{k} = [{', '.join(v)}]")
        else:
            parts.append(f"{k} = {v}")
    return "\n".join(parts)


def import_items():
    names = {}
    for line in csv_records(E052 / "db/item_db.txt"):
        r = split_item(line)
        if r and r[0][0].strip().isdigit():
            names[int(r[0][0])] = (r[0][2].strip(), r)
    rows = {}
    for line in csv_records(M631 / "db/item_db.txt"):
        r = split_item(line)
        if r and r[0][0].strip().isdigit():
            rows[int(r[0][0])] = (r, SRC631)
    for iid, (_, r) in names.items():
        if iid not in rows and iid > 0:
            rows[iid] = (r, SRC052)
    out = [
        "# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.",
        "# Numbers: athena mod0631 (2003-12-18); display names: eAthena 0.5.2. GPL-2.0-or-later data.",
        "[meta]",
        'tier = "athena2003"',
        f"source = {q(SRC631 + ' db/item_db.txt; display names from ' + SRC052)}",
        'note = "weight is in tenths; type: 0 healing 2 usable 3 etc 4 weapon 5 armor 6 card 7 egg 8 pet-equip 10 ammo"',
        "",
    ]
    for iid in sorted(rows):
        (cols, use, equip), src = rows[iid]
        cols += [""] * (17 - len(cols))
        lua = translate_use(use) if use else ""
        rec = {
            "id": iid,
            "name": cols[1].strip(),
            "display": names.get(iid, ("",))[0] or cols[1].strip().replace("_", " "),
            "type": num(cols[3]),
            "buy": num(cols[4]),
            "sell": num(cols[5], None) if cols[5].strip() else None,
            "weight": num(cols[6]),
            "atk": num(cols[7]),
            "def": num(cols[8]),
            "range": num(cols[9]),
            "slots": num(cols[10]),
            "jobs": num(cols[11]),
            "gender": num(cols[12], 2),
            "loc": num(cols[13]),
            "weapon_level": num(cols[14]),
            "equip_level": num(cols[15]),
            "view": num(cols[16]),
            "use_script": lua or "",
            "athena_use": use if (use and lua is None) else "",
            "athena_equip": equip,
        }
        out.append("[[item]]")
        out.append(toml_kv(rec))
        if rec["gender"] == 0:
            out.append("gender = 0")
        if src != SRC631:
            out.append('# record only in eAthena 0.5.2')
        out.append("")
    (ROOT / "data/db/items.toml").write_text("\n".join(out), encoding="utf-8")
    return len(rows)


MOB_COLS = ("id name jname level hp sp base_exp job_exp range atk1 atk2 def mdef str agi vit int dex luk "
            "view_range chase_range size race element mode speed attack_delay attack_motion damage_motion").split()


def import_mobs():
    eng = {}
    for line in csv_records(E052 / "db/mob_db.txt"):
        c = line.split(",")
        if c[0].strip().isdigit():
            eng[int(c[0])] = (c[2].strip(), c)
    rows = {}
    for line in csv_records(M631 / "db/mob_db.txt"):
        c = line.split(",")
        if c[0].strip().isdigit() and len(c) > 30 and c[3].strip():
            rows[int(c[0])] = (c, SRC631)
    for mid, (_, c) in eng.items():
        if mid not in rows and len(c) > 30 and c[3].strip():
            rows[mid] = (c, SRC052)
    out = [
        "# Generated by tools/import/athena_import.py. Numbers: athena mod0631 (2003-12-18);",
        "# display names: eAthena 0.5.2 (2004-05). GPL-2.0-or-later data.",
        "[meta]",
        'tier = "athena2003"',
        f"source = {q(SRC631 + ' db/mob_db.txt; display names from ' + SRC052)}",
        'note = "rate is per 10000; element = level*20+element? see docs/PROVENANCE.md; mode is the Athena bitmask"',
        "",
    ]
    for mid in sorted(rows):
        c, src = rows[mid]
        c = [x.strip() for x in c] + [""] * 60
        rec = {k: num(c[i]) for i, k in enumerate(MOB_COLS) if k not in ("name", "jname")}
        rec["name"] = c[1]
        rec["display"] = eng.get(mid, ("",))[0] or c[1].title()
        drops = []
        for k in range(8):
            iid, rate = num(c[29 + 2 * k]), num(c[30 + 2 * k])
            if iid > 0 and rate > 0:
                drops.append(f"{{ item = {iid}, rate = {rate} }}")
        rec["mvp_exp"] = num(c[47])
        mvp = []
        for k in range(3):
            iid, rate = num(c[49 + 2 * k]), num(c[50 + 2 * k])
            if iid > 0 and rate > 0:
                mvp.append(f"{{ item = {iid}, rate = {rate} }}")
        out.append("[[mob]]")
        out.append(toml_kv({k: rec[k] for k in ["id", "name", "display"] + [m for m in MOB_COLS if m not in ("id", "name", "jname")]}))
        if drops:
            out.append(f"drops = [{', '.join(drops)}]")
        if rec["mvp_exp"]:
            out.append(f"mvp_exp = {rec['mvp_exp']}")
        if mvp:
            out.append(f"mvp_drops = [{', '.join(mvp)}]")
        if src != SRC631:
            out.append("# record only in eAthena 0.5.2")
        out.append("")
    (ROOT / "data/db/mobs.toml").write_text("\n".join(out), encoding="utf-8")
    return len(rows)


def import_exp():
    cols = [[] for _ in range(6)]
    for line in csv_records(M631 / "db/exp.txt"):
        vals = [num(x) for x in line.split(",")]
        for i in range(min(6, len(vals))):
            cols[i].append(vals[i])
    out = [
        "# Generated by tools/import/athena_import.py from athena mod0631 db/exp.txt (2003-12-18).",
        "[meta]", 'tier = "athena2003"', f"source = {q(SRC631 + ' db/exp.txt')}",
        'note = "entry n = EXP needed to go from level n+1 to n+2; mod0631 columns 0-2 are identical base tables"', "",
        f"base = {cols[0]}", f"job_novice = {cols[3]}", f"job_first = {cols[4]}", f"job_second = {cols[5]}", "",
    ]
    (ROOT / "data/db/exp.toml").write_text("\n".join(out), encoding="utf-8")


JOBS = ["Novice", "Swordsman", "Magician", "Archer", "Acolyte", "Merchant", "Thief", "Knight", "Priest",
        "Wizard", "Blacksmith", "Hunter", "Assassin", "Knight (Peco)", "Crusader", "Monk", "Sage", "Rogue",
        "Alchemist", "Bard", "Dancer", "Crusader (Peco)"]


def import_jobs():
    db1 = [line for line in csv_records(M631 / "db/job_db1.txt")]
    db2 = [line for line in csv_records(M631 / "db/job_db2.txt")]
    out = ["# Generated by tools/import/athena_import.py from athena mod0631 db/job_db1.txt + job_db2.txt.",
           "[meta]", 'tier = "athena2003"', f"source = {q(SRC631 + ' db/job_db1.txt, db/job_db2.txt')}",
           'note = "weight in tenths; aspd_base per weapon type (bare,dagger,1hsword,2hsword,1hspear,2hspear,1haxe,2haxe,mace,2hmace,rod,bow,knuckle,instrument,whip,book,katar)"', ""]
    for i, name in enumerate(JOBS):
        if i >= len(db1):
            break
        c = [x.strip() for x in db1[i].split(",")]
        bonus = [num(x) for x in db2[i].split(",")] if i < len(db2) else []
        out += ["[[job]]", f"id = {i}", f"name = {q(name)}", f"weight = {num(c[0])}",
                f"hp_coef = {float(c[1] or 0)}", f"sp_coef = {num(c[2])}",
                f"aspd_base = [{', '.join(str(num(x)) for x in c[3:20])}]",
                f"job_bonus = {bonus}", ""]
    (ROOT / "data/db/jobs.toml").write_text("\n".join(out), encoding="utf-8")


# ---------------------------------------------------------------- skills
E817 = REFS / "athena-817-src/athena"


def import_skills():
    """mod0631 skill_db.txt + skill_tree.txt; quest flag (inf2 & 1) from 817, which added the column."""
    quest = set()
    for line in csv_records(E817 / "db/skill_db.txt"):
        cols = line.split("//")[0].split(",")
        if len(cols) >= 11 and num(cols[10]) & 1:
            quest.add(num(cols[0]))
    skills = []
    for line in csv_records(M631 / "db/skill_db.txt"):
        body, _, comment = line.partition("//")
        cols = [c.strip() for c in body.split(",")]
        if len(cols) < 9 or not cols[0].isdigit():
            continue
        name = comment.strip().split("#")[0].strip()
        sid, rng, hit, inf, ele, nk, mx = (num(c) for c in cols[:7])
        sp = [num(x) for x in cols[7].split(":")]
        if len(sp) == 1:
            sp = sp * mx
        rec = [f"id = {sid}", f"name = {q(name)}", f"max = {mx}", f"inf = {inf}", f"range = {rng}", f"hit = {hit}",
               f"element = {ele}", f"nk = {nk}", f"sp = [{', '.join(map(str, sp[:max(mx, 1)]))}]"]
        if sid in quest:
            rec.append("quest = true")
        skills.append("[[skill]]\n" + "\n".join(rec))
    tree = []
    for line in csv_records(M631 / "db/skill_tree.txt"):
        body = line.split("/")[0]  # one row has a single-slash comment
        cols = [num(c) for c in body.split(",") if c.strip()]
        if len(cols) < 3:
            continue
        need = [(cols[i], cols[i + 1]) for i in range(3, min(len(cols), 13), 2) if cols[i]]
        needs = ", ".join(f"[{a}, {b}]" for a, b in need)
        tree.append(f"[[tree]]\njob = {cols[0]}\nskill = {cols[1]}\nmax = {cols[2]}\nneed = [{needs}]")
    hdr = ("# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.\n"
           "[meta]\ntier = \"athena2003\"\n"
           f"source = \"{SRC631} db/skill_db.txt, db/skill_tree.txt; quest flag from athena-817 db/skill_db.txt (inf2)\"\n"
           "note = \"inf: 0 passive 1 enemy 2 place 4 self 16 friend; range -1 = weapon range; sp per level "
           "(the server prefers the client's leveluseskillspamount.txt); quest = learnt only from NPCs\"\n\n")
    (ROOT / "data/db/skills.toml").write_text(hdr + "\n\n".join(skills + tree) + "\n", encoding="utf-8")
    return len(skills), len(tree)


# ---------------------------------------------------------------- random items, Dead Branch
BOXES = [(1, "blue_box", "item_bluebox.txt"), (2, "violet_box", "item_violetbox.txt"),
         (3, "card_album", "item_cardalbum.txt"), (4, "gift_box", "item_giftbox.txt"), (5, "scroll", "item_scroll.txt")]


def import_random():
    """Old Blue Box & co (`getitem -N`) and the Dead Branch list, mod0631 db/."""
    out = ["# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.",
           "[meta]", 'tier = "athena2003"',
           f'source = "{SRC631} db/item_bluebox.txt, item_violetbox.txt, item_cardalbum.txt, item_giftbox.txt, '
           'item_scroll.txt, mob_branch.txt"',
           'note = "box: Athena itemdb_searchrandomid picks a random entry and keeps it with chance per/1000000, '
           'up to 2000 tries, else the default (the rate of an id-0 row). branch: monsters a Dead Branch may summon"', ""]
    for flag, name, fn in BOXES:
        default, items = 0, []
        for line in csv_records(M631 / "db" / fn):
            cols = [c.strip() for c in line.split("//")[0].split(",")]
            if len(cols) < 3 or not cols[0].lstrip("-").isdigit() or not cols[-1].isdigit():
                continue
            iid, per = num(cols[0]), num(cols[-1])  # some names contain commas
            if iid == 0:
                default = per
            else:
                items.append(f"[{iid}, {per}]")
        out += ["[[box]]", f"flag = {flag}", f'name = "{name}"', f"default = {default}",
                "items = [" + ", ".join(items) + "]", ""]
    mobs = [str(num(l.split("//")[0])) for l in csv_records(M631 / "db/mob_branch.txt") if l.split("//")[0].strip().isdigit()]
    out += ["[branch]", "mobs = [" + ", ".join(mobs) + "]", ""]
    (ROOT / "data/db/random.toml").write_text("\n".join(out), encoding="utf-8")
    return len(mobs)


# ---------------------------------------------------------------- pets
def import_pets():
    """mod0631 db/pet_db.txt: tameable monsters and their care."""
    out = ["# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.", "[meta]",
           'tier = "athena2003"', f'source = "{SRC631} db/pet_db.txt"',
           'note = "item: taming item; egg/accessory/food: item ids; fullness: hunger points per feeding; '
           'hungry_delay: seconds per hunger point; r_hungry/r_full: intimacy gained/lost feeding hungry/full; '
           'intimate: intimacy when caught (of 1000); die: lost when it dies; capture: base catch chance (1/100 %); '
           'speed: ms per cell"', ""]
    n = 0
    for line in csv_records(M631 / "db/pet_db.txt"):
        c = [x.strip() for x in line.split(",")]
        if len(c) < 15 or not c[0].isdigit():
            continue
        f = [num(x) for x in c[3:15]]
        out += ["[[pet]]", f"mob = {c[0]}", f'name = "{c[1]}"', f"item = {f[0]}", f"egg = {f[1]}", f"accessory = {f[2]}",
                f"food = {f[3]}", f"fullness = {f[4]}", f"hungry_delay = {f[5]}", f"r_hungry = {max(f[6], 1)}",
                f"r_full = {f[7]}", f"intimate = {f[8]}", f"die = {f[9]}", f"capture = {f[10]}", f"speed = {f[11]}", ""]
        n += 1
    (ROOT / "data/db/pets.toml").write_text("\n".join(out), encoding="utf-8")
    return n


# ---------------------------------------------------------------- map flags
def import_mapflags():
    """Map flags from mod0631 conf/mapflag.txt and every NPC file it loads."""
    flags = {}
    for f in [M631 / "conf/mapflag.txt"] + npc_files():
        if not f.exists():
            continue
        for line in read(f).splitlines():
            cols = line.strip().split("\t")
            if len(cols) < 3 or cols[1].strip() != "mapflag" or line.lstrip().startswith("//"):
                continue
            mp = strip_gat(cols[0].strip())
            flag = cols[2].strip()
            e = flags.setdefault(mp, {"flags": set(), "nosave": None})
            e["flags"].add(flag)
            if flag == "nosave" and len(cols) > 3:
                v = cols[3].strip()
                if v.lower().startswith("savepoint"):
                    e["nosave"] = "SavePoint"
                else:
                    m = re.match(r"([\w\-]+?)(?:\.gat)?,(\d+),(\d+)", v)
                    if m:
                        e["nosave"] = f"{strip_gat(m.group(1))},{m.group(2)},{m.group(3)}"
    out = ["# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.", "[meta]",
           'tier = "athena2003"', f'source = "{SRC631} conf/mapflag.txt and the mapflag lines of its NPC files"',
           'note = "noteleport: no Fly Wing/Teleport; nobranch: no Dead Branch; nopenalty: no EXP loss on death; '
           'nosave: logging out here puts you at this point next time (SavePoint = your save point); '
           'nomemo: no Warp Portal memo; pvp"', ""]
    for mp in sorted(flags):
        e = flags[mp]
        out += ["[[map]]", f'name = "{mp}"', "flags = [" + ", ".join(f'"{x}"' for x in sorted(e["flags"])) + "]"]
        if e["nosave"]:
            out.append(f'nosave = "{e["nosave"]}"')
        out.append("")
    (ROOT / "data/db/mapflags.toml").write_text("\n".join(out), encoding="utf-8")
    return len(flags)


# ---------------------------------------------------------------- npc files
NPC_FILES = None


def npc_files():
    files = []
    for line in read(M631 / "conf/map_athena.conf").splitlines():
        m = re.match(r"^npc:\s*(\S+)", line)
        if m:
            files.append(M631 / m.group(1))
    return files


def _load_maps():
    import importlib.util
    spec = importlib.util.spec_from_file_location("ro_maps", Path(__file__).with_name("maps.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.MAP_ALIAS


MAP_ALIAS = _load_maps()  # Athena map name -> this client's (tools/import/maps.py)


def strip_gat(m):
    m = m[:-4] if m.lower().endswith(".gat") else m
    return MAP_ALIAS.get(m, m)


def shop_names_817():
    """English shop names from eAthena 817 (mod0631's are Japanese), by map and cell."""
    out = []
    for f in sorted((E817 / "npc").rglob("*.txt")):
        for line in f.read_bytes().decode("cp1252", "replace").replace("\r", "").split("\n"):
            m = re.match(r"([\w\-]+)\.gat,(\d+),(\d+),\d+\tshop\t([^\t]+)\t(\d+)", line)
            if m and m.group(4).isascii():  # a few are still Japanese: those stay "Shop"
                out.append((m.group(1), int(m.group(2)), int(m.group(3)), m.group(4).strip(), int(m.group(5))))
    return out


def shop_name(names, mp, x, y, sprite):
    """Same cell, or the nearest same-sprite shop within 3 cells; else "Shop"."""
    best = None
    for m, a, b, n, s in names:
        if m != mp:
            continue
        d = max(abs(a - x), abs(b - y))
        if d == 0 or (d <= 3 and s == sprite):
            if best is None or d < best[0]:
                best = (d, n)
    return best[1] if best else "Shop"


def import_npcs():
    warps, spawns, shops = [], [], []
    names817 = shop_names_817()
    for f in npc_files():
        if not f.exists():
            continue
        for ln, line in enumerate(read(f).splitlines(), 1):
            if line.lstrip().startswith("//"):
                continue
            cols = line.split("\t")
            if len(cols) < 4:
                continue
            kind = cols[1].strip()
            where = f"{f.name}:{ln}"
            if kind == "warp":
                m = re.match(r"([\w\-]+(?:\.gat)?),(\d+),(\d+)", cols[0])
                d = re.match(r"(\d+),(\d+),([\w\-]+(?:\.gat)?),(\d+),(\d+)", cols[3].strip())
                if m and d:
                    warps.append(f'warp {{ map = "{strip_gat(m.group(1))}", x = {m.group(2)}, y = {m.group(3)}, '
                                 f'span = {{ {d.group(1)}, {d.group(2)} }}, to = {{ "{strip_gat(d.group(3))}", {d.group(4)}, {d.group(5)} }} }} -- {where}')
            elif kind == "monster":
                m = re.match(r"([\w\-]+(?:\.gat)?),(\d+),(\d+),(\d+),(\d+)", cols[0])
                d = [x.strip() for x in cols[3].split(",")]
                if m and len(d) >= 2 and d[0].isdigit():
                    vals = [num(x) for x in d] + [0] * 6
                    mobid, count, d1, d2 = vals[0], vals[1], vals[2], vals[3]
                    spawns.append(f'spawn {{ map = "{strip_gat(m.group(1))}", x = {m.group(2)}, y = {m.group(3)}, '
                                  f'xs = {m.group(4)}, ys = {m.group(5)}, mob = {mobid}, count = {count}, '
                                  f'delay = {d1}, delay2 = {d2} }} -- {where}')
            elif kind == "shop":
                m = re.match(r"([\w\-]+(?:\.gat)?),(\d+),(\d+),(\d+)", cols[0])
                d = [x.strip() for x in cols[3].split(",")]
                if m and d and d[0].lstrip("-").isdigit():
                    items = []
                    for it in d[1:]:
                        if ":" in it:
                            a, b = it.split(":")[:2]
                            items.append(f"{{ {num(a)}, {num(b)} }}" if num(b) > 0 else f"{{ {num(a)} }}")
                        elif it.isdigit():
                            items.append(f"{{ {it} }}")
                    mp = strip_gat(m.group(1))
                    name = shop_name(names817, mp, int(m.group(2)), int(m.group(3)), num(d[0]))
                    shops.append(f'shop {{ map = "{mp}", x = {m.group(2)}, y = {m.group(3)}, dir = {m.group(4)}, '
                                 f'sprite = {num(d[0])}, name = {q(name)}, items = {{ {", ".join(items)} }} }} -- {where}')
    hdr = ("-- Generated by tools/import/athena_import.py from athena-dev-2.1.1-mod0631 (2003-12-18)\n"
           "-- conf/map_athena.conf npc list. GPL data. Map names follow this client (tools/import/maps.py);\n"
           "-- entries on maps the client lacks are skipped by the server at load time.\n"
           "-- Price {id} = the item's db price; {id, n} = n zeny. Shop names: eAthena 817 by cell\n"
           "-- (mod0631's are Japanese); \"Shop\" where 817 has none within 3 cells.\n")
    out = ROOT / "scripts/import"
    out.mkdir(parents=True, exist_ok=True)
    (out / "warps.lua").write_text(hdr + "\n".join(warps) + "\n", encoding="utf-8")
    (out / "spawns.lua").write_text(hdr + "-- delay/delay2: respawn ms (0 = Athena default 5000)\n" + "\n".join(spawns) + "\n", encoding="utf-8")
    (out / "shops.lua").write_text(hdr + "\n".join(shops) + "\n", encoding="utf-8")
    return len(warps), len(spawns), len(shops)


if __name__ == "__main__":
    (ROOT / "data/db").mkdir(parents=True, exist_ok=True)
    print("items", import_items())
    print("mobs", import_mobs())
    import_exp()
    import_jobs()
    print("skills/tree", import_skills())
    print("branch monsters", import_random())
    print("maps with flags", import_mapflags())
    print("pets", import_pets())
    print("warps/spawns/shops", import_npcs())

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
from collections import Counter
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
            fn = "item_heal" if m.group(1) == "itemheal" else "percent_heal"
            out.append(f"{fn}({a}, {b})")
            continue
        m = re.fullmatch(r"sc_end\s+(SC_\w+)", stmt)
        if m:
            # mod0631's item_db spells Poison "SC_Posion" on Panacea and Royal Jelly: no such
            # constant, so its script ended nothing. The item text says they cure poison.
            name = "SC_Poison" if m.group(1) == "SC_Posion" else m.group(1)
            if sc_const(name) is not None:
                out.append(f"sc_end({sc_const(name)}) --[[{name}]]")
                continue
        m = re.fullmatch(r"set\s+Zeny\s*,\s*Zeny\s*\+\s*(rand\(\s*\d+\s*,\s*\d+\s*\))", stmt)  # Gold Coin pouches
        if m:
            out.append(f"player.zeny = player.zeny + {lua_expr(m.group(1))}")
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


# Card slots the client draws for an item (data\itemslotcounttable.txt of the
# played client, tier client) where mod0631's table has another count. The
# client's table was compared whole on 2026-11-14: only these two weapons
# differ among the items that can be had (mod0631's eight elemental armours
# and other placeholders, which nothing gives, differ too and are left).
CLIENT_SLOTS = {1115: 3, 1524: 1}  # Scimiter (817: 0 as well), Golden Mace


# Where this client's own item text (data\idnum2itemdesctable.txt, tier client) and
# RODataZone's iRO item tables of February 2004 (data:refs/web/rodatazone-2004/*_items_*.js)
# agree on a number that mod0631's (jRO) table has otherwise, theirs is taken
# (research/rdz_items_check.py lists every difference; THREADS.md R6). Weights in tenths.
CLIENT_IRO = {
    501: {'weight': 100},  # Red Potion
    502: {'weight': 130},  # Orange Potion
    503: {'weight': 160},  # Yellow Potion
    504: {'weight': 200},  # White Potion
    505: {'weight': 200},  # Blue Potion
    506: {'weight': 60},  # Green Potion
    507: {'weight': 40},  # Red Herb
    508: {'weight': 70},  # Yellow Herb
    509: {'weight': 90},  # White Herb
    510: {'weight': 90},  # Blue Herb
    511: {'weight': 20},  # Green Herb
    512: {'weight': 30},  # Apple
    513: {'weight': 30},  # Banana
    514: {'weight': 30},  # Grape
    515: {'weight': 30},  # Carrot
    516: {'weight': 30},  # Sweet Potato
    517: {'weight': 200},  # Meat
    518: {'weight': 150},  # Honey
    519: {'weight': 50},  # Milk
    520: {'weight': 50},  # Hinalle Leaflet
    521: {'weight': 50},  # Aloe Leaflet
    522: {'weight': 50},  # Mastela Fruit
    523: {'weight': 100},  # Holy Water
    525: {'weight': 200},  # Panacea
    526: {'weight': 200},  # Royal Jelly
    528: {'weight': 200},  # Monster's Feed
    531: {'weight': 50},  # Apple Juice
    532: {'weight': 50},  # Banana Juice
    533: {'weight': 50},  # Grape Juice
    534: {'weight': 50},  # Carrot Juice
    535: {'weight': 30},  # Pumpkin
    536: {'weight': 100},  # Ice Cream
    1009: {'weight': 100},  # Hand of God
    1093: {'weight': 50},  # Empty Potion Bottle
    1168: {'atk': 190},  # Zweihander
    1237: {'atk': 150},  # Grimtooth
    1304: {'weapon_level': 4},  # Orcish Axe
    1363: {'atk': 160},  # Bloody Axe
    1364: {'atk': 185},  # Great Axe
    1466: {'atk': 150},  # Crescent Scythe
    # Partizan without slots: mod0631's other two Partizans say 2/18 too, and so does Ragu Memo's
    # jRO spear table (weapon4.html, Wayback 2003-12-18)
    1459: {'weapon_level': 2, 'equip_level': 18},
    1468: {'weight': 1200},  # Zephyrus
    1524: {'atk': 130},  # Golden Mace
    1550: {'atk': 100},  # Book
    1551: {'atk': 139},  # Bible
    1718: {'weight': 1100},  # Hunter Bow
    2607: {'weight': 50},  # Clip
    2616: {'weight': 50},  # Critical Ring
    5008: {'def': 0},  # Puppy Love
    5012: {'weight': 100},  # Ph.D Hat
}


# Who may equip an item, where this client's text ("Applicable Job" / "Available Job" in
# data\idnum2itemdesctable.txt, tier client) and RODataZone's iRO item tables of February
# 2004 ("Used By") agree against mod0631's (jRO) job mask, which 817 shares on every one.
# Printed by `research/item_jobs_check.py desc.txt --rdz DIR` (2026-11-14): a first job
# changes where both say so; a first job dropped takes the second jobs RODataZone does not
# list; a 2-2 job follows its 2-1 sibling unless the text names it. So: Novices may use the
# better one-handed swords, daggers, flails, staves and the hats for "Every Job"; Thieves no
# axes or maces; knuckles are the Monk's, the Lance the Knight's and Crusader's, the Stunner
# the Priest's (THREADS.md #31). Where only one of the two speaks, mod0631's mask stays.
# Spawn counts of late 2003 (research/THREADS.md #10). Ragu Memo's jRO dungeon pages of 2003-07
# have mod0631's and 817's counts on these floors; its captures of 2003-12-05 (and 2003-12 for
# Glast Heim) have these, and rAthena's pre-renewal spawns (tier later) keep the December figure
# wherever they did not change the monster again (Poporing 15, Whisper 20, Rideword 15 and 8,
# Sage Worm 14; Skeletons on Payon Cave 1F). Total per map and monster; a new pair is added as
# one group anywhere on the map. data:refs/web/ragmemo-2003/ragmemo_map_*.html.
# Not taken: the 25 Skeletons jRO put on Payon Cave 1F (pay_dun00, 1076). Phelan's Battle Priest
# guide (GameFAQs 27955, by 2004-01-13, an English server: iRO or pRO) lists that floor as
# "Familiars, Zombies ... spores", and a Battle Priest's guide would name 25 undead (round 24).
SPAWNS_2003_12 = {
    ("gef_dun00", 1005): 20, ("gef_dun00", 1031): 15,
    ("gef_dun01", 1100): 25, ("gef_dun01", 1114): 20,
    ("gef_dun02", 1100): 20, ("gef_dun02", 1179): 20,
    ("gl_cas01", 1131): 5, ("gl_cas01", 1195): 15, ("gl_cas01", 1267): 30, ("gl_cas01", 1281): 40,
    ("gl_in01", 1143): 8, ("gl_in01", 1195): 8, ("gl_in01", 1281): 14,
}


def spawns_2003_12(spawns):
    """Set SPAWNS_2003_12's totals: the first anywhere-on-the-map group of each pair takes the
    difference (fixed-cell groups keep theirs); a pair with no group gets one."""
    pat = re.compile(r'map = "(\w+)", x = (\d+), y = (\d+), .*mob = (\d+), count = (\d+)')
    total = Counter()
    for line in spawns:
        m = pat.search(line)
        if m:
            total[(m.group(1), int(m.group(4)))] += int(m.group(5))
    out, done = [], set()
    for line in spawns:
        m = pat.search(line)
        key = m and (m.group(1), int(m.group(4)))
        if key in SPAWNS_2003_12 and key not in done and m.group(2) == "0" and m.group(3) == "0":
            n = int(m.group(5)) + SPAWNS_2003_12[key] - total[key]
            done.add(key)
            if n <= 0:
                continue
            line = line.replace(f"count = {m.group(5)},", f"count = {n},", 1) + " (count: Ragu Memo 2003-12)"
        out.append(line)
    for (mp, mob), n in SPAWNS_2003_12.items():
        if (mp, mob) not in done:
            out.append(f'spawn {{ map = "{mp}", x = 0, y = 0, xs = 0, ys = 0, mob = {mob}, count = {n}, '
                       f'delay = 0, delay2 = 0 }} -- Ragu Memo 2003-12 (not in mod0631)')
    return out


CLIENT_IRO_JOBS = {
    1119: 0x654e3,  # Tsurugi: + Novice
    1120: 0x654e3,  # Tsurugi: + Novice
    1121: 0x654e3,  # Tsurugi: + Novice
    1122: 0x654e3,  # Ring Pommel Saber: + Novice
    1123: 0x654e3,  # Haedonggum: + Novice
    1125: 0x654e3,  # Ring Pommel Saber: + Novice
    1127: 0x654e3,  # Saber: + Novice
    1128: 0x654e3,  # Haedonggum: + Novice
    1131: 0x654e3,  # Ice Falchon: + Novice
    1133: 0x654e3,  # Fire Brand: + Novice
    1135: 0x654e3,  # Cutlas: + Novice
    1143: 0x654e3,  # Gaia Sword: + Novice
    1144: 0x654e3,  # Sashimi: + Novice
    1219: 0x1f5eef,  # Gladius: + Novice
    1220: 0x1f5eef,  # Gladius: + Novice
    1221: 0x1f5eef,  # Gladius: + Novice
    1222: 0x1f5eef,  # Damascus: + Novice
    1223: 0x1f5eef,  # Fortune Sword: + Novice
    1226: 0x1f5eef,  # Damascus: + Novice
    1234: 0x1f5eef,  # Walgwanggum: + Novice
    1236: 0x1f5eef,  # Sucsamad: + Novice
    1237: 0x1f5eef,  # Grimtooth: + Novice
    1239: 0x1f5eef,  # Poison Knife: + Novice
    1301: 0x654a3,  # Axe; - Thief
    1302: 0x654a3,  # Axe; - Thief
    1303: 0x654a3,  # Axe; - Thief
    1304: 0x654a3,  # Orcish Axe; - Thief
    1410: 0x4080,  # Lance; - Swordman
    1411: 0x4080,  # Lance; - Swordman
    1412: 0x4080,  # Lance; - Swordman
    1501: 0x4c5b3,  # Club; - Thief Assassin Rogue
    1502: 0x4c5b3,  # Club; - Thief Assassin Rogue
    1503: 0x4c5b3,  # Club; - Thief Assassin Rogue
    1504: 0x4c5b3,  # Mace; - Thief Assassin Rogue
    1505: 0x4c5b3,  # Mace; - Thief Assassin Rogue
    1506: 0x4c5b3,  # Mace; - Thief Assassin Rogue
    1507: 0x4c5b3,  # Smasher; - Thief Assassin Rogue
    1508: 0x4c5b3,  # Smasher; - Thief Assassin Rogue
    1509: 0x4c5b3,  # Smasher; - Thief Assassin Rogue
    1510: 0x4c5b3,  # Flail: + Novice; - Thief Assassin Rogue
    1511: 0x4c5b3,  # Flail: + Novice; - Thief Assassin Rogue
    1512: 0x4c5b3,  # Flail: + Novice; - Thief Assassin Rogue
    1513: 0x4c5b3,  # Morning Star: + Novice; - Thief Assassin Rogue
    1514: 0x4c5b3,  # Morning Star: + Novice; - Thief Assassin Rogue
    1515: 0x4c5b3,  # Morning Star: + Novice; - Thief Assassin Rogue
    1516: 0x4c5b3,  # Sword Mace: + Novice; - Thief Assassin Rogue
    1517: 0x4c5b3,  # Sword Mace: + Novice; - Thief Assassin Rogue
    1518: 0x4c5b3,  # Sword Mace: + Novice; - Thief Assassin Rogue
    1519: 0x4c5b3,  # Chain: + Novice; - Thief Assassin Rogue
    1520: 0x4c5b3,  # Chain: + Novice; - Thief Assassin Rogue
    1522: 0x8100,  # Stunner; - Acolyte
    1524: 0x4c5b3,  # Golden Mace: + Novice Swordman Merchant Knight Blacksmith Crusader Alchemist
    1525: 0x4c5b3,  # Long Mace: + Novice Swordman Merchant Knight Blacksmith Crusader Alchemist
    1607: 0x18315,  # Staff: + Novice
    1608: 0x18315,  # Staff: + Novice
    1609: 0x18315,  # Staff: + Novice
    1610: 0x18315,  # Arc Wand: + Novice
    1612: 0x18315,  # Arc Wand: + Novice
    1614: 0x18315,  # Wand of Occult: + Novice
    1615: 0x18315,  # Bone Wand: + Novice
    1801: 0x8000,  # Waghnakh; - Priest
    1802: 0x8000,  # Waghnakh; - Priest
    1809: 0x8000,  # Claw; - Priest
    1810: 0x8000,  # Claw; - Priest
    1811: 0x8000,  # Finger; - Priest
    1812: 0x8000,  # Finger; - Priest
    2222: 0x1fdfff,  # Turban: + Novice
    2223: 0x1fdfff,  # Turban: + Novice
    2224: 0x654e2,  # Goggle; - Archer Hunter Bard Dancer
    2225: 0x654e2,  # Goggle; - Archer Hunter Bard Dancer
    2244: 0x1fdfff,  # Big Ribbon: + Novice
    2245: 0x1fdfff,  # Sweet Gent: + Novice
    2247: 0x1fdfff,  # Romantic Gent: + Novice
    2248: 0x1fdfff,  # Western Grace: + Novice
    2250: 0x1fdfff,  # Cute Ribbon: + Novice
    2254: 0x1fdfff,  # Angel Wing: + Novice
    2255: 0x1fdfff,  # Evil Wing: + Novice
    2258: 0x1fdffe,  # Spiky Band: + Mage Archer Wizard Hunter Sage Bard Dancer
    2260: 0x1fdfff,  # Mini Glasses: + Novice
    2263: 0x1fdfff,  # Zoro Masque: + Novice
    2264: 0x1fdffe,  # Munak Hat; - Novice
    2269: 0x1fdfff,  # Romantic Flower: + Novice
    2270: 0x1fdfff,  # Romantic Leaf: + Novice
    2271: 0x1fdfff,  # Jack a Dandy: + Novice
    2273: 0x1fdfff,  # Doctor Band: + Novice
    2274: 0x1fdfff,  # Ghost Bandana: + Novice
    2281: 0x1fdfff,  # Opera Masque: + Novice
    2284: 0x1fdfff,  # Antler: + Novice
    2285: 0x1fdfff,  # Apple o' Archer: + Novice
    2286: 0x1fdfff,  # Elven Ears: + Novice
    2287: 0x1fdfff,  # Pirate Bandana: + Novice
    2288: 0x1fdfff,  # Mr. Scream: + Novice
    2291: 0x1fdfff,  # Masquerade: + Novice
    2295: 0x1fdfff,  # Blinker: + Novice
    2297: 0x1fdfff,  # Goblini Mask: + Novice
    2307: 0x1e5cea,  # Mantle; - Mage Acolyte Priest Wizard Monk Sage
    2308: 0x1e5cea,  # Mantle; - Mage Acolyte Priest Wizard Monk Sage
    2309: 0x1e5cea,  # Coat; - Mage Acolyte Priest Wizard Monk Sage
    2310: 0x1e5cea,  # Coat; - Mage Acolyte Priest Wizard Monk Sage
    2321: 0x1fdffe,  # Silk Robe: + Archer Thief Hunter Assassin Rogue Bard Dancer
    2322: 0x1fdffe,  # Silk Robe: + Archer Thief Hunter Assassin Rogue Bard Dancer
    2325: 0x4c5b2,  # Saint's Robe: + Swordman Knight Crusader
    2326: 0x4c5b2,  # Saint's Robe: + Swordman Knight Crusader
    2409: 0x1e5cea,  # Highheals; - Mage Acolyte Priest Wizard Monk Sage
    2607: 0x1fdffe,  # Clip; - Novice
    2627: 0x1fdffe,  # Ancient Belt; - Novice
    5001: 0x1fdfff,  # Headset: + Novice
    5003: 0x1fdfff,  # Joker Jester: + Novice
    5004: 0x1fdfff,  # Oxygen Mask: + Novice
    5005: 0x1fdfff,  # Gas Mask: + Novice
    5007: 0x1fdfff,  # Grand Circlet: + Novice
    5008: 0x1fdfff,  # Puppy Love: + Novice
    5009: 0x1fdfff,  # Safety Helmet: + Novice
    5012: 0x1fdfff,  # Ph.D Hat: + Novice
    5016: 0x444a2,  # Boys Cap; - Mage Archer Acolyte Thief Priest Wizard Hunter Assassin Monk Sage Rogue Bard Dancer
    5019: 0x1fdfff,  # Corsair: + Novice
    5024: 0x1fdffe,  # Cake Hat; - Novice
    5029: 0x1fdfff,  # Spore Hat: + Novice
    5033: 0x1fdfff,  # Smokie Hat: + Novice
    5034: 0x1fdfff,  # Lightbulb Hairband: + Novice
    5045: 0x1fdfff,  # Magic Hat: + Novice Swordman Archer Merchant Thief Knight Blacksmith Hunter Assassin Crusader Rogue Alchemist Bard Dancer
    5047: 0x1fdfff,  # Fashion Sunglasses: + Novice
}

# The Safety Ring's MDEF +3: 817's script (db/item_db.txt), the client's text and
# RODataZone's iRO table all give it ("DEF + 5 , MDEF + 3"); mod0631 has DEF 5 and nothing else.
# The Baphomet Card's splash: the client's own text ("Do splashed damages around 9 cells every
# time"), 817 and jAthena //0797 (2004-02-13) `bonus bSplashRange,1`; mod0631 has only the HIT -10.
EXTRA_EQUIP = {2615: "bonus bMdef,3;", 4147: "bonus bHit,-10; bonus bSplashRange,1;"}
# Cards mod0631 left empty, from the client's own text (data\idnum2itemdesctable.txt), with the
# bonus names and the jAthena commit that first scripted each (git log -G on db/item_db.txt):
#   Muka "HP recovery + 10%", Eggyra "SP Recovery + 15%", Sidewinder "[Double Attack] Lv1
#   with all Weapons" (5%), Moonlight Flower "Fast-Move all the time.DEX -2": //0679 (2004-01-05),
#   the speed as //0695's bSpeedRate (2004-01-11); 817 the same.
#   Zombie "HP recovery + 15%": the client's number; jAthena //0679 and 817 have 20.
#   Hunter Fly "Regain 15% of the Damage Done on an enemy as HP(Success Chance 3%)": //0842
#   (2004-02-24); Dracula "Regain 5% ... as SP (Success Chance 3%)": //0843 (2004-02-26).
#   Orc Lord "reflect 25% of every single Physical Damage": the client's number; //0843 has
#   bShortWeaponDamageReturn,30 (close blows only, which we keep).
#   Maya "reflect Magical Damages back( Success Chance 30%)": the client's text alone (ours:
#   bMagicDamageReturn, rAthena's later name). jAthena has nothing to 2005; 817 and eAthena 0.5.2
#   wrote bNoMagicDamage,30 (spells nullified, not turned back), which the text does not say.
# The Archer Skeleton and Doppelganger Cards stay empty: the client says "(Not implemented yet)".
EXTRA_EQUIP.update({
    4036: "bonus bHPrecovRate,10;",
    4038: "bonus bHPrecovRate,15;",
    4070: "bonus bSPrecovRate,15;",
    4115: "bonus2 bHpDrainRate,3,15;",
    4117: "bonus bDoubleRate,5;",
    4131: "bonus bSpeedRate,25; bonus bDex,-2;",
    4134: "bonus2 bSpDrainRate,3,5;",
    4135: "bonus bShortWeaponDamageReturn,25;",
    4146: "bonus bMagicDamageReturn,30;",
})
# Effects the client's text and RODataZone's iRO tables (items/*.js, captures of 2004-02-11 and
# 2004-02-24) both call not implemented, which mod0631 (jRO) scripts: taken out
# (research/nyi_items.py lists every such item). The status arrows' chances ("[Not Implemented
# yet]" / "Chance of causing Stun effect (NYI)"; the Poison and Freeze Arrows keep their
# element), the Sharp Arrow's critical, the Great Axe's stun ("Stun + 10% (Not Implemented
# yet)" / "Stun +10% (NYI)") and the Requiem Card's chaos ("Success Chance 5% Not implemented
# yet" / "(Not Yet Implemented)"). The Crescent Scythe's critical is 20 in both, 30 in mod0631
# (and its HIT + 10 is jAthena //0556's, below).
EXTRA_EQUIP.update({
    1758: "", 1760: "", 1761: "", 1764: "", 1768: "", 1769: "",
    1759: "bonus bAtkEle,1;",
    1763: "bonus bAtkEle,5;",
    1364: "",
    4104: "",
    1466: "bonus bCritical,20;",
})
# Stats where the client's text and RODataZone's iRO tables give the same, against mod0631's
# script (the rule of THREADS.md R6; found by comparing every stat RODataZone names):
# Muramasa "Critical + 20%" (mod0631 30), the Kobold Card "Critical + 5%" (4), the Poring Card
# "LUK + 1" and nothing more (LUK 2, Perfect Dodge 1), the Condor Card "Dodge + 5" / "Flee +5"
# (10), the Ninja Suit "AGI + 1,MDEF + 3" (no MDEF; 817 has it too), the Nurse Cap "INT+1"
# (LUK +1).
EXTRA_EQUIP.update({
    1164: "bonus bCritical,20;",
    4091: "bonus bStr,1; bonus bCritical,5;",
    4001: "bonus bLuk,1;",
    4015: "bonus bFlee,5;",
    2337: "bonus bAgi,1; bonus bMdef,3;",
    2277: "bonus bInt,1;",
})
# jAthena //0556 (8ec48b3, 2003-11-21, no message) rewrote 56 rare weapons: new bonuses on 48,
# ATK on 23, weight on 5 (Light Epsilon's Heal 1 -> 3, Muramasa 170 -> 155, the Assassin
# Dagger's SP +30 -> HP +20% SP +15%, ...). mod0631 carries most of it. No period witness has
# it: the client's text gives the earlier ATK on all 23 and the earlier weight on all 5;
# RODataZone's iRO weapons.js (2004-02-11) the earlier ATK on all 8 it lists; Ragu Memo's jRO
# weapon pages (2003-12-18, a month after the commit, mod0631's own server) the earlier row on
# all 21 it lists (ATK, "CRI + 20", "MSP + 30", "コールドボルトLv1", "サンダーストームLv1",
# "ヒールLv1"). So every row the commit changed takes its row from before it (weight, ATK, script),
# read from jAthena's git (weapon_rows_before_0556). Where the period sources name an effect the
# earlier row lacks, EXTRA_EQUIP below gives it: the Katzbalger's DEF + 10 (client, Ragu Memo:
# the one addition of //0556 they show), Gungnir's "Accuracy + 30" with no property (client,
# RODataZone; the earlier row has Wind and HIT 20), the Cutlas's Bash 5 (client, RODataZone,
# Ragu Memo), the Ice Falchion's Cold Bolt 1 (RODataZone, Ragu Memo), the Fire Brand's Fire
# Bolt 1 (RODataZone), the Schweizersabel's Lightning Bolt (the client's text, no level: 1,
# as every other weapon "enchanted with" a bolt). research/notes/round35-review.md.
J0556 = "8ec48b358c02d4c9d7524d1d39775b5c853f9f0e"
EXTRA_EQUIP.update({
    1170: "bonus bVit,5; bonus bDef,10;",
    1413: "bonus bHit,30;",
    1135: "skill 5,5;",
    1131: "bonus bAtkEle,1; skill 14,1;",
    1133: "bonus bAtkEle,3; skill 19,1;",
    1167: "bonus bAtkEle,4; skill 20,1;",
})


def weapon_rows_before_0556():
    """{id: (weight, atk, equip)} from jAthena's item_db before //0556, for the rows it changed."""
    import subprocess

    def rows(rev):
        text = subprocess.run(["git", "-C", str(JATHENA), "show", f"{rev}:db/item_db.txt"],
                              capture_output=True, check=True).stdout.decode("cp932", "replace")
        out = {}
        for line in text.splitlines():
            r = split_item(line.strip().rstrip(","))
            if r and r[0][0].strip().isdigit():
                out[int(r[0][0])] = r
        return out
    before, after = rows(J0556 + "^"), rows(J0556)
    return {iid: (num(before[iid][0][6]), num(before[iid][0][7]), before[iid][2])
            for iid in after if iid in before and before[iid] != after[iid]}


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
    before_0556 = weapon_rows_before_0556()
    for iid in sorted(rows):
        (cols, use, equip), src = rows[iid]
        cols += [""] * (17 - len(cols))
        if iid in before_0556:
            w, a, equip = before_0556[iid]
            cols[6], cols[7] = str(w), str(a)
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
            "slots": CLIENT_SLOTS.get(iid, num(cols[10])),
            "jobs": CLIENT_IRO_JOBS.get(iid, num(cols[11])),
            "gender": num(cols[12], 2),
            "loc": num(cols[13]),
            "weapon_level": num(cols[14]),
            "equip_level": num(cols[15]),
            "view": num(cols[16]),
            "use_script": lua or "",
            "athena_use": use if (use and lua is None) else "",
            "athena_equip": EXTRA_EQUIP.get(iid, equip),
        }
        rec.update(CLIENT_IRO.get(iid, {}))
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
    # mod0631 writes `512,0` (an Apple at rate 0) in drop slots it had no item for; eAthena
    # 817 (2004-01) has a real item in 95 of those very slots (92 taken), and RODataZone's iRO monster
    # table of 2004-02 names about 80 of them for the same monster (research/mob_drop_holes.py,
    # THREADS.md #32). Those slots take 817's item and rate. Among them: the Nut Shell from the
    # Wild Rose, the Bongun Hat, the Poring Hat from Marin.
    e817 = {}
    for line in csv_records(E817 / "db/mob_db.txt"):
        c = [x.strip() for x in line.split(",")]
        if c[0].isdigit() and len(c) > 45:
            e817[int(c[0])] = c
    holes = {}
    for mid, (c, src) in rows.items():
        b = e817.get(mid)
        if src != SRC631 or not b:
            continue
        have = {c[29 + 2 * k].strip() for k in range(8)}
        for k in range(8):
            i = 29 + 2 * k
            # not an item mod0631's row drops already: 817 lists the Spore's Mushroom Spore, the
            # Arclouse's Zargon and the Abysmal Knight's Broadsword twice, and Ragu Memo's jRO
            # page of 2003-12 has that Broadsword once, at mod0631's 1/5000 (round 24)
            if (c[i].strip() == "512" and c[i + 1].strip() == "0" and b[i] != "512" and num(b[i + 1]) > 0
                    and b[i] not in have):
                c[i], c[i + 1] = b[i], b[i + 1]
                holes.setdefault(mid, []).append(int(b[i]))
    # Sasquatch: mod0631 (jRO, 2003-12) drops a Poo Poo Hat (2289, rate 7), which this client
    # names but does not describe; 817 has the Panda Cap (5030, rate 1) in that slot, and so do
    # RODataZone's iRO monster table of 2004-02 ("Panda Hat") and Ragu Memo's jRO hat page of
    # 2003-08 (Sasquatch 1/5000; research round 16, T9-090..092). This client describes the
    # Panda Cap. 817's item and rate are taken (THREADS.md #31).
    for mid, (old_item, new_item, rate) in {1243: (2289, 5030, 1)}.items():
        c = rows[mid][0]
        i = next(29 + 2 * k for k in range(8) if c[29 + 2 * k].strip() == str(old_item))
        c[i], c[i + 1] = str(new_item), str(rate)
    # Ragu Memo's item pages (jRO, captures of 2003-12; research/ragmemo_drops.py, THREADS.md
    # #32, round 24) agree with mod0631 on almost every drop. But on eight monsters they side with
    # 817's whole row, item after item, where mod0631 has another set (Osiris's Assassin Dagger,
    # Jamadhar and Sphinx Hat; Doppelganger's five swords and helms at 817's rates...), and
    # RODataZone's iRO monster table of 2004-02 names 817's items for the seven bosses too
    # (data:refs/web/rodatazone-2004/*_monsters_monsters.js). Those take 817's drops and MVP prizes. Then single rates where the page differs from what we
    # hold: 817's hole slots, slots where 817 and the page agree against mod0631, and three of
    # the eight where the page also differs from 817.
    swapped = (1038, 1039, 1046, 1087, 1150, 1159, 1249, 1272)
    # 817's own rows for Osiris, Baphomet and Doppelganger leave out MEXP and ExpPer: their
    # three MVP prizes start at column 47, where 817's engine reads MEXP, and the last pair is
    # empty. Read there, the first prize was lost (Osiris's Old Blue Box, Baphomet's third,
    # Doppelganger's Cursed Ruby; review of round 25). RODataZone's iRO table (2004-02) and a
    # cRO MVP guide (GameFAQs 28266, 2004-03) list those prizes too. mod0631's MEXP is kept.
    shifted = (1038, 1039, 1046)
    for mid in swapped:
        c, b = rows[mid][0], e817[mid]
        c[29:45] = b[29:45]
        if mid in shifted:
            assert b[45:47] == ["0", "0"] and all(x.strip() in ("", "0") for x in b[53:55]), mid
            c[49:55] = b[47:53]
        else:
            c[49:55] = b[49:55]
    # Baphomet's: 817 has the Yggdrasil Seed (608); RODataZone and the cRO guide both name the
    # Yggdrasil Berry (607). The Berry at 817's rate.
    c = rows[1039][0]
    c[49] = "607" if c[49].strip() == "608" else c[49]
    page_rates = {
        (1039, 2607): 625,   # Clip, Baphomet: 1/16 (817 800)
        (1087, 1124): 1000,  # Orcish Sword, Orc Hero: 1/10 (mod0631 100, 817 10)
        (1150, 638): 667,    # Kodachi of Purity, Moonlight Flower: 1/15 (817 500)
        (1120, 2335): 50,    # Thief Clothes, Ghostring: 1/200 (817 150, a hole slot)
        (1145, 2224): 5,     # Goggles, Martin: 1/1666 (817 15)
        (1192, 2505): 2,     # Manteau, Wraith: 1/3333 (817 10)
        (1200, 2291): 5,     # Butterfly Mask, Zherlthsh: 1/1666 (817 3)
        (1261, 5037): 125,   # Nut Shell, Wild Rose: 1/80 (817 120)
        (1270, 520): 625,    # Hinalle Leaflet, Clock Tower Manager: 1/16 (817 850)
        (1008, 1002): 200,   # Iron Ore, Pupa: 1/50, as 817 (mod0631 300)
        (1011, 1002): 150,   # Iron Ore, Chonchon: 1/66, as 817 (mod0631 250)
        (1044, 720): 10,     # Aquamarine, Obeaune: 1/900, as 817 (mod0631 5)
    }
    # The Thief Bug (Larva, 1051): Insect, Dark 1 in Ragu Memo's jRO element table of 2003-12-13
    # (at_monster.html) and in RODataZone's iRO monster table of 2004-02; every athena of the
    # period has Formless, Neutral 3 (race 0, element 60). rAthena's pre-renewal table (later)
    # has Insect, Neutral 3. The two period tables are taken (research/ragmemo_elements.py).
    rows[1051][0][22], rows[1051][0][23] = "4", "27"
    ragmemo = {}
    for (mid, item), rate in page_rates.items():
        c = rows[mid][0]
        i = next(j for j in list(range(29, 45, 2)) + list(range(49, 55, 2)) if c[j].strip() == str(item))
        c[i + 1] = str(rate)
        ragmemo.setdefault(mid, []).append(item)
    out = [
        "# Generated by tools/import/athena_import.py. Numbers: athena mod0631 (2003-12-18);",
        "# display names: eAthena 0.5.2 (2004-05). GPL-2.0-or-later data.",
        "[meta]",
        'tier = "athena2003"',
        f"source = {q(SRC631 + ' db/mob_db.txt; drops in its empty slots from athena-817 db/mob_db.txt; display names from ' + SRC052)}",
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
        if mid in swapped:
            out.append("# drops and MVP prizes: eAthena 817's row, which Ragu Memo's jRO item pages of 2003-12 support")
        elif mid in holes:
            out.append(f"# drops {', '.join(map(str, holes[mid]))}: eAthena 817, in slots mod0631 left empty")
        if mid in ragmemo:
            out.append(f"# rate of {', '.join(map(str, ragmemo[mid]))}: Ragu Memo's jRO item pages of 2003-12")
        out.append("")
    (ROOT / "data/db/mobs.toml").write_text("\n".join(out), encoding="utf-8")
    return len(rows)


def import_exp():
    cols = [[] for _ in range(6)]
    for line in csv_records(M631 / "db/exp.txt"):
        vals = [num(x) for x in line.split(",")]
        for i in range(min(6, len(vals))):
            cols[i].append(vals[i])
    # the tables come first: after a [meta] header TOML would read them as part of meta
    out = [
        "# Generated by tools/import/athena_import.py from athena mod0631 db/exp.txt (2003-12-18).",
        f"base = {cols[0]}", f"job_novice = {cols[3]}", f"job_first = {cols[4]}", f"job_second = {cols[5]}", "",
        "[meta]", 'tier = "athena2003"', f"source = {q(SRC631 + ' db/exp.txt')}",
        'note = "entry n = EXP needed to go from level n+1 to n+2; mod0631 columns 0-2 are identical base tables"', "",
    ]
    (ROOT / "data/db/exp.toml").write_text("\n".join(out), encoding="utf-8")


JOBS = ["Novice", "Swordsman", "Magician", "Archer", "Acolyte", "Merchant", "Thief", "Knight", "Priest",
        "Wizard", "Blacksmith", "Hunter", "Assassin", "Knight (Peco)", "Crusader", "Monk", "Sage", "Rogue",
        "Alchemist", "Bard", "Dancer", "Crusader (Peco)"]


# HP factors where eAthena 817 (db/job_db1.txt) and RODataZone's sub-stats page (iRO,
# captured 2003-12-11: "Class Modifiers to HP") agree against mod0631: its Archer,
# Acolyte and Thief look shuffled (0.4, 0.504848, 0.4). Every other class agrees.
JOB_HP = {3: 0.5, 4: 0.4, 6: 0.5}  # Archer, Acolyte, Thief


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
                f"hp_coef = {JOB_HP.get(i, float(c[1] or 0))}", f"sp_coef = {num(c[2])}",
                f"aspd_base = [{', '.join(str(num(x)) for x in c[3:20])}]",
                f"job_bonus = {bonus}", ""]
    (ROOT / "data/db/jobs.toml").write_text("\n".join(out), encoding="utf-8")


# ---------------------------------------------------------------- skills
E817 = REFS / "athena-817-src/athena"


# mod0631 gives Fire Wall to the Sky Deleter (1384) and Meteor Storm to the Earth Deleter (1385);
# jAthena //0743 (2004-01-26: "the Deleter's sky and ground skills were the wrong way round") and
# 817 (db/mob_skill_db.txt:1772-1778) swap them. mod0631's row labelled "monster summon @ Dracula"
# (summoning Familiars, 1005) stands under Lord of Death's id (1373); 817 files it under Dracula
# (1389, db/mob_skill_db.txt:1800). mod0631 files the Fake Angel's (1371) last three rows, named
# "@フェイクエンジェル", under the Fur Seal (1317): jAthena by //0742 and 817 have them under 1371.
# (mob, skill) or (mob, skill, val) -> mob.
MOB_SKILL_MOVE = {(1384, 18): 1385, (1385, 83): 1384, (1373, 209, 1005): 1389,
                  (1317, 180): 1371, (1317, 196): 1371}

# jAthena //0743 (6528dcf, 2004-01-26, "to make slave summons and the like resemble the real
# server"; 817 db/mob_skill_db.txt has the same rows): summons cast in 3 s and come back 10 s
# later once every slave is gone, several MVPs gain a second summon, Turtle General and
# Archangeling summon in steps, Gajomart, Lava Golem, Lord of Death and Dracula summon slaves
# (196) instead of free monsters (209), Ghostring calls Giant Whispers. Read from jAthena's git
# (refs/jathena beside refs/eathena-archive): each summon row the commit removes is dropped
# from mod0631's rows where it is found, and the commit's rows take the place of the first.
JATHENA = REFS.parent.parent / "jathena"
J0743 = "6528dcf305bfab7e287af3f702165eadf6b969d6"


def summon_patch_0743():
    import subprocess
    diff = subprocess.run(["git", "-C", str(JATHENA), "show", "-U0", "--format=", J0743, "--",
                           "db/mob_skill_db.txt"], capture_output=True, check=True).stdout
    removed, added = [], {}
    for line in diff.decode("cp932", "replace").splitlines():
        if line[:1] not in "+-" or not line[1:2].isdigit():
            continue
        cols = [c.strip() for c in line[1:].split(",")]
        if cols[3] not in ("196", "209"):
            continue  # the commit's Deleter rows are MOB_SKILL_MOVE
        key = tuple([cols[0]] + cols[2:13])  # the name column is free text
        (removed.append(key) if line[0] == "-" else added.setdefault(cols[0], []).append(cols))
    return removed, added


# mod0631 names NPC_METAMORPHOSIS (193) and gives five monsters a line of it (the Fabre, the
# Pupa and three eggs), but never built the skill, and the lines carry no monster to become.
# 817 builds it (map/skill.c:2421) and its lines (db/mob_skill_db.txt) are mod0631's with the
# monster filled in: Fabre -> Pupa -> Creamy, Peco Peco Egg -> Peco Peco, Thief Bug Egg -> Thief
# Bug Larva, Ant Egg -> Andre. Taken from 817 by (mob, skill), like the drop holes.
METAMORPHOSIS = 193


def metamorphosis_817():
    out = {}
    for line in csv_records(E817 / "db/mob_skill_db.txt"):
        cols = [c.strip() for c in line.split(",")]
        if len(cols) > 12 and cols[0].isdigit() and cols[3] == str(METAMORPHOSIS) and cols[12]:
            out[cols[0]] = cols[12]
    return out


def import_mob_skills():
    """mod0631 db/mob_skill_db.txt: which skill a monster uses, when and how often."""
    rows = []
    meta817 = metamorphosis_817()
    removed, added = summon_patch_0743()
    found = set()
    lines = []
    for line in csv_records(M631 / "db/mob_skill_db.txt"):
        cols = [c.strip() for c in line.split(",")]
        if len(cols) < 12 or not cols[0].isdigit() or not cols[3].isdigit():
            continue
        val = num(cols[12]) if len(cols) > 12 and cols[12] else 0
        key = (int(cols[0]), int(cols[3]))
        cols[0] = str(MOB_SKILL_MOVE.get(key + (val,), MOB_SKILL_MOVE.get(key, cols[0])))
        k = tuple([cols[0]] + (cols + [""] * 13)[2:13])
        if k in removed:
            found.add(k)
            lines.extend((c, "jAthena //0743 db/mob_skill_db.txt") for c in added.pop(cols[0], []))
            continue
        if cols[3] == str(METAMORPHOSIS) and not val and cols[0] in meta817:
            cols = (cols + [""] * 13)[:13]
            cols[12] = meta817[cols[0]]
            lines.append((cols, "eAthena 817 db/mob_skill_db.txt (the monster it becomes)"))
            continue
        lines.append((cols, None))
    for k in removed:
        if k not in found:
            print("  //0743 removes a row mod0631 lacks:", ",".join(k))
    # A monster none of whose removed rows mod0631 has (the Incantation Samurai: mod0631 summons
    # Shinobi with both rows, //0743's parent Tengu): its summon rows become the commit's.
    for mob, cs in added.items():
        old = [i for i, (c, _) in enumerate(lines) if c[0] == mob and c[3] in ("196", "209")]
        print(f"  //0743 rows for {mob} replace mod0631's {len(old)} summon rows")
        at = old[0] if old else len(lines)
        new = [(c, "jAthena //0743 db/mob_skill_db.txt") for c in cs]
        lines = [l for i, l in enumerate(lines[:at]) if i not in old] + new + \
                [l for i, l in enumerate(lines[at:], at) if i not in old]
    for cols, src in lines:
        yes = lambda v: "true" if v in ("yes", "1") else "false"
        rec = [f"mob = {cols[0]}", f"state = {q(cols[2])}", f"skill = {cols[3]}", f"lv = {num(cols[4])}",
               f"rate = {num(cols[5])}", f"cast = {num(cols[6])}", f"delay = {num(cols[7])}",
               f"cancel = {yes(cols[8])}", f"target = {q(cols[9])}", f"cond = {q(cols[10])}", f"cond_val = {num(cols[11])}"]
        if len(cols) > 12 and cols[12] and num(cols[12]):
            rec.append(f"val = {num(cols[12])}")
        if src:
            rec.append(f"source = {q(src)}")
        rows.append("[[skill]]\n" + "\n".join(rec))
    hdr = ("# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.\n"
           "[meta]\ntier = \"athena2003\"\n"
           f"source = \"{SRC631} db/mob_skill_db.txt; slave summons from jAthena //0743 (2004-01-26)\"\n"
           "note = \"state: any idle walk attack dead loot chase; rate per 1000 each time the monster thinks; "
           "cast/delay ms; cancel = damage interrupts the cast; target: target self friend; cond: always "
           "myhpltmaxrate attackpcgt attackpcge slavelt slavele closedattacked longrangeattacked skillused "
           "casttargeted (friend conditions were not implemented in mod0631); val = skill argument "
           "(monster to summon, emotion)\"\n\n")
    (ROOT / "data/db/mob_skills.toml").write_text(hdr + "\n\n".join(rows) + "\n", encoding="utf-8")
    return len(rows)


def import_skills():
    """mod0631 skill_db.txt + skill_tree.txt; quest flag (inf2 & 1) from 817, which added the column."""
    quest = set()
    for line in csv_records(E817 / "db/skill_db.txt"):
        cols = line.split("//")[0].split(",")
        if len(cols) >= 11 and num(cols[10]) & 1:
            quest.add(num(cols[0]))
    cast = {}
    for line in csv_records(M631 / "db/cast_db.txt"):
        cols = [c.strip() for c in line.split("//")[0].split(",")]
        if len(cols) >= 3 and cols[0].isdigit():
            cast[num(cols[0])] = ([num(x) for x in cols[1].split(":")], [num(x) for x in cols[2].split(":")])

    # 817's cast_db adds two duration columns (upkeep_time, upkeep_time2); mod0631 hard-codes durations
    times = {}
    for line in csv_records(E817 / "db/cast_db.txt"):
        cols = [c.strip() for c in line.split("//")[0].split(",")]
        if len(cols) >= 5 and cols[0].isdigit():
            times[num(cols[0])] = ([num(x) for x in cols[3].split(":") if x.strip()], [num(x) for x in cols[4].split(":") if x.strip()])

    def per_level(v, mx):
        v = v * mx if len(v) == 1 else v + [v[-1]] * (mx - len(v))
        return ", ".join(map(str, v[:max(mx, 1)]))

    # Names: the enum of map/skill.h is what the emulator's code calls each skill. skill_db.txt's
    # comments agree for the players' skills and are free text for the monsters' ("npc_poison：...").
    enum_names = {}
    text = (M631 / "map/skill.h").read_bytes().decode("cp932", "replace")
    body = text[text.index("NV_BASIC"):]
    body = body[:body.index("};")]
    nxt = None
    for tok in re.split(r"[,\s]+", re.sub(r"//[^\n]*", "", body)):
        m = re.fullmatch(r"([A-Z][A-Z0-9]*_[A-Z0-9_]+)(?:=(\d+))?", tok)
        if not m:
            m2 = re.fullmatch(r"=?(\d+)", tok)
            if m2 and nxt is not None and pending:
                enum_names.pop(nxt - 1, None)
                nxt = int(m2.group(1))
                enum_names[nxt] = pending
                nxt += 1
            continue
        pending = m.group(1)
        if m.group(2):
            nxt = int(m.group(2))
        elif nxt is None:
            nxt = 1
        enum_names[nxt] = pending
        nxt += 1

    skills = []
    for line in csv_records(M631 / "db/skill_db.txt"):
        body, _, comment = line.partition("//")
        cols = [c.strip() for c in body.split(",")]
        if len(cols) < 9 or not cols[0].isdigit():
            continue
        sid, rng, hit, inf, ele, nk, mx = (num(c) for c in cols[:7])
        name = comment.strip().split("#")[0].strip()
        if not re.fullmatch(r"[A-Z][A-Z0-9]*_[A-Z0-9_]+", name):
            name = enum_names.get(sid, name)
        sp = [num(x) for x in cols[7].split(":")]
        if len(sp) == 1:
            sp = sp * mx
        rec = [f"id = {sid}", f"name = {q(name)}", f"max = {mx}", f"inf = {inf}", f"range = {rng}", f"hit = {hit}",
               f"element = {ele}", f"nk = {nk}", f"sp = [{', '.join(map(str, sp[:max(mx, 1)]))}]"]
        hits = [num(x) for x in cols[8].split(":")]
        if any(h != 1 for h in hits):
            rec.append(f"num = [{per_level(hits, mx)}]")
        if sid in cast:
            c, d = cast[sid]
            if any(c):
                rec.append(f"cast = [{per_level(c, mx)}]")
            if any(d):
                rec.append(f"delay = [{per_level(d, mx)}]")
        if sid in times:
            t1, t2 = times[sid]
            if any(t1):
                rec.append(f"time = [{per_level(t1, mx)}]")
            if any(t2):
                rec.append(f"time2 = [{per_level(t2, mx)}]")
        if len(cols) > 9 and num(cols[9]) & 2:
            rec.append("npc = true")
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
           f"source = \"{SRC631} db/skill_db.txt, db/cast_db.txt, db/skill_tree.txt; quest flag from athena-817 db/skill_db.txt (inf2)\"\n"
           "note = \"inf: 0 passive 1 enemy 2 place 4 self 16 friend; range -1 = weapon range; sp per level "
           "(the server prefers the client's leveluseskillspamount.txt); num = hits per level (when not 1); "
           "cast/delay = ms per level (when not 0); time/time2 = how long the skill and its side effect last, "
           "ms per level, from athena-817 db/cast_db.txt (2004-02-19; mod0631 hard-codes its durations in skill.c); "
           "npc = monster skill; quest = learnt only from NPCs\"\n\n")
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
    # 817's Priest test sets its own flags (jobs/priest.txt:539); mod0631 has no such test
    for f in [M631 / "conf/mapflag.txt"] + npc_files() + [REFS / "athena-817-src/athena/npc/jobs/priest.txt"]:
        if not f.exists():
            continue
        for line in read(f).splitlines():
            cols = line.strip().split("\t")
            if len(cols) < 3 or cols[1].strip() != "mapflag" or line.lstrip().startswith("//"):
                continue
            mp = strip_gat(cols[0].strip())
            if f.name == "priest.txt" and mp != "job_priest":
                continue  # priest_2-1, priest_3-1: the same map under other names (maps.py)
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
    # Ours: no period emulator flags job_wiz (817, jAthena and eAthena 0.5.2 run the Wizard test
    # without any). Its rooms have no doors: someone who logged out in one would come back to a
    # sealed, empty room, and a Fly Wing would carry a candidate past the guards. Flags by analogy
    # with 817's Priest test, sending you back to where the test's own failure does
    # (scripts/npc/wizard_test.lua, docs/DEVIATIONS.md). Not nopenalty: nothing says so.
    flags["job_wiz"] = {"flags": {"nobranch", "nomemo", "nosave", "noteleport"}, "nosave": "geffen,120,110"}
    # Ours likewise: jAthena's Priest test on job_prist (scripts/npc/priest_test.lua) sets no flags;
    # 817's on job_priest did, and the new one takes them, sending you to the church door.
    flags["job_prist"] = {"flags": {"nobranch", "nomemo", "nosave", "noteleport"}, "nosave": "prontera,234,318"}
    # jAthena's Sage test (scripts/npc/sage_test.lua) flags its arena: jAthena //1046
    # script/npc/job/npc_job_16sage.txt:2321-2324 (2004-09-12).
    flags["job_sage"] = {"flags": {"nobranch", "nopenalty", "nosave", "noteleport"}, "nosave": "SavePoint"}
    out = ["# Generated by tools/import/athena_import.py -- edit freely, but note changes in git.", "[meta]",
           'tier = "athena2003"', f'source = "{SRC631} conf/mapflag.txt and the mapflag lines of its NPC files; job_priest from eAthena 817 npc/jobs/priest.txt; job_wiz and job_prist ours, by analogy with it; job_sage from jAthena //1046 npc_job_16sage.txt"',
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
            m = re.match(r"([\w\-]+)\.gat,(\d+),(\d+),\d+ *\tshop\t([^\t]+)\t(\d+)", line)  # two lines have a space before the tab
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


def script_warp(f, ln, cols, where):
    """A `script` warp tile of mod0631's npc_warp files as an npc{} with a touch area."""
    lines = read(f).splitlines()
    body, i = cols[3], ln
    while body.count("{") > body.count("}") and i < len(lines):
        body += "\n" + lines[i]
        i += 1
    m = re.match(r"([\w\-]+(?:\.gat)?),(\d+),(\d+)", cols[0])
    h = re.match(r"45,(\d+),(\d+),", cols[3].strip())
    dests = [(strip_gat(a), int(b), int(c)) for a, b, c in re.findall(r'warp\s+"([\w\-]+(?:\.gat)?)",(\d+),(\d+)', body)]
    if not (m and h and dests):
        return None
    mp, x, y = strip_gat(m.group(1)), m.group(2), m.group(3)
    lst = ", ".join(f'{{ "{a}", {b}, {c} }}' for a, b, c in dests)
    rnd = re.search(r"rand\((\d+)\)", body)
    setv = re.search(r"set\s+anthell,(\d)", body)
    if rnd:
        code = f"local d = {{ {lst} }} d = d[rand({rnd.group(1)}) + 1] or d[1] warp(d[1], d[2], d[3])"
    elif setv:
        code = f'var("anthell", {setv.group(1)}) warp("{dests[0][0]}", {dests[0][1]}, {dests[0][2]})'
    elif "anthell" in body:
        # `if(anthell) goto moc15` / `if(!anthell) goto moc15`: dests are (moc04, moc15)
        test = '(var("anthell") or 0) == 0' if "!anthell" in body else '(var("anthell") or 0) ~= 0'
        code = f"local d = {{ {lst} }} d = {test} and d[2] or d[1] warp(d[1], d[2], d[3])"
    else:
        code = f'warp("{dests[0][0]}", {dests[0][1]}, {dests[0][2]})'
    return (f'npc {{ map = "{mp}", x = {x}, y = {y}, sprite = 45, name = "", id = "{cols[2].strip()}@{mp},{x},{y}", '
            f'touch = {{ {h.group(1)}, {h.group(2)} }}, src = "mod0631:{where}",\n'
            f"  on_touch = function() {code} end }}")


def import_npcs():
    warps, spawns, shops, touch_npcs = [], [], [], []
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
            if kind == "script" and cols[3].strip().startswith("45,") and f.name.startswith("npc_warp"):
                touch_npcs.append(script_warp(f, ln, cols, where))
            elif kind == "warp":
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
                    shops.append([mp, m.group(2), m.group(3), m.group(4), num(d[0]), name, cols[2].strip(), items, where])
    # A shop 817 has no name for takes the English 817 gives most often to other shops
    # that mod0631 calls by the same Japanese name (a role: tool, weapon, armour dealer).
    english = {}
    for sh in shops:
        if sh[5] != "Shop":
            english.setdefault(sh[6], Counter())[sh[5]] += 1
    for sh in shops:
        if sh[5] == "Shop" and sh[6] in english:
            sh[5] = english[sh[6]].most_common(1)[0][0]
    # Two that 817 never translates take a later source's English (research round 9,
    # research/notes/round9-review.md): rAthena's npc/merchants/shops.txt, official
    # names (tier later). Juno's 王窓商会: eAthena 1.0 (2004-11-04) says "Equip Dealer",
    # a role; rAthena's "Mr. King's Shop" translates mod0631's name, so that one.
    later = {("prontera", "48", "58"): "Vegetable Gardener", ("yuno", "205", "103"): "Mr. King's Shop"}
    for sh in shops:
        if sh[5] == "Shop" and (sh[0], sh[1], sh[2]) in later:
            sh[5] = later[(sh[0], sh[1], sh[2])]
    shops = [f'shop {{ map = "{mp}", x = {x}, y = {y}, dir = {dr}, sprite = {spr}, name = {q(name)}, '
             f'items = {{ {", ".join(items)} }} }} -- {where}' for mp, x, y, dr, spr, name, _, items, where in shops]
    hdr = ("-- Generated by tools/import/athena_import.py from athena-dev-2.1.1-mod0631 (2003-12-18)\n"
           "-- conf/map_athena.conf npc list. GPL data. Map names follow this client (tools/import/maps.py);\n"
           "-- entries on maps the client lacks are skipped by the server at load time.\n"
           "-- Price {id} = the item's db price; {id, n} = n zeny. Shop names: eAthena 817 by cell\n"
           "-- (mod0631's are Japanese). Where 817 has none within 3 cells, the English 817 gives\n"
           "-- most often to shops mod0631 names alike (\"Tool Dealer\" for its 道具商人); two from\n"
           "-- rAthena's official names (tier later); else \"Shop\".\n")
    out = ROOT / "scripts/import"
    out.mkdir(parents=True, exist_ok=True)
    (out / "warps.lua").write_text(
        hdr + "\n".join(warps) + "\n\n"
        "-- Warp tiles that are scripts in the source (sprite 45 with a touch area): the\n"
        "-- random exits of Payon Cave 3F and the Clock Tower, and Ant Hell's two doors,\n"
        "-- which remember the field you came in from. A rand(n) with fewer branches than\n"
        "-- n falls through to the first one, as the source's goto chain does.\n"
        + "\n".join(t for t in touch_npcs if t) + "\n", encoding="utf-8")
    spawns = spawns_2003_12(spawns)
    (out / "spawns.lua").write_text(hdr + "-- delay/delay2: respawn ms (0 = Athena default 5000)\n-- Counts marked \"Ragu Memo 2003-12\": jRO's late-2003 rebalance (SPAWNS_2003_12).\n" + "\n".join(spawns) + "\n", encoding="utf-8")
    (out / "shops.lua").write_text(hdr + "\n".join(shops) + "\n", encoding="utf-8")
    return len(warps), len(spawns), len(shops)


# Forging checked against the client's own forge list (data\metalprocessitemlist.txt and
# metalprocessitemtable.txt of the played client, tier client), 2026-11-14:
# * the Pike (1407, 70 Iron, Smith Spear) is in the client's list, 817's and jAthena's
#   tables, and not in mod0631's: added from 817 (db/produce_db.txt);
# * Oridecon and Elunium from three rough ores and a Coal (skill 97, Oridecon Research) are
#   in all three emulators and not in the client's list, and the iRO guide's Blacksmith page
#   (2004-03-27) says Oridecon Research only raises the success of level 3 weapons: left out.
#   The rough ores are purified by NPCs (Dietrich, Christopher...).
PRODUCE_ADD = ["1407,1,104,998,70"]
PRODUCE_DROP = {984, 985}


def import_craft():
    """mod0631 db/create_arrow_db.txt and db/produce_db.txt as Lua tables for the skill rules."""
    arrows, produce = [], []
    for line in csv_records(M631 / "db/create_arrow_db.txt"):
        cols = [num(c) for c in line.split("//")[0].split(",") if c.strip()]
        if len(cols) >= 3 and cols[0] > 0:
            pairs = ", ".join(f"{{ {cols[i]}, {cols[i + 1]} }}" for i in range(1, len(cols) - 1, 2) if cols[i])
            arrows.append(f"  [{cols[0]}] = {{ {pairs} }},")
    for line in list(csv_records(M631 / "db/produce_db.txt")) + PRODUCE_ADD:
        cols = [num(c) for c in line.split("//")[0].split(",") if c.strip()]
        if len(cols) >= 5 and cols[0] > 0 and cols[0] not in PRODUCE_DROP:
            mats = ", ".join(f"{{ {cols[i]}, {cols[i + 1]} }}" for i in range(3, len(cols) - 1, 2) if cols[i])
            produce.append(f"  {{ id = {cols[0]}, lv = {cols[1]}, skill = {cols[2]}, materials = {{ {mats} }} }},")
    hdr = ("-- Generated by tools/import/athena_import.py. Do not edit: regenerate.\n"
           f"-- Source: {SRC631} db/create_arrow_db.txt, db/produce_db.txt (tier athena2003);\n"
           "-- forging checked against the client's own list (PRODUCE_ADD, PRODUCE_DROP in the importer).\n"
           "-- skill_arrow_db[item] = {{arrow, amount}, ...}: what Arrow Crafting makes of one item.\n"
           "-- skill_produce_db: what can be forged or refined: item, item level (1-3 weapon level, 1 for ores),\n"
           "-- the skill it takes, the materials {item, amount}.\n")
    (ROOT / "scripts/import/craft.lua").write_text(
        hdr + "skill_arrow_db = {\n" + "\n".join(arrows) + "\n}\n\nskill_produce_db = {\n" + "\n".join(produce) + "\n}\n",
        encoding="utf-8")
    return len(arrows), len(produce)


if __name__ == "__main__":
    (ROOT / "data/db").mkdir(parents=True, exist_ok=True)
    print("items", import_items())
    print("mobs", import_mobs())
    import_exp()
    import_jobs()
    print("skills/tree", import_skills())
    print("monster skills", import_mob_skills())
    print("arrow recipes / forge recipes", import_craft())
    print("branch monsters", import_random())
    print("maps with flags", import_mapflags())
    print("pets", import_pets())
    print("warps/spawns/shops", import_npcs())

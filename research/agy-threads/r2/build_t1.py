#!/usr/bin/env python3
import os
import re
import sys
import struct
import zlib
import csv

GRF_PATH = "/data/play/RagnarokOnline/data.grf.original"
M631_DIR = "/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/conf"
IMPORT_DIR = "/work/scripts/import/npc"
NPC_OVERRIDE_DIR = "/work/scripts/npc"
OUT_TSV = "/work/research/agy-threads/r2/t1-mod0631-vs-817.tsv"

TARGET_FILES = [
    "npc_event_skillget.txt",
    "npc_job_1st.txt", "npc_job_2nd.txt", "npc_job_aco.txt", "npc_job_merchant.txt", "npc_job_thief.txt", "npc_job_newbie.txt",
    "npc_town_refine.txt", "npc_town_guide.txt",
    "npc_town_alberta.txt", "npc_town_aldebaran.txt", "npc_town_amatsu.txt", "npc_town_comodo.txt",
    "npc_town_geffen.txt", "npc_town_izlude.txt", "npc_town_kafra.txt", "npc_town_lutie.txt",
    "npc_town_morocc.txt", "npc_town_payon.txt", "npc_town_prontera.txt", "npc_town_yuno.txt",
    "npc_event_arrow.txt", "npc_event_bunny.txt", "npc_event_carnival.txt", "npc_event_doll.txt",
    "npc_event_dye.txt", "npc_event_ice.txt", "npc_event_ma_sword.txt", "npc_event_making.txt",
    "npc_event_mobtim.txt", "npc_event_potion.txt", "npc_event_pumpkin.txt", "npc_event_rental.txt",
    "npc_event_smilegirl.txt", "npc_event_yuno.txt",
    "npc_cTower.txt", "npc_turtle.txt", "npc_pvp.txt", "npc_pvproom.txt", "npc_test_arena.txt"
]

# 1. GAT Checker
sys.path.insert(0, "/work/research")
import grf_peek

gat_cache = {}
alias = {}
table_data = None
for n, cs, csa, rs, flags, pos in grf_peek.entries(GRF_PATH):
    if n.lower() == "data\\resnametable.txt":
        with open(GRF_PATH, "rb") as f:
            f.seek(pos)
            table_data = zlib.decompress(f.read(csa))[:rs]
        break
if table_data:
    for line in table_data.decode("latin1", "replace").splitlines():
        p = line.split("#")
        if len(p) >= 2:
            alias[p[0].lower()] = p[1]

def is_walkable(map_name, x, y):
    map_base = map_name.lower().replace(".gat", "")
    gat_name = alias.get(f"{map_base}.gat", f"{map_base}.gat")
    if gat_name not in gat_cache:
        data = None
        for n, cs, csa, rs, flags, pos in grf_peek.entries(GRF_PATH):
            if n.lower() == f"data\\{gat_name}".lower():
                with open(GRF_PATH, "rb") as f:
                    f.seek(pos)
                    data = zlib.decompress(f.read(csa))[:rs]
                break
        gat_cache[gat_name] = data
    data = gat_cache[gat_name]
    if not data or data[:4] != b"GRAT":
        return False
    w, h = struct.unpack_from("<ii", data, 6)
    if not (0 <= x < w and 0 <= y < h):
        return False
    kind = struct.unpack_from("<I", data, 14 + (y * w + x) * 20 + 16)[0]
    return kind not in (1, 5)

# 2. Index our NPCs
our_npcs = []
our_by_pos = {}
our_by_name = {}

def parse_lua_npcs(root_dir):
    for root, dirs, files in os.walk(root_dir):
        for f in files:
            if not f.endswith(".lua"):
                continue
            path = os.path.join(root, f)
            with open(path, "r", encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            # Find each npc { ... }
            # Match start of npc {
            idx = 0
            while True:
                pos = text.find("npc {", idx)
                if pos == -1:
                    pos = text.find("npc{", idx)
                if pos == -1:
                    break
                # Find matching brace
                brace_cnt = 0
                end_pos = -1
                for i in range(pos, len(text)):
                    if text[i] == '{':
                        brace_cnt += 1
                    elif text[i] == '}':
                        brace_cnt -= 1
                        if brace_cnt == 0:
                            end_pos = i + 1
                            break
                if end_pos == -1:
                    break
                block = text[pos:end_pos]
                idx = end_pos

                mp_m = re.search(r'map\s*=\s*"([^"]+)"', block)
                x_m = re.search(r'x\s*=\s*(\d+)', block)
                y_m = re.search(r'y\s*=\s*(\d+)', block)
                src_m = re.search(r'src\s*=\s*"([^"]+)"', block)
                name_m = re.search(r'name\s*=\s*"([^"]+)"', block)
                sprite_m = re.search(r'sprite\s*=\s*(\d+)', block)

                if mp_m and x_m and y_m:
                    mp = mp_m.group(1).lower().replace(".gat", "")
                    x = int(x_m.group(1))
                    y = int(y_m.group(1))
                    src = src_m.group(1) if src_m else f"lua:{os.path.relpath(path, '/work')}"
                    name = name_m.group(1) if name_m else ""
                    sprite = int(sprite_m.group(1)) if sprite_m else 0
                    rec = {
                        "map": mp, "x": x, "y": y, "src": src, "name": name, "sprite": sprite,
                        "file": os.path.relpath(path, "/work"), "body": block
                    }
                    our_npcs.append(rec)
                    our_by_pos.setdefault((mp, x, y), []).append(rec)
                    if name:
                        our_by_name.setdefault(name.lower(), []).append(rec)

parse_lua_npcs(IMPORT_DIR)
parse_lua_npcs(NPC_OVERRIDE_DIR)
print(f"Indexed {len(our_npcs)} of our NPCs.")

# Common Japanese translations
JA_EN_NAMES = {
    "カプラ職員": "Kafra Employee",
    "カプラ": "Kafra",
    "案内要員": "Guide",
    "騎士デトマソ": "Knight De Tomaso",
    "アルコスク": "Alcouskou",
    "転職案内員": "Job Change Guide",
    "修練場案内員": "Training Grounds Guide",
    "修練場教官": "Training Grounds Instructor",
    "修練教官": "Training Instructor",
    "精錬職人": "Refiner",
    "鍛冶屋": "Blacksmith",
    "商人": "Merchant",
    "武器商人": "Weapon Dealer",
    "防具商人": "Armor Dealer",
    "道具商人": "Tool Dealer",
    "少年": "Boy",
    "少女": "Girl",
    "老人": "Old Man",
    "おばあさん": "Old Woman",
    "兵士": "Soldier",
    "警備兵": "Guard",
    "門番": "Gatekeeper",
    "宿屋": "Innkeeper",
    "居酒屋主人": "Tavern Master",
    "神父": "Priest",
    "シスター": "Nun",
    "修道士": "Monk",
    "船乗り": "Sailor",
    "船長": "Captain",
    "スマイル": "Mr. Smile",
    "ルルムニ": "Rurumu",
    "クローリン": "Clorin",
    "ハイネ": "Heine",
    "ハンス": "Hans",
    "ヴィンセント": "Vincent",
    "アルバイト": "Part-timer",
    "受付": "Receptionist",
    "責任者": "Manager",
    "騎士団長": "Knight Commander",
}

def translate_name(name_ja):
    n = name_ja.strip()
    if "::" in n:
        n = n.split("::", 1)[0].strip()
    for ja, en in JA_EN_NAMES.items():
        if ja in n:
            return n.replace(ja, en)
    return n

def extract_reqs(body):
    items = set(re.findall(r'(?:countitem|checkitem|count_item)\s*\(\s*(\d+)\s*\)\s*>=\s*(\d+)', body))
    zeny = set(re.findall(r'Zeny\s*>=\s*(\d+)', body, re.IGNORECASE))
    job_lv = set(re.findall(r'(?:JobLevel|job_level)\s*(?:>=|<)\s*(\d+)', body, re.IGNORECASE))
    get_items = set(re.findall(r'(?:getitem|get_item)\s*\(?\s*(\d+)\s*,\s*(\d+)\s*\)?', body))
    warps = set(re.findall(r'warp\s*\(?\s*"?([^",\s\)]+)"?\s*,\s*(\d+)\s*,\s*(\d+)\s*\)?', body))
    return {
        "items": sorted(list(items)), "zeny": sorted(list(zeny)), "job_lv": sorted(list(job_lv)),
        "get_items": sorted(list(get_items)), "warps": sorted(list(warps))
    }

def format_diffs(our_reqs, m_reqs):
    diffs = []
    if our_reqs["job_lv"] != m_reqs["job_lv"]:
        diffs.append(f"job_lv: {our_reqs['job_lv']} -> {m_reqs['job_lv']}")
    if our_reqs["zeny"] != m_reqs["zeny"]:
        diffs.append(f"zeny: {our_reqs['zeny']} -> {m_reqs['zeny']}")
    if our_reqs["items"] != m_reqs["items"]:
        diffs.append(f"items: {our_reqs['items']} -> {m_reqs['items']}")
    if our_reqs["get_items"] != m_reqs["get_items"]:
        diffs.append(f"rewards: {our_reqs['get_items']} -> {m_reqs['get_items']}")
    if our_reqs["warps"] != m_reqs["warps"]:
        diffs.append(f"warps: {our_reqs['warps']} -> {m_reqs['warps']}")
    return "; ".join(diffs) if diffs else "none"

HEADER = re.compile(r"^([^\t]+)\t([^\t]+)\t([^\t]*)\t(.*)$")

tsv_rows = []

for tf in TARGET_FILES:
    path = os.path.join(M631_DIR, tf)
    if not os.path.exists(path):
        continue
    with open(path, "rb") as fh:
        raw = fh.read()
    text = raw.decode("cp932", errors="replace")
    lines = text.splitlines()

    i = 0
    while i < len(lines):
        line = lines[i]
        line_s = line.strip()
        if not line_s or line_s.startswith("//"):
            i += 1
            continue
        m = HEADER.match(line)
        if not m:
            i += 1
            continue
        w1, w2, w3, w4 = m.groups()
        w2_clean = w2.strip().lower()
        if w2_clean != "script" and not w2_clean.startswith("duplicate"):
            i += 1
            continue

        start_line = i + 1
        pos_str = w1.strip()
        m_pos = re.match(r"([^,]+?)(?:\.gat)?,(\d+),(\d+),(\d+)", pos_str)
        if not m_pos:
            # pos could be "-"
            if pos_str == "-":
                mp, x, y, d = "-", 0, 0, 0
            else:
                i += 1
                continue
        else:
            mp = m_pos.group(1).lower()
            x = int(m_pos.group(2))
            y = int(m_pos.group(3))
            d = int(m_pos.group(4))

        name_ja = w3.strip()
        name_en = translate_name(name_ja)

        # sprite
        w4_parts = [p.strip() for p in w4.split(",") if p.strip()]
        sprite = -1
        if w4_parts:
            try:
                sprite = int(w4_parts[0])
            except ValueError:
                pass

        # extract body
        body_lines = []
        if "{" in line:
            body_lines.append(line[line.index("{"):])
        while True:
            if body_lines and body_lines[-1].rstrip().endswith("}"):
                break
            i += 1
            if i >= len(lines):
                break
            body_lines.append(lines[i])

        body_text = "\n".join(body_lines)
        m_reqs = extract_reqs(body_text)

        # Walkability check
        if mp == "-":
            walkable = "yes"
        else:
            walkable = "yes" if is_walkable(mp, x, y) else "no"

        # Role summary
        if "kafra" in tf or "カプラ" in name_ja:
            role = "Kafra Service"
        elif "refine" in tf or "精錬" in name_ja:
            role = "Equipment Refine"
        elif "guide" in tf or "案内" in name_ja:
            role = "Town Guide"
        elif "skillget" in tf:
            skills = re.findall(r'getskilllv\((\d+)\)', body_text)
            role = f"Quest Skill (id {','.join(sorted(set(skills)))})" if skills else "Quest Skill"
        elif "job" in tf or "転職" in name_ja:
            role = "Job Change / Test"
        elif "pvp" in tf:
            role = "PvP NPC"
        else:
            role = "Town / Quest NPC"

        # Match with ours
        matched_our = None
        same_cell = "no"
        diffs_str = "none"

        # Try exact cell match
        if (mp, x, y) in our_by_pos:
            matched_our = our_by_pos[(mp, x, y)][0]
            same_cell = "yes"
        else:
            # Try name match on same map or general name match
            if name_en.lower() in our_by_name:
                candidates = our_by_name[name_en.lower()]
                for cand in candidates:
                    if cand["map"] == mp:
                        matched_our = cand
                        break
                if not matched_our and candidates:
                    matched_our = candidates[0]

        if matched_our:
            our_src = matched_our["src"]
            our_reqs = extract_reqs(matched_our["body"])
            diffs_str = format_diffs(our_reqs, m_reqs)
        else:
            our_src = "none"
            same_cell = "no"
            diffs_str = f"mod0631 only: reqs={m_reqs['items']} zeny={m_reqs['zeny']} job={m_reqs['job_lv']}"

        tsv_rows.append([
            tf, start_line, mp, x, y, sprite, name_ja, name_en, role, our_src, same_cell, diffs_str, walkable
        ])
        i += 1

print(f"Total TSV rows generated: {len(tsv_rows)}")

# Write TSV
headers = [
    "m631_file", "m631_line", "map", "x", "y", "sprite", "name_ja", "name_en", "role",
    "ours", "same_cell", "differences", "cells_ok"
]

with open(OUT_TSV, "w", encoding="utf-8", newline="") as fh:
    writer = csv.writer(fh, delimiter="\t")
    writer.writerow(headers)
    for row in tsv_rows:
        writer.writerow(row)

print(f"Written to {OUT_TSV} successfully.")

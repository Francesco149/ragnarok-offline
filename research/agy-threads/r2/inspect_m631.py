import os
import glob
import re

m631_dir = "/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/conf"
target_files = [
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

HEADER = re.compile(r"^([^\t]+)\t([^\t]+)\t([^\t]*)\t(.*)$")

types = {}
total_headers = 0

for tf in target_files:
    path = os.path.join(m631_dir, tf)
    if not os.path.exists(path):
        print(f"Missing: {tf}")
        continue
    with open(path, "rb") as fh:
        raw = fh.read()
    text = raw.decode("cp932", errors="replace")
    file_npcs = 0
    for line_no, line in enumerate(text.splitlines(), 1):
        line_s = line.strip()
        if not line_s or line_s.startswith("//"):
            continue
        m = HEADER.match(line)
        if m:
            w1, w2, w3, w4 = m.groups()
            t = w2.strip().lower()
            if "(" in t:
                t = t.split("(")[0]
            types[t] = types.get(t, 0) + 1
            file_npcs += 1
            total_headers += 1

print(f"Total headers: {total_headers}")
print("Types distribution:", types)

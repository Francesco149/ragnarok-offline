import os
import re

count = 0
sources = set()
imported_npcs = []

for root, dirs, files in os.walk("/work/scripts/import/npc"):
    for f in files:
        if f.endswith(".lua"):
            path = os.path.join(root, f)
            with open(path, "r", encoding="utf-8", errors="replace") as fh:
                text = fh.read()
                # find npc blocks: npc { ... }
                # simple regex matching header attributes
                for block in re.finditer(r'npc\s*\{([^}]+)\}', text):
                    b = block.group(1)
                    src_m = re.search(r'src\s*=\s*"([^"]+)"', b)
                    map_m = re.search(r'map\s*=\s*"([^"]+)"', b)
                    x_m = re.search(r'x\s*=\s*(\d+)', b)
                    y_m = re.search(r'y\s*=\s*(\d+)', b)
                    name_m = re.search(r'name\s*=\s*"([^"]+)"', b)
                    sprite_m = re.search(r'sprite\s*=\s*(\d+)', b)
                    if src_m:
                        src = src_m.group(1)
                        mp = map_m.group(1) if map_m else ""
                        x = int(x_m.group(1)) if x_m else 0
                        y = int(y_m.group(1)) if y_m else 0
                        name = name_m.group(1) if name_m else ""
                        sprite = int(sprite_m.group(1)) if sprite_m else 0
                        imported_npcs.append({
                            "src": src, "map": mp, "x": x, "y": y, "name": name, "sprite": sprite, "file": os.path.relpath(path, "/work")
                        })
                        count += 1
                        sources.add(src.split(":")[0])

print(f"Total imported NPCs: {count}, sources: {sources}")
print("Sample imported NPC:", imported_npcs[0] if imported_npcs else "None")

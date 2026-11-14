#!/usr/bin/env python3
"""Check the castle cells scripts/npc/castle_lord.lua uses against THIS client's maps.

  castle_cells.py <data.grf>

For each castle: is every NPC and arrival cell walkable, which cells make up
the master room (the walkable cells joined to the steward's arrival cell,
found by flood fill, so a room that sits elsewhere on this client's map
than in the emulator's rectangle is still found), is the lever back out in
that room, and does jAthena's box rectangle overlap it. The CASTLES table
below is the one castle_lord.lua carries, copied by hand; keep them equal.
"""
import struct
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from gat_cells import cat  # noqa: E402

# map: emperium, kafra, steward, master-room arrival, lever out (cell -> to), dungeon lever (cell),
#      jAthena box rectangle
CASTLES = {
    "aldeg_cas01": ((216, 24), (218, 170), (218, 175), (119, 223), (123, 223), (211, 179), (114, 218, 123, 227)),
    "aldeg_cas02": ((214, 24), (84, 74), (78, 74), (135, 231), (139, 234), (194, 136), (130, 226, 139, 235)),
    "aldeg_cas03": ((206, 32), (118, 76), (110, 118), (225, 269), (229, 267), (200, 177), (220, 264, 229, 273)),
    "aldeg_cas04": ((36, 218), (45, 88), (67, 116), (85, 12), (83, 17), (76, 64), (80, 8, 89, 17)),
    "aldeg_cas05": ((28, 102), (31, 190), (51, 179), (62, 12), (64, 8), (22, 205), (58, 8, 67, 17)),
    "gefg_cas01": ((198, 182), (83, 47), (40, 48), (155, 112), (152, 117), (78, 84), (150, 108, 159, 117)),
    "gefg_cas02": ((176, 178), (23, 66), (12, 66), (141, 115), (145, 115), (167, 40), (136, 112, 145, 121)),
    "gefg_cas03": ((245, 167), (116, 89), (106, 23), (270, 290), (267, 287), (221, 43), (266, 286, 275, 295)),
    "gefg_cas04": ((174, 178), (59, 70), (73, 46), (116, 118), (113, 115), (58, 75), (112, 114, 121, 123)),
    "gefg_cas05": ((194, 184), (61, 52), (70, 52), (144, 110), (141, 107), (65, 22), (140, 106, 149, 115)),
    "payg_cas01": ((139, 139), (128, 58), (120, 58), (290, 7), (287, 5), (101, 25), (295, 8, 304, 17)),
    "payg_cas02": ((39, 25), (22, 275), (22, 260), (145, 143), (141, 141), (278, 247), (140, 140, 149, 149)),
    "payg_cas03": ((269, 265), (9, 263), (268, 264), (158, 168), (155, 165), (38, 42), (154, 164, 163, 173)),
    "payg_cas04": ((271, 29), (40, 235), (38, 284), (146, 48), (143, 45), (52, 48), (142, 44, 151, 53)),
    "payg_cas05": ((30, 30), (276, 227), (277, 249), (156, 131), (153, 129), (238, 11), (152, 128, 161, 137)),
    "prtg_cas01": ((197, 197), (96, 173), (112, 181), (11, 208), (15, 209), (94, 200), (6, 204, 15, 213)),
    "prtg_cas02": ((158, 174), (71, 36), (94, 61), (202, 228), (206, 228), (84, 72), (198, 224, 207, 233)),
    "prtg_cas03": ((17, 221), (181, 215), (51, 100), (189, 132), (193, 130), (5, 70), (184, 128, 193, 137)),
    "prtg_cas04": ((292, 14), (258, 247), (259, 265), (271, 162), (274, 161), (56, 283), (266, 158, 275, 167)),
    "prtg_cas05": ((266, 266), (52, 41), (36, 37), (277, 178), (280, 177), (212, 95), (272, 174, 281, 183)),
}


def main():
    grf = sys.argv[1]
    for m, (emp, kafra, stew, arr, lev, dun, rect) in CASTLES.items():
        data = cat(grf, f"data\\{m}.gat")
        w, h = struct.unpack_from("<ii", data, 6)

        def ok(x, y):
            if not (0 <= x < w and 0 <= y < h):
                return False
            return struct.unpack_from("<I", data, 14 + (y * w + x) * 20 + 16)[0] not in (1, 5)

        room, todo = set(), [arr] if ok(*arr) else []
        while todo and len(room) < 400:
            c = todo.pop()
            if c in room or not ok(*c):
                continue
            room.add(c)
            todo += [(c[0] + dx, c[1] + dy) for dx in (-1, 0, 1) for dy in (-1, 0, 1)]
        xs = [c[0] for c in room] or [0]
        ys = [c[1] for c in room] or [0]
        bbox = (min(xs), min(ys), max(xs), max(ys))
        inrect = sum(1 for c in room if rect[0] <= c[0] <= rect[2] and rect[1] <= c[1] <= rect[3])
        bad = [n for n, c in (("emperium", emp), ("kafra", kafra), ("steward", stew), ("dungeon lever", dun))
               if not ok(*c)]
        # a lever is an NPC: it may stand on a blocked cell, but must be clickable from the room
        near = any((lev[0] + dx, lev[1] + dy) in room for dx in range(-2, 3) for dy in range(-2, 3))
        print(f"{m:12} {w}x{h} room {len(room):3} cells bbox {bbox} jAthena rect {inrect:3} of them"
              f" | lever out {'by the room' if near else 'AWAY FROM ROOM'}"
              f"{' | BLOCKED: ' + ', '.join(bad) if bad else ''}{' | ROOM OPEN' if len(room) >= 400 else ''}")


if __name__ == "__main__":
    main()

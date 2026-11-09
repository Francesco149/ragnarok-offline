"""Map names in the Athena sources -> this client's names.

The 2003-12-18 client carries some maps under older names than the Athena
scripts of the time (which follow kRO/jRO). Each alias was chosen by checking
every NPC/warp cell the scripts use on that map against the client's .gat
(tools: research/notes/maps.md). Parallel copies the official servers ran
for crowding (new_2-1 .. new_5-4, sword_2-1, priest_2-1, ordeal_1-3 ...) are
left unaliased: a single player needs one, and the server skips entries on
maps the client lacks.
"""

MAP_ALIAS = {
    # novice training ground: 30 of 32 cells walkable
    "new_1-1": "new_zone01", "new_1-2": "new_zone02", "new_1-3": "new_zone03", "new_1-4": "new_zone04",
    # swordsman test course: 207 of 209 cells walkable
    "sword_1-1": "job_sword1",
    # priest test: 4 of 4
    "priest_1-1": "job_priest",
    # Izlude arena rooms: 36/38, 44/44, 76/76
    "force_1-1": "force_map1", "force_1-2": "force_map2", "force_1-3": "force_map3",
    # labyrinth rooms: 32/32 and 16/16
    "ordeal_1-1": "ordeal_a00", "ordeal_1-2": "ordeal_a02",
}


def alias(name):
    return MAP_ALIAS.get(name, name)

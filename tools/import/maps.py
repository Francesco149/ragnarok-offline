"""Map names in the Athena sources -> this client's names.

The 2003-12-18 client carries some maps under older names than the Athena
scripts of the time (which follow kRO/jRO). The client says so itself: its
`data\\resnametable.txt` (in data.grf) redirects the newer names to the files
it has, e.g. `new_1-1.gat#new_zone01.gat#`, and likewise .gnd/.rsw. It lists
new_{1..5}-{1..4} -> new_zone0{1..4}, sword_{1..3}-1 -> job_sword1,
priest_{1..3}-1 -> job_priest, knight_N-1 -> job_knight, hunter_N-1 ->
job_hunter, wizard_N-1 -> job_wizard, force_N-{1,2,3} -> force_map{1,2,3},
ordeal_N-{1,2,3,4} -> ordeal_a0{0,2,3,4}, and the PvP/guild-versus copies of
towns and job maps. So a server of the time sent "new_1-1" and this client
drew new_zone01. Before that table was read, each alias below had been
chosen by checking every NPC/warp cell the scripts use against the client's
.gat (research/notes/maps.md); the counts are kept as a second witness.

Parallel copies the official servers ran for crowding (new_2-1 .. new_5-4,
sword_2-1, priest_2-1 ...) are left unaliased: a single player needs one,
and the server skips entries on maps the client lacks. ordeal_a03/a04 are
named in the table but their files are not in this client's data.

Not aliases: `job_wiz`, `job_knt`, `job_prist`, `job_hunte` (warps of
mod0631, the Wizard test of eAthena 0.5.2). They are later maps; their cells
do not fit this client's job_wizard / job_knight / job_priest / job_hunter
(3 of 10, 0 of 4, 1 of 3 and 1 of 2 warp cells walkable).
"""

MAP_ALIAS = {
    # novice training ground: resnametable; 30 of 32 cells walkable
    "new_1-1": "new_zone01", "new_1-2": "new_zone02", "new_1-3": "new_zone03", "new_1-4": "new_zone04",
    # swordsman test course: resnametable; 207 of 209 cells walkable
    "sword_1-1": "job_sword1",
    # priest test: resnametable; 4 of 4
    "priest_1-1": "job_priest",
    # arena rooms: resnametable; 36/38, 44/44, 76/76
    "force_1-1": "force_map1", "force_1-2": "force_map2", "force_1-3": "force_map3",
    # labyrinth rooms: resnametable; 32/32 and 16/16
    "ordeal_1-1": "ordeal_a00", "ordeal_1-2": "ordeal_a02",
}


def alias(name):
    return MAP_ALIAS.get(name, name)

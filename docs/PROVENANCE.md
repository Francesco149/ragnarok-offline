# Provenance: where every value comes from

Nothing server-side survives from euRO itself (its server software was
Gravity's and was never released). So each value is tagged with how much it
can be trusted. Tiers (best first), as `ro_data::db::Tier`:

| tier | meaning | examples |
|---|---|---|
| `client` | read from the client's files or binary: confirmed | packet lengths, map cells, item/map display names, NPC sprites |
| `official` | Gravity/publisher material from the period | kRO patch notices, the 2004 iRO official guide (Wayback) |
| `athena2003` | Athena-family emulator data contemporary with the client (2003-12 .. 2004-05) | items, monsters, EXP, job tables, warps, spawns, shops, formulas |
| `period` | community measurements/documents written 2002–2004 | archived player databases and guides |
| `later` | reconstructions after 2004 (rAthena `pre-re`, iRO Wiki Classic) — often reflect ~2008–2012, not 2003 | |
| `guess` | placeholder / our estimate | |

A TOML table sets `[meta] tier/source/note`; a record may override with its
own `tier`. Lua rules cite their source in comments (file + line of the
period source). Research behind the tiers: `research/notes/mechanics-sources.md`.

## What is what today

| Area | Source | Tier |
|---|---|---|
| Packet lengths | `Ragexe.exe` packet map (`research/extract_packet_lengths.py`) | client |
| Packet layouts | client receive code for 0x0069/0x006a/0x006b/0x006d; others Athena layouts verified by the real client drawing them correctly | client / athena2003 |
| Map walkability | client `.gat` | client |
| Pathfinding | Athena path.c algorithm | athena2003 (client's own finder: ROADMAP) |
| Walk speed 150 ms/cell, diagonal ×1.4 | client default + Athena | athena2003 |
| Items, monsters, drops | athena-dev-2.1.1-mod0631 `db/` (archived 2003-12-18) — numbers; English names from eAthena 0.5.2 (2004-05) | athena2003 |
| EXP tables, job HP/SP/weight/ASPD | mod0631 `db/exp.txt`, `job_db1.txt`, `job_db2.txt` | athena2003 |
| Warps (2,038), spawns (2,001 groups), shops (80) | mod0631 `conf/map_athena.conf` npc list | athena2003 |
| HP/SP/ATK/hit/flee/crit/ASPD formulas | mod0631 `map/pc.c` pc_calcstatus | athena2003 |
| Damage, hit chance, criticals, DEF | mod0631 `map/battle.c` battle_calc_weapon_attack (no element/size/cards yet) | athena2003 |
| Natural regen | mod0631 `map/pc.c` natural heal (4 s HP / 8 s SP clocks) | athena2003 |
| Death penalty 1%, respawn 1 HP | mod0631 `conf/battle_athena.conf` | athena2003 |
| Start point `new_zone01 53,111` | Athena's `new_1-1 53,111` on this client's older training map (cell verified walkable) | guess |
| Monster AI | simplified Athena behaviour (aggressive pick-up, chase, random walk) | guess |
| Guide/Kafra/Counter Boy NPCs | written for this project | guess |
| `@go` coordinates | rAthena | later |
| Client first-run settings written by `RagnarokOffline.exe` (registry) | the value set the client's own `Setup.exe` writes (observed on Windows 11); 800×600×16 fullscreen picked by us | client / guess |
| DDrawCompat settings (`DDrawCompat.ini`) | chosen for this client, reasons in docs/WINDOWS.md | guess |
| Patch server files (notice, list, allow, front page) | euRO's `wpatch.euro-ro.net` as archived by the Wayback Machine: list 2004-02-21, notices 2004-07-18, front page 2004-02-21 (digests checked) | official |
| Served patch list | the archived list with entries above the CD's `patch.inf` (303) commented out | official / ours |

## Known period disagreements (don't "fix" these silently)

From `research/notes/mechanics-sources.md`: Baphomet 568k HP in Dec 2003 data
vs 668k in Feb 2004; the official iRO element table differs from emulators on
the level-1 Poison row; party EXP bonus +25% per member (kRO 2003-02) vs +10%
(2004 iRO guide); Kafra teleport fares changed 2004-02-24; Phracon 2000z
(iRO page) vs 200z (data); armor DEF per refine recorded as 1, 5 and 0.7 in
different emulator versions. When you pick one, record which and why.

## euRO specifics

euRO ran 1x rates (licence forbade modification; no rate was ever published),
max level 99, three character slots, WoE from open beta, and opened Juno/2-2
jobs only on 2004-06-09. The closed beta (this disc) had "higher experience"
of unknown size. Details and sources: `research/notes/euro-history.md`.

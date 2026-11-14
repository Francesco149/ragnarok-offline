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
| Pathfinding | the client's `CPathFinder` (`0x4d2300`, docs/CLIENT.md), checked under an emulator; the 32-step limit and monsters' straight random walks are Athena's path.c | client / athena2003 |
| Walk speed 150 ms/cell, diagonal `(int)(speed × 1.414f)`, the first step and when a unit enters each cell | client default; the step times from the client's walk timing (`0x4d2cf0`, `path::client_times`, checked under an emulator; docs/CLIENT.md, "The walk clock and step times") | client |
| Items, monsters, drops | athena-dev-2.1.1-mod0631 `db/` (archived 2003-12-18) — numbers; English names from eAthena 0.5.2 (2004-05). The 95 drop slots mod0631 leaves as `512,0` placeholders take eAthena 817's item and rate (`research/mob_drop_holes.py`; RODataZone's iRO table of 2004-02 names about 80 of them) | athena2003 |
| EXP tables, job HP/SP/weight/ASPD | mod0631 `db/exp.txt`, `job_db1.txt`, `job_db2.txt` | athena2003 |
| Warps (2,038), spawns (2,001 groups), shops (80) | mod0631 `conf/map_athena.conf` npc list | athena2003 |
| Spawn counts on Geffen Dungeon 1F-3F, Glast Heim castle 1F and inner rooms, Payon Cave 1F's Skeletons (14 pairs) | Ragu Memo's jRO dungeon pages of 2003-12, which differ from its 2003-07 captures (= mod0631); rAthena pre-renewal keeps the December figure where it did not change the monster again (`SPAWNS_2003_12`, THREADS #10) | research (later as support) |
| Warp tiles that are scripts (22): Ant Hell's two doors, Payon Cave 3F→4F, the random exits of the Clock Tower and of `aldeg_cas04` | mod0631 `conf/npc_warp.txt`, `npc_warp25.txt` (`script` entries with sprite 45), generated into `scripts/import/warps.lua`; a `rand(n)` with fewer branches than n falls through to the first, as the source does | athena2003 |
| HP/SP/ATK/hit/flee/crit/ASPD, job bonuses, equipment and card bonuses, refine bonuses, passive skills | mod0631 `map/pc.c` pc_calcstatus, pc_bonus, pc_bonus2; `db/size_fix.txt`, `db/refine_db.txt`; the items' own scripts (`scripts/rules/status.lua`) | athena2003 |
| Weapon damage: hit, critical, DEF, masteries, size, cards, elements, two weapons, Double Attack | mod0631 `map/battle.c` battle_calc_weapon_attack, battle_addmastery, `db/attr_fix.txt` (`scripts/rules/battle.lua`) | athena2003 |
| Magic and trap/falcon damage | mod0631 `map/battle.c` battle_calc_magic_attack, battle_calc_misc_attack | athena2003 |
| Skills: what each does, cast time, delay, SP, gemstones, weapon needs | mod0631 `map/skill.c`, `db/skill_db.txt`, `db/cast_db.txt` (`scripts/rules/skill*.lua`, `data/db/skills.toml`); gaps from eAthena 817 `map/skill.c`, marked in the Lua; the few neither has are listed in DEVIATIONS.md | athena2003 / guess |
| Skill SP per level | the client's `leveluseskillspamount.txt` | client |
| Ground skills (walls, traps, Sanctuary, storms, portals): cells, timing, effects | mod0631 `map/skill.c` skill_unitsetting, skill_unit_onplace and friends (`scripts/rules/skill_units.lua`) | athena2003 |
| Status changes: which, how long, what they change | mod0631 `map/skill.c` skill_status_change_start, `map/pc.c`, `map/battle.c` (`scripts/rules/status_change.lua`); numbers below 64 are the client's icon ids | athena2003 / client |
| Ailments: chance, resistance by stats, length; poison's HP loss | eAthena 817 `map/skill.c` skill_additional_effect, skill_status_change_start, `db/cast_db.txt` time columns (2004-02-19). mod0631's are placeholders | athena2003 |
| Skill packets (cast bar, skill damage, ground cells, status icons, option words) | layouts: mod0631 `map/clif.c`; lengths: the client; the real client draws them (cast bar, bolts, Fire Wall, Safety Wall, Sight, a frozen monster; 2026-11-10) | client / athena2003 |
| Natural regen, Increase HP/SP Recovery, Moving HP Recovery, no regen at half weight or hidden | mod0631 `map/pc.c` pc_natural_heal_hp / _sp (4 s HP / 8 s SP clocks; 10 s skill clocks) | athena2003 |
| Arrows: one a shot, element from the arrow, no ATK of their own | mod0631 `map/battle.c:1687`, `map/pc.c:854` (its arrow ATK line is commented out) | athena2003 |
| Steal, Steal Coin | mod0631 `map/pc.c` pc_steal_item, pc_steal_coin | athena2003 |
| Arrow Crafting and forge recipes; forging success rates, anvils, Star Crumbs and elemental stones | mod0631 `db/create_arrow_db.txt`, `db/produce_db.txt` (`scripts/import/craft.lua`), `map/skill.c` skill_produce_mix (`scripts/rules/produce.lua`) | athena2003 |
| Discount, Overcharge; healing items scaled by VIT / INT and the recovery skills; overweight icons at 50% and 90% | mod0631 `map/pc.c` pc_modifybuyvalue, pc_modifysellvalue, pc_itemheal, pc_checkweighticon | athena2003 |
| Death: 1% of the base and job EXP held, taken on returning to the save point; back with 1 HP (a Novice: half) | mod0631 `conf/battle_athena.conf` (death_penalty_type 0), `map/pc.c` pc_setrestartvalue | athena2003 |
| Start point `new_zone01 53,111` | Map: the client redirects `new_1-1` to `new_zone01` itself (`data\\resnametable.txt`). Cell: `start_point: new_1-1.gat,53,111` of every Athena config of 2003–04 (jAthena //0307); where they got it is unknown (THREADS.md #6) | client (map) / athena2003 (cell) |
| Monster AI: targets, chase, attack, assist, looting, slaves, random walk | mod0631 `map/mob.c` mob_ai_sub_hard and friends (`crates/ro-game/src/combat.rs`). Maps nobody is on stand still (DEVIATIONS.md) | athena2003 |
| Monster skills: which, when, how often | mod0631 `db/mob_skill_db.txt` (`data/db/mob_skills.toml`), `map/mob.c` mobskill_use | athena2003 |
| Monster skills iRO had in April 2004 that mod0631 lacks (93 pairs) | Which monster: unknown13's Monster Skill FAQ (GameFAQs 29608, 2004-04; `tools/import/mob_skills_faq.tsv`); Goblin and Kobold by weapon from the client's sprites. How: jAthena //0953 `db/mob_skill_db.txt` (2004-04-27), else rAthena pre-renewal (rate / 5), else the skill's commonest row on other monsters (`data/db/mob_skills_faq.toml`, each row its own tier) | period / athena2003 / later / guess |
| The bosses' rule: every MVP uses Two-Hand Quicken, Increase Agility, Guided Attack and Piercing Attack, and teleports when hit by someone it cannot reach | The rule: the same FAQ quoting iRO. The rows: as above; Teleport on "rudeattacked" as rAthena reads it (`src/map/mob.cpp:1936-1976`; raised in `combat.rs` `damage_mob`) | athena2003 / later / guess |
| EXP share, drops (equipment unidentified), looted items, MVP reward | mod0631 `map/mob.c` mob_damage | athena2003 |
| Card insertion, item identification | mod0631 `map/pc.c` pc_insert_card, `map/clif.c` clif_use_card, clif_item_identify_list | athena2003 |
| NPC scripts (611 NPCs: towns, job quests, training ground, guides, refiners, quests) | eAthena 817 `npc/` (2004-02-19), translated by `tools/import/athena_npc.py`; each NPC's `src` gives file:line; source bugs kept, fixes in `scripts/npc/` (DEVIATIONS.md) | athena2003 |
| Kafra service (`scripts/npc/kafra.lua`) | 817 places/sprites/save cells/destinations and wording; fares as 817's menus and mod0631 show them; storage fee per place, Comodo and Al De Baran routes, Izlude's save cell and six more Kafras from mod0631 `conf/npc_town_kafra.txt` (jRO lineage, archived 2003-12-18); storage rule from the client's Basic Skill description | athena2003 / client |
| Map names of the training ground and test maps | The client's own `data\\resnametable.txt` (`new_1-1.gat#new_zone01.gat#` ...); before that, NPC/warp cells of the scripts checked against the client's `.gat` (`tools/import/maps.py`) | client |
| Skill tree, max levels, targets | mod0631 `db/skill_db.txt`, `db/skill_tree.txt` (quest flag from 817) | athena2003 |
| Skill SP per level | the client's `leveluseskillspamount.txt` (read from the GRF at start); skills.toml as fallback | client |
| Basic Skill unlocks (sit 3, emote 2, storage 6...) | the client's `skilldesctable.txt` | client |
| Refine success chances | mod0631 `db/refine_db.txt` (`scripts/rules/refine.lua`) | athena2003 |
| Kafra storage size (100) and layout | 817 `common/mmo.h`, `map/storage.c` | athena2003 |
| Pushcart: 100 entries, 8000 weight | 817 `common/mmo.h` MAX_CART, `battle_athena.conf` max_cart_weight ×10 (`battle.c:7837`); the client shows "0/8000" | athena2003 |
| Status changes (so far: speed potions 30 min, attack delay 90/85/80%) | mod0631 `map/skill.c` skill_status_change_start, `map/pc.c:895` (`scripts/rules/status_change.lua`, `status.lua`) | athena2003 |
| Pets: tameable monsters, taming items, eggs, food, accessories, hunger and intimacy numbers; catch chance and feeding rules | mod0631 `db/pet_db.txt` (`data/db/pets.toml`), `map/pet.c` | athena2003 |
| Map flags (noteleport, nobranch, nopenalty, nosave; nomemo and pvp recorded) | mod0631 `conf/mapflag.txt` and the mapflag lines of its NPC files (`data/db/mapflags.toml`) | athena2003 |
| Monsters in ownerless guild castles | eAthena 817 `npc/mobs/gvg.txt` (2004-02-19); kRO notices of 2004-02-10/17 say such monsters existed and were then removed. When they appear (each server start) is ours | athena2003 / guess |
| Old Blue/Violet Box, Card Album, Gift Box contents; Dead Branch monsters | mod0631 `db/item_*box.txt`, `item_cardalbum.txt`, `mob_branch.txt` (`data/db/random.toml`); picking as mod0631 `itemdb_searchrandomid` / `mob_once_spawn` (branch monster not above your level) | athena2003 |
| `@go` coordinates | rAthena | later |
| Client first-run settings written by `RagnarokOffline.exe` (registry) | the value set the client's own `Setup.exe` writes (observed on Windows 11); 800×600×16 fullscreen picked by us | client / guess |
| DDrawCompat settings (`DDrawCompat.ini`) | chosen for this client, reasons in docs/WINDOWS.md | guess |
| Patch server files (notice, list, allow, front page) | euRO's `wpatch.euro-ro.net` as archived by the Wayback Machine: list 2004-02-21, notices 2004-07-18, front page 2004-02-21 (digests checked) | official |
| Served patch list | the archived list with entries above the CD's `patch.inf` (303) commented out | official / ours |

## The client's skill text

`data\skilldesctable.txt` in the client gives per-level numbers for many
skills. It is Gravity's own, but not kept up to date: it gives Teleport ten
levels and Kyrie Eleison a length by level, as in kRO's beta. So it is used
where the emulators have nothing (DEVIATIONS.md lists those), and where it
disagrees with them the emulator's value stands: the iRO guide's skill
pages of spring 2004 give the emulator's number on every such row
(THREADS.md R6).

## Known period disagreements (don't "fix" these silently)

From `research/notes/mechanics-sources.md`: Baphomet 568k HP in Dec 2003 data
vs 668k in Feb 2004; the official iRO element table differs from emulators on
the level-1 Poison row; party EXP bonus +25% per member (kRO 2003-02) vs +10%
(2004 iRO guide); Kafra teleport fares changed 2004-02-24; armor DEF per
refine recorded as 1, 5 and 0.7 in different emulator versions and as "1
defense" on the iRO page, while kRO's notice of 2003-02-04 says refine DEF
counts for less than the armor's own; refine fees ten times higher in
mod0631 (jRO) than in 817 and on the iRO page. When you pick one, record
which and why.

Settled since (research/notes/round2-review.md): the iRO refine page of
2004-06 had Phracon at 2000z and level-4 weapons at the level-3 fee and
bonus; Gravity corrected both on the page itself (200z by 2004-08-10;
20,000z and +7 by 2004-12-05), and kRO's notice of 2003-02-04 gives level 4
its +7. The data was right.

mod0631 follows jRO. Its zeny amounts (refine fees, the per-place storage
fees we took from it) are jRO's and may not have been euRO's.

## euRO specifics

euRO ran 1x rates (licence forbade modification; no rate was ever published),
max level 99, three character slots, WoE from open beta, and opened Juno/2-2
jobs only on 2004-06-09. The closed beta (this disc) had "higher experience";
one player's post of 2004-01-27 puts EXP and drops at 3x, and 2-1 jobs came
without quests until "New Comodo" in January 2004 (round2-review.md). Details and sources: `research/notes/euro-history.md`.

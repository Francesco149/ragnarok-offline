# Roadmap

Ordered by "confirmable first". Each item says where the truth would come from.

## 1. Confirm more from the client binary (tier `client`)

- **Client→server lengths**: read the send code for 0x01d5 / 0x019c (and
  scan all `send` call sites) instead of trusting `CLIENT_SEND_OVERRIDES`.
- **Remaining layouts** the server already sends but only Athena + "it draws
  right" vouch for: 0x0079, 0x007b, 0x00a0, 0x00a3/0x00a4, 0x00bd, 0x01d7,
  0x009d/0x009e, 0x0119. Find each handler via the zone-mode dispatch (like
  the login-mode jump table at `0x52fa88`).
- **Path finder**: the client walks its own path to the target; extract its
  algorithm (`CPathFinder`) and make `path.rs` produce identical paths.
- **Walk/attack timing** the client animates (sub-cell, half-step) vs ours.
- Client tables not used yet: skill names/descriptions, `itemslottable`,
  `leveluseskillspamount`, `mapnametable` (for messages), `num2cardillustnametable`.

## 2. euRO content state

euRO opened Juno and the 2-2 jobs on 2004-06-09; the closed beta (this disc,
Dec 2003) and the "New Comodo" update (Jan 2004) define the default world.
Add a `content_date` / episode setting that filters `scripts/import/*`
(e.g. Juno warps) and NPCs by date, default 2004-01. Closed-beta EXP boost:
unknown factor, keep as a rate.

## 3. Battle completeness (tier `athena2003` sources already on disk)

From athena-dev mod0631 (`~/data/ragnarok-cd/refs/eathena-archive/x/...`):
element table `db/attr_fix.txt`, size modifiers `db/size_fix.txt` + weapon
`atkmods`, refine bonuses `db/refine_db.txt`, card/equipment bonus scripts
(`athena_equip` in items.toml → Lua `bonus` API), double attack, arrows,
MDEF/magic damage. Monster element/race/size are already in mobs.toml.

## 4. Skills

`db/skill_db.txt`, `skill_tree.txt`, `cast_db.txt` → TOML; packets 0x010f
(list), 0x0112 (level up), 0x0113/0x0116/0x0190 (use), 0x0114/0x011a/0x0117
(results), 0x013e (casting). Skill effects in Lua (`rules/skills/*.lua`).

## 5. Status effects

`sc_start`/`sc_end` (poison, stun, blessing, AGI up, ...) with timers and the
0x0196 icon packet; item scripts already call `sc_end`.

## 6. Monsters

Athena `mob.c` behaviour: random-walk cadence, chase/give-up ranges, looters,
assist, cast sensor; monster skills (`db/mob_skill_db.txt`); MVP rewards;
`mob_branch.txt` (Dead Branch).

## 7. NPC content

Translate the era Athena NPC scripts (`conf/npc_*.txt`) into Lua: Kafra
(save/storage/teleport with pre-2004-02-24 fares), job-change quests,
refiners, healers, quest NPCs. A converter for the common subset (mes/next/
menu/close/warp/set/if/goto) would do most of the work; English text from
AppleMod 641 / Athena 659.

## 8. Systems

Kafra storage, carts, pets (egg/taming/hunger), item identification, skill
reset NPCs; party/guild/trade/vending only matter for multiplayer.

## 9. Robustness

Done: `RagnarokOffline.exe` (drop-in launcher) and the all-in-one installer
(docs/WINDOWS.md); open items there under "Not verified yet". Next: packet fuzz tests against the length table;
`check` validating that every item/mob id referenced by scripts exists.

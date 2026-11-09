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

Done: tree, points, skill window (0x010f/0x010e/0x0112), quest skills from
NPCs, reset, Basic Skill gating. Next: using skills — `cast_db.txt` → TOML;
packets 0x0113/0x0116/0x0190 (use), 0x0114/0x011a/0x0117 (results), 0x013e
(casting). Skill effects in Lua (`rules/skills/*.lua`).

## 5. Status effects

`sc_start`/`sc_end` (poison, stun, blessing, AGI up, ...) with timers and the
0x0196 icon packet; item scripts already call `sc_end`.

## 6. Monsters

Athena `mob.c` behaviour: random-walk cadence, chase/give-up ranges, looters,
assist, cast sensor; monster skills (`db/mob_skill_db.txt`); MVP rewards;
`mob_branch.txt` (Dead Branch).

## 7. NPC content

Done: eAthena 817's NPCs translated (`tools/import/athena_npc.py`,
611 NPCs), the runtime they need (events, timers, variables, script
monsters, refining, storage), a hand-written Kafra service. Next, in order:

* play through each 1st-job quest and the training ground in the client;
  fix what breaks in `scripts/npc/` (one file per fix, listed in
  DEVIATIONS.md);
* hand-port the Wizard quest (817's is in a later dialect; THREADS.md #8);
* NPC waiting rooms (0x00d7 chat bubbles over arena/PvP NPCs);
* a period dyer (THREADS.md #13); English shop names from 817 by cell (#11);
* better dialogue where period sources exist (THREADS.md #2).

## 8. Systems

Done: Kafra storage, pushcart contents, pets (taming, eggs, hatching,
following, hunger, intimacy, feeding, accessories, renaming). Next: item
identification, status changes (`sc_start`). Party/guild/trade/vending
only matter for multiplayer (DEVIATIONS.md).

## 9. Robustness

Done: `RagnarokOffline.exe` (drop-in launcher) and the all-in-one installer
(docs/WINDOWS.md); open items there under "Not verified yet". Next: packet fuzz tests against the length table;
`check` validating that every item/mob id referenced by scripts exists.

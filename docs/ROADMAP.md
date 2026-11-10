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

## 3. Battle, skills, status changes, monsters

Done (scripts/rules/, crates/ro-game/src/battle.rs and combat.rs; sources
in PROVENANCE.md, differences in DEVIATIONS.md): equipment, card and job
bonuses; elements, sizes, masteries, refine bonuses, criticals, Double
Attack, two weapons, arrows; the skills of the Novice, the six first jobs
and the six 2-1 jobs with cast times, delays, SP and item costs; ground
skills; status changes with icons and option words; ailments with
resistance; natural recovery with its skills; monster AI and monster
skills; MVP rewards; card insertion and item identification.

Next:

* **Forging** (Blacksmith): `produce_db` is imported (`scripts/import/craft.lua`),
  the window (0x018d / 0x018e / 0x018f) and the success rates
  (mod0631 `skill.c:4549`) are not.
* **Skills left out** (DEVIATIONS.md, THREADS.md #22): Poison React, Venom
  Splasher, Remove Trap, Spring Trap, Talkie Box's text, attacking an Ice
  Wall.
* **The 2-2 jobs' skills** (behind `jobs2_2`): Crusader, Monk, Sage, Rogue,
  Alchemist, Bard, Dancer. mod0631 has most; nothing is ported.
* **Line of sight** for skills and bows (the emulator's path check).
* **Check against the client's skill text and period guides**
  (THREADS.md #20, #21).
* Play every job in the real client: skill window, hotkeys, each skill's
  effect and sound.

## 4. NPC content

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

## 5. Systems

Done: Kafra storage, pushcart contents, pets (taming, eggs, hatching,
following, hunger, intimacy, feeding, accessories, renaming), item
identification, cards. Party/guild/trade/vending only matter for
multiplayer (DEVIATIONS.md).

## 6. Robustness

Done: `RagnarokOffline.exe` (drop-in launcher) and the all-in-one installer
(docs/WINDOWS.md); open items there under "Not verified yet". Next: packet fuzz tests against the length table;
`check` validating that every item/mob id referenced by scripts exists.

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
skills; MVP rewards; card insertion and item identification; forging and
ore refining; Discount and Overcharge; potions scaled by VIT and INT; the
overweight icons.

Next:

* **Skills left out** (DEVIATIONS.md, THREADS.md #22): Talkie Box's text,
  attacking an Ice Wall, Venom Splasher's delayed burst. Poison React,
  Remove Trap and Spring Trap are drafts to try in the client.
* **The 2-2 jobs' skills** (behind `jobs2_2`): Crusader, Monk, Sage, Rogue,
  Alchemist, Bard, Dancer. Their damage formulas and status changes came
  over with the battle port; what each does at cast end did not. mod0631
  is thin here (the Strips are commented out, Chemical Protection is
  empty, Auto Guard, Defender, Volcano and half the songs sit behind
  `#if 0`, Potion Pitcher is nonsense, the Monk needs spirit spheres and
  combo timing the server does not have). A port has to lean on 817 and
  the iRO guide's skill pages of spring 2004
  (research/notes/round4-review.md), skill by skill: a draft, not a
  transcription. This client has no packets for spirit spheres, combos,
  Devotion or Auto Spell (THREADS.md #25): start with the skills that need
  none. Drafted: the Crusader's Guard, Defender, Shield Reflect, Shield
  Charge, Shield Boomerang and Holy Cross (test `crusader_shield_skills`),
  Providence, the Sage's endows and Dispell, the Rogue's Strips, the
  Alchemist's Chemical Protections (test
  `endows_strips_protections_dispell`). **The client cannot offer any of
  them** (THREADS.md #25, seen in the rig): its exe knows no 2-2 skill, so
  the skill window leaves them out and a cast shouts "Unknown Skill !!";
  only the status icons are drawn. Until a later exe is found (euRO's of
  2004-04-15 or 2004-05-31) they are reachable through `@useskill` alone,
  so more 2-2 drafts come after that search. Then: Grand Cross, the Sage's
  ground skills, Magic Rod and Spell Breaker, Potion Pitcher, Acid Terror,
  Demonstration.
* **Check the drafted skills against the iRO guide's pages** (THREADS.md
  #22) and the ailments against period guides (#21). The client's skill
  text against the emulators is settled (R6).
* Play every job in the real client: skill window, hotkeys, each skill's
  effect and sound.

## 4. NPC content

Done: eAthena 817's NPCs translated (`tools/import/athena_npc.py`,
611 NPCs), the runtime they need (events, timers, variables, script
monsters, refining, storage), a hand-written Kafra service; a test that
reaches every job and learns every quest skill through the NPCs
(`tests/job_quests.rs`, on all-walkable maps); from mod0631: the Peco Peco
Breeder, the dye makers and the hairdresser, the Turtle Island chain, the
doomed swords and Alberta's doll child; the Wizard test on `job_wiz` from
817's unfinished script.
Next, in order:

* play through each 1st-job quest and the training ground in the client
  (the test does not walk the real maps). The Swordman course is checked
  on the client's map by search (`research/course_check.py`: each stretch
  has a way past the pits to its warp, and every pit drops you somewhere
  with a way out) but not yet played; the Priest test is untried there.
  Fix what breaks in `scripts/npc/` (one file per fix, listed in
  DEVIATIONS.md);
* the Knight's and the Hunter's tests on `job_knt` and `job_hunte`, which
  the client has (THREADS.md #8): no period script is known;
* NPC waiting rooms (0x00d7 chat bubbles over arena/PvP NPCs);
* clothes dyeing (THREADS.md #13: the dye makers and the hairdresser are
  in, from mod0631); English shop names from 817 by cell (#11);
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

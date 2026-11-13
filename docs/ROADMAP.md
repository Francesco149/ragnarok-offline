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
Done as switches rather than a date: `[game.content]` (juno, jobs2_2, pvp,
events, arena, castle_monsters; DEVIATIONS.md lists the defaults). A
single `content_date` that sets them is still possible. Closed-beta EXP
boost: unknown factor, keep as a rate.

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
  attacking an Ice Wall, Venom Splasher's delayed burst (no period source
  for its timer). Sandman's sleep on those nearby is in. Poison React,
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
  Tried in the client on 2026-11-13 (`@useskill`): the Assassin's Sonic
  Blow, Enchant Poison, Venom Dust and Cloaking (which ends away from a
  wall, as in mod0631, `skill.c:4132`); the Blacksmith's Weapon
  Perfection, Over Thrust and Maximize Power, and the refusal of Adrenaline
  Rush and Hammer Fall with a katar worn. Later the same day: the Hunter's
  falcon from the Falcon Breeder, every trap, Remove Trap and Spring Trap
  aimed at a trap in the skill window, Blitz Beat, Double Strafe and Arrow
  Shower; the Wizard's spells (Sense opens the monster window); the
  Priest's spells on a Zombie and the blessings on himself; the Knight's
  spear skills on a Peco Peco, Two-Hand Quicken and Counter Attack. Playing
  found: Ankle Snare ended when any monster walked near it (mod0631's slip,
  fixed as jAthena did), a skill asked for from one cell beyond its range
  was dropped (the client sends it from there), Sightrasher did nothing (now
  drafted), and a job change to less SP crashed the debug server at the next
  recovery tick. Not built yet: Frost Nova draws nothing on a target
  (THREADS.md #22).

## 4. NPC content

Done: eAthena 817's NPCs translated (`tools/import/athena_npc.py`,
611 NPCs), the runtime they need (events, timers, variables, script
monsters, refining, storage), a hand-written Kafra service; a test that
reaches every job and learns every quest skill through the NPCs
(`tests/job_quests.rs`, on all-walkable maps); from mod0631: the Peco Peco
Breeder, the dye makers and the hairdresser, the Turtle Island chain, the
doomed swords and Alberta's doll child; the Wizard test on `job_wiz` from
817's unfinished script; the Priest test on `job_prist` from jAthena's of
2004-06-08; the Hunter test on `job_hunte` from eAthena 1.0's script, as
an iRO player's walkthrough of 2004-02-15 has it; the Knight test on
`job_knt` from jAthena's of 2004-09-12 (no account dates it earlier).
Next, in order:

* play through each 1st-job quest in the client (the test does not walk
  the real maps; `tools/client-test/play.sh` walks the client and reads
  the dialogue, `hunt.sh` fights). The training ground is played and
  817's script read against mod0631's (THREADS.md #6: mended where 817
  cannot be played; what still differs is listed there and asked in the
  queue's T10). Played since: the Thief test (interview, mushroom farm,
  job change), the Mage test (the Mixing Machine's number box), the Archer
  test (trunks, the bow and the arrows) and the Acolyte test (an ascetic
  and back). 817's Thief, Merchant and Acolyte scripts are read against
  mod0631's and mended where a player was stopped (THREADS.md #27: one
  Merchant applicant in four could not finish). The Swordman test is
  played too, from the guild to the job change (`play.sh walk` keeps off
  the course's pits with `AVOID`), and the Knight test from the
  application to the verdict. Swordman and Archer are read against
  jAthena's of 2004-03 (THREADS.md #27). Since 2026-11-13 the Merchant
  test (the Morroc delivery) and the Priest test on `job_prist` are played
  too (the Priest's last room only to a death: its door is in the test).
  Fix what breaks in `scripts/npc/` (one file per fix, listed in
  DEVIATIONS.md);
* the Assassin's and the Blacksmith's tests are in since 2026-11-13, from
  eAthena 1.0's scripts of 2004-11-04 (THREADS.md #8): every 2-1 job has
  its test. Both are played in the client to the job change (the
  Assassin's on 2026-11-13: quiz, targets, hide room, maze). No account
  dates either before
  autumn 2004 (queue T1): sharpen when one turns up;
* later, once this game is complete: its own version for euRO's May-2004
  client (owner, 2026-11-11), which has the 2-2 jobs' packets and text.
  What it needs is collected in `research/notes/may2004-client.md`;
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
(docs/WINDOWS.md); open items there under "Not verified yet". `check` walks the world on the client's own cells (`crates/ro-game/src/audit.rs`):
warps onto blocked cells, maps nothing leads to, NPCs out of reach, item
and monster ids the database lacks. Its first run found Ant Hell, Payon
Cave 4F and the Clock Tower cut off (fixed); what it still lists is
THREADS.md #26. A packet fuzz test sends every id of the length table with
plausible payloads and answers the server's prompts
(`crates/ro-game/tests/fuzz.rs`: no panic so far). Next: run `check` in CI
against a client folder; the Tip of the Day and other client
windows under Wine with the card instead of software drawing
(DEVELOPMENT.md).

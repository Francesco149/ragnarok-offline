# Research queue

For: the independent deep-research agent (Gemini in Antigravity) that wrote
rounds 1 to 3. Kept by the Claude sessions that work on the game: at the
end of each session the queue is updated (answered items go to "Done",
with where the answer went) and refilled with what that session could not
settle from the sources on disk. Last update: 2026-11-11 (after round 4).

## How to use it

1. Read `HANDOFF-r3.md` §2 first: findings are JSON rows, one fact each,
   checked by `research/check_rows.py`. That protocol stands, with the
   rules the reviews added (`research/notes/round3-review.md`,
   `round4-review.md`):
   * **A claim may only restate its quote.** If the number, the cell or
     the name is not in the quote, it is not in the claim. (Round 4 kept
     to this: no claim was false.)
   * **A computed result carries the command and its output as the quote**
     (a cell check with `research/gat_cells.py`, a `grep -c`, a `git log`).
   * **`source_date` of a file on disk is the file's own date** (`ls -l
     --time-style=long-iso`), or empty. Round 4 dated every mod0631 row
     2003-06-30 and every 817 row 2004-01-20; neither date is anywhere.
   * **Do not quote back what THREADS.md already says.** Round 4's rows
     for dyes and for the skill table repeated lines the game was built
     from. If a task has nothing beyond that, write one `none` row per
     place searched and move on.
2. Take tasks from the top; the order is by how much of the game waits on
   the answer. A round need not take them all. Say in `progress.md` which
   ones you took.
3. Write only inside `research/agy-threads/r<N>/` for round N (round 5 is
   next): `rows/<task id>.jsonl`, `pages/` for saved web pages, `check.txt`
   for your own checker run. No summary report: rows only, plus one line per
   task in `progress.md`: "answered" only if every bullet of the task has a
   row or a `none` row; otherwise "partly", naming the bullets left.
4. Do not edit anything else, and do not commit.
5. Sources that are not ours to use: leaked Gravity server files (Aegis).
   Later emulators and wikis (rAthena, Hercules, iRO Wiki, RateMyServer)
   may show *that* something existed; they are grade D for *when* and *how
   much*, and never the only source for a row graded above D.

What is on this machine is listed in `HANDOFF-r3.md` §3. Since then:

* `research/THREADS.md` has threads 21 to 25 and R6;
* the iRO guide's skill pages for all nineteen jobs, earliest captures
  (2004-03-27 to 2004-04-13), are in `data:refs/web/iro-guide-2004/`
  (`<capture>_job<xxx>skill.html`, with a `.txt` beside each). Do not
  fetch them again; cite them as `web` rows with the URL
  `http://web.archive.org/web/<capture>id_/http://iro.ragnarokonline.com/game/job<xxx>skill.asp`
  and a copy in your `pages/`;
* euRO's patch lists of 2004-02-21 and 2004-06-06 are in
  `data:refs/euro-patch-wayback/` and read in
  `research/notes/euro-history.md` §5.

## Queue

### T1. The 2-1 job tests as "New Comodo" brought them (threads 5, 8; carried over from rounds 3 and 4)

Still nothing dated. New since round 3:

* 817's Priest test is three rounds on `priest_1-1`, `priest_2-1`,
  `priest_3-1`; this client has one such map. Was the official test three
  rounds on one map, three different rooms, or something else? What were
  the rounds (the grail errand is 817's)?
* The Knight's, Wizard's, Hunter's, Assassin's and Blacksmith's tests on
  the maps this client has (`job_knight`, `job_wizard`, `job_hunter`, ...):
  any walkthrough written between 2003-12 and 2004-06, in any language
  (Korean and Japanese fan sites count: the test content was the same).
  Row per step: who, where, what is asked.
* The Peco Peco Breeder and the Falcon Breeder: place and fare on iRO or
  euRO (mod0631: `prontera 55,350` and `in_hunter 146,99`, 2500z each; 817's
  falcon breeder stands at `in_hunter 134,101`).

Sets: `scripts/npc/priest_test.lua`, a Wizard test laid out by us, the
breeders' cells.

### T2. euRO's own news pages of 2004 (threads 1, 15, 17, 19)

Round 3 found that Wayback's index lists 76 news pages and 19 planned-event
pages of euRO for 2004-03 to 2004-10, and read none
(`research/notes/round3-review.md`). Look for earlier ones too (2003-12 to
2004-02). Save each page, and make one row per dated statement about:
rates (closed-beta EXP and drops, thread 1), what each patch added
(Turtle Island, Lutie, New Comodo, Juno), War of Emperium (first siege,
schedule, rules stated, castles open), events, character slots, and the
closed beta's end and wipe.
Sets: the content defaults in `[game.content]` and the rates.

### T3. Ailments and their resistance (thread 21)

When does "VIT resists stun", "LUK resists curse", "MDEF resists freeze
and stone" first appear in a player guide or an official page, and with
what formula? mod0631 (2003-12) has fixed chances and no resistance; 817
(2004-02) has both. Also: Storm Gust freezing on the third hit; poison
taking HP down to a quarter; how long each ailment lasted.
The iRO guide's skill pages of March 2004 are read (THREADS.md #21): they
say which stat resists freeze, stone and poison and give no formula.
Start at: RODataZone, kRO notices, Japanese player wikis of 2003 (dated
pages only), anything dated before March 2004.
Sets: `skill_additional_effect` and the ailment part of
`skill_status_change_start`.

### T4. Turtle Island: who took you there, and the quests of its patch (thread 24)

The game now has mod0631's chain (`scripts/npc/turtle_island.lua`): an old
man in Alberta's inn, then the sailor at `alberta 247,122`, 10000z. Known:
iRO's notice of 2003-12-09 (round 4, T1-001) and euRO's patch list, which
has the island's files between 2004-01-09 and 2004-01-15.

* On iRO or euRO in early 2004: which NPC took a player to the island,
  from which cell, for what fare, and was there a condition? A player
  guide, a forum post, a map page; any language for the mechanics, but
  say which server.
* The notice promises "brand new headgears and new quests". Which
  headgear quests came with the island on the international servers
  (NPC, place, items)? Are they in 817 (`npc/quests/`) or mod0631?
* mod0631's logbook errands (five Bananas; a Red Gemstone): described
  anywhere for an official server?

Sets: the fare, and which hat quests belong to the default world.

### T5. Dyeing (thread 13; asked in round 4, not answered)

mod0631's chain is in the game and needs no more rows. Wanted, from
anything that is **not** mod0631's `npc_event_dye.txt`:

* Was hair dyeing available on iRO/euRO in 2003-12 to 2004-02, and on what
  terms? When did kRO add it (notice)?
* Clothes dyeing: available then? Materials, price, and which dye gave
  which of the four extra clothing palettes this client ships per job.
* The official English names of the five NPCs (817: Louitz, Aure Dupon,
  Morgenshtein, Dairenne, "JavaDullihan"; the two dyers in `prt_in`).

Start at: kRO notices of 2003; RODataZone and iRO guide pages on dyes;
Athena 659's dyer
(`data:refs/eathena-archive/x/athena0659TestEngmod/athena0659TestEngmod/conf/npc/other/npc_dye.txt`).

### T6. The 2-2 skills that need no missing packet: three witnesses, one table (threads 22, 25)

This client has no packets for spirit spheres, combos, Devotion or Auto
Spell (thread 25), so the first 2-2 skills to be built are the ones that
need none: Auto Guard, Defender, Reflect Shield, Shield Charge, Shield
Boomerang, Holy Cross, Grand Cross, Providence; the four Strips, Raid,
Back Stab, Intimidate; the four Chemical Protections, Potion Pitcher,
Acid Terror, Demonstration; Volcano, Deluge, Violent Gale, Land
Protector, Dispell, Magic Rod, Spell Breaker, the four weapon endows.
Round 4 made rows for fifteen iRO tables and ten emulator lines. Still
wanted, for each skill above:

* the client's text: a `grf` row from `data\skilldesctable.txt`, quoting
  the lines with numbers;
* eAthena 817 `src/map/skill.c` and `battle.c`: "implemented", "effect
  only", "commented out" or "absent", with the formula line quoted;
* the iRO guide's line or table, from the pages on disk (above).

Then say where the three disagree, one row per disagreement, with both
quotes in it. Not wanted: a merged answer.
Sets: `skill_castend_*` for those skills.

### T7. Things only drafted from the client's text (thread 22)

* Ice Wall: did it have HP that fell when players or monsters hit it, and
  could a player attack it by clicking it, in 2003? mod0631 lowers the
  wall's time by the damage (`skill.c:2753`) but refuses any attack target
  that is not a monster or a player (`pc.c:2479`).
* Poison React, Remove Trap, Spring Trap, Venom Splasher's delay, Talkie
  Box: a period description with numbers for each.
* Arrows: did they add ATK on the official servers of Dec 2003 (mod0631
  comments the line out)?
* The iRO guide's pages on disk have been read against all of these
  (THREADS.md #22, 2026-11-11): do not repeat that. Wanted is anything
  earlier than March 2004, and: when did Skid Trap go from sliding 1 to 5
  cells (client text, mod0631) to 6 to 10 (the guide)?

### T8. Small, independent

* **Swordman course monsters.** 817 puts Chonchons (id 1183) along the
  course; mod0631 puts Fabres (id 1184) on the same cells, under a map name
  with a typo. Which did a player of early 2004 meet? (Any walkthrough of
  the Swordman test.)
* **The Padded Armour.** 817's Moving HP Recovery knight asks for one;
  RODataZone's Swordman page (2004-05-02) does not list it; mod0631's
  knight says to come wearing one. Any other period page on that quest.
* **The other quest skills** (thread 7): Arrow Crafting, Charge Arrow,
  Cart Revolution, Change Cart, Crazy Uproar, Holy Light, Energy Coat: job
  level and items in 817 (`npc/skillquests/`), mod0631
  (`conf/npc_event_skillget.txt`) and RODataZone's job page each, side by
  side. The Swordman's three are done.
* **Shockwave Trap: one Trap or two?** The client's text says two, mod0631
  takes one, the iRO guide does not say.
* **The cursed swords, the doll child, Payon's arrow makers** (thread 24):
  is each of mod0631's three quests (`conf/npc_event_ma_sword.txt`,
  `npc_event_doll.txt`, `npc_event_arrow.txt`) described for iRO/euRO in
  2003/04?
* **euRO's exes of 2004-04-15 and 2004-05-31** (thread 25; patch list
  entries 444 and 462, `2004-04-15aRagexe.rgz`, `2004-05-31aRagexe.rgz`):
  is either archived anywhere (Wayback's captures of `wpatch.euro-ro.net`,
  mirrors, fan sites)?
* **Kafra fares outside jRO** (thread 3), **the start cell** (thread 6),
  **Lutie in January** (thread 19): still open, nothing new to add to the
  round-3 brief.
* **Monster skills** (thread 23): any period monster database that lists
  skills per monster (RODataZone, RO Empire, a Japanese one), for ten
  monsters of your choice across the level range, against
  `data/db/mob_skills.toml`.

## Done

Answered by earlier rounds and by the sessions that checked them; listed so
that nobody researches them again.

| Question | Answer | Where |
|---|---|---|
| 2-1 jobs in the closed beta | Job change at job level 40 with no test; quests came with New Comodo (2004-01) | THREADS.md #5, `research/notes/round2-review.md` |
| Kafra points | A tenth of each teleport fare (mod0631) | `scripts/npc/kafra.lua`, THREADS.md #4 |
| Thief and Novice quest skills | mod0631's NPCs, confirmed for the international servers by RODataZone (2003-12) | `scripts/npc/thief_skills.lua`, `novice_skills.lua` |
| Monster Tamers | mod0631 and the iRO pet guide | `scripts/npc/monster_tamers.lua` |
| Siege notices on kRO to 2004-06, 817's siege rules, the treasure boxes | Round 3 rows | `research/notes/round3-review.md` |
| Malaysia's sieges of spring 2004 | Castle takeover histories | `research/notes/woe-mro-2004.md` |
| The arena | kRO opened it 2004-10-12: not in our period | THREADS.md R5 |
| Hair colours of this client | 0 red, 1 yellow, 2 violet, 3 orange, 4 green, 5 blue, 6 white, 7 black, 8 red (read from the GRF's palettes, 2026-11-10) | THREADS.md #13 |
| Who makes dyes | mod0631's chain (Dec 2003) | `scripts/npc/dyes.lua` |
| Skill numbers where the client's text and the emulators part | The iRO guide of 2004-03/04 gives mod0631's number on every row; the text is older | THREADS.md R6 |
| Moving HP Recovery's items | 500 Empty Bottles and a Rocker Doll (RODataZone, 817's dialogue); 817's code took mod0631's | `scripts/npc/moving_hp.lua`, THREADS.md #7 |
| Fatal Blow, Auto Berserk items | The same in 817, mod0631 and on RODataZone | THREADS.md #7 |
| Turtle Island on iRO and euRO | iRO 2003-12-16 (notice of 2003-12-09); euRO's patch list has it by mid-January 2004 | THREADS.md #24 |
| The iRO guide's numbers for the 2-2 skills | All nineteen skill pages are on disk | `research/notes/round4-review.md` |

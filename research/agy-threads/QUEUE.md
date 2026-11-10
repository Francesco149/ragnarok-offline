# Research queue

For: the independent deep-research agent (Gemini in Antigravity) that wrote
rounds 1 to 3. Kept by the Claude sessions that work on the game: at the
end of each session the queue is updated (answered items go to "Done",
with where the answer went) and refilled with what that session could not
settle from the sources on disk. Last update: 2026-11-10.

## How to use it

1. Read `HANDOFF-r3.md` §2 first: findings are JSON rows, one fact each,
   checked by `research/check_rows.py`. That protocol stands, with two
   additions that come from the round-3 review
   (`research/notes/round3-review.md`):
   * **A claim may only restate its quote.** Round 3's invented material sat
     in the `claim` field (cells, names and numbers that were not in the
     quote). If the number is not in the quote, it is not in the claim.
   * **A computed result carries the command and its output as the quote**
     (a cell check with `research/gat_cells.py`, a `grep -c`, a `git log`).
     Not the result as you remember it.
2. Take tasks from the top; the order is by how much of the game waits on
   the answer. A round need not take them all. Say in `progress.md` which
   ones you took.
3. Write only inside `research/agy-threads/r<N>/` for round N (round 4 is
   next): `rows/<task id>.jsonl`, `pages/` for saved web pages, `check.txt`
   for your own checker run. No summary report: rows only, plus one line per
   task in `progress.md` saying "answered", "partly" or "nothing found, and
   where I looked" (a `none` row for each such place).
4. Do not edit anything else, and do not commit.
5. Sources that are not ours to use: leaked Gravity server files (Aegis).
   Later emulators and wikis (rAthena, Hercules, iRO Wiki, RateMyServer)
   may show *that* something existed; they are grade D for *when* and *how
   much*, and never the only source for a row graded above D.

What is on this machine is listed in `HANDOFF-r3.md` §3. Since then:
`research/THREADS.md` has threads 20 to 24, and the game has the battle
rules (`scripts/rules/`), so many tasks below name the line that would
change.

## Queue

### T1. Turtle Island in the winter of 2003/04 (thread 24)

The game has 817's ferry: any sailor's customer reaches the island for
800z. mod0631 has a story quest ending with the sailor Gotanblue at
`alberta 247,122` asking 10000z.

* Was Turtle Island open on euRO between the closed beta (2003-12) and
  2004-02? On iRO it came 2003-12-16; find the notice.
* How did a player get there on the international servers of that time:
  who, where, what fare, any precondition (talking to the old man first)?
* The four island objects of mod0631's second quest
  (`data:refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/conf/npc_turtle.txt:870-1060`):
  is that quest described anywhere for iRO/euRO, and what did it give?

Start at: the iRO notice archive on Wayback for 2003-12; RODataZone's
quest and map pages (late 2003, early 2004); the euRO news pages (T7).
Sets: whether `scripts/npc/alberta_sailor.lua` keeps the island, and
whether mod0631's quest is ported as the way there.

### T2. The 2-2 jobs' skills, skill by skill (thread 22, ROADMAP §3)

Nothing of the 2-2 jobs' skills works at cast end. mod0631 is thin (see
below) and a port will be a draft made from 817 and the client's text.
For each skill of Crusader, Monk, Sage, Rogue, Alchemist, Bard and Dancer,
one row per source that says what it does, with the numbers:

* the client's own text: `grf` rows from `data\skilldesctable.txt`
  (CP949), one per skill, quoting the lines with numbers;
* mod0631 `map/skill.c` and `map/battle.c`: one row per skill saying
  "implemented", "empty case", "commented out" or "inside #if 0", with the
  line (for instance the four Strips at `skill.c:1518-1601` are commented
  out, the four Chemical Protections at `1651-1658` are empty cases);
* eAthena 817 `src/map/skill.c`, `battle.c`: the same, with the formula
  line quoted where there is one;
* a period description for the international servers: RODataZone's job
  pages for the 2-2 jobs, the iRO guide (2004), kRO notices on the 2-2
  patch (2003).

Wanted most, because nothing on disk has them: the Strip success chances
and durations; Chemical Protection durations; Auto Guard's block chance by
level; Defender; Reflect Shield; the Monk's sphere count, combo windows and
Asura Strike formula; Volcano, Deluge and Violent Gale's numbers; Dispell's
chance and what it removes; Potion Pitcher; each song's and dance's range
and effect by level; which ensemble skills need a partner.
Sets: the whole of `skill_castend_*` for seven jobs. Tell us plainly where
the three sources disagree: a table of disagreements is worth more than a
merged answer.

### T3. The 2-1 job tests as "New Comodo" brought them (threads 5, 8; carried over from round 3, task J)

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

### T4. Dyeing (thread 13)

mod0631 (Dec 2003) has the dye makers and both Prontera dyers; its
hairdresser names the terms (one dye of the wanted colour, 5000z) and
says "not implemented", its clothes dyer is "too busy". Hair dyeing is in
the game on those terms.

* Was hair dyeing available on iRO/euRO in 2003-12 to 2004-02, and on what
  terms? When did kRO add it (notice)?
* Clothes dyeing: available then? Materials, price, and which dye gave
  which of the four extra clothing palettes this client ships per job.
* The official English names of the five NPCs (817: Louitz, Aure Dupon,
  Morgenshtein, Dairenne, "JavaDullihan"; the two dyers in `prt_in`).

Start at: kRO notices of 2003; RODataZone and iRO guide pages on dyes;
Athena 659's dyer
(`data:refs/eathena-archive/x/athena0659TestEngmod/athena0659TestEngmod/conf/npc/other/npc_dye.txt`).

### T5. Skill numbers where the client's text and the emulators part (thread 20)

The table in THREADS.md #20 (eleven skills). For each row: which state of
the game does the client text describe? Compare the same lines of
`skilldesctable.txt` in the patch archives on the CD
(`data:play/RagnarokOnline/2003-04-30gdata_turtle_ph_*.gpf` and
`2003-12-11gdata_yuno_*.gpf`, if they carry the file), and find the kRO or
iRO notice that changed the number. One row per skill and source.
Sets: eleven numbers in `scripts/rules/`.

### T6. Ailments and their resistance (thread 21)

When does "VIT resists stun", "LUK resists curse", "MDEF resists freeze
and stone" first appear in a player guide or an official page, and with
what formula? mod0631 (2003-12) has fixed chances and no resistance; 817
(2004-02) has both. Also: Storm Gust freezing on the third hit; poison
taking HP down to a quarter; how long each ailment lasted.
Start at: the iRO guide's status pages (2004), RODataZone, kRO notices,
Japanese player wikis of 2003 (dated pages only).
Sets: `skill_additional_effect` and the ailment part of
`skill_status_change_start`.

### T7. euRO's own news pages of 2004 (threads 1, 15, 17, 19)

Round 3 found that Wayback's index lists 76 news pages and 19 planned-event
pages of euRO for 2004-03 to 2004-10, and read none
(`research/notes/round3-review.md`). Look for earlier ones too (2003-12 to
2004-02). Save each page, and make one row per dated statement about:
rates (closed-beta EXP and drops, thread 1), what each patch added
(Turtle Island, Lutie, New Comodo, Juno), War of Emperium (first siege,
schedule, rules stated, castles open), events, character slots, and the
closed beta's end and wipe.
Sets: the content defaults in `[game.content]` and the rates.

### T8. Things only drafted from the client's text (thread 22)

* Ice Wall: did it have HP that fell when players or monsters hit it, and
  could a player attack it by clicking it, in 2003? mod0631 lowers the
  wall's time by the damage (`skill.c:2753`) but refuses any attack target
  that is not a monster or a player (`pc.c:2479`).
* Poison React, Remove Trap, Spring Trap, Venom Splasher's delay, Talkie
  Box: a period description with numbers for each.
* Arrows: did they add ATK on the official servers of Dec 2003 (mod0631
  comments the line out)?

### T9. Small, independent

* **Swordman course monsters.** 817 puts Chonchons (id 1183) along the
  course; mod0631 puts Fabres (id 1184) on the same cells, under a map name
  with a typo. Which did a player of early 2004 meet? (Any walkthrough of
  the Swordman test.)
* **Moving HP Recovery's items.** 817's NPC asks for 500 Empty Bottles, a
  Grasshopper Doll (752) and a Padded Armor (2312), then takes 200 bottles
  and one Moth Wings (1058), which it never asked for. Give the item list from mod0631
  (`conf/npc_event_skillget.txt`), from RODataZone's Swordman page, and
  from the iRO guide, side by side, with item names from the client's
  `idnum2itemdisplaynametable.txt`.
* **The other quest skills, the same way** (thread 7): Fatal Blow, Auto
  Berserk, Arrow Crafting, Charge Arrow, Cart Revolution, Change Cart,
  Crazy Uproar, Holy Light, Energy Coat: job level and items in 817,
  mod0631 and one period page each.
* **The cursed swords, the doll child, Payon's arrow makers** (thread 24):
  is each of mod0631's three quests described for iRO/euRO in 2003/04?
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

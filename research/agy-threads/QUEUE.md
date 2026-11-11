# Research queue

For: the independent deep-research agent (Gemini in Antigravity) that wrote
rounds 1 to 3. Kept by the Claude sessions that work on the game: at the
end of each session the queue is updated (answered items go to "Done",
with where the answer went) and refilled with what that session could not
settle from the sources on disk. Last update: 2026-11-11 (fifth session
of that day: the training ground and five of the six first-job tests
played in the real client, 817's scripts read against mod0631's; T10 is
new, and it comes third).

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
* **read maps from the merged GRF**, `data:play/RagnarokOnline/data.grf`,
  not from `data.grf.original`. Rounds 1 to 3 used the CD's base GRF and
  concluded that `job_wiz`, `job_knt`, `job_prist` and `job_hunte` were
  "later maps". They are in the CD's second Turtle Island patch file
  (`2003-04-30gdata_turtle_ph_2.gpf`), which the launcher merges and euRO
  applied in January 2004 (THREADS.md #8). `research/gat_cells.py` takes
  either file;
* euRO's patch lists of 2004-02-21 and 2004-06-06 are in
  `data:refs/euro-patch-wayback/` and read in
  `research/notes/euro-history.md` §5.

## Queue

### T1. The 2-1 job tests on the maps of the Turtle Island patch (threads 5, 8)

Changed on 2026-11-11: the maps are known now. `job_wiz`, `job_knt`,
`job_prist` and `job_hunte` are in the client we play; the Wizard test is
in the game on `job_wiz` from eAthena 817's unfinished script
(`scripts/npc/wizard_test.lua`). Wanted, dated 2003-12 to 2004-06, any
language (the test content was the same everywhere; say which server):

* **Knight** on `job_knt` and **Hunter** on `job_hunte`: a walkthrough.
  Row per step: who, where, what is asked. No period script exists
  (jAthena's Knight test is of 2004-09-12; 817's Hunter quest never
  enters `job_hunte`). mod0631 has four warp tiles on `job_knt`
  (`165,151`, `122,151`, `65,145`, `22,145`) and two on `job_hunte`
  (`89,139`, `89,68`), all "to 0,0": what were they?
* **Priest** (changed on 2026-11-11: 817's test turns out to be its
  author's own invention, and jAthena's `script/npc/job/npc_job_priest.txt`
  of 2004-06-08, commit `0f45810` in `data:refs/jathena`, is the official
  three-part test on `job_prist`; wanted now is a player's account of that
  test from before June 2004, to date it). 817's test runs on `priest_1-1` (the older `job_priest`, where
  its cells fit). mod0631 has two warps on `job_prist` (`97,104` and
  `24,109` to `168,17`). On which map was the test in early 2004, and what
  were its rounds (the grail errand is 817's)?
* **Wizard**, to sharpen what is in: how the waiting room worked (one
  candidate at a time out of an NPC chat room?); whether the Fire room's
  guards had one minute (jAthena, eAthena 0.5.2) or two (817); whether
  dying or logging out in a room cost anything; any of the examiner's
  questions quoted by a player (ours are 817's translation).
* **Assassin** and **Blacksmith**: on which maps, if any.
* The Peco Peco Breeder and the Falcon Breeder: place and fare on iRO or
  euRO (mod0631: `prontera 55,350` and `in_hunter 146,99`, 2500z each; 817's
  falcon breeder stands at `in_hunter 134,101`).

Sets: Knight and Hunter tests; `scripts/npc/priest_test.lua`,
`wizard_test.lua`; the breeders' cells.

### T2. A client that knows the 2-2 skills (thread 25)

New on 2026-11-11, and placed here because the seven 2-2 jobs wait on it.
Seen in the real client: `Ragexe.exe` of 2003-12-18 holds the Aegis names
of the skills up to the 2-1 jobs (`KN_BOWLINGBASH` is in it) and not one
`CR_`, `MO_`, `SA_`, `RG_`, `BA_`, `DC_` or `BD_` name. Its skill window
leaves out every 2-2 skill the server lists, and a cast shouts "Unknown
Skill !!". `data\skillnametable.txt` and `data\skilldesctable.txt` in the
merged GRF end with the 2-1 jobs as well. euRO replaced the exe twice
before the jobs opened there on 2004-06-09: patch list entries 444 and 462,
`2004-04-15aRagexe.rgz` and `2004-05-31aRagexe.rgz`
(`research/notes/euro-history.md` §5).

* Is either file archived anywhere: Wayback's captures under
  `wpatch.euro-ro.net` (list them with the CDX API and put the command and
  its output in the row), mirrors, fan sites, patch collections? One row
  per place searched, `none` rows included.
* Failing those: any international client exe dated 2004-03 to 2004-07
  (iRO, euRO, another English-language server) that is archived, with URL,
  size and date. Do not download anything over 50 MB; give the URL.
* The data files that went with the jobs on euRO: the patch lists on disk
  (`data:refs/euro-patch-wayback/`) name files, not their contents. Which
  entries from 444 up to the Juno block of the July list look like language
  or table data (`gdata_lang`, `gdata_scr` and the like), and is any of
  those `.gpf` files archived?

Sets: whether `jobs2_2` can be played at all, and with which client.

### T10. The novice training ground outside Japan (thread 6)

New on 2026-11-11, and placed here because every character starts with it.
The game runs eAthena 817's English script (`npc/jobs/novice.txt`,
2004-02-19), mended in six places from mod0631's Japanese one
(`conf/npc_job_newbie.txt`, 2003-12-18; `scripts/npc/novice_result.lua`,
DEVIATIONS). The two are ports of the same official script but not of the
same version of it, and nothing says which one a player of iRO or euRO
met. Wanted, dated 2003-10 to 2004-06, from an international server (say
which): a walkthrough, a guide page, forum posts or screenshots of the
training ground. Row per fact:

* **Leo Von Frisch's quiz** (`new_1-2 84,115`). 817 asks for "the wrong
  description of a stat" second and "one thing which has nothing to do with
  me" third (`novice.txt:409` and `:480`); mod0631 asks "which
  monster does not exist" (`npc_job_newbie.txt:610-647`) and "pick my
  name" (`:741-768`). Which questions, and what was each prize? (817: one
  Banana, one Banana, three Sweet Potatoes, one Apple, one Meat, one Milk;
  mod0631: nothing, five or three Bananas, ten Sweet Potatoes, three
  Apples, one Meat, one Milk.) How many Red Herbs at the end (817 always
  four; mod0631 four, two, one or none by the score, `:1155-1173`)?
* **The end** (`new_1-4 60,149`): what a player was given with the result
  (mod0631 `:2262-2284`: 100z to 500z and one to ten Red Herbs by points),
  and what "follow the result of the test" did. mod0631 sends you to a
  guide room of the suggested job where a stand-in hands over the guild's
  gift (`:2461-2645`: Swordman five Red Potions, Archer three and 50z,
  Thief a Leather Jacket, Mage 100z, Acolyte three Red Potions and a Club,
  Merchant 100z); 817 says "Under development". Any player's account of
  those rooms or gifts.
* **The Fabre forest** (`new_1-3`): where a knocked-out novice woke
  (mod0631: beside the Helper of `new_1-2 16,183`, who counts the tries
  and lets you give up at the fifth; 817: inside the forest).
* **The Usher's subsidy** (`new_1-2 100,29`): both scripts give 150z; 817's
  line says "50 Zeny" (`novice.txt:728`). Which did the screen say?
* **Starting gear**: a fresh character's equipment window. Both emulators
  create the Knife and the Cotton Shirt already worn (mod0631
  `char/char.c:408-415`), and so do we since today.
* Start with RODataZone's Novice page (the capture of 2003-12-15 that
  gave the quest-skill NPCs: `web.archive.org/web/20031215225409/http://www.rodatazone.com/jobs/0/novice.php`),
  the iRO guide's beginner pages, the euRO and Gamona forum captures listed
  in `research/notes/round2-review.md`.

Sets: `scripts/npc/novice_result.lua`; which of the differences listed in
THREADS.md #6 get mended, and which way.

### T3. euRO's own news pages of 2004 (threads 1, 15, 17, 19)

Round 3 found that Wayback's index lists 76 news pages and 19 planned-event
pages of euRO for 2004-03 to 2004-10, and read none
(`research/notes/round3-review.md`). Look for earlier ones too (2003-12 to
2004-02). Save each page, and make one row per dated statement about:
rates (closed-beta EXP and drops, thread 1), what each patch added
(Turtle Island, Lutie, New Comodo, Juno), War of Emperium (first siege,
schedule, rules stated, castles open), events, character slots, and the
closed beta's end and wipe.
Sets: the content defaults in `[game.content]` and the rates.

### T4. Ailments and their resistance (thread 21)

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

### T5. Turtle Island: who took you there, and the quests of its patch (thread 24)

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
* The smith of the doomed swords (`pay_fild08 218,283`, in the game from
  mod0631's `conf/npc_event_ma_sword.txt`): his three lists as an official
  server took them. mod0631 names five Cursed Rubies and ten Blades Lost in
  Darkness for Grimtooth and tests for one ruby; names one Young Twig for
  Mysteltain and takes a hundred. We take what he names.

Sets: the fare, and which hat quests belong to the default world.

### T6. Dyeing (thread 13; asked in round 4, not answered)

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

### T7. The 2-2 skills that need no missing packet: where the witnesses part (threads 22, 25)

Lower than it was: see T2. This client has no text for any 2-2 skill
(T2), so the witnesses are two: eAthena 817
(`data:refs/eathena-archive/x/athena-817-src/athena/map/skill.c`,
`battle.c`, `db/cast_db.txt`, `db/skill_require_db.txt`) and the iRO
guide's pages on disk. Drafted and in need of no rows: the Crusader's
Guard, Defender, Shield Reflect, Shield Charge, Shield Boomerang, Holy
Cross and Providence; the Sage's four endows and Dispell; the Rogue's four
Strips; the Alchemist's four Chemical Protections (DEVIATIONS.md says
which witness each number follows). Open on those, wanted from a third
source dated 2004 (a player guide, a forum post, a kRO notice; say which
server):

* Shield Reflect: ranged hits and skill damage too, or only close weapon
  hits? (The guide: "the full damage taken from an enemy".)
* The endows: did a failed endow destroy the weapon, as the guide says
  ("weapon is destroyed on failure")? Neither emulator lets one fail.
* The Strips: the guide says "DEX difference between the caster and the
  target affects on the success chance" and gives no number; 817 rolls
  5 + 2 x level and nothing else. Any period formula. And what a Strip did
  to a monster: the guide says a tenth of its attack or its DEF.
* Dispell: which effects it ended. 817 ends the ailments, Increase and
  Decrease AGI, Suffragium, Magnificat, Gloria, Apple of Idun, the endows
  and the Chemical Protections (`skill.c:2313`); the guide says "all
  magical effects".

Still to build, for each of which two rows are wanted (817: "implemented",
"effect only", "commented out" or "absent", with the formula line quoted;
the guide's line or table), and one row per disagreement with both quotes
in it: Grand Cross; Potion Pitcher, Acid Terror, Demonstration; Volcano,
Deluge, Violent Gale, Land Protector, Magic Rod, Spell Breaker. Not
wanted: a merged answer.
Sets: `skill_castend_*` for those skills.

### T8. Things only drafted from the client's text (thread 22)

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

### T9. Small, independent

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
* **The first-job tests' small sums** (thread 27, new on 2026-11-11).
  The Thief guild's pay: 817 gives 200z + 15 per Orange Net Mushroom + 2
  per Orange Gooey one (`npc/jobs/thief.txt:421`), mod0631 a flat 100z
  (`conf/npc_job_thief.txt:446`); did the mushrooms stay in the bag? The
  Acolyte's ascetics: 817 has Yosuke at `prt_fild00 208,218`, mod0631 at
  `208,255` (a blocked cell on this client's map). The Merchant's wrong
  parcel: item 1083 or 1091? One row each from RODataZone's job pages
  (late 2003) or a walkthrough, if they say.
* **Shockwave Trap: one Trap or two?** The client's text says two, mod0631
  takes one, the iRO guide does not say.
* **Alberta's doll child** (thread 24; in the game from mod0631's
  `conf/npc_event_doll.txt`): which doll brought what on iRO/euRO in
  2003/04? (The doomed swords are in T5. The arrow makers are closed:
  mod0631's file says they are its author's own.)
* **Two castle warps onto blocked cells**: `aldeg_cas05 188,49` to
  `165,288` and `prtg_cas02 184,44` to `157,140` are the same in mod0631,
  817, eAthena 0.5.2 and jAthena, and both targets are solid on this
  client's maps. Where did those two doors lead?
* **Places the world audit cannot reach** (thread 26, new on 2026-11-11):
  `tur_dun05` has monsters and two exits in mod0631 and no way in: was a
  fifth Turtle Island map in play in 2003/04, and how was it entered?
  `pay_dun04 193,43` (one of four random arrivals from Payon Cave 3F) is a
  wall on this client's map: do 817, eAthena 0.5.2 or jAthena have another
  cell? Four exits on `prt_in` at x 280 to 285 stand where this client's
  map has no room: which rooms of Prontera were they, and since when?
  Rows as computed results (`research/gat_cells.py` on the merged GRF,
  `grep -n` on each emulator's warp file).
* **The Clock Tower's random warps**: now in the game from mod0631
  (`conf/npc_warp.txt:1140-1320`, "import from EoR-162"). Any period
  player description of which warps on which floor were random.
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
| Are `job_wiz`, `job_knt`, `job_prist`, `job_hunte` in this client | Yes: the CD's second Turtle Island patch file, on euRO in January 2004 | THREADS.md #8 |
| The Wizard test | 817's unfinished script (2004-02-19), finished from eAthena 0.5.2 and jAthena | `scripts/npc/wizard_test.lua` |
| mod0631's cursed swords and doll child | In the game | `scripts/npc/cursed_swords.lua`, `doll_child.lua` |
| mod0631's arrow makers in Payon | Not jRO: the file's header says its author made them up until Arrow Crafting worked | THREADS.md #24 |
| Can the Swordman course be walked on this client's map | Yes, each stretch and every pit's cave | `research/course_check.py` |
| Guard's chances, Defender's percentages, Shield Boomerang's reach | 817 and the iRO guide agree | THREADS.md #25, DEVIATIONS |
| Does this client ship the 2-2 skills' text | No: neither names nor descriptions, and the exe knows none of the skills (seen in the client) | THREADS.md #25 |
| How a player gets into Ant Hell, Payon Cave 4F and the upper Clock Tower | Script warp tiles in mod0631's `npc_warp.txt`, which the importer skipped until 2026-11-11 | `scripts/import/warps.lua`, PROVENANCE.md |
| Why Clock Tower 1F could not be walked | Its cells have a stray type (`0x444c74`); only 1 and 5 block, as in Athena | DEVIATIONS.md "Cell types" |
| Does the training ground play through on this client's maps | Yes, walked in the real client from the bridge to the job guide room; 817's script against mod0631's is read NPC by NPC | THREADS.md #6, `scripts/npc/novice_result.lua` |
| The Priest test of 2004 | jAthena's of 2004-06-08 (pilgrimage, exorcism on `job_prist`, vow) is in the game; T1 asks to date it earlier | `scripts/npc/priest_test.lua`, THREADS.md #8 |
| Can the Merchant test be finished at each of its four places | Not in 817 (Byalan's Kafra never took a parcel; Kay could not change one); mended from mod0631, each place tested | `scripts/npc/merchant_delivery.lua`, THREADS.md #27 |
| The Mage test's recipes | In 817's Book Shelf (`npc/other/books.txt`), the same numbers as jAthena's of 2004-03-02; played in the client | THREADS.md #27 |
| Is a new character's Knife worn | Yes in mod0631 and 817 (`char/char.c`); now here too | `crates/ro-game/src/game.rs` |
| The Thief test in the client | Interview, mushroom farm, weighing and job change work on the real maps | ROADMAP §4 |
| Providence, the endows, Dispell, the Strips, Chemical Protection: the numbers | Drafted from mod0631, 817 and the iRO guide; what is still open is in T7 | DEVIATIONS.md, `scripts/rules/skill_effects.lua` |

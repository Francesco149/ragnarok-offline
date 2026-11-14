# Research queue

For: the independent deep-research agent (Gemini in Antigravity) that wrote
rounds 1 to 3. Kept by the Claude sessions that work on the game: at the
end of each session the queue is updated (answered items go to "Done",
with where the answer went) and refilled with what that session could not
settle from the sources on disk. Last update: 2026-11-14, later (thread 28 built from the client: T9's ground-skill item widened); before that, round 10 reviewed (`research/notes/round10-review.md`: T8's Skid Trap item and two T9 items moved to Done, T8 asks when euRO got iRO's Comodo skill patch, thread 28 narrowed; the boss rule built); before that, later (T9 monster skills: the FAQ pairs added, the rate question); before that, round 9 reviewed (`research/notes/round9-review.md`: two T9 items moved to Done, the monster-skill item rewritten); before that, later the same day (thread 28 added to T9); before that, round 8 reviewed (`research/notes/round8-review.md`); before that, later the same day (T7 narrowed by euRO's May-2004 text; two shop names added to T9). Before that, 2026-11-14 (round 7 reviewed: `research/notes/round7-review.md`.
T11 answered and moved to Done; T1, T9 and T10 updated with what it found).

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
   * **Rows from a file that is not a period source are grade D**: a
     later client's tables (`euRO_quests.txt` is a quest log of the 2010s),
     our own extractions. A GameFAQs guide is dated by the version-history
     line that added the section you quote: put that line in `reasoning`.
   * **Before searching the web for a file, look on `data:`** and in
     `docs/PRESERVATION.md`: round 5's best find (T2) was on our own disk.
   * **Date a FAQ's section, not the FAQ.** Round 7 read Weggy's simple
     2-1 job changes as iRO in May 2004. They are text from before his
     year-long break (about mid-2003), and his own v6.1 says the 2-1
     quests were added since. Version numbers and dates go in
     `reasoning` only as the page gives them (round 7 wrote "v3.3
     (2003-11-14)" for a page that says 3.2 and gives no date).
   * **Do not quote back what THREADS.md already says.** Round 4's rows
     for dyes and for the skill table repeated lines the game was built
     from. If a task has nothing beyond that, write one `none` row per
     place searched and move on.
2. Take tasks from the top; the order is by how much of the game waits on
   the answer. A round need not take them all. Say in `progress.md` which
   ones you took.
3. Write only inside `research/agy-threads/r<N>/` for round N (round 9 is
   next): `rows/<task id>.jsonl`, `pages/` for saved web pages, `check.txt`
   for your own checker run. No summary report: rows only, plus one line per
   task in `progress.md`: "answered" only if every bullet of the task has a
   row or a `none` row; otherwise "partly", naming the bullets left.
4. Do not edit anything else, and do not commit.
5. Sources that are not ours to use: leaked Gravity server files (Aegis).
   Later emulators and wikis (rAthena, Hercules, iRO Wiki, RateMyServer)
   may show *that* something existed; they are grade D for *when* and *how
   much*, and never the only source for a row graded above D.
   **Owner, 2026-11-14:** where period sources cannot settle a question,
   later sources are wanted as the best approximation: early knowledge can
   be incomplete. So when a task's period search comes back empty, add the
   later answer as grade D rows (say which source and its date) instead of
   stopping at `none`.

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

### T1. The 2-1 job tests: dating and sharpening them (threads 5, 8)

**After round 9:** do not quote eAthena 1.0's or jAthena's test scripts
again. THREADS #8 and the headers of `scripts/npc/*_test.lua` already
hold everything in them, and round 9's 23 T1 rows only repeated it. What
is wanted is a *player's* account of 2003-04 (a guide, a forum post)
saying what the tests asked.

Changed on 2026-11-13: every 2-1 job now has its test in the game. The
Assassin's (`scripts/npc/assassin_test.lua`) and the Blacksmith's
(`scripts/npc/blacksmith_test.lua`) came from eAthena 1.0's scripts of
2004-11-04 (`data:refs/rathena` commit `706e78509`,
`npc/jobs/2-1/assassin.txt` and `blacksmith.txt`), the Knight's from
jAthena's of 2004-09-12. None of the three is dated by a player before
autumn 2004. Wanted, dated 2003-12 to 2004-06, any language (say which
server):

* **Knight** on `job_knt`: in the game since 2026-11-11 (seventh
  session) from jAthena's `npc_job_07knight.txt` (2004-09-12, commit
  `6332410`), with eAthena 1.0's English names. Wanted: a player's
  account from before September 2004, iRO or euRO above all, that says
  (a) whether the test existed yet (Edge's guide, GameFAQs 29457, says pRO
  had none in February 2004: "Pay a small fee"); (b) Sir Andrew's items
  (jAthena names three lists and fills one: Bug Leg, Heart of Mermaid,
  Maneater Blossom, Clam Flesh, Old Frying Pan, Snail's Shell, five each;
  eAthena adds Elder Pixie's Moustache, Wing of Red Bat, Orcish Voucher,
  Moth Dust, Reptile Tongue, Mane); (c) Sir James's questions (the two
  scripts differ on three of eight: Bowling Bash's and Brandish Spear's
  prerequisites and the spear for a Nightmare, against "not an attack
  skill", "most SP" and "strongest against Undead"); (d) whether Sir Grey
  could fail you, and on what; (e) the arena's time (three minutes a
  stage?) and Sir Edmond's (four minutes or four and a half?).
* **Blacksmith**: a walkthrough from before June 2004 (round 7 found
  pRO's of 2004-08-31, GameFAQs 28512, which follows eAthena 1.0's outline
  and gives no answers or items). The two scripts
  on disk (eAthena 1.0, and jAthena //1046's unfinished
  `script/npc/job/npc_job_10blacksmith.txt`) part on: (a) the pass marks
  (eAthena: Geshupenschte 100, 90 on a retry, Mitmayer 80; jAthena: 90, 80,
  and 100); (b) the eighth question (a level 3 weapon's safe refine:
  jAthena counts +6, eAthena +5); (c) the Arbalest order (Trunk or Eye
  Patch); (d) where Tilpitz stood in the old Payon (jAthena `151,39`).
  Which customers and orders did a player of 2004 meet?
  research/THREADS.md #8 lists them.
* **Assassin**: a walkthrough from before June 2004: the Nameless One's
  quiz (eAthena has three sets of ten and wants nine right), Barcardi's
  two rooms (six "Job Change Target"s among decoys and traps; then a room
  of Mummies and Hydras to cross without a kill; three minutes each),
  the Guild Master's maze, and which dagger or katar he gave. Was the
  guild's door on `moc_fild16` into the entrance hall (eAthena 1.0) or
  into Kai's room (mod0631's and 817's warp lists)?
* **Hunter, to sharpen**: the walkthrough's Master Hunter "at the central
  palace" of Payon (this client has no `payon_in03`: which map was it on
  iRO in early 2004?); any other item list of the Demon Hunter's (the
  walkthrough found three); what the interview's right answers were.
* **Priest**: round 7 found Weggy's (iRO, 2004-06-07): questions in
  the antechamber, a hall of temptations, then rooms of undead "You can
  actually just run past". jAthena's script, which we play, makes
  thirteen undead fall before the gate opens and puts the tempters after
  them. Wanted: another account of 2004 that says whether the undead had
  to be killed, in which order the parts came, and whether a pilgrimage
  to the three ascetics came first.
* **Wizard**: one minute or two for the Fire room's guards (jAthena and
  eAthena 0.5.2 against 817); whether dying or logging out in a room cost anything; more
  of the examiner's questions quoted by a player (round 5 found one, in a
  guide of 2004-07-30).

Sets: the details of the Knight, Assassin, Blacksmith and Hunter tests,
and whether each belongs in the default world of early 2004.

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

Round 5 found one player's account (Weggy's FAQ: "100 zeny, or 200 zeny
if the final few NPCs guessed your class correctly"; skipping the
training "whisks you away to a random town"), and nothing on Leo's
questions. Its other rows quoted eAthena 0.5.2's `novice.txt`, which is
817's script two months on: not wanted again.

Round 7 found nothing beyond round 5's Weggy rows. Weggy is used up for
this: look elsewhere (RODataZone's Novice page, forum captures, other
GameFAQs guides with a Novice section).

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
Round 6 read about fifteen of them (saved in `r6/pages/`); nearly all of
what they say was already in `research/notes/euro-history.md`. Wanted
now: only pages dated 2003-12 to 2004-03 (closed beta and open beta), and
any page that says what a patch changed in the game (not when).
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
Round 6 found jAthena's first resistance (//0729, 2004-01-21; //0736 takes
Stone Curse's time from "マジスレテンプレ", the 2channel mage thread's
template): THREADS.md #21. Wanted now: that template itself, or any 2003
copy of it (2ch archives, mirrors), with its numbers.
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

Round 6 quoted mod0631's `npc_turtle.txt` and 817's `headgearquest.txt`,
both already in the game, and a later client's quest log: nothing from an
international server of 2004. Only that is wanted.
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

* Shield Reflect: answered by the May-2004 client's text (T11): "against
  short-distance attack ... Reflect only physical attack".
* The endows: answered by the May-2004 client's text ("if it's failed the
  weapon'll be broken", round 7).
* The Strips: the guide says "DEX difference between the caster and the
  target affects on the success chance" and gives no number; 817 rolls
  5 + 2 x level and nothing else; euRO's May text says the chance depends
  on the level and the caster's DEX. Any period formula. What a Strip did
  to a monster is answered for three of them by euRO's text (DEF -15%,
  VIT -40%, INT -40%; in the game since 2026-11-14). Strip Weapon: "ATK
  will be reduced", no number; the guide says a tenth. Any other number
  dated before 2004-07.
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
  earlier than March 2004.
* **iRO's Comodo "skill balance patch" on euRO** (round 10; THREADS.md
  #22). Forever-Zero's guide (GameFAQs 27633, its "Future Update Changes")
  copies iRO's list: Skid Trap 6 to 10 cells and 10 SP, Ankle Snare by AGI
  and 12 SP, Double Strafe 12 SP, Arrow Shower 5x5, the trap rules. euRO's
  May-2004 text has part of it. Did it reach euRO with "New Comodo"
  (2004-01-08/18), with open beta (2004-02-04), or later? A euRO notice,
  forum post or player guide that names any of those changes with a date;
  and the date iRO's Comodo update went live.
* Sightrasher (drafted 2026-11-13 from the guide's Wizard page, 817 and
  jAthena): which cells did it hit, only the eight lines from the caster
  or every cell within reach, and how far at each level? The guide's
  "Range +1 cells" column (levels 1-2: 1, ... 9-10: 5) is read as the
  reach; jAthena's pattern reaches 5 cells at every level. A period
  description or screenshot settles it.
* Frost Nova: what did the official client show when it was cast on a
  target (this client draws nothing for 011a or 0117 of skill 88)? A
  period screenshot, video or player's description of its animation.

### T9. Small, independent

* **Three quest-skill differences** (thread 7, from round 8): Cart
  Revolution's Iron (817 15, RODataZone 20), Gershaun's Banana Juice, and
  a Crossbow for Charge Arrow (RODataZone only). Any other account of 2004
  (a guide, a forum post) that gives these quests' items; failing that,
  what later sources (iRO Wiki, rAthena's official conversions) give.
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
* **A castle warp onto a blocked cell**: `prtg_cas02 184,44` to `157,140`
  is the same in mod0631, 817, eAthena 0.5.2, jAthena and rAthena, and
  the target is solid on this client's map. Where did that door lead?
  (`aldeg_cas05`'s is settled: round 10.)
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
* **"Unknown Skill !!" over monsters** (thread 23, new on 2026-11-11):
  this client shouts a skill's name over whoever uses it, and has no name
  for the monster-only skills, so a goblin's wind attack (`NPC_WINDATTACK`)
  or an Ant Egg hatching shows "Unknown Skill !!". Did players of
  2003/04 see that on the official servers (a forum thread, a screenshot,
  a guide that mentions it), or nothing at all?
* **Monster skills, levels and rates** (thread 23, from round 9).
  unknown13's FAQ (round 9) says which monster had which skill on iRO in
  April 2004, but not at what level, how often or when. Any period source
  that does, for the skills mod0631 gives no monster (Endure, Increase and
  Decrease Agility, Piercing Attack, Mental Breaker, Napalm Beat, Status
  Recovery, Pneuma); failing that, a later table's rows for the same
  monsters, graded D. **Since 2026-11-14** the 93 pairs are in the game
  (`data/db/mob_skills_faq.toml`): 47 of them on a guessed row (column
  `tier = "guess"`), so a source for *those* monsters comes first. And
  **how often**: the period emulators disagree tenfold (mod0631 rolls a
  row's rate against 1000, 817 and jAthena from 2004-02-20 against 10000,
  same numbers; THREADS #23). A player's account of how often a monster
  cast (how many Fire Bolts from a Marduk in a fight, how soon a Marc
  went to Increase Agility) is worth a row, any server, dated.
* **Others' ground skills, seen twice** (thread 28). Round 10 settled the
  traps: they were hidden from anyone who walked away and back (FAQ 27633).
  Still wanted: did a monster's or another player's Fire Wall, Safety
  Wall, Pneuma or Warp Portal ever vanish after you walked away and back,
  or stay? A forum thread or guide line either way; one `none` row per
  place searched if nothing. Do not quote FAQ 27633 again. (2026-11-14:
  the client hides any such unit on its second showing, so the game now
  sends only traps twice; THREADS #28.) Also wanted: did a player who
  *arrived* after a trap was set see it? "Invisible only to people who
  were off screen when it was planted" says no; any other account of it.

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
| When Skid Trap went from 1-5 cells to 6-10 | iRO's Comodo update (Episode 3.0) skill balance patch, quoted whole in GameFAQs 27633 | THREADS.md #22, `research/notes/round10-review.md` |
| Were others' traps hidden | Yes: from anyone off screen when it was set, or who walked away and back (GameFAQs 27633) | THREADS.md #28 |
| `aldeg_cas05 188,49`'s door | A slip: `165,228` (rAthena; the castle's own pairs agree) | `scripts/npc/castle_warps.lua` |
| Forever-Zero's Archer/Hunter guide (GameFAQs 27633) | Read whole in rounds 5 and 10: Hunter test, skill notes, the Comodo patch list. Don't quote it again | `research/notes/round10-review.md` |
| Does this client ship the 2-2 skills' text | No: neither names nor descriptions, and the exe knows none of the skills (seen in the client) | THREADS.md #25 |
| How a player gets into Ant Hell, Payon Cave 4F and the upper Clock Tower | Script warp tiles in mod0631's `npc_warp.txt`, which the importer skipped until 2026-11-11 | `scripts/import/warps.lua`, PROVENANCE.md |
| Why Clock Tower 1F could not be walked | Its cells have a stray type (`0x444c74`); only 1 and 5 block, as in Athena | DEVIATIONS.md "Cell types" |
| Does the training ground play through on this client's maps | Yes, walked in the real client from the bridge to the job guide room; 817's script against mod0631's is read NPC by NPC | THREADS.md #6, `scripts/npc/novice_result.lua` |
| The Priest test of 2004 | jAthena's of 2004-06-08 (pilgrimage, exorcism on `job_prist`, vow) is in the game; T1 asks to date it earlier | `scripts/npc/priest_test.lua`, THREADS.md #8 |
| The Assassin's and the Blacksmith's tests | eAthena 1.0's scripts (2004-11-04), the only complete ones on disk; in the game, played in the client; T1 asks to date them | `scripts/npc/assassin_test.lua`, `blacksmith_test.lua`, THREADS.md #8 |
| A client that knows the 2-2 skills | euRO's `Ragexe` of 2004-05-07 and its language patch, on the owner's MICO07 DVD (ripped 2026-11-09); found by round 5 | THREADS.md #25, `research/notes/round5-6-review.md` |
| The Hunter test of early 2004 | iRO walkthrough of 2004-02-15 (round 5) and eAthena 1.0's script; in the game | `scripts/npc/hunter_test.lua`, THREADS.md #8 |
| jAthena's first stat resistance | //0729, 2004-01-21 (round 6) | THREADS.md #21 |
| euRO's news of 2004 | Round 6: nearly all already in `euro-history.md` | `research/agy-threads/r6/` |
| Can the Merchant test be finished at each of its four places | Not in 817 (Byalan's Kafra never took a parcel; Kay could not change one); mended from mod0631, each place tested | `scripts/npc/merchant_delivery.lua`, THREADS.md #27 |
| The Mage test's recipes | In 817's Book Shelf (`npc/other/books.txt`), the same numbers as jAthena's of 2004-03-02; played in the client | THREADS.md #27 |
| Is a new character's Knife worn | Yes in mod0631 and 817 (`char/char.c`); now here too | `crates/ro-game/src/game.rs` |
| The Thief test in the client | Interview, mushroom farm, weighing and job change work on the real maps | ROADMAP §4 |
| Providence, the endows, Dispell, the Strips, Chemical Protection: the numbers | Drafted from mod0631, 817 and the iRO guide; what is still open is in T7 | DEVIATIONS.md, `scripts/rules/skill_effects.lua` |
| Swordman course monsters | Both: "aggressive Fabres and Chonchons" in the pits (Weggy, iRO, May 2004; round 7) | THREADS.md #7 |
| The Padded Armour for Moving HP Recovery | Asked for, and kept (vgmaster88, iRO, autumn 2003; round 7) | THREADS.md #7 |
| The two bytes the May-2004 unit packets gained | The base level (817 `map/clif.c:544`; round 7) | `research/notes/may2004-client.md` |
| The May-2004 client's text for the drafted 2-2 skills | Agrees on endows, Dispell, Chemical Protection; differs on the Strips (round 7) | THREADS.md #25, `research/notes/round7-review.md` |
| The May-2004 data.grf against ours | Sorted (round 7); totals as in the notes | `research/notes/may2004-client.md` |
| What the Strips do to a monster | euRO's May-2004 text: DEF -15%, VIT -40%, INT -40%; jAthena of 2004-05-07 agrees on two; Strip Weapon still the guide's 10% | THREADS.md #25, `scripts/rules/battle.lua` |
| 0x01f1 to 0x01ff | Named from OpenKore's later table, lengths agree with the May exe (round 8) | `research/notes/may2004-client.md` |
| The other quest skills, side by side | Five agree everywhere; three differences (round 8) | THREADS.md #7, `research/notes/round8-review.md` |
| Sir Grey's pass condition | No period source; rAthena's official conversion scores 0, 5 or 10 to pass (later, round 8 review) | THREADS.md #8 |
| Shop names 817 lacked | 14 of 16 from 817's English for the same Japanese role words (2026-11-14) | THREADS.md #11 |
| The two shops 817 never names | No period English; later: Vegetable Gardener, Mr. King's Shop (rAthena) | THREADS.md #11, `research/notes/round9-review.md` |
| Which monster had which skill, iRO April 2004 | unknown13's Monster Skill FAQ: 96 pairs mod0631 lacks | THREADS.md #23, `research/mob_skills_faq.py` |

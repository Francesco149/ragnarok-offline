# Research threads

A running list of open questions whose answers would make the reconstruction
more exact. Each one says what is uncertain, what depends on it, and where
to look. Add new ones at the end, keep the numbers stable (docs refer to
them), and move a thread to "Resolved" with its answer and source when done.

Tags: **impact** = what changes if we learn the answer. **look** = where to
dig first. Background research: `research/notes/` (euro-history.md,
mechanics-sources.md, emulators.md).

Research rounds: round 1 (Antigravity, then a Claude pass to confirm) is
briefed in `research/agy-threads/HANDOFF.md`; its report is
`research/agy-threads/REPORT-antigravity.md`. **Read it together with
`research/notes/round1-review.md`**, the second pass: it says which claims
were loaded again from the source, which turned out wrong, and which are
still only the report's word. Below, "round 1" marks what that pass
confirmed. Round 2 is briefed in `research/agy-threads/HANDOFF-r2.md`: the
same questions with a stricter method (one graded, quoted row per claim)
and the local period sources first, above all mod0631's own NPC scripts.
Its output is `research/agy-threads/r2/`; **read
`research/notes/round2-review.md` first**: the summary report there invents
quotes and references, the sub-agents' notes under it mostly hold. Below,
"round 2" marks what that review confirmed from the source. Round 3 is
briefed in `research/agy-threads/HANDOFF-r3.md`: War of Emperium in depth
first, and findings only as rows that `research/check_rows.py` can check
against the cited file, revision or saved page.

What the outside agent is asked next is in
`research/agy-threads/QUEUE.md`: a standing list, updated and refilled at
the end of each working session.

Ruled out as a source: leaked Gravity server packages (Aegis binaries and
scripts). They are not ours to use or redistribute, and nothing here may be
derived from them.

## Open

### 1. euRO closed-beta rates
"As we are testing with higher experience…" (euRO forum, 2003-12). The
factor is unknown, as are open-beta and commercial rates (presumably 1x).
**Impact:** `base_exp_rate`/`job_exp_rate`/`drop_rate` defaults, and a
"closed beta" preset.
**Look:** forum.euro.bhi-oe.de topics t=67, t=80, t=16 (no Wayback
captures). The thousands of 2004 `www.euro-ro.net/community/forum`
captures. German fan sites rns.gamona.de and ragnarokonline.de (2003-12 to
2004-02).
**Round 1 (report only, not re-checked):** no number was ever published;
a Gamona staff post of 2004-01-27 calls the closed beta's "EXP und Drop
Rate zu hoch" (`ragnarok.gamona.de/forum/index.php?act=ST&f=31&t=9857`,
capture 20040213093633); players reached level 64 in four days. Next:
load those captures, then IRC logs of `#bhi-oe`.
**Round 2:** the captures were loaded. A player in that Gamona thread,
2004-01-27: the rate will not stay "wie sie in der closed war (nämlich
3fach)". Nobody from BHI-OE confirms the 3x. It fits what the euRO forum
shows: an Assassin three days after the start (2003-12-23), a Knight at
64/34 within a week. The default stays 1x; DEVIATIONS names the 3x. Still
unarchived: forum topics t=67 and t=80.

### 2. Official English NPC dialogue, iRO/euRO 2003–04
Our dialogue is the emulator authors' (817), often a fan translation or
paraphrase. euRO used Gravity's English text, very likely iRO's.
**Impact:** every imported NPC's wording, job quests first.
**Look:**
* GameFAQs RO guides from 2003–04: job-change walkthroughs quote NPCs.
* iRO forum and fan-site captures on Wayback (ro.doddlercon.com,
  roempire.com guides).
* OpenKore 2004 `tables/` and logs (npc names and positions).
* rAthena `npc/pre-re` (official transcriptions, but ~2006+: tier later).
  Use it only where the period text is clearly a paraphrase, and mark it.

**Round 2:** nothing found. The round's text table is the client's own
`pettalktable.xml` and `msgstringtable.txt` plus eight bug-board lines whose
captures do not load; its quoted NPC lines are not on the pages it cites.
What it did find: RODataZone's job pages (2003-10 to 2004-02 on Wayback)
give NPC names as the international servers showed them (Alcouskou, Nami,
Leon von Frich, Necko...), though no dialogue. GameFAQs was blocked for the
agent (HTTP 403); its 2003–04 guides are still the best lead for dialogue
and need a browser.

### 3. Kafra fares before 2004-02-24 on iRO and euRO
Round 1 settled the emulator side (see R4): mod0631 and jAthena agree with
817's menus, and add per-place storage fees and the Comodo routes. Both
follow jRO. What is still unattested is the fare table of **iRO and euRO
themselves** in Dec 2003 – Feb 2004, and when euRO took kRO's new table of
2004-02-24.
**Impact:** `scripts/npc/kafra.lua`.
**Look:** iRO notices on Wayback (`allim/content.asp?bbs=tNotice`),
screenshots of a Kafra menu in period guides and fan sites, euRO patch
notes of spring 2004. The kRO notice of 2004-02-24 lists only the new
fares.

### 4. Kafra points and the Kafra Pass in Dec 2003
817 had a Kafra-points "Sweet Potato" exchange (broken) and a Kafra Pass
seller in the Al De Baran HQ. kRO removed the *voucher* menu on 2003-05-20.
**Round 1:** the points outlived the voucher. kRO's notice of 2004-02-10:
"the NPC that rewards Kafra points has been improved; take your saved-up
points to the Al De Baran headquarters". jAthena (2004-01-31) adds a tenth
of each teleport fare to `KF_PIT` and has a "check Kafra points" menu
entry. (The round-1 report says the opposite; it is wrong here.)
**Round 2, done:** mod0631 has the rule (our brief said it had none; it has
131 uses of `KF_PIT`): a tenth of every teleport fare, nothing for storage,
and a "check points" entry that calls it a mileage programme redeemable at
the head office. The only prize list in a period file is 817's counter at
`aldeba_in 79,161`: Sweet Potatoes for 100–900p, Red Potions for
1100–2800p, a lottery at 1000p and 3000p. `kafra.lua` now earns the points
and the counter hands out the potatoes and potions.
**Open:** the lottery prizes (817 names none); the "other windows" 817's
counter mentions; what kRO "improved" on 2004-02-10; whether iRO/euRO had
points at all. And the voucher: mod0631 and 817 both still sell the Kafra
Pass (2000z, one conversation of free services) months after kRO dropped
its menu. We follow kRO: no pass.
**Impact:** the Aldebaran Kafra HQ NPCs; "Other Information Check".
**Look:** jAthena's Al De Baran scripts of 2004 (`KF_PIT`), kRO notices of
2003, iRO guide pages (2004).

### 5. Job-change quests on euRO before "New Comodo"
euRO's "New Comodo" update (Jan 2004) lists "2nd-job quests" as new. So in
the closed beta (Dec 2003), how did 2-1 job changes work? Instant at job
level 40, or not at all? iRO added the Knight/Assassin/Hunter quests with
Turtle Island (2003-12-16).
**Impact:** a "closed beta" content state; whether 2-1 quests are on by
default.
**Look:** euRO news 2004-01-08/18 (forum t=1791), the euRO closed-beta
forum, iRO 2003-12 notices.
**Clue:** Athena 659 (2004-01-01) has only simple 2-1 guild NPCs ("Change
a Job into a Knight", job level 40, no test). 817 (2004-02-19) adds the
Priest and Hunter quests. The emulator authors followed the official
servers, so this agrees with euRO's "quests came with New Comodo".
**Round 2: settled for the closed beta.** The New Comodo list was loaded
(Gamona, 2004-01-18, quoting euro.bhi-oe.de): "2. added 2nd job quest",
"3. added monster skills", "6. adjusted monsters HP / Exp". The euRO forum
had an Assassin on 2003-12-23 and a Knight by 2003-12-27, before that
patch; mod0631 changes job at job level 40 with no test. So December's
closed beta had 2-1 jobs without quests. Our baseline is New Comodo, so
the open part is #8: which quests, on which maps.

### 6. The start cell and the novice training ground on this client
The client has `new_zone01-04` under older names; 817's `new_1-1..4`
scripts fit them (30 of 32 cells). Two NPCs stand on blocked cells: the
Bulletin Board at `new_1-1 66,114` and a Helper at `new_1-4 100,29`. In the
real client (Windows, 2026-11-09) the gate Guard at `117,107` is drawn
behind the castle tower: its cell is walkable but the older map's walls
sit differently.

Our start cell `new_zone01 53,111` is Athena's. Every Athena config of
2003–04 has `start_point: new_1-1.gat,53,111`. The Athena changelog dates
it: `//0305` moved new characters to the training ground, `//0307` made the
cell configurable. Where 53,111 itself came from is unknown. From there a new
character crosses a long bridge east to the castle.

**Round 1:** the map half is settled. The client redirects `new_1-1` to
`new_zone01` itself (R2), so a server of the time that sent Athena's start
point put the character on exactly this map. The cell is still only
Athena's. The report's further arguments (a 2017 private-server video, a
2013 rAthena NPC three cells north) do not date the cell.

**Played in the client (rig, 2026-11-11):** a new character walked the
whole course on this client's maps: the bridge, the castle door, the
Usher, Leo Von Frisch's six questions, the NPC Master and the Guide in
the east room, the three Helpers, the Fabre field (`new_zone03`: the
arrival cell `96,21` lies behind the rampart as the camera sees it, so the
character is hidden until she walks north), both personality tests and
the last Helper. Every dialogue, menu and warp works.
**817's script against mod0631's** (`conf/npc_job_newbie.txt`, 2,722 lines
against 817's 1,400; read NPC by NPC, 2026-11-11). 817 is an English port
of the same original, partly of another version of it: Leo's second and
third questions are different questions (817: "wrong description of a
stat" and "which has nothing to do with me"; mod0631: "which monster does
not exist" and "pick my name"), and the second personality test has a
question mod0631 lacks. So 817 may be nearer the international servers
there, or just looser. Mended from mod0631 where 817 contradicts itself
or cannot be played (DEVIATIONS): the last Helper, the points of both
courses, the forest Helper's save point and tries, Leo's last question and
his two dead ends, the hall Guide's directions. **Left as 817 has them, open:**
* the Usher says "Training Subsidy 50 Zeny" and gives 150 (mod0631 says
  and gives 150: `npc_job_newbie.txt:64-66`);
* Leo's prizes: one Banana, one Banana, three Sweet Potatoes, one Apple
  (mod0631: no item, five or three Bananas, ten Sweet Potatoes, three
  Apples), and his Red Herbs: always four (817 tests the wrong counter;
  mod0631 gives four for six right, two for five, one for one to four);
* mod0631 draws the second, fourth, fifth and sixth question from two
  variants each; 817 has one;
* hearing Leo again after the NPC Master hands the prizes out again in
  817 (its guard variable is only set by the last right answer); mod0631
  asks each question once;
* the Newbie Instructor gives one Milk every time until Leo's first
  question is answered (mod0631: three, once);
* first personality test, "Theory / Experience": 817 gives no point for
  Experience (mod0631: Swordman); the second test's "competition"
  question gives one job a point where mod0631 gives two;
* the Helper of `new_zone04 100,29` always sends to `94,56` (817 forgot
  the `rand(4)` it uses a room later); mod0631 has the first test's
  Helper at `20,69` and sends to `19,54`, where 817 has him at `100,69`;
**Look:** an iRO or euRO walkthrough of the training ground (2003-04:
the quiz questions and the prizes would tell which version the
international servers ran).
**Round 5 (2026-11-11):** one player's word, iRO, from the base text of
Weggy's FAQ (GameFAQs faq 20092, older than its 6.0 of 2004-05-17):
asking for "access directly to the game ... whisks you away to a random
town"; "After completing the training grounds, you should have 100 zeny,
or 200 zeny if the final few NPCs guessed your class correctly"; then you
choose a town and are "whisked away" to it. 100 or 200 is nearer neither
script (mod0631 100z to 500z by points plus a guild gift; 817 up to 500z);
nothing on Leo's questions.
**Goal (owner, 2026-11-09):** find the period-accurate start position and
document it. Failing that, pick the best-supported cell and list it as a
possible deviation.
**Impact:** start point, first impressions, novice rewards.
**Look:** period first-screen reports and screenshots, OpenKore 2004 logs,
kRO/jRO notices on the training ground (초보자 수련장), the iRO guide's
beginner pages. Round 1: research/agy-threads/HANDOFF.md.
**Round 7 (2026-11-14):** nothing new: round 5's Weggy rows and the
Usher's lines above again (`research/notes/round7-review.md`).


### 7. Quest skills and their NPCs in iRO, Dec 2003
817 places the Thief quests (Back Slide, Pick Stone) at morocc 165,124,
both on the same cell, with spaces instead of a tab in the header, so 817
never loaded them. Were the other platinum quests (Arrow Crafting, Charge
Arrow, First Aid…) on iRO/euRO by Jan 2004?
**Impact:** `scripts/import/npc/skillquests`.
**Look:** iRO guide skill pages (2004-03/04), kRO notices.
**Round 2, Thief skills done:** mod0631 has one NPC for all four,
"アルコスク" in the Thief guild (`moc_prydb1 154,128`), with a pouch maker
in Payon for Sand Attack, and the client ships that quest's two items.
`scripts/npc/thief_skills.lua` replaces 817's lone Stone Fling NPC.
RODataZone's Thief page of 2003-12-04 says the same in English ("Alcouskou
... (moc_prydb1, 154/128)", same items), so this is how the international
servers had it too. It puts the Bag Seller at `payon 28/121` (mod0631:
30,120) and Stone Fling at Thief job 20 (mod0631, 817: 15); we keep
mod0631's.
**Open:** the other jobs. Three witnesses to read side by side: 817 (ours),
mod0631's `npc_event_skillget.txt` (15 NPCs), and RODataZone's job pages of
late 2003 (`www.rodatazone.com/jobs/<n>/<job>.php` on Wayback: names, cells,
items). First known difference: Moving HP Recovery is a Rocker Doll and 500
Empty Bottles in 817 and on RODataZone, 200 bottles and another item in
mod0631. First Aid and Play Dead (Nami, the Chivalry member, the Nursing
Instructor) are in mod0631 and on RODataZone and not in our import.
**Round 3:** NPC, cell and job level of every quest skill in the three
witnesses are rows now (`research/agy-threads/r3/rows/q.jsonl`, checked in
research/notes/round3-review.md). 817 and mod0631 agree on every cell
outside the Thief's; RODataZone gives the same cells for the Archer,
Acolyte and Merchant skills, none for the Mage's, and other numbers on its
Swordman page (`214,129`, `46,345`, `178,185`: not read yet). Job levels
differ for Arrow Crafting (817: 35; mod0631, RODataZone: 30), Cart
Revolution (817, RODataZone: 35; mod0631: 30) and Stone Fling (emulators:
15; RODataZone: 20). The emulators' item lists were not compared. First
Aid and Play Dead are confirmed on the same cells by mod0631 and
RODataZone's Novice page of 2003-12-15, and are in the game
(`scripts/npc/novice_skills.lua`, DEVIATIONS).
**Round 4, the Swordman's three:** Fatal Blow and Auto Berserk have the
same items in 817, mod0631 and on RODataZone's Swordman page (capture of
2004-05-02). Moving HP Recovery does not: RODataZone and 817's dialogue
say 500 Empty Bottles and a Rocker Doll (817 adds a Padded Armour),
mod0631 says 200 Empty Bottles and a Moth Wings and that the armour is
only to be worn, and 817's code tests for the first list and takes the
second. `scripts/npc/moving_hp.lua` takes the bottles and the doll; the
armour is only to be worn, as in mod0631, and is not tested (DEVIATIONS;
unconfirmed).
**Open:** the item lists of the other jobs' quests side by side (Archer,
Merchant, Mage, Acolyte: mod0631's are in `conf/npc_event_skillget.txt`,
RODataZone's on its job pages); whether the Padded Armour was asked for
on the international servers at all.
**Round 7 (2026-11-14): the Padded Armour was asked for.** vgmaster88's
Swordsman guide (GameFAQs 19031, v3.2, iRO, autumn 2003): "The items for
Moving HP recovery are 1 Padded Armor (which you get to keep), 500 empty
bottles, and a rocker doll." That is 817's code (it tests for one and does
not take it); `scripts/npc/moving_hp.lua` now does the same (2026-11-14).
The same guide gives Fatal Blow 10 Tentacles where mod0631, 817 and
RODataZone have 30: the guide's slip. Auto Berserk agrees with all.
Weggy's FAQ (v6.0/6.2, May 2004) on the Swordman course: "If you do fall
off, there are some aggressive Fabres and Chonchons down there": both
(817 has Chonchons, mod0631 Fabres).
**Round 8 (2026-11-14), the other quest skills side by side**
(`research/notes/round8-review.md`): Arrow Crafting, Change Cart, Crazy
Uproar, Holy Light and Energy Coat take the same items in 817, mod0631 and
on RODataZone. Three differ: Cart Revolution (817 asks and takes 15 Iron;
mod0631 picks one of three sets, 23/20/15 Iron; RODataZone 2003-10 gives
the 20-Iron set), Gershaun's Banana Juice before he talks business
(mod0631 and RODataZone; not in 817), and a Crossbow for Charge Arrow
(RODataZone 2004-04 only). We play 817's for all three: to decide.
**Round 11 (2026-11-14) settles two of them**
(`research/notes/round11-review.md`): Godhand's Arrow Crafting FAQ
(GameFAQs 26381, idRO, v1.3 of 2003-10-23) and Battousai X's quest-skill
FAQ (27646, pRO, v1.3 of 2004-01-22) list a Crossbow for Charge Arrow and
say it is given back; Battousai X says Gershaun wants a Banana Juice
"but if you already have all the items, you won't need the juice", which
is mod0631's script line for line. Both are in the game
(`scripts/npc/skill_quests.lua`, DEVIATIONS). Cart Revolution's Iron is
still open, now with a fourth figure: Battousai X gives 30 Iron (817 15,
RODataZone 20, mod0631 23/20/15 at random). The same FAQ gives Charge
Arrow from job level 37 (817 and mod0631 35).
**Decided (2026-11-14): 20 Iron.** RODataZone's iRO list (2003-10: 20
Iron, 30 Sticky Mucus, 20 Fly Wing, 5 Tentacles, 2 Grape Juice) is
mod0631's middle set item for item; 817's and Battousai X's lists differ
from it only in the Iron. So the international servers had one fixed
list, the middle set, and 817 and Battousai X each slipped on one
number. `scripts/npc/skill_quests.lua` replaces 817's Gershaun (test
`charge_arrow_crossbow_and_gershauns_juice`). An iRO or euRO account of
2004 naming another figure would reopen it. Charge Arrow's job level 37
is Battousai X's alone: kept at 35.


### 8. Wizard (and other 2-1) job quests
817's wizard.txt is an unfinished conversion that 817 couldn't run (later
dialect, `Job_Magician`, missing gotos). We used 659's simple guild NPC
(job change at job level 40, no test) until 2026-11-11; see the last
paragraph. 817's own Knight, Blacksmith and Assassin are still such
one-line NPCs. 817's Priest and Hunter do have quests.
**Impact:** whether the 2-1 jobs should have their tests; see #5.
**Look:** eathena-v0.5.2 (2004-05) `npc/jobs/*.txt`; period
walkthroughs (#2).
**Round 1:** the Wizard test existed on kRO by 2004-02-10 (a notice fixes
"no recovery when moving to the next stage"). eathena-v0.5.2 has a full
three-stage script, but it runs on `job_wiz`, a later map: this client's
test map is `job_wizard` (its `wizard_1-1`), and the cells do not match
(3 of 10). A port has to find the three rooms on the older map first.
**Round 2:** this client's `job_wizard` is a cross of five rooms: a centre
room and one at the end of each arm (`job_knight` has the same plan). No
script for it turned up: jAthena's first Wizard test (2004-04-09, "from
eAthena's data and old data from elsewhere") is for `job_wiz` too, and its
Knight, Blacksmith, Hunter and Assassin quests date from September 2004.
817's Priest test does fit this client's `job_priest` (4 of 4 cells). A
Wizard test here would have to be laid out by us. (2026-11-10: the cells
fit, but 817 plays the test's three rounds on three copies of the map and
this client has one; they are played on `job_priest` in turn,
`scripts/npc/priest_test.lua`.)
**Round 3:** nothing new. The round's Wizard rows quote jAthena //1046
(2004-09-12), again for `job_wiz`: of its three room cells, `116,171` and
`46,99` are blocked on `job_wizard`. No dated walkthrough of any 2-1 test
was found; RODataZone's Wizard and Knight pages on Wayback are empty.

**2026-11-11: the maps are in the client after all.** Rounds 1 to 3 read
the CD's base GRF (`data.grf.original`). The four maps `job_wiz`,
`job_knt`, `job_prist` and `job_hunte` are in
`2003-04-30gdata_turtle_ph_2.gpf`, the second of the two Turtle Island
patch files on the CD, which the launcher merges (`ragnarok-offline grf ls`
on the file; the base GRF's `job_wizard`, `job_knight`, `job_priest`,
`job_hunter` stay beside them). On euRO those two files are patch-list
entries 400 and 401, between "New Comodo" (2004-01-09) and 2004-01-15
(`research/notes/euro-history.md` §5): the test maps arrived in the very
weeks "added 2nd job quest" was announced. All nineteen warp cells that
mod0631 and jAthena (from //0505, 2003-10-30, `conf/npc_warp25.txt`) put
on the four maps are walkable on them (`research/gat_cells.py` on the
merged GRF: 10 of 10, 3 of 3, 4 of 4, 2 of 2). So "a later map" was
wrong: a Wizard test written for `job_wiz` fits the client we play, and
three period scripts are (817's unfinished `npc/jobs/wizard.txt` of
2004-02-19, jAthena's of 2004-04-09, eAthena 0.5.2's of 2004-05, which
its own config switches off as "totaly bugged"). The Wizard test is now
in the game from them (`scripts/npc/wizard_test.lua`, DEVIATIONS).
`job_prist` and `job_cru` are the same file.
**2026-11-11, the Priest's official test is on disk.** jAthena's
`script/npc/job/npc_job_priest.txt` of 2004-06-08 (commit `0f45810`) is
the three-part test on `job_prist`: a pilgrimage to the three ascetics of
the Acolyte test (job level 50 skips it), an exorcism on `job_prist`
(thirteen undead must fall before the gate at `24,109` opens; tempters
further on whose wrong answer throws you out; 350 seconds in all), and
a nun's vow
(`prt_church 27,24`); the reward is a Book. 817's Priest test, which the
game plays (`scripts/npc/priest_test.lua`), is its author's own invention
on the older `job_priest`. Five months after our period, but the nearest
script of the test that "added 2nd job quest" brought in January 2004,
and on the map the client has for it. **Ported the same day**
(`scripts/npc/priest_test.lua`, DEVIATIONS). Open on it: whether the test
of January 2004 was already this one (a player's account before June
would date it), the demons' sprites as the official server sent them
(jAthena gives monster ids, and 736 for Baphomet, which this client may
not have; we send the monster's id), and 817's Wraith spawn left on the
unused `job_priest`. Open: what the Knight's and
the Hunter's tests on `job_knt` and `job_hunte` were (jAthena's Knight
test dates from 2004-09-12; nothing earlier), and whether the Priest's
ran on `job_prist` by then (817's fits the older `job_priest`).
**Round 5 (2026-11-11): the Hunter test on iRO by February 2004.**
Forever-Zero's Archer/Hunter guide (GameFAQs faq 27633; "1.9 (02/15/04)
- Added the Hunter Quest") walks it: before Turtle Island the front desk
changed you at once; after it, a personality quiz, one of three item
lists from the receptionist, a Master Hunter in Payon (the palace or the
Archer guild) who gives 200 Silver Arrows and sends you to a waiting
room's chat, one candidate at a time; in the room four "Job Change
Monster"s among others named "Hunter Change Monster" or "Job Test
Monster", watery tiles that are traps; when the four fall, a switch in
the middle offers "escape" and three minutes to leave by the far side;
then a necklace for the front desk. eAthena 1.0's `npc/jobs/2-1/hunter.txt`
(2004-11-04, "converted from aegis script", translated by yoshiki like
817's Wizard test; `data:refs/rathena` commit `706e78509`) is that test
on `in_hunter`, `payon_in02` and `job_hunte`, down to the monster names.
The Wizard guide (faq 30054; its Wizard section is of 2004-07-30) adds:
a failure leaves you saved outside the tower, and after five or six
failures the examiner lets you skip the fight for a Worn Out Scroll.
Edge's Knight guide (faq 29457, pRO, from 2004-02-08): "Pay a small fee
and magically you become a Knight": no test there.
**2026-11-11: the Knight test is in** (`scripts/npc/knight_test.lua`,
DEVIATIONS), from jAthena //1046 (2004-09-12) with eAthena 1.0's English
names; played in the client from the application to Sir Edmond's room.
jAthena and eAthena 1.0 are two separate reconstructions, not a
translation and its source: they differ in three of Sir James's eight
questions, in which of Lady Amy's answers count, in Sir Grey's interview
(eAthena scores it, jAthena passes everyone), in Andrew's lists (eAthena
picks one of two; its changelog says it added the second), in the arena's
cells and Sir Edmond's room (eAthena: eight Spores, live Chonchons, 240
seconds). Open: a player's account of the Knight test before September
2004 (which questions, which items, and whether it existed at all before
then: pRO had none in February); what Sir Grey's pass condition was.
**Round 8 (2026-11-14):** jAthena's own comment on Sir Grey says the pass
condition was not known, so it passes everyone on the second talk. A later
source has one: rAthena's official conversion (`npc/jobs/2-1/knight.txt`,
"PGRO TEAM (Aegis)") scores the interview and passes a total of 0, 5 or 10,
otherwise back a step. Grade D for when; the best approximation we have
(owner, 2026-11-14: use later sources where the period ones cannot
settle). Not in the game yet.
**2026-11-13: the Blacksmith test is in** (`scripts/npc/blacksmith_test.lua`,
DEVIATIONS), from eAthena 1.0's `npc/jobs/2-1/blacksmith.txt` (2004-11-04,
"EREMES (Aegis), translated by yoshiki", the only complete script on
disk). jAthena //1046's `npc_job_10blacksmith.txt` (2004-09-12) has the
same NPCs (アルトアイゼン, ゲッシュ, ミットマイヤ and the five customers),
the same two sets of Geshupenschte's ten questions and two of Mitmayer's
three, but its guild member changes you "for lack of data" and its
Mitmayer is "未調査"; so it is a witness, not a script to play. Where it
parts from eAthena: pass marks (jAthena 90 first, 80 on a retry, Mitmayer
100; eAthena 100, 90, 80); question 8 (jAthena: a level 3 weapon, +6;
eAthena: level 3 +5 and level 4 +4); question 6 of the first set (jAthena
a Ring Pommel Saber, eAthena a Buster); the Arbalest order (jAthena an
Eye Patch, eAthena a Trunk); Tilpitz's cell (jAthena `payon 151,39`;
eAthena's `214,79` is off this client's Payon) and his order (jAthena:
a pouch for his eye patch; eAthena: the Arbalest); jAthena's Mitmayer
counts any answer to "which ore do you have most of", eAthena all but
Wind of Verdure. jAthena's Gromgast refuses a Tsurugi with cards or
refines; eAthena does not check. Open: a player's account of the
Blacksmith test before autumn 2004 (queue T1). Played in the client from the
application to the job change (Gladius order, Mitmayer at 80 points).
**2026-11-13: the Assassin test is in** (`scripts/npc/assassin_test.lua`,
DEVIATIONS), from eAthena 1.0's `npc/jobs/2-1/assassin.txt` (2004-11-04,
kobra_k88: "Based of Official RO Assassin jobchange quest", dialogue partly
from the official script via pRO's translators), the only script on disk:
jAthena's of 2004-09-12 is a stub ("不完全"). The map came with the patch
too: the turtle patch's `in_moc_16` walls off the Guild Master's maze, which
`data.grf.original`'s leaves an open hall (sampled with `gat_cells.py`:
126 of 208 cells walkable against 169). mod0631's and 817's warp list
chains the rooms with free warps from the field door into Kai's room;
the test needs the door into the entrance hall instead (eAthena 1.0).
Played in the client from the door to the job change (2026-11-13): Kai
hides and then registers, the quiz (set 1), Barcardi's targets (a decoy
killed and the clock running out both fail you back to her room), the
hide room (the Mummies kill a lv-70 Thief without armour who stops;
dying fails the test like the clock), the maze's voices, the six testers
in the office, the reward choice at job level 50, and Huey's job change
(the necklace stays, as in eAthena). Open: any account of the Assassin
test before autumn 2004 (queue T1).
**Round 7 (2026-11-14, `research/notes/round7-review.md`).** Weggy's
FAQ (GameFAQs 20092, iRO). Its text from before his year-long break
(about mid-2003) changes Knights for 2500z and Assassins for 666z with no
test. On 2004-05-18 he noted that "Gravity also added more in-depth
quests to the 2-1 classes". His Priest section (v6.4, 2004-06-07) has a
priest's questions in the antechamber, then "a hall of temptations",
then "rooms where undead monsters spawn. You can actually just run past
them all". So iRO had a three-part Priest test by 2004-06-07. It differs
from jAthena's of 06-08, which we play: there thirteen undead must fall
before the gate opens, and the tempters come after them. Guide_Maker's
Forging guide (GameFAQs 28512, pRO, section of 2004-08-31) walks the
Blacksmith test as eAthena 1.0 has it: a quiz in Alberta, materials, a
"Reciept", a quiz in Morroc, the "Hammer of Blacksmith". Still no account
of the Knight's, the Assassin's or the Blacksmith's test before autumn
2004.
**Round 12 (2026-11-14):** iRO Wiki's walkthroughs of 2008-09, in a
Russian translation (sharonov.ru; grade D, later evidence only). Knight:
Sir James's answers are jAthena's, which we ask; Sir Andrew picks one of
four lists (jAthena's set 1 or eAthena's set 2, each whole or less one
item), where we ask set 1; seven Awakening Potions at the change.
Blacksmith: the "old" orders (8 Iron Ore + 1 Trunk + 2 Blue Gemstone for
the Arbalest) and the level-4 answer (+4 "before the 2008 patch") are
eAthena's, as we have them; the Eye Bandage list is of June 2008.
Assassin: the rewards and the necklace as eAthena 1.0. Nothing older.


### 9. Refining details
Phracon price (iRO page 2000z vs data 200z), armor DEF per refine (1, 5 or
0.7 across emulator versions), refiner dialogue, and what a failure does to
cards.
**Impact:** shops, `rules/status.lua` refine bonuses (not applied yet),
`scripts/import/npc/other/refine.lua`.
**Look:** iRO *Refine System* page (2004-06, Wayback), kRO notices.
**Round 1:** the iRO page (capture 20040605070057) was loaded. It says: on
failure "the item, Zeny used, and any cards in the slots will all be
destroyed completely"; Phracon 2000z, Emveretarcon 1,000z; fees 50 / 200 /
5,000 / 5,000 (weapon levels 1–4) and 2,000 (armor); +2 / +3 / +5 / +5
damage per level; armor "+1 adds 1 defense"; safe to +7 / +6 / +5 / +4,
armor +4. Its level-4 row repeats level 3's, so the page is not careful,
but it is the only official statement found. Cards: settled. The rest
stays a recorded disagreement (PROVENANCE).
**Round 2: mostly settled.** The page corrected itself: Phracon is 200 zeny
in the capture of 2004-08-10, and level 4 is 20,000 zeny and +7 in that of
2004-12-05. kRO's notice of 2003-02-04 already had level 4 at +7 per
refine and safe to +4, armor safe to +4, and "DEF from refining is less
efficient than the armor's own DEF". mod0631's refiners charge ten times
817's fees: that is jRO.
**Open:** how much DEF a refine level gives (not 1, by kRO's notice; no
official number), and the size of the random bonus above the safe limit.

### 10. Monster spawns, Dec 2003
mod0631's spawn lists are the closest in time. Some kRO spawn rebalances
happened in 2003 (Lutie toy factory 2003-04-08).
**Impact:** `scripts/import/spawns.lua`.
**Look:** OpenKore portals/monster tables, RO Empire maps (2004), iRO
Rune-Midgarts Library (monster locations).

### 11. Shop NPC names and stock
Done in part: shops are named from 817 by cell. Two of 817's lines have a
space before the tab and were missed until 2026-11-14 (Izlude's Butcher,
the church's Nun). Where 817 has no shop near the cell, the importer takes
the English 817 gives most often to shops mod0631 names alike (its
道具商人, 武器商人, 防具商人 and テイミング商人 become Tool, Weapon and
Armor Dealer and Pet Merchant). Two still show "Shop": Prontera's
野菜商人 at `48,58` and Juno's 王窓商会, words 817 never translates.
**Round 9 (2026-11-14):** no period English for either; later ones are
"Vegetable Gardener" (rAthena) and, for Juno, "Equip Dealer" (eAthena 1.0,
a role) or "Mr. King's Shop" (rAthena, a translation of mod0631's name).
The game now shows "Vegetable Gardener" and "Mr. King's Shop" (tier later,
`tools/import/athena_import.py`; research/notes/round9-review.md). Every
shop has an English name.
**Impact:** shop names over NPC heads; stock lists vs iRO 2004.
**Look:** match 817 `npc/shops/shop.txt` by cell; the iRO item library
(2004) for prices.

### 12. Map flags
mod0631's flags are applied (noteleport, nobranch, nopenalty, nosave).
817 adds about 170 more (nopenalty indoors, castle flags) that mod0631
lacks. Which did the official servers have? Unknown.
**Impact:** dungeon teleport rules, death penalty in towns.
**Look:** 817 `conf/mapflag/*.txt`, mod0631 `conf/mapflag.txt`; check
what the client's own `mapnametable`/indoor tables imply.
**Round 1:** the client's `indoorrswtable.txt` lists 22 interiors (town
insides, `prt_are_in`, `arena_room`, `sword_1-1..3-1`, ...). It is a
rendering table; nothing ties it to Teleport. No change.

### 13. Dyes and palettes
817's dyer needs a 77-palette client patch. Which clothes dyes existed in
Dec 2003? This client ships 5 clothing palettes per job and sex (the default
plus 4) and 9 hair colours per hairstyle (`data\palette\`, tier client).
**Impact:** `scripts/npc/dyes.lua`.
**Look:** the client GRF's `data\palette\` (tier client); 659's
`npc_dye.txt`; kRO 2003 notices.
**Found (2026-11-10):** mod0631's `conf/npc_event_dye.txt` (Dec 2003) has
the dye makers and both Prontera dyers. Its hairdresser states the terms
(one dye of the wanted colour, 5000z) and stops at "not implemented"; its
clothes dyer turns everyone away as "too busy", with the working dialogue
commented out. The client's hair palettes, read from the GRF (all 38 sets
agree): 0 red, 1 yellow, 2 violet, 3 orange, 4 green, 5 blue, 6 white,
7 black, 8 red again (the same file as 0 in 36 sets). Hair dyeing is built
on that. **Still open:** was clothes dyeing available on euRO/iRO in
Dec 2003 to Feb 2004, with which materials and price, and which of the four
extra clothing palettes went with which dye; whether 5000z and one dye were
the hairdresser's terms outside jRO; the NPCs' official English names.
(Round 11: ArislanX's Alchemist guide, iRO 2004-08-20, has "Louitz",
"Aure Dupon" and "Molgensten"/"Old Molgy" on our cells; blood3d, pRO:
Counteragent 3000z, Mixture 4000z, Black Dye 7000z, Blue Dye 3500z,
possibly as of 2004-08.) (Round 10, tier D:
rAthena's hairdresser asks 1000z and a dye; its tailor turns everyone away
as mod0631's does, and its note v2.4 says cloth dyeing was "not
implemented on official servers".)

### 14. Pets on euRO's closed beta
The closed beta "includes Comodo and Q-Pet". Which monsters were tameable,
with which items, and the pet food and accessory list.
**Impact:** the pet system (ROADMAP).
**Look:** iRO `qpetsystem.asp` (2004-03, 24 tameable), 817 `db/pet_db.txt`.
**Round 2:** the iRO page (capture 2004-06-05) was loaded: 24 pets, Monster
Tamers "in the buildings of Alberta, Izlude, and Al De Baran", Taming
Merchants in every other town. mod0631 has those three tamers
(`npc_event_mobtim.txt`); they are in the game now
(`scripts/npc/monster_tamers.lua`). The page has no table of pets: the
per-pet items are still only the emulators', and the Rune-Midgarts Library
pages (`libraryListItem.asp?Category=7`) are the next thing to load.
**Round 3:** loaded (capture 2004-06-05). Its item texts name the monster
each taming item lures, and all 25 agree with `data/db/pets.toml`. It lists
25 pet eggs, Bongun's included, against the guide's "24 monsters": whether
Bongun could be tamed in early 2004 is the one open point. Food and
accessories were not compared.

### 15. euRO-specific content
Loading screens, event NPCs (Squad Tournament July 2004, King of the Hill
Aug 2004), German/Italian/Turkish text.
**Impact:** optional euRO flavour.
**Look:** euro-history.md sources; the 2004 patch lists (`gdata_g` files
carry euRO's own data).

### 16. The ordeal maps
`ordeal_a00`/`ordeal_a02` are in the client. In Athena they are
`ordeal_1-1..3-4`, reached by nothing in 817.
**Impact:** whether to link them in.
**Look:** what kRO used them for in 2003 (a labyrinth? a job test?).

### 17. WoE and guild castles in single player
WoE needs guilds and opponents. Today the castles are empty and walkable,
and the guild dungeons (`gld_dun01-04`) are unreachable: no warp leads in,
yet they hold mini-bosses and an Eddga.

**Goal (owner, 2026-11-09), in three parts:**
1. Find out what WoE and castle ownership gated in 2003–04: treasure rooms
   and Treasure Chest drops (Yggdrasil items, Freyja's Jewel, Angel
   Helm...), guild dungeon access, castle Kafra, investment, guild skills.
2. Decide whether it can be repurposed into a single-player activity.
3. Otherwise, give access to that content another way.

Whatever we build gets a DEVIATIONS.md entry saying what is period
behaviour and what is invented.

**Decided (owner, 2026-11-10):** part 2 it is. Once we know how the
original worked, exactly, the castles are reworked into something a single
player can do that keeps the feel of the content, as with anything else
that needed other people. So the research comes first: mechanics, numbers,
and what a siege and a held castle were like (round 3, tasks W1–W7). The
options below are the round-1 sketches, not the design.

**Round 1, done:** a castle without an owner held monsters. kRO's notices
of 2004-02-10 and -17 describe it (and remove it); 817's `mobs/gvg.txt`
has the lists. With one player no castle ever has an owner, so the castles
now hold those monsters (`scripts/npc/castles.lua`, DEVIATIONS).

**Round 1, for the owner to decide** (the report's options, its sources
mostly tier later; research/notes/round1-review.md):
* A. Let the player take a castle: break an Emperium guarded by that
  garrison, then a steward, investment, the daily treasure room and the
  lever into the guild dungeon. Uses period maps and numbers; the solo
  capture is invented.
* B. Leave castles as they are and open the guild dungeons and treasure
  another way (an NPC, a quest). Little is period.
* C. A timed trial modelled on kRO's later novice/training WoE. Later
  inspiration only.

**Round 2:** the iRO pages were loaded; every rule on them is quoted in
research/notes/round2-review.md. For this decision: the master room gives
"1 rare item ... once a day, depending on the Commerce Development
Investment"; an emptied castle "becomes filled with monsters" (iRO, April
2004: what our castles do); the owners "recover their HP/SP faster".
kRO's notices date treasure boxes to before 2003-07-29 and PvP in the
guild dungeon to before 2003-12-26. **No period source gives a number** for
the treasure count or the investment cost: the round's figures are from
GungHo's guide of today and a Japanese wiki (tier later), and its summary
attributes them to the iRO page, which does not have them. jAthena's first
steward (2004-02-12) takes a flat 10,000z in a test script.

**Round 3** (research/notes/round3-review.md; rows in
`research/agy-threads/r3/rows/w*.jsonl`):
* kRO's notices to 2004-06 are read in full: 23 rules and fixes, none with
  a number. The Emperium became Holy/Angel only on 2004-04-27.
* 817's code: skills do nothing to the Emperium, nor does a player without
  a guild that has Official Guild Approval; no knock-back on siege maps;
  Endure, Teleport, Warp Portal, Ice Wall and Back Slide are refused.
* 817 has the forty treasure boxes in `mob_db.txt` and **no script that
  puts one in a castle**, so which box belongs to which castle, and how
  many appear, has no period source. Still no period number for investment
  beyond the test steward's 10,000z, twice a period.
* 817's Emperium cells fit this client's maps in 19 of 20 castles
  (`prtg_cas02 158,174` is blocked), and every guild dungeon switch target
  is walkable. The round's own castle rows have wrong Emperium cells: use
  817's scripts, not those rows.
* euRO: ownership and investment were live in the open beta by 2004-04-08;
  treasure, Emperium and guild dungeons were off from 2004-09-03 and WoE
  back on 2004-09-22.
* "What it was like", in numbers, from another official server
  (research/notes/woe-mro-2004.md): a fan site's takeover record of
  Malaysia's servers, 2004-03-18 to 06-07. Sieges on Wednesdays and
  Saturdays; a contested castle changed hands a median of 3 times in a
  siege and up to 24; three holders in four lost it by the next siege; the
  longest hold on record is three weeks. Its siege commentary topics are
  not on Wayback. Still nothing first-hand on numbers of players, where the
  fighting was, or treasure; euRO's own news of 2004 is listed and unread.
**Impact:** castle treasure, guild dungeons, guild-only drops.
**Round 11** (`research/notes/round11-review.md`): Arctic Breeze's WoE
guide (GameFAQs 28786, iRO, 2004-02-18; its treasure and dungeon sections
are twRO/kRO data via fan sites) names the god-item material of each
castle. 817's forty treasure boxes, read in pairs in region order (Al De
Baran 1-5, Geffen, Payon, Prontera), drop exactly those materials from the
second box of each pair, all twenty: the box-to-castle pairing is period.
Rate: 1/250 in the guide, 10/10000 in 817. Also: two investments a day,
each twice the last; trade (and defence) -2 when a castle changes hands
("Unconfirmed" in the guide); castle Kafra free storage, warps to every
town; guild dungeon spawns and respawns (MVPs 8 h, Ghostring 4 h); WoE
without EXP loss, Phen card, Anodyne or Intimidate.
**Look:** kRO notices on WoE rules; the iRO guide's guild/WoE pages; euRO
WoE news (Apr and Sep 2004: "treasure boxes and guild dungeons disabled");
period guides. Round 1: research/agy-threads/HANDOFF.md.

### 19. Lutie (Xmas) in January
Was Lutie a permanent town on euRO, or seasonal? The CD patcher carries
`2003-10-31gdata_xmas.gpf`.
**Look:** euRO notices Dec 2003–Jan 2004.

### 21. Ailments: which emulator is nearer the official servers
mod0631 (2003-12): fixed chances, no resistance from the target's stats,
placeholder lengths, no Curse, poison without HP loss. eAthena 817
(2004-02): chances and lengths fall with MDEF / VIT / INT / LUK, lengths
per skill level in `cast_db.txt`, poison takes HP down to a quarter, Storm
Gust freezes on the third hit. We follow 817 (DEVIATIONS.md). Neither says
where its numbers came from. 817 sends opt1 = 6 for the five seconds of
turning to stone; we keep it server-side and send nothing until the stone
sets: does the Dec-2003 client draw opt1 6 (look for it in the exe)?
**The iRO guide's skill pages** (2004-03-27 to 2004-04-13) side with 817
wherever they speak: Frost Diver's chance and time fall with the target's
magic defence, Freezing Trap's chance likewise; Stone Curse's time
depends on the target's level and LUK and costs 1% HP every 5 s down to 1
HP; Envenom's chance falls with the target's VIT; Storm Gust freezes on
the third hit, never bosses or the undead. They give no formula and say
nothing on stun, sleep, blind, silence or curse resistance. So 817's
shape is right for March 2004; whether it held in December 2003 is open.
**Round 6 (2026-11-11):** when the resistance entered the Athena family.
jAthena //0729 (`71d06d6`, 2004-01-21) adds it to `skill_status_change_start`:
`sc_def_mdef = 100 - mdef`, `sc_def_vit = 100 - (vit + luk/3)`,
`sc_def_int = 100 - (int + luk/3)`, the same for monsters from
`mob_db`. //0736 (`188a761`, 2004-01-23) makes it cut the time too, "by
the same rate as the chance", and takes Stone Curse's time from the
2channel mage thread's template ("マジスレテンプレ準拠"). So jRO's players
had a resistance worked out by January 2004; 817's `3 +` and LUK-only
curse/sleep terms are a step later.
**Round 12 (2026-11-14, `research/notes/round12-review.md`): a dated
English copy of that research.** RODataZone's "Status Effects" page
(`data:refs/web/rodatazone-2004/`, written between 2004-01-19 and
2004-02-10 by the site's directory listing) says "This information comes
from Ragu Memo" (a Japanese research site). Its chances are 817's to the
letter: `BaseChance * (97 - VIT - LUK/3) / 100` for stun and silence,
MDEF for freeze, INT for sleep and blind. It parts from 817 on: Stone
Curse resisted by INT (817 MDEF), 17 - MDEF/20 s; Poison by VIT + LUK/5,
3% of max HP every 3 s down to a quarter, 10-30 s on monsters and 5-60 s
on players; Blind `30 - BaseLevel/10 - INT/15` s (at least 15); Stun
`3 * (97 - VIT - LUK/3) / 100` s; Curse "LUK helps, formula unknown".
The nearest-dated source we have for the lengths (817's come from its
`cast_db.txt` per skill): set them side by side, skill by skill, before
changing any. A pRO guide of 2004-08 (GameFAQs 31808) copies the page.
**Impact:** `skill_additional_effect` and the ailment part of
`skill_status_change_start` in `scripts/rules/`.
**Look:** period player guides on stun and freeze resistance ("VIT 97
immune to stun" is a later formula: when does it first appear?), the iRO
guide's status pages (2004), kRO notices.

### 22. Skills approximated or missing
Built from the client's text or by analogy, to be sharpened (DEVIATIONS.md
has the table): Flasher's chance, Shockwave Trap on monsters, Enchant
Poison's chance to poison, Brandish Spear's cells, Detect's range, Change
Cart's level steps. Open in both emulators and drafted from the client's
text only: Poison React (what counts as a poisonous attack, the counter's
strength, how long it lasts), Remove Trap and Spring Trap (whose traps,
which kinds, Spring Trap's range by level). Not built: Venom Splasher's
delayed burst (817 makes it a plain splash), Talkie Box, Repair Weapon. Arrows add no ATK in mod0631 (the line is
commented out) and its bow skills use none: did they on the official
servers of Dec 2003? The client shows an ATK on arrow items.
**Read against the iRO guide's skill pages** (2004-03-27 to 2004-04-13,
`research/notes/round4-review.md`; 2026-11-11):
* *Agrees with the draft:* Enchant Poison's chance to poison (3% to 7%)
  and length; Flasher blinds; every trap's time on the ground; Ankle
  Snare's hold (4 s a level); Claymore Trap; Talkie Box's 600 s; Remove
  Trap gives one Trap back; Venom Splasher's 220% to 400%; Ice Wall (the
  guide: 400 to 2200 durability, 50 lost a second, which is our 8 s to
  44 s).
* *Changed to the guide's:* Poison React lasts 30 s (817's table: 30
  minutes); Detect reaches 3/5/7/9 cells and Spring Trap 4 to 8 by level;
  a trap that has gone off cannot be taken up; Brandish Spear's cells
  follow the guide's drawing and it pushes two cells.
* *Differs, kept:* Skid Trap slides 6 to 10 cells in the guide, 1 to 5 in
  the client's text and in mod0631, which is what we do. Unlike the rows
  of R6, the period emulator sides with the client's text here, so this
  may be a change made between December and March. **Round 10:** it came
  with iRO's Comodo update (Episode 3.0), a "skill balance patch" that
  Forever-Zero's guide (GameFAQs 27633, "Future iRO update nerfs/changes",
  begun 2003-11-23, edited 2004-01-10) copies whole: "Skid Trap's SP cost
  has been increased from 5 to 10. The shove range of it has been increased
  from 6 to 10 cells"; Double Strafe 15 to 12 SP; Arrow Shower 5x5 and a
  push of 2 cells; Improve Concentration raised; Flasher and Sandman 12 SP,
  Freezing Trap 10; Beast Bane 4 a level, on brutes and insects; Ankle
  Snare's hold by the monster's AGI, never under 3 s, items usable while
  snared, 12 SP; traps take 0.4 s to set, not on a monster's cell or
  another trap, and spare monsters standing in range when set. euRO's
  May-2004 client text has part of it (Ankle Snare "affected by enemy's
  AGI", Flasher and Freezing Trap two Traps, Beast Bane +5 a level) and
  still says Skid Trap slides 1 to 5. So euRO had the patch by May 2004;
  whether it came with New Comodo (January 2004, our baseline) is open, and
  the game keeps mod0631's values from before it.
* *Still a draft:* the guide says Venom Splasher marks the target with a
  timer, bursts when it is under a third of its HP, and splashes a quarter
  of the damage on 3x3 cells; that Ice Wall loses durability by the damage
  it takes; that Sandman also puts those near the victim to sleep. The
  first two are not built; Sandman's is (2026-11-11), on the cells around
  the victim, a reach nothing gives. Venom Splasher's burst stays out: no
  period source says how long the timer runs. It gives no chance for Flasher and says nothing
  on arrows or on what counts as a poisonous attack.
**Found by playing the Hunter and the Wizard in the client (2026-11-13):**
* *Sightrasher* did nothing (mod0631 has no code; 817 lays out its cells
  and never sets them). Drafted: needs Sight and ends it (817's ST_SIGHT,
  jAthena 2004-05-07), MATK 120% to 300% (817 `battle.c:3178`, the guide),
  pushed back 5 cells (jAthena's skill_db), on the eight lines around the
  caster, reaching 1 cell at levels 1-2 to 5 at 9-10 (the guide's "Range +1
  cells" column; jAthena's fixed 41-cell pattern reaches 5 at every level).
  The eight lines and the reach are ours; the client draws the burst.
* *Frost Nova* draws nothing in this client when cast on a target, hit
  around it or not (011a and 0117 tried by hand): mod0631 and 817 comment
  out its effect packet. The guide: "freezes all enemies around a target,
  however the target is not frozen", as built. jAthena (2004-05) makes it
  a ground unit around the target. What did the official client show?
**Impact:** Hunter, Assassin, Blacksmith and Wizard play.
**Look:** jAthena's history (`refs/jathena`, `git log -S` on each skill
name gives the date and often a source note), 817 and eathena-v0.5.2, the
client's item text for arrows (`idnum2itemdesctable.txt`).

### 23. Monster skills and behaviour
The monsters' skill lists are mod0631's `db/mob_skill_db.txt` (1,109
lines): its authors' reconstruction of what jRO's monsters did, with
"friend" targets and several conditions never implemented. euRO got
monster skills with "New Comodo" (January 2004). The AI is mod0631's
`mob_ai_sub_hard`: 9-cell sight for aggressive monsters, chase to 13
cells, 25% chance to switch target, looters, assist within 13 cells.
**Seen in the client (rig, 2026-11-11):** an Ant Egg hatching
(`NPC_METAMORPHOSIS`, a two-second cast) had "Unknown Skill !!" over it.
The exe holds no `NPC_` name and `skillnametable.txt` none either, so
every monster-only skill with a cast time did that. Such casts are now
sent without the casting bar (DEVIATIONS). Open: what an official server
sent for a monster-only cast, and what a client of January 2004 showed.
**Seen again (rig, 2026-11-11, the Knight test's goblins):** the shout
also comes with the skill's result. The goblins' elemental attacks
(`NPC_WINDATTACK`, `NPC_FIREATTACK`... 184-188, no cast time) go out as
0x01de skill damage with that id, and the client puts "Unknown Skill !!"
over the goblin each time one lands; so does every monster with such an
attack. Left as it is: whether Aegis sent these as skill damage (named or
not) or as plain attacks (0x008a) is the same open question.
**Round 9 (2026-11-14): a whole layer is missing.** unknown13's Monster
Skill FAQ (GameFAQs 29608, April 2004; a pRO player writing up "the data
on iRO") lists every monster skill. `research/mob_skills_faq.py` compares
it with our table: of 325 (skill, monster) pairs, 209 are in ours, 96 are
not, 20 name monsters it cannot match. mod0631 gives **no** monster
Endure, Increase or Decrease Agility, Piercing Attack, Mental Breaker,
Napalm Beat, Status Recovery or Pneuma; the FAQ has each on 3 to 11
monsters, and quotes iRO: "All boss monsters will use the Two-Hand
Quicken, Increase Agility, Guided Attack, and Piercing Attack depending on
the probability", "All boss monsters will Teleport if they are attacked
from beyond their attacking range". This fits New Comodo's "added monster
skills" (January 2004) arriving after mod0631's table (December 2003).
**Added (2026-11-14):** 93 of the missing pairs, in
`data/db/mob_skills_faq.toml` (DEVIATIONS.md has the how; each row names its
tier and source). The unmatched names were settled by hand
(`tools/import/mob_skills_faq.tsv`, column 5); "Goblin (Hammer)" and the like
by the weapon each `goblin_N`/`kobold_N` sprite carries. Found on the way:
mod0631's element changes never ran (mended, DEVIATIONS.md), and its numbers
for them are the reverse of their names. Still open:
* **The rate scale.** mod0631 rolls a row's rate against 1000 (`mob.c:2168`);
  817, eAthena 0.5.2 and jAthena from //0822 (2004-02-20, when Kalen's monster
  skill database went in) against 10000, with the same numbers for the
  rows they share: every monster skill there fires ten times less often.
  rAthena's numbers are five times jAthena's (median), so by rAthena's
  later reading mod0631's monsters use their skills twice as often as
  they should, and jAthena's five times too seldom. The game keeps
  mod0631's roll. A period player's word on how often a monster cast (a
  Marc's or a Hydra's skill per minute) would settle it.
* **The boss rule** is built (2026-11-14, DEVIATIONS.md): every MVP gets the
  four skills, and a Teleport on a hit from someone it cannot walk to
  (rAthena's "rudeattacked"). Open: whether "beyond their attacking range"
  meant only that (a hunter could then shoot a boss that can walk up to
  him), and what chance the four skills had. A period account of a boss
  teleporting away from an archer would settle the first.
* **47 rows are guesses** (the commonest row of that skill on other
  monsters, as no table has the pair: tables after 2004 were reworked).
  The Status Recovery rows target "friend", which this AI skips, as
  mod0631 skipped every friend row.
* **Not placed:** Detoxify (Obeaune), Lazy Attack (Merman), and the FAQ's
  "Grimtooth" as a monster (Maximize Power, Piercing Attack).
**Impact:** how every fight feels; `data/db/mob_skills.toml`,
`data/db/mob_skills_faq.toml`.
**Look:** kRO notices on monster skills (2003); period monster databases
that list skills (RODataZone, RO Empire); later official data is tier
later but shows which monster has which skill at all.

### 24. Turtle Island's fare, and mod0631's NPCs we lack
mod0631 (`conf/npc_turtle.txt`, "Turtle Quest! From jRO") has an old man
in Alberta's inn who sends you to the sailor Gotanblue (`alberta
247,122`), 10000z to the island's beach (`tur_dun01 157,39`), a scholar
and a logbook on the quay that start one of two errands among four
objects on the island (five Bananas, or a Red Gemstone), and the sailor
waiting on the beach to bring you back. That chain is in the game
(`scripts/npc/turtle_island.lua`), in our words. 817's sailor, who took
anyone to the mouth of the cave for 800z with no way back, no longer
sells the trip. iRO announced the island on 2003-12-09 for 2003-12-16,
"with new headgears and new quests" (round 4, T1-001): that there were
quests is all the international servers tell us. On euRO the island's two
patch files are entries 400 and 401 of the patch list, after "New Comodo"
(2004-01-09) and before the patch of 2004-01-15
(`research/notes/euro-history.md` §5), so the island was there in our
period. Its maps are not in the CD's base GRF: they come with the same two
files (`2003-04-30gdata_turtle_ph`), which the launcher merges.
Other NPC files mod0631 loads and 817 has little of: the cursed swords
(`npc_event_ma_sword.txt`: a gloomy man at `pay_fild08 218,283`, Gatu at
`prt_in 163,12`, and three rumour-tellers 817 has as passers-by, for
Mysteltain, Executioner and Grimtooth) and the doll child of Alberta
(`npc_event_doll.txt`, `alberta 117,134`; 817's "Kid" one cell north).
**Both are in the game** (2026-11-11: `scripts/npc/cursed_swords.lua`,
`doll_child.lua`, DEVIATIONS). mod0631's smith asks for one list and takes
another for two of the three swords; we take what he asks for, which a
pRO guide of March 2004 confirms (round 11, below).
**Not ported, on purpose:** the two arrow makers in Payon
(`npc_event_arrow.txt`, `payon 98,57` and `100,121`). The file's own
header says they are its author's ("huge") relief for archers "until the
arrow-making skill is done", an event rather than a plain shop, with
prices "made up". They are not from jRO, and Arrow Crafting works here.
**Impact:** the fare (jRO's zeny amounts are not always the international
ones: refine fees differ tenfold); three item quests.
**Look:** who took a player there on iRO or euRO and for how much
(RODataZone's map and quest pages, player guides of early 2004); the new
headgear quests the notice speaks of; whether the doomed-sword smith's
lists (five Cursed Rubies, ten Blades Lost in Darkness, one Young Twig) are
what an official server took.
**Round 11:** they are, on pRO: blood3d's mixing-quest FAQ (GameFAQs
29037, v1.01 of 2004-03-06) gives all three lists, item for item what our
smith takes (it names Grimtooth's ten "Bloody Edge", probably a slip for
Blade Lost in Darkness). The same guide has Alberta's Boy's Cap and Antler
quests (817's Grampa and Cherokee, same items) on 2004-02-28 and pRO's
Turtle Island on 2004-03-31, so on pRO those hats were not the island's.

### 25. What this client can show of the 2-2 jobs
The exe's packet table (`data/client/packet_lengths.tsv`, 2003-12-18) has
no 0x01cd to 0x01d2 and no 0x01e1. In kRO's numbering those are Auto
Spell's list and choice, Devotion's lines, the spirit spheres (both
forms), Blade Stop and the combo delay. OpenKore's table for iRO of
2003-12-16 leaves out the same six (`research/notes/emulators.md` §4), so
this is the international client of that month, not a gap in our
extraction: none of the ids occurs as a constant inside the table's
function. Of the 2-2 skills the client ships the icons (109 sprites under
`data\sprite\아이템\`) and SP amounts for thirty of them
(`leveluseskillspamount.txt`), but no names and no descriptions:
`skillnametable.txt` and `skilldesctable.txt` end with the 2-1 jobs
(checked 2026-11-11; an earlier note here said it ships their text). Nor
does the exe: `Ragexe.exe` holds the Aegis names of the skills up to the
2-1 jobs, the quest skills and the guild skills (`KN_BOWLINGBASH` is
there) and not one `CR_`, `MO_`, `SA_`, `RG_`, `BA_`, `DC_` or `BD_` name.
**Seen in the client (rig, 2026-11-11):** a Crusader is drawn (on a Peco
Peco too); the skill window lists none of the 2-2 skills the server sends
in 0x010f, so they cannot be picked or put on a hotkey; a cast started by
the server shouts "Unknown Skill !!" over the character; the status icons
of Guard (58), Shield Reflect (59), Providence (61) and Defender (62) are
drawn. Shots: `data:shots/2-2/`. So on this client the 2-2 skills can only
be used through `@useskill`, whatever the server does. With
`jobs2_2` on, a Monk's spheres cannot be drawn, a Sage cannot be offered
the Auto Spell list, and Devotion has no lines.
**Impact:** how the 2-2 skills are drafted (ROADMAP §3): spheres and
combos kept on the server and shown some other way, or the jobs left as
they are until a later client is a target.
euRO replaced the exe on 2004-04-15 and 2004-05-31 (patch list entries
444 and 462, `research/notes/euro-history.md` §5), the second nine days
before the jobs opened there: they came with a later client than ours.
**2026-11-11, the first of them:** the Crusader's Guard, Defender, Shield
Reflect, Shield Charge, Shield Boomerang and Holy Cross are drafted
(DEVIATIONS; `scripts/rules/`, test `crusader_shield_skills`). 817 and the
iRO guide's Crusader page (2004-04-11) agree on Guard's chances, Defender's
percentages and the Boomerang's reach; Shield Reflect is the guide's alone.
The three stances' icons are drawn (above); the block is not yet seen.
**2026-11-11, the second lot:** Providence, the four endows, Dispell, the
four Strips and the four Chemical Protections (DEVIATIONS; test
`endows_strips_protections_dispell`). Where the three witnesses part:
endows last 30 to 90 s in mod0631 and 20 minutes in 817 and the guide; only
the guide has an endow fail and break the weapon, a Strip weaken a monster,
and Dispell roll; 817's Dispell ends a fixed list where the guide says
"all magical effects". Open: the DEX term of a Strip's chance; whether a
failed endow really cost the weapon on a server of 2004 (the guide's word
only); which blessings Dispell took.
**2026-11-11, a later client is on this machine** (pointed out by round
5, `research/notes/round5-6-review.md`): the owner's MICO07 DVD, ripped on
2026-11-09 to `data:mico07/`, carries euRO's May-2004 installer with
`2004-05-07aRagexe.rgz` (`data:mico07/Ragexe-2004-05-07.exe`) and
`2004-05-06gdata_yuno_lang_g.gpf`. That exe has all six missing packets
and 0x01e1, and 128 Aegis names of 2-2 skills; the gpf has the 2-2
skills' names and English descriptions. Against our exe only the spawn
and idle packets (0x0078 to 0x007b, 0x01d8 to 0x01da) grew by two bytes,
and 0x014e is gone; 0x01f1 to 0x01ff are new. **Owner, 2026-11-11:** that client gets its own version of the game once
this one is complete; until then nothing is built for it, and what
matters to it is collected in `research/notes/may2004-client.md`
(packets, skill text, the 35 maps whose cells differ).
**Look:** first of all whether either of those two exes is archived: a
client that lists the skills is what the 2-2 jobs wait on, more than any
skill's numbers (answered: see the paragraph above). Then what this client does with the skills' effects (does
it have the effects and sprites for Asura Strike, the songs?); which 2-2
skills need none of the
six packets (Auto Guard, Defender, Reflect Shield, the Strips, Chemical
Protection, the Sage's ground skills and Dispell: their numbers are in the
iRO guide pages, round 4).
**Round 7 (2026-11-14): the May client's text against the drafts**
(`research/notes/round7-review.md`). It agrees on the endow that breaks
the weapon, Dispell's two gemstones, and the Coating Bottle. **It differs
on the Strips:** Strip Shield takes a monster's DEF down 15%, Strip Armor
its VIT 40%, Strip Helm its INT 40%, and Strip Weapon its ATK (no number;
`yuno_lang/data/skilldesctable.txt:1309-1330`). The draft has the iRO
guide's 10% ATK and 10% DEF for each. Re-drafted on 2026-11-14 with
euRO's numbers: jAthena of 2004-05-07 (`7d0b373`, `battle.c`) has VIT
x60/100 and DEF x85/100 too, and INT x90/100 (we take euRO's 40%).
Strip Weapon keeps the guide's tenth: jAthena's `atk2 -= atk2*90/100`
looks like a slip for x90/100. The two bytes the unit packets gained are the base level (817
`map/clif.c:544`, `630`, `855`, its packet version 4).


### 26. What `check` still lists
The world audit (`ragnarok-offline check`, 2026-11-11) walks the client's
cells from the start point through every warp and every destination a
script names. After its first fixes it still reports:
* **`tur_dun05`**: mod0631 has three monster groups and two exits there
  (`npc_warp25.txt:13-14`, to `tur_dun02` and `tur_dun01`) and nothing
  leading in. Was there a fifth Turtle Island map in play, and from where?
* **`pay_dun04 193,43`**, one of the four random arrivals from Payon Cave
  3F (`npc_warp.txt:511`), is a wall on this client's map; `192,43` is
  floor. As in Athena, the player lands on a random cell of the map
  instead. An older or newer `pay_dun04`? (rAthena has `189,43`, round 10.)
* **`prt_in` 280-285, 36-130**: four exits to Prontera on cells this
  client's `prt_in` has no room around, and no warp leads to them.
  Rooms of a later Prontera?
* **`prt_are_in 31,122` and `169,122`**, **`yuno_in03 104,115`**: warp
  tiles that cannot be stepped on.
* **Castles**: `prtg_cas02 184,44` arrives on a blocked cell (rAthena has
  the same); `aldeg_cas05 188,49`'s "165,288" was a slip for `165,228`
  (round 10, fixed in `scripts/npc/castle_warps.lua`); `prtg_cas02`'s Emperium cell `157,174` is blocked (known,
  #17).
* **The guild dungeons** `gld_dun01-04`: nothing leads in (#17).
* `job_wiz` is entered through a destination the script computes, which
  the audit cannot see: not a fault.
* `gef_fild10 54,232` and `morocc 120,39` (2026-11-13): destinations of
  817 Kafras that `scripts/npc/kafra.lua` replaces with mod0631's cells.
  The audit reads destinations per file, so a replaced NPC's still count:
  not a fault in the game.
* Scripts that warp with a variable for the map (`warp(MAP, ...)`) hide
  their rooms from the audit; write the map name out (the Priest test's
  demons showed as "out of sight" until 2026-11-13 for that reason).
**Impact:** small; each is a place a player cannot stand or reach.
**Look:** 817's and jAthena's warp files for the same cells; the `.gat`
of each map in `data.grf.original` against the merged one.

### 27. 817's first-job quests against mod0631's
Read side by side on 2026-11-11 (Thief, Merchant, Acolyte; mod0631 has
`conf/npc_job_thief.txt`, `npc_job_merchant.txt`, `npc_job_aco.txt` and the
Kafras' parts in `npc_town_kafra.txt`). 817's are ports of the same
official scripts with slips; the ones that stopped a player or gave the
wrong thing are mended (DEVIATIONS: the Merchant's parcels, the Thief's
weighing). Played in the client since: the Thief test and the Mage test
(the Mixing Machine's number box works; its recipes are in 817's Book
Shelf, `other/books.txt`). **Left as 817 has them, open:**
* *Thief.* The Comrade pays 200z + 15 per Orange Net Mushroom + 2 per
  Orange Gooey one (`thief.txt:421`, `:485`); mod0631 pays a flat 100z
  (`npc_job_thief.txt:446`) and calls it "subsidy and the mushrooms'
  price". 817 takes "99 of each" mushroom, which takes none when fewer are
  held (so they stay in the bag); mod0631 takes one of each. mod0631 has a
  separate application step and counts at most 9 Net and 20 Gooey.
* *Merchant.* Kay's wrong parcel is item 1083 in 817 and 1091 in mod0631;
  817 goes on to the confirmation with a number out of range where mod0631
  asks again. Both read the first Byalan parcel's number out as 3012685
  (it is 3318702): the official script's own slip, it seems.
* *Acolyte.* Yosuke stands at `prt_fild00 208,218` in 817 and `208,255`
  in mod0631; the second is a blocked cell on this client's map, so 817's
  is kept (iRO Wiki of 2009 has `208,218` too: round 12, grade D). 817
  saves you a few cells from each ascetic, mod0631 on the ascetic's own
  cell.
* *Mage* (817 only; jAthena's of 2004-03-02 has the same numbers): the
  Mixing Machine checks each solution's own amounts and then takes two
  Jellopy and three Fluff whatever the solution.
* *Scripts that end without a word* when the player is short of zeny or
  goods (a jump to a label the source lacks; `research/npc_lint.py` lists
  them, with the tests that decide nothing). **Mended on 2026-11-14**
  (DEVIATIONS.md), each from a period script that has the missing part:
  Christopher in Geffen (817's own line under another label; jAthena
  //0505 of 2003-10-30 for the rest: his Mini Furnace could never be
  bought, his Emveretarcon sold Phracon), the Grampa Pharmacist of Alberta
  (mod0631's `npc_event_potion.txt`: 817's "as many as I can" took the
  herbs and gave nothing for three potions), the headgear Blacksmith of
  Geffen (a misspelt label), the Swordman's Test Hall Staff (jAthena
  //0854). Comodo's Camp Ground Boy is left: his silent branch needs
  `comodo_npc`, which no script sets. Also found on the way: 817's `input`
  reads a negative number as 0, which the server now does too (its shops
  test `!= 0` and would have paid out for -10).
* *Swordman and Archer*, read against jAthena //0854 of 2004-03-02
  (`conf/npc/npc_job_swordman.txt`, `npc_job_archer.txt`,
  `conf/warp/npc_warp_job.txt`; the same official tests). Mended: the
  Archer's trunks and score, the course's pits (DEVIATIONS). Left as 817:
  the briefing Swordman speaks of a time limit (neither script has one)
  and of giving up "at the entrances of each course", where jAthena has
  three cells that ask (`sword_1-1 7,245`, `8,207`, `8,169`) and 817 has
  none (one gives up at the checkpoint staff); 817 marks the end of a
  stretch with a plain warp, jAthena with a cell that announces "point N
  passed"; the Sword Master says the test is "the right room next to me"
  (jAthena: to the left; the door Swordman stands west of him); the
  Archer guild gives 100 Arrows (jAthena: `rand(100)`); a pass needs 25
  points in 817 as its text says (jAthena's code wants more than 25).
* *Priest and Hunter.* 817's Priest test is its author's own ("Designed
  For Athena": ten true-or-false questions and a grail errand), not a
  port; see #8 for the official one. 817's Hunter NPC is a plain job
  changer.
**Impact:** small sums and one cell. **Look:** RODataZone's job pages
(late 2003) for the Thief's reward and the ascetics' cells.

### 28. Ground units placed by others, seen twice
The client keeps a list of the ids of ground units (0x01c9) whose placer is
not the player. A unit not on it is shown and added; one already on it is
taken off and drawn **hidden**. 0x0120 does not clear the list
(`0x52a9a6`-`0x52aa75`, docs/PROTOCOL.md). So when a monster's ground unit
leaves view and comes back (mapsrv.rs sends 0x0120, then 0x01c9 again), the
second showing should be invisible and the third visible. Athena does the
same, so either the official server never re-sent a unit, or this is how
traps of others stayed hidden.
**Impact:** monster-made ground skills (and any later unit of an NPC or
pet) after the player walks away and back.
**Round 10: the official servers did hide traps this way.** Forever-Zero's
guide (GameFAQs 27633) on Sandman: "It is invisible only to people who were
off screen when it was planted, or those who go off screen and come back."
iRO's Comodo patch, as it copies it: "Invisible traps are visible to only
the caster and those who were on the screen at the time the traps were
installed. Traps can also be seen using a detecting skill". "Go off screen
and come back" is the client's second showing; "off screen when planted"
fits if the server told players beyond the screen of a new trap, so that
the showing they got on coming near was their second. Open: which units
the official server treated so (a Fire Wall or Safety Wall re-sent would
vanish the same way, so it must not have re-sent those), and how far it
announced a placement. The server here re-sends every unit.
**2026-11-14, the client read whole** (docs/PROTOCOL.md, 0x01c9 and 0x01ac):
the list is cleared by every 0x0091, same map or not; hidden means neither
drawn nor given its effect, for **every** unit type, not only traps; a
placer in the player's party (session `+0xe4c`, filled by 0x00fb) is
exempt; the `isVisible` byte the server sends is never read; 0x01ac shows a
hidden unit again only for types 0x8b-0x99. So the client was built for
hidden traps, and an official server that re-sent a Fire Wall or a Warp
Portal on its second showing would have hidden it too: it must have
re-sent only traps, or nothing.
**Built:** the server sends a trap again when it comes back into sight (so
it is drawn hidden, as the patch notes say) and keeps any other unit of
another's on the client while out of sight, taking it off only when it ends
(DEVIATIONS.md); Detect sends 0x01ac for the traps within its 3 cells (test
`others_ground_units_seen_again`). Seen in the client (rig, prt_fild08): a
Pupa's Safety Wall stays drawn after walking 21 cells away and back, its
Skid Trap comes back hidden, and Detect on its cell shows it again.
**Still open:** whether the official server re-sent traps at all, or told a
whole map of a new trap so that whoever came near later got their second
showing at once ("invisible only to people who were off screen when it was
planted"). We do neither: a trap placed while the player is away is seen on
arrival. In single player only monsters place traps (Skid Trap, Claymore,
Sandman, Flasher, Freezing, Ankle Snare, Shockwave: 10 monsters), and they
do it near their target.

## Resolved

### R1. Basic Skill levels
The client's own `skilldesctable.txt` gives them: 1 trade, 2 emotions,
3 sit, 4 chat room, 5 join party, 6 storage, 7 organize party, 8 alignment,
9 first job. Tier client. Used for the Kafra storage check and the
sit/emote gating.

### R2. Training-ground and test maps under older names
The test was checking every NPC/warp cell of the scripts against the
client's `.gat`. Results: `new_1-N` = `new_zone0N`, `sword_1-1` =
`job_sword1` (207/209), `force_1-N` = `force_mapN`, `ordeal_1-1` =
`ordeal_a00`, `ordeal_1-2` = `ordeal_a02`, `priest_1-1` = `job_priest`.
`tools/import/maps.py`.

Round 1 found the client saying so itself: `data\resnametable.txt` in
data.grf redirects each newer name to the file it has
(`new_1-1.gat#new_zone01.gat#`, 143 `.gat` lines; the full list is in
research/notes/round1-review.md). Tier client. The names `job_wiz`,
`job_knt`, `job_prist`, `job_hunte` are different maps, not aliases: they
come with the Turtle Island patch files and are in the merged client
(thread 8, 2026-11-11).

### R3. Skill SP costs
The client's `leveluseskillspamount.txt` has them per level. The server
reads them from the GRF at start (tier client).

### R4. Kafra fares in the period emulators (was part of #3)
817's menus were right and its code was not: mod0631's Kafra script
(archived 2003-12-18) and jAthena's of 2004-01-31 show and take
Prontera→Geffen 1400 and Alberta→Prontera 2100. They also give what 817
lacked: a storage fee per place (30 to 80z), Prontera→Comodo 2000,
Morroc→Comodo 1500, Al De Baran→Geffen 1600 and →Mt. Mjolnir 1700, the
Kafras of Comodo, Pharos, Mt. Mjolnir and Prontera's gates, and Izlude's
save cell 94,103. kRO's notice of 2004-02-24 confirms that the dungeon
Kafras saved until that day. All in `scripts/npc/kafra.lua`; the table is
in research/notes/round1-review.md. Tier athena2003 (jRO lineage).

### R5. The arena (was #18)
Not open in Dec 2003 – Feb 2004. kRO, 2004-10-12, main servers: "after a
long reconstruction, an arena was added to Izlude". 817's arena is its own:
entered from an NPC in the middle of Prontera, on `prt_are_in` and
`force_1-N` (maps this client does ship). `[game.content] arena`, off.

### R6. Skill numbers: the client's text against the emulators (was #20)
The client's `data\skilldesctable.txt` gives per-level numbers. Some agree
with mod0631 (Mammonite, Discount, Stone Fling, First Aid, Moving HP
Recovery, Suffragium); for these it does not:

| Skill | Client text | mod0631 (used) |
|---|---|---|
| Sword / Two-Handed Sword Mastery | +5 a level (to +50) | +4 a level |
| Beast Bane | +3 a level | +4 a level |
| Two-Hand Quicken | lasts 15 s to 60 s | 30 s a level |
| Impositio Manus | +2 a level, 40 s | +5 a level, 60 s |
| Aspersio | 5 s to 17 s | 60 s to 180 s |
| Kyrie Eleison | 10 s a level, blocks 1 to 3 times | 120 s, 5 to 10 hits, a share of max HP |
| Resurrection | 1 HP, 15%, 30%, 50% | 10%, 30%, 50%, 80% |
| Charge Arrow | throws the target 4 cells | 6 cells |
| Land Mine | lasts 65 s down to 5 s | 200 s down to 40 s |
| Blast Mine | goes off after 10 s down to 2 s | 25 s down to 5 s |
| Shockwave Trap | needs 2 Traps | 1 Trap |

The iRO guide's skill pages, in their earliest captures (2004-03-27 to
2004-04-13; `research/notes/round4-review.md` says where they are saved),
give mod0631's number on every row: masteries and Beast Bane +4 a level,
Two-Hand Quicken 30 s a level, Impositio Manus +5 a level for a minute,
Aspersio one to three minutes, Kyrie Eleison 120 s and 5 to 10 hits,
Resurrection 10/30/50/80%, Arrow Repel 6 cells, Land Mine 200 s at level
1, Blast Mine 25 s down to 5 s. RODataZone's Swordman page (2004-05-02)
says +4 for the masteries too. So the text is an older state of the game
(it also has Teleport with ten levels), and `scripts/rules/` keeps
mod0631's numbers. The guide says nothing on how many Traps Shockwave Trap
takes (client text: 2; mod0631: 1; we use 1). Tier: athena2003, confirmed
for the international servers three months after our date.

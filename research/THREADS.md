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

**Goal (owner, 2026-11-09):** find the period-accurate start position and
document it. Failing that, pick the best-supported cell and list it as a
possible deviation.
**Impact:** start point, first impressions, novice rewards.
**Look:** period first-screen reports and screenshots, OpenKore 2004 logs,
kRO/jRO notices on the training ground (초보자 수련장), the iRO guide's
beginner pages. Round 1: research/agy-threads/HANDOFF.md.

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
**Open:** the item lists of the other jobs' quests side by side, starting
with Moving HP Recovery (mod0631: 200 Empty Bottles and item 1058).

### 8. Wizard (and other 2-1) job quests
817's wizard.txt is an unfinished conversion that 817 couldn't run (later
dialect, `Job_Magician`, missing gotos). We use 659's simple guild NPC:
job change at job level 40, no test. That is like 817's own Knight,
Blacksmith and Assassin. 817's Priest and Hunter do have quests.
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
Wizard test here would have to be laid out by us.
**Round 3:** nothing new. The round's Wizard rows quote jAthena //1046
(2004-09-12), again for `job_wiz`: of its three room cells, `116,171` and
`46,99` are blocked on `job_wizard`. No dated walkthrough of any 2-1 test
was found; RODataZone's Wizard and Knight pages on Wayback are empty.

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
Done in part: shops are named from 817 by cell. 18 of 80 have no English
name in 817 (none nearby, or only a Japanese one) and show "Shop".
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
**Look:** kRO notices on WoE rules; the iRO guide's guild/WoE pages; euRO
WoE news (Apr and Sep 2004: "treasure boxes and guild dungeons disabled");
period guides. Round 1: research/agy-threads/HANDOFF.md.

### 19. Lutie (Xmas) in January
Was Lutie a permanent town on euRO, or seasonal? The CD patcher carries
`2003-10-31gdata_xmas.gpf`.
**Look:** euRO notices Dec 2003–Jan 2004.

### 20. Skill numbers: the client's own text against the emulators
The client's `data\skilldesctable.txt` gives per-level numbers for many
skills. Some agree with mod0631 (Mammonite 150–600%, Discount 7–24%, Stone
Fling 30, First Aid 5 HP, Moving HP Recovery 25%, Suffragium 15/30/45% for
30/20/10 s). Others do not, and the text is partly older than the client
(Teleport with ten levels):

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

**Impact:** each row is a number in `scripts/rules/`.
**Look:** which state of the game the text describes (kRO beta? when was
each line last edited: compare the same file in later clients and in the
2003-04-30 and 2003-12-11 patch archives on the CD); iRO skill pages of
late 2003 on Wayback (RODataZone's job pages list skill tables); kRO
notices on skill changes in 2003.

### 21. Ailments: which emulator is nearer the official servers
mod0631 (2003-12): fixed chances, no resistance from the target's stats,
placeholder lengths, no Curse, poison without HP loss. eAthena 817
(2004-02): chances and lengths fall with MDEF / VIT / INT / LUK, lengths
per skill level in `cast_db.txt`, poison takes HP down to a quarter, Storm
Gust freezes on the third hit. We follow 817 (DEVIATIONS.md). Neither says
where its numbers came from. 817 sends opt1 = 6 for the five seconds of
turning to stone; we keep it server-side and send nothing until the stone
sets: does the Dec-2003 client draw opt1 6 (look for it in the exe)?
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
**Impact:** Hunter, Assassin and Blacksmith play.
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
**Impact:** how every fight feels; `data/db/mob_skills.toml`.
**Look:** kRO notices on monster skills (2003); period monster databases
that list skills (RODataZone, RO Empire); later official data is tier
later but shows which monster has which skill at all.

### 24. Turtle Island: how one got there, and mod0631's NPCs we lack
817's Alberta sailor (`alberta 189,151`) takes anyone to Turtle Island for
800z, in one menu with the Sunken Ship and Izlude. mod0631
(`conf/npc_turtle.txt`, 1,084 lines, Dec 2003) has what reads like the
official quest: an old man in Alberta (`alberta_in 23,104`) tells the
story, after which the sailor Gotanblue (`alberta 247,122`) takes 10000z
for the trip to `tur_dun01 157,39`; a scholar and a logbook on the quay
(`alberta 248,89`, `248,94`) start a second quest among four objects on
the island, and a sailor there (`tur_dun01 165,29`) brings you back. We
have 817's sailor; mod0631's chain is not ported. The island's maps are
not in the CD's base GRF: they come with its `2003-04-30gdata_turtle_ph`
patch, which the launcher merges.
Other NPC files mod0631 loads and 817 has nothing of: the cursed swords
(`npc_event_ma_sword.txt`: a gloomy man at `pay_fild08 218,283`, Gatu at
`prt_in 163,12`, Nine at `izlude_in 160,129`, for Mysteltainn, Executioner
and Grimtooth), the doll child of Alberta (`npc_event_doll.txt`, `alberta
117,134`) and two arrow makers in Payon (`npc_event_arrow.txt`, `payon
98,57` and `100,121`).
**Impact:** whether Turtle Island is a 800z ferry or a 10000z expedition
with a story; three item quests.
**Look:** was Turtle Island open on euRO in Dec 2003 to Feb 2004 (iRO:
2003-12-16), and the fare; RODataZone and iRO guide pages on the island and
on the cursed-sword quest; then port mod0631's files.

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
research/notes/round1-review.md). Tier client. The later names `job_wiz`,
`job_knt`, `job_prist`, `job_hunte` are different maps, not aliases.

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

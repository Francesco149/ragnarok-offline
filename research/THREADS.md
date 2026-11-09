# Research threads

A running list of open questions whose answers would make the reconstruction
more exact. Each one says what is uncertain, what depends on it, and where
to look. Add new ones at the end, keep the numbers stable (docs refer to
them), and move a thread to "Resolved" with its answer and source when done.

Tags: **impact** = what changes if we learn the answer. **look** = where to
dig first. Background research: `research/notes/` (euro-history.md,
mechanics-sources.md, emulators.md).

Research rounds: round 1 (Antigravity, then a Claude pass to confirm) is
briefed in `research/agy-threads/HANDOFF.md`; its report lands in the same
directory.

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

### 3. Kafra fares and save cells before 2004-02-24
817's menus and code disagree (Prontera→Geffen 1400 shown, 1800 charged;
Alberta→Prontera 1800/2100). kRO replaced the fare table on 2004-02-24.
iRO's early-2004 fares are unknown. 817's Izlude "Save inside City" saved
nothing; we use 91,105.
**Impact:** `scripts/npc/kafra.lua`.
**Look:** Fandom "RO Patch (2004 Feb. 24)": does the Korean text list the
old fares? iRO notices on Wayback (`allim/content.asp?bbs=tNotice`).
Screenshots in period guides.

### 4. Kafra points and the Kafra Pass in Dec 2003
817 had a Kafra-points "Sweet Potato" exchange (broken) and a Kafra Pass
seller in the Al De Baran HQ. kRO removed the voucher menu on 2003-05-20.
**Impact:** whether the Aldebaran Kafra HQ NPCs should offer anything.
**Look:** kRO notices 2003 (S1/S2 in mechanics-sources.md), iRO guide
pages (2004).

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

### 8. Wizard (and other 2-1) job quests
817's wizard.txt is an unfinished conversion that 817 couldn't run (later
dialect, `Job_Magician`, missing gotos). We use 659's simple guild NPC:
job change at job level 40, no test. That is like 817's own Knight,
Blacksmith and Assassin. 817's Priest and Hunter do have quests.
**Impact:** whether the 2-1 jobs should have their tests; see #5.
**Look:** eathena-v0.5.2 (2004-05) `npc/jobs/*.txt`; period
walkthroughs (#2).

### 9. Refining details
Phracon price (iRO page 2000z vs data 200z), armor DEF per refine (1, 5 or
0.7 across emulator versions), refiner dialogue, and what a failure does to
cards.
**Impact:** shops, `rules/status.lua` refine bonuses (not applied yet),
`scripts/import/npc/other/refine.lua`.
**Look:** iRO *Refine System* page (2004-06, Wayback), kRO notices.

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

### 13. Dyes and palettes
817's dyer needs a 77-palette client patch. Which clothes dyes existed in
Dec 2003? This client ships 5 clothing palettes per job and sex (the default
plus 4) and 9 hair colours per hairstyle (`data\palette\`, tier client).
**Impact:** a hand-written dyer (`scripts/npc/dye.lua`, not written yet).
**Look:** the client GRF's `data\palette\` (tier client); 659's
`npc_dye.txt`; kRO 2003 notices.

### 14. Pets on euRO's closed beta
The closed beta "includes Comodo and Q-Pet". Which monsters were tameable,
with which items, and the pet food and accessory list.
**Impact:** the pet system (ROADMAP).
**Look:** iRO `qpetsystem.asp` (2004-03, 24 tameable), 817 `db/pet_db.txt`.

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
**Impact:** castle treasure, guild dungeons, guild-only drops.
**Look:** kRO notices on WoE rules; the iRO guide's guild/WoE pages; euRO
WoE news (Apr and Sep 2004: "treasure boxes and guild dungeons disabled");
period guides. Round 1: research/agy-threads/HANDOFF.md.

### 18. Izlude Arena (time attack) in iRO 2003
817 ships the Izlude arena NPCs. Did iRO have it in Dec 2003?
**Look:** kRO notices (arena added when?), iRO guides.

### 19. Lutie (Xmas) in January
Was Lutie a permanent town on euRO, or seasonal? The CD patcher carries
`2003-10-31gdata_xmas.gpf`.
**Look:** euRO notices Dec 2003–Jan 2004.

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

### R3. Skill SP costs
The client's `leveluseskillspamount.txt` has them per level. The server
reads them from the GRF at start (tier client).

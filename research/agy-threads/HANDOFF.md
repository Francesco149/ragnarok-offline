# Handoff: deep research on the open threads (round 1)

For: an independent deep-research agent (Antigravity). Written 2026-11-09 by
the session that built the NPC port. A second session (Claude) will later
re-check everything you report, load each source again and fold the results
into the project. So **say exactly what you loaded and quote it**. A finding
nobody can re-load is worth nothing to that second pass.

Write only inside `research/agy-threads/`. Do not edit anything else in the
repository and do not commit: other sessions work on this repo in parallel.

## 1. The project in five lines

`ragnarok-offline` is a single-player server for one preserved Ragnarok
Online client: the **euRO** (European RO, operator Burda/BHI) client CD of
December 2003, `Ragexe.exe` built 2003-12-18. The goal is the game as a euRO
player met it in the **closed and open beta, Dec 2003 – Feb 2004**. Nothing
of euRO's server survives (it ran Gravity's own server, "Aegis"). So every
server-side value is reconstructed from period sources and tagged with how far
it can be trusted (`docs/PROVENANCE.md`). The main sources are Athena-family
emulators of 2003–04, whose NPC scripts we translated. Where we deliberately
do something else, it is listed in `docs/DEVIATIONS.md`. Open questions are
`research/THREADS.md`, and this brief is about them.

Read first, in the repo:
* `research/THREADS.md`: the threads, numbered (keep the numbers);
* `research/notes/euro-history.md`: euRO's history with sources, incl.
  the "New Comodo" update and the Sept-2004 WoE notes;
* `research/notes/mechanics-sources.md`: the source catalogue (kRO notice
  board, Fandom kRO patch translations, iRO site on Wayback, jRO, emulator
  archives) and known disagreements. **Reuse its source ids (S1–S18)**;
* `docs/DEVIATIONS.md`: what we already decided and why.

Glossary: **817** = eAthena release of 2004-02-19 (our main NPC source),
**659** = Athena English mod of 2004-01-01, **mod0631** = Athena of
2003-12-18 (our numbers source), **kRO/iRO/jRO** = Korean/international/
Japanese official servers, **2-1 / 2-2** = the two sets of second jobs,
**WoE** = War of Emperium (guild castle sieges), **New Comodo** = euRO's
update of January 2004 (2nd-job quests, monster skills...).

## 2. Rules

* Read-only web research. Never post, reply, register to post, vote or send
  messages. Never solve CAPTCHAs.
* **Never use leaked Gravity server files** (Aegis binaries, scripts, DB
  dumps), even as a reference. If a page offers them, note that it exists and
  move on.
* Do not download executables, client data or disc images.
* Every claim gets a tag:
  * **[READ]**: you loaded the page and quote it;
  * **[SNIPPET]**: search-engine text only;
  * **[INFER]**: your reasoning.
* Every value gets **region** (kRO/iRO/jRO/euRO/other) and **date** (when it
  was true, or when the page was written). We care about **2003-07 to
  2004-06**. Later sources (iRO Wiki Classic, rAthena `pre-re`, RateMyServer)
  are allowed but tag them **later** and say why you think the value held
  in 2003.
* Wayback citations carry the capture timestamp:
  `https://web.archive.org/web/<timestamp>/<url>`. CDX queries per host or
  prefix work; whole-domain CDX queries often time out (504).
* Korean and Japanese sources are welcome and often the best ones. Quote the
  original text and give your translation.
* A page you could not load is not evidence: list it as an unverified lead.

## 3. Deliverables

1. `research/agy-threads/REPORT-antigravity.md` (format in §6).
2. `research/agy-threads/evidence/<thread>/`: plain-text excerpts of what
   you quote (URL, timestamp, the passage), and screenshots only where the
   content is an image (period screenshots, scanned magazine pages).

## 4. Priorities

Spend most of the effort on **P1**. Go to P2 and P3 only after giving each P1
thread a real try.

| Priority | Threads |
|---|---|
| P1 | #17 WoE in single player (expanded below), #6 start cell (expanded below), #5 2-1 job changes before New Comodo, #1 closed-beta rates, #2 where official 2003-04 English NPC text survives, #3 Kafra fares before 2004-02-24 |
| P2 | #4 Kafra points/pass, #7 quest-skill NPCs in iRO Dec 2003, #8 Wizard (and other 2-1) quests, #9 refining details, #12 map flags, #14 pets on euRO, #18 Izlude arena, #19 Lutie in January |
| P3 | #10 monster spawns, #11 shop names, #13 dyes, #15 euRO-specific content, #16 ordeal maps |

## 5. The threads

Each thread lists what we already know (much of it read locally from files
you cannot see, so take it as given), where to look, and what a good answer
looks like.

### #17 WoE and guild castles in single player (P1)

**Question.** War of Emperium needs guilds and opponents, which a single
player doesn't have. We need to know:
1. **What WoE and castle ownership gated** on the official servers in
   2003–04 (kRO; iRO; euRO once WoE started in its open beta).
2. **Whether it can be repurposed** into a single-player activity that
   stays close to how it worked.
3. **What content a single player would otherwise never see**, so that we
   can give access to it some other way if a repurposed WoE isn't
   convincing.

**What we know.**
* euRO had WoE from the open beta, Sunday 18:00 CET in April 2004, with a
  GM attack on the castles. In September 2004, WoE was suspended over spell
  problems, there was a test WoE on a Wednesday, and **"treasure boxes and
  guild dungeons disabled"**. WoE on Chaos was back on 2004-09-22.
  Sources: euro-history.md [W14][W28][W29].
* The client has all 20 castles (`aldeg_cas01-05`, `gefg_cas01-05`,
  `payg_cas01-05`, `prtg_cas01-05`), the castle-town fields (`alde_gld`,
  `pay_gld`, `prt_gld`; Geffen's is `gef_fild13`), the four guild dungeons
  `gld_dun01-04` and `guild_vs1-5`. They come from the client's
  Turtle-Island/Juno patch files.
* Our period data (mod0631, 2003-12) has the castle warps (17–18 per
  castle, counting the castle's internal ones), and monsters in the guild
  dungeons, mini-bosses and an Eddga included (8 h respawn). **No warp
  leads into a guild dungeon.**
* 817's castle scripts (2004-02) have:
  * flags, a castle Kafra and lever "switches" (one warps into
    `gld_dun01` with **no ownership check**);
  * per-castle events: the Emperium spawns at WoE start, and breaking it
    hands the castle over.

  817 has no treasure-room logic, and its steward NPC is commented out.
  `npc/mobs/gvg.txt` spawns monsters inside the castles (we haven't
  checked under which condition).
* The monster database has 40 "Treasure Chest" monsters (ids 1324–1363).
  Their drops include Yggdrasil Berry and Seed, Freyja's Jewel, Thor's
  Gauntlet, Angel Helm, Cape of Old Marquess and Oridecon.
* We import none of the castle NPCs today. Castles are empty and
  walkable; guild dungeons are unreachable.

**Look.**
* kRO notices on WoE/castle rules 2003 (Fandom "RO Patch" pages, S1/S2).
* The iRO official guide's guild and WoE pages (S3; e.g. `game/guild*.asp`,
  `game/woe*.asp`, `game/agit*.asp`; use a CDX search).
* euRO news and FAQ pages on castles (2004).
* Period community guides (GameFAQs 2003–04, RO Empire, the Doddler
  calculator site), and period OpenKore configs.

Specifically:
* what the treasure room gave and when (daily? how many boxes? by castle
  investment?), and the treasure chest drop lists;
* who could enter guild dungeons and when, and what was special about them
  (monsters found nowhere else, cards);
* castle investment (economy/defense), castle Kafra perks, guardians;
* guild skills, and whether any needed a castle;
* whether any official or period-community **PvE or solo** variant of castle
  play existed (NPC-held castles, "guardian" sieges). Later official
  variants (e.g. kRO's later "Training Edition") count as **later**
  inspiration only.

**Done when.**
* A table of every castle-gated benefit, with region, date and source.
* A list of guild-dungeon monsters and drops that exist nowhere else in the
  2003-12 data. Check it against `data/db/mobs.toml` and
  `scripts/import/spawns.lua`, which you *can* read.
* 2–4 single-player design options, each saying which parts are period
  behaviour and which are invention. We decide, not you; give the trade-offs.

### #6 The start cell (P1)

**Question.** Where did a new character appear in Dec 2003 (map and cell),
and is that cell on this client's older-named training ground? If no period
source pins it, which cell is the best-supported choice?

**What we know.**
* The client's training ground is `new_zone01-04`. The Athena scripts' maps
  `new_1-1..4` fit it: 30 of 32 NPC and warp cells are walkable on the
  client's maps. Two NPC cells are blocked: the Bulletin Board at
  `66,114` and a Helper at `new_1-4 100,29`. In the real client, the gate
  Guard at `117,107` is drawn behind a castle tower. So the layout is close
  to kRO's, but not identical.
* **Every** Athena config of the era (601, 631, 641, 659, 715, 817, eAthena
  0.5.2) says `start_point: new_1-1.gat,53,111`. It is in the first jAthena
  git snapshot (//0505, 2003-10-30). The Athena changelog (`Readme-jap`)
  dates it:
  * `//0305`: 「新規PCの位置を初心者修練場に変更。」 ("moved new
    characters to the novice training ground");
  * `//0307`: made it configurable, with `new_1-1.gat,53,111` as the
    example.

  So 53,111 is an **emulator choice of mid-2003** whose origin we don't
  know. rAthena still uses it today, which says nothing about where it came
  from.
* From `53,111` a character walks east across a long bridge to the castle
  where the Training Ground Chief stands (`157,124`).

**Look.**
* Period player reports of the first screen: guides ("when you create your
  character you start at…"), forum posts, screenshots with coordinates
  (`/where` output), OpenKore 2004 logs or configs.
* kRO notices about the training ground (초보자 수련장): when it was
  introduced or redesigned, and whether `new_zone` became `new_1-x` there.
* jRO notices.
* The iRO guide (S3) "basic system" and "beginner" pages.
* Video captures (2003–04) are fine as [READ] if you can cite timestamp and
  frame.

**Done when.** Either an attested cell with source and date, or a short list
of candidates with the evidence for each. Also answer:
* Was the training ground the start on iRO/euRO in Dec 2003, or did new
  characters start in a town?
* What did the official training ground flow look like then (NPCs, tests,
  exits to towns, rewards)?

### #5 2-1 job changes before New Comodo (P1)

**Question.** euRO's New Comodo update (Jan 2004) lists "2nd-job quests" as
new. How did a closed-beta player (Dec 2003) become a Knight, Priest,
Wizard, Blacksmith, Hunter or Assassin? A plain job-change NPC at job level
40? No way at all?

**What we know.** Athena 659 (2004-01-01) has only simple 2-1 guild NPCs:
"Change a Job into a Knight", job level 40, no test. 817 (2004-02-19) adds
the Priest and Hunter quests; its Wizard quest is unfinished. iRO got the
Knight/Assassin/Hunter quests with Turtle Island (2003-12-16). An iRO
Sakray notice of 2003-10-07 mentions the 2-1 job-quest update slipping.

**Look.** euRO forum `viewtopic.php?t=1791` (New Comodo) and the euRO news
of 2004-01-08/18; the iRO notices of 2003-10 to 2004-01; kRO notices on
when 2-1 quests replaced instant job change.

**Done when.** Per region and date: how 2-1 job change worked, with sources.

### #1 euRO closed-beta rates (P1)

**Question.** The closed beta had "higher experience". By how much? Also
any stated rates for the open beta and commercial service.
**Look.** euro-history.md §6: the closed-beta forum topics have no
captures; try the euro-ro.net community forum captures of 2004 page by
page, German fan sites `rns.gamona.de`, `ragnarokonline.de`,
`ragnarok.gamona.de` (2003-12 to 2004-02), Italian clarence.com.
**Done when.** A number with a source, or a documented negative result.

### #2 Where official English NPC text of 2003–04 survives (P1)

**Question.** Our dialogue is the emulator authors' (often fan-translated
or paraphrased). Which sources preserve **Gravity's own English NPC
text** from 2003–04 (iRO; euRO used English too)?
**Look.**
* GameFAQs RO guides of 2003–04 (job quests often quote NPCs);
* iRO forums and bug board captures (S4 `community/bugcontent.asp`, about
  1,400 captures, unread);
* fan sites (ro.doddlercon.com, roempire.com guides);
* OpenKore 2004 logs;
* period screenshots and videos.

**Done when.**
* A catalogue of sources, each with era, coverage (which NPCs) and access.
* Two or three worked comparisons of period official text vs our text. Use
  the Kafra greeting and the Prontera NPC "Tono": our version is in
  `scripts/import/npc/cities/prontera.lua` and `scripts/npc/kafra.lua`.

### #3 Kafra fares before 2004-02-24 (P1)

**Question.** The official teleport fares (and the storage and cart fees) in
Dec 2003 – Feb 2004. kRO replaced the fare table on 2004-02-24. iRO's fares
then are unknown.

**What we know** (817's menus; the code sometimes charged differently):

| From | Destinations (zeny) |
|---|---|
| Prontera | Izlude 900, Geffen 1400 (code took 1800), Payon 1700, Morroc 1800, Orc Dungeon 1700 |
| Izlude | Prontera 900, Payon 1700, Geffen 1800 |
| Geffen | Prontera 1400, Izlude 1800, Al De Baran 2100, Morroc 2200 |
| Payon | Alberta 1300, Prontera 1800, Morroc 1800 |
| Morroc | Prontera 1800, Payon 1800, Alberta 2200, Geffen 2200 |
| Alberta | Payon 1300, Prontera 1800 or 2100 (two Kafras disagree), Morroc 2200 |
| Al De Baran | no teleport service in 817 |

Storage costs 30z and a cart 800z.

**Look.** Fandom "RO Patch (2004 Feb. 24)": does the Korean text give the
*old* fares? iRO notices, iRO guide pages (S3), period guides with fare
tables, OpenKore `tables/` if they carry fares.

**Done when.** A fare table with region, date and source per value.

### P2 threads

* **#4 Kafra points and Kafra Pass, Dec 2003.** Did they exist on iRO/euRO?
  kRO removed the voucher menu on 2003-05-20.
* **#7 Quest-skill NPCs.** Which platinum/quest skills (First Aid, Play
  Dead, Arrow Crafting, Charge Arrow, Back Slide, Pick Stone, ...) were
  obtainable on iRO/euRO by Jan 2004, and from which NPC where?
* **#8 Wizard (and other 2-1) quests.** Content of each 2-1 job test as
  of early 2004, if they existed (see #5).
* **#9 Refining.** Phracon 200z vs 2000z; armor DEF per refine level;
  refiner NPC text; what a failed refine does to cards.
* **#12 Map flags.** Which maps forbade teleport, memo or save, and which
  had no death penalty, officially in 2003–04. We apply mod0631's set;
  817 adds about 170 more.
* **#14 Pets on euRO.** The closed beta "includes Comodo and Q-Pet". The
  tameable list, taming items, food and accessories (the iRO guide's
  `qpetsystem.asp` of 2004-03 lists 24). We use mod0631's 25.
* **#18 Izlude Arena.** Did iRO/euRO have the time-attack arena in
  Dec 2003?
* **#19 Lutie.** Permanent town or seasonal, on euRO in Dec 2003 – Jan 2004?

### P3 threads

#10 monster spawns, #11 shop names (18 of 80 shops are still unnamed),
#13 dyes, #15 euRO-specific content, #16 ordeal maps: as in THREADS.md.

For #13: the client ships **5 clothing palettes per job and sex** (default +
4) and **9 hair colours** per hairstyle. Find which dye colours an NPC sold
in 2003 and for what.

## 6. Report format

`REPORT-antigravity.md`:

1. **Summary table:** thread → short answer → confidence
   (high/medium/low/none) → key sources.
2. **One section per thread you worked on**, with:
   * findings (tagged, with region and date);
   * the answer as you see it;
   * what is still unknown;
   * the queries and sites that would come next.
3. **New threads** you think we should open (question, why it matters,
   first source).
4. **Negative results:** where you looked and with which queries, per
   thread. This saves the second pass from repeating them.
5. **Where you think our notes are wrong**, and why.

Keep quotes short and exact; put long excerpts in `evidence/`.

## 7. Phase 2 (for the Claude session that follows, not for you)

The owner will start a separate Claude session once this report exists. That
session:
* re-loads every [READ] source and confirms or rejects it;
* chases the "what would come next" leads;
* merges confirmed answers into `research/THREADS.md` (moving them to
  "Resolved") and `research/notes/`;
* proposes, but does not make without the owner's word, the game changes
  that follow (start cell, WoE design, fares). Each such change gets a line
  in `docs/DEVIATIONS.md` or `docs/PROVENANCE.md`.

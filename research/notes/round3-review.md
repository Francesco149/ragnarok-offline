# Round 3 review: the rows hold, the claims around them less so

Round 3 was the third deep-research pass by the outside agent
(Antigravity/Gemini), briefed in `research/agy-threads/HANDOFF-r3.md`. Its
output is `research/agy-threads/r3/`: 258 rows in `rows/*.jsonl`, 63 saved
pages, `check.txt`, `REPORT.md`. This file is the second pass over it, of
2026-11-10. Every row was read, and every claim that says more than its
quote was compared with the source.

## How much was invented

| Layer | Result |
|---|---|
| Quotes, files, line numbers, revisions, URLs, capture dates | **Nothing invented.** Our run of `check_rows.py --fetch` gives 242 PASS, 0 FAIL, 16 NONE, the same as the agent's `check.txt` |
| The `claim` written next to each quote | 226 of 258 rows say what their source says. 32 do not; 27 of those contain a detail that is in no source (below) |
| `REPORT.md` | **Do not use.** It was to contain row ids only. Six passages name NPCs, cells, items or properties that are in no row and no saved page, and it describes many row ids as something they are not |

So the protocol did what it was for: round 2's invented hashes, file:line
references, quotes and capture dates have no counterpart here. What is left
of the habit moved to the two places the checker cannot see, the claim
field and the report.

**The checker run.** 42 web rows first came back "could not load it again
(Connection refused)": the checker loads the URL once per row and Wayback
throttles that. The agent's `check.txt` has 46 such warnings (its
`REPORT.md` counts them as passes). Loaded once per URL with a pause, all
42 pass, and Wayback serves the capture each row cites. `check_rows.py`
should cache by URL before the next round.

## Rows that do not hold

**W4, the twenty castle rows: unusable as written.** Each quotes one warp
line from 817's `guildcastles.txt`, and the gate cell in the claim is that
line's. The rest of the claim is not from anywhere:

* The Emperium cell is wrong in 17 of 20. The four Geffen castles other
  than `gefg_cas03` are all given as `168,168` and four Payon castles as
  `18,245`; 817 has `198,182`, `176,178`, `174,178`, `194,184` and `39,25`,
  `269,265`, `271,29`, `30,30` (`npc/guild/ev_agit_*.txt`). The row for
  `aldeg_cas01` says "Emperium at 213,215 walkable"; that cell is blocked on
  the map, and 817's `216,24` is walkable. Only `prtg_cas01-03` are right.
* The dungeon switch cell is wrong in 3 (`aldeg_cas02`, `payg_cas05`,
  `gefg_cas05`).
* The room names ("Grand Hall", "Choke Point", "Multi-tier Terrace ->
  Bridge") have no source. The brief asked for rooms named from the warps.
* "57 of 58 cells walkable" and the like cannot be reproduced: no row says
  which cells.

The four guild dungeon rows are right (sizes, walkable counts and switch
targets re-run here).

**What W4 should have said, run here** (`gat_cells.py` on `data.grf`): all
twenty of 817's Emperium cells are walkable on this client's maps except
`prtg_cas02 158,174`, which is blocked. The switch targets `gld_dun01
186,165`, `54,165`, `54,39`, `186,39`, `223,202`, `gld_dun02 32,122`,
`gld_dun03 28,251`, `gld_dun04 39,258` are walkable. 817's `gswitch.txt`
has a typo, `"gld_dun01.gat.gat"`, on the `payg_cas02` switch.

**W6-001 to -004: not euRO.** The rows call the Ro-Guardians pages "euRO
Iris". The forum lists boards for "eRO - Iris", "eRO - Penril", "mRO -
Chaos", "mRO - Loki" and a "Pasar Malam" board, and euRO's own notices of
the time speak of one server, Chaos (W7-004). It is Malaysia's RO: the
forum's front page and its one archived topic say so (woe-mro-2004.md).
The content is still first-hand and dated: twelve siege commentary topics
from 2004-04-17 to 2004-05-22, every Wednesday and Saturday, up to 244
replies each, and a table of twenty castles with owner and days held. The
topics themselves were not loaded. That is the lead for "what it was like".

**Single rows.**

| Row | Claim | Source |
|---|---|---|
| W3-MOB-EMPELIUM | "Plant defense type" | The quoted line reads scale 0, race 4, element 22 in 817's column order. The HP (68,430) is right |
| W7-001 | euRO's "first recorded" siege, 2004-04-11 | The notice of 2004-04-09 warns of a GM attack "beim nächsten War of Emperium": the next one, so not the first |
| Q-TF-SF-A | Sono-san at `morocc 155,80` | Its own quote says `x = 165, y = 124` |
| Q-SM-MH-B | mod0631 wants "200 Empty Bottles and 1 Padded Armor" | `countitem(713)>=200` and `countitem(1058)>=1`: item 1058, not Padded Armor (2312) |
| Q-SM-MH-A | 817 wants 500 Empty Bottles and a Rocker Doll | It also wants item 2312 (`moving_hp.lua:15`) |
| J-KNI-NPC, J-BS-NPC | "Captain Herman", "Altisen" | The quotes are `プロンテラ騎士団長` and `ギルド員`; the names are not in them |
| P-BON_GUN | Bongun is tamed with the Sword of Chinese Exorcist, "matching pets.toml" | `pets.toml` has item 659, and the same saved page has "Hear Heart ... Bongun the monster's favorite item". The sword is the accessory |

Two rows carry a `source_date` of 2004-06-03 for a capture of 2004-04-23
(Q-AC-AC-C, Q-AC-CA-C).

## `REPORT.md`

In no row and no saved page:

* First Aid at `prt_in 238,168`, `prt_in 274,166`, `prt_church 103,98`
  with "1 Empty Bottle". The rows say Nami at `prt_in 234,133` and
  Sterilized Bandages from the Nursing Instructor at `prt_castle 175/146`.
* Play Dead from "Peter Smail at `prontera 270,212`". The rows say the
  Prontera Chivalry Member at `prt_in 73,87`.
* Sand Attack from "Bagot" for "5 Fine Sand, 1 Earthworm the Dude, 1 Bag of
  Grain". The rows say Alcouskou, 5 Fine Grit and a Leather Bag of Infinity.
* The Emperium as "Small, Holy 1, Plant-type with 100% flee and status
  immunity", citing W2-006 to -009, which are a warp, a switch and two
  clock events.
* A Blacksmith examination "in Geffen & Einbroch".
* "Bongun was never listed": the saved library page lists a Bongun Egg.

Also in no row: "+0.7 DEF" per refine level.

Row ids described as something else: W2-001 to -005 ("GvG map flags and
damage penalties"; they are five steward fees), W2-006 to -010, W2-014 as
the investment fee (it is the Phen card), W5-001 and -002 as "flat 10,000z"
(they are two sentences of the iRO guide), W5-004 to -007 (four different
formulas than the rows withdraw).

Its recommendation to build the Wizard test on `job_wizard` at `116,171`,
`116,97`, `46,99` repeats what rounds 1 and 2 refuted: those are `job_wiz`
cells from jAthena //1046 (2004-09-12, `6332410`), and two of the three are
blocked on this client's map. Its recommendation to set Baphomet's HP to
668,000 would replace a period value with a later one.

## What holds, task by task

**W1, kRO notices: complete.** 23 rows from 18 notices, all confirmed.
Checked against all 55 notices Fandom has from 2003-01 to 2004-06: every
rule about sieges, castles, the Emperium, treasure, guardians and guild
dungeons is a row. Not rowed: the Novice siege event (2004-04-07 to 05-18)
and the "strengthened" relay quest for castle owners (06-15). Fandom has no
siege line before 2003-06-10, so the Sakray tests of spring 2003 stay
undocumented. Nine notices are new against round 2: a fix to the castle
Kafra's teleport (2003-09-08); the Wizard's monster-information skill on a
guardian froze the client (12-02); guild flags could be used to block the
entrance (12-30); reconnecting in a guild dungeon returns to the save point
(2004-01-08); `/notrade`, "for sieges" (03-23); a relay quest for guilds
that own a castle (03-30); castle names in the siege broadcasts (05-04);
Guild Extension from +2 to +4 members a level (06-15); other guilds could
enter a castle through the flags (06-29).

**W2, 817's code: right, and thin.** 20 rows, all confirmed
(`battle.c:1153-1163`, `skill.c`, the steward test script). Skills do no
damage to the Emperium; a player without a guild, or whose guild lacks
Official Guild Approval, does none either; knock-back is zero on siege
maps; Endure, Teleport, Warp Portal, Ice Wall and Back Slide are refused;
the Phen card does not protect a cast. Not rowed, though the brief asked:
the `gvg_*_damage_rate` settings two lines further down, guardians
(`mobs/gvg.txt`), the flags, what the guild skills do in code.

**W3, the data.** All forty treasure box rows restate their `mob_db.txt`
line correctly (compared mechanically). The negative holds and matters: no
817 script puts a treasure box in a castle; boxes 1333, 1342, 1343 and 1353
appear only as ordinary spawns in `mobs/monster35.txt` (Amatsu, Gonryun).
So "which box belongs to which castle" has no period emulator source.
Guardian and Emperium HP, `castle_db.txt`, the five guild skill
descriptions from the client: confirmed. One of the client's 39 castle
messages was rowed. mod0631's boxes were not compared.

**W5, numbers: the answer is "none before mid-2004", confirmed.** The only
period number is the flat 10,000z of the jAthena/817 test steward, which
also caps investment by a counter ("一期間に2回まで", twice a period) and
not by a value; W5-007's reading of that script is right. The Japanese wiki
pages named in the reasoning were not saved.

**W7, euRO.** Castle ownership and investment were live in the open beta by
2004-04-08, to be restored at the start of P2P; a siege with a GM attack on
Sunday 2004-04-11, 18:00. On 2004-09-03 treasure boxes, the Emperium and
guild dungeons were switched off; WoE came back on Chaos on 2004-09-22.
When the first siege was is still open.

**Q, quest skills: the table #7 asked for, minus the items.** 45 witness
rows; NPC, cell and job level were re-read for every 817 and mod0631 row
and are right apart from the three above. Differences now on record:
Arrow Crafting is job 35 in 817 and 30 in mod0631 and on RODataZone; Cart
Revolution is 35 in 817 and on RODataZone and 30 in mod0631; Stone Fling is
15 in both emulators and 20 on RODataZone. The emulator rows give no item
lists, and First Aid and Play Dead are one header line each, not "in full".
RODataZone's Novice page (capture 2003-12-15) does give both quests whole,
and its Nursing Instructor at `prt_castle 175/146` is mod0631's `看護将校`
on the same cell.

**J, 2-1 quests: not answered.** The resnametable rows hold (and
`wizard_2-1`, `wizard_3-1` alias `job_wizard` too). The rest is NPC header
lines and the September 2004 Wizard script. The six RODataZone 2-1 pages
were saved and not used; the Wizard and Knight ones are empty shells.

**D, dialogue: nothing found.** The six round-2 lines are withdrawn; the
only place the rows say was searched is the RODataZone front page. The five iRO bug-board rows are players describing bugs,
with no NPC line. The two "authenticated" lines are mod0631's Japanese.

**P, pets: closes the open part of THREADS #14.** The iRO Rune-Midgarts
Library (capture 2004-06-05) describes each taming item by the monster it
lures. The 24 rows other than Bongun's name the item `pets.toml` has for
that pet (compared by item id and name here), and Bongun's, read here, does
too: all 25 agree. The library lists 25 pet eggs, Bongun's among them,
where the pet guide captured the same day says 24 monsters. RODataZone's
pet page was saved and not rowed.

**K, R, M.** Hold, and all of it was in round 2. No international fare or
points list was found, and no row says so. The count M asked for, run here
over the 406 monsters both tables have: 17 differ in HP and 11 in EXP. Eight
monsters are simply zero in mod0631 in both. The real HP changes are nine,
all upward in 817: Baphomet 568,000 to 668,000, Doppelganger 149,000 to
249,000, Golden Thief Bug 86,000 to 126,000, Orc Hero, Drake, Orc Lord,
Mutant Dragon, Dark Lord, Wraith Dead. That fits "adjusted monsters HP /
Exp" as a boss change; which side euRO's January 2004 had is not shown.

## Leads

* **Followed, 2026-11-10:** the Ro-Guardians record. The siege commentary
  topics are not on Wayback; the castle takeover histories are, and are
  written up in [woe-mro-2004.md](woe-mro-2004.md). The site is for
  Malaysia's servers (Gameflier), which settles the "euRO Iris" rows.
* euRO news between `events/planned/de/00074` and `news/eu/00499`, for the
  first siege and when treasure arrived: Wayback's index lists 76 news and
  19 planned-event pages for 2004-03 to 2004-10.
* Item 1058 in mod0631's Moving HP Recovery, against the Rocker Doll of 817
  and RODataZone.
* Why `prtg_cas02`'s Emperium cell is blocked on this client's map.

## What changed in the game

| Change | Source | Where |
|---|---|---|
| First Aid from Nami, Play Dead from the Chivalry member, the Nursing Instructor in Prontera Castle | mod0631 (NPCs, cells, steps, items); RODataZone 2003-12-15 (names, job level 4, the Newbie Tag as an item, the ten minutes); translation ours | `scripts/npc/novice_skills.lua` |
| Test for it | | `crates/ro-game/tests/npc_mod0631.rs` |

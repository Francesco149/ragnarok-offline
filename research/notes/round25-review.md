# Round 25 review (2026-11-16)

34 rows, all T9 (threads 10, 32, 33, 34), from four GameFAQs guides and
two emulator files: Captain FAI's MVP guide (28266), AZn2th3N's Monsters
FAQ (19673), excaladbiur's (Kirby Lo's) Dungeon FAQ (24632), Jaypogi's
Trap Hunter guide (25790), mod0631's and 817's `mob_db.txt`. The agent's
checker run: 34 PASS. Ours with `--fetch --pages
research/agy-threads/r25/pages`: the same.

## Invention

**Quotes and URLs: none.** Every quote is on its page; the captures are
the gamefaqs.com pages of 2010 that the rows cite.

**Dates and regions: eight rows imprecise, none invented.**

* T9-331..335 (Dungeon FAQ 24632) are dated "2003". That is the
  copyright line. The page is v1.31 of 2004-02-01, and its history
  says v1.30 "Revised a lot of info, removed monster not in current
  episode": the monster lists are of v1.30, undated, before 2004-02-01.
  The iRO tag holds ("come back to iRO!" in the credits).
* T9-336/337 (Jaypogi 25790) are dated "2004": the page is v2.3 of
  2004-12-31 and no version line dates the Claymore section. pRO holds.
* T9-322/323, 326..330 (AZn2th3N 19673, "Date: 10/01/03") have no region.
  The guide names no server. Its credits say "most of the monsters' info
  I got from" Ragnagate, a database site: by the rule of round 17 this is
  a copy of a database, not a player's account (see below).
* T9-324/325 tag mod0631 as jRO and 817 as iRO: 817 is an emulator with
  English names, not iRO. The dates (2003-12-07, 2004-02-09) are the
  files' own.

**Claims: none past their quotes.** progress.md lists the rows as they
are.

## What is new

**MVP prizes (T9-304..314) found a slip of ours.** The cRO guide (v0.94,
07-03-04 D/M/Y, "I play the CRO version") gives Osiris a "Mystic Blue
Box" and Doppelganger a Cursed Ruby at 15%; ours had neither. The cause
was in the importer, not the data: 817's own rows for Osiris, Baphomet
and Doppelganger lack the MEXP and ExpPer pair, so their three MVP prizes
start at column 47, and the import (yesterday's, from round 24) read
columns 49-54 as for every other row. Each of the three lost its first
prize. RODataZone's iRO table (2004-02) lists Osiris's Old Blue Box and
Doppelganger's Cursed Ruby as MVP prizes too, and names Baphomet's third
the Yggdrasil Berry, as the cRO guide does, where 817's row has the Seed.
Fixed in `tools/import/athena_import.py` (the shifted rows read from 47,
asserted; Baphomet's Berry at 817's rate). The cRO guide's rates differ
from ours on most prizes (Osiris's Seed 30% to our 10%, Baphomet's Horn
50% to 38%); cRO is not our server and RODataZone has no rates, so 817's
stay.

**Monster numbers (T9-326..330).** AZn2th3N's table agrees with ours on
Andre, Pierre, Bathory, Mummy and Sandman, where RODataZone's iRO table
(thread 33) differs. Compared whole (240 rows, 200 matched by name): 181
agree with mod0631 on HP and both EXP, so the table is the emulators'
own data line (Ragnagate's), and its agreement on the five is not a
second witness against RODataZone. Thread 33 is unchanged.

**The Thief Bug (T9-322..325).** AZn2th3N has the Thief Bug (126 HP)
Insect, Shadow 1, as we built it on 2026-11-15 from Ragu Memo and
RODataZone, against both emulators' Formless, Neutral 3: a third
agreement, from a table that elsewhere follows the emulators. Its Thief
Bug Egg is Insect, Neutral 3; Ragu Memo has the egg Dark 1, Insect, and
RODataZone Dark 1, Formless; ours (both emulators) Insect, Dark 1. Kept.

**Summons (T9-316..321).** The cRO guide's "Pawns": Osiris 6 Isis,
Baphomet 6 Baphomet Jr., Moonlight Flower 3 Nine Tails, Phreeoni 6
Wormtail, Dark Lord 3 "Lord of Illusion", Eddga 6 Bigfoot. RODataZone's
"Minions": Osiris 7 Isis, Baphomet 8 Baphomet Jr., Doppelganger 9
Nightmares, Moonlight Flower 4 Nine Tails, Orc Hero 6 Orc Warriors,
Phreeoni Hode. Ours summon 4 or 5 a call (jAthena //0743), Orc Hero
High Orcs. The counts agree with nothing, nor with each other; they may
describe the escort a boss spawns with rather than its summons. Thread 34.

**Dungeon lists (T9-331..335).** Payon Cave 1F without Skeletons: a
second English-server account after Phelan's (round 24), already built.
Geffen Dungeon 3F has a Joker in the guide; ours (mod0631) has none.
Glast Heim "Castle Level 1" in the guide is Baphomet Jr., Joker,
Khalitzburg, Marduk, Mysteltainn, Pest, Rideword, Tirfing; mod0631's
`gl_cas01` has Joker, Khalitzburg, Whisper, Rideword, Tirfing, Dark Frame,
Carat, Alice, Sageworm and Owl Duke, several counts confirmed by Ragu
Memo's jRO map of 2003-12. Possibly an iRO/jRO split, or the guide's
"Castle Level 1" is another map. Thread 10.

**Claymore Trap (T9-336).** "Uses 2 traps" (pRO, by 2004-12). This
client's `skilldesctable.txt` says "1 Trap Needed", the May-2004 client's
"2 Trap Needed": euRO changed it between the two. Ours spends one.
Noted in `research/notes/may2004-client.md`. T9-337 (traps fire when an
enemy steps in their area) is what the game does.

## Count

New: the MVP prize slip (fixed), the Claymore's cost changing by May 2004,
the summon/escort counts, two spawn differences. Not new: the MVP EXP
line (T9-315), the Thief Bug (built), Payon Cave 1F (built).

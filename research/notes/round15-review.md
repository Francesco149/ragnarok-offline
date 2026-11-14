# Round 15 review (2026-11-14)

51 rows (T1 14, T9 35, T10 2). The agent's checker run: 51 PASS. Ours with
`--fetch --pages research/agy-threads/r15/pages`: 32 PASS, 19 WARN, 0 FAIL.

## Invention

**No quote invented**, no false claim. Slips, all in claims past their
quotes and in metadata:

* **19 rows dated 2004-04-17** (T9-055..072, T10-019): RODataZone's
  `quests.js` is the capture of 2004-05-02 (Wayback serves no other; the
  checker's WARN). `20040417` is round 13's file name for the page, a date
  found nowhere (round 13 review). The round copied the file name into the
  rows. (We had copied it too, into `scripts/npc/hat_makers.lua` and
  DEVIATIONS.md, an hour before; mended.)
* **Ten rows dated the day they were read** (T9-080..089: `source_date`
  2026-11-14 for RODataZone's tables of 2004-02-11 and 2004-02-24, which
  their claims name).
* **Three rAthena rows graded B** (T1-028..030), with `source_date` the
  repository's 2026 head: a later emulator is grade D.
* **Claims past their quotes**, all true when checked: T1-029 (Spear
  Boomerang, Zephyrus: rAthena `knight.txt:890,903`), T1-030 (the fail
  above 10 points), T1-033..037 (each adds the other emulator's value,
  which THREADS.md #8 has), T9-055..072 (our item ids in brackets).
  T1-033's "> 80" and "> 70" looked like a clash with THREADS.md #8 ("90,
  80"); they are the same marks, scores counting in tens and twenties
  (eAthena 1.0 `blacksmith.txt:489,1410`).

## What is new

About four rows of 51:

* T9-077, T9-078 (grade D): rAthena's training ground gives every Novice
  five free storage and five free teleport tickets, and a Merchant-to-be
  four cart tickets (7059-7061): a lead for THREADS.md #31's Kafra items.
* T9-079: the Kafra payroll is a WoE Training quest item in rAthena's
  renewal scripts (later still).
* T9-076: rAthena's Bongun Hat row (5046), nothing we lacked.

Everything else was on file: the eighteen hat recipes (T9-055..075) are the
`quests.js` and `trade.html` lines this session built the makers from
(THREADS.md #31); T9-080..089 are among the differences
`research/rdz_items_check.py` prints, kept as they are by the rule of R6;
T1 restates THREADS.md #8; T10-019 is the Usher's 50z (THREADS.md #29),
T10-020 quotes our own `game.rs`.

## Queue

The hat item narrowed to the four hats no list has (Kafra Band, Panda Cap,
Wings of Demon, Bell of Pussycat) and the Kafra tickets; added: who may
wear which item (the client's "Available Job" lines against mod0631's
masks), the drops mod0631 left empty (THREADS.md #32). Two rules: date a
page by the capture Wayback serves; a later emulator is grade D and
`source_date` is never the reading day.

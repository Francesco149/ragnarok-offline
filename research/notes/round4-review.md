# Round 4 review: nothing invented, little that was new

Round 4 was the outside agent's (Antigravity/Gemini) pass over the queue
(`research/agy-threads/QUEUE.md` as of 2026-11-10). Its output is
`research/agy-threads/r4/`: 85 rows in `rows/*.jsonl`, 8 saved pages,
`check.txt`, `progress.md`. This is the second pass over it, of 2026-11-11.
Every row was read against its source.

## How much was invented

| Layer | Result |
|---|---|
| Quotes, files, line numbers, URLs, capture dates | **Nothing invented.** Our run of `check_rows.py --fetch` gives 84 PASS, 0 FAIL, 1 NONE, the same as the agent's `check.txt` |
| The `claim` next to each quote | **Nothing false.** All 85 say what their source says. Seven add a cell or a name that is not in the quote (T1-005, T1-007 to T1-010: NPC cells; T4-004 "guild"; T4-011 "one dye"); each of those is right in the file |
| `source_date` | **Made up for the emulator rows.** Every mod0631 row is dated 2003-06-30 and every 817 row 2004-01-20 or 2004-02-17. The files are dated 2003-07-28 to 2003-12-12 (archive of 2003-12-18) and 2004-02-03 to 2004-02-18. Nothing was built on these dates |
| `progress.md` | The row list for T1 is shifted by one from T1-005 on and names a cell (`alberta 189,151`) that is in no row. Every task is marked "Answered"; none is (below) |

The two rules added to the brief after round 3 (a claim may only restate
its quote; a computed result carries its command output) did what they
were for: round 3 had 27 claims with a detail from nowhere, this round has
none. There was no report to invent in. What is left of the habit is the
date field and the word "Answered".

`check_rows.py` now loads each URL once per run, with a pause and two
retries, and finds the data directory inside the workbench: the 42
"connection refused" warnings of round 3 did not come back.

## What the round covered

It took T1, T2, T4, T5 and one item of T9. It did not take T3, T6, T7, T8
or the rest of T9.

| Task | Rows | New to us | Not done |
|---|---|---|---|
| T1 Turtle Island | 12 | The iRO notice, dated 2003-12-09: the island comes to Chaos and Loki on 2003-12-16 with "a series of new dungeons", "new headgears and new quests" and a new MVP (T1-001; read from Gravity's own news archive, not a capture of the time). The other ten rows quote lines of mod0631 and 817 that THREADS.md #24 already had | How a player of the international servers got there; anything on euRO (one `none` row) |
| T2 the 2-2 skills | 25 | Fifteen tables from the iRO guide's Crusader, Monk, Sage, Rogue and Alchemist pages (captures of 2004-06-11 to -15). Ten rows on what mod0631 and 817 do with the same skills; all hold (`#if 0` at mod0631 `skill.c:1255`, the commented-out Defender at 817 `battle.c:1106`) | No row from the client's skill text, none for Bard and Dancer, nothing on the Monk's combos, most skills of the five jobs it did open, no table of disagreements |
| T4 dyes | 12 | Nothing: twelve lines of mod0631's `npc_event_dye.txt`, which `scripts/npc/dyes.lua` was written from | Every question asked (hair and clothes dyeing on iRO/euRO, the kRO notice, the English names) |
| T5 skill numbers | 23 | Nothing: the client's text and mod0631's lines for the table already in THREADS.md #20 | Which state the client's text describes; the patch archives; any notice |
| T9 Moving HP Recovery | 13 | RODataZone's Swordman page of 2004-05-02: "1 Rocker Doll, 500 Empty Bottle" | The other quest skills |

## What we did with it, and beyond it

* **The iRO guide's skill pages.** The round's pages gave the address
  pattern. Wayback has all nineteen jobs' skill pages, the earliest of
  2004-03-27 to 2004-04-13; they are saved in
  `~/data/ragnarok-cd/refs/web/iro-guide-2004/` (not ours to commit) as
  `<capture>_job<xxx>skill.html` with a text copy beside each. Reload with
  `http://web.archive.org/web/<capture>id_/http://iro.ragnarokonline.com/game/job<xxx>skill.asp`.
* **Thread 20 is settled by them** (now R6): on every row of that table
  the guide of March and April 2004 gives mod0631's number, not the
  client's text. The text is an older state of the game.
* **Moving HP Recovery** (`scripts/npc/moving_hp.lua`): 817 asks for the
  international servers' items and takes the Japanese servers'. Fixed to
  take what it asks. The Swordman's other two quests (Fatal Blow, Auto
  Berserk) have the same items in 817, mod0631 and on RODataZone.
* **Turtle Island** (`scripts/npc/turtle_island.lua`): mod0631's chain is
  in the game, and 817's 800z passage is out. The notice's "new quests"
  is the only word from the international servers; the fare and the story
  are mod0631's (THREADS.md #24).
* **The 2-2 skills were not drafted**, though the numbers for a draft are
  now on disk. Looking at what the client can show first: its packet table
  has no 0x01cd to 0x01d2 and no 0x01e1, the packets for Auto Spell's
  list, Devotion's lines, spirit spheres, Blade Stop and the combo delay
  (THREADS.md #25). The Monk in particular cannot be played on it as
  designed.

## For the next brief

* Keep the two claim rules; they worked.
* `source_date` for a file on disk: the file's own date (`ls -l
  --time-style=long-iso`), quoted, or leave it empty.
* "Answered" means every bullet of the task has a row or a `none` row.
  Otherwise "partly", with the bullets that are missing.
* A task that restates what THREADS.md already says is not an answer: say
  "nothing new" and spend the time on another task.

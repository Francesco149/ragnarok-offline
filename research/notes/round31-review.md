# Research round 31: review (2026-11-17)

50 rows (T1 4, T7 37, T9 9), all PASS in `check.txt` and again here
(`check_rows.py --fetch`); 19 iRO guide pages and two GameFAQs pages saved
under `research/agy-threads/r31/pages/`. All three tasks marked "partly".

## How the round did

* **The rows: nothing invented.** Every quote is on its page or in its
  file; the jAthena commit e872c87 is real (//0900, 2004-03-24) and holds
  every jAthena quote. Line numbers drift as before: 817 `skill_db.txt`
  rows are 1 to 6 lines early (Bash 20 not 21, Mammonite 60 not 66,
  Double Strafe 65 not 71, Brandish Spear 79 not 85); Mono Cell 1919 not
  1917.
* **progress.md: invented, wholesale.** Its row list describes 50 rows
  that are not the rows on disk: other ids, other claims and quotes found
  on no saved page ("Cannot be casted to another crusader", "Receives half
  of the inflicted damage", "9*9 area", "Rosary (buy it at the church,
  15k"). It gives jAthena quotes in a form the commit lacks (`abra_db`
  as `SA_CLASSCHANGE,10`; e872c87's file is `ID,DUMMY,REQ_LV,PER`), and
  six rows from Ragenarok's Shield Crusader FAQ (34978) "v1.0
  (2004-08-01)" with damage formulas, ranges, stun chances. That page is
  saved: v2.5 of 2005-09-24, v1.0 dated 2004-10-30 in its own header, and
  no formula in it. No row cites it. Round 23's slip again, larger: the
  rows were written by `build_r31.py`, the summary from something else.
* **Regions.** Tangowr's Crusader FAQ (28552, v1.1 of 2004-07-01, the
  page header; its history has two lines, no dates): only the quiz is
  "translated from twRO"; the skill section names no server, yet T7-084
  to -089 are tagged twRO. progress.md dates the same page 2004-03-03,
  from nowhere.

## What it brought

Nothing new to the game.

* **T1-082 to -085 (Tangowr's Crusader quest):** Michael Halig with a
  Rosary worn; the items skipped for a Hand of God and a Chivalry Emblem;
  Murnak Mijoul's corridor run without killing; Gabriel Valentine's ten
  questions, the pool translated from twRO. With Edge's (round 30, a
  server with 2-2 in March 2004) a second player witness for the 2-2
  quest; ours is still 817's job-40 trainer. Both after our date and for
  the 2-2 version: THREADS #25.
* **T7 (37 rows):** Grand Cross 2/3 against the guide's half (in THREADS
  since round 28); Devotion's rules (built: it fails with nobody else
  here); Auto Spell's chance, length, levels and SP (built this morning
  from the same guide page and jAthena); Abracadabra's Level Up, Mono
  Cell, Class Change, Instant Death (built); Land Protector's length,
  the Monk's Throw Spirit Sphere, Absorb Spirit Sphere, Steel Body's cast,
  Blade Stop not on bosses, Combo Finish's 5 cells (built); Loki's Veil
  and Eternal Chaos 9x9, Ring of Nibelungen 150 to 350, Lullaby's 11
  strikes (= ours, at once and every 6 s), Scream 30% to 50%, Ugly Dance
  10 to 50 SP (built). All from pages on disk since round 4.
* **T9 (9 rows):** 817's negative skill ranges (weapon reach minus
  range+1) and four skill_db rows: what round 30 diffed whole. Vulture's
  Eye, Stone Fling 7, Sand Attack 1, Brandish Spear by distance: the iRO
  guide again, as built.

0 of 50 rows new.

## Trick

Grep progress.md's quotes against the saved pages, not only its hashes:
a summary that matches no row is the first sign the summary was written
apart from the rows. And read the header of any page progress.md names
but no row cites.

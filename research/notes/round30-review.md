# Research round 30: review (2026-11-17)

33 rows (T1 5, T4 8, T7 6, T8 2, T9 12), all PASS in `check.txt` and again
here (`check_rows.py --fetch`); four GameFAQs pages saved under
`research/agy-threads/r30/pages/`. All five tasks marked "partly".

## How the round did

* **Nothing invented.** The four Wayback captures (24428 `20100509055018`,
  29457 `20100508125837`, 31808 `20100508130401`, 31925 `20100509045639`)
  are each the page's first capture in CDX. No quote, URL or line made up;
  no claim past its quote.
* **Regions and dates, Edge's Knight/Crusader FAQ (29457).** Its v1.1 line
  (02/29/04): "I have temporarily stopped playing pRO due to it's server
  problems. I now have started playing on another version of Ragnarok that
  includes the Alternate 2nd job classes." So:
  * T1-078 to T1-081 (the Crusader quest, added in v1.2, 03/16/04) are
    tagged pRO, but pRO had no 2-2 then (v2.1, 05-13-04: "maybe the
    Alternate Second Jobs will come out"). The server is unnamed: a server
    with 2-2 in March 2004 (kRO, a test server or a private one).
  * T9-419 to T9-424 (the rare weapons, added in v1.1, spears in v1.4) are
    dated by those lines; v2.4 (05-28-04) "Updated the Equipment section"
    and v2.5 "fixed a few things in the Equips", so by the queue's rule
    the section is of 2004-05-28 to -30. Region as above.
* **KoW (24428), T1-077:** dated 2003-07-13 (the page header), but the
  history stops at v0.2 of 4/13/2003, which wrote the whole guide; the
  region cRO/kRO is right this time ("I am not currently playing iRO! You
  can find me on cRO ... and kRO").
* **progress.md** calls Maruko's bow formula (T8-066) a "second
  independent witness" for RODataZone's. His credits: "rodatazone.com ...
  i got the technical stuff in my guide from this site". It is the same
  page copied (round 12 found the same of his ailment appendix).

## What it brought

Nearly every row is an earlier round's again: the four guides were all
saved before (29457 in round 11, 31808 in round 12, 31925 in round 11/28,
24428 in round 24).

* **T4 (ailments, 8 rows):** Maruko's appendix A.4 = RODataZone's status
  page, set out in round 12 and THREADS #21. Nothing new, and against the
  task's own last line: "Do not quote RODataZone's page or the GameFAQs
  guides that copy it again".
* **T7 (Alchemist, 6 rows):** ArislanX again, built from round 28.
* **T8 (bow damage, 2 rows):** RODataZone's formula through Maruko, built
  from round 19.
* **T9 Auto Counter (3 rows):** built 2026-11-15 as 817 type 1 and jAthena
  //0751 (HIT +20, twice the critical rate, held still). Maruko gives 0.3 s
  a level; ours 0.4 s (817's cast_db), as the iRO guide's Knight page of
  2004-04-13 ("0.4sec" at level 1). Kept.
* **Spear Boomerang reaches 3, 5, 7, 9, 11 cells (T9-418): built.** Maruko
  "1 + 2 X Skill Level tiles", and the iRO guide's Knight page (2004-04-13)
  "+ 3 ... + 11" by level, and 817's skill_db `-3:-5:-7:-9:-11` (weapon
  reach plus 2 a level). mod0631 and jAthena of 2003-11-21 have 11 at
  every level; jAthena changed it in //0749 (2004-01-27), a format overhaul
  with no "real server" remark. Looking at the other ranges where mod0631
  and 817 differ turned up **Grimtooth**: the iRO guide's Assassin page
  (2004-03-27) 3 to 7 cells, 817 `3:4:5:6:7`, mod0631 7 flat: built too.
  The skill list now tells the client the learned level's reach
  (`rules.skill_reach`); before, Spring Trap and Shield Boomerang (already
  by level on the server) were shown at their table value, so the client
  walked to the full reach at level 1 and the server refused.
* **DEF to 2/3 while casting Bowling Bash, Brandish Spear (T9-416, -417):
  built.** 817 has the rule as a skill_db column (`cast_def_rate` 33 for
  both and Grand Cross, `battle.c:505`, players and monsters); round 24's
  Comodo preview said the same, unbuilt then. mod0631 has no such column.
  The casts were already uninterruptible.
* **Crusader quest (T1-078 to -081):** four parts, items, Rosary, Holy
  Water. A server with 2-2 in March 2004, unnamed: a witness for the 2-2
  version only (the client lacks 2-2 skills).
* **Rare weapons (T9-419 to -424):** what mod0631's item scripts do.

About 3 of 33 rows were new, and 2 rules built from them.

## Trick

When a row gives a per-level number the game has as one value, diff that
column of mod0631's table against 817's for every skill: one row led to
two skills here, and the rest of the list (Double Strafe, Arrow Shower,
Charge Arrow, Mammonite, the melee skills given as weapon reach) is in
THREADS.md.

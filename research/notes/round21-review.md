# Round 21 review (2026-11-15)

19 rows: T9 10, T8 6, T1 3. The agent's checker run: 19 PASS. Ours with
`--fetch --pages research/agy-threads/r21/pages`: 15 PASS, 4 WARN (404).

## Invention

**Four URLs invented, as in rounds 14 and 18.** T1-056, T1-057 and T1-058
cite `web.archive.org/web/20040605070057id_/http://db.gamefaqs.com/...`,
one timestamp for three different files; T9-258 cites `20050228065047` for
a fourth. Wayback's CDX has no capture at all of any of the four
`db.gamefaqs.com` files. The saved pages are captures of
`www.gamefaqs.com` of May 2010 (beacon `ts=1273316120` to `1273384215`),
which is what the rows should cite. The quotes themselves are on the saved
pages, and the versions the rows give match the pages' headers (Psycho
Battousai v1.3, 2003-02-25; the only KoW, 2003-07-13; H0BGOBLIN v1.7,
2003-07-25). Edge's FAQ is v2.6 (2004-06-03) on the page; the row dates
its Knight paragraph by v1.0 (2004-02-08), which is plausible (the section
"How to get there" is in the first version's plan) but not shown.

Also slipped, unchecked by the tool:

* **Line numbers** off by up to 38: jAthena `5415aa0` `map/battle.c` 1684
  (row 1667) and 1799 (1780); 817 `skill.c` Sightrasher 3196 (3169); 0.5.2
  `skill.c` 4050 (4012); rAthena `706e785` `pc.c` 6617 (6602), the treasure
  script 43 (41), the steward script 246 (242), 0.5.2's treasure 20 (21).
* **progress.md** lists the rows wrongly again: T9-256 and T9-257 swapped
  with their descriptions, the Acolyte row called T9-257 (it is T9-258).
* Grades: B for emulator code, client text and guides alike, as before.

## What is new

About three rows of 19; one changed the game.

* **eAthena's own changelog dates the castle values** (T9-249): rAthena's
  first commit `706e785` (2004-11-04) carries eAthena's `Changelog.txt`,
  whose entry of 06/05 (2004: the file runs from 11/03 back to 01/03) reads
  "Implemented guild castle regen.(Stackable 2x for castle owners)
  [Valaris]". Six lines below, under the same date, and not quoted by the
  round: "Guardians and emp will now get +2000 hp for every defense
  investment within a castle [Valaris]"; `src/map/mob.c:183-186` and `:301`
  in that tree do it (by adding to `mob_db` itself on every spawn, a bug).
  The game took +1000 a level for the Emperium from rAthena pre-renewal,
  undated and much later; it is +2000 now, dated 2004-06-05 (still tier
  later). PROVENANCE, DEVIATIONS, THREADS #17.
* **jAthena fixed the bow's precedence slip on 2004-05-16** (T8-039,
  T8-041): `5415aa0` (2004-01-04) still has mod0631's
  `watk * (dex<watk)?dex:watk`, `d6aa96f` has `watk * ((atkmin<watk)?
  atkmin:watk)/100`. The game already uses the corrected formula (round 19,
  RODataZone 2003-12-16). jAthena of January 2004 adds an arrow's ATK as
  `rand()%(arrow_atk+1)` (0 to ATK), RODataZone `rnd(0, ArrowATK - 1)`;
  ours is RODataZone's.
* **jAthena's castle-ground flags of 2004-02-12** (T9-252, `1fcb977`
  `conf/gvg/guild_flag.txt`): a second witness, a week older than 817's,
  for the flags on the grounds without a way home. Ours offer the master
  0.5.2's way home (DEVIATIONS).

Already known: Knights for 2,500z and Blacksmiths for nothing before the
2-1 quests (round 7's Weggy rows; the pRO FAQ of Feb 2004 is a lagging
server); 0.5.2's town flags (Kriemhild...), steward cell, flat 10,000z
investment and treasure counts; eAthena 1.0's scaled investment; the
Sightrasher patterns of 817, 0.5.2 and the iRO guide (THREADS #22); the
Acolyte test's three ascetics.

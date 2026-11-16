# Research round 28: review (2026-11-16)

44 rows (T8 6, T9 38), all PASS in `check.txt`; pages saved under
`research/agy-threads/r28/pages/`. Both tasks marked "partly".

## How the round did

* **One capture invented.** T9-378 (KoW's Battlesmith guide, GameFAQs
  24428) cites `20100508064548`, the timestamp of a different FAQ (29538,
  T9-385). CDX has no capture of 24428 that day; the checker is served
  `20100509055018`. Every other capture is in CDX.
* **Regions.** T9-378: KoW writes "I am not currently playing iRO! You
  can find me on cRO ... and kRO" (round 24 caught the same guide). T9-385:
  anjingbalap's Dagger Assassin guide is idRO ("in idRO we are underrated
  build", thanks to "All my friend @ idRO"), tagged iRO.
* Dates hold: SoulEclipse's v4.0 is "2.25.05" (the page header is v4.x of
  2006); Maruko v1.1 "August 8, 2004"; F0rever Zer0 1.11 "04/11/04".
* No claim goes past its quote. No `url` slip this time (they sit under
  `source.url`).

## What it brought

* **Acid Terror and Bomb on monsters (T9-374 to -377): built.**
  ArislanX's iRO Alchemist guide (2004-08-20), read whole: a monster's
  armour broken by Acid Terror loses 25% DEF; Bomb "lower[s] the enemies
  attack by 25% if fighting a monster"; Acid Terror "is neutral, so not
  subject to elemental weaknesses". We had left the breaks out as
  players-only. Now a monster's armour (3% to 13% a throw) or weapon (1%
  to 5% a burn) breaks for as long as it lives, and Acid Terror is
  Neutral (jAthena //1072, 2004-12-06, agrees: "防御無視、必中、無属性、
  カードを乗らない"; the DEF and cards part no 2004 player source says,
  not built). DEVIATIONS.md, test `alchemist_breaks_monster_gear`.
* **Hunter traps and Blitz Beat (T8-060 to -063):** Ragu Memo's jRO
  Hunter page of 2003-12-05 has the very formulas `battle.lua:997-1001`
  already uses (817's). One more witness, nothing to change.
* **Boss slaves (T9-398 to -404):** Maruko's pRO list matches the slaves
  we import from jAthena //0743 for all seven bosses, Phreeoni's
  Hode/Sandman split included.
* **Grand Cross (T9-372/-373):** SoulEclipse (2005) restates the iRO
  guide's Crusader page word for word in substance (0.9 s, "by 2/3",
  half to the caster): not a second witness. Tangowr (2004-07, server not
  said; quiz translated from twRO) has "total damage 2/3 would be reflect
  toward you" against the guide's half. Open, noted in THREADS #25.
* Cards (T9-382 to -392), Bloody Axe, Light Epsilon, Great Axe: Ragu
  Memo's jRO pages again (thread 32's iRO/jRO split); the pRO/idRO guides
  give the iRO numbers. Nothing settled.
* Boss teleport from beyond range (T9-405): unknown13, already in
  DEVIATIONS (rude attack) since round 9.
* Volcano/Whirlwind (T9-369 to -371, SoulEclipse 2005, D): what
  `status.lua:382` builds from the iRO guide.
* Repair Weapon, Magnetic Earth: as built or out of scope.

## Count

New: one (the monster breaks and Acid Terror's element), built.
Invented: one capture timestamp. Mis-tagged regions: two.

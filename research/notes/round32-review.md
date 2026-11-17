# Research round 32: review (2026-11-17)

29 rows (T1 6, T7 14, T8 2, T9 7), all PASS in `check.txt` and again here
(`check_rows.py --fetch`); five GameFAQs pages saved under
`research/agy-threads/r32/pages/`, three of them new (JBKMDX's Swordsman/
Knight FAQ 17140, Skygor's B.S. Sacramenti guide 28518, slashgg's Battle
Alchemist guide 33290); 26766 and 30527 were saved in round 12. All four
tasks marked "partly".

## How the round did

* **progress.md matches the rows** this time: same ids, same claims, same
  quotes (round 31's rule held).
* **Captures are real**: 17140 `20100508064115`, 28518 `20100508105559`,
  33290 `20100510003134` are each the page's first capture in CDX.
* **Claims past their quotes**, on 15 of 29 rows: the claim adds what the
  page says elsewhere but the quote lacks. T1-087 (Louitz's and Dupon's
  cells), T7-123 (140-300%, 3-13%), T7-124 (120-200%, 1-5%), T7-125 (the
  potions by level), T7-126 (60-300 s; "Divest skills" where the quote says
  Divest Weapon), T7-127 (the horizontal line), T7-128 ("nine adjacent
  cells": the page's nine cells are the 3x3 *with* the centre), T7-130 (the
  element modifier), T7-131 ("Holy armor"), T7-132 ("reduces"), T7-133
  (the Emperium), T7-134 ("a party"), T8-068 (2 SP), T9-435 (Peco Peco),
  T9-436 (15 SP: the iRO guide gives 13-22 by level).
* **Dates by section, not header.** The rows take each page's last
  version for every section. i3lckdeth's forging section is dated right
  (added 2/16/04, said in `reasoning`). Skygor's sections are not:
  v1.00 is 02/03/04, the formula was "Totally correct[ed]" in v1.03
  (07/11/04) and the Undead confirmation added in v1.05 (08/04); every row
  says v1.05. JBKMDX's Knight section is v4.0 (3/17/03, "updated the
  information to include beta 2"): iRO beta 2, not the game of 2004.

## What it brought

* **B.S. Sacramenti (Priest, 2-1: in scope).** Skygor's section 5.A,
  iRO (Loki): it strikes Undead **element**, Demon race and Ghost
  **element**, each tested ("works on Sohees who are Demon/Water 1 ...
  works on Eggra that are Normal/Ghost 2 ... does not work on Injustice
  which is Undead/Shadow 2"), and its damage is Heal's / 2 *as Holy*
  ("BSS does holy damage, so the final result is multiplied by the
  appropriate elemental modifier"). The iRO guide's Priest page
  (2004-03-27) says the same of the element ("damaged by holy property").
  mod0631, 817 and jAthena all strike Undead or Demon **race**, and their
  skill_db rows give the skill element 0, so ours struck as Neutral.
  **Built** as Skygor tested it (THREADS #22).
  Also from both pages: it needs two Acolyte/Priest helpers beside the
  caster, 10 SP each (jAthena //1020 too, behind
  `player_skill_partner_check`). Ours casts alone, as the ensembles do:
  now said in DEVIATIONS.md. Holy armour stopping Frost Diver's freeze
  (4.D) is the author's guess ("It just appears to work that way"): not
  built.
* **Monsters' elemental attacks (Skygor 4.C, in v1.00, 2004-02-03):**
  "ALL MONSTERS CAN USE A GENERIC ELEMENTAL ATTACK OF THEIR OWN
  ATTRIBUTE_. (The only excpetion are Undead.)" Our tables give one to 196
  of 410 non-Neutral, non-Undead monsters. Not built from this alone; it
  points at the RODataZone skill column of 2004-02-11, whose elemental
  attack pairs are unmapped (THREADS #23).
* **Star Crumb from 5 Star Dust** (i3lckdeth, iRO, 2004-02-16): the
  client's own `data\metalprocessitemlist.txt` says "1000# 10 Star Dust",
  as every emulator does. Ours stays at 10; the guide is wrong.
  Mini-Furnaces sold in Geffen, the other recipes: as ours.
* **Counter Attack 0.3-1.5 s, 2 SP** (JBKMDX, iRO beta 2, 2003-03): the iRO
  guide's Knight page (2004-04-13) gives 0.4-2 s and 3 SP, as ours. Changed
  before our date; nothing to do.
* **Spear Boomerang 3-11 cells, Brandish Spear's rows at 4/7/10, Bowling
  Bash 150-600%** (JBKMDX): as built. A second, older witness.
* **The Alchemist rows (T7-121 to -126, T1-086/087)**: 2-2, slashgg's
  guide of 2004-11-05, iRO; what ours does already (summons, Acid Terror,
  Bomb, Aid Potion, the protections) or the 2-2 quest (the dye-making
  quest as a prerequisite: noted for the 2-2 version, THREADS #25).
* **Sanctuary heals the Emperium** (Anthwerp, pRO, v2.0's WoE update):
  ours heals whatever stands in it that is not Undead or Demon
  (`skill_units.lua`, as mod0631), the Emperium too. Nothing to do.

## Trick

A claim that adds what the page says elsewhere is still past its quote:
check each number in a claim against the quote, not the page.

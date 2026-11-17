# Research round 33: review (2026-11-17)

30 rows, all T9, all PASS in `check.txt` (`check_rows.py`); eight pages
saved under `research/agy-threads/r33/pages/`: seven of RODataZone's job
pages (Swordman, Archer, Thief, Merchant, Magician, Acolyte, Priest) and
Strider VM's Cute Pet Guide (GameFAQs 23652, v1.00, 2003-05-29). T9 marked
"partly"; the open list in progress.md is the queue's own.

## How the round did

* **progress.md matches the rows**: same ids, claims and quotes.
* **Captures are real**: all eight timestamps are in CDX with status 200.
  Three are not the nearest to our date: the Archer page has captures of
  2003-08-11 and 2003-10-03 (the latter checked: no skill details yet, so
  2004-04-23 is the first with ranges), the Magician page 2003-12-16 (the
  round took 2004-02-11), the Merchant page 2003-10-03. The Swordman page
  has one capture before June 2004 (2004-05-02).
* **The generic quote.** Twenty-one rows quote `<B>Range:</B> 9 tiles` or
  the like, which is on the page many times: `check_rows.py` passes them
  whatever skill the claim names. Each was read against its entry here:
  all true (Frost Diver's entry is spelt "Frost Driver" and marked up
  differently, but says 9 tiles).
* **A claim past its quote**: T9-466 "with Bongun omitted" is the agent's
  comparison, not the guide's. Strider's list (kRO, May 2003, "as far as I
  know") has 22 pets; ours (mod0631's pet_db) has 25: Bongun, Desert Wolf
  Baby and Yoyo besides. Older than our date and hedged: nothing changed.
* **Region**: the pet guide is tagged kRO; Strider names no server in the
  part quoted, and 2003-05 predates iRO's open beta by weeks at most. Left.

## What was built

* **Skill reach from RODataZone** (THREADS.md R7, DEVIATIONS.md): read
  whole, not only the rows (every `Range:` line of the seven pages).
  "Bow Range" for Double Strafe and Arrow Shower, "Bow Attack Range" for
  Charge Arrow: the three now reach as far as the bow (5) plus Vulture's
  Eye, where rAthena's 9 had decided. Envenom "Melee Range" and Sand
  Attack "Melee": 1 (mod0631 and 817 3 and 8). Mammonite "Melee Range"
  agrees with what was built. Test `bow_skill_reach`.
* **Aspersio on the Undead** (T9-464): 40 Holy damage past MDEF, the
  element table after, as jAthena //1003 (2004-05-31) has it; RODataZone
  says it in December 2003. Test `aspersio_strikes_the_undead`.

## Not built, and why

* RODataZone's "9 tiles" for every spell and Provoke, 7 for Stone Fling,
  9 for Detoxify: one cell from mod0631's and 817's 8, 8, 10, which agree
  with each other. While checking, I first read rAthena's 9 as "Athena's 8
  counted differently"; that is wrong (Detoxify and Stone Fling go the
  other way: the emulators part skill by skill). Left open in R7.
* B.S. Sacramenti (T9-462/463): "Visual range", Holy armour, "unknown"
  damage to the Undead: what was built from Skygor's test (round 32)
  stands; RODataZone also says it needs a helper on each side (DEVIATIONS.md
  already: ours casts alone).
* Pets (T9-466 to 470): the incubator, the hunger words, Return to Egg are
  what the game does already.

## For the queue

The skill-reach item is answered for what RODataZone covers; what is left
is the one-cell question, asked again with the numbers.

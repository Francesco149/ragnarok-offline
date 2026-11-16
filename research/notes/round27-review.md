# Research round 27: review (2026-11-16)

33 rows (T1 8, T4 4, T9 21), all PASS in `check.txt`; pages saved under
`research/agy-threads/r27/pages/`.

## How the round did

* **Every row's `url` is empty.** The protocol asks for the address each
  saved page came from (`QUEUE.md`, "How to use it"). The pages can be
  matched by file name (the iRO guide's Mage page is byte for byte our own
  capture 20040406194622 in `refs/web/iro-guide-2004/`), but a row has to
  say it.
* **T1-069 to T1-076 are misdated.** They date F0rever Zer0's Hunter test
  walkthrough (GameFAQs 27633) to v1.1, "11/02/03 ... Finished Changing to
  Hunter". The same version history says "1.9* (02/15/04) - Added the
  Hunter Quest ... Changed old 'Changing to Hunter' into 'Aftermath'": the
  v1.1 section was the old instant job change, and the quest text is of
  2004-02-15, as round 5 already read it (THREADS.md #8). Nothing changes:
  the items, the 200 Silver Arrows, the four Job Change Monsters, the
  three minutes and the necklace are what `scripts/npc/hunter_test.lua`
  already does from that walkthrough.
* Grades: SoulEclipse's guides of 2005 are D, as given. Anthwerp's pRO guild
  FAQ (2004-12-29) is graded B; it is later than our window and pRO: C at
  best. Three jAthena rows (T9-355, -367, -368) carry no date; `git log`
  dates them.

## What it brought

* **Frost Diver (T4-033/034): built.** The iRO guide's Mage page of
  2004-04-06: frozen 3 s at level 1 to 30 s at level 10, the chance 38% to
  65% (ours), MDEF shortening both. With mod0631's own `3000 x level`
  and lvlika's "around 20-30 seconds" (round 26), three witnesses against
  817's cast_db (6 to 15 s). Frost Diver only; Frost Nova and Freezing Trap
  keep 817's (DEVIATIONS.md, test `frost_diver_freezes_three_seconds_a_level`).
* **Cards and weapons (T9-348 to -360):** Ragu Memo's jRO card page of
  2004-02-06 gives the Zombie Card 20% (RODataZone's iRO 25%, the client
  15%, ours 15), the Orc Lord Card 30% (the client 25%), the Maya Card 30%;
  it marks the Eddga Card's endure, the Ghostring Card's ghost armour and
  the Great Axe's stun as not working on jRO, the Bloody Axe's faster walk
  and Light Epsilon's Heal as working. jRO is mod0631's server: this
  restates thread 32's iRO/jRO split and settles none of it. Not changed.
* **Castles (T9-364 to -368):** jAthena's box count and investment cost
  are what `castle_lord.lua` already builds from //0974. The economy lost
  when a castle falls (jAthena -5, pRO -20) has nothing to act on here: a
  lost castle is cleared whole.
* **Monster skills (T9-361 to -363):** SoulEclipse (2005): Fabre > Pupa >
  Creamy, eggs hatching, Hallucination's random numbers "and screen
  distortion", Harden/Keeping not moving or attacking: as built. Grade D.

## Count

New: Frost Diver's length (built). Misdated: eight rows. No URL: 33 rows.

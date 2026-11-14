# Round 10 review (2026-11-14)

73 rows in six tasks (T1 12, T5 8, T6 5, T7 25, T8 2, T9 21), four saved
pages (Forever-Zero's Archer/Hunter guide, GameFAQs 27633, the same file as
round 5's; the iRO guide's Crusader, Alchemist and Sage skill pages).

## Invention

* **Rows:** `research/check_rows.py --fetch` run again here: 73 PASS, 0
  WARN, 0 FAIL, as the agent's `check.txt`. No claim is false. Two say more
  than their quote, both right on the page: T7-020 (Magnetic Earth's "1 Blue
  Gemstone and 1 Yellow Gemstone") and T7-024 (Spell Breaker's "2 SP").
  T9-010's "leaving the crossbow with the player" is an inference, and
  right: rAthena's Jason never counts or takes a crossbow.
* **Metadata:** every row from FAQ 27633 is dated 2004-12-14, its last
  version (2.0). The Hunter test was added in 1.9 (2004-02-15), the Comodo
  patch notes belong to the "Future iRO update" section (begun 1.3,
  2003-11-23; edited 1.6, 2004-01-10). Same slip as rounds 7 and 9.
* **`progress.md`:** "central palace (`payon_in03`) or Payon Village
  (`payon_in02`)" puts map names on T1-005 that the FAQ does not give (and
  this client has no `payon_in03`); Cart Revolution "requires 20 Irons and
  1 Banana Juice" (T9-008): rAthena asks 2 Grape Juice, 20 Iron, 30 Sticky
  Mucus, 20 Fly Wings, 5 Tentacles; Banana Juice is Charge Arrow's. It calls
  rAthena's `prontera.txt` an "official Aegis script".

## What is new

About 9 of 73 rows.

* **T9-001, T9-002 (thread 28).** FAQ 27633 on Sandman: "It is invisible
  only to people who were off screen when it was planted, or those who go
  off screen and come back." And iRO's Comodo patch: "Invisible traps are
  visible to only the caster and those who were on the screen at the time
  the traps were installed." The client's list of others' ground units
  (shown the first time, hidden the second) is how that was done.
* **T8-001, T8-002 (thread 22, queue T8).** Skid Trap's 6 to 10 cells came
  with iRO's Comodo update (Episode 3.0), a "skill balance patch" whose
  whole list the FAQ copies. Not a drift between December and March. Read
  against euRO's May-2004 client text: part of the patch is there (Ankle
  Snare by AGI, Flasher and Freezing Trap two Traps), so euRO had it by
  May; whether with New Comodo is open (THREADS #22).
* **T9-004 (thread 26).** rAthena's `aldeg_cas05 188,49` lands on
  `165,228`, not mod0631's and 817's `165,288` (off the 240x240 map). Our own
  warp list agrees (the partner tile is `165,232`; every pair there lands
  four cells off): fixed in `scripts/npc/castle_warps.lua`, and `check` no
  longer lists it.
* **T9-006.** rAthena's `pay_dun04` arrival is `189,43`; we knew `192,43`
  is floor. Noted in #26, nothing changed (Athena's random-cell fallback
  stays).
* **T6-002 to T6-004 (thread 13).** rAthena's hairdresser asks 1000z (mod0631:
  5000z); its tailor turns everyone away as mod0631's does, and its v2.4
  note says cloth dyeing was "not implemented on official servers". Tier
  D, but it agrees with mod0631.

## Padding

* **T1** (12 rows) re-quotes FAQ 27633's Hunter test: round 5 found it and
  `scripts/npc/hunter_test.lua` is built from it.
* **T7** (25 rows) quotes the iRO guide pages already on disk (the same
  2004-03-27 captures) and 817's unit settings. It is what the queue asked
  for, but without the status word per skill ("implemented", "effect
  only", "commented out", "absent") and with 817 rows for six of ten.
* **T9-015 to T9-021** are rAthena's monster-skill rows, which
  `tools/import/mob_skills_faq.py` already reads. T9-007 restates THREADS
  #26. T5 restates #24 (Gotanblue, 10000z, `tur_dun01 157,39`).

## Lesson

The saved page paid again, not the rows: the same FAQ round 5 used for one
section had a patch list and two lines on traps in other sections. When a
page comes back a second time, read it whole for the open threads.

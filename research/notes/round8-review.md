# Round 8 review (2026-11-14)

68 rows (T1: 14, T9: 54). `research/check_rows.py --fetch`: 68 PASS, the
same as the round's own `check.txt`. Every quote is where its row says.

The round's files went into git before this review, by accident: the
commit `a984ba1` (packet layouts) ran `git add -A` while they sat
untracked. Its `pages/` and the `rows/pages` symlink are git-ignored and
stayed out.

## Invention and errors

* **One wrong claim.** T1-012 says eAthena 1.0's Knight arena "times out
  at 184000 ms" from `addtimer 7000, "TimerKnt::OnTimer184000"`
  (`npc/jobs/2-1/knight.txt:974`). That line runs the timeout label seven
  seconds after the last kill. The arena's clock is the NPC timer's own
  labels, `OnTimer182000` and `OnTimer184000` (`:1008`, `:1012`): about
  three minutes, as the queue already said.
* **Mis-graded: T9-001 to T9-015 are grade D, not B.** They come from
  OpenKore's `tables/packetlist.txt` at `615f5c3b7`, a checkout of
  2026-11-09; the names arrived in 2018 (`ad0d670fe`), and its handler
  column names Hercules functions. The task asked for tables of 2004. All
  fifteen lengths agree with the May-2004 exe's own table, so the
  identification holds as a later source.
* **Claims past their quotes**, facts right when checked: T1-001 (item
  names for ids), T9-016 (an item list under a quote that only gives the
  job level), T9-017 (names for ids), T9-039 ("deletes only 1 of each":
  true, `conf/npc_job_thief.txt:415`, `:418`, not in the quote).
* **Overstated.** T1-009: jAthena's Sir Grey passes on the second talk
  (`if (change_knight == 14) goto LOK; close;`), and its comment says why:
  `合格条件不明なため、2回目で無条件合格` ("the pass condition is unknown, so
  a pass without conditions the second time").
* `progress.md` invents nothing this time.

## New to THREADS.md

* **0x01f1 to 0x01ff** (T9-001 to -015): AC_NOTIFY_ERROR, ZC_UPDATE_CHARSTAT2,
  ZC_NOTIFY_EFFECT2, the second trade request and answer, the four
  adoption packets, CA_LOGIN3, CH_DELETE_CHAR2, the repair list, request and
  answer, ZC_HIGHJUMP. Grade D names, lengths confirmed by the exe.
* **Quest skills side by side** (T9-016 to -037). Arrow Crafting, Change
  Cart, Crazy Uproar, Holy Light and Energy Coat take the same items in
  817, mod0631 and on RODataZone. Three differences:
  * Cart Revolution: 817 asks 15 Iron and takes 15
    (`skillquests/cartrevolution.txt:7`, `:37`, `:65`); mod0631 picks one
    of three sets at random (23, 20 or 15 Iron with other counts,
    `conf/npc_event_skillget.txt:1085-1087`); RODataZone (2003-10-03) gives
    mod0631's second set: 20 Iron, 30 Sticky Mucus, 20 Fly Wings, 5
    Tentacles, 2 Grape Juice. We play 817's.
  * Gershaun first wants a Banana Juice before he talks business
    (mod0631 `:1100`, RODataZone); 817 has no such step.
  * Charge Arrow: RODataZone (2004-04-23) adds "1 Crossbow" to the list;
    neither emulator asks for one.
* **The Thief's mushrooms** (T9-038, -039): 817 deletes up to 99 of each
  and pays by count; mod0631 takes one of each and pays 100z.

Repeats: T1-002 to -014 restate the queue's own bullets; T9-040 to -043
(Yosuke's cell, the parcel ids) are in THREADS #27; T9-044 to -046
(Shockwave Trap's Traps) are R6; T9-047 to -053 are the sources of #24 and
#26 as THREADS already has them; T9-054 adds nothing. About 22 of 68 rows
are new.

## What follows

* Cart Revolution, Gershaun's juice, Charge Arrow's Crossbow: decide with
  the owner's rule of 2026-11-14 (best approximation, later sources where
  the period ones part); THREADS #7.
* Sir Grey's pass condition: no period source knows it. Per the same rule
  I looked at the later transcription: rAthena's `npc/jobs/2-1/knight.txt`
  (`d4b8e7b8f`, "PGRO TEAM (Aegis)", "[Official Conversion]", v3.3)
  scores his questions (+5, +10, +15, -5 by answer) and passes a total of
  0, 5 or 10; anything else sets the quest back a step to try again.
  Grade D for when, but the best approximation there is (THREADS #8).

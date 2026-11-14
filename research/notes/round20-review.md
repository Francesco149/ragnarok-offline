# Round 20 review (2026-11-14)

28 rows: T8 18, T1 6, T9 3, T7 1. The agent's checker run: 28 PASS. Ours
with `--fetch --pages research/agy-threads/r20/pages`: 28 PASS.

## Invention

**No quote, URL, hash or date invented.** The jAthena revisions are what
the rows say (//1012 `e274508` 2004-06-08, //1047 `f79e4c6` 2004-09-14,
//1092 `79b0303` 2004-12-30, whose message does carry "1088のアイスウォール
の変更を巻き戻し(本鯖に合わせる)"); OpenKore `1cd0757` is 2004-02-29; the
Wizard FAQ rows are dated by v2.1 (2004-07-30), the version line that
"Added Wizard Quest Section", as the queue asks. The 817 files' dates are
the files' own (2004-02-18). What slipped:

* **Line numbers**, which the checker's window hides: 817 `db/cast_db.txt`
  Poison React is line 96, not 103; `map/skill.c` Ice Wall's decay is 6156
  (row: 6203) and its damage 3725 (3758); `map/battle.c` 3465 (3464); the
  Ice Wall `count=5` is not at 2904 (2880 or 2959). jAthena's Spring Trap
  is `skill_db.txt:133` (142), Venom Splasher `skill_cast_db.txt:101` (108).
* **A claim past its quote**: T8-033 adds "requiring under 2/3 HP", which
  is on the FAQ's page two lines above the quoted part. True.
* **Names**: the Wizard FAQ is "by Kiruwa" in the saved v2.3 (2005-02-13);
  its introduction is signed Ghent. The rows' "Ghent" is the author of the
  quoted section, fine.
* Grades again B for client text, emulator code and guides alike;
  "official / OpenKore" for a bot's portal list.

## What is new

About seven rows of 28; none changes the game tonight, three sharpen threads.

* **Venom Splasher's threshold changed in 2004** (T8-032, T8-033, T8-035,
  and this client's own text, read for the review): our client (Dec 2003)
  "HP at least by 1/3", the iRO guide (2004-03-27) "less than 1/3"; euRO's
  May-2004 text "at least by 2/3", anjingbalap's iRO FAQ v2.2 (2004-06-02)
  "less than 2/3", target poisoned, a timer over it, cancelled if it is hit
  first, 25% splash on 3x3. jAthena //1047 (2004-09-14, tier later) gives
  the timer: 5.5 s to 10 s by level, 15 s to 60 s before it can be used
  again (T8-034). So the period skill is the 1/3 one; the timer has a source
  now, a late one. THREADS #22.
* **Ice Wall can be attacked** in 817 (2004-02-18): `battle_check_target`
  treats unit 0x8d as an enemy (`battle.c:3465`, 0 = enemy in its own
  header), damage comes off its durability (`skill.c:3725`), and it decays
  5 every 100 ms (`skill.c:6156`), as the iRO guide's page says (T8-021 to
  025). ROADMAP lists "attacking an Ice Wall" as not built for want of a
  source; 817 is one. Whether this client lets the player click a skill
  unit to attack it is the next question. THREADS #22.
* **The Wizard test failure** (T1-050): saved and put outside Geffen Tower
  on iRO by July 2004, which `wizard_test.lua` (817) already does. Its quiz
  versions match ours (T1-053 to 055 against `wizard_test.lua:301-345`). The
  "worn out scroll" skip after five failures (T1-051) is iRO, July 2004:
  later than our window, noted in THREADS #8.
* **OpenKore's iRO portal list of 2004-02-29** (T9-246 to 248): no portal
  of `tur_dun05`, and none of the four `prt_in` exits at x 280-285,
  y 36-130 that THREADS #26 asks about (only `prt_in 282,179`, ours too).
  Two arrivals of Payon Cave 3F into 4F are listed (202,206 and 40,37), not
  the walled `193,43`. Second witness for #26's three items.

Already known: Poison React's 30 s against 817's 30 min (T8-027/028), Remove
Trap and Spring Trap (T8-029 to 031), Skid Trap's 1-5 cells and Ankle Snare's
AGI in the May text (T8-037/038), Strip Weapon's DEX (T7-031), the Turtle
Island sailor's arrival (T9-247), Ice Wall placement under units (T8-026, a
December 2004 revert).

## Lesson

Read the client's own text for any skill a round dates: our CD settled
Venom Splasher's threshold in one lookup, where the rows had a March and a
May source pulling apart.

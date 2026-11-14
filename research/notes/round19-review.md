# Round 19 review (2026-11-14)

28 rows: T8 16, T1 7, T9 5. The agent's checker run: 28 PASS. Ours with
`--fetch --pages research/agy-threads/r19/pages`: 28 PASS.

## Invention

**No quote and no URL invented.** Every cited capture is in CDX
(`rodatazone.com/mechanics/combat.php` 20040209083240, `substats.php`
20031211193200, GameFAQs 20092 at 20100508105531, 29457 at 20100508125837),
and both jAthena revisions are what the rows say (//0988 2004-05-25, //0991
2004-05-26). What slipped:

* **Weggy's 2-1 job changes dated to 2004 again** (T1-047, 048, 049): "simply
  talk to the man inside" (Blacksmith), "pay the guildmaster 666 zeny"
  (Assassin), "talk to the Wizard there" (Wizard), dated v6.5, v6.1 and v6.3
  (2004-05/06) by the class section's update line. Weggy's own v6.1 line:
  "I just realized that Gravity also added more in-depth quests to the 2-1
  classes, so i'll hopefully be able to update those soon". These are his
  text from before his year-long break, round 7's slip, against the queue
  rule written for it. `progress.md` does call them "mid-2003"; the rows do
  not.
* **File dates**: the 817 rows are dated 2004-02-19 (the release); the files
  are `map/battle.c` 2004-02-18, `npc/jobs/merchant.txt` 2004-02-03,
  `db/skill_require_db.txt` 2004-01-28. mod0631's `conf/npc_job_merchant.txt`
  is 2003-08-10, dated 2003-12-18.
* **Claims past their quotes**, all three true: T8-005's "matching official
  server behavior" is in the commit message (//0991: "本鯖仕様に変更"), not
  the quoted code; T8-012's "without arrow ATK" reads an absence; T9-241's
  "remained in player inventory" is more than "you could carry them over".
* **Region and grade**: T8-005's region is "official / jAthena"; every row is
  grade B, emulator code and client text alike.
* **Padding**: T1-045 and T9-241 are the same quote.

## What is new

About four rows of 28; two changed the game.

* **Arrows add ATK** (T8-011, RODataZone's iRO combat page, capture
  2004-02-09): "rnd( 0 , ArrowATK - 1 )" after the size modifier. THREADS #22
  had asked this since the battle port; the page had been on `data:` since
  round 12 and nobody had read its bow line. Its CDX history dates it better
  than the row: the capture of **2003-12-16** (two days before our client)
  already has the same formula; the one of 2003-09-10 added the full ATK.
  817 adds 0 to ATK at the same place (`battle.c:2318`) and all of it on a
  critical (`:2269`); mod0631 comments the line out (`battle.c:913`), which
  is what the game followed. Built on RODataZone's bound with 817's
  critical. Both captures are now in `data:refs/web/rodatazone-2004/`.
* **The Talkie Box is silent for its own Hunter** (T8-005, T8-006): the iRO
  guide's Hunter page (2004-03-27), "The message is not played for the
  caster", and jAthena //0991, whose commit message calls it the official
  server's way. Built, with a test that fails without it.
* **The Master Hunter stood in Payon's Archer guild** (T1-043, Weggy v6.2,
  iRO 2004-05-25, "though i've heard he can be in the palace as well"):
  where `hunter_test.lua` has him (`payon_in02`). Queue T1 narrowed.
* Client text: the two delivery boxes of the Merchant test (1083, 1091) have
  the same description and weight 120 (T9-244/245): either id is fine.

Already known (THREADS #21, #22, #27, R6, or the queue itself): Talkie Box's
600 s and //0988's packets (T8-007 to 010), Sightrasher's lines and reach
(T8-015/016), Storm Gust's third hit (T8-017), Shockwave Trap's two Traps in
the client text against one in both emulators (T8-018 to 020; R6's table:
client text alone stays an older state), Edge's "pay a small fee" (T1-046),
the Hunter test's four monsters and the water (T1-044, round 5), Kay's two
parcel ids (T9-242/243).

## Lesson

The gold was again on our own disk, one capture away: when a row quotes a
dated page, list its CDX history and read the capture nearest our date.

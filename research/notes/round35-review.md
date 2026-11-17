# Research round 35: review (2026-11-17)

41 rows (15 T1, 26 T9), all PASS in `check.txt` and on my own run of
`check_rows.py`. No pages saved: every row quotes a file on `data:`.
`progress.md` is written by the round's `build_r35.py` from the rows and
matches them.

## How the round did

* **Two claims read the script backwards.** T1-100 and T1-101 call
  jAthena's Dancer quiz "a menu bug" where option 3 (Martine) and option 1
  (one lighthouse) "fall through" to `+10`. That is how the whole quiz
  marks its right answers: in jAthena's `menu`, `-` goes on to the next
  line, which is the `+10`; the wrong answers jump past it (questions 1 to
  10 of both sets, //1046 lines 380-472). jAthena's answers are Martine
  and 1; our test (`scripts/npc/dancer_test.lua`) already has them.
* **One date found nowhere.** The eleven rAthena rows are dated
  2004-10-01. The files carry no date (dancer.txt's header gives
  versions, not dates) and their git history starts 2006-01-29
  (`git log --diff-filter=A`). Grade D is right.
* **Nothing new in T1.** The Alchemist rows (Darwin's Illusion Flower,
  Job Level 50 skipping Gregory, the fee) are what the Alchemist test was
  built with from jAthena //1046; the Dancer rows (Moo, question 9, the
  89-second timer, the step sensors) are rAthena's, already set beside
  jAthena in the test's header; Bain and Bajin's rAthena errand is the
  later Aegis one (ours: jAthena's, replacing 817's). The Monk rows are
  rAthena only: jAthena's Monk is a stub marked 不完全 whose elder
  job-changes at once (//1046 `npc_job_15monk.txt`, 192 lines). The
  queue's T1 item asked for players' walkthroughs; none came.
* **Nothing new in the T9 card rows.** The Zombie, Orc Lord, Maya,
  Doppelganger and Archer Skeleton Cards and the Bloody Axe, Light
  Epsilon, Ghostring and Eddga Card splits are thread 32's iRO/jRO split,
  read in rounds 27 and 28 from the same pages. Mama's Knife, Spike, the
  Weeder Knife and Caesar's Sword: the client's text alone.

## What was built

One row led somewhere none of the rounds went: Light Epsilon's Heal is
level 1 in the client's text, RODataZone and Ragu Memo, and level 3 in
ours. Ours is mod0631's, and jAthena's history says where it came from:
**//0556** (`8ec48b3`, 2003-11-21, no commit message) rewrote 56 rare
weapons at once: new bonuses on 48, ATK on 23, weight on 5. mod0631
carries most of it. Set beside every witness (the commit's rows are
what `weapon_rows_before_0556()` in the importer reads):

* the client's text gives the earlier ATK on all 23 and the earlier
  weight on all 5;
* RODataZone's iRO weapons.js (2004-02-11) the earlier ATK on all 8 rows
  it lists (Muramasa 170, Assassin Dagger 125, Infiltrator 100, Gungnir
  90, ...);
* Ragu Memo's jRO weapon pages (2003-12-18: a month after the commit, and
  mod0631's own server) the earlier row on all 21 it lists: Muramasa ATK
  170 "CRI + 20", the Assassin Dagger 125 "[闇属性] MSP + 30", Mysteltainn
  160, Masamune "FLEE + 30 DEF = 0" without STR -5, the Ice Falchion
  "コールドボルトLv1", Zephyrus "サンダーストームLv1", Light Epsilon
  "ヒールLv1". The other 35 are not on its pages (not yet obtainable on
  jRO).

So //0556 was jAthena ahead of jRO, not jRO's state, and not euRO's in
December 2003. Every row it changed now takes its row from before it
(weight, ATK and script), read from jAthena's git in
`tools/import/athena_import.py`. Where the period sources name an effect
the earlier row lacks, it is given: the Katzbalger's DEF + 10 (client and
Ragu Memo: the one //0556 addition they show), Gungnir's HIT + 30 with no
property (client, RODataZone), the Cutlas's Bash 5 (all three), the Ice
Falchion's Cold Bolt 1 and the Fire Brand's Fire Bolt 1 (RODataZone, Ragu
Memo for the Falchion), the Schweizersabel's Lightning Bolt 1 (the client
names the bolt, no level; 1 as every other). The Crescent Scythe loses
//0556's HIT + 10. The Sabbath regains its Dark property, which //0556
had dropped. The Bloody Axe's walk and the other thread-32 splits stay as
they were.

## Trick

When a value of ours disagrees with every period witness, find the
emulator commit that wrote it (`git log -G` on the line) and set the
whole commit beside the witnesses, not just the row: one Heal level led to
55 more weapons from the same commit.

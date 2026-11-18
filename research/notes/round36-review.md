# Research round 36: review (2026-11-18)

40 rows (13 T1, 27 T9), all PASS in `check.txt` and on my own run of
`check_rows.py`. No pages saved: every row quotes a file on `data:`.
`progress.md` is written by the round's `build_r36.py` from the rows and
matches them.

## How the round did

* **One claim false.** T9-536 to T9-538 say jAthena //0970 (`7189b01`,
  2004-05-14) *added* Mental Breaker to the Clock, Nightmare and
  Phreeoni. It renamed them: the same rows were there before as
  `SP減少攻撃` ("SP Decrease Attack") and //0970 calls them `精神破壊`
  (`git show 7189b01 -- db/mob_skill_db.txt`). Our table already takes
  them from //0953 (2004-04-27).
* **Claims past their quotes.** T1-117's "waiting room with 10 slots"
  (the `waitingroom` line is not in the quote); T1-114's "Schweizers
  Academy" (the quote names no academy); T1-116's "Crystal of Wisdom" for
  慧眼, which is item 1007, Necklace of Wisdom in our database (mod0631's
  JName column, as the rules ask). The romanized names (Kayron Glick,
  Metheus Sylph, Klaytos Vermo) are the round's own.
* **One date found nowhere, again.** T9-549's rAthena row is dated
  2004-10-01, as round 35's were (rAthena's history starts 2006-01-29).
* **817's file date is right.** 2004-02-06 is `db/mob_skill_db.txt`'s own
  date in the 817 zip (`unzip -l`).
* **Nothing new in T1.** The Sage rows are jAthena //1046's test, which
  was built the same day from the whole script (`scripts/npc/sage_test.lua`,
  read line by line for that); the Bard and Rogue rows say what THREADS.md
  #25 says (stubs marked 不完全). No player's walkthrough came, which is
  what the queue asked for.
* **Little new in T9's rows.** Every 817 row quoted (Energy Drain on six
  monsters, the Defence-ignoring Attack on four, the Thief Bugs', Tarou's
  and the Giant Spider's elemental attacks) is in mod0631 too, word for
  word: our table has them. The Ancient Mummy's Envenom (Ragu Memo) and
  Muramasa's rows (Ragu Memo, //0556) were taken in rounds 16 and 35. The
  May-2004 client's item text keeps the pre-//0556 numbers (Muramasa 170,
  the Assassin Dagger 125 with MaxSP + 30, the Infiltrator 100, Gungnir
  95), as the 2003 client's does; item text is not the server's table, so
  it does not date the rework.

## What was built

The 817 rows were not new, but setting them beside our *guessed* rows was:

* RODataZone's "HP Absorption" is on exactly the six monsters mod0631
  gives Energy Drain (`HP吸収(魔法打撃)`, skill 200): Bathory, Deviruchi,
  Evil Druid, Joker, Wraith, Dark Lord. The import had read the name as
  Blood Drain (199) and guessed a second drain onto all six.
* RODataZone's "SP Decrease Attack" is on exactly the three monsters
  jAthena gave skill 159 under that very name (`SP減少攻撃`) until //0970,
  and that the FAQ lists with Mental Breaker. The import had read it as
  Energy Drain and guessed it onto all three, at a 100% rate below 70% HP.
* The field Thief Bugs (1017, 1021) had a guessed Dark Element Attack
  (level 2, the commonest row); mod0631 has the row for the same monsters
  under their second ids (1053, 1054): level 1. The importer now takes the
  same monster's own row before anyone else's.

`research/rdz_mob_skills.py` maps both names anew; nine guessed rows are
gone and two are mod0631's (commit `eb954b4`). The Sage test's written
test (built the same night) also turned up a porting slip in Energy Coat,
now jAthena's (`1e9fd88`).

Two leads for the queue: jAthena //0505 (2003-10-30) has Peco Pecos
summoning Peco Peco Eggs (`idle`, 0.1%, once an hour), which mod0631 two
months later lacks; and "Piercing Attack", which the import reads as skill
158, sits in RODataZone on Cat O'Nine Tails, Dark Illusion and Maya
Purple, the three that mod0631 gives the Defence-ignoring Attack (170):
the same name mix-up may be there too.

## Trick

When a table maps another source's skill names to ours, check every
guessed row against what the period table already gives the *same
monsters*: a guess that lands on a monster which already has a skill of
that kind is usually the name read wrong. Old emulator revisions keep the
names other sources used (jAthena's `SP減少攻撃`).

# Round 9 review (2026-11-14)

53 rows (T1: 23, T9: 27, T10: 3). `research/check_rows.py --fetch`: 53
PASS, the same as the round's own `check.txt`. Every quote is where its
row says. The `rows/pages` symlink is git-ignored and stays out.

## Invention and errors

* **No false claim.** Four go past their quotes and hold when checked
  against eAthena 1.0 (`data:refs/rathena` `706e78509`, 2004-11-04):
  T1-001's retry mark ("over 80", `npc/jobs/2-1/blacksmith.txt:489`),
  T1-018's Job-50 Priest skipping the pilgrimage (`priest.txt:194-199`),
  T1-022's retry with no fee (nothing taken in `wizard.txt:1080-1130`),
  T1-021's minute for the door guards. That one is loose: the timer jumps
  to 120 s, says "1 minute left" at 124 s and fails at 184 s
  (`wizard.txt:1221`, `:1352-1388`), 64 seconds, where the claim says "a
  180,000 ms timeout". T1-016's "Huey" is not in its quote but is in the
  file 19 times.
* **Metadata, not checked by the tool:**
  * every row has an empty `region`. The two GameFAQs guides behind T9
    are **pRO** (the Philippine server): unknown13 started on pRO in
    February 2004 and says the list "is currently the data on iRO";
    blood3d writes of "PRO" throughout;
  * blood3d's rows (T9-021 to -027) carry the guide's last date,
    2004-09-26. Its version history dates the sections: the Doomed
    Swords came in v1.01 (2004-03-06), Mixture and Counteragent were
    "fixed" in v1.05 (2004-08-08), the juices are from v1 (2004-02-25 to
    -28). Grade A is generous for a pRO guide of autumn 2004.
* **`progress.md` invents again**, in item names it adds to ids:
  T1-014's Job-40 prize is "Jur [2], Stiletto [2], or Jamadhar"; the ids
  are 1250 Jur [2], 1217 Stiletto [3] and 1252 Katar [1] (Jamadhar is
  1254). T10-002 becomes "test items scale directly with the number of
  correct answers"; the quote says only that right answers are rewarded.

## What is new

* **T1: nothing.** All 23 rows restate THREADS #8 and the headers of
  `scripts/npc/blacksmith_test.lua` and `assassin_test.lua`, which were
  written from the same two files; T1-008 quotes our own script back. The
  task asked for a player's account of the Blacksmith and Assassin tests
  before autumn 2004; none was found.
* **The two shop names** (T9-001 to -003, thread 11). Prontera's 野菜商人
  at `48,58` is "Vegetable Gardener#prt" in rAthena's
  `npc/merchants/shops.txt`; Juno's 王窓商会 is "Equip Dealer" in eAthena
  1.0 (at `205,104`) and "Mr. King's Shop#yuno" in rAthena. Both later.
  Taken, with the owner's rule of 2026-11-14 (later sources where period
  ones are silent): "Vegetable Gardener" and "Mr. King's Shop", the
  second because it translates mod0631's own name where eAthena's is a
  role (`tools/import/athena_import.py`).
* **A period monster-skill list** (T9-004 to -020). The rows pick 17
  pairs for ten monsters, every one of which `data/db/mob_skills.toml`
  already has. The page itself is worth more: unknown13's FAQ (GameFAQs
  29608, v1.0, April 2004) lists every monster skill iRO had then, and
  `research/mob_skills_faq.py` compares all of it with our table: of 325
  (skill, monster) pairs, 209 are in ours, 96 are not, 20 name monsters
  the script cannot match. The gap is not scattered: **mod0631 gives no
  monster at all** Endure, Increase or Decrease Agility, Piercing Attack,
  Mental Breaker, Napalm Beat, Status Recovery or Pneuma, and the FAQ has
  them on 3 to 11 monsters each (Endure on 11), and quotes iRO's rule that every boss
  uses Two-Hand Quicken, Increase Agility, Guided Attack and Piercing
  Attack. euRO's "New Comodo" (January 2004) announced "added monster
  skills". THREADS #23.
* **Recipes as a pRO player had them** (T9-021 to -027): Banana Juice,
  Counteragent (3000z), Mixture (4000z), Executioner's and Mysteltain's
  ingredients agree with the game. **Grimtooth does not**: the guide
  asks for 10 Bloody Edge (Executioner); mod0631
  (`conf/npc_event_ma_sword.txt:101`) and rAthena's official conversion
  (`npc/quests/doomed_swords.txt:918`) take 10 Blade Lost in Darkness
  (Abyss Knight). Two against one: the game keeps mod0631's.
* **T10** (Mistiknoname3k's Assassin guide, v1.9 of 2003-12-13, iRO): a
  new character gets a Knife and a Cotton Shirt, and a Thief by the
  test's advice goes to Morroc with extra zeny. Both are in the game
  already (mod0631 `char/char.c:408`; `scripts/npc/novice_result.lua`);
  the guide is the first period witness for them outside Japan.

About 23 of 53 rows are new (the shop names, the monster list, the
Grimtooth difference); the rest confirm.

## What follows

* The missing monster skills (THREADS #23): the best approximation is
  the FAQ's pairs, with levels and rates from a later table where the
  period has none (owner's rule). Not done in this review: it changes
  every fight, so it wants its own session and a playtest.

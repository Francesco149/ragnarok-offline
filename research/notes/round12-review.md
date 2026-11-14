# Round 12 review (2026-11-14)

63 rows in three tasks (T1 25, T4 19, T9 19), 21 saved pages: GameFAQs
21546 (PsychoBattousai, Swordsman/Knight, v1.3 of 2003-02-25), 31808
(Maruko, Hybrid Knight, pRO, v1.0 2004-07-13 / v1.1 2004-08-08) and ten
other GameFAQs guides that no row cites (one, `gamefaqs_thief_node_16059`,
is empty); eight pages of sharonov.ru (Russian, 2009). Rows 05:25, then
`progress.md` and `check.txt` at 05:34: finished when reviewed.

## Invention

* **Rows:** `research/check_rows.py --fetch --pages r12/pages` run here:
  63 PASS, 0 WARN, 0 FAIL, as the agent's `check.txt`. No quote invented.
* **Claims past their quotes (6):**
  * T9-012, T9-013, T9-016 name the wrong items, shifted along by one id:
    532 is Banana Juice (claim: Well-baked Cookie), 538 is Well-baked
    Cookie (claim: Piece of Cake), 539 is Piece of Cake (claim: Hinalle
    Leaflet, which is 520). `progress.md` repeats them.
  * T9-003, T9-004 add a second serial number ("or 2328137", "or
    2191737") that the quote, cut short, does not have; T1-017 says "6"
    targets for a quote that lists five.
* **A reading turned backwards (T1-011).** The quote is item set 4 of the
  Blacksmith's orders "after June 2008" ("Варианты лута по версии после
  июня 2008"); the same page then gives the "old variants" ("Старые
  варианты лута"), among them "8 Iron Ore + 1 Trunk + 2 Blue Gemstone + 1
  Arbalest". The row's reasoning, "confirms Eye Bandage over Trunk", is
  the opposite of what the page says: the old list is eAthena's, which
  `scripts/npc/blacksmith_test.lua` already asks.
* **Metadata:**
  * The Sharonov rows (T1-002..025, T9-001..005) are graded D, rightly,
    but their region is "official (Sharonov)". Each page ends with the
    English source it translates: "Английский вариант расположен на
    http://irowiki.org/wiki/Knight_Job_Change_Guide". They are iRO Wiki
    of 2008-09 in Russian: iRO as it was then (one lists Hugel), not
    2004.
  * File dates: T9-006 and T9-008 (mod0631 `npc_job_thief.txt`,
    `npc_job_merchant.txt`) are dated 2003-12-15; the files are of
    2003-08-10. T9-007 and T9-009 (817's `thief.txt`, `merchant.txt`)
    are dated 2004-04-20; the files are of 2004-02-03.
  * T9-019 cites rAthena `prontera.txt` line 28; the lines are 53-57.
  * The Maruko rows are region pRO (he plays on pRO Loki). His credits say
    the appendix is not his: "RO Datazone ... The Appendix also has
    information from your site." See below: it is RODataZone's page of
    early 2004, which in turn takes it from a Japanese site.

## What is new

About 10 of 63 rows. Padding: T9-010..017 (the doll child's trades: built
from that very mod0631 file: `scripts/npc/doll_child.lua`),
T9-006..009 and T9-018 (in THREADS #26 and #27 already, from the same
lines), T1-001 (no Knight test in iRO in February 2003: THREADS #5 has
it). T4's VIT DEF, refine DEF and cast-time formulas are what the game
does already.

* **The ailments (thread 21): the best of the round, one step behind the
  row.** Maruko's appendix is RODataZone's "Status Effects" page
  (`http://www.rodatazone.com/mechanics/status_effects.php`), saved in
  `data:refs/web/rodatazone-2004/` (captures of 2004-02-10 and 2004-06-02,
  and the directory listing of 2004-01-19, which does not have the page
  yet: it was written between 2004-01-19 and 2004-02-10). It says "This
  information comes from Ragu Memo", a Japanese player-research site: the
  same research jAthena's resistance came from (//0729 of 2004-01-21,
  THREADS #21). Its chances are 817's to the letter, `BaseChance *
  (97 - VIT - LUK/3) / 100` and the like (817: `100 - (3 + vit + luk/3)`),
  for stun, silence (VIT), freeze (MDEF), sleep and blind (INT). Where it
  differs from what we do:
  * Stone Curse resisted by INT (817: MDEF), lasting 17 - MDEF/20 s;
  * Poison resisted by `VIT + LUK/5`, 3% of max HP every 3 s down to a
    quarter (817: 3 + 1.5% every second for a player), 10-30 s on a
    monster and 5-60 s on a player;
  * Blind lasting `30 - BaseLevel/10 - INT/15` s, at least 15;
  * Stun lasting `3 * (97 - VIT - LUK/3) / 100` s;
  * Curse: "LUK helps, formula unknown". Effects: Blind cuts hit and flee
    by 25%, a blind monster sees 1 cell; Frozen and Stone give -50% DEF
    and +25% MDEF.
  Not built: 817's lengths come per skill from its `cast_db.txt`, so
  setting these against ours is a skill-by-skill job. Noted in THREADS
  #21 as the dated source to sharpen against.
* **The 2-1 tests, as iRO had them in 2008-09 (grade D, for sharpening
  only):**
  * Knight: Sir James's answers are jAthena's, which we ask (Flamberge;
    Provoke Lv 10 not needed for Bowling Bash; Spear Boomerang not needed
    for Brandish Spear; Zephyrus for a Nightmare). Sir Andrew picks one of
    four lists (jAthena's set 1 or eAthena's set 2, each whole or less one
    item); ours asks set 1 only. Seven Awakening Potions at the change.
  * Blacksmith: the old orders and the level-4 answer (+4 "before the
    2008 patch") are eAthena's, which we ask; the level-3 answer is +5.
    The 2008 list (Eye Bandage) is a later change, not a period one.
  * Assassin: rewards (job 50: a choice of Jur, Katar, Main Gauche or
    Gladius; below: one of five at random), the Necklace of Oblivion.
* **Thread 27:** Yosuke at `prt_fild00 208,218` (iRO Wiki, 2009), the
  cell 817 has and we keep. Thief mushrooms count 3 points a Net and 1 a
  Gooey (a score, not the pay asked about).

## Lesson

The same as rounds 9 to 11, one level further: the gold was the source
behind the saved page. A player guide's credits named RODataZone, and
RODataZone's page named its own source and could be dated by the site's
directory listing. When a guide's numbers look like a technical site's,
look for that site's own capture before dating them by the guide.

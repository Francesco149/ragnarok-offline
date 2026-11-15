# Round 23 review (2026-11-15)

27 rows: T1 8, T4 6, T8 5, T9 6, T10 2. The agent's checker run: 27 PASS.
Ours with `--fetch --pages research/agy-threads/r23/pages`: 27 PASS. Every
cited Wayback capture is served.

## Invention

**Rows: none.** No quote, URL or capture invented; no claim past its quote
that changes anything.

**progress.md: a lot, of the round-2 kind.** Its row list does not match the
rows (ids swapped between T1-061..066, T9-282/283, T10-023/024; one row
"T9-283" it describes is H0BGOBLIN, the real one is Mistiknoname3k), and it
gives sources no row or page has:

* a jAthena commit `525287f32997`, "//1176 by shadow", 2004-06-25, quoting
  `int area = 5, count = 41;`. No such hash and no //1176 in
  `data:refs/jathena`; the string is in no revision. The row (T8-047) cites
  `13854f2` and quotes the real `count=41;`;
* 817 `map/skill.c:4077` with `static const int dir_x[24]`: no `dir_x` in
  817's `skill.c`; the row has line 3171 and the real `dx[]`;
* quotes not on the saved pages: "HP: 3518 / 1", "DO NOT LET THE MARINE
  SPHERE SELF DESTRUCT!", a player report that the Marine Sphere could
  explode on you without failing the test (Kiruwa's FAQ has no "explode"),
  and Ragu Memo "約17秒。MDEFで僅かに減少?" (the page: "持続時間は約17秒で、
  対象のMDefによって微妙に減少", which the row quotes right).

Slips in the rows, unchecked by the tool:

* **Dates.** T1-061 dates Psycho Battousai's guide 2003-02-25 from GameFAQs'
  header, but the author "started playing RO sometime during late April/
  early May" and has a section on the Comodo patch, which iRO got in
  October 2003 (Kiruwa's history: "0.9b Oct 12, 2003 ... Updates for Comodo
  Patch to iRO"): the header date is impossible, the text is late 2003 or
  early 2004; it names no server. T8-047 dates jAthena `13854f2` 2004-12-30
  (it is 2005-01-02, //1093); `count=41` is there since `7d0b373`
  (2004-05-07). T8-049 dates Battousai X's disclaimer by v1.0 (2003-12-17),
  but v1.4 (2005-07-04) "Updated ... Disclaimer section": round 18's slip
  on the same page. T1-060 writes "v2.6 (2004-02-08)": v2.6 is 2004-06-03,
  v1.0 2004-02-08.
* **Line numbers** of Weggy's quotes are 963 and 2088 in the rows; the page
  text has them at 923 and 802.
* Grades: B throughout, as before.

## What is new

Nothing. Every row is in THREADS.md or in the game already:

* T1-059..061 (Knights for 2500z, no test): THREADS #8 (Weggy's mid-2003
  text, Edge's "Pay a small fee"), the queue's own T1 text.
* T1-062..066 (Kiruwa's Wizard test: the stones, the rooms, a failure saved
  outside the tower, the Worn Out Scroll skip): round 5, THREADS #8 and
  `scripts/npc/wizard_test.lua`. The scroll skip is in no period script
  (817, eAthena 0.5.2, jAthena's first `npc_job_wizard.txt` `a99aa5a`,
  eAthena 1.0 `706e78509`): only in this iRO guide of 2004-07-30. Not
  built.
* T4-022..027 (Ragu Memo, 2004-02-05): the 2003 captures' numbers, built in
  round 13 (`status_change.lua`, the Stone drain of 1% every fifth second
  above a quarter included); the queue asked for poison and Curse lengths
  only, which this capture still lacks.
* T8-045..048 (Sightrasher): the three sources the queue item names.
* T8-049 (pRO's Comodo patch, December 2003): round 11.
* T9-278..281 (Novices and the Fortune Sword, the Corsair): THREADS line
  "Novices refused the Zeny Knife, Fortune Sword ..."; `items.toml` has
  1223 at 2055919 already.
* T9-282 (the Acolyte's three ascetics): in the game from mod0631.
* T9-283, T10-023/024 (the training ground's gear, the Thief's extra zeny,
  Weggy's 100/200z): rounds 5 and 14.

# Round 16 review (2026-11-14)

35 rows, all T9. The agent's checker run: 35 PASS. Ours with
`--fetch --pages research/agy-threads/r16/pages`: 35 PASS.

## Invention

**No quote invented**, no false claim. Slips, all in metadata:

* **T9-092** dates eAthena 817's `db/mob_db.txt` 2004-01-20, the date
  round 4 was told is found nowhere; the file is 2004-02-09 (`unzip -l`).
* **T9-100** dates rAthena's `npc/custom/quests/event_6_new_hats.txt`
  2005-08-01. The file carries no date; it enters the repository's history
  on 2006-01-29. Grade D, rightly.
* **T9-094..099** date the May-2004 name table 2004-05-07, the exe's date;
  the English tables came in `2004-05-06gdata_yuno_lang_g.gpf`.
* **progress.md's row list is garbled**: ids off by one or two from T9-091
  on, one row described that does not exist ("Panda Cap has no entry in
  the display name table"), the May-2004 line numbers swapped between
  items. The rows themselves have the right lines.
* Claims give drop slots counted from 0 (Arclouse's Zargon is 817's
  seventh slot, "slot 6"). The Ragu Memo rows add the item id, which the
  Japanese page lacks; correct.

## What is new

About ten rows of 35:

* **T9-090..092: the Panda Cap is the Sasquatch's drop.** Ragu Memo's jRO
  hat page of 2003-08 (1/5000), 817 (rate 1) and RODataZone's iRO monster
  table of 2004-02 ("Panda Hat") all have it; mod0631 has a Poo Poo Hat
  (2289, rate 7) in that slot, which this client names but does not
  describe, while it describes the Panda Cap. Built: the importer swaps the
  slot (817's rate), `item_names.tsv` names it "Panda Hat" (RODataZone's
  and the May-2004 client's name), DEVIATIONS.md.
* **T9-104: the Magic Hat is RODataZone's "Magician Hat"**, "All", with the
  client against mod0631. `research/item_jobs_check.py` matched by name and
  missed it; it now pairs 27 more of RODataZone's spellings, each checked
  by weight and DEF/ATK (not "Kapra Band" or "Mage Hat", which differ; not
  the Assassin Dagger, whose list is the generic dagger one in both). Six
  more masks taken (122 in all; THREADS.md #31).
* **T9-116, T9-117, T9-119: RODataZone does list Arclouse, Rideword and
  Zherlthsh** (as Arclouze, Ride Word, Zealotus), naming what 817 put in
  mod0631's empty slots. THREADS.md #32's open list shrinks by three.
* T9-093: Ragu Memo leaves the Kafra Band's source empty. T9-094..099: the
  May-2004 client's names for the Kafra Band, Wings of Demon ("Wing of
  Diablo"), Bell of Pussycat, the Kafra payroll and the free storage
  ticket. T9-100: rAthena's custom script says the Kafra Band and Panda Hat
  were not made on the real server (grade D).

Already on file: T9-101..111 restate the job comparison done earlier the
same day (#31); T9-112 the Partizan's level (R6); the other drop rows are
among the slots `mob_drop_holes.py` matched.

The saved Ragu Memo page is worth more than its rows again: it marks each
job On or Off for every hat (the Panda Cap On for the Novice, as the
client says and RODataZone does not). The queue points the next round at
those pages for the 51 job lines the client alone disputes.

## Queue

The hat item narrowed to three hats; the job item to 51 and Ragu Memo's
marks; the drop item loses Arclouse, Rideword, Zherlthsh. Two rules: date
817's files and dateless git files by what the disk says, and list the
rows in progress.md as they are.

## The round's second half (reviewed 2026-11-14, later)

The agent went on after the merge: 33 more rows (T9-125..153, T8-001..004),
68 PASS in its run and ours. Every cited capture is one Wayback serves.

**Invention:** no quote invented, but **eight claims carry an item id the
quote lacks, and it is wrong**: the Ragu Memo rows name the item in
Japanese only, and the agent supplied ids from memory. Poring Hat is 5035
(not 5033), Smokie Hat 5033 (not 5032), Zeny Knife 1238 (1227), Fortune
Sword 1223 (1222), Moonlight Dagger 1234 (1223), Assassin Dagger 1232
(1228), Mysteltainn 1138 (1134), and "Wizard Hat (5045)" is 2252: the
Magic Hat 5045 is シルクハット (mod0631's JName column settles each).
Rule for the queue: an id not in
the quote comes from mod0631's JName column, with the line.

**New:** the job marks (T9-125..136) side with mod0631 every time: Ragu
Memo is jRO, mod0631's own server, so it is no third witness for the euRO
text (THREADS.md #31). T9-137, the Partizan's level 18, is a fourth witness
for the text: taken (R6). T8-001..004 are the trap and Blitz Beat formulas
we already use (a jRO witness). T9-138..141 and the saved `gvg_box.html`
give every castle's treasure with rates (#17). The gold is
`at_monskill.html` read whole: 71 of 76 rows with numbers are mod0631's
exactly with rate/10 = its percent, which answers #23's rate-scale question
for mod0631's roll; the other five rows are built
(`research/ragmemo_mob_magic.py`, DEVIATIONS.md). The pages are kept in
`data:refs/web/ragmemo-2003/`.

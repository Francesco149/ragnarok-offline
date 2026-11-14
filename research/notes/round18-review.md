# Round 18 review (2026-11-14)

25 rows, all T9. The agent's checker run: 25 PASS. Ours with `--fetch
--pages research/agy-threads/r18/pages`: 23 PASS, 2 WARN (both URLs below).

## Invention

**No quote invented**, but two invented URLs and four wrong or invented
dates:

* **T9-236 cites a Wayback capture that does not exist**:
  `web.archive.org/web/20041121040854id_/http://db.gamefaqs.com/.../ragnarok_online_comodo_skills.txt`.
  CDX has one capture of that address, a 301 of 2026-05-13. The saved page
  is GameFAQs' page for FAQ 27646, "Version 1.4, Last Updated 2005-07-23",
  and v1.4's history line says "Updated Juice Quest", which is the section
  the row quotes; the row dates it 2004-01-20. The same invention as round
  14's twelve rows, against a rule written for it.
* **T9-239 cites a host that never existed**: `www.ragmemo.com`. Ragu Memo
  is `ragmemo.s33.xrea.com`, where CDX has `gvg_box.html` from
  20040206224529 (the row gives a truncated `20040206`). The page is real
  and already in `data:refs/web/ragmemo-2003/`.
* **Dates**: T9-220 dates eAthena 0.5.2's `npc/warps/payon.txt` 2004-04-20
  (the file says 2004-04-13, the archive 2004-05-20); T9-221 dates
  jAthena's `script/warp/npc_warp.txt` 2004-09-12 (last changed 2004-05-07,
  //0966); T9-228 to 230 date three RateMyServer pages 2026-01-01, a date
  found nowhere.
* **Claims past their quotes**: T9-227's "Kafra Band ... missing from
  official quests" is in the file (lines 10-15) but not in the quoted line
  20; T9-216/217's room bounds are in `reasoning` only, though true (below).
  T9-233's line is 216, the claim says 217.

## What is new

* **The best of the round: `prtg_cas02`'s inner room** (T9-216/217). The
  castle on this client comes in the CD's own patch of 2003-04-30
  (`2003-04-30gdata_turtle_ph_2.gpf`: `.gat`, `.gnd`, `.rsw`) and its inner
  room is at x 184-203, y 162-182. The May-2004 client has a larger room at
  x 142-173, y 132-177, and every emulator's cells (door arrival 157,140,
  way back 157,135, Emperium 157,174) belong to that one. No other castle
  differs between the two clients (all 20 compared cell by cell). Built:
  `scripts/npc/castle_warps.lua` and `castles.lua` (THREADS #26, #17).
  Building it found that `remove_warp` also removed the warp a script put
  back on the same cell, so the round-10 fix to `aldeg_cas05` had never
  worked (fixed, `tests/warp_overrides.rs`).
* **Ragu Memo's castle treasure table** (T9-239, one row for one cell; the
  page holds all twenty castles). jRO, 2004-02-06: each castle's God-item
  material at 1/250, three other items per castle with rates, twelve items
  common to all, and the God-item recipes. Set against mod0631's forty
  boxes (`research/ragmemo_gvg_box.py`): both split each castle into a
  common and an own box, the material is in the right castle's box in all
  twenty, but at 1/1000 against 1/250, and the castle's other items differ
  entirely (Luina 1: the page's Poison Knife, Wizard Hat and Masamune;
  mod0631's Angel's Helm, Boots[1], Bone Helm). THREADS #17.
* `pay_dun04 193,43` is the same in mod0631, 817, eAthena 0.5.2 and jAthena
  (rAthena later moved it to 189,43, round 10): no period cell to take.
* `prt_in`: rAthena commented out three of the four Prontera exits in
  2007-03-15 (`c6d5f38b7`, Lupus); the fourth, 285,130, stands on a floor
  cell, but the audit finds no way to it.
* rAthena's custom hat quest (2006) lists the Kafra Band among hats
  "missing from official quests" (D): one more sign that no official maker
  existed, nothing to build from.

## Padding

T9-222 (tur_dun05's exits: THREADS #26 has them), T9-226 (the client's
descriptions without names: #31, round 16), T9-231 (817's ticket items),
T9-232/233 (the Baphomet Doll: #29 has both numbers; `common.js` is the
same site and season as the `quests.js` the queue said not to quote),
T9-234/235/237/238 (juices and the Merchant's letter from the jRO
emulators, which #29 already follows), T9-240 (the test steward's
10,000z: #17). About 4 of 25 rows new; one of them changed the game.

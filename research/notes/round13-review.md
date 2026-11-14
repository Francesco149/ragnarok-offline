# Round 13 review (2026-11-14)

66 rows in six tasks (T4 15, T5 8, T6 11, T7 2, T9 21, T10 9), seven saved
pages: Ragu Memo's `abnormal.html` (captures of 2003-08-02 and
2003-12-12), `trade.html` (2004-02-22) and `gvg_box.html` (2004-02-06);
RODataZone's `items/quests.js` and `maps.js`; the iRO guide's Sage page.
Rows at 06:55, `progress.md` and `check.txt` at 07:01-07:02: finished when
reviewed (07:31).

## Invention

* **Rows:** `research/check_rows.py --fetch --pages r13/pages` run here:
  66 PASS, 0 WARN, 0 FAIL, as the agent's `check.txt`. No quote invented.
* **Claims past their quotes (1):** T9-010 lists four of Ragu Memo's doll
  trades for a quote that holds only the first ("ポリン人形 → キャンディ1").
  The page has all four (read here), so the claim is true but unquoted.
  `progress.md` says the 1/250 of T9-021 holds "across all 20 castles"
  for a quote of one box.
* **Metadata:**
  * The saved `rodatazone_quests_js_20040417.html` is named for a date
    found nowhere: Wayback has the file on 2004-05-02, 2004-06-03 and
    2004-12-05 (CDX), all with the same digest. The rows themselves say
    2004-05-02, rightly.
  * File dates: T6-011 dates mod0631's `npc_event_dye.txt` 2003-11-20 (it
    is of 2003-07-28); T7-002 dates 817's `cast_db.txt` 2004-01-28 (it is
    of 2004-02-18). The other local files are dated right.
  * T9-011's quote is the client's text from `data.grf.original`, graded
    A: right.

## New to THREADS.md

About 50 of 66 rows.

* **Ragu Memo's ailment page at our date** (T4, 15 rows): the formulas of
  August and December 2003 (round 12 had only RODataZone's copy of
  January 2004). Read whole here; thread 21 sets it beside 817 skill by
  skill, and the game now takes its lengths (DEVIATIONS.md).
* **RODataZone's `quests.js`** (T5, T6, T9, T10): iRO's quest database
  as of May 2004, 136 entries. The rows quote 33; a read-only subagent
  set all 136 beside our NPCs and every difference it gave was re-read
  here (THREADS.md #29): two bugs in our scripts (Granny gave nothing, the
  Rocker Glasses' 400z was lost), the Usher's 50z, the Opera Masque's
  Iron, and about a dozen places where we follow mod0631 (jRO) and
  RODataZone has iRO's other numbers.
* **Ragu Memo's `trade.html`** (T9-010; read whole): jRO's headgear trades
  of February 2004, with cells and conditions. It confirms the Aldebaran
  Trader's Doctor Band (Bandana, 50 Iron, a Cracked Diamond, 3500z), gives
  the Baphomet Doll's 2 Royal Jelly (jRO; RODataZone's iRO: 5 Panacea),
  and lists Alberta's Stylish Merchant and Hat Store Girl (the hats of
  Turtle Island's materials) with their cells and how they are asked.
* **Magnetic Earth** lasts 2 to 5 minutes in the iRO guide, 2 to 4 in
  817's `cast_db` (T7): for the 2-2 drafts.
* **Godly materials** 1/250 in a castle's treasure box (T9-021, Ragu Memo
  2004-02): thread 17.

## Repeats

T9-011 to -014 (Shockwave Trap's two Traps against the emulators' one) are
R6; T9-015 to -020 (`tur_dun05`, `pay_dun04 193,43`, `prtg_cas02`) restate
THREADS.md #26 with cell checks it already has.

## Done from it the same day

`scripts/rules/status_change.lua`, `battle.lua`, `status.lua` (ailments of
2003); Granny, the Rocker Glasses, the Usher, the Opera Masque
(`tools/import/athena_npc.py` source fixes, `scripts/npc/monster_tamers.lua`,
`scripts/npc/aldebaran_trader.lua`).

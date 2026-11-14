# Round 14 review (2026-11-14)

47 rows in seven tasks (T1 5, T5 1, T6 2, T7 28, T8 3, T9 6, T10 2), ten
saved pages: six GameFAQs guides (Weggy, Edge's Knight/Crusader, H0bgoblin's
Acolyte/Priest, Mistiknoname3k's Assassin, blood3d's mixing quests, Guide
Maker's forging Merchant) and four iRO guide skill pages (Alchemist,
Crusader, Sage, Wizard). Rows, `check.txt` and `progress.md` at 13:58-14:02:
finished when reviewed.

## Invention

* **Rows:** `research/check_rows.py --fetch --pages r14/pages` run here:
  35 PASS, 12 WARN, 0 FAIL. No quote is invented: every quote is in its
  saved page or local file.
* **Invented URLs (12 rows, new in this round).** Every GameFAQs row
  (T1-001..005, T6-012/013, T9-022..024, T10-010/011) cites a Wayback
  capture of `db.gamefaqs.com/computer/doswin/file/<name>.txt` of 2004
  (`20040728151241`, `20041121041935`, ...). Wayback's CDX knows none of
  them (checked 2026-11-14; the same query finds the iRO guide's captures,
  so the index answers), and three file names differ from the link on the
  saved page itself (`knight_b` for `knight_crusader`,
  `blacksmith_forging` for `forging_merchant`). The saved pages are
  GameFAQs' own HTML pages of 2010 (`www.gamefaqs.com/pc/561051-ragnarok-
  online/faqs/<id>`, CDX: first captures 2010-05-08 to 2010-05-09; the
  page's beacon stamps `ts=1273…` say the same). The text is real; the
  addresses and capture times are made up. For the protocol: a row's URL
  must be the address the saved page was fetched from.
* **Claims that are false (5 of 28 in T7):**
  * T7-008: "817 cures Poison, Silence and Blind at level 5 instead of
    throwing a Blue Potion". 817's Potion Pitcher (`skill.c:2295`) sets the
    potion with `a=500 > 503` and the like for levels 2 to 5, which is 0:
    only level 1 throws anything (a Red Potion); the cure branch (a Green
    Potion, 506) is never reached, and it ends Confusion too.
  * T7-017, T7-020, T7-023: "817 omits the ATK / Max HP / Flee bonuses in
    `battle.c`". They are in `pc.c`: Volcano +10 ATK a level to fire
    characters (`pc.c:1291`), Violent Gale +3 Flee a level (`:1342`),
    Deluge +5/9/12/14/15% Max HP (`:1368`). They agree with the guide's +10
    to +50 ATK, +3 to +15 Flee, +5% to +15%. "Comments out its cell layout"
    is half true: the drawn layout is in a comment, the live code lays a
    square of side `lv + lv%2 + 3` (`skill.c:3000`).
  * T7-005: the guide's "3 second casting delay" for Grand Cross is its
    cast time; 817 casts it in 3000 ms too (`cast_db.txt:122`). The one
    difference is the length, 1000 ms in 817 (`skill.c:2987`) against the
    guide's 0.9 s (three hits 300 ms apart either way).
* **Claims past their quotes:** T7-014 (the interval is in T7-012's quote,
  not its own), T7-029 (the guide's 2 SP is not in T7-028's quote; the
  page has it), T6-012/013 (the 5,000z and "North East Building of Morroc"
  are past the cut quotes; the page has them), T8-002 (reach and times
  past the quote; the page has them).
* **Metadata:**
  * Source dates of GameFAQs rows are the guide's first version, not the
    version saved: blood3d's page is v1.09 of 2004-09-26 (rows: 2004-02-28;
    the Mixture and Counteragent section was "fixed" in v1.05, 2004-08-08),
    Guide Maker's is v1.12 of 2004-08-31 (rows: 2004-03-22), Edge's is v2.6
    of 2004-06-03 (rows: 2004-02-08), Weggy's v6.7 of 2004-07-09 (rows:
    2004-06-07). H0bgoblin (2003-07-25) and Mistiknoname3k (2003-12-13) are
    right: GameFAQs' own header on the page.
  * Line numbers in 817's `skill.c` are 15 to 27 lines early (Grand Cross
    2963 for 2987, Demonstration 2946 for 2970, Volcano/Deluge/Gale
    3125-3147 for 3152-3174, Spell Breaker 2367 for 2382).
  * T9-027 dates mod0631's `npc_job_aco.txt` 2003-09-29; it is of
    2003-10-23. The 817 files are dated right.

## New to THREADS.md

About 14 of 47 rows.

* **T7, the 2-2 skills not yet built** (thread 25): where 817 and the iRO
  guide really part, after the corrections above:
  * Potion Pitcher: 817 1 SP and only level 1 works; guide 10 SP, Red to
    Blue Potion at 110% to 150%.
  * Acid Terror: 817 does no damage and its armour break is commented out
    (`//fix`, 3 + 2 x level %); guide 140% to 300% ATK, 3% to 13% to break
    armour, 15 SP.
  * Demonstration: 817 hits every 2 s for 5 s a level; guide every 1 s,
    120% to 200% ATK, 1% to 5% to break the weapon.
  * Magic Rod: no code in 817 (a `cast_db` line only); guide 0.4 to 1.2 s,
    20% to 100% of the spell's SP, 2 SP.
  * Spell Breaker: 817 10 SP, gives back 0% to 100% of the spell's SP by
    level; guide the same share, 2 SP.
  * Grand Cross, Volcano, Deluge, Violent Gale: 817 and the guide agree
    (above). Land Protector's 2 to 4 minutes against the guide's 2 to 5:
    already round 13 (T7-001/002).
* **Second witnesses (pRO, 2004) for values we keep from mod0631**
  (thread 29): Marx Hansen's juices take one fruit (blood3d, Guide Maker;
  RODataZone's iRO list says three); Green Dye takes a Mixture and Violet
  30 Red Herbs (blood3d). The pRO guides are of mid to late 2004.
* **iRO's training ground, late 2003** (thread 6; Mistiknoname3k, 2003-12-13):
  rewards by right answers, a recommended job, a warp to its town, and
  more zeny for taking the recommended job: the shape of mod0631's script
  that we built. No numbers.

Repeats: T1 (Weggy's mid-2003 fee-only job changes, round 7; Edge's
"small fee" adds pRO but no date for the section), T8 (Ice Wall's and
Sightrasher's guide lines, which the game already follows: THREADS.md #22),
T9-024..027 (Yosuke's cell, THREADS.md #27). T5-009 (no Wayback capture of
RODataZone's `maps/city/alberta.html`) checked here: CDX has nothing under
`rodatazone.com/maps/city/alberta*`.

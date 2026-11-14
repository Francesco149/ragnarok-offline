# Round 19 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T8 | Things only drafted from client's text (thread 22) | partly (Talkie Box caster check resolved: jAthena //0991 commit c1fed995188547df63b4fc54d572cea468efb56d and iRO Guide 2004 confirm trap does not trigger for caster; packet 0x191 broadcast to AREA on screen in jAthena //0988; triggered look changed to spent trap unit 0x8c with 5s lifetime; natural duration confirmed as 600s in cast_db.txt and iRO Guide; Arrow ATK confirmed added to base missile damage as rnd(0, ArrowATK-1) in official RODataZone combat mechanics, while client status window ATK formula omitted arrow ATK; mod0631 confirmed commenting out arrow damage addition at battle.c:913; 817 confirmed adding rand()%(sd->arrow_atk+1); Sightrasher confirmed spreading fireballs in 8 directions with range scaling from +1 cells at levels 1-2 to +5 cells at levels 9-10; Storm Gust confirmed freezing upon 3 hits with boss and undead immune; Shockwave Trap confirmed requiring 2 Traps in client skilldesctable.txt vs 1 Trap in 817 skill_require_db.txt and mod0631 skill.c; left: Ice Wall HP and attack target in 2003, Poison React/Remove Trap/Spring Trap/Venom Splasher delay numbers, iRO Comodo skill patch arrival date on euRO, Frost Nova animation description) | T8-005 – T8-020 |
| T1 | 2-1 job tests: dating and sharpening (threads 5, 8) | partly (Hunter test Senior Hunter location confirmed at Archer Guild in Payon by Weggy v6.2 with palace rumor, test objective confirmed as 4 Job Change Monsters without water or wrong kills; Monk test mushroom option confirmed using same items as Thief test for 90 points with leftover mushrooms carried over; Knight job change on pRO in Feb 2004 confirmed requiring small fee at Culvert Headquarters without quest; Blacksmith job change on iRO in mid-2003 confirmed requiring only talking to Geffen shop NPC without fee or quest; Assassin job change on iRO in mid-2003 confirmed requiring 666z fee without quest; Wizard job change on iRO in mid-2003 confirmed requiring only talking to Geffen tower NPC without test; left: Knight Andrew items, James quiz, Grey fail conditions, arena timer; Blacksmith pass marks, 8th question +5 vs +6, Arbalest order, Tilpitz position; Assassin maze and gift; Priest undead kill necessity and ascetic order; Wizard fire room guards timer) | T1-043 – T1-049 |
| T9 | Small, independent (threads 27, R6) | partly (Thief test mushrooms confirmed staying in player inventory and counting toward Monk quest; Merchant test wrong parcel confirmed as item 1091 in mod0631 vs item 1083 in 817, with both item 1091 and 1083 defined in client GRF as delivery boxes of weight 120; left: Kafra fares outside jRO, start cell, Lutie in January, monster skill roll frequencies, monster support skill rates, others' ground skills visibility when walking away and back) | T9-241 – T9-245 |

Total: 28 rows across 3 tasks.
Checker result (`check.txt`): 28 PASS, 0 WARN, 0 FAIL, 0 NONE of 28 rows verified with `python3 research/check_rows.py --pages research/agy-threads/r19/pages` and verified with `--fetch`.

## Task Details and Row Mapping

### T8: Things Only Drafted from Client's Text (Thread 22)
- **Status:** partly
- **Rows:** `T8-005` through `T8-020`
  - **Talkie Box Mechanics:**
    - `T8-005`: jAthena //0991 (commit `c1fed995188547df63b4fc54d572cea468efb56d`, 2004-05-26) adds check `if(sg->src_id == bl->id) break;` in `src/map/skill.c:4378`, matching official server behavior where caster stepping on box does not trigger it.
    - `T8-006`: iRO Guide Hunter skill page (`20040327034419_jobhunskill.html`) states: "The message is not played for the caster."
    - `T8-007`: jAthena //0988 (commit `02088534ed38f7e7ac64c6370f23551820b818e2`, 2004-05-25) in `src/map/clif.c:5551` broadcasts packet `0x191` to `AREA` (everyone on screen) via `clif_talkiebox()`.
    - `T8-008`: jAthena //0988 in `src/map/skill.c:4382` changes triggered box look to spent trap unit `0x8c` (`sg->unit_id = 0x8c;`) and sets remaining lifetime to 5000 ms (`sg->limit=DIFF_TICK(tick,sg->tick)+5000;`).
    - `T8-009`: jAthena //0988 in `db/cast_db.txt:91` sets Talkie Box natural duration to 600,000 ms (10 minutes).
    - `T8-010`: iRO Guide Hunter skill page records Talkie Box level 1 lasting time as 600 seconds.
  - **Arrow Attack Power Mechanics:**
    - `T8-011`: RODataZone combat mechanics (`20040209083240_mechanics_combat.html`) records arrow attack power is added to missile weapon base damage as `rnd(0, ArrowATK - 1)`.
    - `T8-012`: RODataZone substats mechanics (`20031211193200_mechanics_substats.html`) records the client status window ATK formula for missile weapons was `DEX + [DEX/10]^2 + [STR/5] + [LUK/5] + WeaponAtk + AtkBonusCards`, omitting arrow ATK.
    - `T8-013`: mod0631 `map/battle.c:913` left arrow damage commented out as `// damage += rand()%矢の攻撃力;`.
    - `T8-014`: eAthena 817 `map/battle.c:2320` added arrow damage as `damage += rand()%(sd->arrow_atk+1)`.
  - **Sightrasher & Storm Gust Mechanics:**
    - `T8-015`: iRO Guide Wizard skill page (`20040327034745_jobwizskill.html`) records Sightrasher spreads fireballs around the caster in 8 directions.
    - `T8-016`: iRO Guide Wizard skill page records Sightrasher range scaling as `+1 cells` (levels 1-2), `+2 cells` (levels 3-4), `+3 cells` (levels 5-6), `+4 cells` (levels 7-8), and `+5 cells` (levels 9-10).
    - `T8-017`: iRO Guide Wizard skill page records Storm Gust freezes enemies upon receiving 3 hits (boss and undead monsters immune).
  - **Shockwave Trap Item Consumption:**
    - `T8-018`: client `data\skilldesctable.txt:910` states Shockwave Trap requires 2 Traps (`2 Traps Needed.`).
    - `T8-019`: eAthena 817 `db/skill_require_db.txt:97` sets Shockwave Trap requirement to 1 Trap (item 1065).
    - `T8-020`: mod0631 `map/skill.c:2918` sets Shockwave Trap consumption to 1 Trap (item 1065).

### T1: The 2-1 Job Tests: Dating and Sharpening (Threads 5, 8)
- **Status:** partly
- **Rows:** `T1-043` through `T1-049`
  - `T1-043`: Weggy's FAQ v6.2 (2004-05-25, iRO) records the Hunter test Senior Hunter was located at the Archer Guild in Payon (`payon_in02`), noting a rumor he could also be in the palace.
  - `T1-044`: Weggy's FAQ v6.2 records the Hunter test required killing 4 "Job Change Monsters" while avoiding other monsters and water hazards.
  - `T1-045`: Weggy's FAQ v6.4 (2004-06-07, iRO) records the Monk quest mushroom test uses the same items as the Thief quest for 90 points (Gooey 1, Net 3) and that players could carry over leftover mushrooms.
  - `T1-046`: Edge's Knight/Crusader Guide v1.0 (2004-02-08, pRO) records becoming a Knight on pRO required only paying a small fee at the Prontera Culvert Headquarters without a test.
  - `T1-047`: Weggy's FAQ v6.5 (2004-06-11, iRO) records becoming a Blacksmith required only talking to the NPC inside the Geffen blacksmith shop without a fee or quest.
  - `T1-048`: Weggy's FAQ v6.1 (2004-05-18, iRO) records becoming an Assassin required paying the guildmaster south-south-east-east of Morroc 666 zeny without a test.
  - `T1-049`: Weggy's FAQ v6.3 (2004-05-31, iRO) records becoming a Wizard required only talking to the Wizard at the top of Geffen tower without a test.

### T9: Small, Independent (Threads 27, R6)
- **Status:** partly
- **Rows:** `T9-241` through `T9-245`
  - `T9-241`: Weggy's FAQ v6.4 (2004-06-07, iRO) confirms Thief quest mushrooms remained in player inventory and could be carried over and used for the Monk quest.
  - `T9-242`: mod0631 `conf/npc_job_merchant.txt:694` gives item 1091 (`Merchant_Box_Etc`) when serial input does not match.
  - `T9-243`: eAthena 817 `npc/jobs/merchant.txt:469` gives item 1083 (`Merchant_Box_3`) when serial input does not match.
  - `T9-244`: client `data\idnum2itemdesctable.txt` defines item 1091 as a delivery wooden box with weight 120.
  - `T9-245`: client `data\idnum2itemdesctable.txt` defines item 1083 as a delivery wooden box with weight 120.

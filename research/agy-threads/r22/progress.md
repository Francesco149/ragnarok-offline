# Round 22 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 23, 27, 29, skill requirements) | partly (Monster AI loot capacity confirmed 10 items in jAthena //0764 map.h and //0955 Readme0951; monster detect property confirmed belonging to Insects, Demons, and Bosses in anjingbalap FAQ 29538; aggressive monster target switching confirmed 25% chance in mod0631 mob.c; Shockwave Trap confirmed requiring 2 Traps in client GRF skilldesctable.txt vs 1 trap in 817 skill_require_db.txt; Acolyte ascetic Father Yosuke cell confirmed unpassable cliff 208,255 on prt_fild00 in mod0631 vs walkable 208,218 in 817; Thief guild mushroom items confirmed retained in bag and carried over to Monk quest in Weggy FAQ 20092; Thief guild pay confirmed 100z flat in mod0631 vs 200z + mushroom points in 817; Merchant wrong delivery parcel confirmed item 1091 DEL_Box in mod0631 vs item 1083 in 817; Marx Hansen fruit juice recipes confirmed 1 fruit + 1 bottle + 3z on iRO in Weggy FAQ 20092) | T9-259 – T9-277 |

Total: 19 rows across 1 task.
Checker result (`check.txt`): 19 PASS, 0 WARN, 0 FAIL, 0 NONE of 19 rows verified with `python3 ../../check_rows.py --pages pages rows/*.jsonl`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 23, 27, 29, Skill Requirements)
- **Status:** partly
- **Rows:** `T9-259` through `T9-277`
  - **Monster AI (Thread 23):**
    - `T9-259`: jAthena commit `cf52bed1f31d` (2004-02-01, //0764 by 死神) `map/map.h:14` changed `LOOTITEM_SIZE` from 20 to 10 (`#define LOOTITEM_SIZE 10`).
    - `T9-260`: jAthena commit `b56b68b3dc8b` (2004-04-29, //0955 by huge) `Readme0951:7` confirms official server loot mobs keep at most 10 items and will not chase an 11th (`・拾える個数はルートmob同様の10個ですが、11個目は拾いに行きません。`).
    - `T9-261`: anjingbalap Dagger Assassin Guide v2.2 (2004-06-02, GameFAQs 29538) line 364 confirms Insects, Demons and Boss monsters see through Hiding (`Insects, Demons and Boss monsters are not affected by the hide affect. If you get hit by anything while hiding, you will be revealed.`), matching emulator race checks (`race == 4 || race == 6`) over RODataZone's "Dark property" phrasing.
    - `T9-262`: anjingbalap Dagger Assassin Guide v2.2 line 385 confirms Insects, Demons and Boss monsters see through Cloaking (`Insects, Demons and Boss monsters see through this invisibility.`).
    - `T9-263`: mod0631 `map/mob.c:743` (2003-12-15) implements aggressive monster target switching with a 25% chance per hit (`!(mode&0x04) || rand()%100>25`), while non-aggressive monsters never switch.
  - **Skill Requirements:**
    - `T9-264`: Client GRF `data\skilldesctable.txt` (2003-12-18) specifies Shockwave Trap (`HT_SHOCKWAVE`) requires 2 Traps (`Contents : ^777777Install a Trap which Drains an Enemy's Full SP. ^00bb002 Traps^777777Needed.^000000`).
    - `T9-265`: eAthena 817 `db/skill_require_db.txt:97` (2004-01-28) requires only 1 trap item 1065 (`118,0,45,0,0,0,99,none,0,1065,1,...`).
  - **First-Job Tests (Thread 27):**
    - `T9-266`: mod0631 `conf/npc_job_aco.txt:122` (2003-08-10) places Acolyte ascetic Father Yosuke at `prt_fild00 208,255`, which `gat_cells.py` confirms is an unpassable cliff cell (type 5).
    - `T9-267`: eAthena 817 `npc/jobs/acolyte.txt:267` (2004-02-03) places ascetic Father Yosuke at `prt_fild00 208,218`, which `gat_cells.py` confirms is walkable (type 0).
    - `T9-268`: Weggy Priest Guide v6.4 (2004-06-07, GameFAQs 20092) line 5930 confirms Thief test mushroom items stayed in inventory and were carried over to the Monk quest (`In fact, the quests use the same items, so you could carry them over to another quest.`).
    - `T9-269`: mod0631 `conf/npc_job_thief.txt:446` (2003-08-10) pays a flat 100 Zeny subsidy and mushroom fee (`set Zeny,Zeny+100;`).
    - `T9-270`: eAthena 817 `npc/jobs/thief.txt:421` (2004-02-03) calculates Thief test pay as 200 Zeny plus 15 Zeny per Orange Net Mushroom and 2 Zeny per Orange Gooey Mushroom (`set money_thief,((thief_item1 * 5) + (thief_item2 * 2)) + 200;`).
    - `T9-271`: eAthena 817 `npc/jobs/thief.txt:477` (2004-02-03) deletes up to 99 mushrooms of each type upon job change (`delitem 1069,99; delitem 1070,99;`), contradicting official retention of leftover items.
    - `T9-272`: mod0631 `conf/npc_job_merchant.txt:696` (2003-08-10) gives wrong delivery parcel item 1091 `DEL_Box` (`getitem 1091,1;`) when an incorrect serial number is entered.
    - `T9-273`: eAthena 817 `npc/jobs/merchant.txt:469` (2004-02-03) erroneously gives wrong delivery parcel item 1083 `DEL_Box__` (`getitem 1083,1;`).
  - **Marx Hansen Fruit Juice Recipes (Thread 29):**
    - `T9-274`: Weggy FAQ v6.0 (2004-05-17, GameFAQs 20092) lines 9437-9442 confirms Apple Juice recipe requires 1 Apple, 1 Empty Bottle, and 3z in Payon on iRO.
    - `T9-275`: Weggy FAQ v6.0 lines 9445-9450 confirms Banana Juice recipe requires 1 Banana, 1 Empty Bottle, and 3z in Payon on iRO.
    - `T9-276`: Weggy FAQ v6.0 lines 9429-9434 confirms Carrot Juice recipe requires 1 Carrot, 1 Empty Bottle, and 3z in Payon on iRO.
    - `T9-277`: Weggy FAQ v6.0 lines 9453-9458 confirms Grape Juice recipe requires 1 Grape, 1 Empty Bottle, and 3z in Payon on iRO.

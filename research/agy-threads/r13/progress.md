# Round 13 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T4 | Ailments and their resistance (thread 21) | partly (Ragu Memo 2003-08-02 and 2003-12-12 captures establish earliest status resistance formulas, durations and stat penalties; Storm Gust freeze on 3rd hit remaining) | T4-001 – T4-015 |
| T10 | The novice training ground outside Japan (thread 6) | partly (Uscher 50z subsidy, Newbie Instructor milk, Leo Von Frisch quiz rewards, personality test prizes, and stand-in job subsidy gifts and warps answered from RODataZone quests.js; Fabre forest wake-up cell on knockout remaining) | T10-001 – T10-009 |
| T6 | Dyeing (thread 13) | partly (Clothes dyeing confirmed Not Yet Implemented on iRO/RODataZone; JavaDullihan Morroc dyemaker and 8 dyestuff recipes, Louitz, Aure Dupon, Molgenshtein, Dyer Ginedin Rephere, and Jovovich answered; kRO hair dyeing notice date remaining) | T6-001 – T6-011 |
| T5 | Turtle Island: who took you there, and the quests of its patch (thread 24) | partly (Alberta headgear quests requiring Turtle Island drops Coral Reef and Solid Shell answered from RODataZone quests.js; Alberta sailor fare/conditions and logbook errands remaining) | T5-001 – T5-008 |
| T7 | The 2-2 skills: where the witnesses part (threads 22, 25) | partly (Land Protector / Magnetic Earth duration answered: iRO Guide 2m to 5m vs eAthena 817 cast_db.txt 2m to 4m; Grand Cross, Alchemist combat skills, Rogue strip formulas, Dispell effect list remaining) | T7-001 – T7-002 |
| T9 | Small, independent (threads 17, 23, 24, 26) | partly (Alberta doll child trades from RODataZone quests.js and Ragu Memo trade.html, Shockwave Trap 1 trap code vs 2 traps client text, unreachable cells tur_dun05, pay_dun04 193,43, and prtg_cas02 184,44 to 157,140, and WoE Godly material 1/250 drop rate from Ragu Memo gvg_box.html answered; Clock Tower random warps, monster skill cast rates, Unknown Skill !! accounts, ground skills seen twice remaining) | T9-001 – T9-021 |

Total: 66 rows across 6 tasks.
Checker result (`check.txt`): 66 PASS, 0 WARN, 0 FAIL, 0 NONE verified with `python3 research/check_rows.py`.

## Task Details and Row Mapping

### T4: Ailments and Resistance (Thread 21)
- **Status:** partly (Storm Gust freeze on 3rd hit remaining)
- **Rows:** `T4-001` through `T4-015`
  - Source: Ragu Memo (`http://ragmemo.s33.xrea.com/abnormal.html`), captured 2003-08-02 (`ragmemo_abnormal_20030802.html`) and 2003-12-12 (`ragmemo_abnormal_20031212.html`).
  - **Poison (`毒`):**
    - 2003-08-02: `Success = 基本毒率 * (100 - (3 + VIT + LUK/3)) [%] (LUK/3ではなくLUK/5との噂も)` (`T4-001`).
    - 2003-12-12: `Success = 基本毒率 * (100 - (3 + VIT + LUK/5)) [%] (LUK/5ではなくLUK/15との噂も)` (`T4-002`).
    - HP drain & DEF penalty: loses 3% MaxHP every 3 seconds down to 25% MaxHP; DEF reduced by 25% (`T4-003`).
    - Duration: Monster 10s–30s; Player 5s–60s depending on target VIT and LUK (`T4-004`).
  - **Stun (`スタン`):**
    - Success and duration: `Success = 基本スタン率 * (100 - (3 + VIT + LUK/3)) [%]`, `Time = 3 * (1 - (3 + VIT + LUK/3)/100) [s]` (`T4-005`).
    - Effects: Prevents all actions and evasion; lasts ~3s reduced by VIT and LUK (`T4-006`).
  - **Sleep (`睡眠`):**
    - Success: `Success = 基本睡眠率 * (100 - (3 + INT + LUK/3)) [%]`, lasts ~30s, and doubles critical hit rate taken (`T4-007`).
  - **Freeze (`凍結`):**
    - 2003-12-12 formula: `Success = 基本凍結率 * (100 - (3 + MDEF + LUK/3)) [%]` (`T4-008`).
    - Duration and effects: `Time = 基本凍結時間 * (1 - MDef/100)`, element becomes Water 1, DEF -50%, MDEF +25%, nullifies Lex Aeterna and Steal (`T4-009`).
  - **Stone Curse (`石化`):**
    - Success and duration: `Success = 基本石化率 * (100 - (3 + INT + LUK/3)) [%]`, `Time = 17 - (MDef/20) [s]` (`T4-010`).
    - Effects: Element becomes Earth 1, DEF -50%, MDEF +25%, loses 1% MaxHP every 5s down to 25% MaxHP (`T4-011`).
  - **Blind (`暗黒`):**
    - Success and duration: `Success = 基本暗黒率 * (100 - (3 + INT + LUK/3)) [%]`, `Time = 30 - BaseLv/10 - INT/15 [s]` (15s–30s), FLEE -25%, HIT -25%, player sight radius becomes 10 cells (`T4-012`).
  - **Silence (`沈黙`):**
    - Success and duration: `Success = 基本沈黙率 * (100 - (3 + VIT + LUK/3)) [%]`, `Time = 基本持続時間 * (100 - VIT)/100 [s]`, prevents all skill usage (`T4-013`).
  - **Curse (`呪い`):**
    - Effects: ATK -25%, LUK = 0, resisted by LUK (`T4-014`).
  - **Confusion (`混乱`):**
    - Status: Unimplemented `(未実装)` (`T4-015`).

### T10: Novice Training Ground Outside Japan (Thread 6)
- **Status:** partly (Fabre forest wake-up cell on knockout remaining)
- **Rows:** `T10-001` through `T10-009`
  - Source: RODataZone `quests.js` (`rodatazone_quests_js_20040417.html`).
  - **Uscher's Subsidy:** Awards `50z` in `new_1-1` upon agreeing to do the course (`QUESTS[2]`) (`T10-001`).
  - **Newbie Instructor:** Awards `1 Milk` at `new_1-2 81/110` (`QUESTS[3]`) (`T10-002`).
  - **Leo Von Frisch's Quiz:** Awards `1 Banana, 3 Sweet Potato, 1 Apple, 1 Meat, 1 Milk, 4 Red Herb` at `new_1-2 84/115` (`QUESTS[4]`) (`T10-003`).
  - **Personality Test Result:** Awards `(3 Red Herb, 1 Milk, 100z) or (5 Red Herb, 1 Milk, 150z)` from Helper at `new_1-3 60/149` (`QUESTS[5]`) (`T10-004`).
  - **Stand-In Job Subsidies (`new_1-4`):**
    - Swordsman: `1 Sword, 1 Red Potion, 3 Meat`, warped to Swordsman Guild in Izlude (`QUESTS[10]`) (`T10-005`).
    - Archer: `Red Potion, 6 Banana, 6 Pumpkin, 6 Carrot`, warped to Archer Village (`QUESTS[7]`) (`T10-006`).
    - Thief: `Leather Jacket, 150z`, warped to Morroc Town (`QUESTS[8]`) (`T10-007`).
    - Merchant: `300z`, warped to Alberta Town (`QUESTS[9]`) (`T10-008`).
    - Acolyte: `Club, Red Potion, 2 Milk, 8 Apple`, warped to Prontera Sanctuary (`QUESTS[6]`) (`T10-009`).

### T6: Dyeing (Thread 13)
- **Status:** partly (kRO hair dyeing notice date remaining)
- **Rows:** `T6-001` through `T6-011`
  - **Clothes Dyeing:** Confirmed `Not Yet Implemented!!!!` on iRO by RODataZone `quests.js` (`QUESTS[1]`) for Dyer Ginedin Rephere at `prt_in 284/168` (`T6-001`).
  - **NPC Official English Names:**
    - `Louitz` (`alberta_in 133/55`), `Aure Dupon` (`geffen 181/114`), `Molgenshtein` (`geffen_in 141/140`) from `ACTIVATION[1]` (`T6-002`).
    - `JavaDullihan` (`morocc_in 146/99`) from `MIXING[0]` (`T6-003`).
    - `Jovovich` (`prt_in 243/168`) from mod0631 `conf/npc_event_dye.txt:653` (`T6-011`).
  - **JavaDullihan's 8 Dyestuff Recipes (`MIXING[0..7]`):**
    - Scarlet: `30 Red Herb + 1 Counteragent + 1 Empty Bottle + 3000z` (`T6-003`).
    - Lemon: `30 Yellow Herb + 1 Counteragent + 1 Empty Bottle + 3000z` (`T6-004`).
    - Cobaltblue: `20 Blue Herb + 1 Counteragent + 1 Empty Bottle + 3500z` (`T6-005`).
    - Darkgreen: `5 Blue Herb + 20 Green Herb + 20 Yellow Herb + 1 Counteragent + 1 Empty Bottle + 5000z` (`T6-006`).
    - Orange: `20 Red Herb + 20 Yellow Herb + 1 Counteragent + 1 Mixture + 1 Empty Bottle + 5000z` (`T6-007`).
    - Violet: `10 Blue Herb + 30 Red Herb + 1 Counteragent + 1 Mixture + 1 Empty Bottle + 5000z` (`T6-008`).
    - White: `30 White Herb + 1 Counteragent + 1 Empty Bottle + 3000z` (`T6-009`).
    - Black: `30 Red Herbs + 30 Yellow Herbs + 30 Green Herbs + 5 Blue Herb + 1 Counteragent + 1 Mixture + 1 Empty Bottle + 7000z` (`T6-010`).

### T5: Turtle Island Quests (Thread 24)
- **Status:** partly (Alberta sailor fare/conditions and logbook errands remaining)
- **Rows:** `T5-001` through `T5-008`
  - Source: RODataZone `quests.js` (`HATS[19..26]`).
  - **Stylish Merchant (`alberta 120/53`):**
    - Heart Hair Pin: `1200 Coral Reef` (Turtle Island drop) (`T5-001`).
    - Fashionable Glasses: `1 Jack a Dandy, 1 Scarlet Dyestuffs` (`T5-002`).
    - First Moon Hair Pin: `1 Heart Hair Pin, 10 Steel` (`T5-003`).
    - Hair Protector: `1 Silk Ribbon, 50 Heroic Emblem` (`T5-004`).
  - **Hat Store Girl (`alberta 136/79`):**
    - Sunday Hat: `250 Fabric, 1 Slotted Hat, 1 Slotted Cap, 600 Soft Feather` (`T5-005`).
    - Mage Hat: `1 Wizard Hat, 400 Dragon Scale, 50 Mould Powder, 1 Elder Willow Card` (`T5-006`).
    - Magician Hat: `1 Wizard Hat, 450 Ancient Lips, 1200 Solid Shell` (Turtle Island drop) (`T5-007`).
    - Helm of Solar God: `1 Emblem of the Sun God, 10 Gold, 40 Steel, 50 Coal, 2 Oridecon` (`T5-008`).

### T7: 2-2 Skills — Land Protector Duration (Threads 22, 25)
- **Status:** partly (Grand Cross, Alchemist combat skills, Rogue strip formulas, Dispell effect list remaining)
- **Rows:** `T7-001` and `T7-002`
  - **Official iRO Guide (2004-03-27):** Magnetic Earth duration scales from 2 minutes to 5 minutes:
    - Level 1: 7 cells, 2 min, 66 SP
    - Level 2: 7 cells, 2 min 45sec, 62 SP
    - Level 3: 9 cells, 3 min 30sec, 58 SP
    - Level 4: 9 cells, 4 min 15sec, 54 SP
    - Level 5: 11 cells, 5 min, 50 SP (`T7-001`).
  - **eAthena 817 `db/cast_db.txt:145`:** SA_LANDPROTECTOR duration is defined as `120000:150000:180000:210000:240000` (capped at 4 minutes) (`T7-002`).

### T9: Small, Independent
- **Status:** partly (Clock Tower random warps, monster skill cast rates, Unknown Skill !! accounts, ground skills seen twice remaining)
- **Rows:** `T9-001` through `T9-021`
  - **Alberta Doll Child Trades:**
    - RODataZone `quests.js` (`DOLLS[2..10]`): Poring Doll -> Candy (`T9-001`), Chon Chon Doll -> Candy Cane (`T9-002`), Puppet -> Candy Cane (`T9-003`), Rocker Doll -> 7 Banana Juice (`T9-004`), Spore Doll -> 5 Well-Baked Cookie (`T9-005`), Osiris Doll -> 2 Mastela Fruit (`T9-006`), Baphomet Doll -> 5 Panacea (`T9-007`), Racoon Doll -> 3 Piece of Cake (`T9-008`), Yoyo Doll -> Yggdrasil Seed (`T9-009`).
    - Ragu Memo `trade.html` (2004-02-22): Confirms Rocker Doll -> 7 Banana Juice, Spore Doll -> 5 Well-baked Cookie, Smokie Doll -> 3 Piece of Cake, Baphomet Doll -> 2 Royal Jelly (`T9-010`).
  - **Shockwave Trap Requirements:**
    - Client `skilldesctable.txt`: States `^00bb002 Traps^777777Needed.` (`T9-011`).
    - mod0631 `map/skill.c:2918`: Consumes 1 Trap (`item_amount[0]+=1;`) (`T9-012`).
    - eAthena 817 `db/skill_require_db.txt:97`: Requires 1 Trap (`1065,1`) (`T9-013`).
    - jAthena `db/skill_require_db.txt:96`: Requires 1 Trap (`1065,1`) (`T9-014`).
  - **Unreachable Map Locations:**
    - `tur_dun05`: 100x100 map with 3,333 walkable cells in merged `data.grf` (`T9-015`). Exits at 89,69 and 9,45 lead to `tur_dun02 148,264` and `tur_dun01 154,237` (mod0631 `conf/npc_warp25.txt:13-14`) (`T9-016`), with 0 inbound warps in mod0631, 817, eAthena 0.5.2, jAthena, or rAthena.
    - `pay_dun04 193,43`: Blocked (type 1) in `data.grf` while adjacent cells `191,41` and `192,42` are walkable (type 0) (`T9-017`). All emulators (817 line 108, mod0631 line 511, jAthena line 499, eAthena 0.5.2 line 108) warp to blocked cell `193,43` (`T9-018`).
    - `prtg_cas02 184,44` to `157,140`: Target cell `157,140` and return cell `157,135` are blocked (type 1) in `data.grf` (`T9-019`). Defined bidirectionally in mod0631 `conf/npc_warp25.txt:363-364`, 817 line 339, and jAthena line 363 (`T9-020`).
  - **WoE Castle Treasure Box Godly Material Drop Rate:**
    - Ragu Memo `gvg_box.html` (2004-02-06): Confirms 1/250 drop rate for all Godly item materials across all 20 castles (`T9-021`).

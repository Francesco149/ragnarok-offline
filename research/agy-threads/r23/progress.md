# Round 23 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | 2-1 job tests (Knight change pre-Sept 2004 fee, Wizard test requirements, rooms, skip) | partly (Knight 2-1 change confirmed pre-Sept 2004 fee of 2,500z in Weggy FAQ 20092, Edge Knight Guide 29457, and Psycho Battousai 21546; Wizard test confirmed requiring 10 each of Blue/Red/Yellow Gemstones or 5 each of 4 elemental stones in Kiruwa FAQ 30054; Wizard water room confirmed Marine Sphere 3518 HP with Hydra; Wizard fire room confirmed Gatekeeper Horongs; Wizard test failure teleports outside Geffen tower; 5-6 failures allows combat test skip with Worn Out Scroll) | T1-059 – T1-066 |
| T4 | Status ailments (Ragu Memo formulas for Stone Curse, Blind, Silence, Stun, Chaos) | partly (Stone Curse confirmed draining 1% HP every 5s down to 25% MaxHP and ~17s duration slightly reduced by MDEF; Blind duration confirmed 30 - BaseLv/10 - INT/15 s; Silence duration confirmed Base * (100 - VIT)/100 s; Stun duration confirmed 3 * (1 - (3 + VIT + LUK/3)/100) s; Chaos confirmed un-implemented on jRO as of February 2004 in ragmemo_abnormal) | T4-022 – T4-027 |
| T8 | Things only drafted from client text (Sightrasher pattern, Comodo patch dating) | partly (Sightrasher confirmed firing into 8 directions with level-based range scaling in Official iRO Wizard Guide; jAthena HEAD map/skill.c:3989 confirmed hardcoding count=41; eAthena 817 map/skill.c:4077 confirmed hardcoding 24 coordinates; Comodo patch confirmed live on pRO on or before 2003-12-17 in Battousai X Guide 27646) | T8-045 – T8-049 |
| T9 | Small, independent (Novice item wearability, 1st job test locations) | partly (Fortune Sword confirmed equippable by Novices in Mad Monarch Gyl FAQ 30075 and client GRF data\idnum2itemdesctable.txt item 1223; mod0631 db/item_db.txt:443 confirmed bugged job mask 2055918 excluding Novices; Corsair confirmed equippable by Novices in FAQ 30075; Acolyte quest 3 ascetic priests confirmed in Yoyoland/Prontera ruins, North Morroc NW moc_fild07, and East Geffen island prt_fild00 in H0BGOBLIN FAQ 18212) | T9-278 – T9-283 |
| T10 | Novice training grounds (Starter items and recommendations) | partly (Starter equipment confirmed exactly 1 Knife and 1 Cotton Shirt in Mistiknoname3k FAQ 20164; Thief recommendation confirmed yielding bonus Zeny) | T10-023 – T10-024 |

Total: 27 rows across 5 tasks.
Checker result (`check.txt`): 27 PASS, 0 WARN, 0 FAIL, 0 NONE of 27 rows verified with `python3 /opt/src/ragnarok-offline/research/check_rows.py --pages pages rows/*.jsonl`.

## Task Details and Row Mapping

### T1: 2-1 Job Tests
- **Status:** partly
- **Rows:** `T1-059` through `T1-066`
  - **Knight Job Change (Pre-Sept 2004):**
    - `T1-059`: Weggy Priest Guide v6.0 (2004-05-17, GameFAQs 20092) line 963 confirms becoming a Knight had no test before the September 2004 quest was added; players reached Job 40 and paid 2,500 zeny to the Knight NPC in Northwest Prontera (`To become a Knight, talk to the Knight in the Culvert Registration building in Northwest Prontera. If you're job level 40, you can pay him 2,500 zeny to become a Knight immediately.`).
    - `T1-060`: Edge Knight/Crusader Guide v2.6 (2004-02-08, GameFAQs 29457) line 269 confirms pRO players paid a small fee to change to Knight without a quest (`For pRO, there's no quest. Just pay a small fee and you're good to go.`).
    - `T1-061`: Psycho Battousai Swordsman/Knight Guide v1.3 (2003-02-25, GameFAQs 21546) line 162 confirms the exact fee in Northwest Prontera was 2,500z (`All you need is 2,500z and a Job Level of 40. Go to the building in the Northwest corner of Prontera.`).
  - **Wizard Test (Kiruwa FAQ 30054):**
    - `T1-062`: Kiruwa Wizard/Mage FAQ v2.1 (2004-07-30, GameFAQs 30054) lines 280-285 confirms the item requirements: 10 Blue Gemstones, 10 Red Gemstones, and 10 Yellow Gemstones, OR 5 each of Green Lives, Red Bloods, Crystal Blues, and Winds of Verdure.
    - `T1-063`: Kiruwa FAQ lines 309-312 confirms the Water Room contains a Marine Sphere (3518 HP) with 1 Hydra, and letting the Marine Sphere self-destruct fails the test (`Water: 1 water room. 1 Marine Sphere, 1 Hydra. HP: 3518 / 1. DO NOT LET THE MARINE SPHERE SELF DESTRUCT! If it does, you will automatically fail.`).
    - `T1-064`: Kiruwa FAQ lines 318-323 records player report that the Marine Sphere could also be killed by letting it explode if it hit the player directly without failing, though killing it outright was standard.
    - `T1-065`: Kiruwa FAQ lines 335-336 confirms the Fire Room gatekeeper is Horongs that walk right through Firewall (`The Gatekeeper horongs, on the other hand, will require something else, since they walk right through firewall.`).
    - `T1-066`: Kiruwa FAQ lines 338-342 confirms that failing the test teleports the player outside Geffen tower, and failing 5–6 times allows skipping the combat test by handing the NPC a Worn Out Scroll (`If you fail 5-6 times, pay careful attention to what the NPC is telling you... he should give you an option to skip the combat test if you give him a worn out scroll.`).

### T4: Status Ailments and Durations
- **Status:** partly
- **Rows:** `T4-022` through `T4-027`
  - `T4-022`: Ragu Memo abnormal status page (2004-02-05, `ragmemo.s33.xrea.com/abnormal.html`) line 40 confirms Stone Curse drains 1% HP every 5 seconds down to 25% of MaxHP (`・5秒ごとに、MaxHPの25%まで1%づつHPが減少`).
  - `T4-023`: Ragu Memo line 41 confirms Stone Curse duration is approximately 17 seconds, slightly reduced by MDEF (`・持続時間は約17秒。MDEFで僅かに減少?`).
  - `T4-024`: Ragu Memo line 209 confirms Blind duration formula is `Time = 30 - BaseLv/10 - INT/15 [s]`.
  - `T4-025`: Ragu Memo line 238 confirms Silence duration formula is `Time = 基本持続時間 * (100 - VIT)/100 [s]`.
  - `T4-026`: Ragu Memo line 77 confirms Stun duration formula is `Time = 3 * (1 - (3 + VIT + LUK/3)/100) [s]`.
  - `T4-027`: Ragu Memo line 292 confirms Chaos/Confusion status ailment was un-implemented on jRO as of February 2004 (`(未実装)`).

### T8: Things Only Drafted from Client Text
- **Status:** partly
- **Rows:** `T8-045` through `T8-049`
  - `T8-045`: Official iRO Wizard Skill Guide (2004-03-27, `iro.ragnarokonline.com`) confirms Sightrasher spreads fire balls around the caster into 8 directions (`Spreads the fire ball around the caster to 8 directions, does fire damage and pushes back the enemies around the caster at the same time.`).
  - `T8-046`: Official iRO Wizard Skill Guide confirms Sightrasher range scales by skill level (+1 cell at Lv 1-2, +2 at Lv 3-4, +3 at Lv 5-6, +4 at Lv 7-8, +5 at Lv 9-10).
  - `T8-047`: jAthena commit `525287f32997` (2004-06-25, //1176 by shadow) `src/map/skill.c:3989` hardcodes `count=41` for Sightrasher (`int area = 5, count = 41;`), firing 5 cells outward at all skill levels.
  - `T8-048`: eAthena 817 `src/map/skill.c:4077` (2004-02-03) hardcodes 24 static coordinate offsets for Sightrasher (3 cells outward) regardless of level (`static const int dir_x[24] = ...`).
  - `T8-049`: Battousai X Comodo Skills Guide v1.0 (2003-12-17, GameFAQs 27646) line 24 confirms Philippine Ragnarok Online (pRO) received the Comodo update on or before December 17, 2003 (`A lot of people were confused by the sudden update of Comodo last December`).

### T9: Small, Independent
- **Status:** partly
- **Rows:** `T9-278` through `T9-283`
  - `T9-278`: Mad Monarch Gyl Perma-Novice Guide v2.6 (2004-05-16, GameFAQs 30075) line 393 confirms Novices equipped the Fortune Sword for perfect dodge on official servers (`R-Arm= Fortune Sword`).
  - `T9-279`: Client GRF `data\idnum2itemdesctable.txt` item 1223 includes Novice in the applicable jobs line (`Applicable Job : ^777777Novice Swordman Mage Archer Merchant Thief^000000`).
  - `T9-280`: mod0631 `db/item_db.txt:443` (2003-12-18) assigns Fortune Sword item 1223 job mask `2055918` (`0x1F5DE`), which erroneously leaves bit 0 (Novice) unset, excluding Novices from equipping it.
  - `T9-281`: Mad Monarch Gyl FAQ 30075 line 315 confirms Novices equipped the Corsair headgear (`Head = Corsair`).
  - `T9-282`: H0BGOBLIN Acolyte/Priest FAQ v1.7 (2003-07-25, GameFAQs 18212) line 160 describes Priest #1 located in yoyoland / Prontera ruins.
  - `T9-283`: H0BGOBLIN FAQ 18212 lines 167-173 describes Priest #2 located in the northwest corner of the map north of Morroc (`moc_fild07`), and Priest #3 located on the island east of Geffen (`prt_fild00`).

### T10: Novice Training Grounds
- **Status:** partly
- **Rows:** `T10-023` through `T10-024`
  - `T10-023`: Mistiknoname3k Assassin Guide v1.2 (2003-12-13, GameFAQs 20164) line 125 confirms the starting inventory upon character creation was exactly 1 Knife and 1 Cotton Shirt (`You will start in the Training Ground with a knife and a cotton shirt.`).
  - `T10-024`: Mistiknoname3k FAQ 20164 line 133 confirms that receiving a Thief recommendation in the training grounds awarded bonus Zeny (`if you choose either thief or merchant, you will receive extra zeny from them`).

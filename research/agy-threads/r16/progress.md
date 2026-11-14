# Round 16 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 23, 29, 31, R6) | partly (Four unmade hats and Kafra items investigated: Panda Cap confirmed as Sasquatch drop from Ragu Memo 2003-08-21 and eAthena 817 mob_db, unreleased status confirmed in 2003-12-18 client, official display names recovered from May 2004 client, and lack of official maker quests documented from rAthena; item wearer disagreements between client descriptions, mod0631 masks, and RODataZone armor tables documented for Miner's Helmet, Magic Hat, Cake Hat, Munak Hat, Boys Cap, Solar God Helm, and Partizan level; 10 monster drop holes in mod0631 filled by 817 verified against RODataZone monster tables; remaining: pre-2004-05 source for Baphomet doll 5 Panacea vs 2 Royal Jelly and Merchant letter 100z vs 500z, pre-2004-02 item tables, 73 unmatched item names, Clock Tower random warps, Unknown Skill !! accounts, monster skill cast rates, ground skills seen twice) | T9-090 – T9-124 |

Total: 35 rows across 1 task.
Checker result (`check.txt`): 35 PASS, 0 WARN, 0 FAIL, 0 NONE of 35 rows verified with `python3 research/check_rows.py --fetch`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 23, 29, 31, R6)
- **Status:** partly
- **Rows:** `T9-090` through `T9-124`
  - **Four Unmade Hats & Kafra Items (THREAD 31 & T9):**
    - `T9-090`: Panda Cap (5030, パンダ帽) in Ragu Memo (`turtle_armor.html`, 2003-08-21 capture) is a mob drop from Sasquatch (`サスカッチ(1/5000)`), required level 40 (`-/40`).
    - `T9-091`: Sasquatch (1243) in eAthena 817 `db/mob_db.txt:243` drops item 5030 at rate 1 (0.01%).
    - `T9-092`: Panda Cap (5030) in the 2003-12-18 client has an entry in `data\idnum2itemdesctable.txt` but no entry in `data\idnum2itemdisplaynametable.txt`.
    - `T9-093`: Panda Cap (5030) received official display name `Panda_Hat` in euRO May 2004 client (`yuno_lang/idnum2itemdisplaynametable.txt:1156`).
    - `T9-094`: Kafra Band (5020) in 2003-12-18 client has an entry in `data\idnum2itemdesctable.txt` but no entry in `data\idnum2itemdisplaynametable.txt`.
    - `T9-095`: Kafra Band (5020) received official display name `Kafra_Band` in euRO May 2004 client (`yuno_lang/idnum2itemdisplaynametable.txt:1149`).
    - `T9-096`: Wings of Demon (5044) received official display name `Wing_of_Diablo` in euRO May 2004 client (`yuno_lang/idnum2itemdisplaynametable.txt:1159`).
    - `T9-097`: Bell of Pussycat (5051) received official display name `Bell_of_Pussycat` in euRO May 2004 client (`yuno_lang/idnum2itemdisplaynametable.txt:1199`).
    - `T9-098`: Kafra Payroll (7056) received official display name `Payroll_of_Kafra` in euRO May 2004 client (`yuno_lang/idnum2itemdisplaynametable.txt:1213`).
    - `T9-099`: Kafra Free Storage Ticket (7059) received official display name `Cargo_Free_Ticket` in euRO May 2004 client (`yuno_lang/idnum2itemdisplaynametable.txt:1220`).
    - `T9-100`: rAthena custom quest script `event_6_new_hats.txt` explicitly documents that Kafra Band and Panda Hat had no official quests on the official server (`"hats which cannot be made on the real server"`).
  - **Item Wearer Disagreements (THREAD R6 / T9):**
    - `T9-101`: Miner's Helmet (5031) client description states `Available Job : ^777777All^000000`.
    - `T9-102`: Miner's Helmet (5031) in mod0631 `item_db.txt:1076` restricts equip jobs to mask 447986 (12 jobs) rather than all jobs.
    - `T9-103`: Magic Hat (5045) client description states `Available Job : ^777777All^000000`.
    - `T9-104`: Magic Hat (5045) in RODataZone (`items_armor.js:170`) confirms available to `All`, matching client description against mod0631 (mask 99092, 6 magic jobs).
    - `T9-105`: Cake Hat (5024) client description states `Available Job : ^777777All except Novice^000000`.
    - `T9-106`: Cake Hat (5024) in RODataZone (`items_armor.js:117`) confirms available to `All except Novice`, matching client description against mod0631 (2088959, which allows Novice).
    - `T9-107`: Munak Hat (2264) client description states `Applicable Job : ^777777Every Job(Except Novice)^000000`.
    - `T9-108`: Munak Hat (2264) in RODataZone (`items_armor.js:187`) confirms available to `All except Novice`, matching client description against mod0631 (2088959).
    - `T9-109`: Boys Cap (5016) client description states `Applicable Job : ^777777Swordman Merchant^000000`.
    - `T9-110`: Boys Cap (5016) in RODataZone (`items_armor.js:189`) lists `Blacksmith, Knight, Merchant, Swordsman`, matching client job classes against mod0631 (2088958, all non-novice).
    - `T9-111`: Solar God Helm (5022) client description states `Available Job : ^777777Swordman Merchant Thief^000000`.
    - `T9-112`: Solar God Helm (5022) in RODataZone (`items_armor.js:115`) lists `All 2nd Jobs`, matching mod0631 (2088832, all 2nd jobs) against client description.
    - `T9-113`: Partizan (1459) client description states `Required Level : ^77777718^000000`.
    - `T9-114`: Partizan (1459) in mod0631 `item_db.txt:689` sets `elv = 33` against client description.
  - **Monster Drop Holes in mod0631 Filled by 817 & Confirmed in RODataZone (THREAD R6 / T9):**
    - `T9-115`: Soldier Skeleton (1028) drops Stiletto (RODataZone `monsters_monsters.js:33`).
    - `T9-116`: Moonlight Flower (1150) drops Silver Knife of Chastity (`monsters_monsters.js:154`).
    - `T9-117`: Petite (Ground) (1155) drops Aloevera (`monsters_monsters.js:160`).
    - `T9-118`: Skeleton Prisoner (1196) drops Memento (`monsters_monsters.js:201`).
    - `T9-119`: Zealotus / Zherlthsh (1200) drops Masquerade (`monsters_monsters.js:205`).
    - `T9-120`: Tirfing (1204) drops Two-handed Sword (`monsters_monsters.js:209`).
    - `T9-121`: Myst Case (1249) drops 2 Carat Diamond (`monsters_monsters.js:254`).
    - `T9-122`: Mutant Dragon (1262) drops Mastela Fruit (`monsters_monsters.js:267`).
    - `T9-123`: Stalactic Golem (1278) drops Rough Elunium (`monsters_monsters.js:283`).
    - `T9-124`: Sage Worm (1281) drops Book (`monsters_monsters.js:286`).

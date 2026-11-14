# Round 16 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 17, 23, 29, 31, R6) | partly (Four unmade hats and Kafra items investigated: Panda Cap confirmed as Sasquatch drop from Ragu Memo 2003-08-21 and eAthena 817 mob_db, unreleased status confirmed in 2003-12-18 client, official display names recovered from May 2004 client, and lack of official maker quests documented from rAthena; item wearer third witnesses established from Ragu Memo 2003-12-12 and 2003-12-18 equipment tables for Solar God Helm [all 2nd jobs], Poring Hat [all except Novice], Miner's Helmet [Swd/Aco/Mch trees], Wizard Hat [Mag/Wiz/Sag], Smokie Hat [all except Novice], Deviruchi Hat [all except Novice], Spore Hat [all except Novice], Zeny Knife [no Novice], Fortune Sword [no Novice], Moonlight Dagger [no Novice], Assassin Dagger [Assassin only], Mysteltainn [no Novice], and Partizan [base level 18]; 10 monster drop holes in mod0631 filled by 817 verified against RODataZone monster tables; WoE god item material rate confirmed at 1/250 across castle treasure boxes in Ragu Memo 2004-02-06, plus common treasure box rates; monster skill cast rates, HP 30% threshold, delay definition, and exact cast rates/cooldowns documented for Marduk Fire Bolt Lv5, Willow/Elder Willow/Cookie/Petite Napalm Beat Lv10, Evil Druid Heaven's Drive Lv4, Joker Soul Strike/Jupitel Thunder/Heaven's Drive, and Wind Ghost uninterruptibility; remaining: pre-2004-05 source for Baphomet doll 5 Panacea vs 2 Royal Jelly and Merchant letter 100z vs 500z, pre-2004-02 item tables, 73 unmatched item names, Clock Tower random warps, Unknown Skill !! accounts, ground skills seen twice) | T9-090 – T9-153 |
| T8 | Things only drafted from the client's text (thread 22) | partly (Hunter trap formulas from Ragu Memo 2003-12-05: Land Mine, Blast Mine, and Claymore Trap exact damage formulas; Blitz Beat falcon damage formula; remaining: Ice Wall HP/attackability, Poison React / Remove Trap / Spring Trap / Venom Splasher delay / Talkie Box numbers, arrow ATK bonus in Dec 2003, date iRO Comodo update went live, Sightrasher cell pattern, Frost Nova animation) | T8-001 – T8-004 |

Total: 68 rows across 2 tasks.
Checker result (`check.txt`): 68 PASS, 0 WARN, 0 FAIL, 0 NONE of 68 rows verified with `python3 research/check_rows.py --fetch`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 17, 23, 29, 31, R6)
- **Status:** partly
- **Rows:** `T9-090` through `T9-153`
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
  - **Item Wearer Third Witnesses (Ragu Memo jRO, THREAD 31 / T9):**
    - `T9-125`: Helm of the Solar God (5022) in Ragu Memo `armor5.html` (2003-12-12) restricts equip to 2nd jobs only (`OffEquip Nov, Swd, Thf, Mag, Aco, Arc, Mch`), confirming mod0631 mask 2088832 and RODataZone against client description.
    - `T9-126`: Poring Hat (5033) in Ragu Memo `armor5.html` restricts equip to all except Novice (`OffEquip Nov`), confirming mod0631 and RODataZone against client description.
    - `T9-127`: Miner's Helmet (5031) in Ragu Memo `armor5.html` restricts equip to Swordman, Acolyte, Merchant trees (`OnEquip Swd, Aco, Mch`), confirming mod0631 mask 447986 against client description.
    - `T9-128`: Wizard Hat / Magic Hat (5045) in Ragu Memo `armor5.html` restricts equip to Mage, Wizard, Sage (`OnEquip Mag, Wiz, Sag`), confirming magic job restriction against client description ("All").
    - `T9-129`: Smokie Hat (5032) in Ragu Memo `armor5.html` restricts equip to all except Novice (`OffEquip Nov`), confirming mod0631 mask 2088958 against client description ("Every Job").
    - `T9-130`: Deviruchi Hat (5038) in Ragu Memo `armor5.html` restricts equip to all except Novice (`OffEquip Nov`), confirming mod0631 mask 2088958 against client description ("Every Job").
    - `T9-131`: Spore Hat (5029) in Ragu Memo `armor5.html` restricts equip to all except Novice (`OffEquip Nov`), confirming mod0631 mask 2088958 against client description ("Every Job").
    - `T9-132`: Zeny Knife (1227) in Ragu Memo `weapon1.html` (2003-12-18) excludes Novice (`OffEquip Nov`), confirming mod0631 against client description ("Novice").
    - `T9-133`: Fortune Sword (1222) in Ragu Memo `weapon1.html` (2003-12-18) excludes Novice (`OffEquip Nov`), confirming mod0631 against client description ("Novice").
    - `T9-134`: Moonlight Dagger (1223) in Ragu Memo `weapon1.html` (2003-12-18) excludes Novice (`OffEquip Nov`), confirming mod0631 against client description ("Novice").
    - `T9-135`: Assassin Dagger (1228) in Ragu Memo `weapon1.html` (2003-12-18) is equippable by Assassin only (`OnEquip Asn`), confirming mod0631 against client description ("Novice...").
    - `T9-136`: Mysteltainn (1134) in Ragu Memo `weapon2.html` (2003-12-18) excludes Novice (`OffEquip Nov`), confirming mod0631 against client description ("Novice").
    - `T9-137`: Partizan (1459) in Ragu Memo `weapon4.html` (2003-12-18) requires base level 18 (`2/18`), confirming client description against mod0631 elv 33.
  - **WoE Numbers for Castle Rework (THREAD 17 / T9):**
    - `T9-138`: Symbol of the Sun God material drop rate in Chung-Rim Guild 4 treasure box is 1/250 in Ragu Memo `gvg_box.html` (2004-02-06).
    - `T9-139`: Valkyrie's Wrath material drop rate in Britoniah Guild 1 treasure box is 1/250 in Ragu Memo `gvg_box.html`.
    - `T9-140`: Thor's Gauntlets material drop rate in Luina Guild 2 treasure box is 1/250 in Ragu Memo `gvg_box.html`.
    - `T9-141`: Old Card Album drop rate in castle-common treasure boxes is 1/20 in Ragu Memo `gvg_box.html`.
  - **Monster Skill Cast Rates & Delays (THREAD 23 / T9):**
    - `T9-142`: Mid-to-high level monsters have higher skill cast rates when HP is under 30% (`残りHP30%未満になるとスキル使用率が高くなる`, Ragu Memo `at_monskill.html`, 2003-12-05).
    - `T9-143`: Standby delay is defined as the time until the same skill can be used again (`同じスキルを次に使用できるまでの時間`, Ragu Memo `at_monskill.html`).
    - `T9-144`: Marduk (マルドゥーク) Fire Bolt Lv5 cast rate is 10-30%, cast 4000ms, cooldown 10000ms (Ragu Memo `at_monskill.html`).
    - `T9-145`: Willow (ウィロー) Napalm Beat Lv10 cast rate is 10%, cast 1000ms, cooldown 10000ms.
    - `T9-146`: Elder Willow (エルダーウィロー) Napalm Beat Lv10 cast rate is 10%, cast 1000ms, cooldown 10000ms.
    - `T9-147`: Cookie Green (緑クッキー) Napalm Beat Lv10 cast rate is 12%, cast 1000ms, cooldown 10000ms.
    - `T9-148`: Petite Ground (地プティ) Napalm Beat Lv10 cast rate is 12%, cast 1000ms, cooldown 10000ms.
    - `T9-149`: Evil Druid (イビルドルイド) Heaven's Drive Lv4 cast rate is 10%, cast 4000ms, delay 10000ms, uninterruptible (`不可`).
    - `T9-150`: Joker (ジョーカー) Soul Strike Lv10 cast rate is 5%, cast 1000ms, delay 5000ms.
    - `T9-151`: Joker (ジョーカー) Jupitel Thunder Lv5 cast rate is 5%, cast 4000ms, delay 10000ms.
    - `T9-152`: Joker (ジョーカー) Heaven's Drive Lv4 cast rate is 5%, cast 4000ms, delay 10000ms, uninterruptible (`不可`).
    - `T9-153`: Wind Ghost (ウィンドゴースト) uniquely becomes uninterruptible when HP is below 30% (`ウィンドゴーストのみ、HP30%未満になると妨害不可になる特性がある`).

### T8: Things Only Drafted from the Client's Text (Thread 22)
- **Status:** partly
- **Rows:** `T8-001` through `T8-004`
  - `T8-001`: Land Mine damage formula is `floor((dex + 75) * (1 + int/100) * SkillLv,1)` (Ragu Memo `hunter.html`, 2003-12-05).
  - `T8-002`: Blast Mine damage formula is `floor((dex/2 + 50) * (1 + int/100) * SkillLv,1)` (Ragu Memo `hunter.html`).
  - `T8-003`: Claymore Trap damage formula is `floor((dex/2 + 75) * (1 + int/100) * SkillLv,1)` (Ragu Memo `hunter.html`).
  - `T8-004`: Blitz Beat falcon damage formula is `(floor(Dex/10,1) + floor(Int/2,1) + StealCrowLv*3 + 40) * 2 * SkillLv` (Ragu Memo `hunter.html`).

# Round 17 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | Job tests added after Dec 2003 (threads 1, 10, 11) | partly (Blacksmith job change on iRO confirmed to have no test, no quest, and no fee at Job Level 40/50 in July 2003 from The Only KoW Battlesmith Guide FAQ 24428, corroborating Edge's account that 2-1 job tests were introduced later; remaining: priest, wizard, hunter, knight, assassin test addition dates in early 2004) | T1-042 |
| T4 | Status ailments and conditions (thread 12) | partly (Poison duration recorded as 10-30s on monsters and 5-60s on players, varying by target VIT and LUK, in Ragu Memo abnormal.html 2004-02-05; curse duration verified as unknown across all Ragu Memo captures through August 2004; remaining: exact curse duration formula and recovery mechanics) | T4-020 – T4-021 |
| T9 | Small, independent (threads 26, 31, 32) | partly (Mod0631 empty drop slots filled by 817 verified in RODataZone 2004-02-11 for Clock Tower Manager, Mutant Dragonoid, Stalactic Golem, Sage Worm, Skel Prisoner, Moonlight Flower, Clock, Alligator, Steam Goblin, Marin, Antonio, Mystcase; negative drop witnesses verified in RODataZone for Zombie Prisoner, Mysteltainn, Wanderer, Magnolia, Nine Tail; Clock Tower spawn counts, 30 min respawns, 4F Abysmal Knight 1h fixed point, Ride Word room 15, and Executioner semi-boss confirmed from Ragu Memo 2004-02-05; 23 level 4 weapons and Phantom Opera Mask confirmed equippable by Novice in Soujiro Perma-Novice Guide 2003-07-22; item wearer third witnesses verified from Ragu Memo 2003 for War Axe, Stunner, Iron Driver, Knuckle Duster, Hora, Fist, Prontera Army Cap, Jewel Crown, Cross Hairband, Deviruchi Hat, Miner's Helmet, Phantom Mask; remaining: pre-2004-05 source for Baphomet doll, 73 unmatched item names, Clock Tower random warp coordinates) | T9-154 – T9-215 |
| T10 | Novice Training Ground (thread 6) | partly (Beginning Novice equipment confirmed as 1 Knife and 1 Cotton Shirt from Mistiknoname3k Assassin Guide 2003-12-13; final personality test confirmed to recommend a class, teleport recommended thief to Morroc with extra zenny, or allow choosing original class; remaining: full training ground NPC script dialogues and exact question scoring trees) | T10-021 – T10-022 |

Total: 67 rows across 4 tasks.
Checker result (`check.txt`): 67 PASS, 0 WARN, 0 FAIL, 0 NONE of 67 rows verified with `python3 research/check_rows.py --fetch`.

## Task Details and Row Mapping

### T1: Job Tests Added After Dec 2003 (Threads 1, 10, 11)
- **Status:** partly
- **Rows:** `T1-042`
  - `T1-042`: The Only KoW Battlesmith Guide (GameFAQs 24428, 2003-07-13, iRO) records: "Head to south-western Geffen, the building with smoke rising from it in the very corner. Head inside and become a Blacksmith; it's that simple. No paying, no quest, no nothing." Confirms Blacksmith job change had no test or quest in mid-2003.

### T4: Status Ailments and Conditions (Thread 12)
- **Status:** partly
- **Rows:** `T4-020` through `T4-021`
  - `T4-020`: Ragu Memo `abnormal.html` (2004-02-05, jRO) records poison duration as 10-30s on monsters and 5-60s on players, varying by target VIT and LUK (`・モンスター－持続時間は10秒～30秒で、対象のVITとLUKにより変化 / プレイヤー－持続時間は5秒～60秒で、対象のVITとLUKにより変化`).
  - `T4-021`: Ragu Memo `abnormal.html` (2004-02-05, jRO) records curse duration as unknown (`・持続時間は不明`). Captures checked through August 2004 confirm the text was never updated.

### T9: Small, Independent (Threads 26, 31, 32)
- **Status:** partly
- **Rows:** `T9-154` through `T9-215`
  - **Drops mod0631 Left Empty (THREAD 32 / T9):**
    - `T9-154`: RODataZone (2004-02-11) confirms Tower Keeper (1270, Clock Tower Manager) drops Hinalle Leaflet (`monsters_monsters.js:234`).
    - `T9-155`: RODataZone confirms Tower Keeper (1270) drops Memorize Book (Book of Summoning) (`:234`).
    - `T9-156`: RODataZone confirms Mutant Dragonoid (1262) drops Mastela Fruit (`:152`).
    - `T9-157`: RODataZone confirms Mutant Dragonoid (1262) drops Blue Potion (`:152`).
    - `T9-158`: RODataZone confirms Mutant Dragonoid (1262) drops White Potion (`:152`).
    - `T9-159`: RODataZone confirms Stalactic Golem (1278) drops Rough Elunium (`:221`).
    - `T9-160`: RODataZone confirms Sage Worm (1281) drops Book (`:197`).
    - `T9-161`: RODataZone confirms Skel Prisoner (1196) drops Memento (`:208`).
    - `T9-162`: RODataZone confirms Skel Prisoner (1196) drops Heaven Ring (`:208`).
    - `T9-163`: RODataZone confirms Moonlight Flower (1150) drops Silver Knife of Chastity (`:148`).
    - `T9-164`: RODataZone confirms Clock (1269) drops Elunium (`:40`).
    - `T9-165`: RODataZone confirms Alligator (1271) drops Rough Oridecon (`:7`).
    - `T9-166`: RODataZone confirms Steam Goblin (1280) drops Rough Elunium (`:218`).
    - `T9-167`: RODataZone confirms Marin (1242) drops Poring Hat (`:129`).
    - `T9-168`: RODataZone confirms Antonio (1247) drops Santa Hat (`:17`).
    - `T9-169`: RODataZone confirms Mystcase (1249) drops 2 Carat Diamond (`:154`).
  - **Negative Drop Witnesses (THREAD 32 / T9):**
    - `T9-170`: RODataZone confirms Zombie Prisoner (1197) drops list lacks Empty Potion Bottle (`:259`).
    - `T9-171`: RODataZone confirms Mysteltainn (1203) drops list lacks pure Elunium, listing Rough Elunium (`:155`).
    - `T9-172`: RODataZone confirms Wanderer (242) drops list lacks Yggdrasil Seed, listing Yggdrasil Leaf (`:242`).
    - `T9-173`: RODataZone confirms Magnolia (1138) drops list lacks Yellow Herb (`:123`).
    - `T9-174`: RODataZone confirms Nine Tail (1180) drops list lacks Royal Jelly and Rough Oridecon (`:158`).
  - **Clock Tower Spawns & Semi-Boss (THREAD 26 / T9):**
    - `T9-175`: Ragu Memo `map_clocktower.html` (2004-02-05, jRO) records Clock Tower 1F, 2F, 3F each spawn 1 Clock Tower Manager with 30 min respawn (`1 (30min)`).
    - `T9-176`: Ragu Memo `map_clocktower.html` records Clock Tower 4F spawns 3 Clock Tower Manager with 30 min respawn (`3 (30min)`).
    - `T9-177`: Ragu Memo `map_clocktower.html` records Clock Tower 4F spawns Executioner (エクスキュージョナー) as Semi-Boss (`準Boss`).
    - `T9-178`: Ragu Memo `map_clocktower.html` records Clock Tower 4F spawns 1 Abysmal Knight with 1 hour fixed-point respawn (`1 (定点:1hour)`).
    - `T9-179`: Ragu Memo `map_clocktower.html` records Clock Tower 4F spawns 15 Ride Word in rooms (`部屋 15`).
  - **Novice Equips (THREAD 31 / T9):**
    - `T9-180`: Soujiro Perma-Novice Guide (2003-07-22, iRO) confirms Novice can equip Nagan (1130).
    - `T9-181`: Soujiro Perma-Novice Guide confirms Novice can equip Edge (1132).
    - `T9-182`: Soujiro Perma-Novice Guide confirms Novice can equip Caesar's Sword (1134).
    - `T9-183`: Soujiro Perma-Novice Guide confirms Novice can equip Solar Sword (1136).
    - `T9-184`: Soujiro Perma-Novice Guide confirms Novice can equip Excalibur (1137).
    - `T9-185`: Soujiro Perma-Novice Guide confirms Novice can equip Mysteltainn (1138).
    - `T9-186`: Soujiro Perma-Novice Guide confirms Novice can equip Tirfing (Talefing, 1139).
    - `T9-187`: Soujiro Perma-Novice Guide confirms Novice can equip Byeollungum (1140).
    - `T9-188`: Soujiro Perma-Novice Guide confirms Novice can equip Immaterial Sword (1141).
    - `T9-189`: Soujiro Perma-Novice Guide confirms Novice can equip Swordbreaker (1224).
    - `T9-190`: Soujiro Perma-Novice Guide confirms Novice can equip Mailbreaker (1225).
    - `T9-191`: Soujiro Perma-Novice Guide confirms Novice can equip Weeder Knife (1227).
    - `T9-192`: Soujiro Perma-Novice Guide confirms Novice can equip Combat Knife (1228).
    - `T9-193`: Soujiro Perma-Novice Guide confirms Novice can equip Fortune Sword (Mama's Knife, 1229).
    - `T9-194`: Soujiro Perma-Novice Guide confirms Novice can equip House Auger (1230).
    - `T9-195`: Soujiro Perma-Novice Guide confirms Novice can equip Bazerald (1231).
    - `T9-196`: Soujiro Perma-Novice Guide confirms Novice can equip Assassin Dagger (1232).
    - `T9-197`: Soujiro Perma-Novice Guide confirms Novice can equip Exorcise (1233).
    - `T9-198`: Soujiro Perma-Novice Guide confirms Novice can equip Azoth (1235).
    - `T9-199`: Soujiro Perma-Novice Guide confirms Novice can equip Spike (1523).
    - `T9-200`: Soujiro Perma-Novice Guide confirms Novice can equip Slash (1526).
    - `T9-201`: Soujiro Perma-Novice Guide confirms Novice can equip Quadrille (1527).
    - `T9-202`: Soujiro Perma-Novice Guide confirms Novice can equip Grand Cross (1528).
    - `T9-203`: Soujiro Perma-Novice Guide confirms Phantom Opera Mask (5043) is equippable by Novice as DEF 2 mid-lower headgear.
  - **Item Wearer Third Witnesses (THREAD 31 / T9):**
    - `T9-204`: Ragu Memo `weapon5.html` (2003-12-18, jRO) records War Axe (1306) equippable by Blacksmith and Alchemist (`OnEquip Bth, Alc`).
    - `T9-205`: Ragu Memo `weapon8.html` (2003-12-18, jRO) records Stunner (1522) equippable by Acolyte, Priest, and Monk (`OnEquip Aco, Pri, Mon`).
    - `T9-206`: Ragu Memo `weapon8.html` (2003-12-18, jRO) records Iron Driver (1529) equippable by Priest and Monk (`OnEquip Pri, Mon`).
    - `T9-207`: Ragu Memo `weapon12.html` (2003-12-18, jRO) records Knuckle Duster (1803) equippable by Monk only, excluding Priest (`OnEquip Mon`, `OffEquip Pri`).
    - `T9-208`: Ragu Memo `weapon12.html` (2003-12-18, jRO) records Hora (1805) equippable by Monk only, excluding Priest (`OnEquip Mon`, `OffEquip Pri`).
    - `T9-209`: Ragu Memo `weapon12.html` (2003-12-18, jRO) records Fist (1807) equippable by Monk only, excluding Priest (`OnEquip Mon`, `OffEquip Pri`).
    - `T9-210`: Ragu Memo `armor5.html` (2003-12-12, jRO) records Prontera Army Cap (2261) excludes Archer and Hunter (`OffEquip Arc, Hun`).
    - `T9-211`: Ragu Memo `armor5.html` (2003-12-12, jRO) records Jewel Crown (5002) excludes Thief tree (`OffEquip Thf, Asn, Rog`).
    - `T9-212`: Ragu Memo `armor5.html` (2003-12-12, jRO) records Cross Hairband (5036) excludes Novice (`OffEquip Nov`).
    - `T9-213`: Ragu Memo `armor5.html` (2003-12-12, jRO) records Deviruchi Hat (Evil Wing Hairband, 5038) excludes Novice (`OffEquip Nov`).
    - `T9-214`: Ragu Memo `turtle_armor.html` (2003-08-21, jRO) records Miner's Helmet (5031) was equippable by All jobs including Novice upon Turtle Island release (`OnEquip Nov, Swd, Thf, Mag, Aco, Arc, Mch...`).
    - `T9-215`: Ragu Memo `turtle_armor.html` (2003-08-21, jRO) records Phantom Mask (5043) was equippable by All jobs including Novice upon Turtle Island release (`OnEquip Nov, Swd, Thf, Mag, Aco, Arc, Mch...`).

### T10: Novice Training Ground (Thread 6)
- **Status:** partly
- **Rows:** `T10-021` through `T10-022`
  - `T10-021`: Mistiknoname3k's Assassin Guide (GameFAQs 20164, 2003-12-13, iRO) records beginning Novice equipment as 1 Knife and 1 Cotton Shirt (`This is the equipment/items/zenny they give you at the beginning:\n1 Knife\n1 Cotton Shirt`).
  - `T10-022`: Mistiknoname3k's Assassin Guide records the training grounds final test determines best class, teleports thief to Morroc with extra zenny, or allows picking original class (`At the end of the training grounds you will take a test which will determine which class suits you the best. If you end up with them recommending you to be a thief you will be teleported to morroc and given extra zenny, but if you get stuck with another class tell them you want to go with your original class which is a thief and tell them to teleport you to Morroc`).

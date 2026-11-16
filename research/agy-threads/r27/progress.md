# Round 27 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | Job quests & early progression (thread 5) | partly (Hunter job quest test requirements after Turtle Island update: receptionist personality quiz, Demon Hunter item combos 1/2/3, 200 silver arrows, 4 Job Change Monster targets avoiding decoys, 3-minute escape switch, and necklace reward confirmed in F0rever Zer0 FAQ 27633) | T1-069 – T1-076 |
| T4 | Skills & mechanics reconstruction (threads 22, 34) | partly (Frost Diver freezing duration progression 3s to 30s scaling by 3s/level, 0.8s cast time, and target MDEF reduction confirmed in official iRO Mage Skills guide and SoulEclipse Skills Guide 30694) | T4-033 – T4-036 |
| T9 | Small, independent (threads 10, 22, 34) | partly (Card implementation status: Zombie +20-25% HP recovery, Orc Lord 30% melee reflect, Maya 30% magic reflect, Archer Skeleton NYI, Doppelganger NYI; Item effects implemented: Bloody Axe STR+10/speed+10%, Light Epsilon [Holy]/Heal Lv1; Client effects NYI in jRO: Great Axe Stun+10%, Eddga infinite endure, Ghostring ghost property/recovery penalty; Monster skills: Metamorphosis Fabre>Pupa>Creamy, Hallucination random damage numbers/screen distortion, Harden/Keeping high DEF/immobile; WoE Castle mechanics: agit 20 economy loss on defense fall, jAthena resetcastledata2 5 economy loss on Emperium break, daily master room treasure boxes dependent on economy, jAthena d6aa96f daily box 1 formula 2+(economy+5)/10, steward commercial investment 5000z base formula) | T9-348 – T9-368 |

Total: 33 rows across 3 tasks.
Checker result (`check.txt`): 33 PASS, 0 WARN, 0 FAIL, 0 NONE of 33 rows verified with `python3 ../check_rows.py --pages r27/pages r27/rows/*.jsonl`.

## Task Details and Row Mapping

### T1: Job Quests & Early Progression (Thread 5)
- **Status:** partly
- **Rows:** `T1-069` through `T1-076`
  - `T1-069`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records that after the Turtle Island update, the receptionist lady gives a personality quiz like the Training Grounds to change into a Hunter (source: `gamefaqs_hunter_27633.html`, value: `personality quiz`).
  - `T1-070`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records Demon Hunter item combination 1 as 9 Yellow Herbs, 9 Shells, and 9 Worm Peelings (source: `gamefaqs_hunter_27633.html`, value: `9 Yellow Herbs\n9 Shells\n9 Worm Peelings`).
  - `T1-071`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records Demon Hunter item combination 2 as 3 Bill of Birds, 4 Skel Bones, and 5 Green Herbs (source: `gamefaqs_hunter_27633.html`, value: `3 Bill of Birds\n4 Skel Bones\n5 Green Herbs`).
  - `T1-072`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records Demon Hunter item combination 3 as 5 White Herbs, 5 Wooden Blocks, and 5 Claw of Desert Wolf (source: `gamefaqs_hunter_27633.html`, value: `5 White Herbs\n5 Wooden Blocks\n5 Claw of Desert Wolf`).
  - `T1-073`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records that the Master Hunter gives 200 silver arrows before sending the player to the test waiting room (source: `gamefaqs_hunter_27633.html`, value: `200 silver arrows`).
  - `T1-074`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records that the Hunter test room requires killing 4 Job Change Monsters (Poring, Zombie, Archer Skeleton, Mummy) while avoiding decoy names (source: `gamefaqs_hunter_27633.html`, value: `Job Change Monster`).
  - `T1-075`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records that after killing all 4 monsters, talking to the central switch and choosing escape gives 3 minutes to exit from the opposite side (source: `gamefaqs_hunter_27633.html`, value: `3 minutes to exit the map`).
  - `T1-076`: F0rever Zer0 FAQ v1.1 (2003-11-02, iRO) records that completing the test awards a necklace to be given to the receptionist lady to become a Hunter (source: `gamefaqs_hunter_27633.html`, value: `recieve a necklace`).

### T4: Skills & Mechanics Reconstruction (Threads 22, 34)
- **Status:** partly
- **Rows:** `T4-033` through `T4-036`
  - `T4-033`: Official iRO Guide (2004-04-06) records Frost Diver freezing time as 3 seconds at Level 1 to 30 seconds at Level 10 scaling by 3 seconds per level (source: `20040406194622_jobmagskill.html`, value: `3sec`).
  - `T4-034`: Official iRO Guide (2004-04-06) records Frost Diver has a 0.8 casting time and target magic defense affects freezing time and chance (source: `20040406194622_jobmagskill.html`, value: `0.8 casting time`).
  - `T4-035`: SoulEclipse Skills Guide v4.0 (2005-02-25, iRO) records Frost Diver Level 1 has a 3 Second Freezing Time and 38% Success Rate (source: `gamefaqs_skills_souleclipse_30694.html`, value: `3  Second Freezing Time`).
  - `T4-036`: SoulEclipse Skills Guide v4.0 (2005-02-25, iRO) records Frost Diver Level 10 has a 30 Second Freezing Time and 65% Success Rate (source: `gamefaqs_skills_souleclipse_30694.html`, value: `30 Second Freezing Time`).

### T9: Small, Independent (Threads 10, 22, 34)
- **Status:** partly
- **Rows:** `T9-348` through `T9-368`
  - `T9-348`: RODataZone cards.js (2004-02-24, iRO) records Zombie Card provides HP Recovery +25% on Foot Gear (source: `20040224122443_items_cards.js`, value: `HP Recovery +25%`).
  - `T9-349`: RODataZone cards.js (2004-02-24, iRO) records Maya Card gives 30% chance to reflect magic damage back on caster on Shield (source: `20040224122443_items_cards.js`, value: `30% chance to Reflect magic damage back on caster`).
  - `T9-350`: RODataZone cards.js (2004-02-24, iRO) records Archer Skeleton Card as not yet implemented for 10% more damage on Flying monsters (source: `20040224122443_items_cards.js`, value: `not yet implemented`).
  - `T9-351`: RODataZone cards.js (2004-02-24, iRO) records Doppelganger Card as not yet implemented for Increased Attack Speed (source: `20040224122443_items_cards.js`, value: `not yet implemented`).
  - `T9-352`: Ragu Memo card.html (2004-02-06, jRO) records Zombie Card provides HP recovery speed + 20% (source: `ragmemo_card_20040206.html`, value: `HP回復速度 + 20%`).
  - `T9-353`: Ragu Memo card.html (2004-02-06, jRO) records Orc Lord Card reflects 30% of physical attack damage (source: `ragmemo_card_20040206.html`, value: `物理攻撃によるダメージの30%を反射する`).
  - `T9-354`: Ragu Memo card.html (2004-02-06, jRO) records Maya Card reflects magic attacks with 30% chance (source: `ragmemo_card_20040206.html`, value: `魔法攻撃を30%の確率で反射する`).
  - `T9-355`: jAthena item_db.txt records Orc Lord Card bonus script as returning 30% short weapon damage (source: `data:refs/jathena/db/item_db.txt`, line 1115, value: `bonus bShortWeaponDamageReturn,30;`).
  - `T9-356`: Ragu Memo weapon5.html (2003-12-18, jRO) records Bloody Axe as providing STR + 10 and Movement Speed + 10% (source: `ragmemo_weapon5_20031218.html`, value: `移動速度 + 10%`).
  - `T9-357`: Ragu Memo weapon5.html (2003-12-18, jRO) records Light Epsilon as Holy Property and enabling Heal Level 1 (source: `ragmemo_weapon5_20031218.html`, value: `ヒールLv1使用可能`).
  - `T9-358`: Ragu Memo weapon5.html (2003-12-18, jRO) records Great Axe Stun + 10% under class NoAbility indicating an unimplemented effect (source: `ragmemo_weapon5_20031218.html`, value: `スタン + 10%`).
  - `T9-359`: Ragu Memo card.html (2004-02-06, jRO) records Eddga Card infinite endure under class NoAbility (source: `ragmemo_card_20040206.html`, value: `無限インデュア`).
  - `T9-360`: Ragu Memo card.html (2004-02-06, jRO) records Ghostring Card ghost property and HP recovery penalty under class NoAbility (source: `ragmemo_card_20040206.html`, value: `念属性付与`).
  - `T9-361`: SoulEclipse Skills Guide v4.3 (2005-06-06, iRO) records Metamorphosis as allowing Fabre to transform into Pupa and Creamy, and eggs to hatch (source: `gamefaqs_skills_souleclipse_30694.html`, value: `Fabre &gt; Pupa &gt; Creamy`).
  - `T9-362`: SoulEclipse Skills Guide v4.3 (2005-06-06, iRO) records Hallucination causes the target to see random numbers instead of real damage and distorts the screen (source: `gamefaqs_skills_souleclipse_30694.html`, value: `random numbers instead of the real damage`).
  - `T9-363`: SoulEclipse Skills Guide v4.3 (2005-06-06, iRO) records Harden as increasing defense significantly while preventing monster movement or attacks (source: `gamefaqs_skills_souleclipse_30694.html`, value: `increase their defense rather significantly`).
  - `T9-364`: Anthwerp Guild FAQ v2.5 (2004-12-29, pRO) records that when an agit defense falls during siege, the agit loses 20 points of economy (source: `gamefaqs_guild_anthwerp_25945.html`, value: `20 points of your agit's economy`).
  - `T9-365`: Anthwerp Guild FAQ v2.5 (2004-12-29, pRO) records that guild treasure in the master room is retrieved daily with items dependent on location and guild economy (source: `gamefaqs_guild_anthwerp_25945.html`, value: `retrieved\n   everyday`).
  - `T9-366`: jAthena (rev d6aa96f, 2004-05-16) tbox.txt calculates daily treasure box 1 spawns as 2 + (economy + 5)/10 capped at 12 (source: `git:d6aa96f:script/npc/gvg/test/tbox.txt`, line 198, value: `2+(getcastledata("prtg_cas04.gat",2)+5)/10`).
  - `T9-367`: jAthena ev_agit_common.txt resetcastledata2 deducts 5 points from economy upon Emperium destruction (source: `data:refs/jathena/script/npc/gvg/ev_agit_common.txt`, line 120, value: `GetCastleData(getarg(0),2)-5`).
  - `T9-368`: jAthena ev_agit_common.txt calculates investment cost as 5000 Zeny at base economy with 0 prior daily investments (source: `data:refs/jathena/script/npc/gvg/ev_agit_common.txt`, line 319, value: `5000*(1+(1+@tmp)*@tmp/2)*(1+3*@TriggerE)`).

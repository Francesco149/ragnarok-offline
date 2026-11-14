# Round 15 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 23, 29, 31, R6) | partly (Hats client ships: 18 hats answered with makers, maps, cells, and ingredient costs from RODataZone quests.js and Ragu Memo trade.html, plus Bongun Hat drop, Kafra free storage/warp/cart tickets, and Kafra payroll from later sources; 43 item stat differences from RODataZone items/*.js documented; remaining: Baphomet doll 5 Panacea vs 2 Royal Jelly and Merchant letter 100z vs 500z from pre-2004-05 source, Clock Tower random warps, Unknown Skill !! accounts, monster skill cast rates, ground skills seen twice) | T9-055 – T9-089 |
| T1 | The 2-1 job tests: dating and sharpening (threads 5, 8) | partly (Knight test details answered: Sir Andrew's two 6-item random sets of 5 items, Sir James Syracuse's 8 questions and answers, Sir Grey penalty points scoring with >10 fail and +15 wealth instant fail, Sir Windsor arena 3 stages at 180s each, Sir Edmond patience timer 270s in jAthena vs 300s in rAthena; Blacksmith test differences answered: pass marks eAthena 100/90/80 vs jAthena 90/80/100, safe refine +5 vs +6, Arbalest Trunk vs Eye Patch, Tilpitz payon.gat 214,79 vs 151,39; Assassin test differences answered: moc_fild16 door mod0631 into Kai's room vs eAthena 1.0 into hall, Nameless One quiz pass score >= 90, Barcardi 6 targets and 3m timer, Guild Master maze and Job 50 Jur [3] reward; remaining: Hunter Payon palace map and interview answers, Priest undead kill vs bypass in early 2004, Wizard fire room 1m vs 2m) | T1-028 – T1-041 |
| T10 | The novice training ground outside Japan (thread 6) | partly (Usher subsidy displayed 50z confirmed from RODataZone quests.js matching 817 dialogue; fresh character Knife and Cotton Shirt starting equipment confirmed; remaining: Leo Von Frisch quiz questions vs mod0631, end gift rooms / job subsidies player accounts, Fabre forest knockout cell) | T10-019 – T10-020 |

Total: 51 rows across 3 tasks.
Checker result (`check.txt`): 51 PASS, 0 WARN, 0 FAIL, 0 NONE of 51 rows verified with `python3 research/check_rows.py`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 23, 29, 31, R6)
- **Status:** partly
- **Rows:** `T9-055` through `T9-089`
  - **Makers of Hats Client Ships (THREAD 31):**
    - `T9-055`: Deviruchi Hat (5038) crafted by Young Man (`payon_in01 122/41`) for 600 Little Evil Horn and 40 Talon of Gryphon (RODataZone `quests.js:78`).
    - `T9-056`: Phantom of Opera / Opera Ghost Mask (5043) crafted by Traveler (`comodo 273/137`) for 1 Opera Masque and 50 Horrendous Hair (`quests.js:80`).
    - `T9-057`: Money Loser's Grief (5021) crafted by Traveler (`comodo 273/137`) for 1 Slotted Circlet, 1 Gold, 20 Steel, 80 Feather, and 800 Sticky Mucus (`quests.js:81`).
    - `T9-058`: Parcel Hat (5023) crafted by Traveler (`comodo 273/137`) for 150 Fabric, 100 Resin, and 1 Cobaltblue Dyestuffs (`quests.js:82`).
    - `T9-059`: Miner's Helmet (5031) crafted by Traveler (`comodo 273/137`) for 1 Safety Helmet, 1 Candle, 1 Crystal Mirror, and 25 Steel (`quests.js:83`).
    - `T9-060`: Blue Bandana (5052) crafted by Hair Ornament Girl (`comodo 228/159`) for 1 Bandana, 1 Cobaltblue Dyestuffs, and 300 Anolian Skin (`quests.js:84`).
    - `T9-061`: Striped Bandana (5049) crafted by Hair Ornament Girl (`comodo 228/159`) for 1500 Worn-out Prison Uniform (`quests.js:85`).
    - `T9-062`: Lightbulb Hairband (5034) crafted by Hair Ornament Girl (`comodo 228/159`) for 1 Slotted Circlet and 20 Glass Bead (`quests.js:86`).
    - `T9-063`: Cross Hairband (5036) crafted by Hair Ornament Girl (`comodo 228/159`) for 1 Rosary and 500 Destroyed Armor (`quests.js:87`).
    - `T9-064`: Cake Hat (5024) crafted by Vending Machine (`xmas 114/296`) for 10 Candy, 5 Candy Cane, 15 Well-Baked Cookie, 20 Piece of Cake, and 10 Steel (`quests.js:88`).
    - `T9-065`: Candle (5028) crafted by Vending Machine (`xmas 114/296`) for 1 Bomb Wick, 50 Matchstick, and 100 Royal Jelly (`quests.js:89`).
    - `T9-066`: Chef's Hat (5026) crafted by Vending Machine (`xmas 114/296`) for 120 Piece of Cake, 1 White Dyestuffs, 300 Feather, and 450 Dragon Scale (`quests.js:90`).
    - `T9-067`: Innocence of Maiden / Blush (5040) crafted by Vending Machine (`xmas 114/296`) for 100 Alice's Apron (`quests.js:91`).
    - `T9-068`: Spotted Eggshell / Rainbow Eggshell (5039) crafted by Vending Machine (`xmas 114/296`) for 1 Egg Shell, 1 Cobaltblue Dyestuffs, and 50 Claw of Desert Wolf (`quests.js:92`).
    - `T9-069`: Mysterious Apple Hat / Wonder Nutshell (5050) crafted by Vending Machine (`xmas 114/296`) for 1 Nut Shell and 500 Wing of Dragonfly (`quests.js:93`).
    - `T9-070`: Spore Hat (5029) crafted by Vending Machine (`xmas 114/296`) for 850 Poison Spore, 300 Burnt Tree, and 1 Tongue (`quests.js:94`).
    - `T9-071`: Smokie Hat / Raccon Hat (5033) crafted by Vending Machine (`xmas 114/296`) for 1 Kitty Band, 20 Dragon Scale, 200 Tough Scalelike Stem, and 300 Sea-Otter Fur (`quests.js:95`).
    - `T9-072`: Angel Helm (5025) crafted by Young Man (`payon_in01 122/41`) for 1 Slot Helm, 1 Angel Wing, and 5 Fang of Hatii (`quests.js:79`).
    - `T9-073`: Ragu Memo (`trade.html`, 2004-02-22) confirms jRO Lutie vending machine trade for Smokie Hat (たぬき帽).
    - `T9-074`: Ragu Memo (`trade.html`, 2004-02-22) confirms jRO Comodo Traveler trade for Miner's Helmet (鉱山ヘルメット).
    - `T9-075`: Ragu Memo (`trade.html`, 2004-02-22) confirms jRO Comodo Hair Ornament Girl trade for Blue Bandana (青いバンダナ).
    - `T9-076`: Bongun Hat (5046) defined in rAthena item_db_equip (Grade D).
    - `T9-077`: Kafra Free Storage Ticket (7059) and Free Transport Ticket (7060) awarded in Novice Training Ground (Grade D).
    - `T9-078`: Kafra Free Cart Rental Ticket (7061) awarded in Novice Training Ground (Grade D).
    - `T9-079`: Kafra Payroll (7056) quest reward in WoE TE missions (Grade D).
  - **Item Numbers (THREAD R6):**
    - `T9-080`: Axe (1301) weight 80 and attack 38 in RODataZone vs 120 and 35 in mod0631.
    - `T9-081`: Battle Axe (1351) attack 80 in RODataZone vs 75 in mod0631.
    - `T9-082`: Buster (1357) attack 155 in RODataZone vs 140 in mod0631.
    - `T9-083`: Two-Handed Axe (1360) attack 185 in RODataZone vs 170 in mod0631.
    - `T9-084`: Infiltrator (1261) attack 100 in RODataZone vs 140 in mod0631.
    - `T9-085`: Gungnir (1413) attack 90 in RODataZone vs 120 in mod0631.
    - `T9-086`: Sword Mace (1516) attack 120 in RODataZone vs 130 in mod0631.
    - `T9-087`: Nurse Cap (2277) defense 2 in RODataZone vs 1 in mod0631.
    - `T9-088`: Pirate Bandana (2287) defense 3 in RODataZone vs 1 in mod0631.
    - `T9-089`: Gold (969) weight 50 in RODataZone vs 20 in mod0631.

### T1: The 2-1 Job Tests (Threads 5, 8)
- **Status:** partly
- **Rows:** `T1-028` through `T1-041`
  - **Knight (`job_knt`):**
    - `T1-028`: Sir Andrew's item test consists of two randomized sets of 6 items (5 each) in rAthena `knight.txt:576-581` (Set 1: Elder Pixie Moustache, Red Bat Wing, Orcish Voucher, Moth Dust, Reptile Tongue, Mane; Set 2: Bug Leg, Heart of Mermaid, Maneater Blossom, Clam Flesh, Old Frying Pan, Snail Shell).
    - `T1-029`: Sir James Syracuse's test questions in rAthena `knight.txt:878` (prerequisites for Bowling Bash, Brandish Spear, and attacking Ghost Nightmare with Zephyrus).
    - `T1-030`: Sir Grey's scoring: score > 10 penalty points fails, prioritizing wealth/money gives +15 instant failure in rAthena `knight.txt:2394`.
    - `T1-031`: Sir Windsor's arena battle trial consists of 3 stages with a 180s (3-minute) timer per stage in jAthena commit `6332410:1735`.
    - `T1-032`: Sir Edmond's patience test timer is 270s (4.5 minutes) in jAthena commit `6332410:859`, compared to 300s (5 minutes) in rAthena.
  - **Blacksmith:**
    - `T1-033`: eAthena 1.0 `blacksmith.txt:464, 489, 1410` pass marks: Geshupenschte 100 first try, 90 retry; Mitmayer 80.
    - `T1-034`: jAthena `npc_job_10blacksmith.txt:330, 807` pass marks: Geshupenschte 90 first try, 80 retry; Mitmayer 100.
    - `T1-035`: Safe refine for level 3 weapons: eAthena 1.0 counts +5 as correct, while jAthena counts +6.
    - `T1-036`: Arbalest order materials: eAthena 1.0 consumes 1 Trunk (1019), while jAthena requires 1 Eye Patch (2216).
    - `T1-037`: Tilpitz location: jAthena places Tilpitz at `payon.gat 151,39`, while eAthena 1.0 places him at `payon.gat 214,79`.
  - **Assassin:**
    - `T1-038`: Entrance door on `moc_fild16`: mod0631 warps directly into Kai's room (`in_moc_16 20,74`), while eAthena 1.0 warps into entrance hall (`in_moc_16 18,11`).
    - `T1-039`: Nameless One's quiz requires score >= 90 (9/10 right) in eAthena 1.0 `assassin.txt:962`.
    - `T1-040`: Barcardi's trial requires eliminating 6 monsters named 'Job Change Target' in eAthena 1.0 `assassin.txt:1044`.
    - `T1-041`: Assassin Guild Master maze awards Jur [3] (at Job 50) and Necklace of Oblivion in eAthena 1.0 `assassin.txt:1568`.

### T10: Novice Training Ground Outside Japan (Thread 6)
- **Status:** partly
- **Rows:** `T10-019` through `T10-020`
  - `T10-019`: Usher subsidy displayed 50z upon agreeing to the course in RODataZone `quests.js:143`, confirming iRO screen text against 150z coded in emulators.
  - `T10-020`: Starting equipment created and equipped in `crates/ro-game/src/game.rs:691`: Knife and Cotton Shirt worn from creation as in mod0631.

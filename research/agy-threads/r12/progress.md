# Round 12 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | The 2-1 job tests: dating and sharpening them (threads 5, 8) | answered | T1-001 – T1-025 |
| T4 | Ailments and their resistance (thread 21) | answered | T4-001 – T4-019 |
| T9 | Small, independent (threads 7, 14, 17, 24) | partly (Acolyte Yosuke/Rubalkabara coordinates, Merchant serial numbers, Thief mushroom points, Thief pay & mushroom deletion, Merchant wrong parcel 1083 vs 1091, Alberta doll child trades, prt_in 280-285 castle warps answered; Clock Tower random warps, monster skill cast rates, Unknown Skill !! accounts, ground skills seen twice remaining) | T9-001 – T9-019 |

Total: 63 rows across 3 tasks.
Checker result (`check.txt`): Verified with `python3 research/check_rows.py --fetch`.

## Task Details and Row Mapping

### T1: The 2-1 Job Tests (Threads 5, 8)
- **Status:** answered
- **Rows:** `T1-001` through `T1-025`
  - **Knight Test Walkthrough & History:**
    - PsychoBattousai Swordsman/Knight Guide (FAQ 21546 v1.3, 2003-02-25) confirms iRO had no quest in February 2003, requiring only a 2,500z fee to the "Knight" NPC in northwest Prontera (`T1-001`).
    - Sharonov Guide (news=274, 2009-01-28) provides the complete test breakdown:
      - Sir Andrew requires one of two 6-item sets of 5 items each (Set 1: Bug Leg, Clam Flesh, Maneater Blossom, Old Frying Pan, Snail Shell, Heart of Mermaid; Set 2: Elder Pixie's Moustache, Orcish Voucher, Mane, Moth Dust, Reptile Tongue, Wing of Red Bat), or sets excluding one item type (`T1-002`, `T1-003`).
      - Sir Siracuse / James questions: Flamberge unaffected by Twohand Quicken, Provoke Lv 10 not required for Bowling Bash, Spear Boomerang not necessary for Brandish Spear, Zephyrus can attack Nightmare, 80%, hunting area, protect everyone, honor (`T1-004`, `T1-005`).
      - Battle test consists of 3 rounds; Round 3 features Goblins and a self-destructing Steamrider (`T1-006`).
      - Sir Edmund administers Self Control test (wait in passive monster map without attacking) (`T1-007`).
      - Sir Gray administers Personality Quiz where friendly and altruistic answers pass (`T1-008`).
      - Job change awards 7 Awakening Potions regardless of job level (`T1-009`).
  - **Blacksmith Test Walkthrough:**
    - Sharonov Guide (news=278, 2009-03-15):
      - Quiz question 8 confirms safe refine limit for Level 3 weapons is up to + 5 (`T1-010`).
      - Delivery item set 4 requires 1 Iron Ore, 1 Eye Bandage, 2 Blue Gemstone, and 1 Arbalest, confirming Eye Bandage over Trunk (`T1-011`).
      - Delivery recipients: Wickebine in Morroc at `27,112` (`T1-012`), Baisulist in Geffen at `46,164` (`T1-013`), Talpiz in Payon at `214,79` (`T1-014`), and Bismark in Hugel/Comodo (`T1-015`).
      - Mitmayer questions confirm Geffen Tool Dealer is located 8 o'clock direction from the town square (`T1-016`).
  - **Assassin Test Walkthrough:**
    - Sharonov Guide (news=279, 2009-06-19):
      - Combat test requires killing 6 "Job Change Target" monsters (3 Poring, 1 Drops, 1 Lunatic, 1 Poporing) within 3 minutes and 5 seconds (`T1-017`).
      - Job 50 Assassins choose one weapon reward from Jur [3], Katar [2], Main Gauche [4], or Gladius [3] (`T1-018`).
      - Below Job 50 Assassins receive a random weapon from Main Gauche [3], Jur [2], Stiletto [2], Knife [3], or Katar [1] (`T1-019`).
      - Test awards a Necklace of Oblivion upon completion as proof of passing (`T1-020`).
      - Quiz question 2 confirms Martin drops a slotted Jur (`T1-021`).
  - **Priest Test Walkthrough:**
    - Sharonov Guide (news=277, 2009-07-26):
      - Pilgrimage to 3 ascetics comes first (Father Rubalkabara at `prt_fild03 365,259`, Mother Marthilda at `moc_fild07 36,354`, Father Yosuke at `prt_fild00 208,218`) and is skipped if Job Level 50 (`T1-022`, `T1-023`).
  - **Hunter Test Walkthrough:**
    - Sharonov Guide (news=276, 2009-11-13):
      - Demon Hunter requires one of 7 item sets, including Set 1: 5 Claw of Desert Wolf, 5 Trunk, 3 White Herb (`T1-024`).
      - Destination Hunter NPC located inside Payon cottage `payon_in03 131,7` (`T1-025`).

### T4: Ailments and Resistance Formulas (Thread 21)
- **Status:** answered
- **Rows:** `T4-001` through `T4-019`
  - Maruko Hybrid Knight Guide (FAQ 31808, v1.1 2004-08-08 & v1.0 2004-07-13):
    - Stun success rate: `BaseChance X (97-VIT-(LUK/3))/100 %` (`T4-001`).
    - Stun duration: `3 X (97-VIT-(LUK/3)) /100) seconds` (`T4-002`).
    - Frozen success rate: `BaseChance X (97 - MDEF - (LUK/3))/100 %` (`T4-003`).
    - Frozen duration: `BaseDuration X (1-MDEF/100) or until hit and damaged` (`T4-004`).
    - Stone Curse success rate: `BaseChance X (97-INT-(LUK/3))/100 %` (`T4-005`).
    - Stone Curse duration: `17-MDEF/20 seconds` (`T4-006`).
    - Stone Curse HP drain: loses 1% Max HP every 5 seconds down to 25% MaxHP (`T4-007`).
    - Poison success rate: `BaseChance X (97-VIT-(LUK/5))/100 %` (`T4-008`).
    - Poison HP drain: loses 3% Max HP every 3 seconds down to 25% MaxHP (`T4-009`).
    - Poison duration: 10-30 seconds vs monsters, 5-60 seconds vs players (`T4-010`).
    - Blind duration: `30 - BaseLevel/10 - Int/15 seconds (15 seconds minimum)` (`T4-011`).
    - Silence duration: `BaseDuration X (100-VIT)/100 seconds` (`T4-012`).
    - Sleep success rate: `BaseChance X (97-INT-(LUK/3))/100 %` (`T4-013`).
    - Sleep duration: `~30 seconds (or until hit and damaged)` (`T4-014`).
    - VIT DEF formula: `[VIT X 0.5] + rnd([VIT X 0.3], max([VIT X 0.3],[VIT^2/150]-1))` (`T4-015`).
    - Upgraded equipment DEF: converts to solid DEF by multiplying upgrade level by 2/3 (`.666666667`) (`T4-016`).
    - Cast time formula: `ActualCastTime = BaseCastTime X EquipmentMods X max(1 - Dex/150,0)` (`T4-017`).
    - Safety Wall vs Pneuma monster attack range: normal monster attack under 4 tiles range is melee (blocked by Safety Wall) (`T4-018`); at least 4 tiles range is ranged (blocked by Pneuma) (`T4-019`).

### T9: Small, Independent (Threads 7, 14, 17, 24)
- **Status:** partly
- **Rows:** `T9-001` through `T9-019`
  - Acolyte ascetics coordinates (Sharonov Guide news=267):
    - Father Yosuke at `prt_fild00 (208, 218)` on the small island, confirming 817's coordinates over mod0631's blocked `208,255` (`T9-001`).
    - Father Rubalkabara at `prt_fild03 (365, 259)` (`T9-002`).
  - Merchant delivery serial numbers (Sharonov Guide news=268):
    - Prontera Former Swordman Association: `2485741` or `2328137` (`T9-003`).
    - Geffen Mage Guild: `2989396` or `2191737` (`T9-004`).
  - Thief test mushroom points and bag deletion:
    - 1 Orange Net Mushroom = 3 points, 1 Orange Gooey Mushroom = 1 point, 25 points minimum (Sharonov news=269) (`T9-005`).
    - mod0631 `conf/npc_job_thief.txt` line 446 awards a flat 100z reward without deleting mushrooms (`T9-006`).
    - 817 `npc/jobs/thief.txt` line 476 deletes up to 99 of each mushroom from inventory upon ceremony (`T9-007`).
  - Merchant wrong parcel:
    - mod0631 `conf/npc_job_merchant.txt` line 694 gives item 1091 (`Merchant_Box_Etc`) (`T9-008`).
    - 817 `npc/jobs/merchant.txt` line 469 gives item 1083 (`Merchant_Box_3`) (`T9-009`).
  - Alberta doll child trade items (mod0631 `conf/npc_event_doll.txt` 2003-12-15):
    - Poring Doll (741) -> Candy (529) x1 (`T9-010`).
    - Chonchon Doll (742) -> Candy Cane (530) x1 (`T9-011`).
    - Rocker Doll (752) -> Well-baked Cookie (532) x7 (`T9-012`).
    - Spore Doll (743) -> Piece of Cake (538) x5 (`T9-013`).
    - Osiris Doll (751) -> Mastela Fruit (522) x2 (`T9-014`).
    - Baphomet Doll (750) -> Royal Jelly (526) x2 (`T9-015`).
    - Smokie Doll (754) -> Hinalle Leaflet (539) x3 (`T9-016`).
    - Yoyo Doll (753) -> Yggdrasil Seed (608) x1 (`T9-017`).
  - Unreachable Prontera Castle exits on `prt_in` at x 280 to 285:
    - mod0631 `conf/npc_warp.txt` lines 609-612 defines 4 warps (`w470`, `w471`, `w472`, `w473`) from `prt_in` at `280,68`, `281,36`, `282,100`, `285,130` leading to castle entrance `prontera.gat 147,287`, where cells 280,68, 281,36, and 282,100 are blocked (type 5) in client `data.grf` (`T9-018`).
    - rAthena `npc/warps/cities/prontera.txt` line 28 disables these four duplicate unreached warps with `//` comments (`T9-019`).

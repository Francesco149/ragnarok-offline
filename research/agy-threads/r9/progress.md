# Round 9 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | The 2-1 job tests: dating and sharpening them (threads 5, 8) | Partly | T1-001 – T1-023 |
| T9 | Small, independent | Partly | T9-001 – T9-027 |
| T10 | The novice training ground outside Japan (thread 6) | Partly | T10-001 – T10-003 |

Total: 53 rows across 3 tasks.
Checker result (`check.txt`): 53 PASS, 0 WARN, 0 FAIL, 0 NONE (verified live with `python3 ../../check_rows.py --fetch`).

## Task Details and Row Mapping

### T1: The 2-1 Job Tests: Dating and Sharpening (Threads 5, 8)
- **Status:** Partly (Blacksmith, Assassin, Priest, Wizard, and Hunter details sharpened across emulator scripts and Aegis conversions; contemporary pre-June 2004 player walkthroughs for Blacksmith/Assassin remain open)
- **Rows:** `T1-001` through `T1-023`
  - **Blacksmith Test:**
    - Pass marks: eAthena 1.0 (`706e78509`) requires 100 on first try, >80 (90+) on retry (`T1-001`), whereas jAthena (`cd53db015`) requires 90 on first try and 80 on retry (`T1-002`).
    - Mitmayer's pass marks: eAthena 1.0 requires >70 (80+) (`T1-003`), whereas jAthena requires 100 (`T1-004`).
    - Safe refine question 8: eAthena 1.0 counts +5 for Level 3 weapon (`T1-005`), whereas jAthena counts +6 (`T1-006`).
    - Tilpitz delivery: eAthena 1.0 requires 8 Iron Ores, 1 Trunk, 2 Blue Gemstones, 1 Arbalest (`T1-007`).
    - Tilpitz standing position: `blacksmith_test.lua` documents jAthena Payon `151,39` vs eAthena `214,79` (rebuilt Payon outside client map) (`T1-008`); jAthena script places Tilpitz (`テルビツ`) at `payon.gat 151,39` waiting for an eye patch pouch (`T1-009`).
  - **Assassin Test:**
    - Nameless One quiz pass mark: eAthena 1.0 requires score $\ge 90$ (`T1-010`).
    - Barcardi test 1: spawns exactly 6 monsters named "Job Change Target" (`T1-011`); hitting any decoy monster triggers immediate failure (`T1-012`).
    - Timer: Barcardi's test rooms use a 3-minute timer with periodic countdown announcements (`T1-013`).
    - Weapon rewards: Guild Master allows Job Level 50 candidates to select from Jur [3], Katar [2], Main Gauche [4], or Gladius [3], while Job Level < 50 receives a random pick among Jur [2], Stiletto [2], or Jamadhar (`T1-014`).
    - Quest completion item: all candidates receive the Necklace of Oblivion (`1008`) (`T1-015`).
    - Entrance doors: eAthena 1.0 places Guildsman Huey at `in_moc_16.gat 19,33` (`T1-016`), whereas mod0631/817 `warps.lua:857` warps `moc_fild16 206,297` directly into Kai's room at `in_moc_16 20,74` (`T1-017`).
  - **Priest Test:**
    - Pilgrimage: eAthena 1.0 skips the pilgrimage entirely for Job Level 50 candidates (`T1-018`).
    - Test 2 timer: Father Peter's 3 trials on `job_prist.gat` allocate a total of 5 minutes (`T1-019`).
    - Weapon rewards: eAthena 1.0 awards Book [3] (`1550`) for Job 40 and Bible [2] (`1551`) for Job 50 (`T1-020`).
  - **Wizard Test:**
    - Door guards timer: eAthena 1.0 sets `TimerWz` to 120,000 ms towards a 180,000 ms timeout, giving exactly 1 minute (60 seconds) to kill the door guards (`T1-021`).
    - Retrying: Raul Expagarus sets savepoint to `gef_tower.gat 106,29` with 0 Zeny / 0 item penalty to retry (`T1-022`).
  - **Hunter Test:**
    - Item sets: rAthena `hunter.txt` official Aegis conversion documents 7 official Demon Hunter item sets (`T1-023`).

### T9: Small, Independent
- **Status:** Partly (bullet items for shop English names and 10 monster skills against `mob_skills.toml` verified; Doomed Swords and Juice recipes documented)
- **Rows:** `T9-001` through `T9-027`
  - **Two shops' English names (Thread 11):**
    - Prontera `48,58` (Carrots, Sweet Potatoes, Pumpkins; mod0631 `野菜商人`): named `Vegetable Gardener#prt` in rAthena (`T9-001`).
    - Juno `205,103` (Full Plate, Greaves, Gemmed Sallet, Repeating Crossbow; mod0631 `王窓商会`): named `Equip Dealer` in eAthena 1.0 (`T9-002`) and `Mr. King's Shop#yuno` in rAthena (`T9-003`).
  - **Monster skills against `data/db/mob_skills.toml` (Thread 23, from unknown13 FAQ 2004-04-15):**
    - Angeling (1096): Safety Wall (`T9-004`), Heal (`T9-005`), Holy Light (`T9-006`).
    - Minorous (1149): Heaven's Drive (`T9-007`), Hammer Fall (`T9-008`).
    - Raydric (1163): Magnum Break (`T9-009`), Maximize Power (`T9-010`).
    - Deviruchi (1109): Jupital Thunder (`T9-011`).
    - Horong (1129): Sight (`T9-012`), Consecutive Attack (`T9-013`).
    - Sohee (1170): Suicide (`T9-014`).
    - Creamy (1018): Teleport (`T9-015`), Sleep Attack (`T9-016`).
    - Khalitzburg (1132): Splash Attack (`T9-017`).
    - Dokebi (1110): Adrenaline Rush (`T9-018`), Mammonite (`T9-019`).
    - Marine Sphere (1142): Self-destruction (`T9-020`).
  - **Quest items, Doomed Swords, and Juice (from blood3d FAQ 2004-09-26):**
    - Banana Juice recipe and location in Payon PVP room (`T9-021`).
    - Legendary Blacksmith location at the north east eye of willow map east of Payon (`pay_fild08`) (`T9-022`).
    - Mysteltain components: 1 Foolishness of the blind, 1 Loki's Whisper, 1 Mother's Nightmare, 1 Young Twig, 1 Emperium (`T9-023`).
    - Grimtooth components: 10 Bloody Edge, 5 Cursed Ruby, 100 Ogre Tooth, 1 Old Hilt, 1 Emperium (`T9-024`).
    - Executioner components: 2 Executioner's Mitten, 10 Bloody Edge, 3 Necklace of Oblivion, 50 Amulet, 1 Emperium (`T9-025`).
    - Counteragent ingredients: 1 Detrimindexta, 1 Alcohol, 1 Empty Bottle, 3000 Zeny (`T9-026`).
    - Mixture ingredients: 1 Karvodailnirol, 1 Alcohol, 1 Empty Bottle, 4000 Zeny (`T9-027`).

### T10: The Novice Training Ground Outside Japan (Thread 6)
- **Status:** Partly (starting equipment, quiz reward scaling, and recommendation town warp / extra zenny confirmed from 2003 iRO guide; Leo Von Frisch exact question text remains unquoted)
- **Rows:** `T10-001` through `T10-003`
  - **Starting gear:** Mistiknoname3k's *Assassin Guide* v1.9 (2003-12-13, iRO) specifies 1 Knife and 1 Cotton Shirt upon entering Midgard (`T10-001`).
  - **Quiz rewards:** Test items awarded scale directly with the number of correct quiz answers (`T10-002`).
  - **Recommendation rewards:** Novice recommended to be a Thief is teleported directly to Morroc and granted bonus starting zenny (`T10-003`).

# Round 7 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | 2-1 job tests: dating and sharpening on international servers (threads 5, 8) | Partly | T1-001 – T1-010 |
| T10 | Novice training ground outside Japan (thread 6) | Partly | T10-001 – T10-007 |
| T9 | Small, independent: Swordman course monsters, Padded Armour, quest skills | Partly | T9-001 – T9-004 |
| T11 | The May-2004 client against ours: packet lengths, 2-2 skills, datagrf diff (thread 25) | Answered | T11-001 – T11-029 |

Total: 50 rows across 4 tasks.
Checker result (`check.txt`): 50 PASS, 0 WARN, 0 FAIL, 0 NONE (verified live with `--fetch`).

## Task Details and Row Mapping

### T1: 2-1 Job Tests: Dating and Sharpening (Threads 5, 8)
- **Status:** Partly (answers the existence/dating and structure for Knight, Assassin, Blacksmith, Wizard, Hunter, and Priest; specific autumn-2004 Knight quiz questions and items remain open)
- **Rows:** `T1-001` through `T1-010`
  - **Knight:**
    - Weggy's FAQ v6.0 (May 17, 2004) confirms iRO had no Knight test; becoming a Knight only required speaking to the Knight at the Culvert Registration building and paying a 2500z fee: `T1-001`
    - Weggy's FAQ v6.1 (May 18, 2004) changelog explicitly notes Gravity had recently added more in-depth quests to 2-1 classes: `T1-003`
  - **Assassin:**
    - Weggy's FAQ v6.1 (May 18, 2004) confirms becoming an Assassin on iRO only required paying 666z to the guildmaster with no test: `T1-002`
  - **Blacksmith:**
    - Weggy's FAQ v6.5 (June 11, 2004) confirms becoming a Blacksmith on iRO was an instant change by speaking to the NPC inside the Geffen shop: `T1-004`
    - Guide_Maker's Forging Merchant/Blacksmith Guide (FAQ 28512 v1.12) dates the introduction of the Blacksmith job change quest to August 31, 2004 ("08/31/04 - Put up a Job change for Blacksmiths section"): `T1-005`
    - Guide_Maker confirms obtaining a "Reciept" item from the Alberta NPC after delivery, matching eAthena 1.0: `T1-006`
  - **Wizard:**
    - Weggy's FAQ v6.3 (May 31, 2004) confirms becoming a Wizard was a direct talk with the Wizard at the top of Geffen tower with no test: `T1-007`
  - **Hunter:**
    - Weggy's FAQ v6.2 (May 25, 2004) answers the location question: the Senior Hunter was at the Payon Archer guild (`payon_in02`): `T1-008`
  - **Priest:**
    - Weggy's FAQ v6.4 (June 7, 2004) dates the official 3-part Priest quest on `job_prist` to before early June 2004 on iRO:
      - Quizzes with Father Mareusis / Peter S. Alberto in the church antechamber, followed by walking a hall of temptations and resisting all temptations: `T1-009`
      - Exorcism stage in rooms where undead monsters spawn that can be run past or killed: `T1-010`

### T10: Novice Training Ground Outside Japan (Thread 6)
- **Status:** Partly (walkthrough, skipping, scoring, and town warps established; Leo's specific question texts remain unquoted in player guides)
- **Rows:** `T10-001` through `T10-007`
  - **Progression & Skipping:**
    - Start point crosses the bridge to the right into the castle: `T10-001`
    - Requesting direct access to the game whisks the player away to a random town: `T10-002`
  - **Rewards & Job Selection:**
    - Completing training grounds awards 100z, or 200z if the final NPCs guess the class correctly: `T10-003`
    - If guessed incorrectly, player chooses from 6 towns (Alberta - Merchant, Geffen - Mage, Izlude - Swordsman, Morroc - Thief, Payon - Archer, Prontera - Acolyte) and is teleported: `T10-004`
  - **Usher Dialogue Discrepancy:**
    - eAthena 817 line 728 dialogue displays "Training Subsidy 50 Zeny": `T10-005`
    - eAthena 817 line 726 script awards 150 Zeny (`set Zeny,Zeny+150;`): `T10-006`
    - mod0631 line 64 script awards 150 Zeny matching its 150z dialogue: `T10-007`

### T9: Small, Independent (Swordman Monsters, Padded Armour, Quest Skills)
- **Status:** Partly (answers course pit monsters and Padded Armour mechanics)
- **Rows:** `T9-001` through `T9-004`
  - **Swordman Course Pit:**
    - Weggy's FAQ v6.0 confirms falling off the Izlude Swordsman ledges lands in a pit with aggressive Fabres and Chonchons: `T9-001`
  - **Padded Armour:**
    - vgmaster88's Swordsman Guide v3.3 (November 14, 2003) confirms the Moving HP Recovery quest required 1 Padded Armor "(which you get to keep)", 500 empty bottles, and a rocker doll: `T9-002`. This explains why 817 and mod0631 require having/wearing the armor but do not delete it from inventory.
  - **Quest Skill Items on iRO:**
    - Attack Weak Point (Fatal Blow): 10 fire arrows, 10 silver arrows, 5 royal jellies, 1 banana juice, 10 tentacles from Leo Von Prisch at Prontera Knight guild: `T9-003`
    - Auto Berserk: 35 powder of butterfly, 10 honey, 10 horrendous mouths, 10 decayed nails: `T9-004`

### T11: The May-2004 Client Against Ours (Thread 25)
- **Status:** Answered
- **Rows:** `T11-001` through `T11-029`
  - **Packet Length Differences:**
    - 30 differences between December 2003 and May 2004 Ragexe tables:
      - 7 spawn/move packets grew by 2 bytes: `0x0078` (52 -> 54), `0x0079` (51 -> 53), `0x007a` (56 -> 58), `0x007b` (58 -> 60), `0x01d8` (52 -> 54), `0x01d9` (51 -> 53), `0x01da` (58 -> 60): `T11-001` – `T11-008`, `T11-010` – `T11-015`
      - The 2-byte field appended is `base_level` (16-bit word, capped at 99 in eAthena 817 `clif.c:543, 627, 855` for aura/display): `T11-016` – `T11-018`
      - `0x014e` (6 bytes in Dec 2003) removed in May 2004: `T11-009`
      - 22 new packets added: `0x01cd`–`0x01d2`, `0x01e1`, and `0x01f1`–`0x01ff`
  - **2-2 Skills in May 2004 `skilldesctable.txt`:**
    - Flame Launcher / endows: weapon is broken on failure: `T11-019`
    - Dispell: consumes 1 Yellow Gemstone and 1 Red Gemstone: `T11-020`
    - Strip Weapon: reduces monster ATK: `T11-021`
    - Strip Shield: reduces monster DEF 15%: `T11-022`
    - Strip Armor: reduces monster VIT 40%: `T11-023`
    - Strip Helm: reduces monster INT 40%: `T11-024`
    - Reflect Shield: reflects physical attack only, requires shield: `T11-025`
    - Defender: slows movement and attack while reducing long-distance damage: `T11-026`
    - Grand Cross: consumes 20% remaining HP: `T11-027`
    - Chemical Protection Weapon: consumes 1 Coating Bottle: `T11-028`
  - **data.grf Diff Analysis:**
    - Total: 13,378 files in Dec 2003, 15,382 in May 2004 (13,244 identical, 134 changed, 2,004 added): `T11-029`
    - Changed (134): 22 maps (aldebaran, cmd_fild02, gef_fild13, gl_in01, in_moc_16, moc_fild02, payon, xmas), 15 tables (item/card/skill tables, clientinfo, msgstringtable, indoorrswtable), 33 sprites/graphics, 53 sounds, 11 jpgs.
    - Added (2004): 126 maps, 24 tables, 1654 sprites/graphics, 67 rsm 3D models, 35 sounds, 43 str effect files, 33 jpgs, 21 gr2 animation files, 1 ezv file.

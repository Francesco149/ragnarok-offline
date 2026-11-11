# Round 5 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | The 2-1 job tests on the maps of the Turtle Island patch (threads 5, 8) | Answered | T1-001 – T1-027 |
| T2 | A client that knows the 2-2 skills (thread 25) | Answered | T2-001 – T2-010 |
| T10 | The novice training ground outside Japan (thread 6) | Answered | T10-001 – T10-018 |

Total: 55 rows across 3 tasks.
Checker result (`check.txt`): 54 PASS, 0 WARN, 0 FAIL, 1 NONE (verified live with `--fetch`).

## Task Details and Row Mapping

### T1: The 2-1 Job Tests on the Maps of the Turtle Island Patch (Threads 5, 8)
- **Status:** Answered
- **Rows:** `T1-001` through `T1-027`
  - **Hunter on `job_hunte`:**
    - Hidden Hunter NPC inside bamboo trees on `pay_fild08` cliff warps candidate to `in_hunter`: `T1-001`
    - Personality quiz at front desk: `T1-001`
    - Item collection test at reception desk B (Combo 1: 9 Yellow Herbs, 9 Shells, 9 Worm Peelings): `T1-002`
    - Master Hunter in Payon central palace or archer village supplies 200 Silver Arrows: `T1-003`
    - Waiting room queue admits only one candidate at a time into `job_hunte`: `T1-004`
    - Combat arena on `job_hunte`: kill 4 "Job Change Monster" targets (Poring, Zombie, Archer Skeleton, Mummy) without triggering traps: `T1-005`
    - Center switch gives "escape" prompt with a 3-minute timer to cross to the north exit (12:00): `T1-006`
    - mod0631 warps on `job_hunte`: entrance at `89,68` (6:00), center switch at `89,104`, exit at `89,139` (12:00)
  - **Knight on `job_knt`:**
    - Windsor Benedict at `job_knt 89,106`: `T1-008`
    - Waiting room chat queue into Arena 1 (`job_knt 43,147`): `T1-009`
    - Progression through Arena 2 (`job_knt 43,52`) and Arena 3 (`job_knt 143,152`): `T1-010`, `T1-011`
    - Dame Amy's patience/restraint test in Arena 4 (`job_knt 143,57`): `T1-012`
    - mod0631 warps on `job_knt`: the four warp tiles (`22,145`, `65,145`, `122,151`, `165,151`) are the boundary gates of Arena 1 (`22,126` to `65,165`) and Arena 3 (`122,132` to `165,171`), which warp failing candidates back to `prt_in.gat 80,100`: `T1-013`
  - **Priest on `job_prist`:**
    - Warped to exorcism and temptation trial on `job_prist 24,180`: `T1-014`
    - Exit warp at `job_prist 98,105` leading to `prt_church 15,36`: `T1-015`
    - Sister Cecile completion NPC at `job_prist 168,180` at the end of the temptation hallway (`168,17` to `168,180`): `T1-016`
    - mod0631 warps on `job_prist`: `24,109` warps candidate to `168,17` (transition from zombie corridor to temptation hallway)
  - **Wizard on `job_wiz`:**
    - Waiting room queue of 20 people admitting 1 candidate at a time to `job_wiz 114,169`: `T1-017`
    - Fire room guard phase timer is exactly 60 seconds (1 minute): `T1-018`
    - Failure penalty: teleported and saved outside Geffen tower: `T1-019`
    - Combat skip: failing 5-6 times unlocks NPC option to skip the combat test with a worn out scroll: `T1-020`
    - Player quote of examiner quiz questions (e.g. "Which spell is not a bolt type? Earth"): `T1-021`
  - **Assassin and Blacksmith:**
    - Assassin quest takes place entirely in `in_moc_16` (Morroc Assassin Guild); no special `job_` map: `T1-022`
    - Blacksmith quest takes place in `geffen_in` (Geffen Blacksmith Guild) and standard town delivery errands; no special `job_` map: `T1-023`
  - **Breeders:**
    - Peco Peco Breeder stands at `prontera 55,350` (walkable type 0 cell) and charges 2500z: `T1-024`, `T1-025`
    - Falcon Breeder stands in Hunter Guild at `in_hunter 134,101` (817 walkable type 0 cell) / `146,99` (mod0631 behind desk) and charges 2500z: `T1-007`, `T1-026`, `T1-027`

### T2: A Client that Knows the 2-2 Skills (Thread 25)
- **Status:** Answered
- **Rows:** `T2-001` through `T2-010`
  - **Wayback Search:** Querying CDX API for `wpatch.euro-ro.net/*` returns 31 HTML and redirect captures, but 0 patch archives (`.rgz` or `.gpf`): `T2-001` (`none` row)
  - **euRO Patch List Entries:**
    - Entry 440: `2004-04-14gdata_lang_g.gpf`: `T2-004`
    - Entry 444: `2004-04-15aRagexe.rgz`: `T2-002`
    - Entry 445: `2004-04-15gdata_scr_g.gpf`: `T2-003`
    - Entry 462: `2004-05-31aRagexe.rgz`: `T2-005`
    - Entry 503: `2004-05-04gdata_yuno_new1_g.gpf`: `T2-006`
  - **Locally Archived International Client and Data (MICO07 DVD):**
    - `Ragexe-2004-05-07.exe` (PE timestamp 2004-05-07T02:17:52Z, extracted from `2004-05-07aRagexe.rgz` on the May 2004 euRO installer `ragSetup_eu`): `T2-008`
    - `2004-05-06gdata_yuno_lang_g.gpf` installed alongside the executable, containing official English 2-2 skill tables (`data\skillnametable.txt`, `data\skilldesctable.txt`) for Crusader, Monk, Sage, Rogue, Bard, Dancer: `T2-007`
    - Full 2-2 packet length definitions present in `Ragexe-2004-05-07.exe`: `0x01cd` (30 bytes), `0x01ce` (6 bytes), `0x01cf` (28 bytes), `0x01d0` (8 bytes), `0x01d1` (14 bytes), `0x01d2` (10 bytes), and `0x01e1` (8 bytes): `T2-009`, `T2-010`

### T10: The Novice Training Ground Outside Japan (Thread 6)
- **Status:** Answered
- **Rows:** `T10-001` through `T10-018`
  - **Leo Von Frisch's Quiz:**
    - Item rewards per question: 1 Banana for Q1 (`T10-001`), 3 Sweet Potatoes for Q3 (`T10-002`), 1 Apple for Q4 (`T10-003`), 1 Meat for Q5 (`T10-004`), 1 Milk for Q6 (`T10-005`)
    - Final reward: 4 Red Herbs (`T10-006`)
  - **Usher's Subsidy:** Screen dialogue says "Training Subsidy 50 Zeny supplied" (`T10-007`) while the script increases zeny by 150 (`T10-008`)
  - **Fabre Forest Knockout:** Knocked-out novice wakes beside Helper at `new_1-2 16,183` (`T10-009`)
  - **The End:**
    - Training ground final score awards up to 500z and 10 Red Herbs: `T10-010`
    - Choosing "I will follow the result of the test" prompts "Under development" and opens manual job selection: `T10-011`
    - Player account (Weggy FAQ): graduates finish with 100z, or 200z if NPCs guessed the job correctly: `T10-012`
    - Player account (Weggy FAQ): teleported to designated class town (Izlude for Swordsman, Geffen for Mage, Morroc for Thief, Payon for Archer, Prontera for Acolyte, Alberta for Merchant): `T10-013`
  - **Starting Gear:** Fresh characters start with Knife (item 1201) in weapon slot and Cotton Shirt (item 2301) in armor slot in mod0631 (`T10-014`, `T10-015`) and 817 (`T10-016`, `T10-017`)
  - **Basic Skill:** Level 9 required to change to a 1st job class: `T10-018`

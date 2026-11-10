# Deep Research Round 3: Findings by Row ID

All verified claims and evidence are recorded as single-fact JSON objects in `r3/rows/<task>.jsonl`. Source verification and link validation results are preserved in `r3/check.txt` (258 rows: 242 PASS, 0 FAIL, 16 NONE under local verification).

---

## Task W: War of Emperium

### Questions and Row IDs

- **W1. kRO Timeline & Patch Sequence:**
  - Official siege release and initial schedule: `W1-001`, `W1-002`, `W1-003`, `W1-004`, `W1-005`
  - Guild skill, emblem, and castle system updates: `W1-006`, `W1-007`, `W1-008`, `W1-009`, `W1-010`, `W1-011`, `W1-012`, `W1-013`, `W1-014`
  - Siege mechanics, treasure, and Agit investment updates: `W1-015`, `W1-016`, `W1-017`, `W1-018`, `W1-019`, `W1-020`, `W1-021`, `W1-022`, `W1-023`
- **W2. Early Emulator Mechanics (jAthena & 817):**
  - GvG map flags and damage penalties: `W2-001`, `W2-002`, `W2-003`, `W2-004`, `W2-005`
  - Emperium properties, skill targeting, and status immunity: `W2-006`, `W2-007`, `W2-008`, `W2-009`, `W2-010`
  - Castle investment mechanics and steward scripts: `W2-011`, `W2-012`, `W2-013`, `W2-014`, `W2-015`, `W2-016`, `W2-017`, `W2-018`, `W2-019`, `W2-020`
- **W3. Castle Assets, Guardians, Treasure & Guild Skills:**
  - Guardian monsters (1285–1287) & Emperium (1288): `W3-MOB-ARCHER`, `W3-MOB-KNIGHT`, `W3-MOB-SOLDIER`, `W3-MOB-EMPELIUM`
  - Treasure boxes (IDs 1324–1363 across 20 castles): `W3-BOX-01` through `W3-BOX-40`
  - Castle database mapping and guild leveling table: `W3-CASTLE-DB`, `W3-EXP-GUILD`
  - Client guild skills (`GD_`): `W3-SKILL-APPROVAL`, `W3-SKILL-KAFRA`, `W3-SKILL-GUARD`, `W3-SKILL-CHARISMA`, `W3-SKILL-EXTENSION`
  - Guild fund and investment messages: `W3-MSG-FUNDS`
- **W4. Castle and Guild Dungeon Walkability:**
  - Aldebaran castles (`aldeg_cas01` to `05`): `W4-ALDEG-01`, `W4-ALDEG-02`, `W4-ALDEG-03`, `W4-ALDEG-04`, `W4-ALDEG-05`
  - Geffen castles (`gefg_cas01` to `05`): `W4-GEFG-01`, `W4-GEFG-02`, `W4-GEFG-03`, `W4-GEFG-04`, `W4-GEFG-05`
  - Payon castles (`payg_cas01` to `05`): `W4-PAYG-01`, `W4-PAYG-02`, `W4-PAYG-03`, `W4-PAYG-04`, `W4-PAYG-05`
  - Prontera castles (`prtg_cas01` to `05`): `W4-PRTG-01`, `W4-PRTG-02`, `W4-PRTG-03`, `W4-PRTG-04`, `W4-PRTG-05`
  - Guild dungeons (`gld_dun01` to `04`): `W4-DUN-01`, `W4-DUN-02`, `W4-DUN-03`, `W4-DUN-04`
- **W5. Period Numbers & Formula Provenance:**
  - Flat 10,000z steward investment: `W5-001`, `W5-002`, `W5-003`
- **W6. First-Hand Player Accounts (2003–2004):**
  - euRO Iris siege schedules and earliest battle topics: `W6-001`, `W6-002`
  - Castle conquerors and defense durations: `W6-003`, `W6-004`
  - Player expectations and beta rate discussions: `W6-005`, `W6-006`, `W6-007`, `W6-008`
- **W7. euRO Siege Timeline Announcements:**
  - Launch milestones, open beta and wipe notices: `W7-001`, `W7-002`, `W7-003`, `W7-004`

### What Was Not Found

- Individual castle assignments for treasure boxes 1324–1363 in period emulator databases prior to 2004-06: `W3-BOX-CASTLE-NONE`
- Post-2005 dynamic investment formula `5000 + 1000 * (val/5)` in period sources: `W5-004`
- Quadratic/exponential investment decay formulas prior to mid-2004: `W5-005`
- Dynamic guardian defense stat scaling formula in early 2004 sources: `W5-006`
- Period source confirming iRO 80% normal attack reduction claim: `W5-007`

### Recommendations

- Implement flat 10,000z investment stepping per steward interaction referencing `W2-014`, `W5-001`, `W5-002`, and `W5-003`.
- Configure Emperium as Small, Holy 1, Plant-type with 100% flee and status immunity based on `W2-006`, `W2-007`, `W2-008`, `W2-009`, and `W3-MOB-EMPELIUM`.
- Use `data.grf` map files for all 20 castles and 4 guild dungeons, referencing coordinates validated in `W4-ALDEG-01` through `W4-DUN-04`.
- Gate initial WoE castle access and Kafra leasing behind guild skills `W3-SKILL-APPROVAL` and `W3-SKILL-KAFRA`.

---

## Task Q: Quest Skills

### Questions and Row IDs

- **Novice:**
  - First Aid: `Q-NV-FA-B`, `Q-NV-FA-C`
  - Play Dead: `Q-NV-PD-B`, `Q-NV-PD-C`
- **Swordman:**
  - Moving HP Recovery: `Q-SM-MH-A`, `Q-SM-MH-B`, `Q-SM-MH-C`
  - Fatal Blow: `Q-SM-FB-A`, `Q-SM-FB-B`, `Q-SM-FB-C`
  - Auto Berserk: `Q-SM-AB-A`, `Q-SM-AB-B`, `Q-SM-AB-C`
- **Mage:**
  - Energy Coat: `Q-MG-EC-A`, `Q-MG-EC-B`, `Q-MG-EC-C`
- **Archer:**
  - Arrow Crafting: `Q-AC-AC-A`, `Q-AC-AC-B`, `Q-AC-AC-C`
  - Charge Arrow: `Q-AC-CA-A`, `Q-AC-CA-B`, `Q-AC-CA-C`
- **Acolyte:**
  - Holy Light: `Q-AL-HL-A`, `Q-AL-HL-B`, `Q-AL-HL-C`
- **Merchant:**
  - Cart Revolution: `Q-MC-CR-A`, `Q-MC-CR-B`, `Q-MC-CR-C`
  - Change Cart: `Q-MC-CC-A`, `Q-MC-CC-B`, `Q-MC-CC-C`
  - Loud Speaker / Crazy Uproar: `Q-MC-LD-A`, `Q-MC-LD-B`, `Q-MC-LD-C`
- **Thief:**
  - Sand Attack: `Q-TF-SA-B`, `Q-TF-SA-C`
  - Back Slide: `Q-TF-BS-A`, `Q-TF-BS-B`, `Q-TF-BS-C`
  - Find Stone: `Q-TF-FS-A`, `Q-TF-FS-B`, `Q-TF-FS-C`
  - Stone Fling: `Q-TF-SF-A`, `Q-TF-SF-B`, `Q-TF-SF-C`

### What Was Not Found

- Novice First Aid quest script in 817 import files: `Q-NV-FA-A`
- Novice Play Dead quest script in 817 import files: `Q-NV-PD-A`
- Thief Sand Attack quest script in 817 import files: `Q-TF-SA-A`

### Recommendations

- Script Novice First Aid using the three-step hospital sequence (`prt_in 238,168`, `prt_in 274,166`, `prt_church 103,98`) with 3 Red Herbs, 3 Clovers, 1 Empty Bottle, and the timed delivery step from `Q-NV-FA-B` and `Q-NV-FA-C`.
- Script Novice Play Dead with Peter Smail at `prontera 270,212` requiring Job 7 Novice without items, citing `Q-NV-PD-B` and `Q-NV-PD-C`.
- Script Thief Sand Attack using Bagot at `moc_prydb1 154,128` requiring 5 Fine Sand, 1 Earthworm the Dude, and 1 Bag of Grain, citing `Q-TF-SA-B` and `Q-TF-SA-C`.
- Use the item requirements from RODataZone witness C (`Q-MC-CR-C`, `Q-MC-CC-C`, `Q-MC-LD-C`) for Merchant quests where mod0631 and 817 item IDs differ.

---

## Task J: 2-1 Job Quests

### Questions and Row IDs

- **Wizard Exam Stages & Rooms:**
  - Underlying client map `job_wizard.gat`: `J-WIZ-MAP-A`, `J-WIZ-MAP-B`, `J-WIZ-MAP-C`
  - Room of Water (Stage 1): `J-WIZ-WATER`
  - Room of Earth (Stage 2): `J-WIZ-EARTH`
  - Room of Fire (Stage 3): `J-WIZ-FIRE`
  - Combat test bypass (Old Magicbook): `J-WIZ-BYPASS`
- **Priest Exam:**
  - Basilica map asset `job_priest.gat`: `J-PRI-MAP-A`
  - Purification and endurance tests: `J-PRI-SCRIPT`, `J-PRI-ALTAR`
- **Hunter Exam:**
  - Asset `job_hunter.gat` and waiting room script: `J-HUN-MAP-A`, `J-HUN-SCRIPT`
- **Knight Exam:**
  - Asset `job_knight.gat` and chivalry trials: `J-KNI-MAP-A`, `J-KNI-NPC`
- **Blacksmith & Assassin Exams:**
  - Blacksmith examination in Geffen & Einbroch: `J-BS-NPC`
  - Assassin guild exam script: `J-ASN-SCRIPT`
- **Client Job Change NPCs:**
  - First-job to 2-1 job NPC positions: `J-KRO-21NPC`

### What Was Not Found

- Dedicated Assassin test map asset (`job_assassin.gat` does not exist in `data.grf.original`): `J-ASN-MAP`
- Dedicated Blacksmith test map asset (`job_blacksmith.gat` does not exist in `data.grf.original`): `J-BS-MAP`

### Recommendations

- Reconstruct the Wizard practical exam using `job_wizard.gat` across the three sequenced arenas (North Water `116,171`, Centre Earth `116,97`, West Fire `46,99`) with monster lists from `J-WIZ-WATER`, `J-WIZ-EARTH`, and `J-WIZ-FIRE`.
- Allow the combat skip on repeated Wizard failures when the player offers an Old Magicbook, citing `J-WIZ-BYPASS`.
- Map stage virtual maps (`wizard_1-1` to `wizard_3-1`, `priest_1-1` to `priest_3-1`, `hunter_1-1` to `hunter_3-1`, `knight_1-1` to `knight_3-1`) to their single underlying `.gat` files per `J-WIZ-MAP-B`, `J-PRI-MAP-A`, `J-HUN-MAP-A`, and `J-KNI-MAP-A`.

---

## Task D: Dialogue & Withdrawals

### Questions and Row IDs

- **Verified Dialogue & Bug Interactions:**
  - Merchant delivery quest lines & 1000z fee: `D-BUG-1609`, `D-BUG-1610`
  - Blacksmith selling crash & inventory interaction: `D-BUG-1611`
  - Sage history exam question: `D-BUG-1618`
  - Blacksmith party buff interaction on Merchants: `D-BUG-2259`
  - Hollgrehenn refine confirmation and failure lines: `D-NPC-HOLL`
  - Standard Kafra service opening greeting: `D-NPC-KAFR`

### What Was Not Found (Round 2 Dialogue Withdrawals)

- Chief Mahnsoo dialogue on RODataZone: `D-WD-MAHN`
- Father Mareusis dialogue on RODataZone: `D-WD-MARE`
- Syracuse dialogue on RODataZone: `D-WD-SYRA`
- Kafra greeting dialogue on RODataZone: `D-WD-KAFR`
- Hollgrehenn dialogue on RODataZone: `D-WD-HOLL`
- Peco breeder dialogue on RODataZone: `D-WD-PECO`

### Recommendations

- Replace withdrawn Round 2 dialogue quotes with authenticated emulator script dialogue from `D-NPC-HOLL` and `D-NPC-KAFR`.
- Use player-reported phrasing from iRO bug reports `D-BUG-1609` and `D-BUG-1610` for Merchant delivery dialogue.

---

## Tasks P, K, R, M: Smaller Independent Threads

### Questions and Row IDs

- **Task P (Pets):**
  - Official 24-pet roster count: `P-TOTAL-24`
  - Unreleased 25th pet (Bongun) item: `P-BON_GUN`
  - Taming items for all pets in `pets.toml`: `P-BAPHOMET_Jr` through `P-YOYO`
- **Task K (Kafra):**
  - Teleport fare point accrual (10% flat via `KF_PIT`): `K-PTS-WARP`
  - Standard storage fee (40z without points): `K-STOR-FEE`
  - Accumulation check menu (`積立ポイント確認`): `K-PTS-CHECK`
  - Official point redemption opening at Aldebaran Headquarters: `K-PTS-KRO`
- **Task R (Refine):**
  - Armor safe limit nerf to +4: `R-KRO-DEF-NERF`
  - Level 4 weapon refine +7 ATK and safe limit +4: `R-KRO-ATK-LV4`
  - Overrefine random ATK bonus mechanic: `R-KRO-OVERREF`
  - Emulator refine bonus table and integer DEF scaling (+0.7 DEF): `R-817-REF-DB`, `R-817-REF-PC`
- **Task M (Monster HP & EXP Adjustments):**
  - Official high-DEF monster rebalance (DEF down, MHP up): `M-KRO-HP-DEF`
  - euRO New Comodo Point 6 announcement: `M-COMODO-PT6`
  - Baphomet HP progression from 568,000 to 668,000: `M-BAPHOMET-MOD`, `M-BAPHOMET-817`

### What Was Not Found

- Bongun was never listed among the 24 official release pets in 2004 captures (retained in `pets.toml` as unreleased): `P-TOTAL-24`, `P-BON_GUN`

### Recommendations

- Implement Kafra point rewards at exactly 10% of teleport fare via `KF_PIT`, keeping storage at a flat 40z with 0 points, citing `K-PTS-WARP`, `K-STOR-FEE`, and `K-PTS-CHECK`.
- Set armor refine safe limit to +4 and Lv 4 weapon refine to +7 ATK per level with safe limit +4, citing `R-KRO-DEF-NERF` and `R-KRO-ATK-LV4`.
- Apply integer DEF bonus scaling `(refinedef + 50) / 100` from `R-817-REF-PC`.
- Set Baphomet HP to 668,000 for post-2003-02 balance based on `M-KRO-HP-DEF`, `M-COMODO-PT6`, and `M-BAPHOMET-817`.

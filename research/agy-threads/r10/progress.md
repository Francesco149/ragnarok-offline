# Round 10 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | The 2-1 job tests: Hunter test walkthrough (thread 8) | Answered | T1-001 – T1-012 |
| T5 | Turtle Island travel, sailor Gotanblue, and Alberta headgear (thread 12) | Answered | T5-001 – T5-008 |
| T6 | Dye NPCs official names, hair dyeing terms, and cloth dyeing status (thread 15) | Answered | T6-001 – T6-005 |
| T7 | The 10 target 2-2 skills across 817 and iRO guide (thread 2) | Answered | T7-001 – T7-025 |
| T8 | Skid Trap shove range change in Comodo 3.0 patch (thread 22) | Answered | T8-001 – T8-002 |
| T9 | Small, independent (threads 7, 23, 24, 26, 27, 28) | Answered | T9-001 – T9-021 |

Total: 73 rows across 6 tasks.
Checker result (`check.txt`): 73 PASS, 0 WARN, 0 FAIL, 0 NONE (verified live with `python3 ../../check_rows.py --fetch`).

## Task Details and Row Mapping

### T1: Hunter Test Walkthrough (Thread 8)
- **Status:** Answered
- **Rows:** `T1-001` through `T1-012`
  - Walkthrough from contemporary player guide (F0rever Zer0, FAQ 27633, iRO):
    - Front desk personality test (`T1-001`).
    - Item gathering combos 1, 2, 3 (`T1-002` – `T1-004`).
    - Finding Master Hunter at central palace (`payon_in03`) or Payon Village (`payon_in02`) (`T1-005`).
    - Test supplies: 200 silver arrows given by Master Hunter (`T1-006`).
    - Waiting room chatroom for 1 candidate at a time (`T1-007`).
    - Test arena objectives: kill 4 Job Change Monsters (Poring, Zombie, Archer Skeleton, Mummy) without killing decoy monsters (Hunter Change Monster, Job Test Monster) (`T1-008`, `T1-009`).
    - Visual indicator of arena traps (water-filled tiles) (`T1-010`).
    - Center switch escape phase with 3-minute timer (`T1-011`).
    - Class change reward: necklace delivered to front desk lady (`T1-012`).

### T5: Turtle Island Travel and Headgear Quests (Thread 12)
- **Status:** Answered
- **Rows:** `T5-001` through `T5-008`
  - Turtle Island access from player guide: Alberta Inn old man -> docks sailor -> ship sailor -> 10,000z fare (`T5-001`).
  - rAthena official conversion:
    - Sailor Gotanblue (`Sailor#tur`) at `alberta 247,122` (`T5-002`).
    - Fare is 10,000z, warping to `tur_dun01 157,39` (`T5-003`).
    - Return sailor `Sailor#tur2` at `tur_dun01 165,29` warps back to `alberta 238,112` / `241,115` (`T5-004`).
  - Headgear quests:
    - Boy's Cap: Grampa at `alberta_in 122,53` exchanges 10 Tiger's Footskins for Boy's Cap (`T5-005`).
    - Antlers: Cherokee at `alberta_in 122,53` (`T5-006`).
    - Stylish Merchant (`Stylish Merchant#new30`) at `alberta 120,53` (`T5-007`).
    - Hat store girl (`Hat store girl#new30`) at `alberta 136,79` (`T5-008`).

### T6: Dye NPCs and Official Status (Thread 15)
- **Status:** Answered
- **Rows:** `T6-001` through `T6-005`
  - Hairdresser Jovovich at `prt_in 243,168`, charging 1000z and 1 Dyestuffs (`T6-001`, `T6-002`).
  - Cloth dyeing status: rAthena script note v2.4 documents cloth dyeing was never implemented on official servers and causes client crashes (`T6-003`).
  - Official Aegis script in `prontera.txt`: Tailor Ginedin Rephere at `prt_in 284,168` turns players away with an apology that he is too busy with orders, confirming cloth dyeing was never active (`T6-004`).
  - Chemist Morgenstein at `geffen_in 141,140` for counteragent and mixture (`T6-005`).

### T7: The 10 Target 2-2 Skills (Thread 2)
- **Status:** Answered
- **Rows:** `T7-001` through `T7-025`
  - **Grand Cross (Crusader):** 817 unit configuration (29 cells, 1000ms limit, 300ms interval, 900ms movement lock, 1000000 hitrate) (`T7-001` – `T7-003`); iRO guide (0.9s duration, 3 hits, self-damage half of enemy damage, 140%-500% ATK, 37-100 SP) (`T7-004` – `T7-006`).
  - **Aid Potion / Potion Pitcher (Alchemist):** 817 requirements (1 SP) (`T7-007`); iRO guide (110%-150% potion value, Red to Blue) (`T7-008`).
  - **Acid Terror (Alchemist):** 817 requirements (15 SP, 1 Acid Bottle) (`T7-009`); iRO guide (consumes 1 Acid Bottle, 140%-300% ATK, 3%-13% Armor Damage chance) (`T7-010`, `T7-011`).
  - **Bomb / Demonstration (Alchemist):** 817 unit configuration (limit 5000*lv ms, interval 2000ms, range 1) (`T7-012`); iRO guide (damages every 1 second, 120%-200% ATK, 1%-5% Weapon Damage chance, 10 SP) (`T7-013`, `T7-014`).
  - **Volcano (Sage):** 817 unit configuration (`T7-015`); iRO guide (10%-20% fire ATK, +10 to +50 base ATK, 5-9 cells, 48-40 SP) (`T7-016`).
  - **Deluge (Sage):** iRO guide (10%-20% water ATK, 5%-15% MaxHP, 5-9 cells, 48-40 SP) (`T7-017`).
  - **Violent Gale / Whirlwind (Sage):** iRO guide (10%-20% wind ATK, +3 to +15 Flee, 5-9 cells, 48-40 SP) (`T7-018`).
  - **Land Protector / Magnetic Earth (Sage):** 817 unit configuration (limit (90+30*lv)*1000 ms, diameter lv+lv%2+5) (`T7-019`); iRO guide (blocks ground magic, consumes 1 Blue and 1 Yellow Gemstone, 7-11 cells, 2-5 min duration, 66-50 SP) (`T7-020`, `T7-021`).
  - **Magic Rod (Sage):** completely missing in 817; iRO guide (receives spell SP instead of damage, consumes 2 SP, 0.4s-1.2s duration, 20%-100% SP received) (`T7-022`, `T7-023`).
  - **Spell Breaker (Sage):** iRO guide (cancels enemy cast, pierces Phen card, consumes 2 SP, drains 0%-100% SP) (`T7-024`, `T7-025`).

### T8: Skid Trap Range in Comodo 3.0 (Thread 22)
- **Status:** Answered
- **Rows:** `T8-001` – `T8-002`
  - FAQ 27633 Comodo Update (Episode 3.0) patch notes confirm Skid Trap shove range was increased from 6 to 10 cells and SP cost from 5 to 10 (`T8-001`).
  - Ankle Snare SP cost increased from 10 to 12 in the same patch (`T8-002`).

### T9: Small, Independent
- **Status:** Answered
- **Rows:** `T9-001` through `T9-021`
  - **Thread 28 (Others' ground units/traps seen twice / invisible traps):** FAQ 27633 explicitly documents: *"It is invisible only to people who were off screen when it was planted, or those who go off screen and come back"* and *"Invisible traps are visible to only the caster and those who were on the screen at the time the traps were installed"* (`T9-001`, `T9-002`). This directly matches the client's packet caching behavior.
  - **Shockwave Trap cost:** FAQ 27633 confirms `TRAPS NEEDED: 1`, confirming mod0631 over client description (`T9-003`).
  - **Castle warps onto blocked cells:**
    - `aldeg_cas05 188,49 -> 165,288`: blocked (out of 240x240 map bounds); corrected in rAthena `guildcastles.txt:389` to `165,228` (walkable type 0) (`T9-004`).
    - `prtg_cas02 184,44 -> 157,140`: blocked (type 1 wall); connects to Component 4 Emperium room (X=[184..203], Y=[162..182]), with return warp at `157,135` also blocked (`T9-005`).
  - **Payon Cave 4F blocked arrival:** `pay_dun04 193,43` is blocked (type 1); rAthena `pay_dun.txt:26` corrects arrival to `189,43` (walkable type 0, near exit at `191,41`) (`T9-006`).
  - **Turtle Island 5F (`tur_dun05`):** 817 and rAthena define two exits (`89,69 -> tur_dun02 148,264` and `9,45 -> tur_dun01 154,237`) and 0 entrances in any script (`T9-007`).
  - **Quest skills:** Cart Revolution requires 20 Irons and 1 Banana Juice in rAthena (`T9-008`); Charge Arrow NPC Jason asks for a Crossbow but never deletes it from inventory (`T9-009`, `T9-010`).
  - **First job test small sums:** Thief test deletes mushrooms (`T9-011`); Acolyte ascetic Yosuke is at `prt_fild00 208,218` (walkable type 0) (`T9-012`); Merchant wrong parcel is item 1091 (`Merchant_Box_Etc`) (`T9-013`).
  - **Doll child:** Elin at `alberta 117,135` exchanges Baphomet Doll for 5 Panacea (`T9-014`).
  - **Monster skills, levels, and rates (rAthena `mob_skill_db.txt`):** Bigfoot Endure (`T9-015`), Savage Inc Agi (`T9-016`), Doppelganger Dec Agi (`T9-017`), Hornet Piercing Attack (`T9-018`), Nightmare Mental Breaker (`T9-019`), Chepet Status Recovery (`T9-020`), Mistress Pneuma (`T9-021`).

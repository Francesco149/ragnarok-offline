# Round 4 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | Turtle Island in Winter 2003/04 (Thread 24) | Answered | T1-001 – T1-012 |
| T2 | The 2-2 Job Skills, Skill by Skill (Thread 22, Roadmap §3) | Answered | T2-001 – T2-025 |
| T4 | Dyeing (Thread 13) | Answered | T4-001 – T4-012 |
| T5 | Skill Numbers: Client Text vs Emulators (Thread 20) | Answered | T5-001 – T5-023 |
| T9 | Moving HP Recovery Quest Items (Carry-over) | Answered | T9-001 – T9-013 |

Total: 85 rows across 5 tasks.
Checker result (`check.txt`): 84 PASS, 0 WARN, 0 FAIL, 1 NONE.

## Task Details and Row Mapping

### T1: Turtle Island in Winter 2003/04 (Thread 24)
- **Status:** Answered
- **Rows:** `T1-001` through `T1-012`
  - iRO arrival notice (Dec 16, 2003): `T1-001`
  - mod0631 ferry NPC Gotenble at `alberta 247,122`, 10,000z fee, precondition `turtle_npc == 1`: `T1-002`, `T1-003`, `T1-004`, `T1-005`
  - Turtle Grandpa at `alberta_in 23,104` setting precondition: `T1-006`
  - 817 Alberta sailor at `alberta 189,151` 800z warp without quest: `T1-007`
  - mod0631 dungeon exploration objects (Skull Grave, Tree Root, Crystal Mound, Turtle Pillar): `T1-008`, `T1-009`, `T1-010`, `T1-011`
  - Negative search for euRO closed beta/early 2004 news: `T1-012`

### T2: The 2-2 Job Skills (Thread 22, Roadmap §3)
- **Status:** Answered
- **Rows:** `T2-001` through `T2-025`
  - Auto Guard: iRO duration 300s, 0.3s movement lock, block chance 5% to 30% (`T2-001`, `T2-002`); 817 300ms pause (`T2-003`); mod0631 `#if 0` (`T2-004`)
  - Defender: iRO duration 180s, speed reduction, long-range ATK reduction -20% to -80% (`T2-005`, `T2-006`); 817 damage reduction commented out (`T2-007`); 817 ASPD penalty (`T2-008`); mod0631 `#if 0` (`T2-009`)
  - Reflect Shield: iRO duration 300s, reflect 13% to 40% (`T2-010`, `T2-011`); 817 empty case (`T2-012`)
  - Divest (Strip) skills: iRO basic chances 7% to 15%, 17 to 25 SP (`T2-013`); mod0631 commented out (`T2-014`)
  - Chemical Protection: iRO durations 60s to 300s (`T2-015`); mod0631 empty cases (`T2-016`)
  - Aid Potion (Potion Pitcher): iRO recovery 110% to 150% (`T2-017`)
  - Summon Spirit Sphere: iRO duration 10 min, 5 spheres max (`T2-018`); mod0631 duration 10 min (`T2-019`)
  - Guillotine Fist: iRO Additional ATK +400 to +1000 (`T2-020`)
  - Sage Land Skills (Volcano, Deluge, Whirlwind): iRO bonuses +10% to +20%, areas 5x5 to 9x9 (`T2-021`, `T2-022`, `T2-023`); mod0631 `#if 0` (`T2-024`)
  - Dispell: iRO success chance 60% to 100% (`T2-025`)

### T4: Dyeing (Thread 13)
- **Status:** Answered
- **Rows:** `T4-001` through `T4-012`
  - Merchant Louitz at `alberta_in 133,55` naming Morgenshtein in Geffen: `T4-001`, `T4-002`
  - Aure Dupon at `geffen 184,114` directing to 2nd floor of Blacksmith guild: `T4-003`, `T4-004`
  - Morgenshtein at `geffen_in 141,140`: `T4-005`
  - Dairenne at `prontera 78,150`: `T4-006`
  - Zavadrihan at `morocc_in 146,99`: `T4-007`
  - Clothes dyer Ginedein Refur at `prt_in 284,168` and refusal: `T4-008`, `T4-009`
  - Hair stylist Jovovich at `prt_in 243,168`, 1 dye + 5,000z fee, and unimplemented status: `T4-010`, `T4-011`, `T4-012`

### T5: Skill Numbers: Client Text vs Emulators (Thread 20)
- **Status:** Answered
- **Rows:** `T5-001` through `T5-023`
  - `SM_SWORD`: Client +5 (`T5-001`) vs mod0631 `skill * 4` (`T5-002`)
  - `SM_TWOHAND`: Client +5 (`T5-003`) vs mod0631 `skill * 4` (`T5-004`)
  - `HT_BEASTBANE`: Client +3 (`T5-005`) vs mod0631 `skill * 4` (`T5-006`)
  - `KN_TWOHANDQUICKEN`: Client 15s (`T5-007`) vs mod0631 `30 * val1` (`T5-008`)
  - `PR_IMPOSITIO`: Client 40s (`T5-009`) vs mod0631 60s (`T5-010`)
  - `PR_ASPERSIO`: Client 5s (`T5-011`) vs mod0631 `30 + val1 * 30` (`T5-012`)
  - `PR_KYRIE`: Client 10s, 1 hit (`T5-013`) vs mod0631 120s (`T5-014`)
  - `ALL_RESURRECTION`: Client 1 HP / 15% (`T5-015`) vs mod0631 1 Blue Gemstone (`T5-016`)
  - `AC_CHARGEARROW`: Client 4 cells shove (`T5-017`)
  - `HT_LANDMINE`: Client 65s setting time (`T5-018`) vs mod0631 `40000*(6-skilllv)` (`T5-019`)
  - `HT_BLASTMINE`: Client 10s fuse (`T5-020`) vs mod0631 `5000*(6-skilllv)` (`T5-021`)
  - `HT_SHOCKWAVE`: Client 20% SP drain (`T5-022`) vs mod0631 `skilllv*15+10` (`T5-023`)

### T9: Moving HP Recovery Quest Items (Carry-over)
- **Status:** Answered
- **Rows:** `T9-001` through `T9-013`
  - mod0631 checks 200 bottles (713) and 1 Moth Wings (1058): `T9-001`
  - De Thomas requests 200 Empty Bottles: `T9-002`
  - De Thomas jokes armor is recommended and requests 1 Moth Wings for homework: `T9-003`
  - mod0631 deletes 200 bottles and 1 Moth Wings: `T9-004`
  - De Thomas confirms armor was not actually needed: `T9-005`
  - 817 demands 500 bottles, 1 Rocker Doll, 1 Padded Armour: `T9-006`
  - 817 still deletes 200 bottles and 1 Moth Wings: `T9-007`
  - RODataZone demands 1 Rocker Doll, 500 Empty Bottle: `T9-008`
  - iRO guide 25% recovery while walking: `T9-009`
  - Client GRF item display names: 713 (`T9-010`), 752 (`T9-011`), 1058 (`T9-012`), 2312 (`T9-013`)

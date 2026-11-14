# Round 8 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent | Partly | T9-001 – T9-054 |
| T1 | The 2-1 job tests: dating and sharpening them (threads 5, 8) | Partly | T1-001 – T1-014 |

Total: 68 rows across 2 tasks.
Checker result (`check.txt`): 68 PASS, 0 WARN, 0 FAIL, 0 NONE (verified live with `python3 ../check_rows.py --fetch`).

## Task Details and Row Mapping

### T9: Small, Independent
- **Status:** Partly (bullets left: Kafra fares outside jRO, start cell, Lutie in January, monster skills)
- **Rows:** `T9-001` through `T9-054`
  - **Packet IDs 0x01f1 to 0x01ff (15 packets):**
    - `0x01f1`: `AC_NOTIFY_ERROR`, len -1 (`T9-001`)
    - `0x01f2`: `ZC_UPDATE_CHARSTAT2`, len 20 (`T9-002`)
    - `0x01f3`: `ZC_NOTIFY_EFFECT2`, len 10 (`T9-003`)
    - `0x01f4`: `ZC_REQ_EXCHANGE_ITEM2`, len 32 (`T9-004`)
    - `0x01f5`: `ZC_ACK_EXCHANGE_ITEM2`, len 9 (`T9-005`)
    - `0x01f6`: `ZC_REQ_BABY`, len 34 (`T9-006`)
    - `0x01f7`: `CZ_JOIN_BABY`, len 14 (`T9-007`)
    - `0x01f8`: `ZC_START_BABY`, len 2 (`T9-008`)
    - `0x01f9`: `CZ_REQ_JOIN_BABY`, len 6 (`T9-009`)
    - `0x01fa`: `CA_LOGIN3`, len 48 (`T9-010`)
    - `0x01fb`: `CH_DELETE_CHAR2`, len 56 (`T9-011`)
    - `0x01fc`: `ZC_REPAIRITEMLIST`, len -1 (`T9-012`)
    - `0x01fd`: `CZ_REQ_ITEMREPAIR`, len 4 (`T9-013`)
    - `0x01fe`: `ZC_ACK_ITEMREPAIR`, len 5 (`T9-014`)
    - `0x01ff`: `ZC_HIGHJUMP`, len 10 (`T9-015`)
  - **Quest skills job level and item requirements across 817, mod0631, and RODataZone:**
    - Arrow Crafting: `T9-016` (817), `T9-017` (mod0631), `T9-018` (RODataZone)
    - Charge Arrow: `T9-019` (817), `T9-020` (mod0631), `T9-021` (RODataZone)
    - Cart Revolution: `T9-022` (817), `T9-023` (mod0631 set 2), `T9-024` (mod0631 Banana Juice check), `T9-025` (RODataZone)
    - Change Cart: `T9-026` (817), `T9-027` (mod0631), `T9-028` (RODataZone)
    - Crazy Uproar: `T9-029` (817), `T9-030` (mod0631), `T9-031` (RODataZone)
    - Holy Light: `T9-032` (817), `T9-033` (mod0631), `T9-034` (RODataZone)
    - Energy Coat: `T9-035` (817), `T9-036` (mod0631), `T9-037` (RODataZone)
  - **First-job tests' small sums:**
    - Thief pay and mushroom deletion: `T9-038` (817), `T9-039` (mod0631)
    - Acolyte Father Yosuke coordinates: `T9-040` (817), `T9-041` (mod0631)
    - Merchant delivery box: `T9-042` (817 item 1083), `T9-043` (mod0631 item 1091)
  - **Shockwave Trap count:**
    - Client text specifies 2 Traps Needed: `T9-044`
    - mod0631 consumes 1 trap: `T9-045`
    - 817 requires 1 trap: `T9-046`
  - **Alberta doll child:**
    - mod0631 doll exchange: `T9-047`
  - **Castle warps onto blocked cells:**
    - `aldeg_cas05 188,49` to `165,288`: `T9-048`
    - `prtg_cas02 184,44` to `157,140`: `T9-049`
  - **Places world audit cannot reach:**
    - `tur_dun05` exits: `T9-050`
    - `pay_dun04 193,43`: `T9-051`
    - `prt_in 280..285` castle exits: `T9-052`
  - **Clock Tower random warps:**
    - `c_tower2` portal `clt006r`: `T9-053`
  - **Unknown skill display:**
    - Packet `0x11a` in 817 `clif.c`: `T9-054`

### T1: The 2-1 Job Tests: Dating and Sharpening (Threads 5, 8)
- **Status:** Partly (Knight test details sharpened; bullets left: Blacksmith, Assassin, Hunter, Priest, Wizard)
- **Rows:** `T1-001` through `T1-014`
  - **Knight test details:**
    - Sir Andrew items: jAthena 1 list (`T1-001`) vs eAthena 1.0 2 lists (`T1-002`)
    - Sir James questions:
      - Question 2: jAthena Bowling Bash prerequisite (`T1-003`) vs eAthena 1.0 non-attack skill (`T1-004`)
      - Question 3: jAthena Brandish Spear prerequisite (`T1-005`) vs eAthena 1.0 most SP spear skill (`T1-006`)
      - Question 4: jAthena Ghost property Nightmare spear (`T1-007`) vs eAthena 1.0 Dark/Undead element (`T1-008`)
    - Sir Grey evaluation:
      - jAthena unconditional pass (`T1-009`)
      - eAthena 1.0 passing dialog (`T1-010`)
    - Timers:
      - Arena combat timeout (3 minutes / 180s): jAthena (`T1-011`), eAthena 1.0 (`T1-012`)
      - Sir Edmond patience test: jAthena 4.5 minutes / 270s (`T1-013`) vs eAthena 1.0 4 minutes / 240s (`T1-014`)

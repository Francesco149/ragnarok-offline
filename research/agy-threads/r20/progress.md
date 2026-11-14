# Round 20 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T8 | Things only drafted from client's text (thread 22) | partly (Ice Wall durability confirmed starting at 400 with +200/lvl decaying at 50/sec in iRO Guide 2004; eAthena 817 subtracts 5 durability per 100ms and explicitly permits attacking unit 0x8d; jAthena //1092 reverts placement check to match official server allowing casting under entities; Poison React duration confirmed 30s across all levels in iRO Guide 2004 vs 30m in eAthena 817; Remove Trap confirmed 5 SP, regains 1 trap, cannot remove active traps; Spring Trap confirmed 10 SP, 4-8 cell range, requires Falcon; Venom Splasher HP threshold confirmed < 2/3 in anjingbalap FAQ 29538 and euRO May 2004 text vs < 1/3 in iRO Guide 2004, with 5.5s-10s explosion delay and 15s-60s cooldown in jAthena //1047; Talkie Box triggers on player/party in Dec 2003 client text; euRO May 2004 text retains Skid Trap 1-5 cell slide while Ankle Snare incorporates AGI duration scaling) | T8-021 – T8-038 |
| T9 | Small, independent (threads 26, 27, R6) | partly (tur_dun05 portal audit confirmed no transitions exist in OpenKore 2004 portals.txt connecting only tur_dun01-04; Turtle Island sailor NPC confirmed at alberta 247,122 taking player to tur_dun01 157,39; prt_in 282,179 portal exit confirmed leading to prontera 84,89) | T9-246 – T9-248 |
| T7 | The 2-2 skills that need no missing packet: where the witnesses part (threads 22, 25) | partly (Strip Weapon success probability confirmed dependent on skill level and caster DEX in euRO May 2004 client text) | T7-031 |
| T1 | 2-1 job tests: dating and sharpening (threads 5, 8) | partly (Wizard test failure confirmed teleporting and saving player outside Geffen Tower; combat test bypass with worn out scroll confirmed after 5-6 failures reaching Marine Sphere; Fire Room gatekeeper Horongs confirmed ignoring firewall; Wizard Quiz Version 1, Version 2, and retake questions and answers confirmed from Ghent v2.1 FAQ 30054) | T1-050 – T1-055 |

Total: 28 rows across 4 tasks.
Checker result (`check.txt`): 28 PASS, 0 WARN, 0 FAIL, 0 NONE of 28 rows verified with `python3 research/check_rows.py --pages research/agy-threads/r20/pages`.

## Task Details and Row Mapping

### T8: Things Only Drafted from Client's Text (Thread 22)
- **Status:** partly
- **Rows:** `T8-021` through `T8-038`
  - **Ice Wall Mechanics:**
    - `T8-021`: iRO Guide Wizard skill page (`20040327034745_jobwizskill.html`) records Icewall consumes 50 durability every second, loses durability equal to attack damage taken, and cancels when durability reaches zero.
    - `T8-022`: eAthena 817 `map/skill.c:2904` initializes Ice Wall unit setting count to 5 cells and limit to skill duration (`skill_get_time`).
    - `T8-023`: eAthena 817 `map/skill.c:6203` decrements Ice Wall durability by 5 every 100ms (`unit->val1 -= 5;`, equivalent to 50/sec), expiring unit when `val1 <= 0`.
    - `T8-024`: eAthena 817 `map/battle.c:3464` in `battle_check_target` explicitly permits targeting and attacking skill unit `0x8d` (Ice Wall).
    - `T8-025`: eAthena 817 `map/skill.c:3758` deducts received damage from Ice Wall durability (`src->val1 -= damage;`).
    - `T8-026`: jAthena //1092 (commit `79b0303f51f544cbbf21c32aa5655761b922dc04`, 2004-12-30) reverts //1088 placement check on Icewall to match official server behavior, allowing casting under players and monsters.
  - **Poison React:**
    - `T8-027`: iRO Guide Assassin skill page (`20040327034423_jobassskill.html`) records Poison React counters a poison property attack once and requires Enchant Poison level 2.
    - `T8-028`: eAthena 817 `db/cast_db.txt:103` sets Poison React upkeep_time to 1,800,000 ms (30 minutes).
    - `T8-029`: iRO Guide Assassin skill page records Poison React lasting time as 30 seconds across all skill levels, with SP cost decreasing from 30 to 18.
  - **Hunter Traps (Remove Trap & Spring Trap):**
    - `T8-030`: iRO Guide Hunter skill page (`20040327034419_jobhunskill.html`) records Remove Trap requires 5 SP, regains 1 trap item, and cannot remove activated traps.
    - `T8-031`: iRO Guide Hunter skill page records Spring Trap requires 10 SP, consumes 1 trap item, has range scaling from 4 to 8 cells, and requires a falcon.
  - **Venom Splasher:**
    - `T8-032`: iRO Guide Assassin skill page records Venom Splasher detonates when target has less than 1/3 max HP, causing 3x3 poison damage.
    - `T8-033`: anjingbalap Dagger Assassin Guide v2.2 (`gamefaqs_daggerassassin_anjingbalap_29538.html`, 2004-06-02) records Venom Splasher inflicts 200% + 20% x level damage with 25% splash in 3x3 area, requires target HP under 2/3, and cancels if target is hit before the timer finishes.
    - `T8-034`: jAthena //1047 (commit `f79e4c6394620f40a17580f57877c48b9fc5cdc7`, 2004-09-14) `db/skill_cast_db.txt:108` sets Venom Splasher explosion delay to 5500–10000 ms (5.5s–10s) and cooldown to 15000–60000 ms (15s–60s).
    - `T8-035`: euRO May 2004 client patch (`/data/mico07/yuno_lang/data/skilldesctable.txt:1144`) documents Venom Splasher requirement as enemy HP at least by 2/3.
  - **Trap Trigger & Comodo Balance Patch:**
    - `T8-036`: client `data\skilldesctable.txt:971` (Dec 2003 GRF) records Talkie Box triggers when stepped on by somebody including player and party.
    - `T8-037`: euRO May 2004 client patch (`/data/mico07/yuno_lang/data/skilldesctable.txt:895`) retained pre-Comodo Skid Trap sliding description of 1 to 5 cells.
    - `T8-038`: euRO May 2004 client patch (`/data/mico07/yuno_lang/data/skilldesctable.txt:899`) records Ankle Snare duration is affected by enemy AGI (matching Comodo patch behavior).

### T9: Small, Independent (Threads 26, 27, R6)
- **Status:** partly
- **Rows:** `T9-246` through `T9-248`
  - `T9-246`: OpenKore `trunk/tablepack/tables/portals.txt` at commit `1cd0757f1b8fe531e3b021b461e4611b61d8d4eb` (2004-02-29) links transitions only between `tur_dun01` and `tur_dun04`, confirming `tur_dun05` had no active portal transitions.
  - `T9-247`: OpenKore `portals.txt:7` records official Turtle Island ferry sailor NPC dialogue at `alberta 247,122` transporting the player to `tur_dun01 157,39`.
  - `T9-248`: OpenKore `portals.txt:987` records portal exit at `prt_in 282,179` leads to `prontera 84,89`.

### T7: The 2-2 Skills That Need No Missing Packet (Threads 22, 25)
- **Status:** partly
- **Rows:** `T7-031`
  - `T7-031`: euRO May 2004 client patch (`/data/mico07/yuno_lang/data/skilldesctable.txt:1310`) records Strip Weapon success probability is affected by skill level and caster DEX (`That probability is affected by Skill-level of Strip Weapon and Caster's DEX.`).

### T1: The 2-1 Job Tests: Dating and Sharpening (Threads 5, 8)
- **Status:** partly
- **Rows:** `T1-050` through `T1-055`
  - `T1-050`: Ghent Mage/Wizard Guide v2.1 (`gamefaqs_wizard_30054.html`, 2004-07-30) records failing the Wizard test teleports and saves the player outside Geffen Tower.
  - `T1-051`: Ghent Mage/Wizard Guide v2.1 records failing the combat test 5–6 times allows skipping the combat section by handing over a worn out scroll (provided the player reached Marine Sphere).
  - `T1-052`: Ghent Mage/Wizard Guide v2.1 records Fire Room monster spawns and notes Gatekeeper Horongs walk straight through Firewall.
  - `T1-053`: Ghent Mage/Wizard Guide v2.1 records Wizard Quiz Version 1 questions and answers (e.g., non-equippable headgear is Cap).
  - `T1-054`: Ghent Mage/Wizard Guide v2.1 records Wizard Quiz Version 2 questions and answers (e.g., skill not required for Firewall is Napalm Beat 4).
  - `T1-055`: Ghent Mage/Wizard Guide v2.1 records Wizard Quiz failure retake questions and answers (e.g., monster not recognizing casting is Marina).

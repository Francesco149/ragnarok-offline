# Round 14 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T7 | The 2-2 skills: where the witnesses part (threads 22, 25) | answered (All 9 remaining 2-2 skills: Grand Cross, Aid Potion/Potion Pitcher, Acid Terror, Bomb/Demonstration, Volcano, Deluge, Whirlwind/Violent Gale, Magic Rod, Spell Breaker, plus Land Protector duration disagreement, documented with 817 status/code, official iRO Guide text, and explicit disagreement rows) | T7-003 – T7-030 |
| T9 | Small, independent (threads 17, 23, 24, 26, 27, 29) | partly (Marx Hansen juices 1 fruit confirmed from Blood3D and Guide_Maker; Acolyte ascetic Yosuke cell prt_fild00 208,218 confirmed from H0bgoblin, gat_cells.py and 817 vs mod0631 208,255 blocked water; remaining: Baphomet Doll trade 5 Panacea vs 2 Royal Jelly, Merchant test letter 100z vs 500z, Kafra points for storage, client text vs mod0631 item stats) | T9-022 – T9-027 |
| T6 | Dyeing (threads 13, 29) | partly (Green Dye 1 Mixture and Violet Dye 30 Red Herbs answered from Blood3D Mixing Guide; remaining: kRO hair dyeing notice date) | T6-012 – T6-013 |
| T10 | The novice training ground outside Japan (thread 6) | partly (Novice starting gear 1 Knife, 1 Cotton Shirt, training ground quiz item rewards, personality test class recommendation and extra zeny subsidy answered from Mistiknoname3k 2003-12-13; remaining: Fabre forest wake-up cell on knockout, Leo Von Frisch quiz questions vs mod0631) | T10-010 – T10-011 |
| T8 | Things only drafted from client's text (thread 22) | partly (Ice Wall durability 400 to 2200 and 50/s drain plus damage taken drain answered from iRO Wizard guide; Sightrasher 8-direction ray pattern answered from iRO Wizard guide and 817 skill.c 24-cell layout; remaining: Frost Nova animation/screenshot, euRO Comodo balance patch date) | T8-001 – T8-003 |
| T1 | The 2-1 job tests: dating and sharpening (threads 5, 8) | partly (Pre-test instant switches and fee-only job changes answered for Knight 2500z, Assassin 666z, Blacksmith and Wizard from Weggy mid-2003 and Edge 2004-02; remaining: post-patch test details for Knight Sir Andrew items / Sir James questions, Blacksmith pass marks / Arbalest order, Assassin quiz / maze) | T1-001 – T1-005 |
| T5 | Turtle Island: who took you there, and quests of its patch (thread 24) | partly (RODataZone maps/city/alberta.html negative result none row confirming page was unarchived; remaining: Alberta sailor fare/conditions and logbook errands) | T5-009 |

Total: 47 rows across 7 tasks.
Checker result (`check.txt`): 46 PASS, 0 WARN, 0 FAIL, 1 NONE of 47 rows verified with `python3 research/check_rows.py`.

## Task Details and Row Mapping

### T7: The 2-2 Skills — Where the Witnesses Part (Threads 22, 25)
- **Status:** answered
- **Rows:** `T7-003` through `T7-030`
  - **Grand Cross (`CR_GRANDCROSS`):**
    - `T7-003`: eAthena 817 (`skill.c:2963`) unit definition: 29 cells, 1000ms limit, 300ms interval.
    - `T7-004`: iRO Guide Crusader: 0.9s duration, 3 hits by holy attack dealing 140% to 500% ATK, damages caster half of what enemy takes.
    - `T7-005`: Disagreement: 817 `cast_db.txt:122` specifies 1500ms delay and 1000ms unit duration, whereas iRO Guide specifies 3 second casting delay and 0.9s skill duration.
  - **Aid Potion / Potion Pitcher (`AM_POTIONPITCHER`):**
    - `T7-006`: eAthena 817 (`skill_require_db.txt:149`) requires 1 SP.
    - `T7-007`: iRO Guide Alchemist: consumes 10 SP to restore HP using Red, Orange, Yellow, White, or Blue potions with 110% to 150% potion value.
    - `T7-008`: Disagreement: 817 `skill.c:2298-2300` cures Poison, Silence, and Blind at level 5 instead of throwing a Blue Potion for 150% value as in iRO Guide (10 SP).
  - **Acid Terror (`AM_ACIDTERROR`):**
    - `T7-009`: eAthena 817 (`skill.c:2268`) calls `clif_skill_nodamage` with armor unequip chance only, dealing 0 HP damage.
    - `T7-010`: iRO Guide Alchemist: attacks target by throwing acid bottle with chance to damage armor.
    - `T7-011`: Disagreement: 817 `skill_db.txt:320` sets damage flag 0 (deals zero HP damage), whereas iRO Guide defines physical attack dealing 140% to 300% ATK damage.
  - **Bomb / Demonstration (`AM_DEMONSTRATION`):**
    - `T7-012`: eAthena 817 (`skill.c:2946`) unit interval 2000ms, duration `5000*skilllv` ms.
    - `T7-013`: iRO Guide Alchemist: sets ground on fire with bottle grenade, damaging target every 1 second without knockback.
    - `T7-014`: Disagreement: 817 `skill_require_db.txt:147` consumes 10 SP and 1 Bottle Grenade with 2000ms interval and no weapon break in `battle.c`, whereas iRO Guide damages every 1 second with 1% to 5% weapon damage chance.
  - **Volcano (`SA_VOLCANO`):**
    - `T7-015`: eAthena 817 (`cast_db.txt:142`) defines 180000ms duration and 5000ms cast time.
    - `T7-016`: iRO Guide Sage: magnetically amplifies fire attacks and attacks of fire-property characters (+10% to +20% Fire ATK, +10 to +50 ATK).
    - `T7-017`: Disagreement: 817 comments out cell layout in `skill.c:3125` and omits ATK increment bonuses in `battle.c`, whereas iRO Guide specifies +10 to +50 ATK.
  - **Deluge (`SA_DELUGE`):**
    - `T7-018`: eAthena 817 (`cast_db.txt:143`) defines 180000ms duration and 5000ms cast time.
    - `T7-019`: iRO Guide Sage: magnetically amplifies water attacks and increases maximum HP of water attribute characters (+10% to +20% Water ATK, +5% to +15% MaxHP).
    - `T7-020`: Disagreement: 817 comments out cell layout in `skill.c:3136` and omits MaxHP bonuses in `battle.c`, whereas iRO Guide specifies +5% to +15% MAX HP increment.
  - **Violent Gale / Whirlwind (`SA_VIOLENTGALE`):**
    - `T7-021`: eAthena 817 (`cast_db.txt:144`) defines 180000ms duration and 5000ms cast time.
    - `T7-022`: iRO Guide Sage: magnetically amplifies wind attacks, increases flee rate of wind characters (+3 to +15 Flee), and extends Fire Wall duration 1.5 times.
    - `T7-023`: Disagreement: 817 comments out cell layout in `skill.c:3147` and omits Flee bonuses in `battle.c`, whereas iRO Guide specifies +3 to +15 Flee increment.
  - **Magic Rod (`SA_MAGICROD`):**
    - `T7-024`: eAthena 817 (`cast_db.txt:135`) defines durations 400ms to 1200ms, but handler code is absent from `skill.c` and `battle.c`.
    - `T7-025`: iRO Guide Sage: absorbs target magic attacks into 20% to 100% SP recovery instead of physical damage.
    - `T7-026`: Disagreement: 817 defines data entries (`skill_require_db.txt:181` 2 SP) but zero implementation code in `skill.c` or `battle.c` (absent), whereas iRO Guide defines functional spell absorption and Spell Breaker counter.
  - **Spell Breaker (`SA_SPELLBREAKER`):**
    - `T7-027`: eAthena 817 (`skill.c:2367`) cancels target spell and heals caster `(skilllv-1)*25/100` of spell SP.
    - `T7-028`: iRO Guide Sage: cancels enemy spell and receives required SP (0% to 100%).
    - `T7-029`: Disagreement: 817 `skill_require_db.txt:182` requires 10 SP, whereas iRO Guide specifies 2 SP regardless of skill level.
  - **Land Protector (`SA_LANDPROTECTOR`):**
    - `T7-030`: Disagreement: 817 `cast_db.txt:145` caps duration at 4 minutes (120s to 240s), whereas iRO Guide scales duration from 2 minutes up to 5 minutes at level 5.

### T9: Small, Independent (Threads 17, 23, 24, 26, 27, 29)
- **Status:** partly
- **Rows:** `T9-022` through `T9-027`
  - **Marx Hansen Juices (1 Fruit vs 3 Fruits):**
    - `T9-022`: Blood3D Mixing Guide (`gamefaqs_mixing_blood3d_29037.html` Section III, 2004-02-28, pRO): Payon PVP juice maker requires **1 Fruit + 1 Empty Bottle + 3 zeny** (Apple, Banana, Carrot, Grape).
    - `T9-023`: Guide_Maker Forging Guide (`gamefaqs_guidemaker_28512.html` Section XVI, 2004-03-22, pRO): Confirms **1 Fruit + 1 Empty Bottle + 3z** for Apple, Carrot, Banana, and Grape Juice.
  - **Acolyte Ascetic Yosuke Location on `prt_fild00`:**
    - `T9-024`: H0bgoblin Acolyte/Priest FAQ (`gamefaqs_acolytepriest_h0bgoblin_18212.html` Section II, 2003-07-25, iRO): Priest #3 is located in East Geffen (2nd east screen) "in the center of the screen on a island" (`208,218`).
    - `T9-025`: `gat_cells.py` check on client `data.grf`: cell `prt_fild00 208,218` is walkable (type 0), while cell `208,255` is blocked water (type 5).
    - `T9-026`: eAthena 817 (`npc/jobs/acolyte.txt:267`): places Ascetic NPC on walkable island cell `prt_fild00 208,218`.
    - `T9-027`: mod0631 (`conf/npc_job_aco.txt:122`): places Ascetic NPC on blocked cell `prt_fild00 208,255`.

### T6: Dyeing (Threads 13, 29)
- **Status:** partly (kRO hair dyeing notice date remaining)
- **Rows:** `T6-012` and `T6-013`
  - `T6-012`: Blood3D Mixing Guide (`gamefaqs_mixing_blood3d_29037.html` Section V, 2004-02-28, pRO): Green Dye requires **1 Mixture**, 1 Counteragent, 20 Red Herb, 20 Yellow Herb, 5 Blue Herb, 1 Empty Bottle, 5000 zeny.
  - `T6-013`: Blood3D Mixing Guide: Violet Dye requires **30 Red Herb**, 10 Blue Herb, 1 Empty Bottle, 1 Counteragent, 1 Mixture, 5000 zeny.

### T10: Novice Training Ground Outside Japan (Thread 6)
- **Status:** partly (Fabre forest wake-up cell on knockout, Leo quiz questions remaining)
- **Rows:** `T10-010` and `T10-011`
  - `T10-010`: Mistiknoname3k Assassin Guide (`gamefaqs_assassin_mistiknoname3k_20164.html` v1.9, 2003-12-13, iRO): Fresh characters start with **1 Knife** and **1 Cotton Shirt**; quiz rewards depend on correct answers.
  - `T10-011`: Mistiknoname3k: Personality test at the end recommends a job, warps directly to that job's town (Morroc for Thief), and gives extra zeny if agreeing to recommended job.

### T8: Things Only Drafted from Client's Text (Thread 22)
- **Status:** partly (Frost Nova animation/screenshot, euRO Comodo balance patch date remaining)
- **Rows:** `T8-001` through `T8-003`
  - `T8-001`: Official iRO Guide (`iro_guide_jobwizskill_20040327.html`): Ice Wall shields 5 cells, consumes 50 wall durability every second, decreases durability by damage taken when attacked, and cancels when durability reaches 0 (400 to 2200 durability).
  - `T8-002`: Official iRO Guide: Sightrasher spreads fireballs in 8 directions with range +1 to +5 cells around caster, pushing back enemies.
  - `T8-003`: eAthena 817 (`skill.c:3169`): defines Sightrasher cell pattern as 24 cells along 8 radial rays reaching up to 3 cells from caster.

### T1: The 2-1 Job Tests (Threads 5, 8)
- **Status:** partly (post-patch test questions/lists remaining)
- **Rows:** `T1-001` through `T1-005`
  - `T1-001`: Weggy FAQ (`gamefaqs_weggy_20092.html`, mid-2003 base text): Knight changed for a 2500 zeny fee with no test.
  - `T1-002`: Edge Knight Guide (`gamefaqs_knight_29457.html` v1.0, 2004-02-08, pRO): Knight changed by paying a small fee with no test.
  - `T1-003`: Weggy FAQ: Assassin changed by paying 666 zeny to guildmaster with no test.
  - `T1-004`: Weggy FAQ: Blacksmith changed instantly by talking to man inside Geffen smith shop.
  - `T1-005`: Weggy FAQ: Wizard changed instantly by talking to Wizard at top of Geffen tower.

### T5: Turtle Island Quests (Thread 24)
- **Status:** partly (Alberta sailor fare/conditions and logbook errands remaining)
- **Rows:** `T5-009`
  - `T5-009`: Negative result (`none` row): RODataZone `maps/city/alberta.html` was not archived on Wayback Machine between 2003-12 and 2004-05 (Wayback CDX API returned 0 captures).

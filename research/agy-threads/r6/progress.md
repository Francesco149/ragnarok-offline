# Round 6 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T3 | euRO 2004 news pages: rates, patches, WoE, events, slots, wipe | Answered | T3-001 – T3-027 |
| T4 | Ailments and resistance formulas: VIT/LUK/MDEF, Storm Gust 3-hit freeze, poison 1/4 HP floor, durations | Answered | T4-001 – T4-018 |
| T5 | Turtle Island: arrival, Gotanblue fare/mechanics, logbook errands, headgear quests, doomed swords | Answered | T5-001 – T5-027 |

Total: 72 rows across 3 tasks.
Checker result (`check.txt`): 72 PASS, 0 WARN, 0 FAIL, 0 NONE (verified live with `--fetch`).

## Task Details and Row Mapping

### T3: euRO 2004 News Pages (Rates, Patches, WoE, Events, Slots, Wipe)
- **Status:** Answered
- **Rows:** `T3-001` through `T3-027`
  - **Closed Beta & Rates:**
    - Closed beta launched on December 20, 2003 with "higher experience" rate and explicit warning of character/account deletion: `T3-001`, `T3-002`
    - Gamona forum staff (Astartus) confirmed on 2004-01-27 that accounts will be wiped because EXP and drop rates were too high: `T3-003`
    - Player report confirmed Closed Beta was 3x rate and would drop to normal 1x for Open Beta: `T3-004`
  - **Patches & Updates:**
    - "New Comodo" update (2004-01-08) added Heal-Effect, modified potions, 2nd job quest, monster skills, lvl99 aura, fixed refining bug, and adjusted monsters HP/Exp: `T3-005`
    - Turtle Island update files in euRO patch list: entries 400 (`2003-04-30gdata_turtle_ph_1.gpf`) and 401 (`2003-04-30gdata_turtle_ph_2.gpf`): `T3-006`
    - Xmas patch present in euRO client binary (`Ragnarok.exe`): `2003-10-31gdata_xmas.gpf`: `T3-007`
    - Juno (Episode 5) update launched on June 9, 2004 (`T3-018`), adding 6 alternative 2-2 jobs, 10 new maps, 30+ monsters, weapon sprites, `/bm` battlemode, and weapon breaking (`T3-019`)
    - Amatsu & Gonryun update scheduled for installation on September 30, 2004: `T3-020`
  - **Wipe & Launch Schedule:**
    - Closed Beta servers shut down January 31, 2004 at 13:00 CET for character and item wipe: `T3-008`
    - Open Beta launch postponed to Wednesday, February 4, 2004 at 10:00 CET: `T3-009`, `T3-010`
    - Final wipe decision announced on April 8, 2004: characters preserved, while items, zeny, and castle ownership rolled back to April 8, 2004 at 09:00 CET: `T3-011`, `T3-012`
    - Commercial P2P launched April 15, 2004 with 15 days free trial: `T3-013`
    - Subscription pricing models: 30 hours for €5.95, 1 month for €11.95, up to 12 months for €99.95: `T3-014`
  - **War of Emperium & Events:**
    - Scheduled on Sundays at 18:00 CET with GM Castle Assault event: `T3-015`
    - Temporarily deactivated on September 3, 2004 due to spell bugs, disabling treasure boxes, emperium, and guild dungeons: `T3-016`
    - Reactivated on Chaos on September 22, 2004 with 3 additional zone servers: `T3-017`
    - In-game events: King of the Hill (`T3-026`) and Loading Screen design contest with 100 entries: `T3-025`
  - **Server Worlds & Infrastructure:**
    - 5 character slots per account introduced August 18, 2004: `T3-021`
    - Second server Loki launched September 16, 2004, enabling 10 character slots across Chaos and Loki: `T3-022`
    - Milestone of 100,000 registered users reached June 30, 2004 (`T3-023`) and 150,000 on August 14, 2004 (`T3-024`)
    - Outage report of August 27–28, 2004 confirms euRO server backend operated on Windows Server 2003 with Gravity database servers: `T3-027`

### T4: Ailments and Resistance Formulas (VIT/LUK/MDEF, Storm Gust, Poison, Durations)
- **Status:** Answered
- **Rows:** `T4-001` through `T4-018`
  - **Genesis of Resistance in jAthena (Commit `71d06d6`, 2004-01-21 by DRG):**
    - First appearance of stat-based resistance formulas in Athena:
      - Player VIT defense: `sc_def_vit = 100 - (sd2->paramc[2] + sd2->paramc[5]/3);`: `T4-001`
      - Player MDEF defense: `sc_def_mdef = 100 - sd2->mdef;`: `T4-002`
      - Player INT defense: `sc_def_int = 100 - (sd2->paramc[3] + sd2->paramc[5]/3);`: `T4-003`
      - Monster VIT defense: `sc_def_vit = 100 - (mob_db[md->class].vit + mob_db[md->class].luk/3);`: `T4-004`
      - Monster MDEF defense: `sc_def_mdef = 100 - mob_db[md->class].mdef;`: `T4-005`
  - **Duration Discounting in jAthena (Commit `188a761`, 2004-01-23 by ぴざまん):**
    - Changelog notes resistance reduces effect duration by the same proportion as trigger rate: `T4-006`
    - Stone Curse duration changed from permanent to 2channel Mage Thread Template (`マジスレテンプレ準拠`): `T4-007`
  - **eAthena 817 Formulas (`skill.c`):**
    - Standardized 817 defense formulas (with +3 base subtraction):
      - MDEF: `sc_def_mdef = 100 - (3 + battle_get_mdef(bl) + luk/3);`: `T4-008`
      - VIT: `sc_def_vit = 100 - (3 + battle_get_vit(bl) + luk/3);`: `T4-009`
      - INT: `sc_def_int = 100 - (3 + battle_get_int(bl) + luk/3);`: `T4-010`
      - LUK: `sc_def_luk = 100 - (3 + luk);`: `T4-011`
    - Branching calculation between PC and MOB targets: `T4-012`
  - **Specific Mechanics in 817:**
    - Storm Gust 3-hit freeze: tracks `sg_count++`; when `sg_count >= 3` on non-undead targets, triggers freeze: `T4-013`
    - Poison HP floor: damage ticks require current HP to strictly exceed one quarter of max HP (`battle_get_hp(bl) > hp>>2`): `T4-014`
    - Stone Curse MDEF reduction: MDEF discounts duration by `battle_get_mdef(bl) * 200` ms: `T4-015`
  - **Contrast with mod0631 (2003-12-18):**
    - Checks only card resistance array (`reseff[type-SC_STONE]`) without stat formulas: `T4-016`
    - Console output confirms card resistance: `T4-017`
    - Status changes used 10-minute placeholder durations (`tick = 1000 * 600;`): `T4-018`

### T5: Turtle Island (Passage, Logbook Errands, Doomed Swords, Headgear Quests)
- **Status:** Answered
- **Rows:** `T5-001` through `T5-027`
  - **Passage to Turtle Island:**
    - Official quest log 11029: directs player to find an old ferryboatman on eastern Alberta port: `T5-001`
    - Old Man Kamezizii at `alberta_in 23,104` unlocks passage by setting `turtle_npc,1`: `T5-002`
    - Sailor Gotanblue at `alberta 247,122` checks `turtle_npc == 1`: `T5-003`
    - Gotanblue charges 10,000 Zeny: `T5-004`
    - Gotanblue warps player to `tur_dun01 157,39`: `T5-005`
    - Return trip: sailor at `tur_dun01 165,29` warps back to `alberta 241,115` free of charge: `T5-006`
  - **Logbook Errands:**
    - Logbook NPC at `alberta 248,94`: `T5-007`
    - Skull tombstone at `tur_dun01 160,81` awards 5 Bananas: `T5-008`
    - Turtle pillar at `tur_dun02 49,156` awards 1 Red Gemstone: `T5-009`
    - Official quest log 11030 clue to `tur_dun01 160,81`: `T5-010`
  - **Doomed Swords Smith at `pay_fild08 218,283`:**
    - Smith NPC header: `T5-011`
    - Grimtooth: script deletes 1 Cursed Ruby and 1 Blade Lost in Darkness (`T5-012`), despite dialogue asking for 5 rubies and 10 blades
    - Mysteltain: dialogue requests 1 Young Twig, 1 Loki's Whispers, 1 Mother's Nightmare, 1 Blind Man's Deceit, and 1 Emperium (`T5-013`)
    - Executioner: checks 2 Skeleton Armor Pieces (`T5-014`), deletes 10 Broken Heroic Emblems (`T5-015`), and 1 Emperium (`T5-016`)
  - **Headgear Quests (Comodo / Turtle Island Patch):**
    - Welding Mask (Geffen): 50 Steel, 2,000z (`T5-017`)
    - Pretend to be Murdered Hat (Payon): 4 Skirts of Virgin (`T5-018`)
    - Ear Muffs (Payon): 1 Cursed Ruby, 200 Feathers, 1 Headset, 5,000z (`T5-019`)
    - Antler (Alberta): 20 Evil Horns (`T5-020`)
    - Stop Post (Morroc): 50 Wooden Blocks, 1 Black Dyestuff, 91,100z (`T5-021`)
    - Doctor Band (Aldebaran): 1 Red Bandana, 50 Iron, 3,500z (`T5-022`)
    - Feather Bonnet (Aldebaran): 1 Romantic Gent, 300 Birds Feathers, 500z (`T5-023`)
    - Boy's Cap (Alberta): 10 Tiger Footskins (`T5-024`)
    - Ph.D Hat (Prontera): 1 Illusion Flower, 1 Hinalle, 1 Aloe, 1 Ment (`T5-025`)
    - Oxygen Mask (Payon): 5 Ora Ora (`T5-026`)
    - Binoculars (Morroc): 1 Geek Glasses, 100 Steel, 50,000z (`T5-027`)

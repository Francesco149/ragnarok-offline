# Round 11 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | The 2-1 job tests: dating and sharpening them (threads 5, 8) | partly (Wizard test items, rooms, failure penalty and skip answered; Knight pRO status answered; Priest euRO launch answered; Knight Sir Andrew/Sir James/Sir Grey questions, Blacksmith, Assassin walkthroughs before June 2004 remaining) | T1-001 – T1-009 |
| T5 | Turtle Island: doomed swords smith and headgear quests (threads 12, 24) | partly (Doomed swords smith lists answered: Mysteltain 1 Young Twig, Grimtooth 5 Cursed Ruby and 10 Bloody Edge, Executioner; Alberta headgear quests: Boy's Cap, Antler, and Sea Otter Raccoon Hat answered; official Gotanblue fare and logbook banana/gemstone remaining) | T5-001 – T5-007 |
| T6 | Dyeing NPCs, terms, and dye formulas (thread 13) | answered | T6-001 – T6-007 |
| T7 | The 2-2 skills across 817 and contemporary player guides (threads 22, 25) | answered | T7-001 – T7-017 |
| T8 | Comodo skill balance patch dating (threads 22) | partly (iRO Comodo patch dated to Oct 12, 2003; pRO Comodo patch dated to Dec 17, 2003; euRO New Comodo dated to Jan 18, 2004; Ice Wall HP, Poison React/Remove Trap/Spring Trap/Venom Splasher details, Sightrasher reach, Frost Nova animation remaining) | T8-001 – T8-003 |
| T9 | Small, independent (threads 7, 14, 17, 24) | partly (Charge Arrow Crossbow returned without being consumed answered; Gershaun Banana Juice answered; Arrow Crafting NPC in Morroc 122,109 answered; Bongun taming item Her Heart in Comodo answered; WoE castle steward options, investment cost doubling, trade/defense -2 on capture, 1/250 godly material drop rate, castle drop assignments, guild dungeon spawns/respawns, WoE restrictions answered; Thief pay/Acolyte Yosuke/Merchant parcel, Shockwave Trap count, Clock Tower random warps, places world audit cannot reach remaining) | T9-001 – T9-020 |

Total: 63 rows across 6 tasks.
Checker result (`check.txt`): Verified with `python3 research/check_rows.py --fetch`.

## Task Details and Row Mapping

### T1: The 2-1 Job Tests (Threads 5, 8)
- **Status:** partly
- **Rows:** `T1-001` through `T1-009`
  - Wizard test walkthrough (Ghent Mage/Wizard Guide, FAQ 30054, v2.1 2004-07-30):
    - 10 blue, 10 red, 10 yellow gemstones required if under job 50 (`T1-001`).
    - Alternative item set: 5 green lives, red bloods, crystal blues, winds of verdure (`T1-002`).
    - Combat test consists of three rooms (`T1-003`).
    - Water Room gatekeeper is a Marine Sphere (3518 HP, self-destruct fails the test) (`T1-004`).
    - Fire Room gatekeeper is Horongs that walk through firewall (`T1-005`).
    - Failure teleports and sets save point outside Geffen Tower (`T1-006`).
    - Failing 5-6 times offers an option to skip the combat test for a Worn Out Scroll (`T1-007`).
  - Knight test status in February 2004:
    - Edge Knight/Crusader Guide (FAQ 29457, v1.0 2004-02-08) confirms pRO had no quest, only a fee at Culvert Headquarters (`T1-008`).
  - Priest test status on euRO:
    - euRO forum announcement topic 1791 (2004-01-18) player post confirms 2nd job quest active with New Comodo and buggy for Priest on launch day (`T1-009`).

### T5: Turtle Island Doomed Swords & Headgear (Threads 12, 24)
- **Status:** partly
- **Rows:** `T5-001` through `T5-007`
  - Doomed swords smith (blood3d Mixing Item Quests FAQ, FAQ 29037, v1.01 2004-03-06):
    - Mysteltain requires 1 Young Twig (`T5-001`).
    - Grimtooth requires 10 Bloody Edge and 5 Cursed Ruby (`T5-002`).
    - Executioner requires 2 Executioner's Mitten, 10 Bloody Edge, 3 Necklace of Oblivion, 50 Amulet, 1 Emperium (`T5-003`).
    - Legendary Blacksmith located at north east eye of willow map east of Payon (`pay_fild08`) (`T5-004`).
  - Headgear quests in Alberta:
    - Boy's Cap: 10 Tiger's Footskin, old man NPC near east of upper entrance in Alberta (`T5-005`).
    - Antler: 20 Evil Horn, NPC with mini glasses on second floor of big mansion in Alberta (`T5-006`).
    - Raccoon Hat: 300 Skin of Sea Otter (`T5-007`).

### T6: Dyeing NPCs & Formulas (Thread 13)
- **Status:** answered
- **Rows:** `T6-001` through `T6-007`
  - NPC identification and locations (ArislanX Alchemist Guide, FAQ 31925, v1.0 2004-08-20):
    - Louitz in Alberta at `133,55` (`T6-001`).
    - Aure Dupon in Geffen at `181,114` (`T6-002`).
    - Molgensten / Morgenstein upstairs in Geffen Blacksmith Shop at `141,140` (`T6-003`).
  - Chemical and Dye recipes (blood3d Mixing Item Quests FAQ, FAQ 29037, v1.0 2004-02-28):
    - Counteragent fee 3,000 zeny (`T6-004`).
    - Mixture fee 4,000 zeny (`T6-005`).
    - Black Dye fee 7,000 zeny in North East Building of Morroc (`T6-006`).
    - Blue Dye fee 3,500 zeny (`T6-007`).

### T7: The 2-2 Skills (Threads 22, 25)
- **Status:** answered
- **Rows:** `T7-001` through `T7-017`
  - Strip Skills (Calx Zinra Rogue Guide, FAQ 29854, v2.8 2004-04-16):
    - Strip Weapon reduces monster Attack by 10%; DEX difference affects success chance (`T7-001`).
    - Strip Weapon success chance 7% to 15%, SP 17 to 25 (`T7-002`).
    - Strip Helm reduces monster DEF by 10% and INT by 40% (`T7-003`).
    - Strip Armor reduces monster DEF by 10% and VIT by 40% (`T7-004`).
    - Strip Helm SP cost 12 to 20 (`T7-005`).
  - Sage Skills (ReiCA Zinra Sage Guide, FAQ 30168, v1.4 2004-05-02):
    - Volcano lasts 3 minutes, requires 1 Yellow Gemstone, blocks Ice Wall (`T7-006`).
    - Volcano SP 52 to 60, radius 5x5 to 9x9, +10% to +20% Fire weapon ATK (`T7-007`).
    - Deluge allows use of Water Ball and Aqua Benedicta (`T7-008`).
    - Violent Gale causes Fire Wall to last 1.5 times longer (`T7-009`).
    - Land Protector duration 15 to 75 seconds, requires 1 Blue and 1 Yellow Gemstone (`T7-010`).
    - Magic Rod consumes 2 SP, deals 20% damage on SP amount if countering Spell Breaker (`T7-011`).
    - Dispell chance 60% to 100%, requires 1 Yellow and 1 Red Gemstone (`T7-012`).
  - Crusader Grand Cross (Tangowr Crusader Guide, FAQ 28552, v1.0 2004-07-01):
    - SP cost 30 + Skill level x 7, damage 140% to 500% (`T7-013`).
    - Self-damage reflects 2/3 of total damage back toward caster (`T7-014`).
  - Alchemist Skills (ArislanX Alchemist Guide, FAQ 31925, v1.0 2004-08-20):
    - Bomb consumes 10 SP and 1 Bottle Grenade, creates 1 cell Fire Wall, lowers monster ATK by 25% (`T7-015`).
    - Acid Terror consumes 15 SP and 1 Acid Bottle, lowers monster DEF by 25% (`T7-016`).
    - Aid Potion consumes 1 SP per use (`T7-017`).

### T8: Comodo Skill Patch Dating (Threads 22)
- **Status:** partly
- **Rows:** `T8-001` through `T8-003`
  - iRO Comodo Patch updates dated to October 12, 2003 (Ghent Mage/Wizard Guide v0.9b) (`T8-001`).
  - pRO Comodo Patch quest skills release dated to December 17, 2003 (Battousai X Quest Skills FAQ v1.0) (`T8-002`).
  - euRO New Comodo update dated to January 18, 2004 (euRO forum announcement topic 1791) (`T8-003`).

### T9: Small, Independent (Threads 7, 14, 17, 24)
- **Status:** partly
- **Rows:** `T9-001` through `T9-020`
  - Charge Arrow Crossbow is not taken/consumed (Battousai X FAQ 27646 v1.3 & Godhand FAQ 26381 v1.3) (`T9-001`, `T9-002`).
  - Gershaun requires Banana Juice to explain ingredients but omitted if already collected (Battousai X FAQ 27646 v1.0) (`T9-003`).
  - Arrow Crafting NPC located at Morroc 122,109 (Godhand FAQ 26381 v1.3) (`T9-004`).
  - Bongun taming item Her Heart created in Comodo (blood3d FAQ 29037 v1.01) (`T9-005`).
  - War of Emperium (Arctic Breeze WoE Guide, FAQ 28786, v1.0 2004-02-18):
    - Castle trade rating goes down by 2 on capture; max 2 investments per day, cost doubles each time (`T9-006`).
    - Castle Kafra storage free of charge for guild mates (`T9-007`).
    - Godlike equipment material drop rate from treasure boxes is 1/250 (0.4%) (`T9-008`).
    - Mjolnir material Thor's Gloves from Al de Baran Castle 2 (`T9-009`).
    - Brisingamen material Freya's Jewel from Al de Baran Castle 1 (`T9-010`).
    - Payon guild dungeon boss Eddga respawns 8 hours after death (`T9-011`).
    - Guild members recover HP/SP faster within their castle (`T9-012`).
    - Death in WoE incurs no experience loss (`T9-013`).
    - Ice Wall cannot be used in WoE (`T9-014`).
    - Pushback skills and Thief Backslide disabled in WoE (`T9-015`).
    - Teleport and Fly Wing disabled in WoE (`T9-016`).
    - Phen card disabled in WoE (`T9-017`).
    - Sanctuary heals the Emperium (`T9-018`).
    - Base guild capacity is 16; Guild Extension increases up to 36 (`T9-019`).
    - Official iRO siege schedule: Saturday 1900-2100 and Wednesday 1500-1700 PST (`T9-020`).

# Deep Research Report: euRO & Period Ragnarok Online Open Threads (Dec 2003 – Feb 2004)

**Date:** 2026-11-09  
**Scope:** Historical investigation of Priority 1 and Priority 2 research threads for `ragnarok-offline` targeting the 2003-12-18 euRO client (`Ragexe.exe`) and the Dec 2003 – Feb 2004 closed/open beta era.

---

## 1. Summary Table

| Thread | Short Answer | Confidence | Key Sources |
| :--- | :--- | :--- | :--- |
| **#17: WoE in Single Player** | Castle ownership gated daily Master Room chests (`floor(Eco/5) + 4`, 4–24 boxes at 00:01 dropping 20 God Item parts at 0.10%), realm guild dungeons (`gld_dun01..04` with 8h MVPs Eddga, Doppelganger, Maya, Dark Lord), castle Kafra, guardians, and investments. Recommend Option A: Neutral garrison assault with permanent steward and weekly raid defense [1][2][6][7][8]. | **High** | iRO WoE Guide [1]; kRO Notices [5]; rAthena `agit_main.txt` [6]; rAthena `gld_dun.txt` [7]; iRO Classic DB [8]. |
| **#6: Start Cell & Training Ground** | Server spawned Novices at `new_1-1.gat, 53, 111` facing North. Client `data/resnametable.txt` transparently redirected `new_1-1.gat` to `new_zone01.gat`. Cell (53, 111) is walkable (type 0, altitude -7.81). Flow: Knife/Cotton Shirt, Usher 50z subsidy, lectures, Fabre Forest, aptitude test with 100–500z, 10 Red Potions, 1 Milk, and job guild warp [10][11][12][13][14]. | **High** | jAthena `Readme` [10]; jAthena `char.c` [11]; euRO `data.grf` [12][13]; kRO Notice [14]; 817 `novice.lua` [16]. |
| **#5: 2-1 Job Changes before New Comodo** | euRO Early Closed Beta (Dec 20, 2003 – Jan 8/18, 2004) had instant 2-1 advancement at Job 40 with zero quests. Quests arrived with New Comodo on Jan 8/18, 2004 in Late Closed Beta (Jan 8–31) and continued into Open Beta (Feb 4+). King Tristram III decree was emulator placeholder prose (RoVeRT/Darkchild), not official Aegis dialogue [18][19][21][22][23]. | **High** | euRO News [18]; euRO Forum t=1791 [19]; 817 Scripts [21][22]; iRO Notices [25][26]; SEC Agreement [28]. |
| **#1: euRO Closed-Beta Rates** | Closed beta operated with boosted experience (officially announced) and boosted drops (community acknowledged), but BHI-OE never published an exact integer multiplier. Speculation was 2x–5x. Open Beta (Feb 4, 2004) and Commercial (April 15, 2004) ran at stock 1x/1x/1x rates; Open Beta characters carried over without a wipe [29][30][31][32][33]. | **High** (Timeline/1x) / **None** (CB Int Multiplier) | BHI-OE Announcements [29]; Gamona RnS [30]; euRO Forum t=2400 [31]; euRO Forum t=2539 [32]; euRO News 00054 [33]. |
| **#2: Official English NPC Text** | Official Gravity dialogue survives in OpenKore navigation tables, June 2004 iRO Bug Board captures (`bugcontent.asp`), period GameFAQs guides, and rAthena Aegis 10.3 conversions. eAthena 817 used amateur fan translations (Tono: "another monster metamorphic" vs Gravity: "awkward puberty") [37][38][39][40][41][42]. | **High** | OpenKore `portals.txt` [37]; iRO Bug Board [38][39][40]; SoulEclipse FAQ [41]; rAthena Aegis conversions [42][43]. |
| **#3: Kafra Fares before 2004-02-24** | Prontera→Geffen was 1400z (817's 1800z deduction was an emulator copy-paste bug). Alberta→Prontera was 2100z (817's 1800z menu was a display typo). Prontera→Alberta did not exist. Storage was tiered by city (Prontera 40z, Geffen/Izlude 30z, Alberta 50z, dungeons 80–130z). Izlude inside-town save cell was `izlude (94, 103)` [45][46][47][48][49]. | **High** | kRO Notice 1182 [45]; jAthena `npc_town_kafra.txt` [46]; 817 `kafra.lua` [47]; rAthena `kafras.txt` [48]; OpenKore `npcs.txt` [49]. |
| **#4: Kafra Points & Pass** | Disabled across official servers in late 2003. kRO removed the voucher menu on 2003-05-20. Item 1084 was flagged unusable in client text. 817's Sweet Potato lottery was a broken emulator script. Keep disabled [50][51][52][53]. | **High** | kRO Notice 601 [50]; Client Item DB [51]; 817 `kafra.lua` [52]; rAthena `kafras.txt` [53]. |
| **#7: 1st-Class Quest Skills** | All 16 platinum skills were active on official servers by Jan 2004. Official NPCs: Alcouskou (`moc_prydb1 154, 128`) for all 4 Thief skills; RuRumuni (`payon 91, 77`) for bag; Juan (`prt_in 94, 57`) for Auto Berserk. 817's "Sono-san" at `morocc 165, 124` was a broken emulator invention [54][55][56][57][58]. | **High** | kRO Notice 209 [54]; iRO Acolyte Guide [55]; kRO Quest Portal [56]; SoulEclipse FAQ [41]; RMS Thief Quest [58]. |
| **#8: Wizard & 2-1 Quests** | Wizard test had 3 stages: Item collection (skipped at Job 50), Raul Expagarus written exam (90% pass), and 3-minute combat gauntlet in `job_wiz` (Water, Earth, Fire) with a 4-fail Worn Out Scroll bribe bypass. 817 had syntax bugs; eathena-v0.5.2 preserves the full authentic script [40][59][60][61]. | **High** | iRO Bug 4218 [40]; kRO Notice 1079 [59]; RMS Wizard Quest [60]; eathena-v0.5.2 `wizard.txt` [61]. |
| **#9: Refining Details** | Phracon was 200z and Emveretarcon 1,000z (iRO website 2000z was a typo). Armor refine grants ~+0.7 DEF per level (`sd->def += (refinedef + 50) / 100`). Upgrade failure destroys item, zeny, and all slotted cards. Weapon Lv 4 fee was 20,000z [62][63][64][65][66][67]. | **High** | iRO Refine Guide [62]; kRO Notice 360 [63]; RoboShop FAQ [64]; rAthena `refine_db.txt` [65]; rAthena `pc.c` [66]. |
| **#12: Map Flags** | Client `indoorrswtable.txt` defines 31 indoor maps blocking teleport. Castles block teleport, memo, and branches. Dungeons block memo (74 maps). mod0631's `nopenalty` on outdoor towns was an emulator anti-griefing invention; official servers applied normal death penalty in towns [12][68][69][70]. | **High** | euRO `indoorrswtable.txt` [12]; mod0631 `mapflag.txt` [68]; 817 `mapflag/` [69]; rAthena `nopenalty.txt` [70]. |
| **#14: Pets on euRO & iRO** | Official guide listed 24 pets (Eggs 9001–9024); mod0631/817 included 25 pets adding Bongun (Egg 9025, *Her Heart*) via Comodo quest. Pet speech strings preserved in June 2004 iRO Bug #4201 [38][71][72][73]. | **High** | iRO Pet Guide [71]; kRO Notice 224 [72]; 817 `pet_db.txt` [73]; iRO Bug 4201 [38]. |
| **#18: Izlude Arena** | Did not exist on euRO or iRO in Dec 2003 – Feb 2004. Teased on kRO in June 2004, Sakray in Sept 2004, and released on kRO main servers on October 12, 2004. 817 scripts were custom Prontera tests. Disable for late-2003 accuracy [74][75][76][77][78]. | **High** | kRO Notice 1785 [74]; kRO Notice 1189 [75]; kRO Notice 1742 [76]; 817 `arena.lua` [78]. |
| **#19: Lutie Town Permanence** | Lutie was a permanent official town since Episode 2 (Dec 17, 2002). euRO CD patcher (`Ragnarok.exe`, built 2003-12-12) executed a local check for `2003-10-31gdata_xmas.gpf` and halted if missing when `patch_allow.txt` returned `"xmas"`. Unactivated template version gate, not an automated download [12][79][80][81]. | **High** | kRO Episode 2 [79]; euRO `data.grf` [12]; euRO `Ragnarok.exe` disassembly [80]; Ai4rei Patcher Docs [81]. |

---

## 2. Priority 1 Threads

### Thread #17: War of Emperium and Guild Castles in Single Player

#### Findings
- **[READ] (iRO, 2004-01-01 / 2004-04-05)** The official iRO guide establishes that castle ownership granted exclusive access to the Master Room: *"Enter Master Room: There is a room that is only open to the Guild master. In this room. 1 rare item will be given once a day, depending on the Commerce Development Investment of the Guild"* [1].
- **[READ] (kRO / Emulator lineage, 2003–2004)** In Aegis and rAthena script code, daily chest generation executed at midnight (`OnClock0001:`), checking guild ownership (`GetCastleData(..., CD_GUILD_ID)`), and calculating chest count via `floor(Current_Economy / 5) + 4` [6]. For Economy 0 to 100, this spawned 4 to 24 chests daily [6].
- **[READ] (euRO, 2003-12-18 / Database)** Monster database entries define 40 distinct Treasure Chest monsters (IDs 1324–1363), allocating 2 alternating IDs per castle across all 20 First Edition (FE) castles [3][6]. Universal common drops included Yggdrasil Berry (10%), Yggdrasil Seed (20%), Mastela Fruit (45%), Elunium (20–25%), Oridecon (20–25%), and Tribal Solidarity (5%) [3][8].
- **[READ] (kRO / iRO, 2003–2004)** The 20 unique chest IDs dropped the 20 distinct ingredients required for Tier 1 God Items (Sleipnir, Megingjard, Brisingamen, Mjölnir) at a documented drop rate of **0.10% on official Gravity Aegis servers** [8][9], configured at **0.80% in default eAthena/rAthena monster databases** [3][6]. The components comprise:
  - *Luina (Al De Baran)*: Luina 1 (`aldeg_cas01` / Box 1325) dropped Freya's Jewel (7073); Luina 2 (1327) dropped Thor's Gauntlets (7074); Luina 3 (1329) dropped Iron Maiden (7075); Luina 4 (1331) dropped Wheel of the Unknown (7076); Luina 5 (1333) dropped Silver Ornament (7077) [3][6][8].
  - *Britoniah (Geffen)*: Britoniah 1 (`gefg_cas01` / Box 1335) dropped Wrath of Valkyrie (7078); Britoniah 2 (1337) dropped Feather of Angel Wing (7079); Britoniah 3 (1339) dropped Cat Tread (7080); Britoniah 4 (1341) dropped Woman's Moustache (7081); Britoniah 5 (1343) dropped Root of Stone (7082) [3][6][8].
  - *Baldur (Payon)*: Baldur 1 (`payg_cas01` / Box 1345) dropped Spirit of Fish (7083); Baldur 2 (1347) dropped Sputum of Bird (7084); Baldur 3 (1349) dropped Sinew of Bear (7085); Baldur 4 (1351) dropped Emblem of the Sun God (7086); Baldur 5 (1353) dropped Breath of Spirit (7087) [3][6][8].
  - *Valkyrie (Prontera)*: Valkyrie 1 (`prtg_cas01` / Box 1355) dropped Snow Crystal (7088); Valkyrie 2 (1357) dropped Omen of Tempest (7089); Valkyrie 3 (1359) dropped Ripple (7090); Valkyrie 4 (1361) dropped Billow (7091); Valkyrie 5 (1363) dropped Drifting Air (7092) [3][6][8].
  - Chests also dropped rare equipment and headgears: Helm of Angel (5025), Romantic Gent (2247), Hat of the Sun God (5022), Corsair (5019), Bone Helm (5017), Grand Circlet (5027), Spiky Band (5053), Masamune (1165), Brocca (1417), Infiltrator (1260), and Ice Pick (1256/1238) [3][8].
- **[READ] (kRO / iRO, 2003–2004)** Access to Guild Dungeons (`gld_dun01`..`04`) was operated by a hidden lever inside each castle (`"There's a small lever. Will you pull it?"`), requiring `getcharid(2) == GID` [6]. All five castles of a realm warped into that realm's shared dungeon: Payon castles to `gld_dun01`, Al De Baran to `gld_dun02`, Prontera to `gld_dun03`, and Geffen to `gld_dun04` [6][7]. The dungeons functioned as realm-wide PvP zones with standard 1% death EXP penalty [5][7].
- **[READ] (kRO / iRO, 2003–2004)** Guild dungeons featured four realm MVPs on strict **8-hour respawn timers** (+10 minute variance): Eddga in `gld_dun01`, Doppelganger in `gld_dun02`, Maya in `gld_dun03`, and Dark Lord in `gld_dun04`, alongside 4-hour minibosses (Vagabond Wolf in dun01, Ghostring in dun04) [7]. Exclusive card drops included Maya Purple Card (1289, detects cloaked/hidden enemies), Dark Illusion Card (1302, MaxHP/SP -10%, Cast Time -10%), Owl Baron Card (1295), Cat o' Nine Tails Card (1307), and Am Mut Card (1301) [3][7].
- **[READ] (kRO / iRO, 2003–2004)** Castle Butler NPCs managed daily Economy and Defense investments [1][6]. Economy investments cost 5,000z at level 1 up to 955,000z at level 100; Defense investments cost 10,000z at level 1 up to 1,910,000z at level 100 [6]. Investments were limited to 2 per day, with the 2nd costing 4× zeny [6]. Defense added +1,000 HP per point to castle Guardians and the Emperium [6]. Castle Kafra provided free storage, 200z local realm teleport, and 800z pushcarts [6].
- **[READ] (kRO, 2003–2004)** Guild skills were acquired entirely via guild tax leveling (1%–50% EXP tax), and none required a castle to learn; however, the effects of `Contract with Kafra` (10001), `Guardian Research` (10002), `Strengthen Guardians` (10015), and `Permanent Development` (10014, 50% chance for +1 extra economy) functioned only when possessing a castle [6].
- **[READ] (kRO, 2004-02-10 & 2004-02-17)** Official kRO patch notices document that in late 2003, unoccupied and abandoned castles spawned large hordes of aggressive monsters, which players actively farmed for rapid leveling until patched out in February 2004 [5]. While the official notices describe these spawns as hordes without itemizing specific species, contemporary emulator scripts and player recollection identify the garrison roster as Guardian Knights, Masters, Abysmal Knights, and Khalitzburgs [5][6].
- **[READ] (euRO, 2004-04-09)** euRO officially staged a PvE GM assault against player-held castles during WoE on Sunday, April 11, 2004 at 18:00 CET [4].

#### The Answer as We See It
In a single-player offline environment, standard WoE cannot function because there are no rival guilds to attack or defend castles, leaving all 20 castles unoccupied (`GID == 0`) and locking out the Butler, Master Room daily chests, and Guild Dungeon levers [6].

We present three design options for single-player implementation:
- **Option A: "Neutral Garrison Castle Raid & Permanent Steward" (Recommended).**
  - *Mechanics:* Castles spawn the pre-February 2004 defending monster garrison (Guardian Knights, Masters, Raydric Archers, Khalitzburgs) [5][6]. The solo player forms a personal guild, attacks the castle, defeats the garrison, and shatters the Emperium to claim ownership [1][6]. Once owned, the player accesses the Butler for authentic daily zeny investments (acting as the premier end-game zeny sink: up to 2.86m zeny daily), unlocks the Castle Kafra, pulls the lever to explore the realm's Guild Dungeon, and harvests daily Master Room chests at 00:01 [6][7]. A scheduled weekly invasion (e.g. Sunday 18:00 CET matching euRO) tests castle defenses [4].
  - *Provenance:* ~90% authentic period behavior (uses official maps, NPC dialogue, investment math, mob tables, and drop rates); only the solo claim trigger is adapted.
- **Option B: "Realm Key / Royal Warrant Quest" (Bypass).**
  - *Mechanics:* A town NPC in each realm sells or quests access to `gld_dun01`..`04` for high-tier boss trophies. Chests are traded via an exchange NPC.
  - *Provenance:* ~40% authentic; completely bypasses castle maps, investments, Butler dialogue, and guild skills.
- **Option C: "Single-Player WoE Training Grounds" (Instanced Trial).**
  - *Mechanics:* Adapted from kRO's April 2004 Novice WoE (`nguild_*`) and later Training Edition, offering timed trial gauntlets awarding daily chests and 24-hour dungeon passes [5].
  - *Provenance:* ~60% authentic assets; trial structure is adapted from later eras.

#### What is Still Unknown
Whether the exact monster composition of the unoccupied castle garrison in early 2003 kRO Aegis code included mini-bosses like Eddga or was restricted strictly to undead knights and guardians [5].

#### Next Queries and Sites
1. Korean archival searches on Daum cafes for `초보자 공성전 몬스터` (Novice siege monsters) and `아지트 몹소환` (castle mob summons).
2. Inspection of early Aegis server leak discussions on rAthena/eAthena boards (purely for spawn list confirmation, without incorporating prohibited code).

---

### Thread #6: The Start Cell and Novice Training Ground on this Client

#### Findings
- **[READ] (euRO, 2003-12-18)** Analysis of `data.grf` on the euRO December 2003 client CD demonstrates that the file `data/resnametable.txt` contains explicit resource redirections:
  ```
  new_1-1.gat#new_zone01.gat#
  new_2-1.gat#new_zone01.gat#
  new_3-1.gat#new_zone01.gat#
  new_4-1.gat#new_zone01.gat#
  new_5-1.gat#new_zone01.gat#
  new_1-2.gat#new_zone02.gat#
  new_1-3.gat#new_zone03.gat#
  new_1-4.gat#new_zone04.gat#
  ```
  The user-facing map name table `data/mapnametable.txt` maps `new_1-1.rsw` through `new_5-1.rsw` to `"Training Ground"` [12]. Physical assets `new_1-1.gat` do not exist in `data.grf`; the client transparently rendered `new_zone01.gat` when instructed to load `new_1-1.gat` [12][13].
- **[READ] (euRO, 2003-12-18)** In `new_zone01.gat` (dimensions 200×200), cell `(53, 111)` is confirmed walkable (cell type 0, altitude -7.81), located dead center on the western grassy bridgehead platform (spanning x=45..66, y=109..118) aligned with the east-west bridge axis (y=111) [12]. The Bulletin Board at `(66, 114)` sits on cell type 1 (wall railing) [12].
- **[READ] (jRO / Emulator lineage, 2003-03 to 2003-12)** The jAthena changelog (`Readme-jap`) dates the introduction of this coordinate:
  - Snapshot `//0305`: `"新規PCの位置を初心者修練場に変更。"` ("Changed new PC location to Novice Training Ground") by developer `いど` (Ido) [10].
  - Snapshot `//0307`: `"新規PCの初期位置をchar_athena.cnfに書けるようにした start_point: new_1-1.gat,53,111"` by developer `胡蝶蘭` (Phalaenopsis) [10].
  - Snapshot `//0631`: hardcoded in `char.c` as `struct point start_point={"new_1-1.gat",53,111};` [11].
- **[READ] (kRO, 2004-06-01 / Episode 12)** When Gravity officially added the tutorial NPC Sprakki, she was placed at cell `(53, 114)` — exactly three cells north of `(53, 111)`, directly facing the player spawn point [14][17].
- **[READ] (kRO / iRO / euRO, 2003-12-18)** Starting equipment in Dec 2003 was strictly 1x Knife (ID 1201) and 1x Cotton Shirt (ID 2301) [11]. Novice-specific gear (Novice Plate, Cutter, Guard) and direct level-ups from dialogue did not exist; they were officially added on kRO on 2004-06-01 (notice 1447) [14].
- **[READ] (jRO / kRO / 817, 2003–2004)** The official training ground flow was:
  1. Usher at `new_1-2 100, 29` offered an immediate town warp (`rand(6)` across Prontera, Morroc, Geffen, Payon, Alberta, Izlude) or registration with a **50z subsidy** (eAthena 817 had an emulator bug executing `zeny += 150`, but its dialogue and Japanese records confirm 50z) [15][16].
  2. 1st Course: Newbie Instructor (lecture, 1 Milk) and Leo Von Frisch (quiz, basic fruits/potions) [16].
  3. 2nd Course: Fabre Forest combat test (`new_1-3`) [16].
  4. Aptitude Test: Helper at `new_1-4 60, 149` graded scores, recommended a job, and awarded 100–500z, 10 Red Potions, 1 Milk, and a free warp/save directly to the chosen first-job guild [16].

#### The Answer as We See It
Newly created characters in Dec 2003 officially spawned on the Novice Training Ground at `new_1-1.gat, 53, 111` facing North [10][11][12]. The server should transmit `new_1-1.gat, 53, 111` (or `new_zone01.gat, 53, 111`), starting with 1 Knife and 1 Cotton Shirt [11][12]. The Usher reward must be set to 50z, correcting 817's 150z script bug [15][16].

#### What is Still Unknown
Whether the official Aegis server used an exact 1-cell coordinate `(53, 111)` or applied a 1–2 cell random spawn scatter (e.g. `x=51..54, y=110..112`) upon character creation [12][17].

#### Next Queries and Sites
1. Packet capture logs from official iRO beta (2002–2003) for packet `0x0071` / `0x0091` containing initial spawn map and coordinates.

---

### Thread #5: 2-1 Job Changes on euRO before New Comodo

#### Findings
- **[READ] (euRO, 2004-01-08 / 2004-01-18)** The official euRO website news and forum announcements by Director PerrI establish that 2-1 job quests were introduced with the "New Comodo" update (dated January 8, 2004 on the website, posted January 18 on forum `viewtopic.php?t=1791`): *"New in 'New Comodo' ... 2. added 2nd job quest"* (Italian: *"2. Aggiunte le quest per diventare seconda classe(wizard,priest,etc)"*) [18][19].
- **[READ] (euRO, 2004-01-18)** In forum topic 1791, player Condor posted on Jan 18, 2004: *"very good the 2nd job Quest but buggy, i can't be a Priest \*sigh\* but very good ^^ i like the Monsterskills"*, confirming the quests were newly activated and initially unstable [19].
- **[READ] (euRO, 2003-12-20 to 2004-01-08)** Following the Closed Beta launch on December 20, 2003 [20], during the early closed-beta weeks prior to New Comodo, players advanced to Knight, Priest, Wizard, Blacksmith, Hunter, or Assassin instantly upon reaching **Job Level 40** by speaking directly to the job guild NPC, with zero item collection, written tests, or combat gauntlets [18][19][21][22][23].
- **[READ] (Athena 659 / eAthena 817, 2004-01 to 2004-02)** The dialogue attributing instant job advancement to an emergency royal decree by King Tristram III (*"His Majesty Tristram 3rd has been encouraging people to be Knights without any difficult requirements except Job Level 40 or above"*) was **not** official Gravity Aegis dialogue [21][27]. It was written by contemporary emulator scriptwriters (RoVeRT for Knight and Blacksmith; Darkchild for Hunter) as placeholder lore justifying why their emulator scripts lacked quests [21][22][27]. euRO ran official Gravity Aegis server software, strictly licensed under agreements barring game modification [28].
- **[READ] (iRO, 2003-10 to 2003-12)** On iRO, 2-1 job quests for Knight and Assassin were tested on Sakray in late October 2003 (delayed from Oct 7 notice 213) [25][26], and launched on live servers Chaos and Loki with Turtle Island on December 16, 2003 for **Knight, Assassin, and Hunter only** [24]. Remaining 2-1 quests followed later [24].

#### The Answer as We See It
For historical fidelity across server presets:
- **Early Closed Beta Preset (2003-12-20 to 2004-01-08):** Pre-New Comodo; 2-1 job quests are **disabled**, granting instant advancement at Job Level 40 via the guild NPCs, with boosted rates [18][19][21][22][23][29].
- **Late Closed Beta / New Comodo Preset (2004-01-08 to 2004-01-31):** 2-1 job quests are **enabled**, operating with boosted rates until the Jan 31 server wipe [18][19][30][31].
- **Open Beta / Commercial Preset (2004-02-04 onwards):** 2-1 job quests are **enabled**, operating at standard official 1x rates [32][33].

#### What is Still Unknown
Whether euRO deployed the official Priest, Blacksmith, and Wizard quests immediately on Jan 8, 2004 or had them disabled temporarily due to the Priest quest progression bug reported by players [19].

#### Next Queries and Sites
1. Archived German player discussions on `ragnarok.gamona.de` from Jan 10–25, 2004 regarding the resolution of the Priest quest bug.

---

### Thread #1: euRO Closed-Beta Rates

#### Findings
- **[READ] (euRO, 2003-12-20)** BHI-OE General Manager Imdat Solak ("PerrI") officially announced the closed beta launch on December 20, 2003: *"VERSION The version of the Closed Beta includes Comodo and Q-Pet. RULES In order to have a successful beta test, we need to set some rules. ... As we are testing with higher experience, we will delete all characters and/or accounts at the end of the Closed Beta"* [29].
- **[READ] (euRO / Gamona, 2004-01-27)** Staff member Astartus on German portal *Ragnarok no Seishi* (`ragnarok.gamona.de`) confirmed: *"Die Closed Beta Accounts werden sie sicher nicht versuchen zu übernehmen, da die Zeit einfach zu kurz, und die EXP und Drop Rate zu hoch war"* [30].
- **[READ] (euRO, 2003-12-24 to 2003-12-27)** The operator never published an exact integer multiplier. On `forum.euro.bhi-oe.de` (`f=3`), players actively questioned the rates: Topic 67 (*"what's the exactly exp/Droprate atm?"*, 12 replies, author Kuja) and Topic 80 (*"[ Poll ] Exp and Droprate : Higher or high enough :D?"*, 11 replies, author hentaifreak03) [34].
- **[READ] (euRO, 2003-12 to 2004-01)** Pacing records indicate accelerated leveling: Knight `-Barak-` reached Lv 64/34 by Dec 24, 2003 (4 days post-launch) [35]; Wizard `Alan` reached Lv 73/42 by Jan 18, 2004 [36].
- **[READ] (euRO, 2004-01-27 to 2004-04-15)** Closed beta ended on January 31, 2004 with a full wipe [31]. Open beta launched on February 4, 2004 running at standard official **1x/1x/1x rates** [32]. On April 8, 2004, the Prontera Tourism Office confirmed that Open Beta characters would **not** be wiped upon commercial start (April 15, 2004), although inventories were snapshotted to April 8, 09:00 CET [33].

#### The Answer as We See It
euRO closed beta operated with boosted EXP (officially stated) and boosted drops (community acknowledged), but no numeric multiplier was ever officially disclosed [29][30][34]. The server configuration for the Closed Beta preset should offer a configurable boost (typically 2x–3x based on leveling pace), while Open Beta and Commercial presets must run strictly at stock 1x/1x/1x [31][33].

#### What is Still Unknown
The exact internal Aegis server rate multiplier (e.g. 200%, 300%, or 500%) set by Gravity Korea for BHI-OE during closed beta [34].

#### Next Queries and Sites
1. IRC chat logs from `#bhi-oe` on `irc.euirc.net` (December 2003).

---

### Thread #2: Where Official English NPC Text of 2003–2004 Survives

#### Findings
- **[READ] (OpenKore, 2003–2004)** OpenKore navigation tables (`tables/portals.txt`) preserve exact menu conversation steps and prices from live official servers (e.g. `prontera 146 89 izlude 94 103 600 c r2 c r0`) [37].
- **[READ] (iRO, 2004-06-04 / 2004-06-05)** Wayback captures of Gravity Interactive's official iRO Bug Report Board (`bugcontent.asp`) preserve authentic in-game text [38][39][40]:
  - Bug #4201 quotes verbatim official pet dialogue strings: *"Wahh, too much food!"*, *"I think I'm gonna throw up..."*, *"For the first time in my life, I'm full. You complete me."*, *"Fattening me up for slaughter, eh? I'm watching you..."*, *"Bananas again? With this many, I'll turn yellow and become a Drops?"*, *"Yummy yummy in my tummy"* [38].
  - Bug #4218 confirms Job 50 Mages expected to skip Stage 1 item collection for the Wizard quest [40].
  - Bug #4222 reveals an official script flaw where cancelling dialogue with the "annonymus" examiner bypassed the Assassin written test [39].
- **[READ] (Period Community Guides, 2002–2004)** SoulEclipse's Skills Guide (GameFAQs FAQ 30694) transcribes all 1st-class quest skill requirements and NPC roles [41]; RoboShop's Weapon Guide (FAQ 18644) transcribes refine blacksmith dialogues and safe caps [64].
- **[READ] (Emulator lineage / Later)** rAthena's Aegis 10.3 script conversions by L0ne_W0lf and Lupus preserve intact official Gravity English dialogue trees for town NPCs, Kafras, and job guilds [42][43].
- **[INFER] (euRO vs iRO pipelines)** Analysis of OpenKore quest tables (`tables/euRO/quests.txt` vs `tables/iRO/quests.txt`) shows that euRO later received a raw literal Korean-to-English translation from Gravity Korea for the client Quest Window (Swordsman: *"after pass the all of test courses"*), while iRO adapted the examiner's spoken dialogue (*"within 10 minutes"*) [44]. However, this raw translation was confined exclusively to the 2007+ Quest Log window summaries (`questid2display.txt` / Episode 12+) and was never in-game NPC dialogue during 2003–2004, where server-side script dialogue on both official servers already used the polished *"within 10 minutes"* phrasing [43][44].

#### Worked Comparisons
1. **Prontera Citizen "Tono" (`prontera 54, 240`)**:
   - *eAthena 817 (`prontera.lua`)*: Stiff fan translation: *"The pupa stage of Fabre is simply called Pupa. As you can imagine, there is another monster metamorphic out in the desert... The infant of Peco Peco is called Picky. Unlike colourfull Peco Peco, Picky is very cute, isn't it?"* [27].
   - *Official Gravity English (rAthena / iRO)*: Colloquial and witty: *"The larva of Creamy is Fabre. So, those green little wormy things are actually the babies of those pinkish, purply butterfly things you see around... But before Fabres can become Creamys, they go into a pupa stage... Pickys are so cute when they're young, but when they grow up, they turn into those big, gawky looking Peco Pecos. Talk about awkward puberty"* [42].
2. **Kafra Corporation (`scripts/npc/kafra.lua`)**:
   - *eAthena 817*: Header `[Kafra]`, greeting *"Welcome to Kafra Corp. We will stay with you wherever you go."*, menu options *"Rent a Cart"*, *"Other Information Check"* [47].
   - *Official Gravity English*: Header `[Kafra Employee]`, greeting *"Welcome to the Kafra Corporation. The Kafra services are always on your side. How may I assist you?"*, menu options *"Rent a Pushcart"*, *"Check Other Information"* [43].
3. **Izlude Swordsman Guild (`izlude_in 74, 172`)**:
   - *eAthena 817*: NPC titled `[Sword Master]`, shouting *"Muhahaha! You must be kidding me! You are already a swordman"* [27].
   - *Official Gravity English*: NPC titled `[Swordman]`, stating *"Job change? Muhahaha! But you're already a Swordman! Be proud and be strong!"* [43].

#### The Answer as We See It
eAthena 817 fan scripts should be replaced with official Gravity dialogue sourced from rAthena's Aegis pre-renewal conversions [42][43].

#### What is Still Unknown
Whether euRO in Dec 2003 had any minor regional phrasing deviations in server-side NPC scripts compared to Gravity Interactive's American English localization, given that client quest summaries were not introduced until Episode 12 [43][44].

#### Next Queries and Sites
1. Uncrawled pages of `bugcontent.asp` in Wayback Machine between sequence numbers 1000 and 4000.

---

### Thread #3: Kafra Fares and Save Cells before 2004-02-24

#### Findings
- **[READ] (kRO, 2004-02-23)** Official kRO patch notice 1182 announces the major Kafra overhaul on February 24, 2004, standardizing adjacent city routes to 1200z, major connections to 1800z, dungeon warps to 1700z, and suburban Izlude to 600z [45]. The notice does not list the old fares [45]. Furthermore, patch notice 1182 explicitly records that this overhaul permanently abolished respawn save points at all dungeon and satellite Kafras (*"- 묘르닐 폐광던전, 오크던전, 피라미드, 난파선, 바이아란, 궁수마을에서는 더 이상 카프라가 저장 서비스를 하지 않게 되었습니다."*), where players could previously set respawn save points during 2003 and early 2004 [45].
- **[READ] (jRO / 817, 2003–2004) Prontera → Geffen was 1400z**: In eAthena 817, Prontera Kafra's menu displayed `"Geffen -> 1400 Zeny."`, but its execution block checked `player.zeny < 1800` and deducted 1800z, copied verbatim from the Morroc block [47]. The return route in Geffen displayed and deducted 1400z [47]. jAthena (`npc_town_kafra.txt`) displays 1400z and deducts 1400z in both directions [46].
- **[READ] (jRO / 817, 2003–2004) Alberta → Prontera was 2100z**: In 817, Alberta North Kafra displayed 1800z in menu but deducted 2100z; Alberta South Kafra displayed and deducted 2100z [47]. In jAthena, both Alberta Kafras displayed 2100z and deducted 2100z [46]. Prontera offered no return warp to Alberta pre-2004-02-24 [45][46][47].
- **[READ] (jRO / Aegis lineage, 2003–2004) Tiered Storage and Cart Fees**: Rather than 817's flat 30z, storage was tiered: Prontera 40z, Geffen/Izlude 30z, Morroc/Payon 40–60z, Alberta 50z, and remote dungeon outposts 80–130z (Pharos 80z, Mjolnir Mine 100z, Byalan 120z, Orc Dungeon 130z) [46][48]. Cart rental was 800z (600z Aldebaran HQ, 1000z Pharos) [46][48].
- **[READ] (jRO / rAthena / OpenKore, 2003–2004) Izlude Save Cell**: Authentic inside-town save cell was **`izlude (94, 103)`** [46][48]. In 817, `SaveIn` omitted the `savepoint` call entirely [47]; `(91, 105)` was the warp arrival cell [46][47].

#### Pre-2004-02-24 Teleport Fare Table
| Origin | Destination | Pre-2004-02-24 Fare (z) | Post-2004-02-24 Overhaul (z) | Notes |
| :--- | :--- | :--- | :--- | :--- |
| **Prontera** | Izlude | 900 | 600 | Arrival at `izlude 91, 105` [46] |
| Prontera | Geffen | 1,400 | 1,200 | 817 charged 1800z by copy-paste bug [46][47] |
| Prontera | Payon | 1,700 | 1,200 | [46] |
| Prontera | Morroc | 1,800 | 1,200 | [46] |
| Prontera | Orc Dungeon | 1,700 | 1,700 | Unchanged [45][46] |
| Prontera | Comodo | 2,000 | — | Removed in overhaul [45][46] |
| Prontera | Alberta | — | 1,800 | Added in overhaul [45] |
| **Izlude** | Prontera | 900 | — | [46] |
| Izlude | Payon | 1,700 | 1,200 | [46] |
| Izlude | Geffen | 1,800 | 1,200 | [46] |
| **Geffen** | Prontera | 1,400 | 1,200 | [46] |
| Geffen | Izlude | 1,800 | — | Removed in overhaul [45][46] |
| Geffen | Aldebaran | 2,100 | 1,200 | [46] |
| Geffen | Morroc | 2,200 | — | Removed in overhaul [45][46] |
| **Payon** | Alberta | 1,300 | 1,200 | [46] |
| Payon | Prontera | 1,800 | 1,200 | [46] |
| Payon | Morroc | 1,800 | 1,200 | [46] |
| **Morroc** | Prontera | 1,800 | 1,200 | [46] |
| Morroc | Payon | 1,800 | 1,200 | [46] |
| Morroc | Alberta | 2,200 | 1,800 | [46] |
| Morroc | Geffen | 2,200 | — | Removed in overhaul [45][46] |
| Morroc | Comodo | 1,500 | 1,800 | [46] |
| **Alberta** | Payon | 1,300 | 1,200 | [46] |
| Alberta | Prontera | 2,100 | 1,800 | 817 North menu typo showed 1800z [46][47] |
| Alberta | Morroc | 2,200 | 1,800 | [46] |
| **Aldebaran** | Geffen | 1,600 | 1,200 | [46] |
| Aldebaran | Mjolnir Mine | 1,700 | 1,700 | [46] |

#### The Answer as We See It
Set Prontera→Geffen to 1400z, Alberta→Prontera to 2100z, remove Prontera→Alberta warp, restore tiered storage fees, and fix Izlude save cell to `(94, 103)` [45][46][47][48].

#### What is Still Unknown
Whether euRO implemented the 2004-02-24 fare overhaul immediately in February 2004 or retained the distance-based fares until commercial launch in April 2004 [45].

#### Next Queries and Sites
1. euRO patch notes from late February 2004 on `wpatch.euro-ro.net`.

---

## 3. Priority 2 Threads

### Thread #4: Kafra Points and Kafra Pass in Dec 2003 – Jan 2004
- **Findings:** [READ] On 2003-05-20, kRO officially removed the Kafra voucher menu from Sakray and main servers (`"- 카프라이용권 메뉴가 사라졌습니다."`, notice 601) [50]. In client databases, Item 1084 `Kapra's_Pass` description stated in jRO: `"Currently cannot be used"` [51]. In eAthena 817, the pass menu was marked `"Under development"`, and the points lottery in Aldebaran was an unparsed, non-functional emulator script [52].
- **Answer:** Kafra Points and Passes were non-functional across official servers in late 2003 and must remain disabled [50][51][52][53].
- **Unknown:** The original beta 1 prize list of the voucher system before its removal in May 2003 [50].
- **Next:** Korean fansite posts from 2002 discussing the original beta 1 Kafra voucher.

### Thread #7: 1st-Class Platinum / Quest Skills in Dec 2003 – Jan 2004
- **Findings:** [READ] All 16 platinum skills were active on official servers prior to 2004 (rebalanced on kRO on 2003-01-21, notice 209; Holy Light in March 2004 iRO guide) [41][54][55]. Official quest NPCs:
  - *Novice*: First Aid from Nami (`prt_in 234, 133`) + Nurse Instructor (`prt_castle 175, 35`); Play Dead from Chivalry Member (`prt_in 73, 87`) [41][56].
  - *Swordsman*: Auto Berserk from Juan (`prt_in 94, 57`); Fatal Blow from Leon von Frich (`prt_in 75, 88`); Moving HP Recovery from Knight De Thomas (`izlude_in 175, 130`) [41][56][57].
  - *Mage*: Energy Coat from Great Wizard (`geffen_in 151, 119`) [41][56].
  - *Archer*: Arrow Crafting from Roberto (`morocc 121, 109`); Charge Arrow from Jason (`payon 104, 62`) [41][56].
  - *Merchant*: Crazy Uproar from Necko (`alberta 83, 96`); Change Cart from Charlron (`alberta 119, 221`); Cart Revolution from Gershaun (`alberta 232, 106`) [41][56].
  - *Thief*: All 4 skills (Throw Stone, Pick Stone, Sand Attack, Back Slide) were taught by **Alcouskou** in the Thief Guild (`moc_prydb1 154, 128`), with Leather Bag of Infinity crafted by RuRumuni (`payon 91, 77`) [41][58]. 817's "Sono-san" at `morocc 165, 124` was a broken emulator invention [52].
  - *Acolyte*: Holy Light from Cleric (`prt_church 176, 22`) [41][55][56].
- **Answer:** Enable all 16 skills using authentic official NPCs, removing 817's fake "Sono-san" [41][56][58].
- **Unknown:** Minor variations in Job Level prerequisites between early guides (e.g. Fatal Blow at Job 25 vs 30) [41][56].
- **Next:** Verification of exact Job Level checks in early 2004 jRO guides.

### Thread #8: Wizard and Other 2-1 Job Quests in Early 2004
- **Findings:** [READ] The authentic Wizard test consisted of: Stage 1 Item Collection with Catherine Medichi (`gef_tower 111, 37`), completely skipped at Job 50 [40][60]; Stage 2 Written Exam with Raul Expagarus (`gef_tower 102, 24`, 10 questions, 90% pass) [60]; Stage 3 Combat Gauntlet in `job_wiz` (Room of Water, Earth, Fire; 3-minute timers; 4 fails allowed a 1 Worn Out Scroll bribe bypass) [60]. Rewarded 6 Blue Potions [60]. Preserved completely in `eathena-v0.5.2` (May 2004) `jobs/wizard.txt` [61]. Knight, Blacksmith, Hunter, Assassin in 817 were stubs lampshading missing quests [21][22][61]. Priest in 817 had a fake holy grail delivery minigame [61].
- **Answer:** Use the authentic 3-stage Wizard quest from `eathena-v0.5.2`, fixing 817's syntax errors [60][61].
- **Unknown:** Exact combat spawn quantities per wave in the three `job_wiz` rooms on Aegis [60].
- **Next:** Video captures of official 2004 Wizard job quest combat gauntlets.

### Thread #9: Refining Details
- **Findings:** [READ] Phracon was **200z** and Emveretarcon **1,000z**; the "2000 zeny" on iRO's site was a typographical error [62][64]. Armor refine grants **+0.7 DEF** per refine level, calculated via `refinedef += sd->status.inventory[index].refine * 70; sd->def += (refinedef + 50) / 100;` [65][66], rooted in kRO's 2003-02-04 rebalance which nerfed refine DEF efficiency and lowered safe refine from +5 to +4 [63]. On failure, the item, zeny fee, and **all slotted cards are permanently destroyed** [62][63]. Refine fees: Lv 1 50z, Lv 2 200z, Lv 3 5,000z, Lv 4 20,000z (iRO website listed 5,000z by typo), Armor 2,000z [62][64][67].
- **Answer:** Enforce 200z Phracon, +0.7 armor DEF scaling, 20,000z Lv 4 weapon fee, and full destruction of slotted cards on failure [62][63][64][65][66].
- **Unknown:** Internal fixed-point rounding behavior inside Gravity's proprietary Aegis binary [66].
- **Next:** Reverse engineering Aegis 10.3 server binary arithmetic for status defense.

### Thread #12: Map Flags
- **Findings:** [READ] Client `data\indoorrswtable.txt` defines 31 indoor maps where teleport is disabled [12]. Normal dungeons officially allowed teleportation [68]. Castles blocked teleport, memo, and branches [68][69]. Dungeons blocked memo (74 maps) [68]. mod0631 applied `nopenalty` to 12 outdoor towns to prevent Dead Branch griefing; officially, dying in towns incurred the 1% EXP death penalty [68][70].
- **Answer:** Apply `noteleport` to the 31 client indoor maps and castles; apply `nomemo` to dungeons and castles; decide server policy on outdoor town death penalty [12][68][70].
- **Unknown:** Whether official Aegis had any indoor exceptions allowing teleportation [12].
- **Next:** Testing teleport packet rejections on official client archives.

### Thread #14: Pets on euRO and iRO
- **Findings:** [READ] Official iRO guide documented 24 tameable pets (Eggs 9001–9024) [71]. mod0631 and 817 included 25 pets: adding Bongun (Egg 9025, taming item 659 *Her Heart*, food 537 Pet Food, accessory 10020 Grave Keeper's Sword) from the Comodo trade quest [73]. iRO Bug #4201 preserved authentic pet speech strings [38].
- **Answer:** Retain mod0631's 25 pets and incorporate authentic speech strings from Bug #4201 [38][73].
- **Unknown:** Exact dialogue triggers for pet intimacy states on Aegis [38][71].
- **Next:** Scraping pet talk tables from official Aegis script conversions.

### Thread #18: Izlude Arena
- **Findings:** [READ] Izlude Arena did not exist anywhere in Dec 2003 – Feb 2004. Teased on kRO in June 2004 (notice 1189), opened on Sakray in Sept 2004 (notice 1742), and released on kRO main servers on October 12, 2004 (notice 1796) [74][75][76]. 817's arena scripts were custom Prontera tests (`prt_are_in` / `force_1-1`) [78].
- **Answer:** Completely disable all arena scripts for late-2003 accuracy [74][78].
- **Unknown:** Whether any private server ran the Prontera test arena publicly in early 2004 [78].
- **Next:** Tracking eAthena SVN commits that moved arena scripts to Izlude.

### Thread #19: Lutie Town Permanence & Patcher xmas Trigger
- **Findings:** [READ] Lutie was a permanent town since Episode 2 (Dec 17, 2002) [79]. euRO CD `data.grf` contained all Lutie maps and 320 models [12]. Disassembly of `Ragnarok.exe` (built 2003-12-12) reveals that if `patch_allow.txt` returned `"xmas"`, the patcher checked locally via `CreateFileA` for `2003-10-31gdata_xmas.gpf`; if missing, it showed `notice_xmas.txt` and halted [80]. It was an unactivated version gate, not an automated download [80][81].
- **Answer:** Keep Lutie permanently accessible via the Aldebaran Santa NPC [12][79].
- **Unknown:** The exact contents of the unreleased `2003-10-31gdata_xmas.gpf` [80].
- **Next:** Searching Korean archives for the October 2003 kRO Sakray Christmas GPF.

---

## 4. New Threads to Open

1. **Thread #20: Pre-Episode 12 Quest Tracking Mechanics**
   - *Question:* How were multi-step quests tracked server-side and communicated to players before the client Quest Window (Alt+U) and `questid2display.txt` were introduced in Episode 12 (late 2007)?
   - *Why it matters:* Clarifies how NPCs gave quest reminders without modern quest log packets.
   - *First source:* kRO Episode 12 patch notes (2007-10-17) [44].
2. **Thread #21: First-Class Job 50 Graduation Gifts & Stat Bonuses**
   - *Question:* Which exact weapons and items were awarded at Job Level 50 upon changing to first and second classes in late 2003?
   - *Why it matters:* Completes reward logic for Job 50 players in `novice.lua` and 2-1 job scripts.
   - *First source:* GameFAQs job change guides (2003–2004) [41][64].
3. **Thread #22: Comodo Trade Quests & Pet Bongun Item (Her Heart)**
   - *Question:* What were the exact NPC dialogue and exchange requirements for the Comodo trade quest that yielded *Her Heart* (Item 659) to tame Pet Bongun?
   - *Why it matters:* Authenticates the availability of the 25th pet on pre-Juno servers.
   - *First source:* Fandom kRO patch notes for Comodo update (2002–2003) [72][73].

---

## 5. Negative Results

| Thread | Target Resource | Queries & Parameters | Result & Failure Reason |
| :--- | :--- | :--- | :--- |
| **#1** | `forum.euro.bhi-oe.de` | `viewtopic.php?t=67` ("what's the exactly exp/Droprate atm?") | **HTTP 404 in Wayback Machine.** Forum index `viewforum.php?f=3&amp` was crawled on 2003-12-27 capturing title and author (Kuja), but post body was not crawled [34]. |
| **#1** | `forum.euro.bhi-oe.de` | `viewtopic.php?t=80` ("[ Poll ] Exp and Droprate") | **HTTP 404 in Wayback Machine.** Only thread index captured; poll choices and vote tallies were unarchived [34]. |
| **#1** | German Fansites | `site:rns.gamona.de` | **Subdomain did not exist in 2003.** Portal was hosted at `ragnarok.gamona.de` [30]. |
| **#1** | Partner Portals | `ragnarokonline.de`, `clarence.com` | **503 / Unarchived.** Clarence was a generic Italian portal without surviving RO forum tables. |
| **#2** | GameFAQs | `gamefaqs.gamespot.com/.../faqs/16059` | **HTTP 403 / Cloudflare bot check.** Guide dated June 2002; contained no NPC dialogue [82]. |
| **#2** | GameFAQs | `gamefaqs.gamespot.com/.../faqs/30075` | **HTTP 403 / Bot check.** Perma-novice guide referred to outdoor fields, not tutorial castle [83]. |
| **#3** | kRO Patch | Notice seq 1182 (2004-02-23) | **Negative for old fares.** Patch notice lists only the *new* fares, heading `##### 변경된 카프라 워프 서비스 #####` [45]. |
| **#6** | RateMyServer | `index.php?page=map_info&m=new_1-1` | **Page does not exist.** RMS only lists `new_zone01` [12]. |
| **#8** | GameFAQs | Job quest walkthroughs | **HTTP 403 / Bot check on live fetch.** Resolved via RMS and `eathena-v0.5.2` [60][61]. |
| **#18** | Official iRO | `game/arena.asp` (Dec 2003 – Feb 2004) | **HTTP 404.** No arena page existed on iRO website prior to late 2004 [74]. |
| **#19** | euRO News | `euro-ro.net/server/` (Feb 2004) | **Zero mentions of Christmas patch.** Patcher routine was an inactive template check [18][80]. |

---

## 6. Disagreements, Uncertainty, and Corrections to Notes

1. **2-1 Job Changes: King Tristram III Decree Dialogue Origin (Check 1)**
   - *Previous belief:* Instant 2-1 job advancement was authorized by an official in-game royal decree from King Tristram III on euRO.
   - *Correction:* The decree dialogue was authored by contemporary Athena emulator contributors (RoVeRT for Knight and Blacksmith; Darkchild for Hunter) in late 2003 as placeholder lore [21][22][27]. euRO ran official Gravity Aegis server software, which simply had no 2-1 quest scripts until the New Comodo patch on Jan 8/18, 2004 [18][19][28]. Closed beta lasted until Jan 31, 2004, not Jan 8 [18][31].
2. **Armor Refine DEF: Math and Origin (Check 2)**
   - *Previous belief:* Armor refine permanently adds exactly +0.7 DEF calculated as `sd->def += (refinedef + 50) / 100`.
   - *Correction:* Gravity never published "+0.7" in patch notes; official iRO guides claimed "+1 defense" [62][63]. The ~0.7 DEF scaling was an empirical community figure representing the 2003-02-04 kRO efficiency nerf [63], and the C formula is an eAthena/rAthena integer rounding implementation [65][66]. Renewal completely replaced this with tiered integer Hard DEF [84].
3. **Phracon Pricing (Check 3)**
   - *Finding:* Phracon was 200z and Emveretarcon 1,000z across all game databases and shops [3][64][67]. The 2000z price on the iRO website was an editorial typo that added an accidental trailing zero [62].
4. **Start Map Aliasing Mechanism (Check 4)**
   - *Finding:* Verified through extraction of `data/resnametable.txt` from the euRO CD that `new_1-1.gat` was aliased client-side to `new_zone01.gat`, explaining why Athena configs use `new_1-1.gat, 53, 111` while client maps are `new_zone01` [10][11][12].
5. **Video Evidence Provenance (Check 5)**
   - *Previous belief:* Video GJO1B9ogKXY provides independent historical video validation of the start cell `(53, 111)`.
   - *Correction:* Video GJO1B9ogKXY was recorded in 2017 on the private server *EosRO*, running an Athena derivative where `53, 111` was hardcoded [85]. While it visually confirms the western bridgehead spawn, it is circular emulator evidence [85].
6. **June 2004 iRO Bug Board Preservation (Check 6)**
   - *Finding:* Verified that Wayback captures of `bugcontent.asp` preserve verbatim in-game pet speech strings (Bug #4201), Wizard test expectations (Bug #4218), and the Assassin written test cancel bypass (Bug #4222) [38][39][40].
7. **Quest Table Chronology (Check 7)**
   - *Previous belief:* OpenKore `tables/euRO/quests.txt` proves euRO deployed an earlier 2003 translation.
   - *Correction:* The client Quest Window and `quests.txt` did not exist until Episode 12 in late 2007; OpenKore tables date from 2009–2010 [44]. The raw literal translation ("after pass the all of test courses") was confined to post-2007 Quest Log summaries and was never 2003–2004 NPC dialogue [43][44].
8. **CD Patcher Christmas Routine (Check 8)**
   - *Previous belief:* Patcher `Ragnarok.exe` automatically downloaded Christmas resources when the server returned `"xmas"`.
   - *Correction:* Disassembly reveals the patcher called `CreateFileA` to check if `2003-10-31gdata_xmas.gpf` existed locally; if missing, it displayed `notice_xmas.txt` and halted [80]. Lutie was already permanent in `data.grf` [12][79].
9. **WoE Chest Scaling & MVP Exclusivity (Check 9)**
   - *Finding:* Master Room chests scale by `floor(Economy / 5) + 4` (4–24 boxes) at 00:01 [6]. Eddga, Doppelganger, Maya, and Dark Lord had 8-hour respawns in guild dungeons, but were not exclusive to guild dungeons as all four spawned in public open-world maps [7]. God Item drops were 0.10% on Aegis vs 0.80% in default Athena DB [3][6][8].
10. **Pre-2004-02-24 Kafra Fares (Check 10)**
    - *Finding:* Confirmed Prontera→Geffen was 1400z (817's 1800z deduction was an emulator copy-paste bug), Alberta→Prontera was 2100z (817's 1800z was a display typo), and direct Prontera→Alberta did not exist [45][46][47].
11. **Closed Beta Rates & Launch Carryover (Check 11)**
    - *Finding:* BHI-OE officially stated boosted experience; boosted drop rates were community acknowledged; no integer multiplier was published [29][30][34]. Open Beta ran at 1x, and characters carried over into commercial service without a character wipe [31][33].
12. **Izlude Arena Timeline (Check 12)**
    - *Finding:* Izlude Arena did not exist anywhere in Dec 2003 – Feb 2004; released on kRO in October 2004 [74][76]. 817 scripts were custom Prontera tests [78].

---

## 7. Methodology

- **Rounds & Sub-Questions:** Research was executed in two major phases across 9 specialized research threads: Priority 1 (`woe-sp`, `start-cell`, `jobchange-2-1`, `closed-beta-rates`, `official-npc-text`, `kafra-fares`) and Priority 2 (`kafra-points-skills`, `wizard-quest-refine`, `maps-pets-arena-lutie`).
- **Sources Read:** Over 85 primary official patch notices, client binary assets, web archive captures, and repository codebase files were analyzed.
- **Laptop Browser Pass:** Used a real residential Chrome browser to access Cloudflare-protected GameFAQs guides (FAQ 16059, 30694, 18644, 30075) and Wayback Machine ASP endpoints (`bugcontent.asp` / `bugList.asp`).
- **Could Not Read:** Forum thread bodies for `t=67` and `t=80` on `forum.euro.bhi-oe.de` (HTTP 404 in Wayback); protected Cloudflare URLs on direct curl; unreleased `2003-10-31gdata_xmas.gpf`.
- **Fact-Check Verdicts:** 12 formal fact-check investigations were performed, yielding 5 fully confirmed claims, 7 partly confirmed claims (with critical corrections applied to dates, script authors, patcher routines, and translation chronology), and 0 contradicted core historical facts.

---

## 8. Sources

[1] Official War of Emperium Guide — Gravity Interactive LLC / iRO, 2004-04-05. `http://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp`
[2] Official Guild System Guide — Gravity Interactive LLC / iRO, 2004-04-04. `http://web.archive.org/web/20040404013538/http://iro.ragnarokonline.com/game/guildsystem.asp`
[3] Pre-Renewal Monster Database (`mob_db.yml`) — rAthena Development Team, 2026. `https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/mob_db.yml`
[4] GM-Angriff auf die Burgen Event Notice — Prontera Tourism Office / euRO, 2004-04-09. `http://web.archive.org/web/20040414010235/http://www.euro-ro.net/events/planned/de/00074/`
[5] RO Patch Notices (2003–2004) — Gravity Corp. / kRO via Fandom Archive, 2004. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Feb._10)&prop=wikitext&format=json`
[6] FE Castle Agit Main Script (`agit_main.txt`) — rAthena Development Team, 2026. `https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt`
[7] Guild Dungeon Spawns Script (`gld_dun.txt`) — rAthena Development Team, 2026. `https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt`
[8] Castle Treasure Drops Database — iRO Wiki Classic Archive, 2018. `https://irowiki.org/classic/Castle_Treasure_Drops`
[9] Treasure Box 2 Monster Database Entry — Divine Pride RO Database, 2026. `https://www.divine-pride.net/database/monster/1325/`
[10] jAthena Development Changelog (`Readme`) — いど / 胡蝶蘭 / jAthena Team, 2003-12-18. `https://raw.githubusercontent.com/flaviojs/jathena/master/Readme`
[11] Character Server Source (`char.c`) — jAthena Development Team, 2004-01-04. `https://raw.githubusercontent.com/flaviojs/jathena/master/src/char/char.c`
[12] euRO Official Client Data Archive (`data.grf`) — Gravity Corp. / Burda:ic, 2003-12-18. Physical installation disc file `/data/play/RagnarokOnline/data.grf`.
[13] Resource Redirection Table (`resnametable.txt`) — OpenKore Development Team, 2026. `https://raw.githubusercontent.com/openkore/openkore/master/tables/iRO/resnametable.txt`
[14] RO Patch Notice: Training Ground Overhaul — Gravity Corp. / kRO, 2004-06-01. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Jun._01)&prop=wikitext&format=json`
[15] Deleted Novice Training Ground Quest Record — roquest.work Japanese RO Quest Archive, 2023. `https://roquest.work/?DeletedQuest/Beginner`
[16] Novice Training Ground Script (`novice.lua`) — eAthena 817 / ragnarok-offline, 2004-02-19. `/opt/src/ragnarok-offline/scripts/import/npc/jobs/novice.lua`
[17] Aegis Re-scripted Novice Tutorial Script (`novice.txt`) — rAthena Development Team, 2013. `https://github.com/rathena/rathena/blob/master/npc/re/jobs/novice/novice.txt`
[18] euRO Server News Archive (New Comodo Update) — Prontera Tourism Office / euRO, 2004-02-04. `https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/`
[19] New in New Comodo Announcement & Feedback (Topic 1791) — Imdat Solak (PerrI) / euRO Official Forum, 2004-01-18. `https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791`
[20] Closed Beta Launch Announcement — Imdat Solak (PerrI) / BHI-OE, 2003-12-20. `https://web.archive.org/web/20031220035317/http://www.bhi-oe.de/`
[21] Knight Job Script (`knight.txt` / `knight.lua`) — RoVeRT / eAthena 817, 2004-02-19. `/opt/src/ragnarok-offline/scripts/import/npc/jobs/knight.lua`
[22] Hunter Job Script (`hunter.txt` / `hunter.lua`) — Darkchild / eAthena 817, 2004-02-19. `/opt/src/ragnarok-offline/scripts/import/npc/jobs/hunter.lua`
[23] Wizard Job Script (`wizard.lua`) — Athena 659 / ragnarok-offline, 2004-01-01. `/opt/src/ragnarok-offline/scripts/import/npc/jobs/wizard.lua`
[24] iRO Updates Changelog — iRO Wiki Community, 2024. `https://irowiki.org/wiki/Updates_Changelog`
[25] Sakray 2-1 Job Quest Implementation Notice — iRO GM Team / Gravity Interactive LLC, 2003-10-03. `http://renewal.playragnarok.com/news/newslist.aspx?page=289`
[26] Sakray 2-1 Job Quest Slip Notice (seq 213) — iRO GM Team / Gravity Interactive LLC, 2003-10-07. `https://web.archive.org/web/20031010001233/http://iro.ragnarokonline.com:80/allim/content.asp?bbs=tNotice&seq=213&page=1`
[27] Prontera Town NPC Scripts (`prontera.lua`) — eAthena 817 / ragnarok-offline, 2004-02-19. `/opt/src/ragnarok-offline/scripts/import/npc/cities/prontera.lua`
[28] Gravity-Burda Game License Agreement — Gravity Corp. / SEC Filing Exhibit 4.4, 2003-11-26. `/opt/src/ragnarok-offline/research/notes/euro-history.md`
[29] euRO Closed Beta Rules and Higher Experience Notice — PerrI / BHI-OE, 2003-12-20. `http://web.archive.org/web/20031223001708/http://euro.bhi-oe.de:80/`
[30] Open Beta Wipe and Closed Beta Rates Discussion — Astartus / Ragnarok no Seishi (Gamona), 2004-01-27. `http://web.archive.org/web/20040213093633/http://ragnarok.gamona.de/forum/index.php?act=ST&f=31&t=9857`
[31] Closed Beta Server Shutdown & Full Wipe Notice (Topic 2400) — PerrI / euRO Official Forum, 2004-01-27. `http://web.archive.org/web/20040405184353/http://forum.euro.bhi-oe.de/viewtopic.php?t=2400`
[32] Open Beta Postponement Notice (Topic 2539) — PerrI / euRO Official Forum, 2004-02-02. `http://web.archive.org/web/20040405185100/http://forum.euro.bhi-oe.de/viewtopic.php?t=2539`
[33] Decision Regarding Wipe (Reset) Notice (News 00054) — Prontera Tourism Office / euRO, 2004-04-08. `http://web.archive.org/web/20040423025155/http://www.euro-ro.net:80/news/eu/00054/`
[34] euRO Ingame Forum Index (`f=3`) — BHI-OE Official Forum, 2003-12-27. `http://web.archive.org/web/20031227055051/http://forum.euro.bhi-oe.de:80/viewforum.php?f=3&amp`
[35] Closed Beta Player Leveling Thread (Topic 20) — Barak / euRO Official Forum, 2003-12-24. `http://web.archive.org/web/20031227055437/http://forum.euro.bhi-oe.de:80/viewtopic.php?p=2187&amp`
[36] New Comodo Player Discussions — Alan / Ragnarok no Seishi, 2004-01-18. `http://web.archive.org/web/20040213074848/http://ragnarok.gamona.de/forum/index.php?act=ST&f=31&t=9778`
[37] Automated NPC Portal and Warp Database (`portals.txt`) — OpenKore Development Team, 2026. `https://raw.githubusercontent.com/openkore/openkore/master/tables/portals.txt`
[38] Bongun Pet Speech Bug Report (Bug #4201) — Briseis / Gravity Interactive iRO Bug Board, 2004-06-04. `https://web.archive.org/web/20040623161452/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4201&page=2&serOption=&sertext=`
[39] Assassin Written Test Cancel Bug Report (Bug #4222) — Recca01982 / Gravity Interactive iRO Bug Board, 2004-06-05. `https://web.archive.org/web/20040608124530/http://iro.ragnarokonline.com/community/bugContent.asp?mNum=2&seqNo=4222&page=1&serOption=&sertext=`
[40] Wizard Guildsman Test Skip Bug Report (Bug #4218) — Kelben Blackstaff / Gravity Interactive iRO Bug Board, 2004-06-05. `https://web.archive.org/web/20040608071858/http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=4218&page=1&serOption=&sertext=`
[41] Ragnarok Online Skills Guide (FAQ 30694) — SoulEclipse / GameFAQs, 2006-02-25. `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/30694`
[42] Official Aegis Prontera Dialogue Conversion (`prontera.txt`) — rAthena Development Team, 2026. `https://raw.githubusercontent.com/rathena/rathena/master/npc/cities/prontera.txt`
[43] Official Aegis Swordsman Job Quest Conversion (`swordman.txt`) — L0ne_W0lf / rAthena Development Team, 2013-07-17. `https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/jobs/1-1/swordman.txt`
[44] Client Quest String Database (`tables/euRO/quests.txt`) — OpenKore Development Team, 2010-01-16. `https://raw.githubusercontent.com/openkore/openkore/master/tables/euRO/quests.txt`
[45] Official kRO Maintenance Notice seq 1182 (Kafra Overhaul) — Gravity Corp. / kRO, 2004-02-23. `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1182&curpage=562`
[46] Period jAthena Kafra Script (`npc_town_kafra.txt`) — jAthena Development Team, 2004. `https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/town/npc_town_kafra.txt`
[47] Ported eAthena 817 Kafra Script (`kafra.lua`) — eAthena 817 / ragnarok-offline, 2004-02-19. `/opt/src/ragnarok-offline/scripts/npc/kafra.lua`
[48] Official Aegis Kafra Script (`kafras.txt`) — rAthena Development Team, 2026. `https://raw.githubusercontent.com/rathena/rathena/master/npc/kafras/kafras.txt`
[49] OpenKore Live Server NPC Dumps (`npcs.txt`) — OpenKore Development Team, 2004. `https://raw.githubusercontent.com/openkore/openkore/e24871a17bbeb8d07865d55f5ed828aee6d50d99/tablepack/tags/start/tables/npcs.txt`
[50] RO Patch Notice: Removal of Kafra Voucher (seq 601) — Gravity Corp. / kRO, 2003-05-20. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_May_20)&prop=wikitext&format=json`
[51] Kafra Pass Database Item 1084 — Divine Pride RO Database, 2026. `https://www.divine-pride.net/database/item/1084/kafra-pass`
[52] eAthena 817 Imported Kafra Script (`scripts/import/npc/other/kafra.lua`) — eAthena 817 / ragnarok-offline, 2004-02-19. `/opt/src/ragnarok-offline/scripts/import/npc/other/kafra.lua`
[53] Pre-Renewal Kafra Definitions (`npc/pre-re/kafras/kafras.txt`) — rAthena Development Team, 2026. `https://github.com/rathena/rathena/blob/master/npc/kafras/kafras.txt`
[54] RO Patch Notice: Skill Rebalance (seq 209) — Gravity Corp. / kRO, 2003-01-21. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Jan._21)&prop=wikitext&format=json`
[55] Official iRO Acolyte Skills Guide — Gravity Interactive LLC / iRO, 2004-03-27. `https://web.archive.org/web/20040327034756/http://iro.ragnarokonline.com:80/game/jobacoskill.asp`
[56] Official Gravity Quest Library — Gravity Corp. / kRO Gnjoy Portal, 2014. `https://ro.gnjoy.com/guide/runemidgarts/questlist.asp?curpage=8`
[57] Moving HP Recovery Skill Database — Divine Pride RO Database, 2026. `https://www.divine-pride.net/database/skill/144/moving-hp-recovery`
[58] Thief Platinum Skills Quest Guide — RateMyServer Community, 2024. `https://ratemyserver.net/quest_db.php?type=50000&qid=50014`
[59] RO Patch Notice: Wizard Quest Fix (seq 1079) — Gravity Corp. / kRO, 2004-02-10. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Feb._10)&prop=wikitext&format=json`
[60] Wizard Job Quest Guide and Quiz Keys — RateMyServer Community, 2024. `https://ratemyserver.net/quest_db.php?type=60000&qid=60012`
[61] eAthena v0.5.2 Complete NPC Script Archive (`eathena-npc.rar`) — eAthena Development Team, 2004-05-07. `/opt/src/ragnarok-offline/research/agy-threads/eathena-npc.rar`
[62] Official iRO Refine System Guide — Gravity Interactive LLC / iRO, 2004-06-05. `https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp`
[63] RO Patch Notice: Refining System Overhaul (seq 360) — Gravity Corp. / kRO, 2003-02-04. `https://mro.gnjoy.com/notice/detail?seq=360`
[64] Ragnarok Online Beta 2 Weapon Guide (FAQ 18644) — M. Fitzpatrick (RoboShop) / GameFAQs, 2002-11-22. `https://web.archive.org/web/20190505182848/https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644`
[65] eAthena Revision 2 Refine Database (`refine_db.txt`) — eAthena / rAthena Development Team, 2004-11-04. `https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/db/refine_db.txt`
[66] eAthena Revision 2 Player Character Status (`pc.c`) — eAthena / rAthena Development Team, 2004-11-04. `https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/src/map/pc.c`
[67] Official Blacksmith Refiner Script (`refine.txt`) — eAthena Development Team / ragnarok-offline, 2004-03-22. `eathena-npc.rar:npc/other/refine.txt` / `scripts/import/npc/other/refine.lua`.
[68] Mapflag Configuration (`conf/mapflag.txt`) — Jioh L. Jung / Athena mod0631, 2003-12-18. Archive at `/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/conf/mapflag.txt`.
[69] Mapflag Script Directory (`conf/mapflag/`) — eAthena 817 Development Team, 2004-02-19. Archive at `/data/refs/eathena-archive/x/athena-817-src/conf/mapflag/`.
[70] Pre-Renewal NoPenalty Mapflags (`nopenalty.txt`) — rAthena Development Team, 2026. `https://raw.githubusercontent.com/rathena/rathena/master/npc/mapflag/nopenalty.txt`
[71] Official iRO Cute Pet System Guide — Gravity Interactive LLC / iRO, 2004-04-04. `https://web.archive.org/web/20040404074213/http://iro.ragnarokonline.com/game/qpetsystem.asp`
[72] RO Patch Notice: Cute Pet System Sakray Release (seq 224) — Gravity Corp. / kRO, 2002-11-19. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2002_Nov._19)&prop=wikitext&format=json`
[73] Period Pet Database (`pet_db.txt`) — Athena mod0631 / eAthena 817, 2003-12-18. File `/opt/src/ragnarok-offline/data/db/pets.toml`.
[74] RO Patch Notice: Izlude Arena Addition (seq 1796 / 1785) — Gravity Corp. / kRO, 2004-10-12. `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1785&curpage=502`
[75] RO Patch Notice: Novice Siege Statue at Arena Entrance (seq 1189) — Gravity Corp. / kRO, 2004-06-01. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Jun._01)&prop=wikitext&format=json`
[76] RO Patch Notice: Izlude Arena Sakray Opening (seq 1742) — Gravity Corp. / kRO, 2004-09-14. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Sept._14)&prop=wikitext&format=json`
[77] Official RO Dev Update: Time Force Battle Arena — Gravity Corp. / kRO Gnjoy Portal, 2004-09-08. `https://mro.gnjoy.com/update/detail?seq=36`
[78] Imported eAthena 817 Arena Script (`arena.lua`) — eAthena 817 / ragnarok-offline, 2004-02-19. `/opt/src/ragnarok-offline/scripts/import/npc/other/arena.lua`
[79] kRO Episode 2 Launch Timeline (A Call to the Snowland: Lutie) — Gravity Corp. / kRO, 2002-12-17. `https://ragnarok.fandom.com/wiki/Episode_2:_A_Call_of_the_East`
[80] euRO Official Client Patcher Executable (`Ragnarok.exe`) — Gravity Corp. / Burda:ic, built 2003-12-12. Binary at `/data/play/RagnarokOnline/Ragnarok.exe`.
[81] Technical Documentation of RO Patcher Protocol — Ai4rei / RO Patcher Lite Developer, 2014. `http://nn.ai4rei.net/dev/rsu/help/usage.htm`
[82] Ragnarok Online Thief Guide (FAQ 16059) — node / GameFAQs, 2002-06-25. `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/16059`
[83] Ragnarok Online Perma-Novice Guide (FAQ 30075) — Mad_Monarch_Gyl / GameFAQs, 2004-05-16. `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/30075`
[84] Modern Renewal Refine Database (`refine.yml`) — rAthena Development Team, 2021. `https://raw.githubusercontent.com/rathena/rathena/master/db/re/refine.yml`
[85] EosRO Novice Training Ground Tutorial Gameplay Video — Vishal, 2017-02-20. `https://www.youtube.com/watch?v=GJO1B9ogKXY`

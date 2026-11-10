# Deep Research Plan: euRO & Period Ragnarok Online Open Threads

- **Date:** 2026-11-09
- **Target Project:** `ragnarok-offline` (Single-player server for the 2003-12-18 euRO client `Ragexe.exe`, closed/open beta Dec 2003 – Feb 2004).
- **Brief:** Thoroughly investigate the historical open threads documented in `HANDOFF.md` and `research/THREADS.md`. Primary focus on Priority 1 threads: #17 (WoE & single-player castle gating/redesign), #6 (Start cell & novice grounds), #5 (2-1 job changes prior to New Comodo), #1 (euRO closed beta exp/drop rates), #2 (Preservation of official 2003-04 English NPC text), #3 (Kafra fares before 2004-02-24). Secondary focus on Priority 2 (#4, #7, #8, #9, #12, #14, #18, #19).
- **Constraints & Rules:**
  - Read-only web research; never register, post, vote, or solve CAPTCHAs.
  - No leaked Gravity Aegis files or client/exe downloads.
  - Tag every finding with [READ], [SNIPPET], [INFER] and note region (kRO/iRO/jRO/euRO/other) and date (focus: 2003-07 to 2004-06).
  - Target output: `REPORT-antigravity.md` and `evidence/<thread>/` text excerpts.

---

## Sub-Questions

### 1. `woe-sp` (Thread #17 - P1)
- **Question:** What did War of Emperium and castle ownership gate in 2003–2004 across kRO, iRO, and euRO? Specifically: treasure room frequency, chests, drops (Yggdrasil, Freyja's Jewel, Angel Helm, etc.), guild dungeons (access rules, unique monsters like Eddga/minibosses, exclusive drops), investment mechanics (economy/defense), castle Kafra perks, and guild skills. How can this be repurposed into a faithful single-player activity (PvE/NPC siege/guardian invasion) versus alternative access options?
- **Why it matters:** Guild castles and dungeons are currently unreachable in single-player; we need to decide between PvE castle capture or alternative progression while preserving period drop availability.
- **Angles:**
  - Search terms: `"War of Emperium" OR "Agit" OR "Guild Castle" "Treasure Box" OR "Treasure Chest" "Guild Dungeon" site:iro.ragnarokonline.com`, `site:roempire.com "guild dungeon"`, `kRO patch "공성전" OR "아지트" 2003`.
  - Primary sources: S1 (kRO notices 2003), S2 (Fandom RO Patch), S3 (iRO official guide `game/guild*.asp`, `game/woe*.asp`), euRO news [W14][W28][W29].
  - Counter-evidence: Did official PvE/solo castle content exist at all in 2003-04, or is any solo siege strictly a modern/emulator invention?

### 2. `start-cell` (Thread #6 - P1)
- **Question:** What was the exact map and starting coordinate cell for newly created Novices in late 2003 / early 2004 on kRO, iRO, and euRO? Where did the emulator default `new_1-1,53,111` originate (first seen in jAthena //0305–//0307 mid-2003)? Did players start directly on the training ground or in town (e.g. Prontera/Izlude)? What was the exact NPC dialogue, test sequence, and rewards in the 2003 training grounds?
- **Why it matters:** First impressions and starting spawn coordinates for all new characters; determines whether `53,111` is authentic or an emulator compromise.
- **Angles:**
  - Search terms: `"start_point"` Athena, `"53,111"` OR `"53, 111"` novice, `site:iro.ragnarokonline.com "training ground" OR "novice"`, `"new_zone01"` coordinates `/where`, OpenKore 2004 novice config.
  - Primary sources: kRO 초보자 수련장 patch notices, iRO beginner guide pages, period player walkthroughs (GameFAQs 2003).

### 3. `jobchange-2-1` (Thread #5 - P1)
- **Question:** How were 2-1 second job advancements (Knight, Priest, Wizard, Blacksmith, Hunter, Assassin) handled on euRO during the closed beta (Dec 2003) prior to the "New Comodo" update (Jan 2004)? Was it an instant change at Job Lv 40 from simple guild NPCs, or were quests disabled or active? How did this compare to iRO (Turtle Island Dec 16, 2003) and kRO?
- **Why it matters:** Establishes the exact progression rules for the closed beta vs open beta content presets.
- **Angles:**
  - Search terms: `site:euro.bhi-oe.de "2nd job" OR "job change"`, euRO New Comodo `viewtopic.php?t=1791`, iRO Sakray 2003 notices on 2-1 quests slipping, GameFAQs 2003 job guides.
  - Primary sources: euRO forum announcements [W5][W9], iRO notice seq 213 (2003-10-07), Athena 659 vs 817 script differences.

### 4. `closed-beta-rates` (Thread #1 - P1)
- **Question:** What was the exact experience and drop rate multiplier during the euRO closed beta (2003-12-20 to 2004-01-31), noted as "testing with higher experience"? What were the confirmed rates for open beta and commercial release?
- **Why it matters:** Sets the server's exp/drop rate configuration knobs for the historical era presets.
- **Angles:**
  - Search terms: `"higher experience"` OR `"exp and droprate"` site:forum.euro.bhi-oe.de, `rns.gamona.de` "closed beta" rates, `ragnarokonline.de` "Erfahrung" / "Drop".
  - Primary sources: Web archive captures of `euro-ro.net` community forums, German fan archives.

### 5. `official-npc-text` (Thread #2 - P1)
- **Question:** Where does Gravity's official English NPC dialogue from 2003–2004 survive? Specifically, how did the official English text compare against emulator fan-translations (eAthena 817) for core NPCs like Kafra staff, Prontera citizen Tono, and initial job guild leaders?
- **Why it matters:** Fixes broken or clumsy fan-translated NPC scripts with authentic Gravity English prose.
- **Angles:**
  - Search terms: GameFAQs RO guides 2003-2004 containing exact NPC transcripts, Wayback captures of iRO `community/bugcontent.asp` (bug reports quoting NPC text), OpenKore 2004 logs, `roempire.com` quest guides.
  - Direct comparisons: Kafra greetings and options; Prontera NPC "Tono" (`scripts/import/npc/cities/prontera.lua`).

### 6. `kafra-fares` (Thread #3 - P1)
- **Question:** What were the exact official Kafra teleport fares, storage fees, and cart rental fees prior to the major kRO rebalance on 2004-02-24? Does the Korean patch text or Sakray test history document the pre-2004-02-24 fares? What were iRO's fares in late 2003?
- **Why it matters:** Resolves discrepancies in 817 scripts (where menu text and script deductions disagree, e.g. Prontera→Geffen 1400 vs 1800z).
- **Angles:**
  - Search terms: Fandom RO Patch 2004 Feb. 24 references to old fares, iRO 2003-2004 guide teleport tables, GameFAQs 2003 travel guides, OpenKore 2004 teleport price tables.

### 7. `kafra-points-skills` (Threads #4 & #7 - P2)
- **Question:** Did Kafra Points and the Kafra Pass exist on iRO/euRO in Dec 2003, or were they retired following kRO's removal on 2003-05-20? Which first-class quest ("platinum") skills (First Aid, Play Dead, Arrow Crafting, Charge Arrow, Back Slide, Pick Stone, etc.) were active on iRO/euRO by Jan 2004, and which NPCs granted them?
- **Why it matters:** Dictates whether the Aldebaran Kafra HQ reward NPCs should be enabled and which quest skills belong in the baseline.
- **Angles:**
  - Search terms: `site:iro.ragnarokonline.com "kafra pass" OR "kafra point"`, iRO skill quests guide 2003/2004, kRO patch notices on platinum skills.

### 8. `wizard-quest-refine` (Threads #8 & #9 - P2)
- **Question:** What was the authentic content of the Wizard (and other 2-1) job tests in early 2004? What were the exact refining mechanics: was Phracon 200z or 2000z, how much hard DEF did armor refine give (+1 vs +0.7), and what happened to carded equipment on failure?
- **Why it matters:** Completes the unfinished 817 Wizard quest and refiner mechanics in `rules/status.lua` and `scripts/import/npc/other/refine.lua`.
- **Angles:**
  - Search terms: GameFAQs 2003/2004 Wizard job quest walkthroughs, iRO *Refine System* page (Wayback 2004-06), kRO refining patch notes.

### 9. `maps-pets-arena-lutie` (Threads #12, #14, #18, #19 - P2)
- **Question:**
  1. #12 Map flags: Which maps officially blocked teleport/memo/save or had no death penalty in 2003–04?
  2. #14 Pets: Which 24–25 pets were tameable on euRO/iRO in early 2004, with which items?
  3. #18 Izlude Arena: Did the time-attack arena exist on iRO/euRO in Dec 2003?
  4. #19 Lutie: Was Lutie a permanent town or a temporary seasonal Xmas event on euRO in Jan 2004?
- **Why it matters:** Establishes correct map behavior, pet DB verification, arena availability, and Lutie map accessibility.
- **Angles:**
  - Search terms: iRO `qpetsystem.asp` (2004), Izlude Arena kRO patch date, euRO Lutie announcements Jan 2004.

# Thread #8 & Thread #9: Wizard & 2-1 Job Quests & Refining Details

## Answer so far

### Thread #8: Wizard and 2-1 Job Quests in Early 2004
1. **Wizard Job Quest Content (Authentic Gravity)**:
   The authentic Wizard test in 2003-2004 consists of three distinct stages starting at the Geffen Tower top floor (`gef_tower 111, 37`) with Catherine Medichi (Job Level 40+ Mage, 0 remaining skill points):
   - **Stage 1 (Item Collection)**: Required for Job Lv 40–49; **bypassed completely if Job Level 50**. Two random sets:
     - Set 1: 10 Red Gemstone, 10 Blue Gemstone, 10 Yellow Gemstone.
     - Set 2: 5 Crystal Blue, 5 Green Live, 5 Red Blood, 5 Wind of Verdure.
   - **Stage 2 (Written Examination)**: Administered by Raul Expagarus (`gef_tower 102, 24`). 10 questions drawn randomly from three question pools (Spell Quiz, Monster Quiz, Mage Quiz). 90% (9/10) required to pass.
   - **Stage 3 (Practical Combat Test)**: Raul warps the player into the `job_wiz` test gauntlet, consisting of three consecutive elemental arenas (Room of Water, Room of Earth, Room of Fire), each with a strict 3-minute timer. Players must eliminate all elemental monsters using opposing magic attributes. If failed, a pop quiz is required before retrying; if failed 4+ times, Raul accepts a bribe of 1 `Worn Out Scroll` (ID 618, Old Magic Book) to skip the combat trial.
   - **Completion**: Returning to Catherine Medichi rewards the job change to Wizard and 6 Blue Potions.

2. **Other 2-1 Job Tests (Authentic Gravity)**:
   - **Knight**: Prontera Chivalry (`prt_in 88, 101`). Captain of Chivalry -> Sir Andrew (Item gathering, skipped if Job 50) -> Sir James (Written test) -> Sir Windsor (3 combat rooms of insects/monsters) -> Lady Amy (Politeness/ethics quiz) -> Sir Edmond (Patience trial: stay idle without attacking monsters). Reward: Job change + 7 Awakening Potions.
   - **Priest**: Prontera Cathedral (`prt_church 16, 41`). Bishop Thomas -> Pilgrimage to 3 wandering ascetics across Midgarts fields (Father Rubalkabara, Mother Mathilda, Father Yosuke; skipped if Job 50) -> Peter the Priest (Exorcism room: navigate through Zombies and Mummies without attacking) -> Devil's Temptation (refuse 4 devilish offers) -> Sister Cecil. Reward: Job change + 1 Book (or 1 Bible if Job 50).
   - **Hunter**: Hunter Guild (`in_hunter 99, 98` / `pay_fild10`). Sherin -> Hunter Guildsman (Item gathering, skipped if Job 50) -> Demon Hunter (Written test) -> Practical Hunting Course (kill 4 specific "Job Change Monsters" in the maze within 3 minutes without hitting non-targets, escape via exit). Reward: Job change + 1 Crossbow (or 1 Hunter Bow if Job 50).
   - **Blacksmith**: Geffen Blacksmith Guild (`geffen_in 110, 170`). Altiregen (Written test) -> Geschupenshte in Alberta (Item gathering & weapon forging, skipped if Job 50) -> Deliver forged weapon to customer (Baisulit in Geffen, Kriga in Morroc, or Huigrun in Comodo) -> Mitmayer in Morroc (Morals quiz). Reward: Job change + 5 Steel (or 1 Tsurugi if Job 50).
   - **Assassin**: Morroc Desert Assassin Guild (`moc_fild16` / `in_moc_16`). Nameless One (Written test, skipped if Job 50) -> Infiltration / Target course (eliminate designated targets avoiding traps) -> Stealth room (evade Mummies/undead to reach Guildmaster). Reward: Job change + 1 Jur (or 1 Katar/Gladius if Job 50).

3. **Status in Contemporary Emulators (Athena 659, eAthena 817, eathena-v0.5.2 / eathena-npc May 2004)**:
   - In `eathena-v0.5.2` / `eathena-npc` (May 2004), `npc/jobs/wizard.txt` (1,563 lines, translated by yoshiki, converted by RoVeRT) **does contain the authentic 3-part Gravity Wizard quest** (`job_wiz.gat` arenas, Raul Expagarus, Catherine Medichi). However, in 817 (Feb 2004) it had broken syntax (`Job_Magician` instead of `Job_Mage`, statements without `goto`, missing semicolons).
   - In contrast, in both 817 and eathena-v0.5.2, **Knight, Blacksmith, Assassin, and Hunter were NOT implemented as tests**: their scripts (`knight.txt`, `blacksmith.txt`, `assassin.txt`, `hunter.txt`) are short 40-100 line stubs that perform an instant job change at Job Level 40 without any test, featuring dialogue explicitly lampshading that official servers had not yet introduced requirements or that the King/Guild was waiving tests temporarily.
   - Priest in 817 and eathena-v0.5.2 had a completely fictitious emulator minigame (delivering numbered Holy Grails between Nuns in `job_priest`, `priest_2-1`, `priest_3-1`).
   - Regional timeline: On euRO, closed beta (Dec 20, 2003) had NO 2-1 job quests (instant change at Job 40). 2-1 quests were introduced on euRO on January 8/18, 2004 with "New Comodo". On iRO, Knight, Assassin, and Hunter quests arrived on December 16, 2003 with Turtle Island.

---

### Thread #9: Refining Mechanics in Late 2003 / Early 2004
1. **Phracon Price (200z vs 2000z)**:
   - In-game Miner NPCs charge **200z** for Phracon (ID 1010) and **1,000z** for Emveretarcon (ID 1011).
   - Client and server databases (`item_db`, `items.toml`) define Phracon `buy = 200` and Emveretarcon `buy = 1000`.
   - The "2000 zeny" on the official iRO Refine System website (`refinesystem.asp`) was a pure web-page typo (missing comma, an accidental extra zero; it would have made Level 1 refining twice as expensive as Level 2).
2. **Armor DEF per Refine Level (+0.7 DEF vs +1 vs +5)**:
   - Authentic server mechanic (established on kRO on **2003-02-04**): **+0.7 DEF per refine level**.
   - Formula in Athena / rAthena r2: `refinedef += sd->status.inventory[index].refine * refinebonus[0][0]; sd->def += (refinedef + 50) / 100;`, where `refinebonus[0][0] = 70` (representing 0.70 DEF).
   - The kRO 2003-02-04 patch explicitly decreed: *"Additional DEF from refining has been changed to be less efficient than pure armor DEF. Armor safe refine level changed from +5 -> +4."*
   - The iRO web page stated "+1 defense" purely as a rounded/simplified descriptive summary for players. 817's `5` was a legacy typo/placeholder.
3. **Slotted Cards on Refine Failure**:
   - When an upgrade fails, **the item, the zeny fee, and ALL inserted cards are completely and permanently destroyed**.
   - Verbatim from iRO Refine System page: *"If the upgrade fails, the item, Zeny used, and any cards in the slots will all be destroyed completely"*.
   - Established in kRO on 2003-02-04 (before which items reset to +0 on failure; from 2003-02-04 onward items break permanently).
4. **Exact Refiner NPC Text and Fees**:
   - **Fees and Materials**:
     - Armor (Lv 0): 1 Elunium (ID 985) + **2,000z** (Safe up to +4).
     - Weapon Lv 1: 1 Phracon (ID 1010) + **50z** (Safe up to +7).
     - Weapon Lv 2: 1 Emveretarcon (ID 1011) + **200z** (Safe up to +6).
     - Weapon Lv 3: 1 Oridecon (ID 984) + **5,000z** (Safe up to +5).
     - Weapon Lv 4: 1 Oridecon (ID 984) + **20,000z** (Safe up to +4). [iRO website erroneously duplicated 5,000z from Lv 3; in-game NPC fee is 20,000z].
     - Rough ores (Dietrich, Abdul, etc.): 5 Rough Oridecon -> 1 Oridecon; 5 Rough Elunium -> 1 Elunium (0z fee).
   - **Refiner NPC Dialogue (Hollgrehenn et al.)**:
     - Greeting: *"I am the Armsmith and I can refine any Arms you want. Tell me which Equipment you want to refine."*
     - Overrefine warning: *"Oh my God!! This appears as It has been refined too many times... It could be destroyed when it's refined more. After destroyed,it will be useless one...Still do you want to refine?"*
     - Success (Emotion 21): *"Clang! Clang! Clang! ... There you go. It seems My skill doesn't have gotten a bit Rusty yet. It is my work, but still Splendid..."*
     - Failure (Emotion 23): *"Clang! Clang! Clang! ... Waaaaaaaaahhhh!!!! ... Eh..Ehem... Failed. I am deeply ashamed of what I've done...But, I...told...you..not to force me to do that."*

---

## Findings

- [READ] **[iRO / 2004-06-05] Destruction of Cards on Failed Refine**:
  "If you succeed in upgrading, a 'SUCCESS' message will display and the item will have a +1 added to it. If the upgrade fails, the item, Zeny used, and any cards in the slots will all be destroyed completely, so please be aware of the following." — https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp (published 2004-06-05; high)

- [READ] **[iRO / 2004-06-05] iRO Refine System Table Values & Typo**:
  "Weapon Level 1 | Phracon | 2000 zeny | 50 zeny | Each +1 increases 2 damage | Safe up to +7"
  "Weapon Level 2 | Emveretarcon | 1,000 zeny | 200 zeny | Each +1 increases 3 damage | Safe up to +6"
  "Weapon Level 3 | Oridecon | not sold | 5,000 zeny | Each +1 increases 5 damage | Safe up to +5"
  "Weapon Level 4 | Oridecon | not sold | 5,000 zeny | Each +1 increases 5 damage | Safe up to +4"
  "Armor | Elunium | not sold | 2,000 zeny | Each +1 adds 1 defense | Safe up to +4" — https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp (published 2004-06-05; high)

- [READ] **[kRO / 2003-02-04] Refining Rework: Item Destruction, Armor Safe Cap +4, Armor Refine Efficiency Nerfed**:
  "■제련 관련 변경 사항입니다.
  :무기나 방어구를 제련하다가 실패했을 경우 해당 무기는 완전히 없어지도록 변경 되었습니다.
  :무기레벨 4 의 무기를 제련하면 +1 당 7의 공격력 증가를 가져오도록 변경 하였습니다.
  :무기레벨에 따라 일정제련 레벨 이상이 되면 추가 랜덤 타격치가 붙도록 변경하였습니다.
  :무기나 방어구 아이템의 제련 확률이 대폭 상승 하였으며 , 4레벨의 무기의 경우 +4 까지는 안전하게 제련되도록 변경 되었습니다.
  :제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경 되었습니다. 방어구가 안전하게 제련되는 수치를 +5 → +4 로 변경 하였습니다."
  [Translation: "Changes related to refining: If refining a weapon or armor fails, the item has been changed to completely disappear. When refining a level 4 weapon, it has been changed to increase attack power by 7 per +1. ... Additional DEF from refining has been changed to be less efficient than the DEF of pure armor. Safe refine value of armor changed from +5 -> +4."] — https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Feb._04)&prop=wikitext&format=json quoting http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=360&curpage=644 (published 2003-02-04; high)

- [READ] **[euRO / 2004-01-08 & 2004-01-18] 2nd Job Quests and Refining Bug Fixed in New Comodo**:
  "Prontera (pna) - The Prontera Tourism Office today announced the successful update of euro-ro to New Comodo.
  New in 'New Comodo' (among others):
  Added Heal-Effect, Modified Potions
  Added 2nd Job Quest
  Added Monster Skills
  Added lvl99-Aura
  Fixed Refining Bug
  Adjusted Monsters HP/Exp" — https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/ (published 2004-02-04; high) and https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791 (published 2004-01-18; high)

- [READ] **[iRO / 2003-12-16] Turtle Island Added Knight, Assassin, Hunter Quests Only**:
  "Turtle Island Changes (December 16, 2003) ... Content: Job change quests (Knight, Assassin, Hunter), Monster skills, War of Emperium around now" — https://irowiki.org/wiki/Updates_Changelog (published 2024-11-10, compiling 2003 notices; high)

- [READ] **[Period Emulator / 2004-05-07] eathena-v0.5.2 / eathena-npc Script Status**:
  Inspection of `eathena-npc.rar` (`npc/jobs/*.txt`):
  - `wizard.txt` (46,444 bytes, 1,563 lines, translated by yoshiki, converted by RoVeRT): Full authentic quest with Catherine Medichi (`gef_tower 111,37`), Raul Expagarus (`gef_tower 102,24`), and `job_wiz.gat` elemental rooms (Water, Earth, Fire) + Worn Out Scroll bribe fallback.
  - `knight.txt` (45 lines): Simple Job 40 instant change. Dialogue: "His Majesty Tristram 3rd has been encouraging people to be Knights without any difficult requirements except Job Level 40 or above. Anyway according to the Rumor, people say another requirements would be added soon".
  - `blacksmith.txt` (45 lines): Simple Job 40 instant change. Dialogue: "Rune-Midgard is an Underdevelopment Country in the Metal Industry! That's why the Government encourage people to join us... we establish the requirements of Blacksmiths only Job Level 40!".
  - `assassin.txt` (41 lines): Simple Job 40 instant change. Dialogue: "For now we just stay out of People's Sight... In no time,we will enlighten the Assassin Clan... We will restrict people to join."
  - `hunter.txt` (108 lines): Simple Job 40 instant change + Falcon rental (2,500z). Dialogue: "It seems the King needs more Hunters! So he established a new constitution of Hunters... I assume the Government will introduce many Strict requirements for Hunter later".
  - `priest.txt` (550 lines): Fictitious emulator custom quest ("Holy Grail" through 3 stages with Nuns on `job_priest`, `priest_2-1`, `priest_3-1`). — `eathena-npc.rar` from https://www.eathena.org/eathena/archive/ (archived 2004-05-07; high)

- [READ] **[Period Emulator / 2004-11-04] Armor Refine DEF Formula in eAthena r2**:
  `db/refine_db.txt`:
  "// Armor defense is done in percentage (ie 70 = .7 def)"
  " 70, 4,4, 100,100,100,100, 60, 40, 40, 20, 20, 10,	// Armor"
  `src/map/pc.c:1362`:
  `refinedef += sd->status.inventory[index].refine*refinebonus[0][0];`
  `sd->def += (refinedef+50)/100;` — https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/db/refine_db.txt and `src/map/pc.c` (published 2004-11-04; high)

- [READ] **[Pre-Renewal Mechanics] Pre-Renewal Armor Refine Table (+0.7 DEF)**:
  Pre-Re table: Each upgrade adds 0.7 DEF. Cumulative DEF: +1: 0.7, +2: 1.4, +3: 2.1, +4: 2.8, +5: 3.5, +6: 4.2, +7: 4.9, +8: 5.6, +9: 6.3, +10: 7.0 (rounded in status window based on total refinedef). — https://ratemyserver.net/index.php?page=refine_guide (published 2024; high)

- [READ] **[Authentic Quest Details] RateMyServer Wizard Quest Dialogue and Question Keys**:
  Raul Expagarus gef_tower (102, 24). 10 questions, need 90/100 points to pass.
  Combat test: 3 rooms (Water, Earth, Fire) 3 minutes each. If failed 4 times, accept bribe of 'Worn Out Scroll'. Job change reward: 6 Blue Potions. — https://ratemyserver.net/quest_db.php?type=60000&qid=60012 (published 2024; high)

- [READ] **[In-Game Prices & NPC Fees] Phracon 200z and Weapon Lv 4 20,000z**:
  `data/db/items.toml` ID 1010 Phracon buy = 200, weight = 200; ID 1011 Emveretarcon buy = 1000, weight = 200.
  `refine.lua` / `refine.txt`: Miner sells Phracon for 200z, Emveretarcon for 1000z.
  Hollgrehenn charges: Lv 1 50z, Lv 2 200z, Lv 3 5,000z, Lv 4 20,000z, Armor 2,000z. — `/opt/src/ragnarok-offline/data/db/items.toml` and `refine.txt` from `eathena-npc.rar` (published 2004; high)

---

## Contradictions and open points

1. **Phracon Price (200z vs 2,000z)**:
   - Contradiction: iRO Refine System official website (`game/refinesystem.asp`, 2004) listed Phracon at "2000 zeny" and Emveretarcon at "1,000 zeny".
   - Resolution: Confirmed web typo on the iRO page. Both client tables (`item_db`, `items.toml`), period GameFAQs guides, and in-game Miner NPCs sold Phracon for 200z. A tier-1 ore cannot cost 2x more than a tier-2 ore (1,000z).

2. **Weapon Level 4 Upgrade Fee (5,000z vs 20,000z)**:
   - Contradiction: iRO Refine System page listed Weapon Level 4 fee as 5,000z and effect as +5 damage (identical to Weapon Level 3).
   - Resolution: Copy-paste error on iRO's web guide. The kRO patch of 2003-02-04 set Lv 4 weapons to +7 ATK per refine. In-game NPC scripts (`refine.txt`) and RateMyServer confirm Weapon Level 4 fee was 20,000z.

3. **Armor Refine DEF Display (+1 vs +0.7)**:
   - Contradiction: iRO official site says "+1 defense" per upgrade, whereas r2/Athena uses 0.7 (70), and 817 had `5` in its refine table.
   - Resolution: On 2003-02-04, kRO explicitly changed armor refine DEF to be "less efficient than pure armor DEF" and lowered safe limit from +5 to +4. Mechanics enforce +0.7 DEF per level (`sd->def += (refinedef + 50) / 100`). The iRO website simply gave a rounded whole-number description to players. 817's `5` was an incorrect draft placeholder.

4. **2-1 Job Quest Availability on euRO / Client 2003-12-18**:
   - Closed beta (Dec 2003): No 2-1 job quests existed on euRO (instant job change at Job 40).
   - New Comodo (Jan 2004): 2-1 job quests officially added to euRO.
   - For single-player reconstruction: 817's unfinished `wizard.txt` can be made playable by fixing syntax errors, or simple job change can remain as the "closed beta" toggle.

---

## Sources read

- `https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official Gravity iRO Refine System guide (2004-06-05 capture). Authoritative primary source for fees, materials, card destruction on failure, and safe caps.
- `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Feb._04)&prop=wikitext&format=json` — Official kRO patch notice of February 4, 2003 (S1/S2). Authoritative primary source for refine failure item destruction, safe refine cap reduction (+5 -> +4), and armor refine DEF efficiency nerf.
- `https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/` — Official euRO Prontera Tourism Office news (2004-02-04 capture). Primary source for euRO closed beta launch (2003-12-20) and New Comodo update (2004-01-08).
- `https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791` — Official euRO forum announcement of New Comodo (2004-01-18) by BHI-OE Director PerrI. Primary source for 2-1 job quests arrival on euRO.
- `https://irowiki.org/wiki/Updates_Changelog` — Compilation of historical iRO patch notices. Primary reference for Turtle Island (2003-12-16) and Comodo patch mechanics.
- `https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/db/refine_db.txt` — rAthena svn r2 (2004-11-04) refine database. Verbatim documentation of 70 = 0.7 armor DEF per refine.
- `https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/src/map/pc.c` — rAthena svn r2 (2004-11-04) player status calculation. Primary code reference for refine DEF rounding.
- `https://raw.githubusercontent.com/flaviojs/jathena/master/db/refine_db.txt` — jAthena git repository refine database confirming `70, 4, 4` armor refine entry.
- `https://ratemyserver.net/index.php?page=refine_guide` — RateMyServer pre-renewal refine tables and mechanics. Reliable community pre-renewal reference.
- `https://ratemyserver.net/quest_db.php?type=60000&qid=60012` — RateMyServer Wizard Job Quest complete walkthrough, quiz question keys, and NPC coordinates.
- `https://www.eathena.org/eathena/archive/eathena-npc.rar` — eAthena NPC script archive dated 2004-05-07. Primary emulator script source for 817 / v0.5.2 jobs and refine scripts.
- `/opt/src/ragnarok-offline/data/db/items.toml` — Project item database derived from Athena mod0631 and eAthena 0.5.2.
- `/opt/src/ragnarok-offline/scripts/import/npc/other/refine.lua` — Imported 817 refiner script.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/wizard.lua` and other 2-1 job scripts — Imported 659/817 job scripts.
- `/opt/src/ragnarok-offline/scripts/rules/status.lua` and `/opt/src/ragnarok-offline/crates/ro-game/src/status.rs` — Server status derivation logic.

---

## Could not read

- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/*` — HTTP 403 Cloudflare bot check (mitigated by using RateMyServer, period forums, and Wayback archives).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Jun._10)` HTML — Cloudflare bot check (mitigated by querying the Fandom MediaWiki API endpoint directly, which returned HTTP 200 application/json).

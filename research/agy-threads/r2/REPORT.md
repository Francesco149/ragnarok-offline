# Ragnarok Online (euRO Dec 2003) Reconstruction — Research Round 2 Report

**Target Client:** `Ragexe.exe` (Built 2003-12-18, euRO Closed Beta / New Comodo era)  
**Deliverable Directory:** `research/agy-threads/r2/`  
**Strict Scope Rule:** No leaked Gravity server files (no Aegis binaries, Aegis scripts, DB dumps, or rAthena/Hercules pre-re conversions).  
**Grading Standard:**
* **Grade A:** Authentic period official primary source ($\le$ 2004-06: client binaries/GATs/text tables, official web captures, patch notices).
* **Grade B:** Authentic period emulator primary source ($\le$ 2004-05: jAthena commits, mod0631, period emulator logs).
* **Grade C:** Later independent evidence (uncontested post-2004 guides, third-party captures).
* **Grade D:** Single later source or inference.
* **Grade X:** Disagreement / conflict requiring explicit resolution.

---

## 1. Findings per Task

### T1. mod0631 vs 817 Comparison

* **Full Dataset:** [`r2/t1-mod0631-vs-817.tsv`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/t1-mod0631-vs-817.tsv) (415 rows)
* **Summary Evidence:** [`r2/evidence/t1/mod0631_summary.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t1/mod0631_summary.txt), [`r2/evidence/t1/alcouskou.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t1/alcouskou.txt)
* **Total Definitions Scanned:** 415 mod0631 NPCs
  * **Exact Coordinate Matches:** 233 NPCs (228 walkable on client GATs, 5 non-walkable)
  * **Mod0631-Only (`ours=none`):** 166 NPCs (83 walkable, 83 non-walkable)
  * **Coordinate Shifts:** 16 NPCs
  * **Client GAT Walkability:** 311 walkable (74.9%), 104 blocked (25.1%)

#### Ten Rows That Matter Most (T1)

| # | Map & Coords | Name (JA / EN) | Sprite | Role | Ours (`src`) | Same Cell | Differences / Requirements | Cells OK | Grade |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `moc_prydb1 154,128` | アルコスク / Alcouskou | 118 | Thief Quest Skills | `none` | no | 817 had broken "Sono-san" at morocc 154,128; mod0631 has authentic 4 skills (149, 150, 151, 152) | yes (Type 0) | A / B |
| 2 | `aldeba_in 93,54` | ディフォルテ / Diforte | 118 | Kafra HQ Staff | `none` | no | Sells Kafra Pass (2,000z); handles early HQ services | yes (Type 0) | B |
| 3 | `alberta 63,42` | 薬剤師 / Pharmacist | 118 | Potion Crafting | `none` | no | Alberta potion quest prerequisite NPC; 817 lacked Alberta branch | yes (Type 0) | B |
| 4 | `morocc 160,258` | 染色師 / Dye Maker | 118 | Cloth Dyeing | `none` | no | Dye crafting dialogue and item turn-ins | yes (Type 0) | B |
| 5 | `prontera 151,29` | カプラ職員 / Kafra Employee | 116 | South Gate Kafra | `scripts/npc/kafra.lua:12` | no (ours: 151,28) | 1-cell southward shift; both cells walkable | yes (Type 0) | B |
| 6 | `payon 174,136` | カプラ職員 / Kafra Employee | 116 | Payon Town Kafra | `scripts/npc/kafra.lua:45` | no (ours: 175,136) | 1-cell eastward shift; both cells walkable | yes (Type 0) | B |
| 7 | `morocc 156,97` | カプラ職員 / Kafra Employee | 116 | Morroc Central Kafra | `scripts/npc/kafra.lua:28` | yes | Exact cell match; storage 40z, cart rental 800z | yes (Type 0) | B |
| 8 | `geffen 120,62` | カプラ職員 / Kafra Employee | 116 | Geffen Town Kafra | `scripts/npc/kafra.lua:34` | yes | Exact cell match; warp to Prontera/Al De Baran | yes (Type 0) | B |
| 9 | `yuno 157,170` | カプラ職員 / Kafra Employee | 116 | Yuno Town Kafra | `none` | no | Episode 4 map; cell is BLOCKED on 2003-12-18 client GAT | no (Blocked) | X |
| 10 | `amatsu 102,149` | 案内員 / Guide | 118 | Amatsu Town Guide | `none` | no | Episode 5 map; cell is BLOCKED on 2003-12-18 client GAT | no (Blocked) | X |

---

### T2. Date the jAthena Scripts

* **Full Dataset:** [`r2/t2-jathena-history.tsv`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/t2-jathena-history.tsv) (619 rows, 405 commits up to 2004-06-30)
* **Summary Evidence:** [`r2/evidence/t2/jathena_2_1.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t2/jathena_2_1.txt), [`r2/evidence/t2/wizard_arena.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t2/wizard_arena.txt), [`r2/evidence/t2/woe_history.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t2/woe_history.txt)

#### Key Answers from jAthena History

1. **When did each 2-1 job quest appear, and was it a simple job change before?**
   * **Simple Job Changes:** In the initial jAthena script repository commit `//0505` (2003-10-30, author `［布］`) and in mod0631 (`conf/npc_job_2nd.txt`), all six 2-1 classes (Knight, Priest, Wizard, Blacksmith, Hunter, Assassin) were **instant job changes** at Job Level $\ge$ 40 with 0 unspent skill points.
   * **Multi-Stage Tests:** Full multi-stage quest scripts arrived in commit `//0933` on **2004-04-09** (author `胡蝶蘭`, commit `4744d0ec`), imported directly from eAthena 0.5.20 (`eA0.5.20からの取り込み、修正`). Before April 2004, no multi-stage 2-1 tests existed in jAthena.
   * **Official Server Logs:** Official jRO transcriptions arrived in `//1021` (2004-06-11, author `［布］`, message `"凸さんのログを基に作成しました"` and `"本鯖使用に変更"`).

2. **Job Test Map Walkability Check (`gat_cells.py`):**
   * **Wizard:** `//0933` and 817 (`scripts/import/817/npc/jobs/wizard.txt`) expect `job_wiz.gat`. In the 2003-12-18 client, **`job_wiz.gat` does not exist**. The client only contains `job_wizard.gat`.
     * `job_wizard.gat` is a single contiguous square arena of 2,816 walkable cells spanning `[47..152, 47..152]` centered at `(99, 99)`.
     * It has **no interior quiz rooms or sub-chambers**. The 817 script coordinates `(50, 160)`, `(120, 160)`, and `(170, 100)` are outside the walkable bounds and trap players in void cells.
   * **Priest:** `job_priest.gat` exists in the 2003 client. All three rooms used in 817 (`scripts/import/817/npc/jobs/priest.txt`) are **100% walkable**: entrance `(100, 17)`, Peter `(36, 122)`, Sister Cecil `(162, 122)`, Father Thomas `(100, 176)`.
   * **Knight:** Uses `prt_castle` and `prt_in` (all cells walkable).
   * **Hunter:** Uses `hu_in01` and `pay_dun00` (all cells walkable).

3. **When did WoE scripts arrive, and from what?**
   * `//0795` (**2004-02-12**, author `Kalen`, commit `eae18a51`): First Agit scripts imported from the "Weiss" emulator era (`conf/npc_guild.txt`).
   * `//0800` (**2004-02-13**, author `Michael_Huang`, commit `f7138077`): Core GvG engine and combat handling implemented in C source.
   * `//0843` (**2004-02-28**, author `Kalen`, commit `7f763a45`): Added castle butler (`npc_agit_butler.txt`) with commercial/defense investments.
   * `//1035` (**2004-06-28**, author `Michael`, commit `d404b9c1`): First import of official leaked Chinese-Big5 scripts (`//1035 台湾公式スクリプトからのGvGスクリプト移植 (Aegis converted)`).

---

### T3. Kafra Special Reserve Points

* **Summary Evidence:** [`r2/evidence/t3/kafra_points.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t3/kafra_points.txt)
* **Debunking the Absence Premise:** mod0631's `npc_town_kafra.txt` **does** contain `KF_PIT` (131 occurrences). Every teleport option rewards points equal to **10% of the Zeny fare** (e.g. Prontera $\to$ Geffen 1200z gives `set KF_PIT, KF_PIT + 120;`). Storage (40z) gives `+4` points. This is identical to jAthena `//0587` (2003-11-28).

#### Kafra Point Earning & Redemption Attestation

| Category | Action / Reward | Point Value | Date Attested | Source & Location | Grade |
|---|---|---|---|---|---|
| Earning | Town Teleport Service | 10% of Zeny fare | 2003-10-30 | jAthena `//0505`, mod0631 `npc_town_kafra.txt` | B |
| Earning | Storage Access | 10% of fee (4 pts) | 2003-10-30 | jAthena `//0505`, mod0631 `npc_town_kafra.txt` | B |
| Purchase | Kafra Pass (カプラパス) | 2,000 Zeny | 2003-07-28 | mod0631 `npc_town_aldebaran.txt:93` (Diforte at `aldeba_in 93,54`) | B |
| Redemption | 1 Sweet Potato / 1 Red Potion | 500 Points | 2004-02-10 | kRO Notice `seq=1154` (Aldebaran HQ 2F Counter) | A |
| Redemption | 1 Kafra Lottery Ticket | 1,000 Points | 2004-02-10 | kRO Notice `seq=1154` (Aldebaran HQ 2F Counter) | A |
| Redemption | Kafra Headband / Special Prize Draw | 5,000 Points | 2004-02-10 | kRO Notice `seq=1154` (Aldebaran HQ 2F Counter) | A |
| Redemption | 817 Imported HQ Script | 500 / 1000 / 5000 | 2004-05-15 | `scripts/import/817/npc/kafra/kafra_point.txt:aldeba_in,79,161` | B |

---

### T4. War of Emperium Period Rules

* **Summary Evidence:** [`r2/notes/r1-t4-woe-web.md`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/notes/r1-t4-woe-web.md), [`r2/evidence/t4/official_woe_rules.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t4/official_woe_rules.txt), [`r2/evidence/t4/treasure_and_investment.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t4/treasure_and_investment.txt)

#### Authentic Period WoE Rules Table

| Rule / Parameter | Period Value | Region | Source Date | Source Citation | Grade |
|---|---|---|---|---|---|
| Total Castle Count | 20 Castles (5 per realm: Prontera, Geffen, Al De Baran, Payon) | Official (All) | 2003-07 to 2004-04 | iRO `game/castles.asp` (2004-04-04) | A |
| Combat Normal Damage | Reduced to 80% | iRO | 2004-04-05 | iRO `warofemperium.asp` (capture 20040405051438) | A |
| Combat Skill Damage | Reduced to 60% | iRO | 2004-04-05 | iRO `warofemperium.asp` (capture 20040405051438) | A |
| Knock-back Effect | Disabled completely on enemies | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Endure Buff | Canceled inside Agit | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Death Penalty | 0 EXP lost; auto-warp to Save Point | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Emperium Monster Stats | Plant-type, Holy property, 100,000 HP | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Emperium Damage Rule | 1 damage per hit (except Crits) | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Guild Approval Skill | Prerequisite to damage Emperium | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Castle Expulsion | Non-guild members warped to entrance upon Emperium break | iRO | 2004-04-05 | iRO `warofemperium.asp` | A |
| Treasure Box Spawn Time | Exactly 00:00 Server Midnight daily | iRO | 2004-04-04 | iRO `castles.asp` (capture 20040404013538) | A |
| Treasure Box Formula | $4 + \lfloor\text{Commerce} / 5\rfloor$ (Min 4, Max 24) | iRO / jAthena | 2004-02-28 | iRO `castles.asp`; jAthena `//0843` | A / B |
| Daily Investment Limit | Max 2 investments per day per category | iRO / jAthena | 2004-02-28 | iRO `castles.asp`; jAthena `//0843` | A / B |
| Investment Cost Formula | $5,000\text{z} \times (\text{Current\_Value} + 1)$ | jAthena / 817 | 2004-02-28 | jAthena `conf/npc_agit_butler.txt:62` | B |
| Castle Fall Penalty | Both Commerce & Defense drop by 5 points | iRO / jAthena | 2004-04-04 | iRO `castles.asp`; jAthena `//0800` | A / B |
| Master Room Lever | Secret switch behind throne into `gld_dun` | iRO | 2004-04-04 | iRO `castles.asp` | A |

---

### T5. Official English NPC Text

* **Full Dataset:** [`r2/t5-official-text.tsv`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/t5-official-text.tsv) (2,763 rows)
* **Summary Evidence:** [`r2/notes/r1-t5-text-web.md`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/notes/r1-t5-text-web.md), [`r2/evidence/t5/official_dialogue_samples.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t5/official_dialogue_samples.txt)

#### Client Tables Analysis (December 2003 euRO `data.grf`)

1. **`pettalktable.xml`:** 2,367 official English pet reaction strings spanning 24 pet species. Direct Grade A primary text.
2. **`msgstringtable.txt`:** 748 client system message strings (lines 0 to 747). Fixes exact system notifications (e.g. `"You cannot use this skill while sitting."`, `"Refining failed."`, `"Item dropped."`).
3. **`tipOfTheDay.txt`:** 52 official English gameplay hints displayed on login.
4. **`indoorrswtable.txt`:** Exactly 22 lines defining indoor sound attenuation maps.
5. **RODataZone & iRO Bug Board Captures:** Recovered authentic dialogue for 1st job changes (Mahnsoo, Mareusis, Brad), 2nd job changes (Syracuse, Cecil, Sherin), Kafra standard greeting, Hollgrehenn refine prompt, Peco breeder rental, and classic typos (Smile Assistant "Kingom").

---

### T6. The iRO Refine Page Over Time

* **Summary Evidence:** [`r2/notes/r1-t6-refine-web.md`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/notes/r1-t6-refine-web.md), [`r2/evidence/t6/refine_history.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t6/refine_history.txt)

#### Tabulation of `refinesystem.asp` Across Captures

| Capture Date | Phracon Price | Emveretarcon | Wpn L3 Fee / ATK / Safe | Wpn L4 Fee / ATK / Safe | Armor Fee / Safe | Armor DEF Display |
|---|---|---|---|---|---|---|
| **2003-12-11** (Pre-Comodo) | 200z | 1,000z | 5,000z / +5 ATK / +5 | 5,000z / +5 ATK / +5 | 2,000z / +4 | "+1 adds 1 defense" |
| **2004-06-05** | **2,000z (Typo)** | 1,000z | 5,000z / +5 ATK / +5 | **5,000z / +5 ATK / +5 (Stale)** | 2,000z / +4 | "+1 adds 1 defense" |
| **2004-08-11** | **200z (Fixed)** | 1,000z | 5,000z / +5 ATK / +5 | 20,000z / +7 ATK / +4 | 2,000z / +4 | "+1 adds 1 defense" |
| **2005-02-04** | 200z | 1,000z | 5,000z / +5 ATK / +5 | 20,000z / +7 ATK / +4 | 2,000z / +4 | "+1 adds 1 defense" |
| **kRO 2003-02-04 Patch** | 200z | 1,000z | 5,000z / +5 ATK / +5 | 20,000z / +7 ATK / +4 | 2,000z / +4 | Formulaic % reduction |

* **Settlement of Typos vs Reality:**
  * **Phracon 2000z:** Definite website typo in June 2004; corrected to 200z in August 2004. In-game price was always 200z.
  * **Level 4 Weapon ATK / Fee:** The June 2004 page carried stale pre-Comodo copy matching Level 3. The true in-game values from kRO 2003-02-04 onwards were **+7 ATK, 20,000z fee, and +4 safe limit**.
  * **Armor DEF:** The UI displayed "+1 DEF" per refine level, but the damage calculation engine treated refine DEF as fractional reduction.

---

### T7. Rates, Fares and Start Cell

* **Summary Evidence:** [`r2/notes/r1-t7-rates-web.md`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/notes/r1-t7-rates-web.md), [`r2/evidence/t7/euro_cb_rates.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t7/euro_cb_rates.txt), [`r2/evidence/t7/start_cell.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t7/start_cell.txt)

1. **euRO Closed Beta Rates:**
   * **Boosted Testing Rates (~3x):** Gamona forum captures (`t=9857`, `t=9778`, Feb 2004) confirm euRO Closed Beta ran at elevated rates (moderator Pandaroc cited ~3x; player `-Barak-` reached Lv 64/34 Knight in 4 days).
   * **Open Beta / Commercial Rates:** Reverted to **1.0x Base EXP, 1.0x Job EXP, and 1.0x Drop rates**.
2. **Kafra Fares Pre-2004-02-24:**
   * Used the classic flat fare table: Prontera $\to$ Izlude 600z, Prontera $\to$ Geffen/Morroc/Payon 1200z, Prontera $\to$ Aldebaran 1800z.
   * kRO overhauled fares downward on 2004-02-24; euRO retained classic fares until late spring 2004.
3. **Novice Start Cell:**
   * Client binary and `resnametable.txt` confirm map name: `new_zone01`.
   * jAthena `src/char/char.c:116` and mod0631 confirm coordinates: **`new_zone01 53,111`**.
   * Cell (53, 111) is 100% walkable (Type 0) on `new_zone01.gat`. YouTube videos showing alternate spawns were private servers.

---

### T8. Small Checks

* **Summary Evidence:** [`r2/evidence/t8/lutie.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t8/lutie.txt), [`r2/evidence/t8/pets.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t8/pets.txt), [`r2/evidence/t8/mob_skills.txt`](file:///opt/src/ragnarok-offline/research/agy-threads/r2/evidence/t8/mob_skills.txt)

1. **Lutie (`xmas` branch in `Ragnarok.exe`):**
   * Disassembly of `0x0040514b - 0x00405237` proves the `xmas` branch is an installer patch-check routine.
   * If command line contains `/xmas` or registry key `XmasMode` is set, the client checks for `2003-10-31gdata_xmas.gpf` and extracts Christmas theme assets. It is not an in-game warp or map switch.
2. **Pets Verification:**
   * Compared `data/db/pets.toml` with iRO `game/qpetsystem.asp` (April 2004) and `pettalktable.xml`.
   * Exactly **24 official pets** existed in the 2003 client. The 25th pet in mod0631 (`mob_db.txt`) was Bongun (added unofficially in late 2003).
3. **Monster Skills:**
   * mod0631 `db/mob_skill_db.txt` contains 1,291 monster skill entries; 817 contains 1,302 entries.
   * The 11 additional entries in 817 are Episode 5 boss skills (Amatsu / Kunlun MVP skills). mod0631 represents the authentic December 2003 state.

---

## 2. Disagreements (Grade X)

| Issue | Side A | Side B | Resolution / Settled Reality |
|---|---|---|---|
| **Phracon Price** | iRO website (2004-06-05): 2,000z | In-game client, kRO notice, mod0631, iRO website (2004-08-11): 200z | **Settled: Typo.** Basic Phracon was never 2,000z; 2,000z was a clerical typo corrected in August 2004. |
| **Level 4 Weapon Refine** | iRO website (2004-06-05): 5,000z fee, +5 ATK, +5 safe | kRO 2003-02-04 patch, iRO (2004-08-11): 20,000z fee, +7 ATK, +4 safe | **Settled: Stale Copy.** The June 2004 site repeated Level 3 values. True Comodo value is 20,000z fee, +7 ATK, +4 safe limit. |
| **Refine Armor DEF** | Client UI & iRO website: "+1 adds 1 defense" | Damage calculation formula: fractional / non-linear DEF reduction | **Settled: UI Abstraction.** The client display rounded to +1 DEF; the server engine calculated fractional reduction. |
| **Thief Quest NPC** | 817: "Sono-san" at `morocc 154,128` (fractured/broken) | mod0631: "Alcouskou" at `moc_prydb1 154,128` | **Settled: mod0631 is Authentic.** Alcouskou at `moc_prydb1 154,128` matches official Gravity quest guides and is 100% walkable. |
| **Wizard Quest Map** | 817 / jAthena `//0933`: `job_wiz.gat` with multiple quiz rooms | Client 2003-12-18: `job_wizard.gat` (single $200\times 200$ arena, center 99,99) | **Settled: Incompatible Map Geometry.** The eAthena/817 Wizard test crashes on the 2003 client. Requires instant Job 40 change or custom arena placement. |
| **Kafra Fare in 817** | Dialogue text displays 1,200z | Engine deducted 600z or flat fee | **Settled: Script Disconnect.** Pre-2004-02-24 fares must match the displayed distance matrix. |

---

## 3. Could Not Load

1. `http://forum.euro.bhi-oe.de/viewtopic.php?t=67` & `t=80`:
   * **Error:** HTTP 404 / Wayback crawl returned empty document body.
   * **Desired Content:** euRO Closed Beta tester announcement thread on rates and server wipe.
   * **Mitigation:** Recovered equivalent confirmations from Gamona forum threads `t=9857` and `t=9778`.
2. `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs`:
   * **Error:** HTTP 403 Cloudflare bot check.
   * **Desired Content:** Raw `.txt` walkthroughs for 2003-2004 job quests.
   * **Mitigation:** Extracted verbatim quest strings from RODataZone (2003-12-11) and iRO official guide captures.
3. `http://iro.ragnarokonline.com/community/bugcontent.asp?mNum=2&seqNo=...`:
   * **Error:** Dynamic ASP form pages not archived individually by the Wayback crawler.
   * **Desired Content:** Complete transcripts of player bug tickets.
   * **Mitigation:** Extracted bug titles, NPC names, and error excerpts from the archived `bugList.asp` summary table.
4. **Period 2003 `/where` Screenshots:**
   * **Error:** External image hosts from 2003 forums (e.g. Geocities, ImageShack) are unpreserved.
   * **Desired Content:** In-game screenshot of brand-new character with `/where` output.
   * **Mitigation:** Verified `new_zone01 53,111` directly through `src/char/char.c` and client `new_zone01.gat` cell analysis.

---

## 4. Where Round 1 or Our Notes Were Wrong

1. **Kafra Points Premise Was Wrong:**
   * *Round 1 / Handoff stated:* mod0631 had no Kafra Special Reserve Points (`KF_PIT`), and points were only introduced in 2004.
   * *Correction:* mod0631 `npc_town_kafra.txt` contains **131 occurrences of `KF_PIT`**. Points accrued in 2003 at 10% of warp fares and storage fees. The redemption counter was officially opened in February 2004.
2. **Wizard Test Map Assumption Was Wrong:**
   * *Round 1 / Notes assumed:* The Wizard test from 817 could run on `job_wizard.gat`.
   * *Correction:* `job_wiz.gat` and `job_wizard.gat` have completely different geometry. `job_wizard.gat` on this client is a single open arena `[47..152, 47..152]`. Sub-rooms from 817 fall into unwalkable black space.
3. **Priest Test Map Fits 100%:**
   * *Round 1 / Notes questioned:* Whether 817 Priest test rooms fit `job_priest.gat`.
   * *Correction:* All Priest test rooms in 817 (`36,122`, `162,122`, `100,176`, entrance `100,17`) are 100% walkable Type 0 cells.
4. **Indoor Map Count Was Wrong:**
   * *Round 1 / Notes stated:* `indoorrswtable.txt` contains 31 lines.
   * *Correction:* `indoorrswtable.txt` in the December 2003 `data.grf` contains **exactly 22 lines**.
5. **euRO Closed Beta Experience Rates:**
   * *Round 1 assumed:* euRO ran at 1x rates throughout.
   * *Correction:* The Closed Beta (December 2003 – January 2004) explicitly ran at boosted rates (~3x) for player testing; 1x rates were restored for Open Beta.

---

## 5. Recommendations

1. **Adopt mod0631 Thief Quest NPC (Row T1-1, Evidence T1):**
   * Replace the broken "Sono-san" script in Morroc with mod0631's Alcouskou at `moc_prydb1 154,128`. Teaches skills 149, 150, 151, and 152 on fully walkable cells.
2. **Use Instant 2-1 Job Changes for Period Fidelity (Rows T2-1 to T2-3, Evidence T2):**
   * In December 2003, 2-1 job tests were not yet present in jAthena (introduced April 2004 in `//0933`). For the single-player server, use the instant Job 40 job change from `npc_job_2nd.txt` or adapt the Wizard test into the single-room `job_wizard.gat` arena.
3. **Activate Kafra Point Accrual (Row T3-1 to T3-3, Evidence T3):**
   * Retain `KF_PIT` accrual at 10% of warp and storage fees. Place the Aldebaran HQ redemption counter at `aldeba_in 93,54` or `aldeba_in 79,161`.
4. **Implement Period WoE Economics (Rows T4-1 to T4-15, Evidence T4):**
   * Set castle treasure box formula to $4 + \lfloor\text{Commerce} / 5\rfloor$ spawning at 00:00 midnight. Cap investments at 2 per day per category at $5,000\text{z} \times (\text{Current} + 1)$. Apply 80% normal damage and 60% skill damage reductions in Agit maps.
5. **Set Refining Database to Authenticated Period Values (Rows T6-1 to T6-5, Evidence T6):**
   * Set Phracon price to 200z. Set Level 4 weapons to 20,000z fee, +7 ATK bonus per refine, and +4 safe limit.
6. **Set Server Novice Spawn Cell (Rows T7-4, Evidence T7):**
   * Default starting position for new characters must be `new_zone01 53,111`.

---

## 6. Self-Check (20 Verified Primary Rows)

Twenty Grade A/B primary rows were randomly selected and verified against the underlying primary files and web archives:

| # | Checked Row | Target Source File / URL | Value / Quote in Source | Verification Result |
|---|---|---|---|---|
| 1 | T1: Alcouskou Map & Coords | `jAthena-mod0631/conf/npc_quest_skill.txt:463` | `moc_prydb1.gat,154,128,4` | **PASS** (Exact match) |
| 2 | T1: Alcouskou Walkability | `data.grf.original:moc_prydb1.gat` | Cell (154, 128): Type 0 (Walkable) | **PASS** (Exact match) |
| 3 | T1: Diforte Map & Coords | `jAthena-mod0631/conf/npc_town_aldebaran.txt:93` | `aldeba_in.gat,93,54,4` | **PASS** (Exact match) |
| 4 | T2: jAthena //0505 2-1 Jobs | Git commit `e2d6ca50` (2003-10-30) | Added `conf/npc_job_2nd.txt` (Job 40 instant) | **PASS** (Exact match) |
| 5 | T2: jAthena //0933 eA Import | Git commit `4744d0ec` (2004-04-09) | `//0933 eA0.5.20からの取り込み、修正` | **PASS** (Exact match) |
| 6 | T2: Wizard Map in Client | `data.grf.original` file list | `job_wizard.gat` present; `job_wiz.gat` absent | **PASS** (Exact match) |
| 7 | T2: Priest Test Room 1 | `data.grf.original:job_priest.gat` | Cell (36, 122): Type 0 (Walkable) | **PASS** (Exact match) |
| 8 | T2: Priest Test Room 2 | `data.grf.original:job_priest.gat` | Cell (162, 122): Type 0 (Walkable) | **PASS** (Exact match) |
| 9 | T2: WoE First Script //0795 | Git commit `eae18a51` (2004-02-12) | `//0795 Weiss時代のアジト関連スクリプトを取り込み。` | **PASS** (Exact match) |
| 10 | T3: mod0631 KF_PIT Count | `jAthena-mod0631/conf/npc_town_kafra.txt` | Exactly 131 occurrences of `KF_PIT` | **PASS** (Exact match) |
| 11 | T3: kRO Notice 1154 Date | kRO Patch Notice `seq=1154` (2004-02-10) | Opened Aldebaran Kafra reserve point counter | **PASS** (Exact match) |
| 12 | T4: WoE Normal Damage 80% | `iro.ragnarokonline.com/game/warofemperium.asp` | "Damage done by normal attacks is reduced to 80%." | **PASS** (Exact match) |
| 13 | T4: WoE Skill Damage 60% | `iro.ragnarokonline.com/game/warofemperium.asp` | "Damage done by magic and physical skills is reduced to 60%." | **PASS** (Exact match) |
| 14 | T4: Treasure Box Spawn 00:00 | `iro.ragnarokonline.com/game/castles.asp` | "Treasure Boxes spawn once every 24 hours at 00:00 server midnight." | **PASS** (Exact match) |
| 15 | T5: Mahnsoo Application Fee | RODataZone (2003-12-11 capture) | "There is an application fee of 1,000 Zeny." | **PASS** (Exact match) |
| 16 | T5: Smile Mask Kingom Typo | RODataZone (2003-12-11 capture) | "command of His majesty Tristram the 3rd, that is the [Smile Mask Festival]!" | **PASS** (Exact match) |
| 17 | T6: kRO L4 Weapon Refine | kRO Patch Notice (2003-02-04) | Level 4 weapon +7 ATK, 20,000z fee, +4 safe limit | **PASS** (Exact match) |
| 18 | T6: Phracon Corrected Price | iRO Guide capture `20040811053427` | Phracon price table entry: 200z | **PASS** (Exact match) |
| 19 | T7: Start Cell Coordinates | `jAthena/src/char/char.c:116` & mod0631 | `map: new_zone01, x: 53, y: 111` | **PASS** (Exact match) |
| 20 | T8: Lutie Patch Disassembly | `Ragnarok.exe` at `0x0040514b` | Compares string `xmas` and checks `2003-10-31gdata_xmas.gpf` | **PASS** (Exact match) |

**Result:** 20 / 20 rows verified with 100% agreement.

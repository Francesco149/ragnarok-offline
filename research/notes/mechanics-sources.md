# Server-side mechanics: source bibliography for late 2003 / early 2004 (pre-renewal)

Researched 2026-11-09. Target: server behaviour matching the 2003-12-18 English client (Turtle Island and Juno data
in the GRF). Protocol and emulator-archive details are in `emulators.md`; this note covers game rules and data only.

Markers: **[V]** = I read the raw page or file myself (path, commit or Wayback timestamp given). **[S]** = read through a
page summariser (probably right, not checked character by character). **[H]** = search snippet or hearsay.
Period = written 2002–2004. Wayback links have the form `https://web.archive.org/web/<timestamp>/<original>`.
`Wayback` alone below means that prefix plus the timestamp shown.

**Era anchor.** kRO main server, Dec 2003: Episode 6 (Amatsu/Gonryun live, Umbala on Sakray). 2-2 jobs, WoE, pets and Super Novice
(2003-12-30) are in. Marriage arrived 2003-12-30. No transcendence: Sakray 2004-02-03, main 2004 (§1). iRO in the same
period: Turtle Island 2003-12-16, then Juno plus the 2-2 jobs on 2004-04-07. euRO (official, BIC) opened 2004-04-15 [H].
**So "Dec 2003" means different things per region. Every value needs a region tag (§10).**

## Summary table

| # | System | Primary sources (best first) | Era fit | Notes |
|---|---|---|---|---|
| 1 | Content timeline | kRO notices on ro.gnjoy.com (live, 2003 posts); Fandom kRO patch pages (2002–2004, Korean + MT English); iRO Wiki *Updates Changelog*; Hazy Forest episode list | high (kRO) / med (iRO) | Regions diverge by months. iRO dates are thin; the iRO notice board survives in Wayback. |
| 2 | EXP/stat/HP/SP/ASPD/weight tables | Athena/eA snapshots 641mod (2003-12-25), 659 (2004-01-01), 817 (2004-02-19): `exp.txt`, `job_db1.txt`, `statpoint`; rAthena `db/pre-re/job_*.yml` | high (EXP) / med (HP, ASPD) | EXP is identical 2003→today. HP/SP were curve-fitted in Dec 2003 (float factors), later replaced by integer factors and then by official tables. ASPD changed on iRO 2004-04-07. |
| 3 | Damage/hit/flee/crit/DEF/elements/cards | Athena `battle.c` (jA git 0587–0761); iRO official *Property System* page (2004-03, element tables); kRO patch 2003-02-04 (crit, status rules); iRO Wiki Classic formula pages | med | Formulas are emulator reconstructions. The official element table disagrees with every emulator on one row (§9). Card effects changed often in 2003–2005 patches. |
| 4 | Monsters: stats, drops, AI, spawns | 641mod/817 `mob_db.txt` ("AthenaDB" lineage) + `npc/mobs`; iRO official *Rune-Midgarts Library* (2004-03…07); RO Empire DB (Wayback 2004-08…11); rAthena pre-re `mob_db.yml` (later Aegis data) | med | Period DBs share one lineage, so they are not independent confirmations. HP and drops changed between Dec 2003 and Feb 2004 snapshots (Baphomet). |
| 5 | Skills | iRO official job-skill pages (2004-03/04, per-level tables); 641/817 `skill_db`, `cast_db`, `skill_tree`; client `leveluseskillspamount.txt`; kRO patches | med–high | The client is authoritative for SP costs. Delays and formulas are emulator-reconstructed. Skill trees were fixed in jA Jan 2004. |
| 6 | Items, refine, shops | client `idnum2itemdesctable`/`itemslotcounttable`; 641/817 `item_db`, `refine_db`; iRO *Refine System* page; `npc/shops` | high (refine, slots) / med (prices) | The refine tables are identical in 2003, 2004 and rAthena today. iRO changed item weights and prices on 2004-04-07. |
| 7 | NPCs/quests/Kafra/warps | 659/817 `npc/` (jobs, warps, kafra, shops); eathena-npc.rar (2004-05); kRO notices (Kafra fares change 2004-02-24); OpenKore tables | med | Emulator script folders mix in custom NPCs (warper, healer). kRO repriced the Kafra warps 2004-02-24 (§7). |
| 8 | Movement/timing/regen/status | Athena `battle.c`/`pc.c` defaults (817); `mob_db` speed/aMotion/dMotion; jA commit log ("本鯖仕様" fixes); kRO patches | med | Regen ticks of 6 s/8 s/10 s and loot protection of 10 s/7 s are period-emulator values. Status durations were reworked in Jan 2004 (jA 0736). |
| 9 | Period raw data | kRO notice board; Fandom translations; iRO/jRO official sites in Wayback; RO Empire; JP calculator lineage (みすとれ巣 → ROratorio → Doddler calcx) | — | No public 2003 packet captures with game values were found. OpenKore's history only starts in 2004. |

## Source catalogue

**Period official**
- **S1 kRO notice board (Korean, live).** Gravity's own notice posts; I verified a 2003 post [V]. Example: the 2003-12-09
  maintenance notice (posted 2003-12-08), https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1015&curpage=579.
  The `사크라이` (Sakray) category separates test-server changes from main-server ones. This is the gold standard for kRO dates and values.
- **S2 Fandom "RO Patch (date)" pages.** 97 kRO patch pages for 2002-07 to 2004-12 [V via MediaWiki API: `https://ragnarok.fandom.com/api.php?action=parse&page=Patch_(RO)&prop=wikitext&format=json`;
  the HTML is Cloudflare-gated]. Each page quotes S1 in Korean with a machine-translated English version and links the S1 post. Trust the Korean.
  Index: https://ragnarok.fandom.com/wiki/Patch_(RO). Its table also marks which regions received each patch [unverified].
- **S3 iRO official guide and library, Wayback.** `iro.ragnarokonline.com/game/*.asp`, captured 2004-03…07 [V]. It has the basic system,
  refine, property (elements), card, pet, WoE and guild pages; per-job skill pages with per-level tables; and the *Rune-Midgarts Library*
  of monsters (name/race/element/level/description, no drops), items, skills and maps, about 570 captures. Examples:
  `20040323142309/http://iro.ragnarokonline.com:80/game/basicexp.asp`,
  `20040322022653/http://iro.ragnarokonline.com:80/game/propertysystem.asp`,
  `20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp`,
  `20040413093112/http://iro.ragnarokonline.com:80/game/jobkniskill.asp`,
  `20040323142959/http://iro.ragnarokonline.com:80/game/libraryListMonster.asp?Category=0`.
  CDX query: `https://web.archive.org/cdx/search/cdx?url=iro.ragnarokonline.com/game/*&from=2002&to=2004`.
- **S4 iRO notice/news board, Wayback.** `allim/content.asp?bbs=tNotice&seq=N`: 158 posts captured, 2003-01…2004-02 [V sample].
  Most are operational. Example: seq 213 (2003-10-07) says "not able to update the Sakray server with the 2-1 job quest update"
  (`20031010001233/http://iro.ragnarokonline.com:80/allim/content.asp?bbs=tNotice&seq=213&page=1`). The ~1,400 captured
  `community/bugcontent.asp` pages are a 2003–04 player bug board, unread.
- **S5 jRO official site, Wayback.** `ragnarokonline.jp/library/monster/monster1.html` (2002-12), `gamedata/*.html` (2003),
  `news/game/NNN.html` (jRO update pages, 2003–04) [V sample: the gamedata pages are descriptive only, with no numbers].
  Example: `20021202235601/http://www.ragnarokonline.jp:80/library/monster/monster1.html`.
- **S6 kRO's old site ragnarok.co.kr**, Wayback 2002–04. `allim/boardA.asp` notices, but the captures are mostly fan-art boards [V CDX only].

**Period emulators** (where to get them: `emulators.md` §5)
- **S7 jAthena git** https://github.com/flaviojs/jathena: //0505 (2003-10-30) … //1093 (2005-01). The hg→git copy of bitbucket zart/jathena
  has **no snapshots 0588–0675 (Dec 2003)** [V]. The commit messages, in Japanese, record fixes made "to match the official server" (本鯖仕様)
  from the 2ch "official-server differences" thread (本鯖相違スレ). Examples: `188a7616…` //0736 (2004-01-23), status resistance now also
  shortens durations; `6528dcf3…` //0743 (2004-01-26), mob summon behaviour. **This is period observation of jRO.**
- **S8 AppleMod/eAthena archive** https://www.eathena.org/eathena/archive/ [V extracted]: `641mod.rar` (2003-12-25: 406 mobs, 1308 items,
  318 skills, Japanese names), `athena0659TestEngmod.rar` (2004-01-01: English, 432 mobs, Turtle/Yuno maps in `map_athena.conf`,
  `conf/npc/` tree), `athena-817-src.zip` (2004-02-19: 433 mobs, 1408 items, full `npc/` tree including 2-2 jobs). Snapshots
  dated exactly 2003-12-18 are at https://www.eathena.org/jathena/ (`mod0631`, see `emulators.md`).
- **S9 eAthena SVN in rAthena git.** It starts at r2, 2004-11-04,
  https://github.com/rathena/rathena/commit/706e785091b4b16b0626289e69fc5ac4ab3bc068 [V]. Its `Changelog.txt` goes back to 2003-12-22,
  undated years, AppleGirl/RoVeRT. It is already post-era: `job_db1` has 68 job rows against 24 in 2003–04, so high and baby
  classes have been added, and the `mob_db` header reads `AthenaDB 2004/05/14` [V].

**Later reconstructions**
- **S10 rAthena `db/pre-re/`** https://github.com/rathena/rathena/tree/master/db/pre-re (YAML: `mob_db`, `item_db_*`, `skill_db`, `job_stats`,
  `job_basepoints`, `job_aspd`, `job_exp`, `statpoint`, `attr_fix`, `size_fix`, `refine`, `castle_db`). It represents the *last* pre-renewal
  official state (~2008–2012 kRO/iRO, largely Aegis-derived). It includes TK/Ninja/GS/homunculus/mercenaries and the
  2005+ balance passes. `#ifndef RENEWAL` code paths in `src/map/*.cpp` hold the pre-re formulas.
  **Hercules** `db/pre-re/` (libconfig) https://github.com/HerculesWS/Hercules/tree/stable/db/pre-re has the same lineage.
- **S11 iRO Wiki Classic** https://irowiki.org/classic/Main_Page is a frozen archive ("EDITING HAS BEEN DISABLED") of
  iRO pre-renewal as of ~2009–2012 [V]. Example: the `ASPD` page was last edited 2012-09-28 [S]. Use only `/classic/` URLs; `irowiki.org/wiki/` is renewal.
  It has few citations. Its item/monster DB is db.irowiki.org/classic (Aegis-era data).
- **S12 iRO Wiki *Updates Changelog*** https://irowiki.org/wiki/Updates_Changelog [S]: iRO beta history. Comodo and Turtle Island cite
  the iRO notices; Juno and Amatsu are uncited.
- **S13 Hazy Forest** https://hazyforest.com/episodes:list [S]: kRO episode dates compiled from ro.gnjoy.com, namu wiki, Fandom and iRO Wiki.
- **S14 Calculators.** みすとれ巣 (JP, opened 2002-08-20) → ROratorio (JP, 2004-08, http://roratorio.2-d.jp/ro/) → Doddler's
  English *calcx* (`20051013062410/http://ro.doddlercon.com:80/calc/calcx.html`) → rocalc.com and forks. JP formulas from mid-2004 on;
  later versions track Aegis [S: https://websitemap.sakura.ne.jp/rohistory.html, https://rocalc.com/].
- **S15 Modern databases.** ratemyserver.net (first capture 2006-01, `20060101032200/http://ratemyserver.net:80/`) and RagnaInfo
  (2005-03, `20050320123318/http://www.ragnainfo.net:80/`). divine-pride.net is modern kRO client data. All three are post-era.
- **S16 RO Empire DB** (`www.roempire.com/database/`; site from 2002-12; about 2,600 DB captures 2004-06…12) [V]. For example, Baphomet
  `20040822202445/http://www.roempire.com:80/database/?page=mo&act=v&mid=1039` gives HP 668000 and drops that are **identical to eA 817**.
  It is period-community packaging of emulator data, not an independent capture. `exptables.php` (`20030811073712`) and `forging.php` (2003-02) are older.
- **S17 OpenKore** https://github.com/OpenKore/openkore. Git history starts in 2004. `tables/` (portals, npcs, maps) were collected from
  live servers, but their path history was broken by a 2016 move [V]. Treat them as period-community observation of iRO with uncertain dates.
- **S18 Our client** (`~/data/ragnarok-cd`, GRF listing [V]): `idnum2itemdisplaynametable`, `idnum2itemdesctable` (descriptions quote
  ATK/DEF/weight/levels), `itemslotcounttable` (`1101#3#`: card slots per item), `itemparamtable` (unidentified `id# a#b#` pairs,
  e.g. `2101# 0#2#`), `leveluseskillspamount` (SP per level, e.g. Bash 8×5, 15×5), `skillnametable`, `skilldesctable`,
  `mapnametable`, 208 `.gat` maps (cells and walkability).

## 1. Content timeline (kRO / jRO / iRO)

| Content | kRO | jRO | iRO | Source |
|---|---|---|---|---|
| Lutie (Xmas) | Ep 2, 2002-12-17 | — | existed by 2003-03 (Lutie PvP event, 03-13…17) | S13 [S]; S12 [S] |
| Comodo | Ep 3, 2003-02-04 | 2003-05-29 [H] | 2003, before Turtle Island | S13; S12 |
| WoE + Turtle Island | Ep 4, 2003-05-02 | — | Turtle Island 2003-12-16 (with the Knight/Assassin/Hunter job quests) | S13; S12 [S] |
| 2-2 jobs | phased: Crusader fix 2003-06-10, "2-2 Monk and Rogue job change possible" 2003-06-10, 2-2 sprite and skill patches Jul–Sep 2003 | 新2次職 with Ep 3.0, 2003-10-07 | with Juno 2004-04-07 | S2 [V]; sakura history [S]; playragnarok notice id=146 [S] |
| Juno (Yuno) | Ep 5, 2003-07-15 (S13); "Sage city" light effects 2003-06-10 (S2) | Ep 3.0, 2003-10-07 | 2004-04-07 | S13, S2, https://renewal.playragnarok.com/news/updatedetail.aspx?id=146 [S] |
| Amatsu / Gonryun / Louyang | Ep 6, 2003-10-07 | — | after Juno [S12] | S13 |
| Super Novice | main 2003-12-30 ("Super Novice has been updated") | — | — | S2 `RO_Patch_(2003_Dec._30)` [V]; another timeline says 2004-01-13 [H] |
| Marriage | 2003-12-30 | — | — | S2 [V] |
| Umbala | Sakray Dec 2003 (Kafra without teleport, 2003-12-09); Ep 7, 2004-01-27 | — | later | S1 [V], S13 |
| Niflheim + Yggdrasil roots | 2004-02-24 | — | later | S2 `RO_Patch_(2004_Feb._24)` [V] |
| Transcendence (rebirth) | Sakray 2004-02-03 (level cap 95 for testing); Ep 9, 2004-04-20 (S13); Fandom "Quest:Rebirth" lists 2004-02-02 | Ep 4.0 (from 2004-06-08) | ~2005 [unverified] | S2 `RO_Patch_(2004_Feb._03)` [V] |
| Pets | 2002-11-19 ("Cute Pet system") | — | had pets by 2004-03 (24 tameable, S3 `qpetsystem.asp` [V]) | S2 [V] |
| Homunculus / Taekwon | 2005+ (not in era) | — | — | not in any 2003–04 snapshot [V: no such jobs in 641/817 `job_db1`] |

Disagreements: Juno dates (kRO 2003-07-15 per S13, though Sage-city work appears in 2003-06 patches); Super Novice (2003-12-30
vs 2004-01-13); rebirth (2004-02-02/03 Sakray vs 2004-04-20 Episode 9). For kRO, prefer S1 dates; S13 dates are episode labels.

## 2. EXP, stat points, HP/SP, ASPD, weight
- **Base/job EXP**: `exp.txt` is the same in 641mod (Dec 2003) and 817 (Feb 2004); only line endings differ [V]. rAthena pre-re
  `job_exp.yml` still has the same values (it adds trans and expanded rows) [V spot check]. Client-confirmable via EXP %. **Era fit: high.**
- **Stat points**: `statpoint.txt` in eA r2 and rAthena `statpoint.yml`; the client's stat-point costs can confirm them.
- **HP/SP**: 641mod `job_db1.txt` uses curve-fitted float factors (Swordman HP 0.700808081, Knight 1.504867943) [V]. These were
  reverse-engineered from observed values. 817 uses integers (70, 150) [V]. rAthena now has official per-level tables
  (`job_basepoints.yml`, `HP_SP_TABLES`). Use the tables and check them against the Dec 2003 floats. Differences mean
  rounding, not era. **Med.**
- **ASPD**: `job_db1` per-weapon base delays (641/817) and rAthena `job_aspd.yml`. The formula is on S11 `ASPD` (`200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)` [S]).
  **Flag:** iRO's 2004-04-07 Juno patch has "attack speed adjustments for several classes" [S]. kRO 2003-01-21 halved
  Two-Hand Quicken's bonus [V S2]. Prefer the 2003 snapshots.
- **Weight**: `job_db1` column 1 (Novice 20000, Swordman 28000 …) [V]; STR scaling is in `pc.c`. Stable.

## 3. Damage, hit/flee, crit, DEF/MDEF, elements, size, cards
- **Formulas**: Athena `map/battle.c` in the jA git between //0587 (2003-12-01, `78faa427…`) and //0761 (2004-01-31, `6cb77a2e…`), and
  817's `battle.c`. Cross-check with S11 (Damage/ASPD/Hit/Flee pages; 2010-era, pre-renewal) and the `#ifndef RENEWAL` code in rAthena. **Med.**
- **kRO rule changes inside the era** (S2 2003-02-04 [V]): target LUK reduces crit chance; sleeping targets take ×2 crits;
  no dodging while stunned, frozen, sleeping or stoned; poison and stone cannot take HP below 25%; frozen/stone: MDEF +25%, DEF −50%;
  blind: −25% HIT/FLEE; Sky Petite card 50→30%. Abysmal Knight card +25% vs bosses (Sakray 2003-12-02). Muramasa curse reworked 2003-12-26.
- **Element table (attr_fix)**: S3 *Property System* gives four 10×10 tables [V]. They match 641/817 `attr_fix.txt` and rAthena
  `attr_fix.yml`, **except Lv1 Poison attack** (§9).
- **Size**: `size_fix.txt` (641/817), rAthena `size_fix.yml`. The S3 knight page confirms spear vs Medium 75% → 100% while mounted [V].
- **Cards**: item scripts in 641/817 `item_db.txt`; client descriptions (S18). Many cards were rebalanced on iRO with Comodo and Juno
  [S12, S]. Prefer the client description text when it differs from emulator scripts.

## 4. Monsters
- **Stats and drops**: 641mod (2003-12-25) and 817 (2004-02-19) `mob_db.txt` [V]. Baphomet HP was 568,000 in 641 and 668,000 in 817.
  Its whole drop list changed between the two [V]. Dec 2003: Evil Horn 55%, Yggdrasil Berry 2%, Crescent Scythe 0.5%, Baphomet Doll 0.5%,
  Animal Skin 100%, Oridecon 10%. Feb 2004: Scythe 2%, Majestic Goat 2%, Clip 8%, Emperium 5%, OVB 30%, Oridecon 43%, Elunium 56%,
  which is the set RO Empire shows in Aug 2004. Moonlight Flower's drops also changed. Poring's did not. The card stays at 0.01%.
  **Snapshots this close in date still disagree. Pick a base and record diffs.**
- **The "independent" period DBs are one lineage.** eA r2 `mob_db.txt` is headed `AthenaDB 2004/05/14` [V], and RO Empire 2004 matches eA [V].
  The JP fan history says the AEGIS server leaked in January 2003 [S]. Early Athena data are therefore a mix of observation and leaked
  official tables, with unknown proportions. **Do not** count agreement between them as confirmation.
- **Official period text**: the S3 Library (race, element, level, description) and the iRO Juno notice (2004-04-07: Elder Willow and
  Scorpion made aggressive, Hode passive, Thief Bug race Neutral→Insect, bosses strengthened, Sandman/Mummy EXP) [S]. kRO S2:
  2002-11-19 drop and card rates, 2003-03-25 stats, EXP and monster-skill adjustments, 2003-04-08 spawns, 2003-08-05 Turtle Island EXP [V].
  iRO Comodo raised the Oridecon/Elunium drop rates [S12].
- **AI modes**: the Athena `Mode` bitmask. Poring is 3 in 641/817 and 131 in r2; Baphomet is 53 → 181 [V]. The added bits
  are emulator flags, not era data.
  For semantics see rAthena https://github.com/rathena/rathena/blob/master/doc/mob_db_mode_list.txt (Aegis AI types).
  Monster skills: `mob_skill_db.txt` (817; jA //0743 fixed summon behaviour to match the official server [V]).
- **Spawns/respawn**: 659 `conf/npc/spawns/*`, 817 `npc/mobs/*` (amounts and delays) [V]. rAthena `npc/pre-re/mobs` (later "official"
  spawns). kRO 2003-04-08 rebalanced the spawns in the Lutie toy-factory warehouse: more Marines, fewer Cookies, Angeling and Mastering added,
  plus a longer Petite respawn [V S2]. iRO Juno changed spawn counts [S].
- **Later contamination**: rAthena pre-re `mob_db.yml` is ~2010 kRO/iRO. It has monsters from Ep 7–11, some re-statted, and drop
  tables that changed in 2005–2008. Use it only to fill fields missing from 641/659/817.

## 5. Skills
- **Per-level data**: S3 job-skill pages (2004-03/04) give per-level %, SP and durations [V; e.g. Pierce 110–200%, SP 7, +5 HIT/level].
  Watch internal contradictions: the Pierce prose says "200% a normal hit". S18 `leveluseskillspamount.txt` is authoritative for SP.
- **DB files**: 641/817 `skill_db.txt` (318 rows), `cast_db.txt`, `skill_tree.txt`, `skill_require_db` (817). jA //0759 (2004-01-30) fixed
  Spear Quicken and the Dancer tree [V]. 817 `npc/skillquests/` covers the 1st-class quest ("platinum") skills [V].
- **kRO skill changes in the era** [V S2]: 2003-01-21 Frost Diver cast time, Holy Light 1.5 s, Two-Hand Quicken halved; 2003-02-25 Bowling Bash cast
  0.7 s with no after-delay; 2003-08-26 Land Protector rules; 2003-10-07 Dispell targets; 2003-12-02 Blade Stop (진검백파도) no longer works on bosses.
- **Later contamination**: 2005+ kRO balance passes, the transcendent skills (2004), and the 2018 kRO mass rebalance (rAthena issue #3715).
  Check per skill whether rAthena applies a change outside `RENEWAL`; I did not verify this. S11 describes iRO ~2010.

## 6. Items, refine, shops
- **Refine**: `refine_db` success rates are identical in 641, 817, r2 and rAthena today (weapon Lv1 safe +7 … Lv4 safe +4; armor safe +4,
  60/40/40/20/20/10) [V]. The S3 *Refine System* page (2004-06) [V] gives materials, fees and per-refine ATK (2/3/5/5): Phracon fee 50z,
  Emveretarcon fee 200z, Oridecon/Elunium fees 5,000z/2,000z, armor +1 DEF per refine. **Armor DEF per refine** is
  encoded as 1 (641), 5 (817) and 70 = 0.7 (r2) [V]. Decide by testing against client display.
- **Item stats and prices**: 641/817 `item_db.txt`, client names, descriptions and slot counts (S18), S3 Library item pages (2004-03…07).
  iRO 2004-04-07 lowered healing-item weights and changed prices and book weapons [S]. kRO 2004-02-03 fixed Lv4 weapon race-crit options [V S2].
- **NPC shops**: 659/817 `npc/shops/shop.txt`, plus `2-2shop.txt` in 817. Prices come from `item_db` Price.

## 7. NPCs, quests, Kafra, warps
- **Best era scripts**: 659 `conf/npc/` (2004-01-01; cities, jobs: 1-1/2-1 only, warps, spawns, kafra, refine, dye, quests) and 817
  `npc/` (2004-02-19; adds the 2-2 job quests, guild castles and skill quests) [V]. `eathena-npc.rar` (2004-05-07) is the next step.
  rAthena `npc/pre-re/` holds later "official" conversions (2006+ Aegis dialogue). They are better worded but post-era.
- **Kafra** [V]: in Feb 2004 scripts, storage is 30z and the cart is 800z. Prontera teleports: Izlude 900, Geffen 1400, Payon 1700, Morroc 1800.
  kRO replaced the fare table on **2004-02-24** (Prontera→Izlude 600, Geffen/Morroc/Payon 1200, Alberta 1800 …) and removed
  storage at the dungeon Kafras (S2 `RO_Patch_(2004_Feb._24)`). kRO removed the Kafra voucher menu 2003-05-20. Dec 2003 = the old fares.
- **Job quests**: iRO got the Knight/Assassin/Hunter quests with Turtle Island (2003-12-16) [S12]. iRO Sakray's 2-1 job-quest
  update slipped (S4 seq 213, 2003-10-07 [V]).
- **Not era content**: emulator conveniences such as `npc_warpra.txt` (warper), `heal.txt`, `card_remover`, `MrSmile`, `IceCream` and
  arenas [V file names]. Tag these as custom. Stylists (hair NPCs) do not appear in the 2003–04 scripts; only clothes dye (`dye.txt`) does [V].
- **Warps**: 659/817 `npc/warps/*` [V]; OpenKore `tables/portals.txt` (live-observed, dates uncertain). The client `.rsw` files carry no warps.

## 8. Movement, timing, regen, status durations
- **817 defaults** [V `map/battle.c` L3612-3690, `pc.c`]: natural HP tick 6000 ms, SP tick 8000 ms, skill regen 10000 ms, no regen at
  ≥50% weight. HP regen amount `1+VIT/5+MaxHP/200`, SP `1+INT/6+MaxSP/100` (+bonus at INT ≥120). Sitting and Magnificat
  each double the tick accrual (`pc_hpheal`).
  Loot protection 10 s first, 7 s second (MVP 10/10). Death penalty 1%/1% (`death_penalty_base/job: 100`, in 0.01% units).
  `monster_max_aspd 199`.
- **Walk speed, amotion/dmotion**: `mob_db` Speed/aDelay/aMotion/dMotion columns (641/817). Player walk speed is
  `DEFAULT_WALK_SPEED 150` ms/cell (817 `common/mmo.h`). Peco and Increase AGI modifiers are in `pc.c` L1229-1233; the S3 knight page also says Peco is faster.
- **Status durations**: jA //0736 (2004-01-23) made resistance shorten durations as well as chance, and made Stone Curse duration
  follow the 2ch "マジスレ" (mage thread) template [V]. So Dec 2003 emulators differ from what jRO players observed.
- **EXP sharing**: the iRO guide (S3 `basicexp.asp`, 2004-03) says +10% total EXP per extra attacker, split by damage. kRO changed party
  even-share to **+25% per member on 2003-02-04** [V S2, Korean text checked]. A known disagreement (§9).

## 9. Data collected back in the day, and known disagreements
- **Period raw data that exists**: S1, S2, S3/S4/S5 (Wayback), S7 commit logs (2ch-sourced observations), S16, and the JP
  fan-site list with dates in https://websitemap.sakura.ne.jp/rohistory.html [S]. That list includes the period calculator site
  みすとれ巣 (2002), 狩場情報 (hunting-ground monster lists, 2002-08), and "OptimizeProject" stat analysis (2002).
  No 2003 English packet logs with combat values were found. Japanese 2ch "本鯖相違スレ" threads are cited by jA but not archived here.
- **Disagreements found while surveying** [V unless marked]:
  1. Element Lv1 Poison attack: iRO official page `100,100,100,100,100,0,75,50,25,-25` vs 641/817/rAthena `100,100,125,125,125,0,75,50,100,-25`.
     The official Lv2 row (125 vs Earth/Fire/Wind) suggests the iRO page has a typo, but this is unproven.
  2. EXP bonus per extra attacker: iRO guide 10% vs kRO 25% (2003-02-04). Possibly regional or stale text.
  3. Baphomet: HP 568k (Dec 2003 data) vs 668k (Feb 2004 onward); drop lists differ.
  4. Phracon price: the iRO refine page says 2000z; `item_db` says 200z (Emveretarcon 1000z in both).
  5. Armor refine DEF: 1 vs 5 vs 0.7 per level across emulator generations; the iRO page says "+1 defense".
  6. Super Novice kRO date (2003-12-30 vs 2004-01-13 [H]); Juno kRO date (2003-07-15 label vs earlier Sage-city patches).
  7. HP/SP: 2003 float fits vs integer factors vs official tables.

## 10. Recommended provenance tiers for emulator data

Tag every table row or constant with `tier`, `src` (a source id from this note plus a path or URL), `asof` (the date the value was true or
captured), `region` (kRO/iRO/jRO/euRO/unknown) and, optionally, `conflicts` (list of source ids that disagree).

| Tier | Name | What qualifies | Examples |
|---|---|---|---|
| T0 | client-confirmed | read from our 2003-12-18 client (GRF tables, `.gat`, exe) | item names, descriptions, slot counts, skill SP, map cells |
| T1 | period-official | Gravity/GungHo text dated ≤2004-06: S1, S2 Korean text, S3/S4/S5 | Kafra fares, refine fees, element tables, patch rule changes |
| T2 | period-emulator | jAthena/AppleMod/eAthena snapshots ≤2004-02 (S7, S8) | EXP tables, `mob_db` stats and drops, spawns, regen ticks, NPC scripts |
| T3 | period-community | player-made 2002–04 material (S16, JP sites, OpenKore tables, forum tables) | RO Empire pages, portal lists |
| T4 | later-official-derived | pre-renewal data from Aegis-era dumps, post-2004 (S10, db.irowiki classic) | rAthena pre-re `mob_db.yml`, `job_basepoints.yml` |
| T5 | later-community | wikis and calculators written after 2004 (S11, S14, S15) | iRO Wiki Classic formulas, rocalc |
| T6 | guess | interpolation or designer choice, with a note on why | missing drop rates, unknown delays |

Rules: prefer the lowest tier number for **values**. Prefer T2 over T4 even when T4 is "official", because T4 is the wrong year.
Override T2 with T1 only when the T1 text is dated and region-matched. Keep `region` explicit: a kRO T1 value does not override
an iRO value for an iRO-flavoured build. Record T1 vs T2 disagreements (§9) in `conflicts` rather than silently picking one.
Baseline: **659 (2004-01-01, English) + 641mod (2003-12-25) data, jAthena logic through //0761.** Then apply kRO S1 changes up to 2003-12-30.
Use rAthena pre-re only to fill gaps, as T4.

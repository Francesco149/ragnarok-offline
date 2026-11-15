# Round 21 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 26, 27, R6) | partly (guild castle regen date confirmed 2004-06-05 and 2x natural regen formula for castle owners in rathena Changelog.txt and pc.c; Prontera castle flags confirmed in eAthena 0.5.2 warping owning guild members to prtg_cas01 97,174; jAthena commit 1fcb977 castle grounds flags confirmed showing kingdom edict; castle steward Abrai confirmed at prtg_cas01 112,181 with 10k flat investment in eAthena 0.5.2 vs 10k-5.12M scaled investment doubling/quadrupling on same day in eAthena 1.0; treasure box count confirmed 2 + Economy/100000 in eAthena 0.5.2 vs Economy/5 + 4 in eAthena 1.0; Acolyte quest ascetic priest locations confirmed in prontera ruins/yoyoland, North Morroc NW, and East Geffen island in H0BGOBLIN FAQ 18212) | T9-249 – T9-258 |
| T8 | Things only drafted from client's text (thread 22) | partly (bow minimum attack formula confirmed watk * (dex < watk ? dex : watk) with random 0 to arrow_atk bonus in jAthena commit 5415aa0; refined bow minimum attack confirmed watk * ((atkmin < watk) ? atkmin : watk) / 100 in jAthena commit d6aa96f; Sightrasher 8-directional ray pattern and +1 to +5 cell range confirmed in iRO Guide 2004 wizard skill page; eAthena 817 confirmed 24 cells over 8 directions reaching distance 3 vs eAthena 0.5.2 41 cells reaching distance 5) | T8-039 – T8-044 |
| T1 | 2-1 job tests: dating and sharpening (threads 5, 8) | partly (Knight job change confirmed requiring no test quest, only 2,500 Zeny paid in NW Prontera culvert HQ in Psycho Battousai FAQ 21546 v1.3 Feb 2003 and Edge FAQ 29457 Feb 2004; Blacksmith job change confirmed requiring no test quest or fee, changing instantly upon entering SW Geffen guild in the only KoW FAQ 24428 Jul 2003) | T1-056 – T1-058 |

Total: 19 rows across 3 tasks.
Checker result (`check.txt`): 19 PASS, 0 WARN, 0 FAIL, 0 NONE of 19 rows verified with `python3 ../check_rows.py --pages r21/pages r21/rows/*.jsonl`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 26, 27, R6)
- **Status:** partly
- **Rows:** `T9-249` through `T9-258`
  - **Castle HP/SP Regeneration:**
    - `T9-249`: rathena `Changelog.txt:1772` under commit `706e78509` (2004-06-05) records Valaris implemented guild castle regen (stackable 2x natural regen for castle owners).
    - `T9-250`: rathena `src/map/pc.c:6602,6629` under commit `706e78509` confirms castle regen doubles natural regeneration step (`a += a;`) for all players whose `guild_id` matches the castle owner's `guild_id` (`g && g->guild_id == gc->guild_id`).
  - **Castle & Town Flags:**
    - `T9-251`: eAthena 0.5.2 `npc/guild/prtg_flag.txt:1` places town flags (Kriemhild, Swanhild, Fadhgrind, Skoegul, Gondul) in Prontera at `146,194`, displaying the kingdom proclamation to non-members and offering direct warp to `prtg_cas01.gat,97,174` for owning guild members.
    - `T9-252`: jAthena commit `1fcb977` (2004-02-12) `conf/gvg/guild_flag.txt:5` places castle flags on castle grounds (`prt_gld.gat,160,210`), displaying the Rune-Midgarts proclamation without a warp menu.
  - **Castle Steward (Abrai) & Investment:**
    - `T9-253`: eAthena 0.5.2 `npc/guild/prtg_cas01.txt:1` places steward Abrai at `prtg_cas01.gat,112,181` checking castle ownership (`GetCastleData("prtg_cas01.gat",1)`).
    - `T9-254`: eAthena 0.5.2 `npc/guild/prtg_cas01.txt:11` sets investment cost at a flat 10,000 Zeny per point.
    - `T9-256`: eAthena 1.0 (commit `706e78509`) `npc/guild/old/prtg_cas01.txt:242` scales commerce investment from 10,000 Zeny (<8 Economy) up to 5,120,000 Zeny (>=88 Economy).
    - `T9-257`: eAthena 1.0 (commit `706e78509`) `npc/guild/old/prtg_cas01.txt:271` quadruples the investment cost (`set @eco_invest,@eco_invest*4;`) if the castle was already invested in during the current day.
  - **Castle Treasure Boxes:**
    - `T9-255`: eAthena 0.5.2 `npc/guild/prtg_cas01_treasure.txt:21` spawns treasure boxes 1324 and 1325 with count calculated as `2 + Economy/100000`.
    - `T9-258`: eAthena 1.0 (commit `706e78509`) `npc/guild/old/treasure/prtg_cas01_treasure.txt:41` spawns treasure boxes with count calculated as `Economy/5 + 4`.
  - **Acolyte Quest Ascetic Priests:**
    - `T9-257` (in rows list): H0BGOBLIN Acolyte/Priest FAQ 18212 v1.7 (2003-07-25) records ascetic Priest #1 in Prontera ruins/yoyoland guarded by mandragoras, Priest #2 in North Morroc NW corner, and Priest #3 in East Geffen center island.

### T8: Things Only Drafted from Client's Text (Thread 22)
- **Status:** partly
- **Rows:** `T8-039` through `T8-044`
  - **Bow Damage Formula Evolution:**
    - `T8-039`: jAthena commit `5415aa0` (2004-01-04) `map/battle.c:1667` calculates bow minimum attack using `watk * (dex<watk)?dex:watk`.
    - `T8-040`: jAthena commit `5415aa0` `map/battle.c:1780` adds random damage from 0 to `arrow_atk` on arrow attacks (`damage += rand()%(sd->arrow_atk+1)`).
    - `T8-041`: jAthena commit `d6aa96f` (2004-05-16) `src/map/battle.c:2496` updates bow minimum attack to `watk * ((atkmin<watk)? atkmin:watk)/100` where `atkmin = atkmin * (80 + wlv*20)/100`.
  - **Sightrasher Pattern and Range:**
    - `T8-042`: iRO Guide Wizard skill page (`20040327034745_jobwizskill.html`) records Sightrasher shoots 8 fire balls in 8 directions with pushback, scaling range from +1 to +5 cells.
    - `T8-043`: eAthena 817 `map/skill.c:3169` hardcodes 24 cells across 8 direction vectors reaching distance 3 for Sightrasher.
    - `T8-044`: eAthena 0.5.2 `src/map/skill.c:4012` hardcodes 41 cells across 8 direction vectors reaching distance 5 for Sightrasher.

### T1: The 2-1 Job Tests: Dating and Sharpening (Threads 5, 8)
- **Status:** partly
- **Rows:** `T1-056` through `T1-058`
  - `T1-056`: Psycho Battousai Swordsman/Knight FAQ 21546 v1.3 (2003-02-25) records changing to Knight on early official servers required no quest or combat test, only paying 2,500 Zeny to the Knight NPC in NW Prontera culvert HQ.
  - `T1-057`: Edge Knight/Crusader FAQ 29457 v1.0 (2004-02-08) confirms Knight job change on pRO required no test, only paying a small fee in NW Prontera culvert HQ.
  - `T1-058`: the only KoW Battlesmith FAQ 24428 (2003-07-13) records changing to Blacksmith required no quest test or fee, instantly changing upon speaking to the guild leader in SW Geffen.

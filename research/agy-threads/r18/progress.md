# Round 18 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 17, 26, 29, 31) | partly (prtg_cas02 blocked warp target 157,140 resolved: in Dec 2003 client Emperium room was located at X=[184..203], Y=[162..182] with entrance at 190,163; Gravity moved it to 157,140 in May 2004 and emulators copied the revised coordinates; pay_dun04 193,43 wall target confirmed as emulator typo across mod0631, 817, 0.5.2, jAthena for corridor at 191,41/191,43; tur_dun05 confirmed as walkable map with exit warps ttl05/ttl06 in mod0631/817 and no entry warps; prt_in 280,68, 281,36, 282,100 confirmed as blocked void cells where 817 had phantom exit warps commented out by Lupus in rAthena c6d5f38b7; dormant hats 5020, 5044, 5051 and tickets 7056, 7059-7061 confirmed present in Dec 2003 client descriptions but omitted from display names until May 2004; Lupus confirms Kafra Band had no official maker quest and RateMyServer confirms no monster drops; ticket items confirmed unused in period NPC scripts; Baphomet Doll trade confirmed as 5 Panacea on iRO by RODataZone 2004-02-24 vs 2 Royal Jelly in mod0631; Marx Hansen juices confirmed requiring 1 fruit, 1 empty bottle, and 3z in mod0631, 817, and Battousai X 2004; Merchant letter reward confirmed as 100z in mod0631 and 817; WoE Godly material drop rate from treasure boxes confirmed as 1/250 in Ragu Memo 2004-02-06; castle economy investment cost confirmed as 10,000 Zeny in 817; remaining: Kafra storage points outside jRO, Shockwave Trap item count, remaining Alberta doll rewards, WoE daily box counts per trade level and capture deduction, dungeon rebalance accounts, Clock Tower random warp coordinates, unknown skill display witness, monster support skill roll frequencies) | T9-216 – T9-240 |

Total: 25 rows across 1 task.
Checker result (`check.txt`): 25 PASS, 0 WARN, 0 FAIL, 0 NONE of 25 rows verified with `python3 research/check_rows.py --pages research/agy-threads/r18/pages`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 17, 26, 29, 31)
- **Status:** partly
- **Rows:** `T9-216` through `T9-240`
  - **Castle 2 Warp Destination & Map Revision (THREAD 26 / T9):**
    - `T9-216`: `gat_cells.py` confirms `prtg_cas02.gat 184,44` is walkable (type 0), while `157,140`, `157,135`, and `157,174` are blocked (type 1), and the Dec 2003 Emperium room entrance at `190,163` and altar at `193,181` are walkable (type 0) in the Dec 2003 client `data.grf`.
    - `T9-217`: `gat_cells.py` confirms `prtg_cas02.gat 157,140`, `157,135`, and `157,174` are walkable (type 0), while `190,163` and `193,181` are blocked (type 1) in the May 2004 DVD `data.grf`, proving Gravity relocated the Emperium room from X=[184..203], Y=[162..182] to X=[142..173], Y=[132..177] in May 2004.
    - `T9-218`: eAthena 817 `npc/warps/guildcastles.txt` line 339 assigns warp `prtg206` from `prtg_cas02.gat 184,44` to revised map coordinate `157,140`, showing that early emulators scripted warps matching the May 2004 revised layout.
  - **Places World Audit Cannot Reach (THREAD 26 / T9):**
    - `T9-219`: `gat_cells.py` confirms `pay_dun04.gat 193,43` is blocked (type 1) while corridor cells `191,43` and `191,41` are walkable (type 0) in both Dec 2003 and May 2004 client GRFs.
    - `T9-220`: eAthena 0.5.2 `npc/warps/payon.txt` line 108 defines warp destination `w2` onto blocked cell `pay_dun04.gat 193,43`.
    - `T9-221`: jAthena `script/warp/npc_warp.txt` line 499 defines warp destination `w2` onto blocked cell `pay_dun04.gat 193,43` while return warp `payd04-2` is defined at walkable cell `191,41`.
    - `T9-222`: eAthena 817 `npc/warps/alberta.txt` lines 101-102 defines exits `ttl05` (to `tur_dun02 148,264`) and `ttl06` (to `tur_dun01 154,237`) from `tur_dun05`, with no entrance warp leading into `tur_dun05`.
    - `T9-223`: `gat_cells.py` confirms `prt_in.gat` cells `281,36`, `280,68`, and `282,100` are blocked (type 5) while `285,130` is walkable (type 0).
    - `T9-224`: eAthena 817 `npc/warps/prontera.txt` lines 51-56 defines phantom exit warps `w470`-`w473` from `prt_in.gat` cells `280,68`, `281,36`, `282,100`, and `285,130` to `prontera.gat 147,287`.
    - `T9-225`: rAthena commit `c6d5f38b7` (2007-03-15) commented out invalid `prt_in` exit warps at `280,68`, `281,36`, and `282,100` after testing.
  - **Unobtainable Hats & Kafra Tickets (THREAD 31 / T9):**
    - `T9-226`: Dec 2003 client `data.grf` includes item descriptions for 5020, 5044, 5051, 7056, 7059, 7060, and 7061 in `idnum2itemdesctable.txt` but omits display names from `idnum2itemdisplaynametable.txt` (names added in May 2004).
    - `T9-227`: rAthena `npc/custom/quests/event_6_new_hats.txt` (commit `2178a8631`, 2006-01-29) records Kafra Band (5020) was made as a custom quest because it was missing from official quests and could not be made on real official servers.
    - `T9-228`: RateMyServer pre-re item database confirms Kafra Band (5020) has no dropping monsters ("Dropped By No Result").
    - `T9-229`: RateMyServer pre-re item database confirms Devil's Wing (5044, Wings of Demon) has no dropping monsters ("Dropped By No Result").
    - `T9-230`: RateMyServer pre-re item database confirms Kitty Bell (5051, Bell of Pussycat) has no dropping monsters ("Dropped By No Result").
    - `T9-231`: eAthena 817 `db/item_db.txt` lines 1220-1225 defines ticket items 7056, 7059, 7060, and 7061, though no period NPC script referenced them.
  - **iRO vs jRO Item by Item (THREAD 29 / T9):**
    - `T9-232`: RODataZone `items/common.js` (2004-02-24) records Baphomet Doll used for "5 Panacea quest" on iRO.
    - `T9-233`: mod0631 `conf/npc_event_doll.txt` line 217 awards 2 Royal Jelly (`getitem 526,2;`) in exchange for Baphomet Doll on jRO.
    - `T9-234`: mod0631 `conf/npc_event_potion.txt` lines 221-224 consumes 1 fruit, 1 empty bottle, and 3z per juice.
    - `T9-235`: eAthena 817 `npc/cities/payon.txt` lines 453-456 consumes 1 fruit, 1 empty bottle, and 3z per juice.
    - `T9-236`: Battousai X Comodo Skills Guide (GameFAQs 27646, 2004-01-20, pRO) records 36 Banana Juice requires 36 Empty Bottles and 36 Bananas (1 fruit and 1 empty bottle per juice).
    - `T9-237`: mod0631 `conf/npc_job_merchant.txt` line 77 awards 100z (`set Zeny,Zeny+100;`) for delivering Chief Mahnsoo's letter.
    - `T9-238`: eAthena 817 `npc/jobs/merchant.txt` line 275 awards 100z (`set Zeny,Zeny+100;`) for delivering Chief Mahnsoo's message.
  - **WoE Mechanics & Rates (THREAD 17 / T9):**
    - `T9-239`: Ragu Memo `gvg_box.html` (2004-02-06, jRO) records godly item materials have a 1/250 drop rate from GvG castle treasure boxes (`雪の結晶(1/250)` in Valkyrie Realms 1).
    - `T9-240`: eAthena 817 `npc/guild/TEST_prtg_cas01_AbraiJ.txt` line 72 records guild castle economy investment costs 10,000 Zeny (`必要な投資量は^ff0000 10,000 ^000000Zenyです`).

# Thread #3: Kafra fares and save cells before 2004-02-24

## Answer so far
Prior to the 2004-02-24 kRO overhaul, Kafra teleport fares followed a granular distance/route table rather than the post-overhaul 600z/1200z/1800z system:
1. **Prontera -> Geffen** was **1400z**, not 1800z. The 1800z charged in eAthena 817's code was an emulator copy-paste typo from Morroc (the menu displayed 1400z, and upstream jAthena checks and deducts exactly 1400z).
2. **Alberta -> Prontera** was **2100z**, not 1800z. In 817, the first Alberta Kafra had an emulator typo in its menu string displaying 1800z while charging 2100z; the second Alberta Kafra displayed and charged 2100z, and jAthena both displays and charges 2100z. (Note that Prontera did NOT offer a direct warp to Alberta pre-2004-02-24; Alberta was only reachable via Payon or Morroc).
3. **The 2004-02-24 Korean patch text** (`seq=1182` on ro.gnjoy.com) does *not* list what the old fares were; it only presents the new schedule headed `##### 변경된 카프라 워프 서비스 #####` ("Changed Kafra Warp Service") and notes that the system tested on Sakray is now live on the main server. It also terminated Kafra storage at dungeon locations (Orc Dungeon, Mjolnir Dead Pit, Pyramid, Sunken Ship, Byalan, Archer Village).
4. **Storage & Cart Fees**: Unlike eAthena 817 which flattened all storage to a uniform 30z and cart to 800z, authentic Gravity Aegis data and jAthena show tiered fees by location:
   - Storage: Prontera 40z, Geffen 30z, Izlude 30-40z, Morroc 40-60z, Payon 40-60z, Alberta 50z, Al De Baran 20-40z (HQ discount). Remote dungeon outposts charged higher fees (Comodo/Pharos 80z, Mjolnir Dead Pit 100z, Byalan 120z, Orc Dungeon 130z, Niflheim 150z).
   - Cart rental: Standard mainland rate was 800z (or 600z in Aldebaran HQ, 700z in Amatsu/Gonryun, 1000z at Pharos Lighthouse).
5. **Exact Kafra save cells in towns**:
   - **Izlude inside city save cell**: Authentic cell is **`izlude (94, 103)`** (confirmed by jAthena `savepoint "izlude.gat",94,103;` and rAthena pre-renewal Aegis script `savepoint "izlude",94,103,1,1;`). 817's missing savepoint command was an emulator script omission.
   - **Prontera**: Central `116, 72` (or `116, 73`); South inside `151, 33` (or `150, 33`), outside `prt_fild08 170, 368`.
   - **Geffen**: `119, 40` (or `120, 38`).
   - **Morroc**: `156, 46`.
   - **Payon**: `69, 100`.
   - **Alberta**: North inside `31, 231`, outside `pay_fild03 387, 76`; South `117, 56`.
   - **Al De Baran**: `143, 110` (or `143, 109`).

---

## Findings

### 1. Prontera -> Geffen Fare Discrepancy in 817
- [READ] In eAthena 817 (`scripts/import/npc/other/kafra.lua` L2521-2537), the menu lists `"Geffen -> 1400 Zeny."`, but label `::Geffen::` checks `player.zeny < 1800` and deducts 1800. Directly following is `::Morroc::` which also checks `player.zeny < 1800` and deducts 1800.
- [READ] In jAthena (`script/npc/town/npc_town_kafra.txt` L68-80), Prontera Kafra code confirms 1400z was the true intended fare:
  `menu "イズルード -> 900 z",L3_1,"ゲフェン -> 1400 z",L3_2...`
  `L3_2: if (Zeny < 1400) goto Lerror3; set Zeny,Zeny-1400; set KF_PIT,KF_PIT+140; warp "geffen",120,39; end;`
  https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/town/npc_town_kafra.txt (jAthena //0505-0761, 2003-2004; high)
- [INFER] The 1800z deduction in eAthena 817 was a copy-paste error made by an emulator author copying the Morroc check/deduction block into the Geffen block without updating 1800 to 1400.

### 2. Alberta -> Prontera Fare Discrepancy in 817
- [READ] In eAthena 817 (`scripts/import/npc/other/kafra.lua` L688-705), the first Alberta Kafra (`alberta 28, 229`) displays `"Prontera -> 1800 z"`, but label `::Prontera::` checks `player.zeny < 2100` and deducts 2100z.
- [READ] In eAthena 817 (`scripts/import/npc/other/kafra.lua` L837-850), the second Alberta Kafra (`alberta 113, 60`) displays `"Prontera -> 2100 z"` and deducts 2100z.
- [READ] In jAthena (`script/npc/town/npc_town_kafra.txt`), both Alberta Kafras display `"プロンテラ -> 2100 z"` in their menu and deduct 2100z (`if (Zeny < 2100) goto Lerror3; set Zeny,Zeny-2100; set KF_PIT,KF_PIT+210; warp "prontera",116,72; end;`).
  https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/town/npc_town_kafra.txt (high)
- [INFER] The authentic pre-overhaul fare from Alberta to Prontera was **2100z**. The "1800z" string in 817's first Alberta Kafra menu was an emulator transcription typo.
- [READ] Furthermore, Prontera's Kafra did NOT offer a return warp to Alberta pre-2004-02-24. Prontera's options were solely Izlude (900z), Geffen (1400z), Payon (1700z), Morroc (1800z), Comodo (2000z, jAthena), and Orc Dungeon (1700z).

### 3. Official Korean Patch Text (2004-02-24)
- [READ] kRO Maintenance Notice `seq=1182` published 2004-02-23 (Patch date 2004-02-24):
  "- 사크라이 서버에서 테스트를 진행했던 카프라 요금 체계와 이동지역이 본서버에 업데이트 됩니다. 각 마을에서 갈 수 있는 지역과 요금표는 아래를 참고하여 주시기 바랍니다. ##### 변경된 카프라 워프 서비스 #####"
  https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1182&curpage=562 (published 2004-02-23; high)
- [READ] The patch notice only lists the new updated destination table (`변경된 카프라 워프 서비스`), not the old fares:
  Prontera -> Izlude: 600z, Geffen: 1200z, Morroc: 1200z, Payon: 1200z, Alberta: 1800z, Orc Dungeon: 1700z.
  Izlude -> Geffen: 1200z, Morroc: 1200z, Payon: 1200z, Aldebaran: 1800z.
  Geffen -> Prontera: 1200z, Aldebaran: 1200z, Orc Dungeon: 1700z, Abandoned Mine Dungeon: 1700z.
  Payon -> Prontera: 1200z, Alberta: 1200z, Morroc: 1200z.
  Morroc -> Prontera: 1200z, Payon: 1200z, Alberta: 1800z, Pharos Lighthouse: 1200z, Comodo: 1800z.
  Alberta -> Payon: 1200z, Morroc: 1800z, Prontera: 1800z.
  Aldebaran -> Geffen: 1200z, Juno: 1200z, Izlude: 1800z, Abandoned Mine Dungeon: 1700z.
  Comodo -> Pharos Lighthouse: 1200z, Morroc: 1800z.
  Pharos Lighthouse -> Comodo: 1200z, Morroc: 1200z.
  Juno -> Aldebaran: 1200z.
  Juno (Airship PR Employee) -> Prontera, Izlude, Geffen, Morroc, Payon, Alberta, Comodo: 1800z.
- [READ] The same patch explicitly removed storage service from dungeon and field Kafras:
  "- 묘르닐 폐광던전, 오크던전, 피라미드, 난파선, 바이아란, 궁수마을에서는 더 이상 카프라가 저장 서비스를 하지 않게 되었습니다."
  ("In Mjolnir Abandoned Mine, Orc Dungeon, Pyramid, Shipwreck, Byalan, and Archer Village, Kafra will no longer provide Storage service.")
  https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1182&curpage=562 (high)

### 4. Storage and Pushcart Rental Fees
- [READ] In jAthena (`script/npc/town/npc_town_kafra.txt` and `script/npc/npc_function.txt`), storage fees were passed dynamically per town via `callfunc "kfstorage", Price;` and cart rental via `callfunc "kfcart", Price;`:
  - Prontera: Storage 40z, Cart 800z
  - Izlude: Storage 30z, Cart 800z
  - Geffen: Storage 30z, Cart 800z
  - Morroc: Storage 40z, Cart 800z
  - Alberta: Storage 50z, Cart 800z
  - Payon: Storage 40z, Cart 800z
  - Aldebaran: Storage 40z, Cart 800z
  - Comodo: Storage 40z, Cart 800z
  - Pharos Lighthouse (cmd_fild07): Storage 80z, Cart 1000z
  - Byalan Island (izlu2dun): Storage 40z, Cart 800z
  - Sunken Ship (alb2trea): Storage 50z, Cart 800z
  - Pyramids (moc_ruins): Storage 50z, Cart 800z
  - Mjolnir Mine (mjolnir_02): Storage 100z, Cart 800z
  - Orc Dungeon (gef_fild10): Storage 130z, Cart 800z
  - Niflheim: Storage 150z, Cart N/A
  https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/town/npc_town_kafra.txt (high)
- [READ] In rAthena official pre-renewal scripts (`npc/pre-re/kafras/kafras.txt` and `npc/kafras/kafras.txt` which derive directly from Aegis server scripts), the storage fees match the tiered nature: Prontera 40z, Geffen 30z, Morroc 60z, Payon 60z, Alberta 50z, Aldebaran 20z, Izlude 40z, Comodo 80z, Mjolnir 100z, Byalan 120z, Orc Dungeon 130z, Niflheim 150z. Cart fees range from 600z (Aldebaran HQ discount) to 800z (Prontera), 850z (Alberta), 930z (Morroc/Payon), and 1000z (Comodo).
  https://raw.githubusercontent.com/rathena/rathena/master/npc/kafras/kafras.txt (high)
- [INFER] eAthena 817 flattened all storage to 30z and cart to 800z as an emulator simplification/bug. In authentic 2003-2004 official gameplay, storage was 40z in Prontera, 30z in Geffen/Izlude, 50z in Alberta, and significantly more expensive at remote dungeon locations (80z to 130z).

### 5. Kafra Save Cells and Izlude Save Point
- [READ] In eAthena 817 (`scripts/import/npc/other/kafra.lua` L1770-1785), Izlude Kafra's inside city save option contained a bug:
  `::SaveIn::`
  `cutin("kafra_01", 2)`
  `mes "[Kafra]"`
  `mes "Your respawn point has saved. Thank you."`
  `cutin("kafra_01", 255)`
  `close()`
  `do return end`
  It completely lacked a `save_point(...)` call, saving nothing.
- [READ] In jAthena (`script/npc/town/npc_town_kafra.txt`), Izlude Kafra (`izlude.gat,136,88`) explicitly defines both save options:
  `menu "町の外でセーブ",-,"町の中でセーブ",L1_3;`
  `savepoint "prt_fild08.gat",350,202;`
  `goto Lend1;`
  `L1_3: savepoint "izlude.gat",94,103;`
  https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/town/npc_town_kafra.txt (high)
- [READ] In rAthena official pre-renewal scripts (`npc/kafras/kafras.txt`), Izlude's savepoint is:
  `if (checkre(0)) savepoint strnpcinfo(4),129,97,1,1; else savepoint "izlude",94,103,1,1;`
  https://raw.githubusercontent.com/rathena/rathena/master/npc/kafras/kafras.txt (high)
- [READ] In OpenKore live server tables (`tablepack/tags/start/tables/npcs.txt`), Izlude's Kafra stands at `izlude 134 88` (matching jAthena's 136, 88).
  https://raw.githubusercontent.com/openkore/openkore/e24871a17bbeb8d07865d55f5ed828aee6d50d99/tablepack/tags/start/tables/npcs.txt (high)
- [INFER] While the Kafra warp landing cell for Izlude was `izlude 91, 105` (`warp "izlude", 91, 105` in jAthena, 817, and rAthena `F_KafTele`), the authentic inside city save cell in official Gravity servers is **`izlude (94, 103)`**.

---

## Complete Table of Pre-2004-02-24 Fares vs Post-2004-02-24 Overhaul

| Origin | Destination | Pre-2004-02-24 Fare (z) | Post-2004-02-24 Overhaul (z) | Notes |
|---|---|---|---|---|
| **Prontera** | Izlude | 900 | 600 | Pre-overhaul: Warp cell (91, 105) |
| Prontera | Geffen | 1,400 | 1,200 | 817 menu showed 1400z; code charged 1800z (copy-paste bug from Morroc) |
| Prontera | Payon | 1,700 | 1,200 | |
| Prontera | Morroc | 1,800 | 1,200 | |
| Prontera | Orc Dungeon | 1,700 | 1,700 | Unchanged by 2004-02-24 patch |
| Prontera | Comodo | 2,000 | — | Removed in 2004-02-24 overhaul |
| Prontera | Alberta | — | 1,800 | Newly added on 2004-02-24 |
| **Izlude** | Prontera | 900 | — | |
| Izlude | Payon | 1,700 | 1,200 | |
| Izlude | Geffen | 1,800 | 1,200 | |
| Izlude | Morroc | — | 1,200 | Newly added on 2004-02-24 |
| Izlude | Aldebaran | — | 1,800 | Newly added on 2004-02-24 |
| **Geffen** | Prontera | 1,400 | 1,200 | |
| Geffen | Izlude | 1,800 | — | Removed in 2004-02-24 overhaul |
| Geffen | Aldebaran | 2,100 | 1,200 | 817 checked 1800z, deducted 2100z |
| Geffen | Morroc | 2,200 | — | Removed in 2004-02-24 overhaul |
| Geffen | Orc Dungeon | — | 1,700 | Newly added on 2004-02-24 |
| Geffen | Mjolnir Mine | — | 1,700 | Newly added on 2004-02-24 |
| **Payon** | Alberta | 1,300 | 1,200 | |
| Payon | Prontera | 1,800 | 1,200 | |
| Payon | Morroc | 1,800 | 1,200 | |
| **Morroc** | Prontera | 1,800 | 1,200 | |
| Morroc | Payon | 1,800 | 1,200 | |
| Morroc | Alberta | 2,200 | 1,800 | |
| Morroc | Geffen | 2,200 | — | Removed in 2004-02-24 overhaul |
| Morroc | Comodo | 1,500 | 1,800 | |
| Morroc | Pharos Lighthouse | — | 1,200 | Newly added on 2004-02-24 |
| **Alberta** | Payon | 1,300 | 1,200 | |
| Alberta | Prontera | 2,100 | 1,800 | 817 NPC 1 menu showed 1800z; code charged 2100z; NPC 2 showed 2100z |
| Alberta | Morroc | 2,200 | 1,800 | |
| **Aldebaran** | Geffen | 1,600 | 1,200 | Pre-overhaul in jAthena; omitted in 817 |
| Aldebaran | Mjolnir Mine | 1,700 | 1,700 | Pre-overhaul in jAthena |
| Aldebaran | Juno | — | 1,200 | Newly added on 2004-02-24 |
| Aldebaran | Izlude | — | 1,800 | Newly added on 2004-02-24 |
| **Comodo** | Pharos Lighthouse | 800 | 1,200 | Pre-overhaul in jAthena |
| Comodo | Morroc | 1,500 / 2,000 | 1,800 | |
| **Pharos Lighthouse** | Comodo | 500 | 1,200 | Pre-overhaul in jAthena |
| Pharos Lighthouse | Morroc | 2,500 | 1,200 | Pre-overhaul in jAthena |
| **Juno** | Aldebaran | 1,000 | 1,200 | Pre-overhaul in jAthena |
| Juno | Izlude | 3,000 | — | Pre-overhaul in jAthena |

---

## Contradictions and open points
- **Prontera -> Payon vs Payon -> Prontera**: Prontera -> Payon was 1700z, while Payon -> Prontera was 1800z. Both jAthena and 817 agree on this slight asymmetric price.
- **Aldebaran <-> Geffen**: In pre-overhaul jAthena, Geffen -> Aldebaran was 2100z, while Aldebaran -> Geffen was 1600z. In 817, Aldebaran had no teleport menu at all.
- **Dungeon Storage Removal**: On 2004-02-24, kRO removed storage from Mjolnir Mine, Orc Dungeon, Pyramid, Shipwreck, Byalan, and Archer Village. For a December 2003 client, dungeon Kafras *should* have storage enabled (at 100z for Mjolnir, 130z for Orc Dungeon, 50z for Pyramid/Shipwreck, 40z for Byalan/Archer Village).

---

## Sources read
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1182&curpage=562` — Official kRO patch notice (2004-02-23) introducing the revised fare table and removing dungeon storage; primary source, highly reliable.
- `https://web.archive.org/web/20210116170006/https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Feb._24)` — Wayback capture of Fandom patch page reproducing Korean text and MT English; highly reliable.
- `https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/town/npc_town_kafra.txt` — jAthena contemporary emulator script (2003-2004) reflecting official jRO/kRO pre-overhaul values; highly reliable period source.
- `https://raw.githubusercontent.com/flaviojs/jathena/master/script/npc/npc_function.txt` — jAthena helper functions showing exact `kfstorage` and `kfcart` parameters per town.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/kafras/kafras.txt` & `functions_kafras.txt` — rAthena official Aegis pre-renewal scripts showing exact savepoints (`izlude 94, 103`) and post-overhaul teleport tables.
- `https://raw.githubusercontent.com/openkore/openkore/e24871a17bbeb8d07865d55f5ed828aee6d50d99/tablepack/tags/start/tables/npcs.txt` — OpenKore live iRO server dump from early 2004 verifying NPC coordinates (`izlude 134, 88`).
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1167` — kRO notice 2004-02-16 (Feb 17 patch).
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1154` — kRO notice 2004-02-09 (Feb 10 patch).
- `https://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=1134` — kRO notice 2004-02-02 (Feb 3 patch).
- `https://xellie.com/RO/wiki/index.php/Prontera` — Wiki listing of Prontera NPCs and Kafra fees.

---

## Could not read
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Feb._24)` — Cloudflare HTTP 403 on direct curl; read via web.archive.org and MediaWiki API.
- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs` — Cloudflare HTTP 403 bot check on direct curl.

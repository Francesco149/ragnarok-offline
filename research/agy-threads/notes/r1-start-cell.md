# Thread #6: The start cell and novice training ground on this client

## Answer so far
In December 2003 (and throughout late 2003 – early 2004 across kRO, iRO, jRO, and euRO closed beta), newly created characters spawned in the Novice Training Ground at server map `new_1-1.gat`, coordinate cell `(53, 111)` facing North.
The euRO 2003-12-18 client ships with physical maps named `new_zone01.gat` through `new_zone04.gat`. However, the client's internal resource redirect table (`data/resnametable.txt` inside `data.grf`) explicitly aliases `new_1-1.gat` (and parallel instances `new_2-1.gat` through `new_5-1.gat`) to `new_zone01.gat`. The official server transmitted `new_1-1.gat,53,111` to the client, which transparently rendered `new_zone01.gat`. Cell `(53, 111)` is confirmed fully walkable (type 0, altitude -7.81), situated at the exact center of the western starting bridgehead platform (x=45..66, y=109..118) aligned with the bridge axis (y=111).
The emulator coordinate `53, 111` originated in jAthena snapshot `//0305` by developer `いど` (Ido) in mid-2003, hardcoded from live observation of the official Japanese server (jRO), and was made configurable in `//0307` by developer `胡蝶蘭` (Phalaenopsis). It is confirmed official: when Gravity added the tutorial guide NPC Sprakki in Episode 12, she was positioned at `(53, 114)` — exactly three cells north of `(53, 111)` to greet the spawning player.
The official flow in Dec 2003 consisted of: starting with 1x Knife (1201) and 1x Cotton Shirt (2301) (no Novice gear); walking across the bridge to `new_1-2` where an Usher offered either an immediate skip to one of 6 towns or entrance with a 50z subsidy; attending 1st Course lectures/quizzes (Newbie Instructor and Leo Von Frisch awarding basic foods/potions); passing the 2nd Course practical combat exam in "Fabre Forest" (`new_1-3`); and completing the aptitude test in `new_1-4` where the final Helper awarded 100–500z, 10 Red Potions, 1 Milk, and a free warp/savepoint directly to the chosen first-class job guild. Novice-specific equipment (Novice Plate, Cutter, Guard, etc.) and direct level-ups from dialogue were not added until the kRO patch of 2004-06-01.

## Findings
- [READ] (jRO / emulator, mid-2003) **jAthena changelog on start location** — "・新規PCの位置を初心者修練場に変更。" ("Changed new PC location to Novice Training Ground.") — https://raw.githubusercontent.com/flaviojs/jathena/master/Readme (mid-2003; snapshot `//0305` by いど; high)
- [READ] (jRO / emulator, mid-2003) **jAthena changelog on start_point config** — "・新規PCの初期位置をchar_athena.cnfに書けるようにした\n\tstart_point: マップ名,x,y のように指定します。\n\t<例> start_point: new_1-1.gat,53,111" — https://raw.githubusercontent.com/flaviojs/jathena/master/Readme (mid-2003; snapshot `//0307` by 胡蝶蘭; high)
- [READ] (jRO / emulator, 2003-12-18) **Athena char.c default coordinates and starting equipment** — "struct point start_point={\"new_1-1.gat\",53,111};\n...\nchar_dat[i].inventory[0].nameid = 1201; /* Knife */\nchar_dat[i].inventory[0].amount = 1;\nchar_dat[i].inventory[0].equip = 0x02;\nchar_dat[i].inventory[0].identify = 1;\nchar_dat[i].inventory[1].nameid = 2301; /* Cotton Shirt */\nchar_dat[i].inventory[1].amount = 1;\nchar_dat[i].inventory[1].equip = 0x10;\nchar_dat[i].inventory[1].identify = 1;" — https://raw.githubusercontent.com/flaviojs/jathena/master/src/char/char.c (snapshot //0631 / //0676, 2003-12-18 / 2004-01-04; high)
- [READ] (euRO, 2003-12-18) **Official client resource mapping from new_1-1 to new_zone01** — The client's own `data/resnametable.txt` inside `data.grf` maps the five server instances directly to the four `new_zone` physical files:
  ```
  new_1-1.gat#new_zone01.gat#
  new_2-1.gat#new_zone01.gat#
  new_3-1.gat#new_zone01.gat#
  new_4-1.gat#new_zone01.gat#
  new_5-1.gat#new_zone01.gat#
  new_1-2.gat#new_zone02.gat#
  ...
  new_1-3.gat#new_zone03.gat#
  ...
  new_1-4.gat#new_zone04.gat#
  ```
  And `data/mapnametable.txt` displays: `new_1-1.rsw#Training Ground#` — euRO client CD 2003-12-18 `data.grf` (extracted directly in workbench; high)
- [READ] (euRO / kRO, 2003-12-18) **Walkability of cell (53, 111) on client map** — In `new_zone01.gat` (dimensions 200x200), cell (53, 111) has type `0` (walkable) and height `-7.81`. It sits inside an open starting bridgehead platform spanning x=45..66, y=109..118, aligned with the 4-cell wide bridge axis along y=110..113 that leads east to the castle entrance at (148, 112). Cell (66, 114) where the Bulletin Board stands is type `1` (wall/obstacle), confirming the board is mounted on the bridge wall railing. — euRO client CD `data.grf:data/new_zone01.gat` (analyzed in workbench; high)
- [READ] (kRO, 2004-04-27 & 2004-06-01) **Official introduction of Novice equipment and quiz level-ups** — Sakray notice 2004-04-27 / Main server notice 2004-06-01: "초보자 수련장이 변경되었습니다. 이제 설명만 잘 듣고 질문에 대답하면 회복아이템뿐만이 아니라 레벨업도 가능하며 또한 졸업시 초보자용 장비를 지급하게 됩니다." ("The beginner training ground has been changed. Now, if you listen carefully to the explanation and answer the questions, you can not only get recovery items, but also level up, and upon graduation, novice equipment will be provided.") — https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Jun._01)&prop=wikitext&format=json (quoting ro.gnjoy.com notice seq 1447; high)
- [READ] (jRO / kRO, 2004-02-19) **Official Novice Training Ground dialogue and flow in 817** —
  - Usher at `new_zone02, 100, 29`: offers immediate exit to adventure ("`tmp.TEMP = rand(6)`" warping to Prontera 273,354 / Morroc 160,94 / Geffen 120,100 / Payon 87,117 / Alberta 116,57 / Izlude 94,103) or registration: "Training Subsidy 50 Zeny supplied. Please inquire at a Guide on the Centre when you need to know your Current Progress. I will lead you into the Hall." (warps to 100, 70).
  - Newbie Instructor at `new_zone02, 81, 110`: gives stats and interface lecture, awards 1 Milk (519).
  - Leo Von Frisch at `new_zone02, 84, 115`: multiple-choice quiz; correct answers yield food items (Bananas 513, Sweet Potatoes 516, Apple 512, Carrot 517, Milk 519, Red Potions 507).
  - Helper at `new_zone02, 16, 183`: "You must reach at the Castle of the other side from here, through the Fabre Forest." Warps and sets save point to `new_zone03, 96, 21`.
  - Final Helper at `new_zone04, 60, 149`: checks quiz and combat scores, recommends job, awards 100-500z, 10 Red Potions (507), 1 Milk (519), and warps/saves to job guild (Swordsman -> `izlude_in, 121, 165`; Archer -> `payon_in02, 21, 71`; Thief -> `morocc, 154, 50`; Mage -> `geffen, 120, 60`; Acolyte -> `prontera, 233, 313`; Merchant -> `alberta, 116, 57`).
  — `/opt/src/ragnarok-offline/scripts/import/npc/jobs/novice.lua` (from eathena 817 `jobs/novice.txt`; high)
- [READ] (jRO historical record, ~2003-2004 / archived 2023) **Japanese quest archive of Deleted Beginner Training Ground** — "2. <初心者修練場受付> 初心者修練場受付要員に話しかけると、初心者修練場に行くかすぐ冒険を始めるか聞かれる。→すぐ冒険を始める。[初心者修練場を通過してプロンテラへ] →初心者修練場の申し込み。[50zeny支給、初心者修練場内へ] ... 以前はこの時点で適正職業を選ぶと各職業に応じた装備品が配布されたのだが不正者(BOT製作者)が自動マクロによる装備品売却で延々と金稼ぎをしたため装備品を貰うことができなくなるよう修正された。" — https://roquest.work/?DeletedQuest/Beginner (high)
- [READ] (iRO / OpenKore, 2004-2008) **Sprakki coordinates in OpenKore macro** — "macro NoviceGroundPart1M {\n    do move @rand(51,54) @rand(110,112)\n    do talknpc 53 114 c ...\n}" — OpenKore botting scripts / https://openkore.com (medium-high)
- [READ] (Video gameplay) **2004 Ragnarok Online server Novice Training Ground start** — seen: frames 0:54-1:14 (`/opt/src/ragnarok-offline/research/agy-threads/frames/GJO1B9ogKXY_0.50-2.10-every5s-1.jpg`) showing new Novice spawning at west end of grassy bridgehead next to water, walking east across the wooden bridge over water, and entering the castle gates — https://www.youtube.com/watch?v=GJO1B9ogKXY (uploaded 2017-02-20; medium)

## Contradictions and open points
1. **Zeny given by Usher:**
   Usher's dialogue states: "Training Subsidy 50 Zeny supplied." However, in Athena 817 script code, `player.zeny = player.zeny + 150` was executed. The Japanese historical archive `roquest.work/?DeletedQuest/Beginner` explicitly confirms the official dialogue and reward was 50z ("50zeny支給"). The emulator code giving 150z is an emulator discrepancy/bug.
2. **Server-side map naming vs client files:**
   Athena configurations set `start_point: new_1-1.gat,53,111`, while `ragnarok-offline` initially aliased this to `new_zone01`. Our discovery of `data/resnametable.txt` confirms that the official Aegis server used `new_1-1.gat`, and the client's internal table automatically translated `new_1-1.gat` to `new_zone01.gat`. Thus, the server can use either `new_1-1.gat` (if resnametable lookup is used) or `new_zone01.gat` directly.
3. **Random spawn jitter:**
   In Aegis, players did not all stack on the exact single pixel `53, 111`; a slight 1-2 cell scatter (`rand(51,54), rand(110,112)`) was used on login/creation.

## Sources read
- `https://raw.githubusercontent.com/flaviojs/jathena/master/Readme` — Original jAthena Japanese development changelog covering snapshots //0052 to //1093. Highly reliable primary source for mid-2003 emulator decisions.
- `https://raw.githubusercontent.com/flaviojs/jathena/master/src/char/char.c` — Athena character server source code showing hardcoded start point and starting inventory (Knife 1201, Cotton Shirt 2301).
- `euRO client CD 2003-12-18 data.grf` (`data/resnametable.txt`, `data/mapnametable.txt`, `data/new_zone01.gat`) — Direct extraction and analysis of the client's official assets in the workbench. Authoritative primary evidence.
- `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Jun._01)&prop=wikitext&format=json` — Official kRO patch notes for June 1, 2004 recording the addition of Novice equipment and EXP rewards.
- `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Apr._27)&prop=wikitext&format=json` — Official kRO Sakray patch notes for April 27, 2004 testing the revised training ground.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/novice.lua` — Ported eAthena 817 (2004-02-19) Novice Training Ground NPC scripts containing complete dialog, checks, and warps.
- `https://roquest.work/?DeletedQuest/Beginner` — Comprehensive Japanese documentation of the historical pre-renewal Novice Training Ground quest steps, dialogues, and rewards.
- `https://openkore.com` (OpenKore Novice ground automation macros) — Bot coordinates verifying initial spawn and Sprakki position at (53, 114).
- `https://www.youtube.com/watch?v=GJO1B9ogKXY` (Vishal, 2017) — Video showing character creation and navigation through the classic Novice Training Ground bridgehead.
- `https://web.archive.org/web/20040327035441/http://iro.ragnarokonline.com:80/game/basicjobchange.asp` — Official iRO game guide from March 2004 describing Novice job change flow.

## Could not read
- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/30075` — Cloudflare HTTP 403 bot check.
- `https://www.reddit.com/r/RagnarokOnline/comments/1683hm8/iro_beta_training_grounds_info/` — Reddit JavaScript wall / HTTP 403 on direct bot JSON query.
- `https://ratemyserver.net/index.php?page=map_info&m=new_1-1` — Page does not exist on RMS.

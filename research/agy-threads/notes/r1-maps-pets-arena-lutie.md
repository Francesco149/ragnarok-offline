# Research Notes: Threads #12, #14, #18, #19 (Map Flags, Pets, Izlude Arena, Lutie)

Sub-question: `maps-pets-arena-lutie` (Threads #12, #14, #18, #19)
Worker: Antigravity Subagent (r1 deep-research)
Date: 2026-11-09

---

## Answer so far

1. **Thread #12 (Map flags):** 
   - **Teleport (`noteleport`):** In official 2003–2004 RO (and client-side `data\indoorrswtable.txt`), teleport was strictly forbidden on all 31 indoor maps and on all 20 WoE guild castles (`aldeg_cas01..05`, `gefg_cas01..05`, `payg_cas01..05`, `prtg_cas01..05`). Normal dungeons (`pay_dun`, `iz_dun`, `glast heim`, etc.) officially **allowed** teleportation. mod0631 (Dec 2003) applied `noteleport` to 34 indoor maps (`indoorrswtable.txt` + `in_hunter`, `in_moc_16`, `prt_are01`). 817 (Feb 2004) expanded this by adding the 20 guild castles and `um_in` (55 maps).
   - **Memo (`nomemo`):** Officially forbidden in all dungeons (74 maps), all indoor maps, and all guild castles. mod0631 flagged 108 maps; 817 expanded this to 178 maps (adding the 20 castles, job trial maps `hunter_1-1..3-1`, `knight_1-1..3-1`, `force_1-1..3-3`, and novice grounds).
   - **Save (`nosave`):** Officially applied only to event/job test areas (`prt_are_in`, `force_1-1`, job test maps) directing relogging characters to `SavePoint`.
   - **Death penalty (`nopenalty`):** Officially, death penalty (1% EXP loss) was absent only in WoE Castles (`gvg`), PvP rooms, and special events. In official RO, dying in towns from Dead Branch monsters or poison incurred the normal EXP penalty. mod0631 invented `nopenalty` on the 12 outdoor towns (`prontera`, `morocc`, `geffen`, `payon`, `alberta`, `izlude`, `aldebaran`, `xmas`, `comodo`, `yuno`, `amatsu`, `gonryun`) to protect players. 817 restructured `nopenalty.txt` into 35 indoor maps + 80 PvP rooms + 3 priest test maps, accidentally omitting the outdoor towns from `nopenalty.txt`.

2. **Thread #14 (Pets on euRO):**
   - The official Cute Pet System guide on iRO (`qpetsystem.asp`, 2004-03) documented **exactly 24 tameable monsters** (Eggs 9001–9024): Poring, Drops, Poporing, Lunatic, Picky, Chonchon, Steel Chonchon, Hunter Fly, Savage Babe, Baby Desert Wolf, Rocker, Spore, Poison Spore, Pecopeco, Smokie, Yoyo, Orc Warrior, Munak, Dokebi, Sohee, Isis, Petite, Deviruchi, Baphomet Jr.
   - mod0631 and 817 `db/pet_db.txt` contained **25 monsters**: the original 24 plus **Bongun** (Egg 9025, taming item 659 "Her Heart", food 537 Pet Food, accessory 10020 Grave Keeper's Sword). Bongun was introduced as a 25th special pet tied to the Munak/Bongun story and Comodo trading quest, present in emulator databases by late 2003 even while official website guides still stated "a total of 24 monsters".

3. **Thread #18 (Izlude Arena):**
   - **Neither iRO nor euRO had the time-attack arena in Dec 2003.**
   - On kRO, the Izlude Arena was teased on **2004-06-01** (entrance statue placed), opened on Sakray on **2004-09-14** (`RO Patch (2004 Sept. 14)`), officially added to main servers on **2004-10-12** (`RO Patch (2004 Oct. 12)`: "After a long renovation, the Arena has been added to the village of Izlude"), temporarily closed on **2004-10-19**, and reopened on **2004-11-02**. iRO received it in late 2004.
   - The arena scripts shipped in mod0631 (`npc_test_arena.txt`) and 817 (`npc/other/arena.txt` and `test_arena.txt`) were **custom emulator scripts** set in Prontera (`prt_are_in.gat` and `force_1-1.gat`), not the official Izlude Arena. 817 also included `arena_mvp.txt`, a private server script for "SvRO" using `quiz_00.gat`.

4. **Thread #19 (Lutie in January):**
   - **Lutie was a permanent town**, not a seasonal event. Introduced in kRO Episode 2 (December 17, 2002), the town (`xmas`), snow field (`xmas_fild01`), Toy Factory dungeon (`xmas_dun01..02`), and the Aldebaran Santa Claus warp NPC (`aldebaran 223, 222`) were permanently accessible year-round across kRO, iRO, and euRO. Only holiday gift quests and event monster spawns were seasonal.
   - Disassembly of the euRO CD patcher (`Ragnarok.exe`, built 2003-12-12) reveals that `2003-10-31gdata_xmas.gpf` and `notice_xmas.txt` are **hardcoded in the executable** (offsets `0x1d0e4` and `0x1d0d4`). At startup, the patcher reads `http://wpatch.euro-ro.net/patch/patch_allow.txt`. If the server returns `"xmas"` (checked via `strncmp` at offset `0x51c4`), the patcher immediately fetches `2003-10-31gdata_xmas.gpf` and shows `notice_xmas.txt` before proceeding to `patch2.txt`. Gravity baked this Christmas remote trigger into the patcher binary on 2003-10-31.

---

## Findings

### Thread #12: Map Flags (Teleport, Memo, Save, Penalty)
- [READ] `data\indoorrswtable.txt` (client data.grf, 2003-12-18; tier client) defines exactly 31 indoor maps:
  `alberta_in.rsw#`, `izlude_in.rsw#`, `gef_tower.rsw#`, `geffen_in.rsw#`, `moc_castle.rsw#`, `morocc_in.rsw#`, `payon_in01.rsw#`, `payon_in02.rsw#`, `prt_in.rsw#`, `prt_castle.rsw#`, `prt_church.rsw#`, `in_orcs01.rsw#`, `aldeba_in.rsw#`, `monk_in.rsw#`, `prt_are_in.rsw#`, `arena_room.rsw#`, `sword_1-1.rsw#`, `sword_2-1.rsw#`, `sword_3-1.rsw#`, `xmas_in.rsw#`, `cmd_in01.rsw#`, `cmd_in02.rsw#`, `yuno_in01.rsw#`, `yuno_in03.rsw#`, `yuno_in04.rsw#`, `yuno_in05.rsw#`, `alde_alche.rsw#`, `sec_in01.rsw#`, `ama_in01.rsw#`, `ama_in02.rsw#`, `gon_in.rsw#`.
  *Client behavior:* The client natively treats these maps as indoor spaces, disabling teleport and displaying "Unable to teleport in this area".
- [READ] mod0631 `conf/mapflag.txt` (dated 2003-11-25 / 2003-12-18; tier athena2003) by Jioh L. Jung defined:
  - `noteleport` and `nobranch` on 34 maps (the 31 indoor maps + `in_hunter.gat`, `in_moc_16.gat`, `prt_are01.gat`).
  - `nopenalty` on exactly 12 outdoor towns: `prontera.gat`, `morocc.gat`, `geffen.gat`, `payon.gat`, `alberta.gat`, `izlude.gat`, `aldebaran.gat`, `xmas.gat`, `comodo.gat`, `yuno.gat`, `amatsu.gat`, `gonryun.gat`.
  - `nomemo` on 108 maps: all 74 dungeon floors (`alde_dun*`, `anthell*`, `beach_dun*`, `c_tower*`, `gef_dun*`, `gl_*`, `gld_dun*`, `in_sphinx*`, `iz_dun*`, `mjo_dun*`, `moc_pryd*`, `orcsdun*`, `pay_dun*`, `prt_maze*`, `prt_sewb*`, `treasure*`, `xmas_dun*`) plus indoor/test maps.
  *Quote from mod0631 conf/mapflag.txt:*
  `//MapFlag By Jioh L. Jung`
  `prontera.gat  mapflag nopenalty dummy`
  `alberta_in.gat mapflag noteleport dummy`
- [READ] 817 (2004-02-18, `conf/mapflag/`; tier athena2003) split mapflags into separate files:
  - `gvg.txt`: 19 castle maps (`aldeg_cas02..05`, `gefg_cas01..05`, `payg_cas01..05`, `prtg_cas01..05`; `aldeg_cas01` omitted by typo).
  - `noteleport.txt` (55 maps) & `nobranch.txt` (55 maps): the 34 mod0631 indoor maps + `um_in.gat` + all 20 guild castles (`aldeg_cas01..05`, `gefg_cas01..05`, `payg_cas01..05`, `prtg_cas01..05`).
  - `nomemo.txt` (178 maps): dungeons + indoor maps + 20 castles + job quests (`hunter_1-1..3-1`, `knight_1-1..3-1`, `force_1-1..3-3`, `new_1-1..4-4`).
  - `nopenalty.txt` (35 maps): 35 indoor maps (`alberta_in`, `aldeba_in`, etc.). The 12 outdoor towns were omitted from `nopenalty.txt`. (80 PvP maps were flagged in `npc/other/pvp.txt` lines 975–1046).
- [READ] rAthena `npc/mapflag/nopenalty.txt` (tier later) records the intended resolution:
  Both outdoor towns AND indoor maps have `nopenalty`:
  `// Municipality (Cities/Towns/Villages)`
  `alberta mapflag nopenalty`
  `alberta_in mapflag nopenalty`
  `aldebaran mapflag nopenalty`
  `aldeba_in mapflag nopenalty`

### Thread #14: Pets on euRO & iRO
- [READ] iRO official guide `http://iro.ragnarokonline.com/game/qpetsystem.asp` (archived `20040404074213`; tier primary official):
  "There are a total of 24 monsters that can be tamed to be pets. While the Poring, Deviruchi, and Sohee are the most favored, there are several other monsters that also have distinct personalities."
- [READ] kRO patch notes `RO Patch (2002 Nov. 19)` (ro.gnjoy.com seq=224, Sakray release; tier primary official):
  "■ 큐펫 시스템 (몬스터 테이밍 시스템) 이 업데이트 되었습니다. 큐펫 시스템은 일정한 아이템을 사용하여 몬스터를 자신의 애완동물처럼 키울 수 있는 시스템입니다."
  ("Cute Pet system (monster taming system) has been updated. Cute Pet system is a system that allows you to raise monsters like your own pets by using certain items.")
- [READ] mod0631 `db/pet_db.txt` & 817 `db/pet_db.txt` (tier athena2003) contain **25 monsters** (mobs 1002, 1011, 1014, 1019, 1023, 1026, 1029, 1031, 1035, 1042, 1049, 1052, 1056, 1057, 1063, 1077, 1101, 1107, 1109, 1110, 1113, 1155, 1167, 1170, 1188):
  The 25th entry is:
  `1188,BON_GUN,659,9025,10020,537,80,20,10,100,250,20,500,200`
  (Bongun: taming item 659 "Her Heart", Egg 9025, accessory 10020 "Grave Keeper's Sword", food 537 "Pet Food").
- [READ] Pet items and accessories table (confirmed against client `idnum2itemdesctable` and `data/db/pets.toml`):
  1. Poring: Unripe Apple (619), Apple Juice (531), Poring Pet Backpack (10013)
  2. Drops: Orange Juice (620), Yellow Herb (508), Poring Pet Backpack (10013)
  3. Poporing: Bitter Herb (621), Green Herb (511), Poring Pet Backpack (10013)
  4. Lunatic: Rainbow Carrot (622), Carrot Juice (534), Silk Ribbon (10007)
  5. Picky: Earthworm the Dude (623), Red Herb (507), Tiny Egg Shell (10012)
  6. Chonchon: Rotten Fish (624), Pet Food (537), Monster Oxygen Mask (10002)
  7. Steel Chonchon: Rusty Iron (625), Iron Ore (1002), Monster Oxygen Mask (10002)
  8. Hunter Fly: Monster's Oxygen (626), Red Gemstone (716), Monster Oxygen Mask (10002)
  9. Savage Babe: Sweet Milk / Sweet Potato (627), Pet Food (537), Green Lace (10015)
  10. Baby Desert Wolf: Well-Dried Bone (628), Pet Food (537), Transparent Head Protector (10003)
  11. Rocker: Singing Flower (629), Pet Food (537), Rocker Glasses (10014)
  12. Spore: Dew Laden Moss (630), Pet Food (537), Bark Shorts (10017)
  13. Poison Spore: Deadly Noxious Herb (631), Pet Food (537), Bark Shorts (10017)
  14. Pecopeco: Fat Chubby Earthworm (632), Pet Food (537), Battered Pot (10010)
  15. Smokie: Sweet Potato / Sweet Roda Frog (633), Pet Food (537), Red Scarf (10019)
  16. Yoyo: Tropical Banana (634), Banana Juice (532), Monkey Circlet (10018)
  17. Orc Warrior: Orc Trophy (635), Pet Food (537), Wild Flower (10009)
  18. Munak: No Recipient (636), Pet Food (537), Punisher (10008)
  19. Dokebi: Old Broom / Shining Stone (637), Pet Food (537), Wig (10005)
  20. Sohee: Silver Knife of Chastity / Black Chiffon (638), Pet Food (537), Golden Bell (10016)
  21. Isis: Armlet of Obedience (639), Pet Food (537), Queen's Hair Ornament (10006)
  22. Petite: Shining Stone / Shining Spore (640), Pet Food (537), Stellar Hairpin (10011)
  23. Deviruchi: Contract in Shadow (641), Shoot (711), Pacifier (10004)
  24. Baphomet Jr.: Book of the Devil (642), Honey (518), Skull Helm (10001)
  25. Bongun: Her Heart (659), Pet Food (537), Grave Keeper's Sword (10020)

### Thread #18: Izlude Arena
- [READ] kRO patch notes `RO Patch (2004 Jun. 01)` (ro.gnjoy.com seq=1189; tier primary official):
  "- 이즈루드의 북쪽 아레나 입구앞에 있는 동상을 클릭하여 보시기 바랍니다."
  ("Please click on the statue in front of the entrance to the north side of Izlude.")
- [READ] kRO patch notes `RO Patch (2004 Sept. 14)` (ro.gnjoy.com seq=1742, Sakray; tier primary official):
  "- 이즈루드 마을에 아레나가 오픈됩니다. 아레나에 진입하시어 사크라이 유저 여러분들의 많은 제보와 건의사항을 부탁드리겠습니다."
  ("Arena is opened in Izlude village. Please enter the arena and ask a lot of tips and suggestions from Sakray users.")
- [READ] kRO patch notes `RO Patch (2004 Oct. 12)` (ro.gnjoy.com seq=1796, Main Server; tier primary official):
  "- 오랜시간의 재공사끝에 이즈루드 마을에 아레나가 추가되었습니다."
  ("After a long renovation, the Arena has been added to the village of Izlude.")
- [READ] kRO patch notes `RO Patch (2004 Oct. 19)` (ro.gnjoy.com seq=1806; tier primary official):
  "- 이즈루드 마을의 아레나가 문제점이 발견되어 잠시 폐쇄됩니다."
  ("Arena of Izlude village is found problem and it is closed temporarily.")
- [READ] kRO patch notes `RO Patch (2004 Nov. 02)` (ro.gnjoy.com seq=1833; tier primary official):
  "- 이즈루드에 아레나가 재개장됩니다."
  ("The Arena is reopened in Izlude.")
- [READ] mod0631 `conf/npc_test_arena.txt` (tier athena2003):
  `prontera.gat,160,185,0 script アリーナ 116` -> warps to `prt_are_in.gat,31,82` -> `force_1-1.gat,99,12` for a 30-second 5-Poring time-attack.
- [READ] 817 `npc/other/arena.txt` (tier athena2003):
  `prontera.gat,160,185,3 script Arena Entrance 116` -> `prt_are_in.gat,29,79,0 script Time Attack 116` with Levels 1 (5 Porings), 2 (7 Roda Frogs), 3 (9 PecoPecos).
  *Conclusion:* Izlude Arena did NOT exist on iRO or euRO in Dec 2003. It was implemented on kRO in autumn 2004. Emulator arena scripts were Prontera-based test scripts.

### Thread #19: Lutie & CD Patcher xmas Strings
- [READ] kRO Episode timeline (Hazy Forest / ro.gnjoy.com; tier primary official):
  Episode 2: "A Call to the Snowland: Lutie" went live on kRO on **2002-12-17**. The maps (`xmas.gat`, `xmas_fild01.gat`, `xmas_dun01.gat`, `xmas_dun02.gat`) and the Aldebaran Santa NPC (`aldebaran 223, 222`) are permanent game fixtures, not temporary event maps.
- [READ] euRO CD client `data.grf` (built 2003-12-19; tier client) contains all Lutie maps (`xmas.gat`, `xmas_fild01.gat`, `xmas_dun01.gat`, `xmas_dun02.gat`, `xmas_in.gat`) and models.
- [READ] euRO CD patcher binary `Ragnarok.exe` (dated 2003-12-12; tier client):
  Strings table:
  - offset `0x1d0d4`: `notice_xmas.txt`
  - offset `0x1d0e4`: `2003-10-31gdata_xmas.gpf`
  - offset `0x1d100`: `xmas`
  - offset `0x1d108`: `deny`
  - offset `0x1d310`: `patch_allow.txt`
- [READ] `Ragnarok.exe` disassembly:
  - Offset `0x6eee`: initializes URL table pointing to `http://wpatch.euro-ro.net/patch/patch_allow.txt`.
  - Offset `0x5150`–`0x5165`: compares received payload against `"deny"` (`strncmp(edi, "deny", 4)`). If equal, aborts with maintenance message (`notice_close.txt`).
  - Offset `0x51c0`–`0x51d5`: compares payload against `"xmas"` (`strncmp(edi, "xmas", 4)`). If equal, downloads `2003-10-31gdata_xmas.gpf` (offset `0x51d6`) and displays `notice_xmas.txt` (offset `0x51e7`).
  - Offset `0x51e0`+: otherwise proceeds to standard `patch2.txt` / `notice_patch.txt` pipeline.
  *Conclusion:* Gravity hardcoded an `"xmas"` trigger into `Ragnarok.exe` allowing the patch server operator to remotely activate the Christmas 2003 event patch (`2003-10-31gdata_xmas.gpf`) simply by changing `patch_allow.txt` to return `"xmas"`.

---

## Contradictions and open points

1. **Town death penalty in 817 vs mod0631:**
   mod0631 had `nopenalty` on 12 outdoor towns. 817 split `conf/mapflag.txt` into `conf/mapflag/*.txt` and created `nopenalty.txt` containing only 35 indoor maps, inadvertently omitting the outdoor towns from `nopenalty.txt` (though 80 PvP rooms were flagged in `npc/other/pvp.txt`). In official Gravity servers, dying in towns from Dead Branch monsters or poison resulted in normal EXP loss (1%); the `nopenalty` on towns was an emulator protection against dead branch griefing.
2. **Castle gvg.txt typo in 817:**
   817 listed only 19 castles in `gvg.txt`, omitting `aldeg_cas01.gat`. It correctly included `aldeg_cas01.gat` in `noteleport.txt`, `nobranch.txt`, and `nomemo.txt`.
3. **24 vs 25 Pets:**
   Official iRO guides stated "a total of 24 monsters that can be tamed" (Eggs 9001–9024). Bongun (Egg 9025) was added to kRO shortly after as a 25th special pet via a Comodo trade quest (`Her Heart`), and was incorporated into emulator `pet_db.txt` by late 2003.

---

## Sources read

1. `https://web.archive.org/web/20040404074213/http://iro.ragnarokonline.com/game/qpetsystem.asp` — Official iRO Cute Pet System guide (captured 2004-04-04; high reliability primary official source). Confirms 24 original pets, taming mechanics, feeding, and accessories.
2. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2002_Nov._19)&prop=wikitext&format=json` — kRO patch notes (ro.gnjoy.com seq=224; high reliability primary official). Sakray introduction of Cute Pet System.
3. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Jun._01)&prop=wikitext&format=json` — kRO patch notes (ro.gnjoy.com seq=1189; high reliability primary official). Izlude north arena statue update.
4. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Sept._14)&prop=wikitext&format=json` — kRO patch notes (ro.gnjoy.com seq=1742; high reliability primary official). Izlude Arena opens on Sakray.
5. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Oct._12)&prop=wikitext&format=json` — kRO patch notes (ro.gnjoy.com seq=1796; high reliability primary official). Izlude Arena added to kRO main servers after long renovation.
6. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Oct._19)&prop=wikitext&format=json` — kRO patch notes (ro.gnjoy.com seq=1806; high reliability primary official). Izlude Arena temporarily closed for bugfixes.
7. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Nov._02)&prop=wikitext&format=json` — kRO patch notes (ro.gnjoy.com seq=1833; high reliability primary official). Izlude Arena reopened.
8. `/data/play/RagnarokOnline/data.grf` (`data\indoorrswtable.txt`, `data\mapnametable.txt`) — Official euRO CD client data files (dated 2003-12-18; tier client). Authoritative list of 31 client indoor maps.
9. `/data/play/RagnarokOnline/Ragnarok.exe` — Official euRO CD patcher binary (dated 2003-12-12; tier client). Disassembly reveals `patch_allow.txt` `"xmas"` trigger for `2003-10-31gdata_xmas.gpf` and `notice_xmas.txt`.
10. `/data/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/` — Contemporary jAthena/eAthena snapshot (archived 2003-12-18; tier athena2003). `conf/mapflag.txt`, `db/pet_db.txt`, `conf/npc_test_arena.txt`.
11. `/data/refs/eathena-archive/x/athena-817-src/` — Contemporary eAthena 817 snapshot (archived 2004-02-19; tier athena2003). `conf/mapflag/*.txt`, `npc/other/arena.txt`, `db/pet_db.txt`.
12. `/data/refs/rathena/` — rAthena repository `npc/mapflag/` (tier later). Verification of subsequent mapflag evolution.

---

## Could not read
- `http://iro.ragnarokonline.com/allim/content.asp?bbs=tNotice` bulk CDX retrieval — Wayback CDX server returned 503 / "temporarily offline" intermittently; individual historical snapshots fetched directly via curl / fetch helper succeeded.

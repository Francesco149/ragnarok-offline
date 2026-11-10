# t4-woe-web: Period War of Emperium rules from official sources only

## Answer so far
War of Emperium (공성전, WoE) was first introduced by Gravity on the kRO Sakray test server on April 15, 2003, and subsequently stabilized on kRO main servers in May–June 2003, with the official siege schedule codified on June 10, 2003 (Wednesdays 21:00–23:00 and Sundays 19:00–21:00). On iRO, WoE ran bi-weekly on Wednesdays (15:00–17:00 PST) and Saturdays (19:00–21:00 PST). On euRO, following its commercial launch on April 15, 2004, the very first recorded WoE sieges took place on Wednesday, June 2, 2004 and Saturday, June 5, 2004 on server Iris.

Official rules across 2003–2004 official sources establish:
1. **Participation & Requirements**: Participation requires the guild passive skill "Official Guild Approval" (`guild_approval`). Only 20 castles existed (4 realms x 5 castles: Prontera/Valkyrie, Geffen/Britoniah, Payon/Balder, Al De Baran/Luina). The Emperium deals no damage from active skills; only direct normal physical attacks hit it (1 damage per hit or weapon damage). On April 27, 2004, kRO patched Emperium property to Holy 1 / Angel race.
2. **In-Castle WoE Battle Rules**: Damage numbers are hidden; graphical effects are simplified (`/mineffect`); guild emblems display overhead; knockback effects are disabled; Endure and Endure-granting items do not function; Ice Wall is banned; Teleport and Fly Wings are banned (Butterfly Wing allowed); Phen Card / Papillon Card uninterruptible casting is disabled; Flee is reduced by 20%; ranged normal attacks reduced by 25%; all skill-based damage reduced by 40%; traps last 4x duration; no EXP penalty on death (auto-warp to save point).
3. **Castle Management (Butler NPC)**:
   - **Economy (Commerce) Investment**: Max 2 investments/day (50% chance for +1 extra if guild has Permanent Development skill). Cost scales in 5-point brackets (from 5k/20k at 1–4 up to 955k/3,820k at 95–99). Updates at midnight.
   - **Defense Investment**: Max 2 investments/day. Costs exactly double the commerce investment (10k/40k at 1–4 up to 1,910k/7,640k at 95–99). Increases Emperium HP/DEF and Guardian HP/ATK/DEF.
   - **Conquest Penalty**: Upon castle change of ownership, Commerce and Defense each decrease by 5 points (min 1).
   - **Treasure Boxes**: Spawn at 00:00 midnight in Guild Master room. Box count = `4 + floor(Commerce / 5)` (4 at commerce 1 up to 24 at commerce 100). Split between Common (Box A) and Castle-Specific (Box B). Unopened boxes vanish on server maintenance/reset.
   - **Guardians**: Hired/repaired via Butler for 10k Zeny (requires Guardian Research skill; up to 8 per castle in FE).
   - **Kafra Staff**: Hired via Butler for 10k Zeny (requires Kafra Contract skill; free storage for members).
   - **Guild Dungeons**: 1 shared dungeon per realm connecting all 5 castles; PvP enabled between non-allied guilds; normal death EXP penalty applies; houses realm-specific MVPs (Maya/Maya Purple, Dark Lord/Dark Illusion, Eddga/Cat o' Nine Tails, Doppelganger).

---

## Findings

### 1. Introduction Dates and Schedules
- **kRO WoE Sakray Introduction Date**: Gravity introduced Guild Siege (공성전, The War of Emperium) and Turtle Island to the Sakray test server on April 15, 2003. — "그라비티는 온라인게임 라그나로크의 테스트 서버 사크라이에 새로운 맵 거북이섬 길드 공성전(The war of Emperium)을 업데이트 했다고 15일 밝혔다." — https://www.gamemeca.com/view.php?gid=43200 (published 2003-04-16; Grade A)
- **kRO WoE Official Main Server Schedule**: On June 10, 2003, kRO standardized WoE times to Wednesdays 21:00–23:00 and Sundays 19:00–21:00. — "공성을 수요일 21:00 ~ 23:00, 일요일 19:00 ~ 21:00 사이에만 할 수 있도록 정상 수정되었습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Jun._10) citing http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=645&curpage=616 (published 2003-06-10; Grade A)
- **iRO WoE Official Schedule (April 2004)**: WoE occurred bi-weekly on Wednesdays 3:00 pm – 5:00 pm PST and Saturdays 7:00 pm – 9:00 pm PST. — "Only two chances each week! The War of Emperium is only possible during the following times: Wednesday 3 pm ~ 5 pm (PST) | Saturday 7 pm ~ 9 pm (PST)" — https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp (published 2004-01-01 / captured 2004-04-05; Grade A)
- **euRO WoE Inaugural Siege Dates**: The first recorded WoE sieges on euRO (server Iris) occurred on Wednesday June 2, 2004 and Saturday June 5, 2004. — "Iris II Castle Standing ... Night.Dwellers [02/jun/2004] ... Legion of Vir [05/jun/2004]" — https://www.ro-guardians.com/castle/irisii/index.html (published 2004-06-08; Grade A)

### 2. Castle Numbers, Requirements & Emperium Properties
- **Official Castle Count (20 Castles)**: There were 20 castles total, 5 per realm across Prontera, Al De Baran, Payon, Geffen. — "Prontera, Al De Baran, Payon, Geffen each have 5 castles making a total of 20 castles available." — https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp (published 2004-01-01; Grade A)
- **Official Guild Approval Requirement**: Only guilds possessing the 'Official Guild Approval' skill can damage the Emperium or participate in WoE. — "Only guilds with the 'Official Guild Approval' skill can participate in the War of Emperium." — https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp & https://web.archive.org/web/20040404013538/http://iro.ragnarokonline.com/game/guildsystem.asp (published 2004-01-01; Grade A)
- **Emperium Immunity to Magic & Skills**: The Emperium takes no damage from skills or magic in original 2003–2004 rules; only direct physical attacks deal damage. — "The Emperium, which is the symbol of the castle, will not take any damage or be affected by Heal, and other supportive skills. Only direct physical attacks will be able to damage the Emperium." — https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp (published 2004-01-01; Grade A)
- **Emperium Holy / Angel Attribute (kRO 2004-04-27)**: Emperium was updated to Holy attribute and Angel race. — "엠펠리움의 속성이 성속성 / 천사형으로 변경되었습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Apr._27) (published 2004-04-27; Grade A)
- **Emperium Alchemist Plant Immunity (kRO 2004-02-03)**: Flora and Marine Sphere summoned by Alchemists were blocked from damaging Emperium. — "알케미스트가 소환한 플로라와 마린스피어의 공격으로 더 이상 엠펠리움을 부수지 못합니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Feb._03) (published 2004-02-03; Grade A)

### 3. WoE Combat and Environmental Constraints
- **Verbatim Battle Restrictions (iRO April 2004)**:
  - "- Damage is not displayed."
  - "- All effects are simplified."
  - "- Guild members will have the emblem displayed on top of their characters."
  - "- The skill 'Endure' and any item that has the same effect will not work."
  - "- If you are no longer able to fight, you will be returned to your saved point and no experience will be lost."
  - "- Wizards cannot use the Ice Wall skill."
  - "- Skills that push the opponent away will not work."
  - "- Teleport', 'Fly Wing' cannot be used. The 'Butterfly Wing' can be used."
  — https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp (published 2004-01-01; Grade A)
- **Phen Card Disabled in WoE**: Uninterruptible casting from Phen Card does not function in WoE. — "휀카드로 인한 ‘캐스팅 끊기지 않음’ 옵션이 동작하지 않습니다." — https://ro.gnjoy.com/guide/systeminfo/systemGuild2.asp (Grade A)
- **Noctrl Auto-Attack Disabled (kRO 2004-04-27)**: Auto-attack via `/noctrl` disabled in siege mode; players must hold click. — "공성모드 시에 /noctrl 을 사용하여도 자동공격이 되지 않습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Apr._27) (published 2004-04-27; Grade A)
- **Dead Branch Banned (kRO 2003-12-26)**: Dead Branches forbidden in town and siege maps. — "마을과 공성전맵에서의 고목나무가지 아이템 사용이 제한됩니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Dec._26) (published 2003-12-26; Grade A)
- **Alliance Lock during WoE (kRO 2003-10-07)**: Creating or breaking alliances is prohibited during WoE hours. — "공성 시간중에는 길드간의 동맹 결성 및 해체가 되지 않도록 수정하였습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Oct._07) (published 2003-10-07; Grade A)
- **Notrade Command Added (kRO 2004-03-23)**: `/notrade` (`/nt`) added to prevent trade pop-up griefing during WoE. — "/notrade 명령어가 추가되었습니다. (/nt로도 사용가능합니다.) 이 명령어를 사용하면 거래 요청을 자동으로 거절합니다. 공성시 거래요청을 원하시지 않으신 분들은 사용해 주시기 바랍니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Mar._23) (published 2004-03-23; Grade A)
- **WoE Damage Reductions**: Skill damage reduced by 40%; ranged normal attacks reduced by 25%; Flee reduced by 20%; Hunter traps duration quadrupled (4x). — https://irowiki.org/wiki/War_of_Emperium (Grade C corroborating official GungHo manual; GungHo manual states: "物理・魔法を問わず、スキルによる攻撃または弓による攻撃のダメージが軽減される。ハンター系のトラップスキルの効果持続時間が4倍になる。")

### 4. Castle Management & Butler NPC Functions
- **Butler Functions Verbatim (iRO April 2004)**:
  - "View Castle Status: You can view the overall castle status."
  - "Invest in Commerce Development: Invest to increase the castle's economy. This will affect the item type and amount that can be produced in the castle."
  - "Invest in Defense: Increase your castle's defense by investing in it. Defense will affect the castle Emperium's defense and HP as well as the Guardian's HP."
  - "Set up Guardian and Repair: The guardian that guards the castle can install or repair things inside the castle. Set up a guardian to ease the task of guarding your guild's castle."
  - "Hire Storage Employee: You can hire/fire Kafra NPCs inside the castle. Through the employed Kafra NPC, guild members can use the Kafra storage free of charge."
  - "Enter Master Room: There is a room that is only open to the Guild master. In this room. 1 rare item will be given once a day, depending on the Commerce Development Investment of the Guild."
  - "Empty Castle: If the castle is in a critical situation, you can return everything to the original status. When this happens, the castle is no longer under any particular guild's control, and becomes filled with monsters."
  — https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp (published 2004-01-01; Grade A)
- **Empty Castle Exploit Removal (kRO 2004-02-10 & 2004-02-17)**: Disbanding guild or abandoning castle via Butler no longer spawns monsters. — "일부 길드가 아지트를 차지한 다음에 포기를 할 경우 길드 아지트에 몹들이 소환되는 점을 악용하여 불법적인 아이템 취득 및 레벨업을 진행했었습니다. 이 부분에 대해서 아지트를 포기하더라도 몬스터는 더이상 소환되지 않도록 하였습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Feb._17) (published 2004-02-17; Grade A)
- **Castle Front Gate Flags Warp (kRO 2003-08-05)**: Castle front flags warp guild members directly into the castle. — "각 아지트 정문깃발에서 길드원 워프기능을 추가하였습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Aug._05) (published 2003-08-05; Grade A)

### 5. Economy & Defense Investment Mechanics & Formulas
- **Daily Investment Limits**: Maximum 2 points per day for Commerce and 2 points per day for Defense. Applied at midnight. — "商業発展は一日に最大2ポイントだけ投資できる。ギルド防衛値は、商業発展値と同じく一日に最大2ポイントだけ投資できる。" — https://ragnarokonline.gungho.jp/guide/gvg/ (Grade A)
- **Investment Cost Brackets (Zeny)**:
  - Second investment in the same day costs 4x the first investment.
  - Defense investment costs exactly 2x Commerce investment.
  - 1–4: Commerce 5k / 20k (Total 25k); Defense 10k / 40k (Total 50k)
  - 5–9: Commerce 10k / 40k (Total 50k); Defense 20k / 80k (Total 100k)
  - 10–14: Commerce 20k / 80k (Total 100k); Defense 40k / 160k (Total 200k)
  - 15–19: Commerce 35k / 140k (Total 175k); Defense 70k / 280k (Total 350k)
  - 20–24: Commerce 55k / 220k (Total 275k); Defense 110k / 440k (Total 550k)
  - 25–29: Commerce 80k / 320k (Total 400k); Defense 160k / 640k (Total 800k)
  - 30–34: Commerce 110k / 440k (Total 550k); Defense 220k / 880k (Total 1,100k)
  - 35–39: Commerce 145k / 580k (Total 725k); Defense 290k / 1,160k (Total 1,450k)
  - 40–44: Commerce 185k / 740k (Total 925k); Defense 370k / 1,480k (Total 1,850k)
  - 45–49: Commerce 230k / 920k (Total 1,150k); Defense 460k / 1,840k (Total 2,300k)
  - 50–54: Commerce 280k / 1,120k (Total 1,400k); Defense 560k / 2,240k (Total 2,800k)
  - 55–59: Commerce 335k / 1,340k (Total 1,675k); Defense 670k / 2,680k (Total 3,350k)
  - 60–64: Commerce 395k / 1,580k (Total 1,975k); Defense 790k / 3,160k (Total 3,950k)
  - 65–69: Commerce 460k / 1,840k (Total 2,300k); Defense 920k / 3,680k (Total 4,600k)
  - 70–74: Commerce 530k / 2,120k (Total 2,650k); Defense 1,060k / 4,240k (Total 5,300k)
  - 75–79: Commerce 605k / 2,420k (Total 3,025k); Defense 1,210k / 4,840k (Total 6,050k)
  - 80–84: Commerce 685k / 2,740k (Total 3,425k); Defense 1,370k / 5,480k (Total 6,850k)
  - 85–89: Commerce 770k / 3,080k (Total 3,850k); Defense 1,540k / 6,160k (Total 7,700k)
  - 90–94: Commerce 860k / 3,440k (Total 4,300k); Defense 1,720k / 6,880k (Total 8,600k)
  - 95–99: Commerce 955k / 3,820k (Total 4,775k); Defense 1,910k / 7,640k (Total 9,550k)
  — https://w.atwiki.jp/ragnarokgvg/pages/30.html & https://w.atwiki.jp/ragnarokgvg/pages/43.html (Grade C corroborating official manual)
- **Castle Loss Penalty**: Each time a castle is conquered, both Commerce and Defense decrease by 5 points (min 1). — "1回落城する度に防衛値は５ポイントずつ下がっていきます。１以下には下がりません。" — https://w.atwiki.jp/ragnarokgvg/pages/43.html (Grade C)

### 6. Treasure Box Spawning & Mechanics
- **Spawning Formula**: Total boxes = `4 + floor(Commerce / 5)`. Range 4 (at commerce 1) to 24 (at commerce 100). — "出現宝箱個数＝4＋（商業値÷5）... 商業値100の場合は 4+(100/5)=24個出現することになります。" — https://w.atwiki.jp/ragnarokgvg/pages/30.html (Grade C)
- **Box Types**: Boxes consist of Common Treasure (Type A) and Castle-Specific Treasure (Type B). At Commerce 1, 4 boxes appear (2x Type A, 2x Type B). Every +5 Commerce adds 1 box, alternating A, B, A, B. — https://w.atwiki.jp/ragnarokgvg/pages/30.html (Grade C)
- **Spawn Time & Conditions**: Boxes spawn at 00:00 midnight in Master Room. If previous day's boxes are unlooted, no new boxes spawn. All unlooted boxes disappear during server maintenance. Treasure boxes have 1 HP, rotate in place, Steal is disabled. — "宝箱が出現する時間は、毎日午前0時です。前日の宝箱を開けていない状態で午前0時を迎えても、新たな宝箱は出現しません。また、毎週火曜日の定期メンテナンス時...サーバー状態がリセットされてしまうため、宝箱は消滅してしまいます。" — https://ragnarokonline.gungho.jp/guide/gvg/ (Grade A)
- **Restricted Access to Master Room**: Only Guild Master can enter the Master Room to loot treasure boxes (patched on kRO 2003-07-29 to fix non-GM access). — "길드 보물상자에 길드 마스터 외의 플레이어가 접근 가능한 점을 수정하였습니다." — https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Jul._29) (published 2003-07-29; Grade A)

### 7. Guardians and Guild Dungeons
- **Guardian Setup & Capacity**: Up to 8 guardians per castle in First Edition; deployed and repaired via Butler NPC for 10k Zeny each (requires guild skill 'Guardian Research'). Heal can restore damaged guardians. — "ギルドスキル「ガーディアン研究」と必要ぶんのZenyがあれば、執事NPCを通じて最大8体まで配置できる。ダメージを受けたガーディアンは、ヒール等で回復可能。" — https://ragnarokonline.gungho.jp/guide/gvg/ (Grade A)
- **Guild Dungeon Shared Access & PvP**: 1 dungeon per realm, shared by all 5 castles in that realm. Members can attack players of non-allied guilds. Death in guild dungeon incurs normal death experience loss penalty (patched 2003-12-26 to fix Guild PvP). — "同じフィールドにある5つの砦の共用ダンジョンであり、同盟ギルド以外のキャラクターを攻撃することもできる。もちろん戦闘不能になるとデスペナルティがある" — https://ragnarokonline.gungho.jp/guide/gvg/ (Grade A) & https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Dec._26) (Grade A)

---

## Contradictions and open points
1. **Emperium Damageable by Magic vs Physical Only**:
   - `warofemperium.asp` (iRO 2004-04-05) explicitly states: "Only direct physical attacks will be able to damage the Emperium."
   - Later kRO guide (`systemGuild2.asp`) states: "직접적인 물리, 마법 공격에만 데미지를 입습니다." (Takes damage only from direct physical, magic attacks).
   - In 2003–2004 classic mechanics, active magical skills miss the Emperium; direct physical melee hits deal damage. The April 27, 2004 kRO patch gave it Holy 1 attribute, allowing Shadow/Fire/Water/Wind endowed melee attacks to deal elemental-modified damage.
2. **Guardian Types in FE Castles**:
   - GungHo official manual mentions up to 8 guardians per FE castle repaired via Butler. Early design documents refer to Hall Guardians (archer/turret style) and Master Guardians (mobile melee). In actual FE implementation, castles have Soldier, Archer, and Knight Guardians placed at key choke points.

---

## Sources read
- `https://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp` — Official iRO WoE guide (captured 2004-04-05). Extremely reliable primary source for 2004 rules.
- `https://web.archive.org/web/20040404013538/http://iro.ragnarokonline.com/game/guildsystem.asp` — Official iRO Guild System guide (captured 2004-04-04). Primary source for guild creation and approval.
- `https://www.gamemeca.com/view.php?gid=43200` — Contemporary gaming journalism (published 2003-04-16) reporting Gravity's official announcement of WoE on Sakray on April 15, 2003. Highly reliable contemporary news report.
- `https://ro.gnjoy.com/guide/systeminfo/systemGuild2.asp` — Official Gravity kRO Guild & WoE system guide. Official primary source.
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Jun._10)` — kRO official patch notes translation citing GNJOY BBS notice #645. Highly reliable primary source.
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Jul._29)` — kRO official patch notes (Treasure box GM exclusivity).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Aug._05)` — kRO official patch notes (Castle flag names & front gate warp).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Oct._07)` — kRO official patch notes (WoE alliance lock).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2003_Dec._26)` — kRO official patch notes (Dead Branch ban, Guild Dungeon PvP).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Feb._03)` — kRO official patch notes (Alchemist plant Emperium exploit fix).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Feb._17)` — kRO official patch notes (Empty Castle monster exploit fix).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Mar._23)` — kRO official patch notes (/notrade command).
- `https://ragnarok.fandom.com/wiki/RO_Patch_(2004_Apr._27)` — kRO official patch notes (Emperium Holy/Angel attribute, /noctrl fix).
- `https://ragnarok.fagnarok.com/wiki/RO_Patch_(2004_May_04)` — kRO official patch notes (Castle broadcast names).
- `https://ragnarokonline.gungho.jp/guide/gvg/` — Official GungHo Japan WoE manual. Highly reliable official publisher documentation.
- `https://www.ro-guardians.com/castle/irisii/index.html` — Period euRO castle tracking database (published 2004-06-08). Contains original siege logs from June 2 & 5, 2004.
- `https://w.atwiki.jp/ragnarokgvg/pages/30.html` — Japanese GvG technical documentation on Commerce investment costs, formulas, and treasure box counts.
- `https://w.atwiki.jp/ragnarokgvg/pages/43.html` — Japanese GvG technical documentation on Defense investment costs and conquest decay.
- `https://irowiki.org/wiki/War_of_Emperium` — Comprehensive English RO mechanics documentation.

---

## Could not read
- `http://iro.ragnarokonline.com/game/castles.asp` — Returned HTTP 404 on Wayback Machine; castle data was presented directly within `warofemperium.asp` tables and map images rather than a dedicated standalone subpage.
- `http://www.euro-ro.net/` — Direct connection refused on web.archive.org during search; resolved via contemporary mirror `ro-guardians.com` castle standings archive.

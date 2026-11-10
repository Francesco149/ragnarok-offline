# Research Notes: Thread #4 (Kafra Points/Pass) & Thread #7 (1st-Class Quest Skills) in Dec 2003 – Jan 2004

Researched: 2026-11-09
Focus era: 2003-07 to 2004-06 (kRO, iRO, jRO, euRO)

---

## 1. Thread #4: Kafra Points and Kafra Pass in Dec 2003 – Jan 2004

### Findings

- **[READ] (kRO, 2003-05-20)** kRO official patch notes explicitly record the removal of the Kafra voucher/pass menu from all servers on 2003-05-20:
  Sakray:
  `":카프라이용권 메뉴가 사라졌습니다."`
  Main server (본 서버):
  `"- 카프라이용권 메뉴가 사라졌습니다."`
  Translation: "The menu for the Kafra voucher has disappeared."
  URL: `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_May_20)&prop=wikitext&format=json` quoting original official notice `http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=601&curpage=620` (published 2003-05-20; high).

- **[READ] (jRO/iRO/kRO/euRO client database, Item ID 1084)** The item `Kapra's_Pass` (ID #1084) existed in client files from beta 1/2.
  In the Japanese client (`jRO`), the item description explicitly states that it is inactive:
  `"カプラの全てのサービスを無料で利用することができる。「１人１枚に限ります。」と書いてある。現在は利用できない。"`
  Translation: "Allows free use of all Kafra services. It is written 'Limited to 1 sheet per person.' Currently cannot be used."
  In the European/international client (`ropEU` / `iRO`):
  `"A special, one-time use ticket that allows you to use any one of the Kafra Services free of charge. Weight: 1"`
  In the Korean client (`kRO`):
  `"카프라의 모든 서비스를 무료로 이용 할 수 있다. '1인 1매에 한합니다.'라 적혀 있다. 무게 : 1"`
  URL: `https://www.divine-pride.net/database/item/1084/kafra-pass` (published 2026-01-01 / classic Gravity strings; high).

- **[READ] (eAthena 817, 2004-02-19)** In 817's `other/kafra.txt` (imported as `scripts/import/npc/other/kafra.lua`), the Kafra Pass menu entry was completely unfinished:
  `::Lpass::`
  `if count_item(1084) < 1 then goto NeedKafraPass end`
  `::NeedKafraPass::`
  `mes "[Kafra]"`
  `mes "Dear you need at least 1 passport."`
  `close()`
  `do return end`
  `mes "[Kafra]"`
  `mes "Now you are Using a Kapra's Pass."`
  `take_item(1084, 1)`
  `next()`
  `mes "[Kafra]"`
  `mes "Under development"`
  `cutin("kafra_03", 255)`
  `close()`
  Similarly, under `Linfo` -> `Special Reverse Check`:
  `::ReserveCheck::`
  `mes "[Kafra]"`
  `mes "Under development"`
  URL: `/opt/src/ragnarok-offline/scripts/import/npc/other/kafra.lua` lines 1018–1042 (published 2004-02-19; high).

- **[READ] (eAthena 817 / rAthena pre-re)** In 817's `other/kafra.txt:995`, an emulator-authored script for a Kafra points exchange existed, but its menu labels began with digits (`1`, `2`...), which 817's parser evaluated as numbers, causing the menu to crash. Official rAthena (`npc/kafras/kafras.txt`) contains only Kafra Leilah at `aldeba_in,96,181` offering standard Save/Storage/Cart. No official Gravity script ever offered a "Sweet Potato" lottery in `aldeba_in` in Dec 2003 – Jan 2004.
  URL: `https://github.com/rathena/rathena/blob/master/npc/kafras/kafras.txt` (high).

- **[INFER] (euRO / iRO Dec 2003 status)** Neither Kafra Points exchange nor Kafra Pass purchases were operational in the Aldebaran Kafra HQ on euRO or iRO in Dec 2003 – Jan 2004. The voucher menu was removed on kRO on 2003-05-20 and never enabled on euRO. The 817 "Sweet Potato" exchange was a broken emulator artifact. The Aldebaran Kafra HQ exchange must remain **disabled** in `ragnarok-offline`.

---

## 2. Thread #7: 1st-Class Platinum / Quest Skills in Dec 2003 – Jan 2004

### Findings

- **[READ] (kRO, 2003-01-21)** The 1st-class quest skills already existed in kRO before January 2003. Official patch notes for 2003-01-21 rebalance `Energy Coat` and `Holy Light`:
  `":- 스킬 에너지코트의 효능이 변경 되었습니다."`
  `"::최대 대미지 감소량이 30% 가 되었으며 , 피격시 SP 소모량이 최대 3% 로 되었습니다."`
  `":- 홀리라이트의 캐스팅 시간이 1.5 초로 감소되었습니다."`
  Translation: "Skill Energy Coat's effect has been changed: maximum damage reduction became 30%, and SP consumption when hit became up to 3%. Holy Light's casting time has been reduced to 1.5 seconds."
  URL: `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Jan._21)&prop=wikitext&format=json` quoting official notice `http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=209&curpage=659` (published 2003-01-21; high).

- **[READ] (iRO official website, 2004-03-27)** Official iRO guide page `jobacoskill.asp` explicitly lists Holy Light as an active quest skill:
  `"Holy Light"`
  `"This skill requires Acolyte job level 30 and a quest. Attacks a target from a distance with a holy light. Consumes 15 SP. Has a 2 second casting delay. Deals 125% of the magic attack strengh."`
  URL: `https://web.archive.org/web/20040327034756/http://iro.ragnarokonline.com:80/game/jobacoskill.asp` (captured 2004-03-27; high).

- **[READ] (kRO official website `ro.gnjoy.com`)** Official Gravity quest library documents all 1st-class quest skills, exact NPCs, maps, and requirements:
  - **Acolyte — Holy Light (홀리라이트, QuestSeq 75):**
    `"/navi prt_church 176/22"`
    Req: Acolyte Job Level 30+. Items: 1 Opal, 1 Crystal Blue, 1 Rosary.
    NPC: Cleric / Head Acolyte (주임복사) in Prontera Church (`prt_church 176,22` / `173,23`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=8&QuestSeq=75&articleSeq=112` (published 2014-05-14 / legacy data; high).
  - **Novice — First Aid (응급치료, QuestSeq 77):**
    Req: Novice Job Level 4+ (5+ in renewal). Items: 3 Red Herbs, 3 Clovers, 1 Sterilized Bandage.
    NPC: Nami inside Prontera inn (`prt_in 234,133` / west of central fountain). Sterilized bandage obtained from Nurse Instructor (간호장교) in Prontera Castle 2F (`prt_castle 175,35`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=8&QuestSeq=77&articleSeq=126` (published 2014-05-14; high).
  - **Novice — Play Dead / Trick Dead (죽은척하기, QuestSeq 78):**
    Req: Novice Job Level 7+, must have First Aid. Novice only.
    NPC: Prontera Chivalry Member (프론테라 기사단원) in Prontera Chivalry (`prt_in 73,87` / 7 o'clock). Directs player to fetch Novice Card (초보자 명함) from Nurse Instructor in Prontera Castle 2F, then returns to teach skill.
    URL: `https://ro.gnjoy.com/guide/runemidgarts/QuestView.asp?runSearch=%EA%B2%8C%ED%8E%9C&curpage=8&QuestSeq=78&ArticleSeq=128` (published 2014-05-14; high).
  - **Swordsman — Fatal Blow / Attack Weak Point (급소치기, QuestSeq 84):**
    Req: Swordman Job Level 25+ (30+ in later guides), Bash Lv 6+.
    Items: 10 Fire Arrows, 10 Silver Arrows, 1 Banana Juice, 30 Tentacles, 5 Royal Jelly.
    NPC: Leon von Frich (레온 폰 프리슈) inside Prontera Chivalry (`prt_in 75,88`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=84` (published 2014-05-14; high).
  - **Swordsman — Auto Berserk (오토버서크, QuestSeq 85):**
    Req: Swordman Job Level 30+.
    Items: 35 Powder of Butterfly, 10 Honey, 10 Horrendous Mouth, 10 Decaying Nail.
    NPC: Juan (후안) inside Prontera upgrade shop (`prt_in 94,57` / `178,185`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=85` (published 2014-05-14; high).
  - **Swordsman — Moving HP Recovery (이동시 HP 회복):**
    Req: Swordman Job Level 35+.
    Items: 500 Empty Bottles (or 200), 1 Rocker Doll, 1 Padded Armor.
    NPC: Knight De Thomas / De Thomaso (기사 디토마소) in Izlude (`izlude_in 175,130` / `214,129`).
    Gravity client description: `"위성도시 이즈루드의 '기사 디토마소'에게 배울 수 있다."`
    URL: `https://www.divine-pride.net/database/skill/144/moving-hp-recovery` (high).
  - **Mage — Energy Coat (에너지코트, QuestSeq 76):**
    Req: Mage Job Level 35+.
    Items: 3 Glass Beads, 1 1-Carat Diamond, 5 Shells, 1 Solid Shell (Hard Shell).
    NPC: Great Wizard inside Geffen Magician Guild (`geffen_in 151,119`).
    URL: `http://www.geocities.ws/encieluna/roskill.htm` (published 2004; high).
  - **Archer — Arrow Crafting (화살 만들기, QuestSeq 86):**
    Req: Archer Job Level 30+ (or 35+).
    Items: 20 Resin, 1 Red Potion, 13 Trunks, 41 Tough Scalelike Stem (Pointed Scale), 7 Mushroom Spores.
    NPC: Roberto / Kyolnari (결나리) in Morroc (`morocc 121,109`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=86` (published 2014-05-14; high).
  - **Archer — Charge Arrow / Arrow Repel (챠지 애로우, QuestSeq 87):**
    Req: Archer Job Level 30+ (or 35+).
    Items: 2 Emeralds, 3 Yoyo Tails, 10 Tentacles, 10 Bill of Birds, 36 Banana Juice.
    NPC: Jason / Gaeho (개호) / Deothen in Payon (`payon 104,62` or `30,94`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=87` (published 2018-03-07; high).
  - **Thief — Back Slide, Pick Stone, Throw Stone, Sand Attack (QuestSeq 79, 80, 81, 82):**
    All 4 Thief skills are officially given by ONE NPC: **Alcouskou** in the Thief Guild in Morroc Pyramids basement (`moc_prydb1 154, 128`):
    - Back Slide (TF_BACKSLIDING): Job 35+, 20 Grasshopper's Legs.
    - Pick Stone / Find Stone (TF_PICKSTONE): Job 20+, 1 Bear's Footskin, 1 Zargon, 5 Spawn.
    - Throw Stone / Stone Fling (TF_THROWSTONE): Job 15+, 2 Garlet, 2 Scell.
    - Sand Attack / Sprinkle Sand (TF_SPRINKLESAND): Job 25+, 5 Fine Grit, 1 Leather Bag of Infinity (crafted by Bag Seller RuRumuni in Payon at `payon 91,77` for 1 Cobweb, 1 Cactus Needle, 1 Earthworm Peeling).
    URL: `https://ratemyserver.net/quest_db.php?type=50000&qid=50014` (high).
  - **Merchant — Crazy Uproar / Loud Exclamation (고성방가, QuestSeq 88):**
    Req: Merchant Job Level 15+. Items: 7 Pearls, 50 Mushroom Spores, 1 Banana Juice.
    NPC: Necko / Brynolf (네코) in Alberta (`alberta 83,96`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=88&articleSeq=137` (published 2014-05-14; high).
  - **Merchant — Change Cart (카트 체인지, QuestSeq 89):**
    Req: Merchant Job Level 30+. Items: 50 Trunks, 20 Animal Skins, 10 Iron.
    NPC: Charlron / CharIron (샤일론) in Alberta (`alberta 119,221`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=89&articleSeq=138` (published 2014-05-14; high).
  - **Merchant — Cart Revolution (카트 레볼루션, QuestSeq 90):**
    Req: Merchant Job Level 35+. Items: 20 Iron, 30 Sticky Mucus, 20 Fly Wings, 5 Tentacles, 2 Grape Juice (and 1 Banana Juice).
    NPC: Gershaun (거상스) in Alberta (`alberta 232,106`).
    URL: `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=90&articleSeq=139` (published 2014-05-14; high).

- **[READ] (817 Thief quest coordinates bug)** In eAthena 817:
  - In `scripts/import/npc/skillquests/throw_stone.lua` (from 817 `skillquests/throwstone.txt`), 817 created a fake NPC named `"Sono-san"` at `morocc, 165, 124`.
  - In `skillquests/backslide.txt` and `skillquests/pickstone.txt`, 817 placed the exact same coordinates `morocc 165, 124`, but the line had leading spaces instead of a tab, which caused 817's parser to skip the header entirely. As a result, neither `back_slide.lua` nor `pick_stone.lua` contained any NPCs!
  - `morocc 165,124` is completely an **emulator bug / fan invention**. Official servers never had Sono-san at `morocc 165,124`. The official NPC was always **Alcouskou** at `moc_prydb1 154,128`.

---

## 3. Contradictions and Open Points

1. **Job Level Prerequisite variations**:
   - Fatal Blow: 817 and classic guides list Job Level 25+ and Bash Lv 5; later renewal gnjoy guides list Job Level 30+ and Bash Lv 6. For Dec 2003, Job Level 25+ / Bash 5 is period accurate.
   - Charge Arrow: 817 lists Job Level 35+; classic guides list Job Level 30+ or 25+.
   - Moving HP Recovery: 817 text quotes 500 Empty Bottles, but takes 200 bottles. Classic guides list 500 Empty Bottles, 1 Rocker Doll, 1 Padded Armor.
2. **Kafra HQ**:
   - 817 contained a broken, non-functional lottery menu at `aldeba_in:995`. Neither official iRO nor euRO had this exchange active in Dec 2003.

---

## 4. Sources Read

1. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_May_20)&prop=wikitext&format=json` — Official kRO patch notes for 2003-05-20 (voucher menu removal). Highly reliable primary source quotation.
2. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Jan._21)&prop=wikitext&format=json` — Official kRO patch notes for 2003-01-21 (Energy Coat & Holy Light balance). Highly reliable primary source quotation.
3. `https://web.archive.org/web/20040327034756/http://iro.ragnarokonline.com:80/game/jobacoskill.asp` — Official iRO website capture from 2004-03-27 confirming Holy Light quest skill in iRO.
4. `https://ro.gnjoy.com/guide/runemidgarts/questlist.asp?curpage=8` and `curpage=9` — Official Gravity Korea quest library detailing all 1st-class quest skills, item requirements, and official NPC names.
5. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=8&QuestSeq=75&articleSeq=112` — Holy Light quest navigation `/navi prt_church 176/22` and items.
6. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=8&QuestSeq=77&articleSeq=126` — First Aid quest details (Nami, Nurse Instructor).
7. `https://ro.gnjoy.com/guide/runemidgarts/QuestView.asp?runSearch=%EA%B2%8C%ED%8E%9C&curpage=8&QuestSeq=78&ArticleSeq=128` — Play Dead quest details (Prontera Chivalry Member, Novice Card).
8. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=84` — Fatal Blow quest details (Leon von Frich in Chivalry).
9. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=85` — Auto Berserk quest details (Juan in refine shop).
10. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=86` — Arrow Crafting quest details (Roberto in Morroc).
11. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=87` — Charge Arrow quest details (Jason in Payon).
12. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=88&articleSeq=137` — Crazy Uproar quest details (Necko in Alberta).
13. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=89&articleSeq=138` — Change Cart quest details (Charlron in Alberta).
14. `https://ro.gnjoy.com/guide/runemidgarts/questview.asp?curpage=9&QuestSeq=90&articleSeq=139` — Cart Revolution quest details (Gershaun in Alberta).
15. `https://www.divine-pride.net/database/item/1084/kafra-pass` — Multilingual client database for Kafra Pass (Item 1084), confirming jRO note "Currently cannot be used".
16. `https://www.divine-pride.net/database/skill/144/moving-hp-recovery` — Client skill description for Moving HP Recovery, confirming Knight De Thomas in Izlude.
17. `http://www.geocities.ws/encieluna/roskill.htm` — Period guide (2004) covering complete 1st job quest skill prerequisites, items, and NPCs.
18. `https://ratemyserver.net/quest_db.php?type=50000&qid=50014` — Thief platinum skill quest guide verifying Alcouskou (`moc_prydb1 154,128`) and RuRumuni in Payon.
19. `/opt/src/ragnarok-offline/scripts/import/npc/other/kafra.lua` — eAthena 817 Kafra script (Lpass and ReserveCheck set to "Under development").
20. `/opt/src/ragnarok-offline/scripts/import/npc/skillquests/throw_stone.lua` — 817's Sono-san at `morocc, 165, 124`.

---

## 5. Could Not Read / Blocked

- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/30694` — Cloudflare 403 bot check. (Information verified through geocities, ro.gnjoy.com, and ratemyserver mirrors).
- `https://forums.warpportal.com/index.php?/topic/54945-kafra-reserve-points/` — HTTP 404 (thread deleted/moved).

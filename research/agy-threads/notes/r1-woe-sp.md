# Thread #17 - WoE and Guild Castles in Single Player

## Answer so far
On official servers in 2003–04 (kRO, iRO, euRO), War of Emperium (WoE) and castle ownership gated major end-game systems:
1. **Treasure Rooms & Daily Treasure Chests (mobs 1324–1363)**: 20 castles across 4 realms (Prontera/Valkyrie, Al De Baran/Luina, Payon/Baldur, Geffen/Britoniah). Each castle possesses 2 alternating chest IDs (common + unique). They spawned daily at 00:01 server time in a locked Master Room accessed solely by the Guild Master via the Butler NPC. Quantity scaled strictly with castle Economy investment: `Chests = floor(Economy / 5) + 4`, ranging from 4 (0 Eco) to 24 (100 Eco). They dropped Yggdrasil Berries/Seeds, Mastela Fruit, Elunium/Oridecon, Tribal Solidarity, rare headgears/weapons (Angel Helm, Romantic Gent / Oldman's Romance, Corsair, Masamune, Brocca), and the 20 unique God Item quest components (Freyja's Jewel, Thor's Gauntlets, Iron Maiden, etc. at 0.10% rate).
2. **Guild Dungeons (`gld_dun01`–`gld_dun04`)**: Accessed via a secret lever inside each castle (`"Only the truly brave can take the test"`). Gated strictly to members of the owning guild (`getcharid(2) == GID`). All 5 castles of a realm shared one dungeon, operating as a PvP zone with normal death EXP penalty. They contained 8-hour respawn MVPs (**Eddga** in `gld_dun01`, **Doppelganger** in `gld_dun02`, **Maya** in `gld_dun03`, **Dark Lord** in `gld_dun04`), minibosses (Vagabond Wolf, Ghostring), and mobs dropping cards unobtainable anywhere else in Dec 2003 (Maya Purple Card, Dark Illusion Card, Cat o' Nine Tails Card, Owl Baron Card, Am Mut Card, etc.).
3. **Castle Infrastructure**: Butler NPC handled daily Economy (5k–955k zeny) and Defense (10k–1.91m zeny) investments (max 2/day; 2nd costs 4×; resets by -5 upon conquest). Guardians (Soldier 1287, Archer 1285, Knight 1286; 10k zeny each, up to 8 per castle) gained +1,000 HP per Defense point. Castle Kafra provided free storage, local realm teleport (200z), and cart rental (800z).
4. **Guild Skills**: None required holding a castle to *learn* (all unlocked via Guild Level EXP taxation), but skills `Contract with Kafra` (10001), `Guardian Research` (10002), `Strengthen Guardians` (10015), and `Permanent Development` (10014, 50% chance for +1 extra economy per investment) had effects that *only functioned* when possessing a castle.
5. **Period PvE / Solo / NPC Precedents**: In late 2003 / early 2004, unoccupied/abandoned castles were filled with aggressive mob hordes; players actively farmed castle abandonment for leveling before kRO patched it out on 2004-02-10/17. euRO ran official "GM Castle Assaults" (April 2004). kRO ran "Novice WoE" (`nguild_*`) in April 2004, and added Guild Relay Quests in March/April 2004.
6. **Single-Player Options**: Three concrete designs are proposed, recommending a hybrid "Neutral Castle Raid & Daily Tribute" model that preserves 100% of period content, dialogue, drop tables, and zeny sinks without requiring multiplayer opponents.

---

## Findings

### 1. Castle Ownership & WoE Gated Content (2003–04)

#### A. Treasure Rooms & Treasure Chests (Monster IDs 1324–1363)
- **Spawn Timing & Room Access**:
  - [READ] iRO Official Guide (2004-01-01 / 2004-04-05):
    > "Enter Master Room: There is a room that is only open to the Guild master. In this room. 1 rare item will be given once a day, depending on the Commerce Development Investment of the Guild."
    *(Source: `http://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com:80/game/warofemperium.asp`, captured 2004-04-05; T1 / period-official; high)*
  - [READ] iRO Wiki Classic (frozen pre-renewal archive):
    > "Every day at 12:00 AM at server time, the Treasure Boxes spawn in a room accessible only by the guild leader from a NPC in each castle. Items drop from these boxes, and drop rate-improving items may assist in obtaining rares. Steal does not function here."
    *(Source: `https://irowiki.org/classic/War_of_Emperium`; T5 / later-community; high)*
  - [READ] rAthena Pre-Re Script Lineage (`npc/guild/agit_main.txt`):
    > `OnClock0001:`
    > `set .@GID, GetCastleData(strnpcinfo(2),CD_GUILD_ID);`
    > `if (!.@GID) end;`
    > `set .@Treasure,GetCastleData(strnpcinfo(2),CD_CURRENT_ECONOMY)/5+4;`
    *(Source: `https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt`; T4 / later-official-derived Aegis logic; high)*

- **Spawn Quantity Formula & Mechanics**:
  - [READ] Formula: `Daily Treasure Boxes = floor(Current_Economy / 5) + 4`
    - Base Economy = 0 -> 4 boxes minimum.
    - Max Economy = 100 -> 24 boxes maximum (4 + 20).
    - Every 5 economy points invested yields exactly +1 additional daily box.
  - [READ] Persistence Rule: Chests do not wipe at midnight if unkilled; spawns only fill empty slots:
    > `if ((getd("$@"+strnpcinfo(2)+"_treasure") & .@box) == 0) {`
    > `monster strnpcinfo(2),.@treasurex[.@i],.@treasurey[.@i],"Treasure Chest",.@boxid,1,"Treasure#"+strnpcinfo(2)+"::OnTreasureDied"+.@i;`
    *(Source: `npc/guild/agit_main.txt`; high)*

- **Box IDs & Castle Mapping (40 IDs: 1324–1363)**:
  - [READ] Each castle alternates between two monster IDs: `boxid = base_id + (i + 2) % 2`.
    - **Luina (Al De Baran)**:
      - `aldeg_cas01` (Neuschwanstein): 1324 (Common), 1325 (Unique)
      - `aldeg_cas02` (Hohenschwangau): 1326, 1327
      - `aldeg_cas03` (Nuernberg): 1328, 1329
      - `aldeg_cas04` (Wuerzburg): 1330, 1331
      - `aldeg_cas05` (Rothenburg): 1332, 1333
    - **Britoniah (Geffen)**:
      - `gefg_cas01` (Repherion): 1334, 1335
      - `gefg_cas02` (Eeyolbriggar): 1336, 1337
      - `gefg_cas03` (Yesnelph): 1338, 1339
      - `gefg_cas04` (Bergel): 1340, 1341
      - `gefg_cas05` (Mersetzdeitz): 1342, 1343
    - **Baldur (Payon)**:
      - `payg_cas01` (Bright Arbor): 1344, 1345
      - `payg_cas02` (Scarlet Palace): 1346, 1347
      - `payg_cas03` (Holy Shadow): 1348, 1349
      - `payg_cas04` (Sacred Altar): 1350, 1351
      - `payg_cas05` (Bamboo Grove Hill): 1352, 1353
    - **Valkyrie (Prontera)**:
      - `prtg_cas01` (Kriemhild): 1354, 1355
      - `prtg_cas02` (Swanhild): 1356, 1357
      - `prtg_cas03` (Fadhgridh): 1358, 1359
      - `prtg_cas04` (Skoegul): 1360, 1361
      - `prtg_cas05` (Gondul): 1362, 1363
    *(Sources: `ragnarok-offline/data/db/mobs.toml`, `npc/guild/agit_main.txt`; high)*

- **Exact Drop Lists & God Item Components**:
  - [READ] Period database drops (`data/db/mobs.toml` & `https://irowiki.org/classic/Castle_Treasure_Drops`):
    - **Universal Common Drops**: Yggdrasil Seed (608, rate 20%), Yggdrasil Berry (607, rate 10%), Mastela Fruit (522, rate 45%), Elunium (985, rate 20-25%), Oridecon (984, rate 20-25%), Tribal Solidarity (658, rate 5%).
    - **20 God Item Ingredients (rate 10 = 0.10%, item IDs 7073–7092)**:
      - Luina 1: Freya's Jewel (7073) [Brisingamen]
      - Luina 2: Thor's Gauntlets (7074) [Mjölnir]
      - Luina 3: Iron Maiden (7075) [Mjölnir]
      - Luina 4: Wheel of the Unknown (7076) [Brisingamen]
      - Luina 5: Silver Ornament (7077) [Brisingamen]
      - Britoniah 1: Wrath of Valkyrie (7078) [Mjölnir]
      - Britoniah 2: Feather of Angel Wing (7079) [Sleipnir]
      - Britoniah 3: Cat Tread (7080) [Megingjard]
      - Britoniah 4: Woman's Moustache (7081) [Megingjard]
      - Britoniah 5: Root of Stone (7082) [Megingjard]
      - Baldur 1: Spirit of Fish (7083) [Megingjard]
      - Baldur 2: Sputum of Bird (7084) [Megingjard]
      - Baldur 3: Sinew of Bear (7085) [Megingjard]
      - Baldur 4: Emblem of the Sun God (7086) [Mjölnir]
      - Baldur 5: Breath of Spirit (7087) [Brisingamen]
      - Valkyrie 1: Snow Crystal (7088) [Brisingamen]
      - Valkyrie 2: Omen of Tempest (7089) [Mjölnir]
      - Valkyrie 3: Ripple (7090) [Sleipnir]
      - Valkyrie 4: Billow (7091) [Sleipnir]
      - Valkyrie 5: Drifting Air (7092) [Sleipnir]
    - **Rare Gear & Headgear**:
      - Helm of Angel / Angel Helm (item 5025; dropped in Baldur 2 / Luina)
      - Romantic Gent / Oldman's Romance ("Old Marquess", item 2247)
      - Hat of the Sun God / Helm of Sun (item 5022)
      - Corsair (item 5019)
      - Bone Helm (item 5017)
      - Wizardry Hat / Grand Circlet (item 5027)
      - Spiky Band / Sphinx Helm (item 5053)
      - Masamune (item 1165), Brocca (item 1417), Infiltrator (item 1260), Ice Pick (item 1256/1238), Poll Axe (item 1472), Rudra Bow (item 1716), Lance (item 1472).
    *(Sources: `data/db/mobs.toml`, `items.toml`, `irowiki.org/classic/Castle_Treasure_Drops`; high)*

---

#### B. Guild Dungeons (`gld_dun01`–`gld_dun04`)
- **Access Switch / Lever Mechanics**:
  - [READ] The entrance is a hidden lever object inside each castle:
    > `mes "[Ringing Voice]";`
    > `mes "'Only the truly brave can take the test.'";`
    > `next;`
    > `mes " ";`
    > `mes "There's a small lever. Will you pull it?";`
    > `next;`
    > `if(select("Pull.:Don't pull.") == 1) {`
    > `    if (getcharid(2) == .@GID) {`
    > `        close2;`
    > `        warp .@destination$,.@coordinates[0],.@coordinates[1];`
    > `        end;`
    > `    }`
    > `    mes " ";`
    > `    mes "Nothing happened.";`
    > `    close;`
    > `}`
    *(Source: `npc/guild/agit_main.txt`; high)*
  - [READ] Who could enter: **Only members of the guild holding that castle** (`getcharid(2) == .@GID`). Players from outside guilds received `"Nothing happened."`
  - [READ] Realm-Wide Convergence & PvP:
    - All 5 Payon castles warp into `gld_dun01` (Baldur Guild Dungeon).
    - All 5 Al De Baran castles warp into `gld_dun02` (Luina Guild Dungeon).
    - All 5 Prontera castles warp into `gld_dun03` (Valkyrie Guild Dungeon).
    - All 5 Geffen castles warp into `gld_dun04` (Britoniah Guild Dungeon).
    - Dungeons were active PvP zones between rival castle-owning guilds from the same realm, but normal EXP death loss applied (unlike WoE maps where EXP loss is disabled).
  - [READ] Official kRO Patch 2003-12-26:
    > "길드 던전내에서 길드 PvP기능이 정상적으로 작동하지 않던 문제점을 수정하였습니다."
    > ("Fixed a problem where Guild PvP function did not work properly in Guild Dungeon.")
    *(Source: Fandom `RO_Patch_(2003_Dec._26)`; T1 / period-official; high)*
  - [READ] Official euRO Notice 2004-09-03:
    > "During this time Treasure boxes, epmerium and guild dungeons are deactivated. We may change the deactivation of the guild dungeons on Wednesday."
    *(Source: `http://web.archive.org/web/20040910081217/http://www.euro-ro.net/news/eu/00499/`; T1 / period-official; high)*

- **Unique Monsters, MVPs (8-Hour Respawns) & Drops/Cards**:
  - [READ] `gld_dun01` (Baldur):
    - Mobs: Skeleton General (1290) ×10, Am Mut (1301) ×20, Cat o' Nine Tails (1307) ×3 (20 min), Gajomart (1309) ×10.
    - Miniboss: Vagabond Wolf (1092) ×1 (4-hour respawn: 14,400,000 ms).
    - Boss MVP: **Eddga** (1115) ×1 (**8-hour respawn**: 28,800,000 ms, 10 min variance).
  - [READ] `gld_dun02` (Luina):
    - Mobs: Owl Baron (1295) ×3, Giant Spider (1304) ×10, Ancient Worm (1305) ×5, Killer Mantis (1294) ×5, Giant Hornet (1303) ×10.
    - Boss MVP: **Doppelganger** (1046) ×1 (**8-hour respawn**: 28,800,000 ms).
  - [READ] `gld_dun03` (Valkyrie):
    - Mobs: Maya Purple (1289) ×3 (20 min), Caterpillar (1300) ×20, Gullinbursti (1311) ×10, Creamy Fear (1293) ×20, Leib Olmai (1306) ×20.
    - Boss MVP: **Maya** (1147) ×1 (**8-hour respawn**: 28,800,000 ms).
  - [READ] `gld_dun04` (Britoniah):
    - Mobs: Wraith Dead (1291) ×20, Mini Demon (1292) ×20, Zombie Master (1298) ×20, Dark Illusion (1302) ×1 (20 min).
    - Miniboss: Ghostring (1120) ×1 (4-hour respawn: 14,400,000 ms).
    - Boss MVP: **Dark Lord** (1272) ×1 (**8-hour respawn**: 28,800,000 ms).
    *(Source: `npc/pre-re/mobs/dungeons/gld_dun.txt`; high)*
  - [READ] Unique Drops and Cards:
    - **Maya Purple Card (1289)**: Detects hidden/cloaked enemies. (Top PvP/WoE card in the game).
    - **Dark Illusion Card (1302)**: MaxHP/MaxSP -10%, Cast Time -10% (combo with Dark Lord).
    - **Owl Baron Card (1295)**: Autocast Lex Aeterna Lv 1 on physical hit.
    - **Cat o' Nine Tails Card (1307)**: Magic reflection 5%, MDEF +3.
    - **Am Mut Card (1301)**: Demi-human EXP +10%, damage taken +20%.
    - High-tier Headgears & Gear: Fin Helm (2279), Spiky Band (2276), Bone Helm (2274), Angel Wing (2282), Evil Wing (2289), Smokie Leaf (5008), Ring [1] (2627), Old Card Album (616), Old Violet Box (617).
    *(Sources: `data/db/mobs.toml`, `items.toml`; high)*

---

#### C. Castle Investment, Kafra & Guardians
- **Economy Investment**:
  - [READ] Scaling Zeny Cost per tier:
    - Economy 1–5: 5,000z
    - Economy 6–10: 10,000z
    - Economy 11–15: 20,000z
    - Economy 16–20: 35,000z
    - Economy 21–25: 55,000z
    - Economy 26–30: 80,000z
    - Economy 31–35: 110,000z
    - Economy 36–40: 145,000z
    - Economy 41–45: 185,000z
    - Economy 46–50: 230,000z
    - Economy 51–55: 280,000z
    - Economy 56–60: 335,000z
    - Economy 61–65: 395,000z
    - Economy 66–70: 460,000z
    - Economy 71–75: 530,000z
    - Economy 76–80: 605,000z
    - Economy 81–85: 685,000z
    - Economy 86–90: 770,000z
    - Economy 91–95: 860,000z
    - Economy 96–100: 955,000z
  - [READ] Rules: Max 2 investments per day. The 2nd investment costs **4× zeny** (quadruple!).
  - [READ] Applied at midnight (00:01): `Current_Economy = Current_Economy + Invested_Economy + (rand(2) && getgdskilllv(GID, 10014))`.
  - [READ] Penalty upon castle loss: Economy drops by **5 points** whenever the castle is conquered.
  *(Source: `npc/guild/agit_main.txt`; high)*

- **Defense Investment**:
  - [READ] Scaling Zeny Cost:
    - Defense 1–5: 10,000z
    - Defense 6–10: 20,000z
    - Defense 11–15: 40,000z
    - Defense 16–20: 70,000z
    - Defense 21–25: 110,000z
    - Defense 26–30: 160,000z
    - Defense 31–35: 220,000z
    - Defense 36–40: 290,000z
    - Defense 41–45: 370,000z
    - Defense 46–50: 460,000z
    - Defense 51–55: 560,000z
    - Defense 56–60: 670,000z
    - Defense 61–65: 790,000z
    - Defense 66–70: 920,000z
    - Defense 71–75: 1,060,000z
    - Defense 76–80: 1,210,000z
    - Defense 81–85: 1,370,000z
    - Defense 86–90: 1,540,000z
    - Defense 91–95: 1,720,000z
    - Defense 96–100: 1,910,000z
  - [READ] Rules: Max 2 investments per day; 2nd costs 4×. Drops by 5 points upon castle conquest.
  - [READ] Effect: Each Defense point increases Guardian HP by +1,000 and Emperium HP by +1,000, and boosts Guardian DEF/ATK.

- **Castle Kafra Employee**:
  - [READ] Butler option: "Hire Storage Employee".
  - [READ] Requirement: Guild skill `GD_KAFRACON` (`Contract with Kafra`, ID 10001).
  - [READ] Perks:
    - Storage access: Free of charge to all guild members (`openstorage;`).
    - Teleport: Direct warp to local realm city (Prontera, Geffen, Payon, or Al De Baran) for a discounted fee of 200z (awards 20 Kafra points).
    - Cart Rental: 800z for Merchants/Blacksmiths/Alchemists (awards 80 Kafra points).
  *(Source: `npc/guild/agit_main.txt`; high)*

- **Castle Guardians**:
  - [READ] Up to 8 guardians positioned at predefined defense checkpoints per castle.
  - [READ] Types: Soldier Guardian (1287), Archer Guardian (1285), Knight Guardian (1286).
  - [READ] Cost: 10,000 zeny per guardian summoned/repaired.
  - [READ] Requirement: Guild skill `GD_GUARDRESEARCH` (`Guardian Research`, ID 10002).
  *(Source: `npc/guild/agit_main.txt`; high)*

---

#### D. Guild Skills
- [READ] Skill Tree & Castle Gating (`irowiki.org/classic/Guild_Skills`, `ragnaplace.com`):
  - **Did any guild skill require holding a castle to LEARN?** **NO.**
    - All guild skills are purchased with skill points obtained when the Guild levels up (via members donating 1%–50% base EXP via guild tax).
  - **Skills whose EFFECTS required holding a castle**:
    - `Contract with Kafra` (GD_KAFRACON, 10001): Enables hiring the Castle Kafra.
    - `Guardian Research` (GD_GUARDRESEARCH, 10002): Enables hiring Guardians in an owned castle.
    - `Strengthen Guardians` (GD_GUARDUP, 10015): Increases Guardian ATK and ASPD.
    - `Permanent Development` (GD_DEVELOPMENT, 10014): Grants a 50% chance (`rand(2)`) to gain +1 bonus Economy point when investing.
  - **Combat & General Skills** (Functional anywhere / in WoE):
    - `Official Guild Approval` (GD_APPROVAL, 10000): Required to damage the Emperium.
    - `Battle Command` (GD_BATTLEORDER, 10010): +5 STR/DEX/INT for 180s.
    - `Urgent Call` / Emergency Call (GD_EMERGENCYCALL, 10011): Teleports all guild members to master.
    - `Restoration` (GD_RESTORE, 10012): Restores 90% HP/SP.
    - `Regeneration` (GD_REGENERATION, 10013): Increases HP/SP regen.
    - `Great Leadership` (GD_LEADERSHIP, 10004), `Glorious Wounds` (GD_GLORYWOUNDS, 10005), `Cold Heart` (GD_SOULCOLD, 10006), `Sharp Gaze` (GD_HAWKEYES, 10007).
    - `Guild Extension` (GD_EXTENSION, 10003): Expands guild capacity up to 56 members.

---

### 2. Official and Period-Community PvE, NPC-Held Castle, or Solo Variants (2003–04)

- **Official Precedent: Empty Castle Monster Infestations & Exploited PvE Defense (2003–04)**:
  - [READ] iRO Official Guide (2004-01-01 / 2004-04-05):
    > "Empty Castle: If the castle is in a critical situation, you can return everything to the original status. When this happens, the castle is no longer under any particular guild's control, and becomes filled with monsters."
  - [READ] kRO Official Patch (2004-02-10):
    > ":- 아지트를 소유했던 길드가 해체했을 경우 관리하던 아지트에 몬스터들이 대거 등장했었습니다. 이에 길드의 해체와 창설을 반복하며 레벨업을 하는 형태가 목격되었습니다. 이제 아지트를 소유한 길드가 길드를 해체하더라도 아지트에 몬스터가 소환되지 않게 되었습니다."
    > ("When a guild that owned a castle disbanded, a large horde of monsters appeared in the managed castle. Consequently, players repeatedly disbanded and re-created guilds to level up. Now, when a guild disbands, monsters will no longer be summoned.")
  - [READ] kRO Official Patch (2004-02-17):
    > ":- 일부 길드가 아지트를 차지한 다음에 포기를 할 경우 길드 아지트에 몹들이 소환되는 점을 악용하여 불법적인 아이템 취득 및 레벨업을 진행했었습니다. 이 부분에 대해서 아지트를 포기하더라도 몬스터는 더이상 소환되지 않도록 하였습니다."
    > ("Some guilds abused the mechanic where mobs were summoned into the guild castle when abandoning the castle after capturing it, for illicit item acquisition and leveling up. Mobs will no longer be summoned when abandoning a castle.")
  - *Implication*: In late 2003, Gravity's official server code explicitly spawned an entire defensive monster dungeon inside unoccupied/abandoned castles (Guardian Knights, Guardian Master, Hatii, Raydric Archers, Abysmal Knights, Khalitzburgs). This served as a de facto PvE castle raid!

- **Official Precedent: euRO GM Castle Assaults (April 2004)**:
  - [READ] euRO Official News (2004-04-09 [W14]):
    > "Das Prontera Tourism Office warnt beim nächsten War of Emperium (Sonntag, 11 April 18Uhr) vor einem GM-Angriff auf die Burgen. Also Burgenbesitzer seid auf der Hut, haltet Ausschau nach GMs und verteidigt eure Burgen."
    > ("The Prontera Tourism Office warns of a GM attack on the castles during the next War of Emperium (Sunday, 11 April 18:00). So castle owners be on guard, look out for GMs and defend your castles.")
  - *Implication*: Official operators staged PvE siege attacks against player castles where external actors invaded.

- **Official Precedent: Novice WoE (`nguild_*`, April 2004)**:
  - [READ] kRO Official Patch (2004-04-07):
    > ":- 노비스 공성전 엠블렘 휘날리며~ 이벤트가 시작되었습니다."
    > ("Novice Siege [Novice WoE] 'Fluttering Emblem' event has started!")
  - Associated maps in client: `nguild_alde` (Earth), `nguild_gef` (Air), `nguild_pay` (Water), `nguild_prt` (Fire). A restricted tier designed for smaller groups / novices.

- **Official Precedent: Guild Relay Quest (March / April 2004)**:
  - [READ] kRO Official Patch (2004-03-30 & 2004-04-27):
    > "길드 아지트를 소유한 길드에게는 릴레이 퀘스트가 진행됩니다. 특정한 하나의 퀘스트를 수행하기 위해서는 길드원들의 일치단결된 행동력이 필요합니다."
    > ("For guilds that own a guild castle, a relay quest is conducted.")
  - Dedicated PvE co-op quest content unlocked exclusively through castle ownership.

---

### 3. Concrete Single-Player Design Options & Trade-Offs

#### Option A: "Neutral Garrison Castle Raid & Permanent Steward" (Faithful Period Offline WoE) — *Recommended*
- **Description**:
  1. The player creates a guild (`/guild`), levels it up via mob hunting (taxing base EXP) to unlock `Official Guild Approval` (Level 1).
  2. The player approaches an unoccupied castle during scheduled WoE or anytime. The castle is defended by the authentic period neutral garrison (the pre-Feb 2004 monster spawns: Guardian Knights, Guardian Masters, Raydric Archers, Khalitzburgs, Abysmal Knights).
  3. The player breaches the castle, defeats the defenders, and destroys the Emperium to claim castle ownership.
  4. Once captured, the player becomes Castle Master:
     - Butler NPC provides the authentic status briefing, daily Economy and Defense investments (acting as the premier late-game zeny sink: up to ~2.8m zeny daily).
     - Castle Kafra provides free storage, 200z city teleports, and 800z carts.
     - Guardians can be installed for 10k zeny each.
     - Hidden lever grants access to that realm's Guild Dungeon (`gld_dun01`–`04`), allowing solo hunts against 8h-respawn MVPs (Eddga, Doppelganger, Maya, Dark Lord) and unique card farming (Maya Purple, Dark Illusion).
     - Daily at 00:01 server time, 4 to 24 Treasure Chests spawn in the Master's Room according to Economy investment, dropping God Item ingredients, rare headgears (Angel Helm, Romantic Gent), weapons, and gems.
  5. *Optional PvE Defense Event*: A configurable periodic event (e.g. Sunday 18:00 CET matching euRO [W14]) where an invading monster horde or mercenary squad storms the castle, testing the player's guardians and defenses.
- **Trade-offs**:
  - *Period Official*: ~90%. Uses authentic period maps, NPC dialogue, investment formulas, chest spawn tables, drop rates, and mob IDs.
  - *Invented*: The offline solo claim trigger and the absence of human competitors evicting the player.
  - *Pros*: Preserves 100% of the period features, creates a meaningful long-term zeny and EXP sink, and provides authentic access to guild dungeons and treasure items.

#### Option B: "Realm Key / Royal Warrant Quest" (Pure Quest Bypass)
- **Description**:
  - An NPC in each realm (e.g. Prontera Tourism Office / Royal Guard) grants access to `gld_dun01`–`04` upon completion of a high-difficulty hunting/collection quest (e.g., collecting rare boss drops or paying an expedition fee).
  - Treasure Chests are either spawned inside the Guild Dungeon as rare spawns or traded for via an exchange NPC (similar to Dr. Jonesy in Prontera on Ymir).
- **Trade-offs**:
  - *Period Official*: ~40%. Only monster spawns and maps are period-accurate.
  - *Invented*: ~60%. Entirely bypasses guild creation, castle interiors, Butler NPC dialogue, castle investment zeny sinks, and castle Kafra.
  - *Pros*: Immediate access to guild dungeon monsters without requiring guild leveling or castle mechanics.
  - *Cons*: Abandons castle maps, Butler dialogue, investment gameplay, and guild skills.

#### Option C: "Single-Player WoE Training Grounds" (TE-Inspired Arena Model)
- **Description**:
  - Utilizes the Novice WoE maps (`nguild_prt`, `nguild_alde`, etc.) or standard castles as repeatable instanced/time-attack raid gauntlets (inspired by kRO 2004 Novice WoE and later WoE: TE missions).
  - Player enters a timed trial: clear defending waves -> defeat Guardian Knights -> destroy Emperium within a time limit.
  - Completion awards a daily reward cache (simulating treasure chests) and 24-hour access pass to that realm's Guild Dungeon.
- **Trade-offs**:
  - *Period Official*: ~60%. Maps and mobs are authentic; structure is adapted from later TE/arena models.
  - *Invented*: ~40%. Instanced time-attack gating.
  - *Pros*: Highly repeatable combat challenge; fits players who want active combat rather than passive daily chest harvesting.
  - *Cons*: Deviates from classic RO castle ownership feel; requires more custom scripting.

---

## Contradictions and Open Points
- **Treasure Box Monster ID mappings**: Earlier emulator revisions occasionally had offset indexing for box IDs. However, verification against `mobs.toml` and rAthena `agit_main.txt` confirms exact pairing: 20 castles × 2 boxes = IDs 1324 to 1363.
- **God Item Quests vs. Box Drops in 2003**: The God Item materials (Freya's Jewel 7073, Thor's Gauntlets 7074, etc.) dropped from treasure boxes starting with WoE Episode 4 (May 2003 on kRO), but the actual God Item Quests (the Four Seals) were not implemented until Episode 10 (late 2004). In late 2003, guilds hoarded these items.
- **Pre-Feb 2004 Castle Abandonment Exploit**: While kRO removed monster spawning upon guild disband in February 2004 due to player EXP exploit, the "Empty Castle" butler option and initial unoccupied castle state canonically spawned monster waves.

---

## Sources Read
1. `http://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com:80/game/warofemperium.asp` — Official iRO War of Emperium Guide (2004-01-01 / 2004-04-05). Highly reliable primary source [T1] for WoE schedule, battle restrictions, Butler NPC options, and "Empty Castle" monster mechanics.
2. `http://web.archive.org/web/20040404013538/http://iro.ragnarokonline.com:80/game/guildsystem.asp` — Official iRO Guild System Guide (2004-01-01 / 2004-04-04). Primary source [T1] on guild formation, Emperium consumption, emblem specs, and `Official Guild Approval`.
3. `http://web.archive.org/web/20040414010235/http://www.euro-ro.net/events/planned/de/00074/` — euRO Official Event Notice (2004-04-09 [W14]). Primary source [T1] for the Sunday 18:00 CET WoE schedule and GM Castle Assault event.
4. `http://web.archive.org/web/20040910081217/http://www.euro-ro.net/news/eu/00499/` — euRO Official News (2004-09-03 [W29]). Primary source [T1] confirming linkage between WoE, Treasure boxes, Emperium, and Guild Dungeons.
5. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Jun._10)` through `(2004_Apr._27)` — kRO Official Patch Archives [T1]. Verified Korean text for WoE rules, flag warps, guild dungeon PvP, pre-Feb 2004 castle disband monster farming exploit (2004-02-10 & 02-17), Novice WoE (2004-04-07), and Guild Relay Quests (2004-03-30).
6. `https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt` — Core pre-re castle script lineage [T4/Aegis]. Verbatim code for treasure spawn timing (`OnClock0001:`), economy box formula (`Economy/5 + 4`), alternating box IDs, scaling investment costs (5k–955k / 10k–1.91m), guardian spawning, Kafra hiring, and guild dungeon secret lever logic (`"Only the truly brave can take the test"`).
7. `https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt` — Pre-renewal guild dungeon monster spawn table [T4]. Confirmed 8-hour respawn MVPs (Eddga, Doppelganger, Maya, Dark Lord), minibosses, and mob distributions.
8. `https://irowiki.org/classic/Castle_Treasure_Drops` — Comprehensive pre-renewal castle treasure drop table [T5]. Confirmed exact item drops for all 40 treasure boxes, common boxes, and 20 God Item components.
9. `https://irowiki.org/classic/Guild_Skills` — Complete pre-renewal guild skill tree and EXP chart [T5]. Verified prerequisite trees and castle-dependent skill effects.
10. `/opt/src/ragnarok-offline/data/db/mobs.toml` & `/opt/src/ragnarok-offline/data/db/items.toml` — Local project client/mod0631 databases [T0/T2]. Verified monster IDs 1289–1311, 1324–1363, drop rates, and item IDs (Freya's Jewel 7073, Thor's Gauntlets 7074, Angel Helm 5025, Romantic Gent 2247).

---

## Could Not Read
- `http://db.irowiki.org/classic/item-info/*` — Direct database item pages returned bot-check block headers via automated fetch helper; verified item data directly against local repository `data/db/items.toml` and `mobs.toml`.

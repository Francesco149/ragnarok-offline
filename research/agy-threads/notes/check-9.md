# Fact-Check: WoE Castle Treasure Chest Mechanics, God Item Drops, Guild Dungeon MVPs, and Single-Player Gating

> **Claim:** War of Emperium castles generated daily Treasure Chests at 00:01 in the Master Room scaling by floor(Economy / 5) + 4 (4 to 24 chests) dropping 20 God Item ingredients at 0.10%, and castle guild dungeons (gld_dun01..04) held 4 realm-exclusive MVPs (Eddga, Doppelganger, Maya, Dark Lord) on 8-hour respawn timers, gating content that requires single-player PvE raid or steward mechanisms to access.

## Verdict
**Partly confirmed** (accurate in formulas, quantities, timers, and offline gating; inaccurate/misleading regarding "realm-exclusive" MVP status, with drop rates differing between official Aegis servers at 0.10% and Athena emulators at 0.80%).

### Corrected Wording
War of Emperium castles generated daily Treasure Chests in the Master Room (accessible only to the Guild Master via the Butler NPC) scaling by `floor(Economy / 5) + 4` (yielding 4 to 24 chests for Economy 0–100), triggered at 00:01 server time in emulator scripts (`OnClock0001:`) and officially documented at 12:00 AM (00:00). The 20 castle-specific chests dropped 20 distinct Tier 1 God Item crafting ingredients at **0.10% on official Gravity Aegis servers** (configured at **0.80% in default eAthena/rAthena monster databases**). The four castle guild dungeons (`gld_dun01`..`04`) each featured one realm-specific MVP (**Eddga** in Balder, **Doppelganger** in Luina, **Maya** in Valkyrie, **Dark Lord** in Britoniah) with an **8-hour respawn timer** (+10 min variance); however, these MVPs were **not realm-exclusive**, as all four existed with regular spawns in standard open-world fields or public dungeons. Because castle ownership checks (`CD_GUILD_ID`) hard-gate access to both the Master Room and the guild dungeons, these features are inaccessible in a single-player offline environment without custom PvE conquest raids or steward dialogue adaptations.

---

## Findings

- **Treasure Chest generation time and Master Room access:**
  - *"OnClock0001: ... set .@GID, GetCastleData(strnpcinfo(2),CD_GUILD_ID); if (!.@GID) end; set .@Treasure,GetCastleData(strnpcinfo(2),CD_CURRENT_ECONOMY)/5+4;"* — [https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt](https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt) (high).
  - *"Enter Master Room: There is a room that is only open to the Guild master. In this room. 1 rare item will be given once a day, depending on the Commerce Development Investment of the Guild."* — [http://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp](http://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp) (published 2004-01-01 / 2004-04-05; high).
  - *"Every day at 12:00 AM at server time, the Treasure Boxes spawn in a room accessible only by the guild leader from a NPC in each castle."* — [https://irowiki.org/classic/War_of_Emperium](https://irowiki.org/classic/War_of_Emperium) (high).

- **Chest quantity scaling formula: `floor(Economy / 5) + 4` (4 to 24 chests):**
  - In rAthena integer arithmetic: `set .@Treasure,GetCastleData(strnpcinfo(2),CD_CURRENT_ECONOMY)/5+4;`. With economy bounded between 0 and 100, `0/5 + 4 = 4` minimum chests, and `100/5 + 4 = 24` maximum chests — [https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt](https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt) (high).
  - *"Invest to increase the castle's economy. Every 5 points on investment adds one new treasure box to the castle's master room. The type of treasure box alternates between the two available at that castle."* — [https://irowiki.org/classic/War_of_Emperium](https://irowiki.org/classic/War_of_Emperium) (high).

- **20 God Item Ingredients across 20 castles:**
  - Across the 20 FE castles, 20 unique chests (`TREASURE_BOX2`, `4`, `6`, ..., `40`, mob IDs 1325, 1327, ..., 1363) each drop exactly one specific God Item ingredient for the Tier 1 God Items (Sleipnir, Megingjard, Brisingamen, Mjolnir):
    1. Luina 1 (`aldeg_cas01` / Box 2 / ID 1325): Jewel of Prayer (`Jewel_Of_Prayer` / `Freya's Jewel`, ID 7073)
    2. Luina 2 (`aldeg_cas02` / Box 4 / ID 1327): Thor's Gauntlets (`Iron_Glove`, ID 7074)
    3. Luina 3 (`aldeg_cas03` / Box 6 / ID 1329): Iron Maiden (`Iron_Maiden`, ID 7075)
    4. Luina 4 (`aldeg_cas04` / Box 8 / ID 1331): Wheel of the Unknown (`Mystery_Wheel`, ID 7076)
    5. Luina 5 (`aldeg_cas05` / Box 10 / ID 1333): Silver Ornament (`Silver_Fancy`, ID 7077)
    6. Britoniah 1 (`gefg_cas01` / Box 12 / ID 1335): Wrath of Valkyrie (`Anger_Of_Valkurye`, ID 7078)
    7. Britoniah 2 (`gefg_cas02` / Box 14 / ID 1337): Feather of Angel Wing (`Feather_Of_Angel`, ID 7079)
    8. Britoniah 3 (`gefg_cas03` / Box 16 / ID 1339): Cat's Footstep (`Foot_Step_Of_Cat`, ID 7080)
    9. Britoniah 4 (`gefg_cas04` / Box 18 / ID 1341): Woman's Moustache (`Beard_Of_Women`, ID 7081)
    10. Britoniah 5 (`gefg_cas05` / Box 20 / ID 1343): Root of Stone (`Root_Of_Stone`, ID 7082)
    11. Balder 1 (`payg_cas01` / Box 22 / ID 1345): Breath of Fish (`Soul_Of_Fish`, ID 7083)
    12. Balder 2 (`payg_cas02` / Box 24 / ID 1347): Bird's Spittle (`Saliva_Of_Bird`, ID 7084)
    13. Balder 3 (`payg_cas03` / Box 26 / ID 1349): Sinew of Bear (`Tendon_Of_Bear`, ID 7085)
    14. Balder 4 (`payg_cas04` / Box 28 / ID 1351): Sun God's Emblem (`Symbol_Of_Sun`, ID 7086)
    15. Balder 5 (`payg_cas05` / Box 30 / ID 1353): Spirit of Ghost (`Breath_Of_Soul`, ID 7087)
    16. Valkyrie 1 (`prtg_cas01` / Box 32 / ID 1355): Snow Crystal (`Crystal_Of_Snow`, ID 7088)
    17. Valkyrie 2 (`prtg_cas02` / Box 34 / ID 1357): Omen of Tempest (`Indication_Of_Tempest`, ID 7089)
    18. Valkyrie 3 (`prtg_cas03` / Box 36 / ID 1359): Quiet Wave (`Slilince_Wave`, ID 7090)
    19. Valkyrie 4 (`prtg_cas04` / Box 38 / ID 1361): Billow (`Rough_Billows`, ID 7091)
    20. Valkyrie 5 (`prtg_cas05` / Box 40 / ID 1363): Draupnir / Air Stream (`Air_Stream`, ID 7092)
    — Verified in [https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/mob_db.yml](https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/mob_db.yml) (high).

- **Drop Rate Discrepancy (Official Aegis 0.10% vs Emulator 0.80%):**
  - **Official Gravity Aegis Drop Rate (0.10%):** On official servers, God Item ingredients have a documented drop rate of **0.1%** (e.g. `Freya's Jewel (7073): Drop chance: 0.1%`, `Billow (7091): Drop chance: 0.1%`) — [https://www.divine-pride.net/database/monster/1325/](https://www.divine-pride.net/database/monster/1325/) and [https://www.divine-pride.net/database/monster/1361/](https://www.divine-pride.net/database/monster/1361/) (high).
  - **Emulator Database Rate (0.80%):** In `eAthena` and `rAthena` (`mob_db.yml` and legacy `mob_db.txt`), the drop rate is set to `Rate: 80` (80 out of 10,000 = **0.80%**), while rare headgear/armor drops were set to `Rate: 10` (0.10%) — [https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/mob_db.yml](https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/mob_db.yml) and [https://raw.githubusercontent.com/rathena/rathena/master/conf/battle/drops.conf](https://raw.githubusercontent.com/rathena/rathena/master/conf/battle/drops.conf) (high).

- **Guild Dungeons and MVP Spawns on 8-Hour Timers:**
  - `gld_dun01` (Balder / Payon): `boss_monster Eddga 1115,1,28800000,600000,1` (28,800,000 ms = 480 min = 8 hours; 600,000 ms = 10 min variance)
  - `gld_dun02` (Luina / Al De Baran): `boss_monster Doppelganger 1046,1,28800000,600000,1` (8 hours + 10 min variance)
  - `gld_dun03` (Valkyrie / Prontera): `boss_monster Maya 1147,1,28800000,600000,1` (8 hours + 10 min variance)
  - `gld_dun04` (Britoniah / Geffen): `boss_monster Dark Lord 1272,1,28800000,600000,1` (8 hours + 10 min variance)
  — [https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt](https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt) (high).

- **MVPs are NOT Realm-Exclusive or Guild-Dungeon-Exclusive:**
  - **Eddga** spawns in `pay_fild11` (Payon Forest).
  - **Doppelganger** spawns in `gef_dun02` (Geffen Dungeon level 2).
  - **Maya** spawns in `anthell02` (Ant Hell level 2).
  - **Dark Lord** spawns in `gl_chyard` (Glast Heim Churchyard).
  - They are designated realm bosses *within* the guild dungeon network, but they are classic open-world/dungeon MVPs available to all players regardless of castle ownership — [https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt](https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt) (high).

- **Single-Player Access Gating:**
  - Entrance to the Master Room: `if (getcharid(2) != .@GID) end;` via Butler/Steward NPC.
  - Entrance to Guild Dungeons: `if (getcharid(2) == .@GID) warp .@destination$,.@coordinates[0],.@coordinates[1];` via Lever NPC. If not in the owning guild: `"Nothing happened."`
  - In a single-player offline server without opposing guilds, all 20 castles start unowned (`CD_GUILD_ID == 0`). Guild Master rooms, daily treasure chest spawns, and guild dungeons are completely inaccessible without custom PvE siege/raid scripts or steward dialogue mechanisms allowing a solo player to claim castle ownership — [https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt](https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt) (high).

---

## Contradictions and Open Points

1. **Drop Rate: Official Aegis (0.10%) vs Athena Emulators (0.80%):**
   - Official server data extracted from Gravity server databases (archived and indexed on Divine Pride) lists the drop rate for the 20 God Item ingredients from their respective chests as **0.1%** (10/10,000).
   - In eAthena and rAthena flatfiles/YAML (`mob_db.txt`/`mob_db.yml`), the rate is codified as `80` (0.80%). The claim's assertion of "0.10%" aligns with official Gravity Aegis server mechanics, while servers running unmodified Athena data experience an 8x higher drop rate (0.80%).

2. **"Realm-Exclusive" Mischaracterization:**
   - The claim describes Eddga, Doppelganger, Maya, and Dark Lord as "4 realm-exclusive MVPs". This is factually inaccurate in the broader game: these four bosses are among RO's most iconic open-world and dungeon MVPs, accessible on standard maps without setting foot in a WoE castle. The exclusivity is only internal: within the 4 guild dungeon maps, each map features one of these four bosses.

3. **Spawn Timing: 00:01 vs 00:00:**
   - In emulator scripting, `OnClock0001` is standard practice to prevent race conditions during midnight rollover. Official documentation describes the spawn as taking place at "12:00 AM server time" (00:00).

---

## Sources read

- `http://web.archive.org/web/20040405051438/http://iro.ragnarokonline.com/game/warofemperium.asp` — Official iRO War of Emperium game guide from early 2004 detailing the Butler NPC, Master Room, and daily rare rewards. Highly reliable primary source.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/guild/agit_main.txt` — rAthena FE WoE castle manager script containing `OnClock0001:`, chest scaling formula `floor(Economy / 5) + 4`, and guild dungeon lever permissions. Highly reliable emulator reference.
- `https://irowiki.org/classic/War_of_Emperium` — Comprehensive iRO Classic WoE guide confirming Master Room mechanics, 5-point economy scaling per chest, and guild dungeon access. Highly reliable historical wiki.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/mobs/dungeons/gld_dun.txt` — rAthena pre-renewal mob spawns for `gld_dun01`..`04` confirming Eddga, Doppelganger, Maya, and Dark Lord on 28,800,000 ms (8-hour) respawn timers with 10-minute variance. Highly reliable emulator spawn script.
- `https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/mob_db.yml` — Pre-renewal monster database entries for `TREASURE_BOX1` through `TREASURE_BOX40` (IDs 1324–1363) detailing all 20 God Item drops and their `Rate: 80` (0.80%) emulator configuration. Highly reliable database file.
- `https://raw.githubusercontent.com/rathena/rathena/master/conf/battle/drops.conf` — Battle configuration defining rate scales (`100 = 1%`, `1 = 0.01%`) and `item_rate_treasure`. Highly reliable configuration reference.
- `https://www.divine-pride.net/database/monster/1325/` — Divine Pride database entry for `TREASURE_BOX2` confirming official Gravity drop rate of 0.1% for Freya's Jewel (ID 7073). Highly reliable official data mirror.
- `https://www.divine-pride.net/database/monster/1361/` — Divine Pride database entry for `TREASURE_BOX38` confirming official Gravity drop rate of 0.1% for Billow (ID 7091). Highly reliable official data mirror.

## Could not read

- `https://irowiki.org/classic/Castle_Treasure_Drops` — Returned bot check / HTTP 403 on full table fetch; partial text was recovered.
- `https://db.irowiki.org/db/treasure-drops/` — Requires JavaScript dropdown selection to render castle tables; data verified via `mob_db.yml` and `divine-pride.net` instead.

# Research Digest: Round 1 (euRO & Period Ragnarok Online Open Threads)

- **Date:** 2026-11-09
- **Brief:** Investigation of historical open threads for `ragnarok-offline` (euRO 2003-12-18 client, closed/open beta Dec 2003 – Feb 2004).
- **Scope:** Priority 1 threads (#17, #6, #5, #1, #2, #3) and Priority 2 threads (#4, #7, #8, #9, #12, #14, #18, #19).

---

## 1. Thread #17 (P1): War of Emperium & Castle Gating in Single Player
- **Summary:** Complete breakdown of castle-gated benefits on official servers (kRO/iRO/euRO 2003-04), official PvE precedents, and concrete single-player designs.
- **Key Findings:**
  - **Treasure Rooms & Daily Chests:** The Guild Master's Master Room spawned Treasure Chests (mobs 1324–1363, 2 alternating IDs per castle across 20 castles) daily at 00:01 via `floor(Economy / 5) + 4` (scaling from 4 boxes at 0 Economy to 24 at 100 Economy). Chest drops included Yggdrasil Berry/Seed, Angel Helm, Romantic Gent, Old Marquess, and the 20 God Item ingredients (Freyja's Jewel, Thor's Gauntlets, Iron Maiden, etc., dropping at 0.10%).
  - **Guild Dungeons (`gld_dun01..04`):** Entered exclusively by the castle-owning guild via the hidden switch/lever inside the castle. Functioned as realm-shared PvP zones holding 4 unique MVPs with 8-hour respawns: Eddga (`gld_dun01`), Doppelganger (`gld_dun02`), Maya (`gld_dun03`), and Dark Lord (`gld_dun04`), plus exclusive cards (Maya Purple, Dark Illusion).
  - **Castle Investments:** Economy (5k–955k) increased daily chest count; Defense (10k–1.91m) added +1,000 HP per point to castle Guardians and the Emperium.
  - **Official PvE Precedents:**
    1. *Unoccupied Castle Invasions:* On kRO, when a guild disbanded, hordes of high-tier monsters spawned inside the castle (farmed by players until patched on 2004-02-10).
    2. *euRO GM Castle Assault:* On Sunday, 2004-04-11 at 18:00 CET, euRO staff officially ran a GM attack against player castles.
- **Single-Player Design Options:**
  - *Option A (Neutral Castle Siege / PvE WoE):* Schedule a weekly WoE where castles are defended by Guardian NPCs and spawned mob hordes; breaking the Emperium claims ownership for the solo player, unlocking the Butler, daily chests, and the guild dungeon lever.
  - *Option B (Castle Steward / Mercenary Contract):* Place a Steward NPC at castle gates allowing the solo player to pay a rental fee or complete a trial to "contract" the castle for a week.
  - *Option C (Independent Realm Dungeon Gatekeeper):* Add a dungeon gatekeeper NPC in town fields requiring a boss trophy (e.g. Broken Emperium) to enter `gld_dun01..04`.

---

## 2. Thread #6 (P1): The Start Cell & Novice Training Ground
- **Summary:** Authenticated starting spawn cell, client map aliasing mechanism, origin of Athena's `53,111`, and early training ground flow.
- **Key Findings:**
  - **Start Map & Cell:** Officially `new_1-1.gat, 53, 111` facing North.
  - **Client Map Name Resolution:** The euRO client `data.grf` internal file `data/resnametable.txt` explicitly aliases:
    ```
    new_1-1.gat#new_zone01.gat#
    new_1-2.gat#new_zone02.gat#
    new_1-3.gat#new_zone03.gat#
    new_1-4.gat#new_zone04.gat#
    ```
    The official server transmitted `new_1-1.gat`, and the client rendered `new_zone01.gat`. Cell `(53, 111)` is confirmed walkable (type 0, altitude -7.81), located dead center on the western starting bridgehead platform.
  - **Origin of 53,111:** Recorded from live observation of jRO by jAthena developer `いど` (Ido) in snapshot `//0305` (mid-2003) and made configurable in `//0307`. Gravity placed tutorial guide NPC Sprakki at `(53, 114)` — exactly three cells north!
  - **Starting Gear & Flow:** Players began with 1x Knife (1201) and 1x Cotton Shirt (2301) (no Novice gear; Novice gear was added to kRO on 2004-06-01). Usher at `100, 29` offered an immediate town exit or entrance with 50z subsidy (817 script bug gave 150z). Flow: 1st course lectures, 2nd course Fabre Forest combat, aptitude test awarding 100–500z, 10 Red Potions, 1 Milk, and job guild warp.

---

## 3. Thread #5 (P1): 2-1 Job Changes before New Comodo
- **Summary:** Progression mechanics during euRO closed beta (Dec 2003) vs New Comodo update (Jan 2004).
- **Key Findings:**
  - **Closed Beta (Dec 20, 2003 – Jan 8, 2004):** Advancements to Knight, Priest, Wizard, Blacksmith, Hunter, and Assassin were instant at Job Level 40 via plain guild NPCs with zero tests. NPCs used authentic Aegis dialogue explaining that King Tristram III issued an emergency decree waiving tests to bolster the realm.
  - **New Comodo Update (Jan 8/18, 2004):** Introduced 2-1 job quests to euRO (*"added 2nd job quest"*). Forum testers filed bug reports on Jan 18 (*"very good the 2nd job Quest but buggy, i can't be a Priest \*sigh\*"*).
  - **iRO Timeline:** Turtle Island on Dec 16, 2003 introduced Knight, Assassin, and Hunter quests. Remaining quests followed later.

---

## 4. Thread #1 (P1): euRO Closed Beta Rates
- **Summary:** Clarification of the closed beta "higher experience" setting and commercial rates.
- **Key Findings:**
  - **Closed Beta Boost:** BHI-OE's launch announcement stated: *"As we are testing with higher experience, we will delete all characters... at the end of the Closed Beta."* Gamona staff member Astartus confirmed on 2004-01-27 that both EXP and Drop rates were excessively high.
  - **Multiplier:** BHI-OE never published an exact numerical multiplier (e.g. 2x, 3x, or 5x). Forum f=3 topics (Topic 67 and 80) asking about exact rates were not preserved in Wayback.
  - **Open Beta & Commercial Rates:** Confirmed strictly stock 1x/1x/1x. Open Beta characters carried over directly into commercial service on 2004-04-15 with items/zeny rolled back to April 8.

---

## 5. Thread #2 (P1): Preservation of Official 2003-04 English NPC Dialogue
- **Summary:** Preservation channels of Gravity English text vs eAthena 817 fan translations.
- **Key Findings:**
  - OpenKore client tables (`tables/euRO/quests.txt` vs `tables/iRO/quests.txt`) confirm euRO used an earlier direct Gravity English translation (*"after pass the all of test courses"*), while iRO received a polished American English pass.
  - eAthena 817 used amateur fan translations (Tono: *"another monster metamorphic"*, Swordsman: *"Sword Master"*). Authentic Gravity text was witty (*"awkward puberty"*, *"Captain Herman"*).
  - Worked side-by-side comparisons produced for Prontera citizen Tono, Kafra services, Izlude Swordsman Guild, and Prontera Chivalry.

---

## 6. Thread #3 (P1): Kafra Fares & Save Cells before 2004-02-24
- **Summary:** Teleport fares, storage fees, and save points prior to the kRO overhaul.
- **Key Findings:**
  - **Prontera → Geffen Teleport:** Authentic fare was **1400z**. 817's 1800z deduction in code was a copy-paste bug from Morroc.
  - **Alberta → Prontera Teleport:** Authentic fare was **2100z**. 817's first NPC menu showing 1800z was a typo.
  - **Prontera → Alberta:** Did not exist prior to 2004-02-24.
  - **Storage Fees Were Tiered:** Prontera 40z, Geffen/Izlude/Payon/Morroc 30z, Alberta 50z, Dungeons 80–130z (817 flattened all to 30z). Cart rental was 800z.
  - **Izlude Save Cell:** Authentic save cell was **`izlude (94, 103)`**. In 817, `SaveIn` omitted the `savepoint` call; `(91, 105)` was merely the warp arrival cell.

---

## 7. Threads #4 & #7 (P2): Kafra Points/Passes & 1st-Class Quest Skills
- **Summary:** Status of Kafra rewards and platinum quest skills in late 2003.
- **Key Findings:**
  - **Kafra Points & Passes:** Completely disabled across official servers. kRO removed the voucher menu on 2003-05-20; Item #1084 was flagged as unusable; 817's Sweet Potato lottery was a broken fan script.
  - **1st-Class Quest Skills:** All 16 platinum skills were active on official servers prior to 2004. 817's Thief quest NPC at `morocc 165, 124` was an emulator fan invention with broken formatting; the authentic official NPCs were Alcouskou at `moc_prydb1 154, 128` (opposite Thief Guild) and RuRumuni at `payon 91, 77`. Novice First Aid and Play Dead quests verified in Prontera.

---

## 8. Threads #8 & #9 (P2): Wizard Quest & Refining Mechanics
- **Summary:** Wizard test content, Phracon pricing, armor DEF math, and card breaking.
- **Key Findings:**
  - **Wizard Test:** Authentic 3-stage test preserved in `eathena-v0.5.2` (item collection, 10-question written exam by Raul Expagarus with 90% pass mark, and 3-minute combat gauntlet in `job_wiz` across Water/Earth/Fire rooms with a 4-fail Worn Out Scroll bribe fallback).
  - **Phracon Pricing:** Confirmed at **200z** (iRO's 2000z website price was an editorial typo).
  - **Armor Refine DEF:** Grants **+0.7 DEF** per refine level (encoded as `70` in `refine_db.txt`, rounded via `(refinedef + 50) / 100`).
  - **Failure Penalty:** Upgrade failure completely destroys the item, the zeny, and all slotted cards (confirmed by iRO guide and kRO 2003-02-04 patch).

---

## 10. Laptop Pass 1 Findings (GameFAQs & iRO Bug Board Archives)
- **SoulEclipse Skills Guide (GameFAQs 30694):** Full breakdown of all 1st-class quest skills, job level requirements, and item costs (e.g. Energy Coat: Job 35, 5 Shells, 1 Solid Shell, 3 Glass Beads, 1 1-Carat Diamond; Auto Berserk: Job 30, 35 Powder of Butterfly, 10 Honey, 10 Horrendous Mouth, 10 Decayed Nails; Cart Revolution: Job 35, 30 Iron, 30 Sticky Mucus, 20 Fly Wing, 5 Tentacle, 2 Grape Juice; Holy Light: Job 35, 1 Crystal Blue, 1 Rosary, 1 Opal).
- **RoboShop Refine Guide (GameFAQs 18644):** Verifies refine blacksmith locations and prices across Alberta, Morroc, Payon, and Prontera; confirms refine ATK is flat base damage not mitigated by size or hard DEF.
- **iRO Bug Board (June 2004, `bugcontent.asp`):**
  - Bug #4201 preserves verbatim official English pet speech strings: *"Wahh, too much food!"*, *"I think I'm gonna throw up..."*, *"For the first time in my life, I'm full. You complete me."*, *"Fattening me up for slaughter, eh? I'm watching you..."*, *"Bananas again? With this many, I'll turn yellow and become a Drops?"*, *"Yummy yummy in my tummy"*.
  - Bug #4218 confirms players expected Job 50 Mages to skip the 1st Wizard test stage (item gathering).
  - Bug #4222 reveals a famous official script logic flaw in the Assassin job change test where cancelling at the 'annonymus' NPC allowed bypassing the written test.

- **Summary:** Map rules, pet database, arena timeline, and Lutie town permanence.
- **Key Findings:**
  - **Map Flags:** `indoorrswtable.txt` in client GRF defines 31 indoor maps where teleportation is disabled. mod0631's `nopenalty` on outdoor towns was an emulator anti-grief invention (official servers applied 1% EXP death penalty in towns).
  - **Pets:** Official guide lists 24 pets (Eggs 9001–9024); mod0631/817 add Bongun (Egg 9025, *Her Heart*) from a Comodo quest.
  - **Izlude Arena:** **Did NOT exist in Dec 2003** on iRO/euRO (added to kRO in Oct 2004). 817 arena scripts were Prontera tests.
  - **Lutie Permanence & Patcher xmas Trigger:** Lutie was a permanent town since Episode 2 (Dec 2002). euRO CD patcher `Ragnarok.exe` contained a hardcoded `"xmas"` trigger at offset `0x51c4` fetching `2003-10-31gdata_xmas.gpf` when `patch_allow.txt` returned `"xmas"`.

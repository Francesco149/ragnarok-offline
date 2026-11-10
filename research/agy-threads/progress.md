# Research Progress: euRO & Period Ragnarok Online Open Threads

- **Date:** 2026-11-09
- **Brief:** Thorough multi-agent research on the open threads presented in `HANDOFF.md` for `ragnarok-offline` (euRO 2003-12-18 client reconstruction), prioritizing P1 threads (#17, #6, #5, #1, #2, #3) followed by P2/P3.
- **Output Directory:** `/opt/src/ragnarok-offline/research/agy-threads`
- **Current Phase:** Complete (Deliverables written to `REPORT-antigravity.md` and approved by critic in review 2)

## Fact Checks

| Check | Role | Claim Summary | Conversation ID | Status | Verdict |
|---|---|---|---|---|---|
| Check 1 | Check 1 | 2-1 job changes instant at Job 40 before New Comodo | `bcba3566-31df-4950-ad8b-c801541fadfb` | done | PARTLY (Confirmed instant Job 40 change prior to New Comodo; King Tristram decree dialogue was Athena fan-written placeholder) |
| Check 2 | Check 2 | Armor refine is +0.7 DEF from 2003-02-04 patch | `e95827ca-de66-47b4-b148-187b006c5d90` | done | PARTLY (Confirmed refine DEF was nerfed below base DEF with safe +4; ~0.7 is emulator implementation of unstated official curve) |
| Check 3 | Check 3 | Phracon is 200z; 2000z on iRO was a typo | `a830ba29-470b-4be5-9ce0-71f6257653dd` | done | CONFIRMED (Mathematically proven: 7x(200z+50z)=1750z for +7 Lv1 weapon) |
| Check 4 | Check 4 | resnametable.txt aliases new_1-1 to new_zone01 | `4e4d31d3-5907-42c4-a676-d8d9f470258d` | done | CONFIRMED |
| Check 5 | Check 5 | Video GJO1B9ogKXY validates 53,111 bridgehead start | `27fe47b3-c591-40ef-adb3-97c483eef7dc` | done | PARTLY (Confirmed bridgehead spawn and eastward bridge walk; footage was from private server so exact cell 53,111 is emulator-derived) |
| Check 6 | Check 6 | iRO bug reports preserve pet speech & test edge cases | `58b80502-73be-4ede-b685-be64dc765f38` | done | CONFIRMED |
| Check 7 | Check 7 | OpenKore tables prove euRO had earlier English pass | `45ba4170-7efb-4997-9252-f4e4821f2de9` | done | PARTLY (Confirmed regional translation branches: Gravity Korea raw literal on euRO vs Gravity Interactive on iRO; quests.txt dates from Ep 12) |
| Check 8 | Check 8 | Ragnarok.exe hardcoded xmas trigger for patch | `67cb25f7-3415-45b9-895c-25fd93bb0629` | done | PARTLY (Confirmed hardcoded strings and offset 0x51c4 check; routine checked local existence of GPF or displayed notice and halted; Lutie permanently packaged with 320 assets in data.grf) |
| Check 9 | Check 9 | WoE chest math (floor(Econ/5)+4) & 4 realm MVPs | `a31612c9-bed3-4e50-a42c-317c4896a6c3` | done | PARTLY (Confirmed chest math floor(Econ/5)+4 and 4 MVPs; MVPs also had public open-world spawns) |
| Check 10 | Check 10 | Pre-2004-02-24 Kafra fares (Prontera->Geffen 1400z) | `ab926623-d4d9-498c-9c5d-52925c0b5004` | done | CONFIRMED |
| Check 11 | Check 11 | Closed beta boosted rates without published multiplier | `c75e4727-9293-42f8-9ffa-89007311e10f` | done | PARTLY (Confirmed boosted EXP/drop rates without published integer; Open Beta 1x carried to P2P with April 8 rollback) |
| Check 12 | Check 12 | Izlude Arena did not exist in Dec 2003 (kRO Oct 2004) | `bd951ce6-cc99-4217-8f17-855b92efe1d0` | done | CONFIRMED |

## Laptop Passes

| Pass | Role | Conversation ID | Status | Notes |
|---|---|---|---|---|
| Pass 1 | Laptop pass 1 | `111812d5-36ec-408d-b816-e119d78f1076` | done | Successfully fetched 5 pages; extracted verbatim pet speech strings, Job 50 Wizard item skip, and 1st-class quest skill item requirements from GameFAQs & iRO bug board |

## Subagent Status (Round 1)

| Sub-question ID | Role | Conversation ID | Status | Key Findings / Result |
|---|---|---|---|---|
| `woe-sp` | Research woe-sp | `d50b1368-ae66-40b9-86fb-f05213291a3f` | done | Completed with high confidence; detailed math on daily treasure box spawn (floor(Economy/5)+4 = 4-24 boxes), drops (God Item materials, Angel Helm), guild dungeon levers & 8h MVPs (Eddga, Doppelganger, Maya, Dark Lord), official PvE precedents (empty castle mob hordes, euRO GM assault), and single-player design options. |
| `start-cell` | Research start-cell | `6abbac25-5e5d-47ca-aff7-193ae5b45159` | done | Completed with high confidence; start cell is confirmed new_1-1.gat,53,111 (aliased via client data/resnametable.txt to new_zone01.gat, cell walkable type 0 on bridgehead platform); originated in jAthena //0305 from jRO live observation; Sprakki later placed 3 cells north at (53, 114); starting items Knife 1201 and Cotton Shirt 2301; training ground flow verified. |
| `jobchange-2-1` | Research jobchange-2-1 | `a12eed60-e6b3-4341-84b6-44a9b3f965c8` | done | Completed with high confidence; in euRO closed beta (Dec 2003) 2-1 job change was instant at Job 40 via plain guild NPCs with authentic King Tristram III dialogue; quests arrived with New Comodo (Jan 8/18 2004). |
| `closed-beta-rates` | Research closed-beta-rates | `8809cb9e-c592-47cf-a50f-89174749c371` | done | Completed with high confidence; BHI-OE confirmed 'testing with higher experience' and Gamona staff confirmed both EXP and Drop rates were excessively high; exact multiplier was never published (Topics 67/80 bodies 404); Open Beta & Commercial rates confirmed 1x/1x/1x because Open Beta characters transferred directly to P2P. |
| `official-npc-text` | Research official-npc-text | `fc4fe3c1-64cb-4624-a6a9-75a6139284fb` | done | Completed with high confidence; OpenKore tables reveal euRO had earlier Gravity translation than iRO; worked comparisons for Tono, Kafra, Swordsman, Knight. |
| `kafra-fares` | Research kafra-fares | `f90b7631-1252-47fc-8117-ed71aadc09bf` | done | Completed with high confidence; Prontera->Geffen was 1400z (817 1800z deduction was an emulator copy-paste bug); Alberta->Prontera was 2100z (menu 1800z was a typo); Prontera did not offer Alberta pre-2004-02-24; storage fees were tiered (Prontera 40z, Geffen/Izlude 30z, Alberta 50z, Orc Dungeon 130z); Izlude official save cell is izlude (94, 103) (91, 105 was only warp arrival). |
| `kafra-points-skills` | Research kafra-points-skills | `c421d1e9-308b-44ba-a8cd-430fbeeaf13b` | done | Completed with high confidence; Kafra points/pass retired by May 2003 (kRO removed menu 2003-05-20, item 1084 disabled); all 16 first-class quest skills active in official 2003; 817's Thief NPC at morocc 165,124 was an emulator bug/invention (real NPC is Alcouskou at moc_prydb1 154,128 and RuRumuni at payon 91,77); Novice First Aid & Play Dead verified in Prontera. |
| `wizard-quest-refine` | Research wizard-quest-refine | `2a373694-2206-4c20-8b71-0115f3aa4ac2` | done | Completed with high confidence; authentic Wizard test verified from eathena-v0.5.2 (1,563 lines, 3 stages, Raul Expagarus exam, job_wiz combat); Phracon is 200z; armor refine is +0.7 DEF (refine_db 70); failure destroys item, zeny, and all slotted cards. |
| `maps-pets-arena-lutie` | Research maps-pets-arena-lutie | `f6e885e7-9377-4e96-9e8f-bbe34d262a0c` | done | Completed with high confidence; indoorrswtable defines 31 indoor noteleport maps; nopenalty on towns was emulator invention; 24 official pets + 1 Bongun (Her Heart); Izlude Arena did NOT exist in Dec 2003 (kRO Oct 2004); Lutie permanent since Ep 2 (Dec 2002); CD patcher Ragnarok.exe had hardcoded xmas trigger for 2003-10-31gdata_xmas.gpf. |

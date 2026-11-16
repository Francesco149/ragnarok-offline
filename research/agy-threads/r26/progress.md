# Round 26 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T1 | Job quests & early progression (thread 5) | partly (Beta 2 Hunter job change at job level 40 without quest test and Falcon rental fee 2500z confirmed in Giga Bahamut FAQ 20075) | T1-067 – T1-068 |
| T4 | Skills & mechanics reconstruction (threads 22, 34) | partly (Frost Diver Level 10 duration 20-30s confirmed in lvlika FAQ 25510; Poison 3% max HP/sec drain, natural HP regen blockage, removal only by Detoxify/Status Recovery and not Blessing, and enemy DEF reduction confirmed in peropero11 FAQ 29579) | T4-028 – T4-032 |
| T8 | Status effects, traps, environmental & skill interactions (threads 22, 34) | partly (Ice Wall 2x5 mob entrapment, monster pathing around unclosed walls without attacking walls, and Storm Gust simultaneous wiping confirmed in lvlika FAQ 25510; Talkie Box 600s duration and picking up expired unused traps confirmed in Giga Bahamut FAQ 20075; Venom Splasher deactivation upon target hit confirmed in peropero11 FAQ 29579; Comodo Brandish Spear and Bowling Bash 1s delay and 2/3 DEF during Brandish cast confirmed in TrinityUser FAQ 25432) | T8-050 – T8-057 |
| T9 | Small, independent (threads 10, 22, 34) | partly (Claymore Trap 2 trap consumption and Shockwave Trap 2 trap consumption in iRO Beta 2 confirmed in Giga Bahamut FAQ 20075; Auto Counter duration bar and forced stationary pose, and Comodo 2x damage + hit rate rework confirmed in TrinityUser FAQ 25432; MVP encountering without minions, Counter Attack speed independence from ASPD, Bash level 6-10 stun chance in Comodo, and Two-Handed Sword Mastery 1 prerequisite for Counter Attack confirmed in mangdiws FAQ 28554; Geffen Dungeon Level 3 without Joker and Glast Heim Castle Level 1 spawns confirmed in Weggy FAQ 20092) | T9-338 – T9-347 |

Total: 25 rows across 4 tasks.
Checker result (`check.txt`): 25 PASS, 0 WARN, 0 FAIL, 0 NONE of 25 rows verified with `python3 ../../check_rows.py --pages pages rows/*.jsonl`.

## Task Details and Row Mapping

### T1: Job Quests & Early Progression (Thread 5)
- **Status:** partly
- **Rows:** `T1-067` through `T1-068`
  - `T1-067`: Giga Bahamut FAQ vFinal (2003-08-12, iRO) records that at Job Level 40, talking to the Hunter NPC outside Payon sends the player to the Hunter Guild where the NPC immediately transforms them into a Hunter with no quest test (source: `gamefaqs_archerhunter_gigabahamut_20075.html`, value: `job level 40`).
  - `T1-068`: Giga Bahamut FAQ vFinal (2003-08-12, iRO) records that a falcon is bought from the NPC to the right of the hunter guild for 2500z after acquiring Beast Bane level 1 and Falcon Mastery level 1 (source: `gamefaqs_archerhunter_gigabahamut_20075.html`, value: `2500z`).

### T4: Skills & Mechanics Reconstruction (Threads 22, 34)
- **Status:** partly
- **Rows:** `T4-028` through `T4-032`
  - `T4-028`: lvlika Agility Wizard FAQ v2.0 (2003-09-29, iRO Loki) records that Level 10 Frost Diver lasts around 20-30 seconds (source: `gamefaqs_wizard_lvlika_25510.html`, value: `20-30seconds`).
  - `T4-029`: peropero11 Double Dagger Poison Assassin Guide v1.5 (2004-11-26, pRO Loki) records that players in a poisoned state appear to lose 3% of max HP every second and Luck reduces bad status duration (source: `gamefaqs_doubledagger_peropero11_29579.html`, value: `loss 3% of their max hp every second`).
  - `T4-030`: peropero11 Double Dagger Poison Assassin Guide v1.5 (2004-11-26, pRO Loki) records that players cannot regenerate HP normally when poisoned (source: `gamefaqs_doubledagger_peropero11_29579.html`, value: `CANNOT regenerate HP/HP normally when poisoned`).
  - `T4-031`: peropero11 Double Dagger Poison Assassin Guide v1.5 (2004-11-26, pRO Loki) records that only Detoxify and Status Recovery skills can remove the poisoned status, and Blessing cannot remove poison (source: `gamefaqs_doubledagger_peropero11_29579.html`, value: `detoxify (thief) and status recovery (priest) skills are the ones capable of removing the poisoned status`).
  - `T4-032`: peropero11 Double Dagger Poison Assassin Guide v1.5 (2004-11-26, pRO Loki) records that poisoning an enemy lowers their defense (source: `gamefaqs_doubledagger_peropero11_29579.html`, value: `LOWERS their def`).

### T8: Status Effects, Traps, Environmental & Skill Interactions (Threads 22, 34)
- **Status:** partly
- **Rows:** `T8-050` through `T8-057`
  - `T8-050`: lvlika Agility Wizard FAQ v2.0 (2003-09-29, iRO Loki) records that casting Ice Wall on the square a mob is running over traps the monsters, forming a 2x5 wall of ice to hold them (source: `gamefaqs_wizard_lvlika_25510.html`, value: `trapping every single one`).
  - `T8-051`: lvlika Agility Wizard FAQ v2.0 (2003-09-29, iRO Loki) records that if a player runs behind an unclosed Ice Wall, every single monster escapes by running around the opening rather than attacking or breaking the wall (source: `gamefaqs_wizard_lvlika_25510.html`, value: `run behind an unclosed icewall, every single monster will escape and chase you all over again`).
  - `T8-052`: lvlika Agility Wizard FAQ v2.0 (2003-09-29, iRO Loki) records that trapped monsters inside closed Ice Walls are kept immobilized for the duration of casting Storm Gust to kill them simultaneously (source: `gamefaqs_wizard_lvlika_25510.html`, value: `manipulate the monsters so that they all fall to ONE SG casting`).
  - `T8-053`: Giga Bahamut FAQ vFinal (2003-08-12, iRO) records Talkiebox as displaying a message to any players stepping on it with a duration of 600 seconds (source: `gamefaqs_archerhunter_gigabahamut_20075.html`, value: `600 seconds`).
  - `T8-054`: Giga Bahamut FAQ vFinal (2003-08-12, iRO) records that any unused trap can be picked up after its lasting time has depleted (source: `gamefaqs_archerhunter_gigabahamut_20075.html`, value: `any unused trap will be able to be picked up after it's lasting time has depleted`).
  - `T8-055`: peropero11 Double Dagger Poison Assassin Guide v1.5 (2004-11-26, pRO Loki) records that hitting the enemy on which Venom Splasher was cast deactivates the skill (source: `gamefaqs_doubledagger_peropero11_29579.html`, value: `venom splasher is deactivated`).
  - `T8-056`: TrinityUser Lancer FAQ v0.99e (2004-06-28, iRO Sakray) records that in the Comodo patch, Brandish Spear and Bowling Bash have a 1 second delay (source: `gamefaqs_lancer_trinityuser_25432.html`, value: `1 second delay on brandish spear and bowling bash`).
  - `T8-057`: TrinityUser Lancer FAQ v0.99e (2004-06-28, iRO Sakray) records that while casting Brandish Spear, the player has 2/3 defense (source: `gamefaqs_lancer_trinityuser_25432.html`, value: `2/3 defense`).

### T9: Small, Independent (Threads 10, 22, 34)
- **Status:** partly
- **Rows:** `T9-338` through `T9-347`
  - `T9-338`: Giga Bahamut FAQ vFinal (2003-08-12, iRO) records Claymore Trap as using 2 traps (source: `gamefaqs_archerhunter_gigabahamut_20075.html`, value: `Uses 2 traps`).
  - `T9-339`: Giga Bahamut FAQ vFinal (2003-08-12, iRO) records Shockwave Trap as using 2 traps (source: `gamefaqs_archerhunter_gigabahamut_20075.html`, value: `Uses 2 traps`).
  - `T9-340`: TrinityUser Lancer FAQ v0.99e (2004-06-28, iRO Sakray) records Auto Counter has a duration bar and makes the Knight stand still while using it (source: `gamefaqs_lancer_trinityuser_25432.html`, value: `makes you stand still while using it`).
  - `T9-341`: TrinityUser Lancer FAQ v0.99e (2004-06-28, iRO Sakray) records that in Comodo, instead of auto-hit-critical, Counter does 2x damage and has an added hit rate (source: `gamefaqs_lancer_trinityuser_25432.html`, value: `do 2x damage and has an added hit rate`).
  - `T9-342`: mangdiws Spear Knight FAQ v1.15 (2004-02-09, pRO Chaos) records that an MVP can be encountered without its minions (source: `gamefaqs_spearknight_mangdiws_28554.html`, value: `encounter an MVP without its minions`).
  - `T9-343`: mangdiws Spear Knight FAQ v1.15 (2004-02-09, pRO Chaos) records that ASPD does not affect countering speed for Counter Attack (source: `gamefaqs_spearknight_mangdiws_28554.html`, value: `ASPD deosn't affect countering speed`).
  - `T9-344`: mangdiws Spear Knight FAQ v1.15 (2004-02-09, pRO Chaos) records that with the Comodo skills update, level 6-10 Bash has a chance of stunning an opponent (source: `gamefaqs_spearknight_mangdiws_28554.html`, value: `level 6-10 bash has a chance of stunning an opponent`).
  - `T9-345`: mangdiws Spear Knight FAQ v1.15 (2004-02-09, pRO Chaos) records that one point in Two Handed Sword Mastery is required for Counter Attack to appear (source: `gamefaqs_spearknight_mangdiws_28554.html`, value: `ONE point in two hand mastery for Counter Attack`).
  - `T9-346`: Weggy FAQ v6.7 (2004-07-09, iRO) records Geffen Dungeon Level 3 enemies as Marionette, Argos, Whisper, Bathoroy, Myst, Jakk, Nightmare, and Doppleganger (MVP), containing no Joker (source: `gamefaqs_weggy_20092.html`, value: `Marionette, Argos, Whisper, Bathoroy, Myst, Jakk, Nightmare, Doppleganger (MVP)`).
  - `T9-347`: Weggy FAQ v6.7 (2004-07-09, iRO) records Ghast Heim Castle Level 1 enemies as Joker, Baphomet Jr., Rideword, Evil Druid, Tirfing, and Mysteltainn (source: `gamefaqs_weggy_20092.html`, value: `Joker, Baphomet Jr., Rideword, Evil Druid, Tirfing, Mysteltainn`).

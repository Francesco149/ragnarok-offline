# Thread #5: 2-1 Job Changes on euRO before New Comodo

## Answer so far

### 1. How did a closed-beta player (Dec 2003) become a Knight, Priest, Wizard, Blacksmith, Hunter, or Assassin?
In euRO closed beta (December 20, 2003 – January 8/18, 2004), players changed to 2-1 classes (Knight, Priest, Wizard, Blacksmith, Hunter, Assassin) by speaking directly to the respective Job Guildsman NPC (in Prontera Chivalry, Prontera Church, Geffen Tower, Geffen Blacksmith Guild, Payon Hunter Guild, or Morroc Assassin Clan).
- **Prerequisite:** First-job class, **Job Level 40 or higher**, with all skill points allocated.
- **Test:** **None.** No item collection, no written exam, and no combat/survival trial.
- **Lore justification:** The NPCs explained in-game that the Kingdom of Rune-Midgarts / King Tristram III (or the respective guild) was urgently promoting 2nd classes to develop and protect the kingdom, so requirements were temporarily lowered to only Job Level 40. However, the NPCs explicitly warned players that rumors suggested strict requirements and tests would be imposed in the future once society stabilized.

### 2. Was it a plain job-change NPC at Job Lv 40 without tests (like Athena 659)? Or was there no way to change jobs at all?
It was **definitely a plain job-change NPC at Job Lv 40 without tests**, exactly as documented in Athena 659 (`athena0659TestEngmod`, 2004-01-01) and eAthena 817 (2004-02-19).
- There was **not** "no way to change jobs". Closed-beta testers were explicitly provided boosted experience rates to reach 2nd classes quickly, and closed-beta player accounts from January 2004 attest to active 2nd-job play (e.g. forum signatures on `forum.euro.bhi-oe.de` topic 1791 mentioning `job:ice/thunder-wizzard`, `battle-aco`, `agi-swordman`).
- The dialogue in Athena 659 and 817 is authentic pre-quest Gravity Aegis script dialogue:
  - **Knight (`prt_in 88, 101`):** *"His Majesty Tristram 3rd has been encouraging people to be Knights without any difficult requirements except Job Level 40 or above. Anyway according to the Rumor, people say another requirements would be added soon when the Society become stabilized. If it were so,you got to become a Knight now."*
  - **Assassin (`in_moc_16 71, 88`):** *"We, Assasins are Guiding Lights of Thiefs who reached the Highest Limit,being at Job Level 40 or above... For now we just stay out of People's Sight... In no time,we will enlighten the Assassin Clan. And then It will be no good to try to become one of us, just because of our fame. We will restrict people to join."*
  - **Blacksmith (`geffen_in 110, 170`):** *"Actually Rune-Midgard is an Underdevelopment Country in the Metal Industry! That's why the Govenment encourage people to join us,Blachsmiths more and more. To make our nation an Advanced Country, we establish the requirements of Blacksmiths only Job Level 40!... Muhahaha... There is no Later for us, Alright?"*
  - **Wizard (`gef_tower 111, 37`):** *"To Utilise the Magical Power of Geffen, Rune-Midgard encourages Mages to Grow Wizards. Why don't you apply on the Wizard if you reach at Job Level 40? At this Time Geffen is at the height of its Magical prosperity.Also this is the Good opportunity to become a Wizard. There won't be a Tomorrow for Mages."*
  - **Hunter (`in_hunter 99, 98`):** *"It seems the King needs more Hunters! So he established a new constitution of Hunters, so you can be a Hunter anytime if you reach at Job Level 40 or above. This is the good chance to become a Hunter! I assume the Government will introduce many Strict requirements for Hunter later if there are too many people want to be Hunters!"*

### 3. When did 2-1 job quests get introduced on kRO and iRO?
- **kRO:**
  - 2-1 classes originated in 2002.
  - Job change quests were phased into kRO across late 2002 and 2003. Official patch notes on 2003-07-29 note dialogue fixes for 2-1 job change NPCs (`:- 2-1 차 전직 NPC 대사 중 일부를 수정하였습니다.`), and 2004-02-10 notes bugfixes for the Wizard job change test stages.
- **iRO:**
  - Prior to October 2003, iRO had no 2-1 job change quests; all 2-1 job advancements were instant at Job Level 40.
  - On 2003-10-03, iRO GM Team announced the implementation of 2-1 job quests for **Knight and Assassin** on the Sakray test server for the maintenance of Tuesday, 2003-10-07.
  - On 2003-10-07 (Notice seq 213), the update slipped due to unforeseen technical difficulties (*"We ran into some technical difficulties and were not able to update the Sakray server with the 2-1 job quest update."*).
  - In late October 2003 (between Oct 26 and Nov 2, 2003, documented on `renewal.playragnarok.com/news/newslist.aspx?page=288`), Sakray received: *"Script Update, new headgear NPC's and 2-1 job change scripts."*
  - On 2003-12-16, the Turtle Island update launched on live servers Chaos and Loki, introducing 2-1 job quests for **Knight, Assassin, and Hunter** (documented on iRO Wiki Updates Changelog and iRO official notices). The remaining 2-1 job quests (Wizard, Priest, Blacksmith) were rolled out later.
- **euRO:**
  - 2003-12-20: euRO launched Closed Beta on the pre-"New Comodo" codebase (stock "Comodo + Q-Pet"), featuring instant 2-1 job changes at Job Lv 40 without quests.
  - 2004-01-08 (website announcement) / 2004-01-18 (forum topic 1791 by PerrI): euRO updated to **"New Comodo"**, officially adding 2nd-job quests (*"2. added 2nd job quest"*, Italian: *"2. Aggiunte le quest per diventare seconda classe(wizard,priest,etc)"*).
  - Players immediately encountered launch bugs with the newly added quests (e.g. Condor in topic 1791: *"very good the 2nd job Quest but buggy, i can't be a Priest \*sigh\*"*).

### 4. Content Profile Recommendation for `ragnarok-offline`
- **Closed-Beta Profile (2003-12-20 to 2004-01-08):**
  - **DISABLE** 2-1 job change quests.
  - Use simple Job Level 40 instant job-change NPCs using the authentic period dialogue preserved in Athena 659 / 817.
- **Open-Beta / Launch Profile ("New Comodo", 2004-01-08 onwards):**
  - **ENABLE** 2-1 job change quests.

---

## Findings

### 1. euRO Closed Beta & New Comodo Update (euRO, 2003-12 to 2004-01)
- **[READ] euRO "New Comodo" Announcement (forum.euro.bhi-oe.de topic 1791, 2004-01-18):**
  Director PerrI posted:
  `"New in 'New Comodo' / 'New Comodo''daki yenilikler / Neu in 'New Comodo' / Ecco le novità della 'New Comodo'`
  `1. added heal-effect, modified potions`
  `2. added 2nd job quest`
  `3. added monster skills`
  `4. added lvl99 aura`
  `5. fixed refining bug`
  `6. adjusted monsters HP / Exp -ends-`
  `ITALIAN 1. Aggiunti gli effetti grafici degli oggetti di cura, pozioni modificate 2. Aggiunte le quest per diventare seconda classe(wizard,priest,etc) 3. Aggiunte ai mostri le skill (ora i mostri possono lanciare magie) 4. Aggiunta l'aura intorno al personaggio per chi è livello 99 5. Sistemati i bug per la raffinazione degli oggetti 6. Modificate le caratteristiche dei mostri e il loro premio in esperienza"`
  — `https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791` (published 2004-01-18; high).
- **[READ] Immediate Player Reaction to 2nd Job Quests in Topic 1791 (2004-01-18):**
  Player Condor posted on Sun Jan 18, 2004 9:11 am:
  `"very good the 2nd job Quest but buggy, i can't be a Priest *sigh* but very good ^^ i like the Monsterskills"`
  And player Brainpain's signature confirms 2nd job characters already played during closed beta:
  `"char:Brainpain job:battle-aco - char:-Brainpain- job:ice/thunder-wizzard - char:Brainpa|n job:agi-swordman"`
  — `https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791` (published 2004-01-18; high).
- **[READ] euRO Official Server News Page (euro-ro.net/server/, 2004-01-08 / 2004-02-04):**
  Under "New Comodo Update | January 8th, 2004":
  `"Prontera Tourism Office is proud to announce the successful update of euro-ro to New Comodo.`
  `Prontera (pna) - The Prontera Tourism Office today announced the successful update of euro-ro to New Comodo.`
  `New in 'New Comodo' (among others):`
  `Added Heal-Effect, Modified Potions`
  `Added 2nd Job Quest`
  `Added Monster Skills`
  `Added lvl99-Aura`
  `Fixed Refining Bug`
  `Adjusted Monsters HP/Exp"`
  — `https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/` (captured 2004-02-07; high).
- **[READ] Closed Beta Launch with Comodo + Q-Pet (bhi-oe.de FAQ, 2003-12-12 / 2003-12-20):**
  Imdat Solak (PerrI) wrote:
  `"Wir starten 'Closed Beta' mit Comodo und werden sehr, sehr schnell alles nachziehen. Gravity wird zukünftige neue Features/Updates auf dem Testserver einspielen, dann (nach Tests) auf kRO laufen lassen und etwa einen Monat später allen anderen zur Verfügung stellen - incl. euRO."`
  — `https://web.archive.org/web/20031220035317/http://www.bhi-oe.de/` (published 2003-12-12/20; high).

### 2. Pre-Quest 2-1 Job Change NPCs in Athena 659 / 817 (2004-01-01 / 2004-02-19)
- **[READ] Knight Job Change Script (`scripts/import/npc/jobs/knight.lua`, from 817:jobs/knight.txt):**
  Lines 46-52:
  `"His Majesty Tristram 3rd has been encouraging people to be Knights without any difficult requirements except Job Level 40 or above. Anyway according to the Rumor, people say another requirements would be added soon when the Society become stabilized. If it were so,you got to become a Knight now. Please keep in mind."`
  Checks `player.class == 1` (Swordman), `player.job_level >= 40`, and instantly executes `job_change(7)`.
  — `scripts/import/npc/jobs/knight.lua` (high).
- **[READ] Assassin Job Change Script (`scripts/import/npc/jobs/assassin.lua`, from 817:jobs/assassin.txt):**
  Lines 45-50:
  `"Hmm...You can't make the Living only with Robbery. We, Assasins are Guiding Lights of Thiefs who reached the Highest Limit,being at Job Level 40 or above. For now we just stay out of People's Sight... In no time,we will enlighten the Assassin Clan. And then It will be no good to try to become one of us, just because of our fame. We will restrict people to join."`
  Checks `player.class == 6` (Thief), `player.job_level >= 40`, and instantly executes `job_change(12)`.
  — `scripts/import/npc/jobs/assassin.lua` (high).
- **[READ] Blacksmith Job Change Script (`scripts/import/npc/jobs/blacksmith.lua`, from 817:jobs/blacksmith.txt):**
  Lines 45-52:
  `"Actually Rune-Midgard is an Underdevelopment Country in the Metal Industry! That's why the Govenment encourage people to join us,Blachsmiths more and more. To make our nation an Advanced Country, we establish the requirements of Blacksmiths only Job Level 40! We are full with Energy to revive the Industry! You will take many advantage of being a Blacksmith! It may not look as much,but our Guild has the power,you know! There is no Later for us! Muhahaha... There is no Later for us, Alright?"`
  Checks `player.class == 5` (Merchant), `player.job_level >= 40`, and instantly executes `job_change(10)`.
  — `scripts/import/npc/jobs/blacksmith.lua` (high).
- **[READ] Wizard Job Change Script (`scripts/import/npc/jobs/wizard.lua`, from 659:jobs/npc_wizard.txt):**
  Lines 46-52:
  `"To Utilise the Magical Power of Geffen, Rune-Midgard encourages Mages to Grow Wizards. Why don't you apply on the Wizard if you reach at Job Level 40? At this Time Geffen is at the height of its Magical prosperity.Also this is the Good opportunity to become a Wizard. There won't be a Tomorrow for Mages. Dpn't forget, A chance like this comes but once in a lifetime."`
  Checks `player.class == 2` (Mage), `player.job_level >= 40`, and instantly executes `job_change(9)`.
  — `scripts/import/npc/jobs/wizard.lua` (high).
- **[READ] Hunter Job Change Script (`scripts/import/npc/jobs/hunter.lua`, from 817:jobs/hunter.txt):**
  Lines 51-57:
  `"It seems the King needs more Hunters! So he established a new constitution of Hunters, so you can be a Hunter anytime if you reach at Job Level 40 or above. This is the good chance to become a Hunter! I assume the Government will introduce many Strict requirements for Hunter later if there are too many people want to be Hunters! You can't miss the perfect chance!"`
  Checks `player.class == 3` (Archer), `player.job_level >= 40`, `player.skill_point == 0`, and instantly executes `job_change(11)`.
  — `scripts/import/npc/jobs/hunter.lua` (high).

### 3. iRO 2-1 Job Quest Implementation Timeline (iRO, 2003-10 to 2003-12)
- **[READ] iRO Announcement of 2-1 Job Quests on Sakray (October 2003, newslist page 289):**
  `"Dear Users,`
  `We are happy to announce that we are implementing the 2-1 job quests for the Knight and Assassin class on the Sakray server. The update will be during our next maintenance on Tuesday, October 7th, 2003.`
  `The patch will be first tested on the Sakray server for bug testing. We ask that all users please report any bugs or issues that may be encountered during this time for the job quests.`
  `After we have fixed any possible bugs with the Knight and Assassin quests, we will implement another set of job quests for the other 2-1 job classes. Again, please make sure to report and possible bugs or found issues while attempting the 2-1 job quests so that we may include all the 2-1 job quests as quickly as possible.`
  `Once all job quests have finished testing, they will be implemented on the Chaos and Loki servers. Good luck and happy hunting!`
  `iRO GM Team, Gravity Interactive LLC."`
  — `http://renewal.playragnarok.com/news/newslist.aspx?page=289` (published 2003-10-03; high).
- **[READ] iRO Sakray Slip Notice seq 213 (2003-10-07):**
  `"Dear users, We apologize, but due to unforeseen circumstances. We ran into some technical difficulties and were not able to update the Sakray server with the 2-1 job quest update. We will be working hard towards bringing you the next update as soon as possible. Again we apologize for the delay. Thank you for your understanding. iRO GM Team, Gravity Interactive LLC."`
  — `https://web.archive.org/web/20031010001233/http://iro.ragnarokonline.com:80/allim/content.asp?bbs=tNotice&seq=213&page=1` (published 2003-10-07; high).
- **[READ] iRO Sakray 2-1 Job Change Script Patch (Late October 2003, newslist page 288):**
  Notice between Halloween event announcements (October 26 – November 2, 2003):
  `"Hello iRO Users, During the server maintenance the following issues were fixed... Sakray. Stuck Characters. Database Update, fixed item drop bug related to headgear quests. Script Update, new headgear NPC's and 2-1 job change scripts. Updated patch client to a newer and faster patch client. Fixed a bug that locked up users computers when attempting to log into the game. Thank you, iRO GM Team, Gravity Interactive LLC."`
  — `http://renewal.playragnarok.com/news/newslist.aspx?page=288` (published late Oct 2003; high).
- **[READ] iRO Live Deployment with Turtle Island (2003-12-16):**
  iRO Wiki Updates Changelog:
  `"## Turtle Island Changes (December 16, 2003)"`
  `"- Job change quests (Knight, Assassin, Hunter)"`
  `"- Monster skills"`
  `"- War of Emperium around now"`
  — `https://irowiki.org/wiki/Updates_Changelog` (high).
  Official iRO Announcement (page 287):
  `"We would like to announce that addition of Turtle Island to the Chaos and Loki servers on December 16th 2003. Turtle Island will add a new map as well as a series of new dungeons. This update will also include brand new headgears and new quests."`
  — `http://renewal.playragnarok.com/news/newslist.aspx?page=287` (published Dec 2003; high).

### 4. kRO Patch History for 2-1 Quests (kRO, 2002–2004)
- **[READ] kRO 2-1 Job NPC Dialogue Update (2003-07-29):**
  kRO regular maintenance notice:
  `":- 2-1 차 전직 NPC 대사 중 일부를 수정하였습니다."` ("Fixed some of the dialogue of the 2-1 job change NPCs.")
  — `RO Patch (2003 Jul. 29)`, Fandom / ro.gnjoy.com notice seq 826 (published 2003-07-29; high).
- **[READ] kRO Wizard Job Change Test Bugfix (2004-02-10):**
  `":- 위저드 전직 시험시 다음 시험단계로 넘어갈 때 회복이 되지 않던 문제점을 수정하였습니다."` ("Fixed an issue where HP/SP was not recovered when proceeding to the next test stage during the Wizard job change test.")
  — `RO Patch (2004 Feb. 10)`, Fandom / ro.gnjoy.com notice seq 1079 (published 2004-02-10; high).

---

## Contradictions and open points

1. **Phased rollout of remaining 2-1 job quests on iRO / euRO:**
   - iRO launched Turtle Island on 2003-12-16 with Knight, Assassin, and Hunter quests.
   - euRO's New Comodo announcement on 2004-01-08 / 18 stated "added 2nd job quest" and Italian "wizard, priest, etc". Player Condor noted Priest was buggy on Jan 18.
   - In eAthena 817 (2004-02-19), Priest and Hunter have full quests, Wizard is an unfinished conversion, and Knight, Assassin, Blacksmith retain simple NPCs in the base repository.
   - It remains open whether Blacksmith and Wizard quests on euRO were fully operational in January 2004 or had similar initial script bugs like Priest.
2. **Exact Date of New Comodo deployment on euRO:**
   - Website news dates the New Comodo update to **January 8, 2004**.
   - Forum post `t=1791` by PerrI is timestamped **January 18, 2004**.
   - Client patch files show `2004-01-09gdata_g.gpf` and `2004-01-15gdata_g_b.gpf`. The patch was likely applied server-side on Jan 8/9 and announced/discussed on the forum around Jan 18.

---

## Sources read

1. `https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791` — Primary official forum post by euRO Director PerrI announcing New Comodo features and player replies. Highly reliable.
2. `https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/` — Primary official website news archive for euRO server updates (New Comodo on Jan 8, 2004, Closed Beta on Dec 20, 2003). Highly reliable.
3. `https://web.archive.org/web/20031220035317/http://www.bhi-oe.de/` — Primary FAQ for euRO launch by BHI-OE management (Dec 12/20, 2003). Highly reliable.
4. `https://web.archive.org/web/20031010001233/http://iro.ragnarokonline.com:80/allim/content.asp?bbs=tNotice&seq=213&page=1` — Primary official iRO Notice seq 213 (2003-10-07) detailing the Sakray 2-1 job quest update delay. Highly reliable.
5. `http://renewal.playragnarok.com/news/newslist.aspx?page=289` — Primary official iRO news archive detailing 2-1 job quests planned for Knight/Assassin on Sakray (2003-10-03). Highly reliable.
6. `http://renewal.playragnarok.com/news/newslist.aspx?page=288` — Primary official iRO news archive confirming Sakray deployment of 2-1 job change scripts in late October 2003. Highly reliable.
7. `http://renewal.playragnarok.com/news/newslist.aspx?page=287` — Primary official iRO news archive announcing Turtle Island on Chaos/Loki for Dec 16, 2003. Highly reliable.
8. `https://irowiki.org/wiki/Updates_Changelog` — Historical wiki compiling iRO official patch logs; confirms Turtle Island (2003-12-16) added Knight, Assassin, Hunter job quests. High reliability.
9. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Jul._29)&prop=wikitext&format=json` — Primary kRO patch notes (ro.gnjoy.com notice seq 826) showing 2-1 job NPC dialogue updates on 2003-07-29. Highly reliable.
10. `https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Feb._10)&prop=wikitext&format=json` — Primary kRO patch notes (ro.gnjoy.com notice seq 1079) showing Wizard job change test bugfix. Highly reliable.
11. `https://forums.consolewars.de/threads/euro.12662/` — Contemporary fan report (2003-12-04) from ragnarok.gamona.de announcing euRO closed beta on Dec 20, 2003 running Comodo. High reliability.
12. `https://www.supernature-forum.de/threads/tipp-mmorpg-ragnarok-online-europa-euro.33620/` — Contemporary player post (2004-02-15) recounting closed beta player counts and timeline. Medium-high reliability.
13. `scripts/import/npc/jobs/*.lua` (derived from eAthena 817 / Athena 659) — Contemporary emulator scripts preserving verbatim pre-quest Aegis dialogues and mechanics. Highly reliable.

---

## Could not read

- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs` — HTTP 403 (Cloudflare bot check). Expected content: period user-written class guides. Not needed due to direct access to official notices and emulator scripts.

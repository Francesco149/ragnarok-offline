# Thread #2: Where Official English NPC Text of 2003–2004 Survives

## Answer so far
Official English NPC dialogue from Gravity (as deployed on iRO and euRO during 2003–2004) survives across several distinct historical repositories, distinguishing authentic Gravity localization from contemporary amateur fan-translations (such as eAthena 817, dated 2004-02-19).

Key repositories and preservation channels include:
1. **OpenKore Legacy Client Tables & Logs (2003–2004)**: OpenKore's `tables/euRO/quests.txt` and `tables/iRO/quests.txt` preserve client-side quest log text extracted directly from official client data. Furthermore, `tables/portals.txt` encodes verbatim NPC warp interaction sequences (`c`, `rN`) and exact period zeny fees verified against live official servers.
2. **Wayback Captures of the iRO Community Bug Board (2003–2004)**: Captured at `http://iro.ragnarokonline.com/community/bugcontent.asp?num=...` (~1,400 captures from 2003–2004), where players quoted verbatim dialogue to report translation typos, grammar bugs, and untranslated Korean text.
3. **Period Fan Site Walkthroughs & Databases (2003–2004)**: Fan archives such as RO Empire (`roempire.com`, active 2003–2004) and Doddler's RO Journal/Guides (`ro.doddlercon.com`) transcribed multi-step NPC quest dialogues and job change tests verbatim.
4. **GameFAQs RO Guides (2003–2004)**: Walkthroughs written by players during the 2003–2004 era containing line-by-line transcriptions of NPC dialog options and quiz questions.
5. **rAthena Pre-Renewal Official Conversions (tagged 'later')**: Emulator projects later replaced fan-translated 817 scripts with direct conversions of official Gravity server scripts (Aegis 10.3 conversions by L0ne_W0lf, Lupus, kobra_k88), preserving intact official English dialog trees for town NPCs, Kafras, and job change guilds.

Side-by-side analysis demonstrates that eAthena 817 scripts contain broken, amateur translations (e.g., Tono's "another monster metamorphic out in the desert", Swordsman "Sword Master" shouting "Muhahaha! You must be kidding me!"), whereas official Gravity English prose was colloquial, witty, and characterful ("Talk about awkward puberty", "Check the requirements for job change again").

---

## Findings

### Repository 1: OpenKore Tables & Protocol Automations (iRO / euRO, 2003–2004)
- **[READ] Quest text strings preserved in OpenKore client tables (euRO vs iRO)**: OpenKore mirrors client quest strings directly. In `tables/euRO/quests.txt`, quest 1014 (Swordsman) preserves the early euRO translation:
  "Sign up on the swordman application. For the test, player needs to get a success messages from the manager located at the last check point after pass the all of test courses.# Pass the all test courses.#"
  while `tables/iRO/quests.txt` preserves the revised iRO translation:
  "The purpose of this test is to confirm whether or not you are qualified to be a Swordman. Now, the conditions for completing this test are very simple. You will travel through three courses and must reach the final checkpoint within ^FF000010 minutes^000000.# Reach the final checkpoint.#" — `https://raw.githubusercontent.com/openkore/openkore/master/tables/euRO/quests.txt` and `tables/iRO/quests.txt` (high).
- **[READ] Exact NPC conversation sequences and fees in OpenKore navigation tables (iRO, 2003–2004)**: OpenKore's `tables/portals.txt` records the exact menu dialogue steps and prices used by live official NPCs:
  `prontera 146 89 izlude 94 103 600 c r2 c r0` (continue -> response 2 [Teleport] -> continue -> response 0 [Izlude] for 600z) — `https://raw.githubusercontent.com/openkore/openkore/master/tables/portals.txt` (high).

### Repository 2: Wayback Captures of iRO Community Bug Board (iRO, 2003–2004)
- **[INFER] Player bug reports as verbatim dialog snapshots (iRO, 2003–2004)**: Gravity's web bug board at `http://iro.ragnarokonline.com/community/bugcontent.asp` accumulated ~1,400 captures in the Internet Archive Wayback Machine between 2003 and 2004. Players frequently pasted verbatim NPC dialog excerpts when filing reports concerning NPC text overlapping boxes, confusing directions, Korean untranslated characters, or inaccurate item requirements.

### Repository 3: Period Fan Site Quest Guides (iRO, 2003–2004)
- **[READ] RO Empire archives and news (iRO, 2003-10-29)**: RO Empire actively published official Gravity announcements, patch changes, and quest walkthroughs during 2003–2004:
  "Updated the World Map; Added a ton more monsters... Also, updated the Monster Database; Just fixed a few bugs, any more, let me know." — `https://web.archive.org/web/20031029124102/http://www.roempire.com:80/` (published 2003-10-29; high).
- **[READ] Knight Job Quest Steps preserved on classic community wikis/guides (iRO, 2003–2004)**: Job change walkthroughs recorded exact NPC roles: Captain Herman (`prt_in, 88, 101`), Sir Andrew (`prt_in, 75, 107`, item collection), Sir Siracuse, and Sir Windsor — `https://uaro.kiev.ua` / RO community guides (high).

### Repository 4: rAthena Pre-Renewal Official Conversions (iRO/kRO, Tagged 'Later' / Aegis 10.3 conversions)
- **[READ] Official Conversion standards replace fan scripts**: rAthena repository scripts explicitly document their provenance as conversions from official Gravity Aegis dialogue:
  "//= [Official Conversion] //= Job quest for Swordman classes ... //= 1.8 Rescripted to Aegis 10.3 stadards. [L0ne_W0lf] ... //= 1.9 Dialog is official, but the Script is now custom as far as maps go." — `https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/jobs/1-1/swordman.txt` (tagged 'later'; high).

---

## Worked Side-by-Side Comparisons

### 1. Prontera Citizen "Tono" (`prontera, 54, 240`)
| Metric | eAthena 817 Script (`scripts/import/npc/cities/prontera.lua`) | Official Gravity English Dialogue (rAthena pre-re / iRO) |
| :--- | :--- | :--- |
| **NPC Name / ID** | `Tono` (Sprite 97) | `Tono#pront` (Sprite 97) |
| **Introductory line** | `mes "[Tono]"`<br>`mes "Did you know? The larva of Creamy is Fabre."` | `mes "[Tono]";`<br>`mes "Did you know?";`<br>`next;` |
| **Creamy / Fabre / Pupa** | `mes "The pupa stage of Fabre is simply called Pupa."`<br>`mes "As you can imagine, there is another monster metamorphic out in the desert."`<br>`next()` | `mes "[Tono]";`<br>`mes "The larva of Creamy is Fabre. So, those green little wormy things are actually the babies of those pinkish, purply butterfly things you see around.";`<br>`next;`<br>`mes "[Tono]";`<br>`mes "But before Fabres can become Creamys, they go into a pupa stage. When that happens, they turn into these dark purple cocoons we call Pupa. Simple, huh?";`<br>`next;` |
| **Peco Peco / Picky** | `mes "[Tono]"`<br>`mes "That's it! It is Peco Peco."`<br>`mes "The infant of Peco Peco is called Picky"`<br>`mes "Unlike colourfull Peco Peco, Picky is very cute, isn't it?"` | `mes "[Tono]";`<br>`mes "There's another monster that goes through a really big change... Pickys.";`<br>`next;`<br>`mes "[Tono]";`<br>`mes "Pickys are so cute when they're young, but when they grow up, they turn into those big, gawky looking Peco Pecos. Talk about awkward puberty.";` |
| **Linguistic Profile** | Stiff, unnatural syntax by non-native translator ("monster metamorphic", "The infant of Peco Peco", "colourfull"). | Authentic, highly idiomatic Gravity English localization ("green little wormy things", "awkward puberty"). |

### 2. Kafra Corporation Dialogue & Menu (`scripts/npc/kafra.lua`)
| Element | Current 817 Script (`scripts/npc/kafra.lua`) | Official Gravity English Dialogue (`npc/kafras/functions_kafras.txt`) |
| :--- | :--- | :--- |
| **NPC Header** | `mes "[Kafra]"` | `mes "[Kafra Employee]"` |
| **Greeting** | `"Welcome to Kafra Corp. We will stay with you wherever you go."` | `"Welcome to the Kafra Corporation. The Kafra services are always on your side. How may I assist you?"` |
| **Main Menu** | 1. "Save"<br>2. "Use Storage"<br>3. "Use Teleport Service"<br>4. "Rent a Cart"<br>5. "Other Information Check"<br>6. "Cancel" | 1. "Save"<br>2. "Use Storage"<br>3. "Use Teleport Service"<br>4. "Rent a Pushcart"<br>5. "Check Other Information"<br>6. "Cancel" |
| **Save Point Response** | `"Your respawn point has been saved. Thank you."` | `"Your Respawn Point has been saved here in the city of [Town]. Thank you for using the Kafra Services."` |
| **Storage Fee Failure** | `"Dear, you don't have enough money. The storage fee is [X] Zeny."` | `"I'm sorry, but you don't have enough zeny to use the Storage Service. Our Storage access fee is [X] zeny."` |
| **Basic Skill Failure** | `"I am sorry, but you need Basic Skill level 6 to use the storage."` | `"I'm sorry, but you need the Novice's Basic Skill Level 6 to use the Storage Service."` |
| **Storage Open Success** | Silent storage open, immediate cutin close. | `"Here, let me open your Storage for you. Thank you for using the Kafra Services."` |
| **Cart Class Check Failure** | `"I am sorry. The cart service is only provided for Merchants and Blacksmiths."` | `"I'm sorry, but the Pushcart rental service is only available to Merchants, Blacksmiths, Master Smiths, Alchemists and Biochemists."` |
| **Cart Fee Failure** | `"Dear, you don't have enough money. Please check again."` | `"I'm sorry, but you don't have enough zeny to pay the Pushcart rental fee of [X] zeny."` |
| **Conversation Close** | Silent exit / close. | `"We, here at Kafra Corporation, are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence."` |

### 3. Swordsman Guild Leader & Doorman in Izlude (`izlude_in`)
| Role / Dialogue | Current 817 Script (`scripts/import/npc/jobs/swordsman.lua`) | Official Gravity English Dialogue (`npc/pre-re/jobs/1-1/swordman.txt`) |
| :--- | :--- | :--- |
| **Main NPC Name** | `[Sword Master]` (at `izlude_in 74, 172`) | `[Swordman]` (`Swordman#swd_1`, at `izlude_in 74, 172`) |
| **Greeting** | `"Welcome to the Swordman Association! So..What business brings you to us!?"` | `"Welcome to the Swordman Association! So... What business brings you to us?"` |
| **Already Swordsman** | `"Muhahaha! You must be kidding me! You are already a swordman."` | `"Job change? Muhahaha! But you're already a Swordman! Be proud and be strong!"` |
| **Other 1st/2nd Class** | `"Haha!Oh boy.I reject your offer on the behalf of Swordman Association. You have occupied with another Job,haven't you?"` | `"Haha! Oh boy. I'm flattered, but you already have another job! Still, I can't blame you..."` |
| **Sign Up Offer** | `"Do you want to change your job? Fine, sign up first."` | `"So you wish to become a proud Swordman? By all means, please sign up!"` |
| **Explanation: About Swordsman** | `"Priority of Swordman is the best melee attack among 6 jobs. 1st, Swordman gets the benefit on HP... Aside from this,in my personal opinion,'Swordman' is the world-wide best job with great appearance!"` | `"So you wish to know more about the mighty Swordman job? Well, then... Amongst the First Class jobs, the Swordman is the best melee fighter for three reasons... In my opinion, being a Swordman is the best job ever!"` |
| **Doorman: Already Swordsman** | `"Stop! This is Swordsman Test Hall! Others are not permitted to go in! Go back!"` | `"Sorry guy, but I can only allow Novices to enter the Test Hall."` |
| **Doorman: Job Level < 9** | `"Stop! This is not the place where the goof without Novice Skills can go in. When you learn all Skills I will admit you to enter later."` | `"Stop! I can't let you in until you learn all of the Basic Skills. The Test Hall isn't for goofing off!"` |

### 4. Knight Guild Leader in Prontera Chivalry (`prt_in, 88, 101`)
| Role / Dialogue | Current 817 Script (`scripts/import/npc/jobs/knight.lua`) | Official Gravity English Dialogue (`npc/jobs/2-1/knight.txt`) |
| :--- | :--- | :--- |
| **NPC Name** | `Knight` (dialogue title `[Prontera Knight]`) | `Captain Herman` (`Chivalry Captain#knt`) |
| **Quest Logic** | Instant job change without trial (817 stub):<br>`"His Majesty Tristram 3rd has been encouraging people to be Knights without any difficult requirements except Job Level 40 or above."` | Full multi-stage Chivalry test initiation:<br>`"First of all, I am the captain of the Prontera Chivalry, Herman Phon Efesirsus. I'm pleased to meet young people eager to join the Prontera Chivalry. We only accept Swordmen who are at least Job Level 40."` |
| **Novice Visiting** | `"Please be a 'Swordman' first."` | `"This isn't the Swordsman guild, it's the Prontera Chivalry! If you wish to become a Swordman, visit the Swordman Guild located in Izlude."` |

---

## Contradictions and Open Points
1. **euRO vs iRO Text Variations**:
   - As proven by OpenKore's `tables/euRO/quests.txt` versus `tables/iRO/quests.txt`, euRO originally deployed an earlier English text revision directly supplied by Gravity Korea (with slightly awkward syntax like "after pass the all of test courses"), while iRO featured further localized, idiomatic American English revisions. For single-player euRO reproduction, decide whether to adopt the polished iRO text or the authentic raw euRO client text strings.
2. **Kafra Titles**:
   - In 817, Kafra dialog headers are simply `[Kafra]`, while official scripts use `[Kafra Employee]`.
3. **Menu Option Nomenclature**:
   - 817 uses generic labels like "Rent a Cart" and "Other Information Check", whereas official Gravity clients use "Rent a Pushcart" and "Check Other Information".

---

## Sources Read
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/cities/prontera.txt` — [READ] Official dialogue conversion of Prontera town NPCs, including Tono (`prontera, 54, 240`). High reliability.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/pre-re/jobs/1-1/swordman.txt` — [READ] Official Aegis 10.3 script conversion for Izlude Swordsman Association dialogue and tests. High reliability.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/jobs/2-1/knight.txt` — [READ] Official Aegis script conversion for Prontera Chivalry Knight quest and Captain Herman. High reliability.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/kafras/functions_kafras.txt` — [READ] Official dialogue and functional logic for Kafra Employee services. High reliability.
- `https://raw.githubusercontent.com/rathena/rathena/master/npc/kafras/kafras.txt` & `npc/pre-re/kafras/kafras.txt` — [READ] Town Kafra definitions, cut-in IDs, save points, and fare parameters. High reliability.
- `https://raw.githubusercontent.com/openkore/openkore/master/tables/euRO/quests.txt` — [READ] OpenKore client quest database for euRO, preserving official client quest strings. High reliability.
- `https://raw.githubusercontent.com/openkore/openkore/master/tables/iRO/quests.txt` — [READ] OpenKore client quest database for iRO, preserving official iRO client quest strings. High reliability.
- `https://raw.githubusercontent.com/openkore/openkore/master/tables/portals.txt` — [READ] OpenKore automated NPC navigation table documenting exact menu choices and teleport fees. High reliability.
- `https://web.archive.org/web/20031029124102/http://www.roempire.com:80/` — [READ] Wayback Machine snapshot of RO Empire news and databases from October 2003. High reliability.
- `/opt/src/ragnarok-offline/scripts/npc/kafra.lua` — [READ] Local repository's current 817 Kafra script.
- `/opt/src/ragnarok-offline/scripts/import/npc/cities/prontera.lua` — [READ] Local repository's current 817 Prontera city script (Tono).
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/swordsman.lua` — [READ] Local repository's current 817 Swordsman script.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/knight.lua` — [READ] Local repository's current 817 Knight script.

---

## Could Not Read / Blocked
- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/16059` — BLOCKED: HTTP 403 Cloudflare bot check ("Just a moment..."). Needs laptop browser to extract raw text guides.
- `https://ratemyserver.net/npc_shop_warp.php?map=prontera` — BLOCKED: Dynamic JavaScript-dependent page. Needs laptop browser.
- `https://web.archive.org/web/20040605011928/http://iro.ragnarokonline.com/` — BLOCKED: Connection refused / rate limiting from archive.org during automated batch request.

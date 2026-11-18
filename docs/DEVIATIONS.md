# Deliberate deviations

The target is euRO as a player met it in its closed and open beta,
December 2003 to February 2004, on this CD's client. The baseline,
following research/notes/euro-history.md §7, is the "New Comodo" content
state: 1st and 2-1 jobs, no Juno, 1x rates, Gravity's stock behaviour.

This file lists every place where we knowingly do something else, and why.
There are two kinds:

* **Single player.** Things that need other people are stubbed, or reworked
  into something that makes sense alone.
* **Sources.** Our period sources (Athena-family emulators of 2003–04) are
  evidence of the official game, not the game itself. Where they are clearly
  wrong, we fix them here and say so. Gaps that aren't decisions (things not
  built yet) are in [ROADMAP.md](ROADMAP.md).

How sure we are of each value is in [PROVENANCE.md](PROVENANCE.md). Open
questions that could change an entry here are in
[research/THREADS.md](../research/THREADS.md).

## Single player

| Area | Period behaviour | Here | Why / how to change |
|---|---|---|---|
| Parties | /organize, /join, EXP share | Not available; `party_name()` is "" | Nobody to party with. Party-only quests: none imported so far |
| Guilds, War of Emperium | Guild castles, flags, treasure, guild dungeons (WoE from open beta, Sundays) | A castle for one (`scripts/npc/castle_lord.lua`). An ownerless castle holds the monsters 817 called into ownerless castles at WoE start (`scripts/npc/castles.lua`) and its Emperium; break the Emperium and the castle is yours: the steward (commerce and defence investment, guardians, a Kafra, the master room), the daily treasure, the lever into the guild dungeon. Sieges come on Sunday 18:00-20:00 (euRO's WoE hour, the server's local clock; `@siege` starts or ends one): the garrison comes back to attack every castle you hold, goes for its Emperium and fights your guardians; if they break it the castle is theirs again, and you may take it back. Give a castle up and the monsters come back. **Ours:** no guild is needed to hurt the Emperium (817 wants one with Official Guild Approval); an ownerless castle can be taken at any time; the attackers are monsters (the castle's own garrison), not other guilds; they set out at random moments over the siege's first hour and take the castle's own portals from section to section, as a player does (Athena's monsters never take a portal; without them only 2% to 17% of a castle's cells lead to its Emperium); monsters fight each other only in a siege; the castle belongs to the character and the announcements name it where they named the guild; defence gives +2000 HP a level to the Emperium and to a guardian (eAthena, 2004-06-05, later; no DEF); the day's treasure is laid out when the master first comes in that day, not at midnight; `prtg_cas02`'s Emperium stands in this client's older inner room (193,181); three guardian cells moved off walls; each castle's flag on its grounds (817's cells, 0.5.2's text) names the character where it named the guild and its master, and takes the master home to a cell beside the steward (0.5.2's were copied between regions); the master recovers HP and SP twice as fast in the castle (the iRO page says faster; twice is eAthena's, 2004-06-05, later), and keeps that pace until leaving if a siege takes the castle while the master is in it. No town flags (0.5.2 put them in the towns under castle names this client does not know) | Needs guilds and opponents (THREADS.md #17: rework for one player, owner 2026-11-10; the counter-siege agreed 2026-11-14). Period parts and their sources are in PROVENANCE.md and the script's header. `castle_monsters = false` leaves the castles without monsters (the Emperium still stands; sieges bring no attackers) |
| PvP | PvP rooms (Prontera/Izlude) | The imported PvP NPCs are off (`[content] pvp = false`) | Empty rooms. Turn it on to visit them |
| Trading, vending to players | Trade window, merchant shops | No other players. The client's buttons do nothing | — |
| Chat rooms | Player chat rooms; NPC "waiting rooms" (arena, PvP queues) | NPC rooms are drawn: the bubble with its title over the NPC (817's PvP ushers). Nobody is ever seated, so a room with limit 0 answers "The Room is already full." like Athena, and so does any other (none in our scripts); its event never fires. Player rooms are not offered | Player rooms are pointless. A queue that leads somewhere is reworked by hand (the Wizard test's lounge) |
| Whisper, friends | — | Whispers get "not online" | — |
| Announcements | Server-wide broadcasts | Shown to you (`announce` and friends) | — |
| Bard and Dancer songs (`jobs2_2`) | A song touches everyone in its square but the one playing it (jAthena //1020 skill.c:4528); ensembles need a Bard and a Dancer side by side | **Ours:** the one playing stands in their own song too, so a lone Bard's Whistle or Dancer's Humming does something; Service For You and Apple of Idun, party songs there, reach everyone who is no enemy (with no party, that is the player). rAthena's later Soul Link gives a Bard or Dancer the same. The ensembles, which need a Bard and a Dancer of one party side by side, are played by one player alone at their own level (jAthena //1003's `player_skill_partner_check: no`) | Nobody else to sing for. `battle_config.song_on_self = false` (`scripts/rules/00_const.lua`) gives the period rule |
| B.S. Sacramenti | Needs two Acolytes or Priests standing left and right of the casting Priest, with 10 SP each (the iRO guide's Priest page, 2004-03-27; Skygor's iRO guide, GameFAQs 28518; jAthena //1020 `skill_check_pc_partner`) | **Ours:** a Priest casts it alone and pays the caster's 20 SP (mod0631 and 817 check no helpers; jAthena's `player_skill_partner_check: no`, as the ensembles) | Nobody else to stand beside you |
| Arena (time attack) | Not open: kRO added the Izlude arena on 2004-10-12 | 817's arena (entered from mid-Prontera, its own placement) is imported but off | `arena = true` to visit it. THREADS.md R5 |

## Meta and convenience

| Area | Period | Here | Why |
|---|---|---|---|
| Accounts | Website registration, CD code, e-mail | First login creates the account; `_M`/`_F` suffix picks sex | No website. Athena-style auto-register |
| GM rights | Players had none; GMs were euRO staff | New accounts are GM 99: all `@commands` | It's your server. Set `new_account_gm_level = 0` in `ragnarok-offline.toml` for a period account |
| Start point | Novice training ground (official map, entry cell unknown) | `new_zone01 53,111`, Athena's `new_1-1` entry on this client's older-named map | THREADS.md #6 |
| Rates | Closed beta: "higher experience" (BHI-OE, 2003-12-20); 3x EXP and drops according to one player's forum post of 2004-01-27, never confirmed. Open beta: 1x | 1x | THREADS.md #1. For the closed beta as that player describes it, set `base_exp_rate`, `job_exp_rate` and `drop_rate` to 3 in the config |
| Character slots | 3 (5 from 2004-08-18) | The client shows 9; the server accepts all 9 | Not enforced yet |
| Patcher | euRO's patch servers | A local stand-in serving an archived snapshot | docs/WINDOWS.md |
| Display | Real fullscreen mode switch | Emulated fullscreen (DDrawCompat) | docs/WINDOWS.md |

## Content switches

`[game.content]` in `ragnarok-offline.toml`. Defaults follow euRO in
Jan–Feb 2004.

| Switch | Default | What |
|---|---|---|
| `juno` | off | Juno town NPCs and Kafras (euRO: 2004-06-09) |
| `jobs2_2` | off | Crusader, Monk, Sage, Rogue, Alchemist, Bard/Dancer job quests (euRO: 2004-06-09) |
| `pvp` | off | PvP room NPCs |
| `events` | off | Seasonal and national events from 817: Ice Cream Maker (Aug 2003), Mr. Smile, Bunny Band festival |
| `arena` | off | 817's monster time-attack arena and its entrance NPC in Prontera (kRO opened the arena, in Izlude, on 2004-10-12) |
| `castle_monsters` | on | Monsters in the guild castles, which have no owner (kRO until 2004-02-10; 817) |

## NPC scripts: what we changed relative to the period emulators

The imported NPCs (`scripts/import/npc/`) are a faithful translation of
eAthena 817 (2004-02-19), bugs included. The translator is
`tools/import/athena_npc.py`. Fixes live in hand-written files under
`scripts/npc/`, which replace imported NPCs by their `src` tag.

* **Not imported at all.** These are emulator conveniences that no official
  server had:
  * the instant job changer "Jobra", the Reset Girl, the "Universal Rental"
    NPC (cart/falcon/peco for anyone), free healers, the warper, the card
    remover, the MVP arena, the Platinum Skill NPC (all quest skills at once);
  * fan-made item quests ("Magic Pimp Hat Dude", "Iron Cane Quest", "Lord
    Kaho's Servant", the "Daily Job Quest" glasses and elven ears, the
    Excalibur quest, the Royal Messenger and the Quest Assistant);
  * the "FGM Kafra" in Prontera (made for the private "FGM RO" server);
  * the 77-palette dyer, which needs a client patch.

  The list, with reasons, is in `SKIP`/`SKIP_NPC` of the translator.
* **Japanese-only events** (Momotaro, the Gonryun tournament, Valentine) are
  not imported.
* **Kafra** (`scripts/npc/kafra.lua` replaces 18 of 817's town and dungeon
  Kafras and adds 6 it did not have):
  * Fares are the ones 817's menus *showed*. Its code charged other fares in
    places: Prontera→Geffen showed 1400 and took 1800. mod0631's Kafra
    script of December 2003 shows and takes the menu fares.
  * From mod0631 (`conf/npc_town_kafra.txt`), because 817 had less: the
    storage fee per place (30 to 80z; 817 took 30 everywhere), Comodo as a
    destination from Prontera (2000z) and Morroc (1500z), Al De Baran's
    teleports (Geffen 1600z, Mt. Mjolnir 1700z), and the Kafras of Comodo
    (town, inn, Pharos beach), Mt. Mjolnir and Prontera's east and west
    gates. Their wording is 817's; mod0631's text is Japanese. mod0631
    follows jRO: research/notes/round1-review.md.
  * A cart costs 800z only when you get one. 817 took the money twice and
    then jumped to a missing label, so no cart.
  * Storage needs Basic Skill 6, as the client's own Basic Skill description
    says ("[Level 6] : Enable to use a Storage"). 817 asked for Novice job
    level 6.
  * No "Use Kafra Pass" entry. kRO removed the voucher menu on 2003-05-20,
    and 817's was "Under development".
  * "Cancel" and "Other Information Check" work (817 pointed them at missing
    labels). The location tip marks the town's Kafras.
  * Kafra points ("special reserve") are earned: a tenth of every teleport
    fare, the rule in mod0631's script. 817's "Special Reserve Check" said
    "Under development"; here it tells you the total, in 817's words.
  * Izlude's "Save inside City" saved nothing in 817. It saves at izlude
    94,103, as in mod0631 and jAthena (2004-01).
  * Typos in the Kafra lines are fixed. The wording is otherwise 817's.
* **The special reserve counter** in the Al De Baran head office
  (`aldeba_in 79,161`) is 817's NPC with code behind it. In 817 it was a
  prize list and nothing else: the menu labels start with digits, which 817
  parsed as numbers, and every prize was a bare `close`. Here it takes the
  points and gives the Sweet Potatoes (100p to 900p) or Red Potions (1100p
  to 2800p) of 817's list. The two "Lottery Chance" entries are left out:
  817 names no prize. That the official counter gave these goods at these
  prices rests on 817 alone (THREADS.md #4).
  A second window across the hall (ours) sells the Kafra free tickets the
  client describes, which every Kafra takes in place of a fee (below,
  "Content the client ships").
* **Thief quest skills** (`scripts/npc/thief_skills.lua`). 817 had four
  separate NPCs in Morroc, of which only Stone Fling's ("Sono-san") loaded,
  and none for Sand Attack. mod0631 has one NPC for all four in the Thief
  guild under the Pyramid ("アルコスク", `moc_prydb1 154,128`) and a pouch
  maker in Payon; the client ships the Sand Attack quest's two items, and
  RODataZone's Thief page of 2003-12-04 describes the same NPC on the same
  cell for the international servers. We use mod0631's NPCs, places, items
  and job levels, and remove Sono-san. The names "Alcouskou" and "Bag
  Seller" are RODataZone's; the dialogue is our translation of mod0631's
  Japanese, not Gravity's text.
* **Novice quest skills** (`scripts/npc/novice_skills.lua`). 817 has no
  First Aid or Play Dead quest. mod0631 has both: Nami in a Prontera inn
  ("ナミ", `prt_in 234,133`), a member of the Chivalry (`prt_in 73,87`) and
  the nursing officer in Prontera Castle (`prt_castle 175,146`).
  RODataZone's Novice page of 2003-12-15 has the same three NPCs on the
  same cells for the international servers. We use mod0631's NPCs, places,
  items and steps, with three changes. First Aid starts at Novice job level
  4, as mod0631's own dialogue and RODataZone say; its code tests for 3.
  The Newbie Tag is the client's item 7039, as on RODataZone; mod0631 keeps
  it in a temporary variable. The ten minutes for Play Dead's errand are
  counted: mod0631's NPC announces them ("if you are late you will have to
  go again") and never checks the clock. The NPC names are RODataZone's;
  the dialogue is our translation of mod0631's Japanese, not Gravity's
  text.
* **The training ground where 817's script is broken**
  (`scripts/npc/novice_result.lua`). 817's script is a rough English port
  of the one mod0631 has in Japanese (`conf/npc_job_newbie.txt`). Where it
  contradicts itself or cannot be played, mod0631's logic is used, with
  817's English:
  * *The last Helper* (`new_zone04 60,149`). "I will follow the result of
    the test" said "Under development. Please choose your job manualy."; it
    now sends you to one of the six job guide rooms of the map, where an
    attendant gives what the guild sent (Swordman five Red Potions, Archer
    three and 50z, Thief a Leather Jacket, Mage 100z, Acolyte three Red
    Potions and a Club, Merchant 100z) and passes you on to the town. The
    subsidy was scrambled (the lowest score got the most, the highest lost
    500z); it is mod0631's table: up to 8 points 100z and a Red Herb, up
    to 15 150z and three, 16 300z and five, more 500z and ten, a Milk each
    time.
  * *The points.* The first course's were never counted ("1st course: 0 /
    6 Points" for everyone); each right answer to Leo Von Frisch now counts
    one, as in mod0631, told by the prize he hands over, the first time
    through only. The second course's fell with a counter 817 never moved;
    they are ten less one for each time the Fabres knocked you out.
  * *The forest Helper* (`new_zone02 16,183`) saved you inside the forest,
    against his neighbour's "you will lose your points and come back here";
    he saves beside himself (`23,188`), counts the tries, and offers to
    give the forest up at the fifth try and from the tenth (no points
    then).
  * *Leo Von Frisch's last question* asks for the key and the command to
    sit and rewarded "'delete' - /stand"; it rewards "'insert' - /Sit"
    (mod0631, and the client's own key). This one is a source fix in the
    importer. He also said nothing at all between his lesson and the NPC
    Master's (a jump to a missing label), and a lesson cut short after the
    last question could never be finished; he has 817's own lines for
    both now.
  * *The Guide in the hall* (`new_zone02 100,113`) told everyone who had
    not finished "finished the First Course? meet the guy at the
    southwest": 817 strung his four tests together without the answers
    between them. He now says, in 817's own lines, where to go next.
  The attendants and the giving-up have no English in any source, so
  theirs is our translation. Unconfirmed for an international server: the
  gifts and the table are jRO's as mod0631 has them. Left as 817 has them,
  though mod0631 differs (THREADS.md #6): two of Leo's questions, his
  prizes' amounts and his Red Herbs (always four), the Instructor's Milk,
  one answer of the first personality test.
* **The Merchant test's parcels** (`scripts/npc/merchant_delivery.lua`,
  and source fixes in the importer). 817's port of the test could not be
  finished by one applicant in four: Byalan's Kafra (`izlu2dun 106,58`)
  asks a variable nothing sets whether you bring a parcel, reads the
  training ground's counter for the serial number, and hands out two items
  that do not exist where mod0631 (`conf/npc_town_kafra.txt:1735`) moves
  the quest on; and Kay could never change a parcel with the wrong number
  ("no box" was written so that it was true for everyone), so the only way
  on was to report it lost, fail, and pay the fee again. Mended from
  mod0631, in 817's words: the Kafras of Byalan and Prontera take the box
  of the errand they were sent and no other (both tested the box and went
  on whatever the answer), Byalan's takes the Chief's letter; Kay changes
  a parcel; Geffen's Guild Staff wants the errand *and* its box (817: or);
  the Chief pays the 100z for the letter when it was delivered (817: to
  everyone sent to Byalan), and being 500z short no longer ends his script
  in mid-air (a misspelt label). Prontera's Kafra, off the errand, sends
  you to Izlude for 900z only if you have them (817 took them unchecked);
  Byalan's, off the errand, gives the one Kafra service of
  `scripts/npc/kafra.lua` (817's own there charged a cart twice), storage
  at mod0631's 35z.
* **The Thief test's weighing.** 817's Comrade told a novice with Orange
  Net Mushrooms and no Orange Gooey ones that the basket was empty (nine
  Net ones are 27 points, a pass); only an empty basket is refused now, as
  in mod0631 (`conf/npc_job_thief.txt:269-270`). A source fix in the
  importer.
* **The Archer test's trunks.** 817's Guildsman took one trunk of each
  kind and left the rest to sell (`delitem` without an amount), and read
  a passing score out as "0 / 40" (the wrong variable). He takes them all,
  as jAthena's script of 2004-03-02 does, and reads the score. Source
  fixes in the importer.
* **The Swordman course's pits** (`scripts/npc/swordman_course.lua`, and
  source fixes). Three of 817's pits on the second stretch sit on a cell
  another pit already has, or reach too far (typing slips: jAthena's list
  of 2004-03-02 has the same pits with the cells one would expect), and
  the four short ones that close the strips at x 130 to 134 are missing,
  so there one could leave the path and not fall. Both are as in
  jAthena's now. The pit at `job_sword1 17,251` no longer tells the faller
  a bare digit (a leftover `announce`), and the Sword Master says goodbye
  on "Cancel." (the menu named a label that does not exist).
* **Monster Tamers** (`scripts/npc/monster_tamers.lua`; the Rocker
  Glasses' 400z, which our first port of them dropped, since 2026-11-14): three NPCs from
  mod0631 that 817 lacks, in Alberta, Izlude and Al De Baran, where the iRO
  pet guide of 2004 says they stood. They turn ingredients into taming
  items and pet accessories. The guide says "available for purchase";
  mod0631's take no zeny, and we follow mod0631. The English is ours, one
  template for all three.
* **Dyes** (`scripts/npc/dyes.lua`). Nothing in 817 makes a dye, though
  its hat quests ask for them and its Merchant quest names "JavaDullihan,
  the Dyermaker in Morroc". mod0631 has the whole chain, and we take it:
  Louitz in Alberta and Aure Dupon in Geffen lead to the chemist
  Morgenshtein (Counteragent 3000z, Mixture 4000z), JavaDullihan in Morroc
  (`morocc_in 146,99`) makes the eight dyes for 3000z to 7000z, and
  Prontera has a clothes dyer who is "too busy" (`prt_in 284,168`) and a
  hairdresser (`prt_in 243,168`). Where 817 has English for the same lines
  it is used; the rest is our translation. Two things are not mod0631's:
  * the hairdresser dyes hair. mod0631's names the terms (one dye of the
    colour you want and 5000z) and then says "not implemented"; here she
    does it. Which palette is which colour is read from the client's files;
  * showing Louitz the solutions a second time does not make the chemist
    forget you (mod0631 sets the quest back to its first step);
  * the Violet dye takes the 30 Red Herbs mod0631's own line names (its
    code takes 20; Ragu Memo's jRO list and RODataZone's iRO one say 30).
* **The Priest test** (`scripts/npc/priest_test.lua`) is jAthena's of
  2004-06-08 (`script/npc/job/npc_job_priest.txt`, //1013), four months
  after our period: a pilgrimage to the three ascetics of the Acolyte
  test, an exorcism on `job_prist` (thirteen Zombies named for failings
  before a gate, 350 seconds; four demons who tempt you to give up; a
  last room of Mummies) and a vow taken before Sister Cecil; a Book on
  changing job. Until 2026-11-11 the game played 817's, which its author
  made up ("Designed For Athena": a true-or-false quiz and a grail carried
  three times over `job_priest`). The test came with "added 2nd job
  quest" in January 2004 and `job_prist` with the Turtle Island patch the
  same month (THREADS.md #8), and jAthena's is the nearest script of it.
  Ours: the English (retold from the Japanese, cut to the window); one
  player, so no Priest along with a Rosary and no waiting for another
  candidate; the Mummies rise in the last room and its door (a warp tile
  "to 0,0" in mod0631 and, commented out, in jAthena) leads to the church
  door, where jAthena reset the counter that called them and left a
  candidate to Teleport out; a demon speaks once per attempt; the clock
  stops once you are off the map; Yosuke saves you where 817 does
  (jAthena's cell is a rock on this map); `job_prist` takes the flags 817
  gave its own test map, logging out putting you at the church door.
* **The Wizard test** (`scripts/npc/wizard_test.lua`). 817 ships the
  test (things to bring, ten questions, three rooms of monsters on
  `job_wiz`) as a half-converted script it could not run, and was played
  with 659's one-line guild NPC, as we were. The map is in the CD's Turtle
  Island patch file (THREADS.md #8), so the test is in: 817's steps, words,
  questions, monsters and times, with eAthena 0.5.2 and jAthena's finished
  versions of the same script consulted for what 817's half leaves open.
  What is not theirs:
  * nobody queues in the waiting room: the Helper lets you in when you
    ask, or you tell him you would like to lounge a bit first, and from
    then on he only asks whether you are ready (the official test called
    candidates one at a time out of an NPC chat room);
  * mod0631's ten warp tiles on `job_wiz` are removed (they lead "to
    0,0", a random cell; they are the rooms' doors, and jAthena removed
    them too when it added the test);
  * `job_wiz` has map flags no period source gives it: no teleport, no
    memo, no Dead Branch, and logging out there puts you in Geffen. The
    rooms have no doors, so without them a Fly Wing skips the guards and
    a logout leaves you in a sealed room;
  * the elemental stones are all needed (the three scripts' test ends in
    an "and" that lets four stones of one kind through), Cartlin takes the
    Apple Juice before her hint (817 and 0.5.2 jump to a missing label),
    wind on Water 3 is 200% (817 marks 175%), and several monsters dying
    to one spell clear a room once, not once each;
  * the Fire room's guards get 817's two minutes; 0.5.2 and jAthena give
    one. Pages are cut to the dialogue window.
* **The Hunter test** (`scripts/npc/hunter_test.lua`). 817 changes an
  Archer into a Hunter at job level 40 with no test, as iRO did before
  Turtle Island; an iRO player's walkthrough of 2004-02-15 (THREADS.md #8)
  describes the test that came with the island, and eAthena 1.0's script
  of 2004-11-04 ("converted from aegis script") is that test: Sharon's
  interview at the front desk, the Demon Hunter's items, the guild
  master's 200 Silver Arrows in the Archer Guild, and the trap room on
  `job_hunte` (four Job Change Monsters to kill among thirty decoys, 381
  traps, a switch, the exit at the far end, three minutes); a Necklace of
  Wisdom, and a Cross Bow (a Hunter Bow from job level 50) on changing
  job. Where they part, the walkthrough decides: its three item lists (not
  eAthena's five), the decoys' names ("Hunter Change Monster", "Job Test
  Monster" and the like, as rAthena's later copy has them; eAthena calls
  them all "Test Monster"), the switch's "escape" button, and three
  minutes for the whole room (eAthena gives 30 seconds after the switch).
  Ours: one player, so no chat room (the Guide lets you in or lets you
  lounge, as in the Wizard test); one guild master, in the Archer Guild
  (the walkthrough also names one "at the central palace", `payon_in03`,
  which this client lacks); mod0631's two warp tiles on `job_hunte`
  ("to 0,0") are removed; a decoy, a trap or the switch's "Re-test" sends
  you back to the waiting room and clears the room; pages cut to the
  window, eAthena's jokes for other jobs shortened.
* **The Knight test** (`scripts/npc/knight_test.lua`). 817 changes a
  Swordman into a Knight at job level 40 with no test (its NPC says
  "another requirements would be added soon"). The test here is
  jAthena's of 2004-09-12, the nearest copy of the official script: the
  captain's application (job level 40, no skill points left), Sir
  Andrew's six items, Sir James's eight questions (all right), Sir
  Windsor's three rooms on `job_knt` (12, 12 and 7 monsters, three minutes
  each), Lady Amy's ten (eight to pass), Sir Edmond's room where you must
  kill nothing for 270 seconds, Sir Grey's talk, the seven knights'
  verdict and seven Awakening Potions. No account dates it before
  September 2004 (THREADS.md #8): it is here because `job_knt` came with
  the January 2004 patch, as the Hunter's map did. Where jAthena is
  unfinished: one item list (it names three, fills one); Sir Grey always
  passes, as its code does ("pass condition unknown"); its "not
  researched" lines are short pointers to the next knight; Sir Edmond's
  plants stand on eAthena 1.0's cells (jAthena's are walls on this map).
  Ours: no chat room (Sir Windsor starts the fight when asked, or lets you
  lounge); the verdict says "this one" where jAthena's text is written
  for a woman; one who falls in Sir Edmond's room does not pass when its
  timer runs out elsewhere (jAthena's player timer would pass them);
  mod0631's four warp tiles on `job_knt` ("to 0,0") are removed;
  dialogue in our English of jAthena's Japanese, cut to the window, with
  eAthena 1.0's English names. Sir Grey makes Knights a Claymore for
  74,000 zeny and a Steel, as both scripts have it.
* **The Blacksmith test** (`scripts/npc/blacksmith_test.lua`). 817 changes
  a Merchant into a Blacksmith at job level 40 with no test, as mod0631's
  guild member does ("conditions will come with an update"). The test here
  is eAthena 1.0's of 2004-11-04, a translation of the official script and
  the only complete one on disk: Altiregen's application in Geffen (job
  level 40), Geshupenschte's ten questions in Alberta (all right, or nine
  on a retry), one of his five orders (three materials and a weapon from a
  store), the delivery to that order's customer in Geffen, Morroc, Izlude,
  Payon or Comodo and the receipt back, Mitmayer's five questions in Morroc
  (four right) and her Hammer of Blacksmiths, then the change (no skill
  points left) and five Steel, ten from job level 50. jAthena's script of
  2004-09-12 has the same NPCs, question sets and two of the orders, but
  changes you unconditionally "for lack of data"; where it differs
  (THREADS.md #8) we keep eAthena: its pass marks (jAthena: 90, 80 and
  Mitmayer 100), its level 4 refining question (jAthena counts +6 for a
  level 3 weapon) and the Trunk of the Arbalest order (jAthena: an Eye
  Patch). No account dates the test before autumn 2004: it is here because
  the other 2-1 tests are. Ours: Tilpitz on jAthena's `payon 151,39`
  (eAthena's cell is in the rebuilt Payon, off this client's map); jAthena's
  Ring Pommel Saber for eAthena's Buster in one question; eAthena's slips
  mended (Geshupenschte checked for a Knife instead of the Gladius he made;
  three customers ran on into their next line); dialogue cut to the window.
* **The Assassin test** (`scripts/npc/assassin_test.lua`). 817 changes a
  Thief into an Assassin at job level 40 with no test, as mod0631's guild
  member does (the requirements "will come with an update"); jAthena's
  script of 2004-09-12 is a stub too. The test here is eAthena 1.0's of
  2004-11-04 ("Based of Official RO Assassin jobchange quest", some
  dialogue from the Aegis script), the only one on disk, all on
  `in_moc_16`: the angry-looking man at the door (Huey; job level 40), Kai
  who hides and then takes your form, the Nameless One's quiz (one of
  three sets of ten, nine right; Kai's tips after a failure), Barcardi's
  test in two parts (six Job Change Targets among 34 decoys and 83 traps,
  then a room of Mummies and Hydras to sneak through without a kill; three
  minutes each, a failure in either starts both again), the Guild Master
  behind his maze (the turtle patch's `in_moc_16` has the maze,
  data.grf.original's an open hall), a dagger or katar (a choice from job
  level 50) and the Necklace of Oblivion, which Huey looks at and lets you
  keep (no skill points left). No account dates the test: it is here
  because the other 2-1 tests are. Ours: the guild's door on `moc_fild16`
  leads into the entrance hall where Huey stands, as in eAthena 1.0 and
  later (mod0631's led into Kai's room), with eAthena's way out of the
  hall; mod0631's ten warp tiles between the test's rooms are removed (the
  test moves you); no chat room (Barcardi sends you in when asked, or lets
  you lounge), and no "busy with another candidate"; eAthena's slips
  mended (a decoy that never spawned, the pass warp that came again 30
  seconds later wherever you were, the first room's way out open before
  any test, the hide room's exit passing you in the seconds after a kill
  had failed you, guild members left standing in the Guild Master's room,
  Huey starting you over after the second test, Kai and the Nameless One
  serving anyone); the Nameless One's tip to a fan web site goes; three of
  the guild members called to the Guild Master stand on the nearest floor
  (eAthena's cells are walls here); dialogue cut to the window.
* **The Alchemist test** (`scripts/npc/alchemist_test.lua`, behind
  `jobs2_2`; test `alchemist_test`). 817 changes a Merchant into an
  Alchemist at job level 40 with no test, and no warp leads into the
  guild hall (`alde_alche`) in any period source of ours. The test here is
  jAthena's of 2004-09-12 (`npc_job_18alchemist.txt`, whole), with the
  hall's eighteen warps from jAthena //0966 (2004-05-07) and eAthena 1.0's
  iRO names: Parmy Gianino's application (job level 40, 50,000 Zeny, then
  100 Portable Furnaces or 7 Berserk Potions, or an Old Magic Book and a
  Hammer of Blacksmith instead), Raspuchin Gregory's ten sums (nine right,
  eight on a second try; all thirty agree with our item prices, weights
  and DEF), Darwin's first potion, Van Helmont's two questions on what
  Morgenshtein taught (`dyes.lua`: no lesson, no pass), Nicholas Flamel's
  magic square, the brothers Bain and Bajin in Juno, and Vincent
  Carsciallo's welcome with an Alcohol Making Book; an Illusion Flower
  given to Darwin skips from Gregory to the end; job level 50 skips the
  fee and Gregory (jAthena marks both, and the flower's last line, as its
  author's guesses). Gever Al Sharp sells Medicine Bowls and the making
  books at jAthena's prices. Ours: the brothers take the 5 Portable
  Furnaces and 2,000 Zeny they ask for (jAthena reads out 5, checks for 3
  and takes no Zeny; eAthena 1.0 takes 5 and the Zeny), and they replace
  817's Bain and Bajin (another errand: a Pair of Sandals for 200,000
  Zeny) while `jobs2_2` is on, standing in Juno even with `juno` off; Van
  Helmont says Morgenshtein is in Geffen; Nicholas's "do not take too
  long" has no timer (jAthena has none); dialogue in our English of
  jAthena's Japanese, cut to the window. No account dates the test before
  September 2004: for the 2-2 version, like the rest of `jobs2_2`.
* **The Dancer test** (`scripts/npc/dancer_test.lua`, behind `jobs2_2`;
  test `dancer_test`). 817's Performance Master makes a female Archer a
  Dancer at job level 40 at once; now he sends her to Comodo's dance
  school and keeps the Bards (jAthena's Bard test is a stub). The test is
  jAthena's of 2004-09-12 (`npc_job_20dancer.txt`, whole) with rAthena's
  iRO names (Bor Robin, Aile, Bijou, Pyorgin; tier later) and jAthena's
  way out of `job_duncer` (2004-03-02): Aile's application (job level 40),
  the 10,000 Zeny fee with 5 Earthworm Peelings and Boots shown and kept
  (jAthena and iRO's script take only the Zeny), Bijou's interview (one of
  two sets of ten questions; 80 points, 70 on a retry; naming yourself the
  best dancer costs 100), and a one-minute audition on the stage: Bijou
  calls the steps; a few seconds after each call the step sensors other
  than the called one open, and stepping on one before the next call puts
  you out; the Dance Poring of the last call, shot (any kill), passes you;
  then the job and a Rope (a Line at job level 50). Ours: no waiting room
  (jAthena's chat room for a queue): the usher sends you to the stage
  when you say you are ready, or lets you lounge first, as the Wizard
  test's Helper; a wrong step ends the audition at once, so you can try
  again (jAthena lets the minute run on for the next in line); the first
  set's third question asks which response is *not* right, as iRO's script
  has it (jAthena's "what do you do?" makes "point out the mistake" the one
  right answer); the "middle" step's sensors, opened in the same second as
  the "hold the pose" call that would close them, stay open until the next
  step (jAthena leaves it to its timers' order); the casino manager is
  Martine as in jAthena (iRO's script says Moo; 817's casino has both, with
  no lines); dialogue in our English of jAthena's Japanese. The client
  needs `effect\ring_red.tga` for the stage lights (next section).
* **The Sage test** (`scripts/npc/sage_test.lua`, behind `jobs2_2`; test
  `sage_test`). 817's Cleitos Berdo makes a Mage of job level 40 a Sage at
  once. The test is jAthena's of 2004-09-12 (`npc_job_16sage.txt`, whole)
  with //1071's refusal of unspent skill points (2004-12-04), its arena's
  mapflags, and rAthena's iRO names (Kayron Grik, Metheus Sylphe, Claytos
  Verdo, Hermes Tris, Saphien Layless, Lucius Celsus, Aebecee George; tier
  later): enrolment with Metheus (job level 40; 70,000 Zeny, or an Old
  Magic Book and a Necklace of Wisdom, or 30,000 Zeny with fifty each of
  Feather of Birds, Fluff, Clover and Feather; free at job level 50);
  Claytos' written test (one of three sets of twenty, 80 to pass); Hermes'
  practical test in `job_sage` (sixteen Chonchons and Fabres in three
  minutes, twenty-four Lunatics in three, then the one Whisper of five that
  is the "Bachelor's Degree" in one); then a course chosen by a score
  jAthena marks as its own guess (10, +1 for full marks and each fight won
  fast, -1 or -2 for each won late; 12 sends you to elemental magic): Lucius
  Celsus' monsters (two errands of fish and insect loot, picked at random,
  two questions) or Aebecee George's elements (30 Stones for three
  elemental stones, 50 each of Crystal, Stone and Wind arrows, a Holy
  Water); either way the thesis' five materials and a Book, which the Dean
  reads. Ours: no waiting room (the Test Helper sends you in when you are
  ready, or lets you lounge, and can send you back to the Academy; jAthena
  says to reconnect); the fee waiver asks for both items it names (jAthena
  takes either and deletes both); the third fight's "late" mark reads its
  own clock (jAthena reads the second fight's); the elemental thesis writes
  the seventh line you chose (jAthena repeats the sixth's); the quest
  variables cleared at the job change; five of the sixty questions changed
  to hold in this game: the Cute Ribbon's bonus is MaxSP +10 (jAthena's
  item db: +20), Safety Wall at level 6 costs 35 SP (40 in jAthena; ours
  and the client's SP table say 35), Double Attack is asked by its chance
  alone (ours doubles the blow; jAthena says 140%), Magnus Exorcismus'
  "not needed" offers Lex Divina for Heal (mod0631's tree needs neither
  Heal nor Divine Protection), and two people this game lacks give way to
  their towns' neighbours (Morroc's Sado to Cetsu, Aldebaran's RS125 to
  Chemirre). 817's academy staff (behind `juno`) give way to jAthena's;
  dialogue in our English of jAthena's Japanese. No account dates the test
  before September 2004: for the 2-2 version, like the rest of `jobs2_2`.
* **The Crusader test** (`scripts/npc/crusader_test.lua`, behind
  `jobs2_2`; test `crusader_test`). 817's Crusader Trainer made a
  Swordman of job level 40 a Crusader at once. The test is eAthena 1.0's
  of 2004-11-04 (`npc/jobs/2-2/crusader.txt`, "just a temp Job Quest"
  from RagnaInfo), set right where the nearest player's account, Edge's
  of 2004-03-16 (GameFAQs 29457 v1.2), says otherwise, with Tangowr's
  names (GameFAQs 28552, 2004-07) and jAthena //1046's Master Crusader and
  guard in the castle's west wing: Michael Halig asks one of Edge's two
  sets of forty items (job level 50 skips them, as eAthena; the Pike he
  gives is then the slotted one); Man in Pain (Murnak Mijoul) in the
  castle prison sends whoever wears a Rosary (Edge; eAthena: carries one)
  into a corridor of `job_cru` to cross without a kill; Gabriel Valentine
  in the cathedral asks ten questions, 8 to pass, five of eAthena's
  twenty on a Crusader's conduct and five of Tangowr's nine on the undead
  and demons (Edge: "questions about Crusaders and Undead monsters"),
  with the choices worded as answers; Harrison, the Guardian Knight, takes
  a Holy Water and gives five minutes to clear a hall of 43 undead (Edge
  both; eAthena neither). Ours: the corridor holds Tangowr's Mandragoras,
  Floras, Greatest Generals and Familiars on eAthena's cells, not its
  undead up to level 63, which killed a job level 40 Swordman at the door
  in the client, and whose ids (0.5.2's G_ copies) this client cannot draw
  (it closed); two of Tangowr's answers read against our data (Holy deals
  25% more than Fire to a level 2 Undead, not 50%; the Whisper for a
  "Jirtas" this game lacks); eAthena's slips mended (an eleventh question,
  a monster spawned zero times, the hall's count lost at a death, the
  rooms left full after a failure); no waiting for others. Dialogue is
  eAthena's and ours, jAthena's master and guard in our English of their
  Japanese. Edge's account (March 2004, a server that had the 2-2 jobs)
  is the earliest; euRO had none before June 2004.
* **Alberta's sailor** (`scripts/npc/alberta_sailor.lua`) no longer sells
  passage to Amatsu and Gonryun: the client has neither map (they are later
  additions 817 took from jAthena), so the fare went and the ship did not.
  Nor to Turtle Island (next entry).
* **Turtle Island** (`scripts/npc/turtle_island.lua`). 817's sailor took
  anyone to the mouth of the island's cave for 800z and nobody brought
  them back. mod0631 has a chain "from jRO", and we take it: an old man in
  Alberta's inn (`alberta_in 23,104`) sends you to the sailor Gotanblue
  (`alberta 247,122`), who asks 10000z for the trip to the beach and waits
  there for the return; a logbook on the quay (`alberta 248,94`) starts
  one of two errands on the island, for five Bananas or a Red Gemstone.
  The words are ours, a short retelling, and the names our reading of the
  katakana. One line is mended: mod0631's rock lacks a `goto`, so its
  Bananas could never be had. As in mod0631, the logbook can be read
  again after an errand, and the errand done again. The 10000z is the
  Japanese servers' fare, kept as the best figure we have; what the
  international servers asked is not confirmed (THREADS.md #24).
* **Moving HP Recovery** (`scripts/npc/moving_hp.lua`). 817's knight asks
  for 500 Empty Bottles, a Rocker Doll and a Padded Armour, tests for
  them, and takes 200 Empty Bottles and a Moth Wings, which are the
  Japanese servers' items (mod0631). He now takes the 500 bottles and the
  doll, the list RODataZone gives for the international servers
  (2004-05-02). The armour is tested for, worn or carried, and kept, as an
  iRO player's guide of autumn 2003 says (THREADS.md #7).
* **Charge Arrow and Cart Revolution** (`scripts/npc/skill_quests.lua`).
  Deothen in Payon now wants to see a Crossbow (in the bag or in hand) and
  gives it back; 817 and mod0631 ask for none, three player accounts do
  (idRO 2003-10, pRO 2004-01, RODataZone 2004-04). Gershaun in Alberta
  teaches at once anyone who brings the items, and wants a Banana Juice
  from anyone else, every time, before he names them, as in mod0631 and a
  pRO guide of 2004-01 (817 has no juice). He asks for 20 Iron, not 817's
  15: RODataZone's iRO list of 2003-10 (20 Iron, 30 Sticky Mucus, 20 Fly
  Wings, 5 Tentacles, 2 Grape Juices) is the middle of the three sets
  mod0631 picks from at random; 817's and a pRO guide's lists differ from
  it in the Iron alone (15 and 30). The rest of his words are 817's, cut
  into pages that fit, with its low-level line mended (it named Change
  Cart) (THREADS.md #7).
* **The doomed swords** (`scripts/npc/cursed_swords.lua`). 817 has three
  people who tell of Executioner, Grimtooth and Mysteltain and of a
  blacksmith who left Prontera, and nothing comes of it. mod0631 has the
  rest, and we take it: a man in Prontera's tavern (`prt_in 163,12`) who
  knows where the smith went, and the smith himself east of Payon
  (`pay_fild08 218,283`), who makes each sword from monster drops and an
  Emperium. The rumours are 817's English; the other two are our retelling.
  Three things are not mod0631's:
  * hearing a rumour again, or a second one, does not set the quest back
    (mod0631 writes the first step over whatever was there);
  * the smith takes what he asks for: five Cursed Rubies and ten Blades
    Lost in Darkness for Grimtooth (mod0631 tests for one ruby and takes
    one of each), one Young Twig for Mysteltain (mod0631 takes a hundred);
    a pRO guide of 2004-03-06 (GameFAQs 29037) gives these same lists;
  * his list shows the swords you have heard of.
* **Alberta's doll child** (`scripts/npc/doll_child.lua`). 817's "Kid"
  wishes for a monkey doll and that is all. mod0631's takes any of nine
  dolls and gives a sweet back (a Yoyo Doll brings an Yggdrasil Seed); we
  take that, on 817's cell, with 817's first two pages and our
  translation of the rest.
* **Christopher, Geffen's smith supplier** (`scripts/npc/christopher.lua`).
  817's script ended without a word whenever the zeny was short (its jumps
  name `NoZeny`, its line is under `NoZenys`), could never sell a Mini
  Furnace (its weight test read two variables that are always 0), sold
  Phracon at 200z to anyone buying Emveretarcon, and charged 100z for the
  1000z Iron Hammer. Each is mended as jAthena's script of 2003-10-30 has
  it (//0505, `conf/npc_town_geffen.txt`), whose overweight line we
  translated; 817's wording otherwise.
* **Aldebaran's Trader** (`scripts/npc/aldebaran_trader.lua`). 817's took
  an Apple o' Archer for the Doctor Band where it had counted the Red
  Bandana and never asked for the Cracked Diamond its list names. mod0631
  loads the same NPC (`conf/npc_event_making.txt`) and takes what the list
  says. The Opera Masque takes 20 Iron as 817's code and RODataZone's iRO
  list have it (mod0631 and Ragu Memo, jRO: Steel), and his list now says
  Iron.
* **Izlude's sailor** (`scripts/npc/izlude_sailor.lua`) tests the fare,
  as mod0631's on the same cell does (817's took it untested, so a
  passenger with too little sailed for what he had), and no longer offers
  817's "Turtle Island: Under development".
* **The Arrow Maker of Morroc** (`scripts/npc/arrow_maker.lua`). 817's
  taught Arrow Crafting to any archer who spoke to him (its answers ran
  on into "Congratulations" with no close), asked for Powder of Butterfly
  and took Resin. mod0631, Godhand's idRO FAQ (2003-10-23) and
  RODataZone's iRO Archer page agree on what 817's code took: 20 Resin,
  7 Poison Spores, 41 Pointed Scales, 13 Trunks and a Red Potion, all now
  taken; job level 30 as in mod0631 and RODataZone (817: 35).
* **Alberta's Grampa Pharmacist** (`scripts/npc/grampa_pharmacist.lua`).
  817's port took a White, Blue or Green Potion's herbs, bottle and fee
  and gave nothing when asked for "as many as I can", took Green Potions
  for the Green Potion, went on making potions with no bottles left, and
  ended without a word in three places. mod0631 loads the same NPC
  (`conf/npc_event_potion.txt`, 2003-12-14) with the same recipes and fees
  and a script that works: we run its steps with 817's words, and its two
  lines 817 lacks (no money, too heavy) translated.
* **The Swordman Test Hall Staff** (`scripts/npc/swordman_course.lua`)
  sends anyone who is not a Novice out of the hall with 817's "how did you
  come inside" line; 817 tested for Novices first and ended without a word
  for anyone else (jAthena //0854 has no such test).
* **Peco Peco Breeder** (`scripts/npc/peco_keeper.lua`): mod0631's keeper
  by Prontera's north-west wall (`prontera 55,350`), 2500z a bird for a
  Knight; the Crusaders' keeper (`232,318`) comes with `jobs2_2`. 817 has
  only its "Universal Rental" convenience, which is not imported, so a
  Knight could not get a mount. As in mod0631, a Knight without the Riding
  skill loses the money and gets no bird; the keeper warns of it. The
  English and the name are ours.
* **Commands 817's scripts used but its engine lacked**, where the meaning
  is unmistakable. 817 stopped the script at each of them; we run them:
  * `releaseall` (unequip everything) comes right before `jobchange` in the
    Merchant and Thief quests, so in 817 neither job change ever happened;
  * `scriptlabel` opens every town guide (a jAthena label marker), so no
    817 guide ever spoke. It is a no-op here;
  * `save` is the later alias of `savepoint` (the Prontera inns);
  * `else(cond) goto L` (Thief quest) is read as `if`.

  817's Wizard quest is an unfinished conversion it couldn't run; it is
  finished by hand in `scripts/npc/wizard_test.lua` (below), which replaces
  the Athena 659 guild NPC the import still carries.
* **The Merchant quest's endless loop.** One label in 817's Merchant guild
  script jumps to itself (`R10: if(@TEMP !=0) goto R10;`), so one
  application in eight hung the server. It goes to `R11`, the second serial
  number, like its three sister branches. This is the one fix made in the
  translator (`SOURCE_FIXES`) rather than in an override: the NPC is 350
  lines long. A stray `f` in Charge Arrow's "you again?" line
  (`skillquests/arrowrepel.txt:115`) is dropped the same way, and so is
  the misspelt label of Geffen's Welding Mask smith
  (`quests/headgearquest.txt:82`, `LKeyDork` for `LHeyDork`, which 817's
  other copy of the NPC spells right): short of Steel or zeny, he ended
  without a word. Source fixes too, each with mod0631's NPC on the same
  cell as the second witness: the Clock Tower's gatekeepers on 3F and B3
  tested `countitem(7026) < 0`, which never holds, and so let anyone
  through to 4F and B4; they want a key again, the Key of Clock Tower on
  3F and the Key of Underground on B3 (817 took the Clock Tower key for
  both; mod0631 `conf/npc_cTower.txt`, "From jRO, by Kalen"). Morroc's
  optician (the Binoculars) took 50000z without testing for them, and
  since zeny stops at 0 the Binoculars then cost whatever one had.
  Payon's Granny (Pretend Murdered) closed before her `getitem`, so she
  took four Skirts of Virgin and gave nothing, and she let three through
  her test for four; both mended as mod0631 has her. The training ground's
  Usher pays the 50z his line names (and RODataZone's iRO list), not 817's
  150z (mod0631's, jRO).
  The PvP Usher 31~40's waiting room was titled "LV 41 ~ LV 50", the next
  usher's levels; it reads "LV 31 ~ LV 40" as in mod0631
  (`conf/npc_pvproom.txt:43`) and 817's own copy in the other room.
* **Monsters and warps of the job tests.** The world's monsters and warps
  are mod0631's. 817's job files place some of their own, and those are
  taken where mod0631 has none: the Chonchons along the Swordman course and
  the three warps that bridge its gaps (mod0631 has Fabres on the same
  cells, under a map name with a typo that kept them from loading), and the
  ten Wraiths of the Priest test.
* **First job needs Basic Skill 9** (`scripts/npc/jobs.lua`). The client's
  skill description says so; 817's guild NPCs checked only the job level, so
  a Novice could carry 9 unspent points into the new job. The refusal line is
  ours (official wording: THREADS.md #2).
* **The Orc Dungeon Kafra saves at `gef_fild10 54,320`**, mod0631's cell
  (`npc_town_kafra.txt:4805`). 817 has `54,232`, a hundred cells south and
  inside a cliff on this client's map: a typing slip, found by
  `ragnarok-offline check`.
* **`aldeg_cas05`'s tile at `188,49` lands on `165,228`**
  (`scripts/npc/castle_warps.lua`). mod0631 and 817 both have `165,288`, off
  the 240x240 map; every other pair in that castle lands four cells from its
  partner tile (`165,232` here), and rAthena has `228`. (Until 2026-11-14
  `remove_warp` also took out the replacement, so the tile led nowhere.)
* **`prtg_cas02`'s inner room is this client's**: the door at `184,44`
  leads to `189,166` and back from `189,162`, and 817's guards of that room
  stand at `193,179` (`scripts/npc/castle_warps.lua`, `castles.lua`). Every
  emulator has `157,140`/`157,135`/`157,174`, cells of the larger room the
  May-2004 client draws; this client's castle (the CD's patch of
  2003-04-30) has a smaller room at x 184-203, y 162-182, and those cells
  are rock. The cells are ours, laid out as the later map's (research round
  18).
* **"morroc"** in the Juno Kafras is a typo for `morocc`.
* **Shop names** come from 817 by cell (mod0631's are Japanese). "Shop" is
  used where 817 has none within 3 cells, or only a Japanese one.
* **Menus whose target label is missing** end the dialog. 817 jumped to the
  start of the script, which restarted the conversation on "Cancel".
* **`end;` with a message showing** gets a close button, so the window
  doesn't hang.

## Content the client ships but no source scripts

Content the client carries (sprites, item text, maps) that no period
emulator made playable is brought back where a player would find it worth
exploring (owner, 2026-11-14). Each piece is built from the best evidence
there is, marked as ours where it is ours, and listed here.

* **Alberta's two hat makers** (`scripts/npc/alberta_hats.lua`). Turtle
  Island's patch (`2003-04-30gdata_turtle_ph_1.gpf`, euRO's patch entry 400
  of January 2004) ships the sprites of exactly eight hats, and the base
  GRF Gravity's English descriptions of them; no period emulator has an NPC
  that makes them. The Stylish Merchant (`alberta 120,53`: Heart Hair Pin,
  First Moon Hair Pin, Fashionable Glasses, Hair Protector) and the Hat
  Store Girl (`136,79`: Sunday Hat, Mage Hat, Magician Hat, Helm of Solar
  God) take their names, cells and recipes from RODataZone's iRO quest list
  (2004-05) and Ragu Memo's jRO one (2004-02), which agree on every recipe.
  The girl talks only to someone carrying a Yellow Potion (Ragu Memo) and
  keeps it (our reading); "It's really hot, isn't it" leads to her three
  hats, "Give me something instead" to the Helm (Ragu Memo's two answers).
  Ours: all the dialogue, and the sprites (rAthena's, a later script). With
  `events` on, the merchant stands at `121,53`, off the Ice Cream Maker's
  cell. The Helm's Emblem of the Sun God drops only from castle treasure
  boxes, which this game has none of yet (THREADS.md #17), so the Helm
  cannot be made until that is reworked. The hats' stats are mod0631's item
  rows as they were (Fashionable Glasses sit in the upper headgear slot).
* **Four more hat makers** (`scripts/npc/hat_makers.lua`), for eighteen
  hats the client describes and draws: Payon's Young Man (`payon_in01
  122,41`, in the weapon and armour shop: Angel Helm, Hat of Petite
  Diablo), Comodo's Hair Ornament Girl (`comodo 228,159`: Cross Hat, Bulb
  Band, Stripe Band, Blue Hairband) and Traveler (`273,137`: Mine Helmet,
  Parcel Hat, Money Loser's Grief, Phantom of Opera), and Lutie's Vending
  Machine, the snowman at `xmas 115,297` (Raccoon Hat, Spore Hat, Wonder
  Nutshell, Rainbow Eggshell, Blush, Chef Hat, Candle, Cake Hat). Makers,
  cells and recipes: RODataZone's iRO quest list (2004-05) and Ragu Memo's
  jRO list (2004-02), which agree but for the Chef Hat's Feathers (300 and
  330: 330, as rAthena has it too). Dialogue and sprites: rAthena's
  (iRO's later official text, tier later), cut to the client's window. Its
  Titicupe, who tells Lutie's recipes, is left out; the machine shows them
  itself. No period emulator has any of the four. Every material drops in
  this world (the Nut Shell since 2026-11-14, below); the Blush wants 100
  Alice's Aprons, and Alice is one of a kind in Glast Heim's castle. The
  hats' stats and who may wear them are mod0631's item rows as they were
  (the Angel Helm is for second jobs; the Mine Helmet's job list is
  narrower than the client's "All", THREADS.md #31).
* **Drops mod0631 left empty.** mod0631's monster table writes `512,0` (an
  Apple at rate 0) in 95 drop slots; eAthena 817 (2004-01) has an item and
  a rate in each, and RODataZone's iRO table of 2004-02 names about 80 of
  them for the same monster. Those slots take 817's (`tools/import/
  athena_import.py`, `research/mob_drop_holes.py`). Among them three hats
  the client ships and nothing else gave: the Nut Shell (Wild Rose), the
  Poring Hat (Marin) and the Bongun Hat; and the Elunium and Oridecon of
  Baphomet and Phreeoni. The rates are 817's: neither the client nor
  RODataZone gives one. One slot that is not empty is changed too: the
  Sasquatch drops the Panda Cap (817, rate 1) where mod0631 has a Poo Poo
  Hat (rate 7): RODataZone's iRO table and Ragu Memo's jRO page of 2003-08
  (1/5000) give the Panda Cap, and this client describes the Panda Cap
  but not the Poo Poo Hat.
* **English names for items the client cannot name.** The Turtle Island
  patch brought these hats with a sprite and a description but no line in
  `idnum2itemdisplaynametable.txt` ("You got 1"). `ragnarok-offline setup`
  (and the Windows launcher) adds a line for each id in
  `data/client/item_names.tsv` that the client's table lacks, with iRO's
  names as RODataZone gave them (the hats above, and the materials and
  drops a player needs to tell apart); a client already set up is set up
  again.
  Gravity's table is never in the repository: setup appends to the copy in
  your data.grf.
* **A red ring for the textures the client lacks.** `job_duncer` (the
  Dancer test's school, from the Juno patch on the CD) has stage lights
  that load `effect\ring_red.tga`, and so do Volcano's, Assassin Cross of
  Sunset's and Ugly Dance's ground units; this client's data has the blue
  and the yellow ring but not the red one, and stops on a ResourceError box
  at each load of the map (the audition's minute passed behind it). euRO's
  May-2004 data adds the file, and it is the yellow ring turned red: every
  alpha the same (checked against the MICO07 DVD's data.grf). `setup` (and
  the launcher) makes ours from your own yellow ring, its green set to its
  blue, so every pixel's hue is red and its brightness kept (mean error
  about 20 of 255 on blue and green against the May-2004 file, whose own
  colours are noisier; `crates/ro-server/src/setup.rs`); a client already
  set up is set up again. Nothing of Gravity's is in the repository.
* **Kafra free tickets** (`scripts/npc/kafra.lua`). The client describes
  three tickets, "to use Kafra Storage for free at a time", "Kafra
  transportation", "to rent a cart for free" (7059-7061), and no period
  script takes or gives them; it names none of them (817's English names
  are added as above). Ours: a Kafra takes one in place of the fee, and a
  free trip earns no special reserve (rAthena's Kafras have done the same
  since 2006-10-11, tier later); a second window in the Al De Baran head
  office (`aldeba_in 88,161`, the counter's cell mirrored across the hall;
  817's counter says "each window requires a different special reserve")
  sells them for special reserve: 30p storage, 500p a cart, 800p a trip,
  each about its fee at the counter's rate. Left out: the Kafra payroll
  (7056, "Payment statement for the Kafra employees"), which only a quest
  of the 2010s uses (rAthena's War of Emperium training missions).

## Engine behaviour relative to Athena 817

* **No casting bar for a skill the client cannot name.** Athena sends the
  bar (0x013e) for every cast. This client shouts the skill's name over the
  caster and says "Unknown Skill !!" for one that is not in its
  `skillnametable.txt`: every monster-only skill (an Ant Egg hatching) and
  every 2-2 skill. Those casts take their time without the bar. Ours;
  what the official server sent is not known (THREADS.md #23).
* **Step times are the client's.** Athena moves a unit into the next cell
  after half a step, then half of each step plus half of the next. The
  halves are rounded down, so odd step times lose 1 ms a cell, and the
  speed is read again at every step. Here the server walks on the times
  the client gives the path's cells (docs/CLIENT.md, "The walk clock and
  step times"): it crosses into a cell halfway between two cells' times,
  and a walk keeps its speed to the end, since the client goes on drawing
  it at that speed. A new speed counts from the next walk.
* **Ground units of others are told once per map visit, traps excepted.**
  The client draws a second 0x01c9 for a unit placed by someone else
  (not in the player's party) hidden, whatever the unit, until the next
  0x0091 (docs/PROTOCOL.md). Athena takes every unit off the client when it
  leaves sight and sends it again when it comes back, so a monster's Fire
  Wall, Safety Wall or Quagmire seen a second time vanished. Here such a
  unit stays on the client while out of sight and is taken off only when
  it ends. The Hunter's traps are sent again as in Athena and so are drawn
  hidden, which is what iRO's Comodo patch notes describe (THREADS.md #28);
  Detect shows them (0x01ac). Ours; what the official server sent is not
  known.
* **Cell types.** Only types 1 and 5 block, as in Athena (mod0631
  `map.c:1052`, `path.c:153`). One map of this client depends on it: the
  whole floor of `c_tower1` has the type `0x444c74` (some builder's stray
  bytes). Until 2026-11-11 the server took unknown types for walls, and
  Clock Tower 1F could not be entered.

* **Leaving the game dead** brings the character back at its save point
  with the restart HP (the EXP went at death), as mod0631 saved a dead
  character (`pc_makesavestatus`); until 2026-11-14 it came back lying
  where it died, drawn standing by the client and unable to move.
* **A Fly Wing (or Dead Branch) refused by a map flag is not used up.** In
  Athena it was spent before its script ran. The client still shows its
  "Unable to Teleport in this area" message.
* **Pet names lose any `%`.** The client copies its own pet's name with
  `sprintf(name)` (docs/PROTOCOL.md, 0x0095 and 0x01a2), so a `%s` would
  read past the name. mod0631 keeps the name as typed.
* **Runaway scripts** are stopped after 5 million Lua instructions without
  waiting for the player. 817 hung the whole server on an endless loop.

* **Quest skills** (First Aid, Play Dead, the 2-1 quest skills…) can't be
  bought with skill points, only from their NPCs. This matches the official
  servers; 817 shipped with `quest_skill_learn: yes`.
* **Variables.** Athena 817 treated *every* variable whose name starts with
  `l` as temporary. Here only `l<digits>` are temporary: quest variables like
  `lutie_quest` survive logout. Server-wide `$` variables are saved
  (`saves/world.json`); in 817 they were lost on restart.
* **Equipment-look `setlook`, card removal.** Scripts that call these
  continue, and the server logs once that it isn't implemented (ROADMAP).
* **Panacea and Royal Jelly cure poison.** mod0631's item table spells the
  status `SC_Posion` on both, a name it does not know, so there they cured
  everything but poison. The importer reads it as Poison.

## Battle and skills: what we changed relative to the period emulators

The battle rules (`scripts/rules/`) are a port of athena-dev-2.1.1-mod0631
(`map/battle.c`, `skill.c`, `pc.c`, `mob.c`, archived 2003-12-18), the
emulator closest in date to this client. Where it has nothing, eAthena 817
(2004-02-19) fills in; each such place is marked "817:" in the Lua. Both are
evidence of the official game, not the game: which numbers a later round of
research should check is in research/THREADS.md #21–#23 and #25.

**Forging follows the client's own list** (`data\metalprocessitemlist.txt`,
2026-11-14): the Pike (70 Iron) can be forged, as in 817 and jAthena
(mod0631 lacks it); Oridecon and Elunium can no longer be made from rough
ores with the Mini Furnace. All three emulators let Oridecon Research do
that; the client's list has no such recipe and the iRO guide (2004-03-27)
says the skill only raises the success of level 3 weapons. Rough ores go
to the NPCs, as before. The client gives the Lance two Evil Horns where
every emulator asks one; we keep one (text, as in THREADS.md R6).

**Dying costs 1% of the EXP the level needs, taken at death** (2026-11-15),
not 1% of the EXP held, taken at the restart, as mod0631 and 817 do (817 at
death). jAthena built the penalty in //0513 (2003-11-02) in `pc_damage`, the
moment of death, with the setting "100 = the usual, 1% of the EXP needed";
a week later //0532 added `death_penalty_type` and made "the EXP held, at the
restart" the default, its author writing that the restart was his choice
("本鯖の仕様がかなり気にいらなかったので変更しました": he did not like the
real server's way, so a Resurrection would cost nothing). mod0631 carries
that default. For one player nobody comes to resurrect, so the timing shows
only in the EXP bar; the amount is about twice mod0631's on average, and is
no longer nothing just after a level. A player's account of 2003-04 would
confirm it (research/THREADS.md #35). `scripts/rules/combat.lua`
`rules.death_penalty`.

**Slips of mod0631 that are not carried over** (the Lua marks each
"mod0631 bug"):

* **Magnum Break** did 65% damage at every level and distance: C operator
  precedence in `5*skill_lv +(wflag)?65:115`. Here: 115% + 5% a level on the
  target, 65% + 5% a level around it, as the expression was meant.
* **A bow's least damage** was the archer's DEX below the bow's ATK and the
  whole ATK above it, the same kind of precedence slip (`battle.c:914`).
  Here it is ATK x DEX / 100, as RODataZone's iRO combat page has it
  (capture 2003-12-16; 817 mends the line to the same while DEX is at most
  the ATK, `battle.c:2167`). A DEX 60 archer with a 50 ATK bow rolls from
  30, not 50; with DEX 30, from 15, not 30. Above 100 DEX (bonuses
  included) the page's best roll rises too, to (ATK + cards) x DEX / 100,
  which neither emulator has: built since 2026-11-15, on the most after
  the size cut.
* **Faith, Invulnerable Siegfried, Providence** added to the *attack* bonus
  against an element or race (`addele`, `addrace`) where a resistance was
  meant. Here they resist (`subele`, `subrace`).
* **Weaponry Research and Star Crumbs** added their damage to a miss. Here
  only to a hit.
* **The pushcart** slowed its owner only if he had Pushcart, the skill the
  cart cannot be had without. Same result; the condition is gone.
* **Blessing on a monster** lifts Curse and Stone unless it is Undead or a
  demon; mod0631 compares the whole element word with 9, which matches
  nothing.
* **A skill's item** (gemstone, trap) was not used up when it sat in the
  first inventory slot.
* **Ankle Snare** ended a second after any enemy moved within 5 cells of
  it, caught or not, sprung or not (`skill_unit_onout` case 0x91, also in
  817): in a field a snare was gone before anything stepped in, and a
  caught monster went free as soon as another walked by (seen in the
  client, 2026-11-13). Here only the caught one leaving ends it early, as
  jAthena //1046 (2004-09-12) has it.
* **A skill asked for from one cell beyond its range** did nothing: the
  client sends the request from there (Remove Trap, range 2, from 3 cells;
  from farther it walks up first), and mod0631 refused it without a word
  (`skill.c:3237, 3360`). jAthena //1082 (2004-12-18) fixed the request; here
  the cast end allows the same cell (ours: jAthena's cast end would have
  spent the SP and the item and failed).
* **Monsters' element changes** (Myst and Angeling to Ghost, Carat to
  Shadow, Sasquatch to Water...) did nothing: mod0631's `skill_db` marks
  skills 161-169 as damage skills (`nk 0`), so `mob.c:1949` sends them to
  `skill_castend_damage_id`, which has no case for them. jAthena //0850
  (2004-02-29) made them support skills, as here. The elements stay
  mod0631's by number (168 Ghost, 169 Shadow), which its monsters' rows and
  the FAQ below agree with, though its `skill.h` names 168
  NPC_CHANGEDARKNESS.
* **Attack speed.** Not mod0631's slip but ours, fixed now: the server
  attacked twice as fast as the status window's ASPD says. The delay between
  two attacks is twice the "aspd" value (`pc.c:2458`).

**Monster skills iRO had and mod0631 lacks.** mod0631's monster skills
(December 2003) are jRO's as its authors knew them. unknown13's Monster
Skill FAQ (GameFAQs 29608, April 2004) lists what iRO's monsters used: of
its 325 pairs, 97 are not in mod0631's table (no monster there uses
Endure, Increase or Decrease Agility, Piercing Attack, Mental Breaker,
Napalm Beat, Status Recovery or Pneuma). euRO's "New Comodo" update
(January 2004) announced "added monster skills". 93 of them are in
`data/db/mob_skills_faq.toml` (`tools/import/mob_skills_faq.py`), read after
mod0631's: 38 with jAthena's rows of 2004-04-27 (tier athena2003), 8 with
rAthena's (later), and 47 that no table has (later tables were reworked:
rAthena gives Golem Auto Guard, not Endure) with the commonest row of that
skill on other monsters (guess). Which Goblin and Kobold the FAQ means by
"Goblin (Hammer)" and the like is read from the client's sprites. The 4
left out: Lazy Attack (no such skill in the table), Detoxify (no table gives
it to a monster), and two pairs on a "Grimtooth" monster. THREADS.md #23.

**Monster magic as Ragu Memo's jRO table gives it.** Ragu Memo's page of
monster attack magic (`at_monskill.html`, Wayback 2003-12-05) gives rate,
cast and wait for 76 rows: 71 are mod0631's rows exactly
(`research/ragmemo_mob_magic.py`). The other five are rows mod0631 lacks and
the page has, now in `mob_skills_faq.toml` (tier period): Marduk's and the
Wind Ghost's raised rate under 30% HP (the Wind Ghost's Jupitel Thunder then
uninterruptible, as the page says of it), the Mutant Dragon's Fire Ball at
80% when shot from range, Gajomart's Magnum Break at 40%, and the Ancient
Mummy's Envenom Lv10 at 25%. The Elder Willow's Napalm Beat (from jAthena)
takes the page's 1000 ms cast instead of 500.

**The bosses' rule.** The same FAQ ends with what it says it took from iRO:
"All boss monsters will use the Two-Hand Quicken, Increase Agility, Guided
Attack, and Piercing Attack depending on the probability" and "will Teleport
if they are attacked from beyond their attacking range". A boss here is an
MVP (the 22 monsters with MVP EXP; jAthena //0953 gives Two-Hand Quicken to
exactly those but Lord of Death). The four skills went through the same
search: 24 rows from jAthena //0953, 2 from rAthena, 32 guesses (every
Piercing Attack: no table gives one to a boss). Teleport is one row per MVP
on a "rude attack": a hit from beyond its attack range by someone it cannot
walk to within its chase range. That is rAthena's reading of the condition
(`src/map/mob.cpp:1936-1976`, later); read word for word, the FAQ would send
a boss away from every arrow. mod0631 has no such condition, and jAthena's
boss Teleports fire on close hits instead. The FAQ names no chance, so it
always fires.

**Taken from eAthena 817 because mod0631 leaves it unfinished:**

* **Ailments** (stone, frozen, stun, sleep, poison, curse, silence, blind):
  the chance falls with the target's MDEF, VIT, INT or LUK, the length with
  the same stats, monsters keep half the chance at least, and the lengths
  come from 817's `cast_db.txt` (`data/db/skills.toml` `time`, `time2`).
  mod0631 has fixed chances, placeholder lengths ("some length or other" in
  its comments), and no Curse at all (the status is refused).
* **Max HP of the Archer, Acolyte and Thief** use 817's factors (0.5, 0.4,
  0.5), which RODataZone's sub-stats page of 2003-12 (iRO) gives too;
  mod0631's (0.4, 0.504848, 0.4) look shuffled. The page's weights, SP
  factors and every other HP factor agree with ours; its weapon delays
  differ in a dozen places where 817 sides with mod0631, so those stay.
* **Armour refines give two thirds of a DEF point a level** (all worn
  armour together, rounded down), as RODataZone's sub-stats page of
  2003-12 says and kRO's notice of 2003-02 implies; mod0631 gave a whole
  point (THREADS.md #9).
* **A blind monster sees only the cells next to it** when it looks for a
  target or for loot (ours; neither emulator has it): Ragu Memo's page of
  2003 and RODataZone's Monster AI page (iRO, 2004-02) both say so.
* **A looter keeps ten items**, not mod0631's 20 (`LOOTITEM_SIZE`): the
  same RODataZone page says "a maximum of 10 items", and jAthena set 10 on
  2004-02-01 (//0764, research round 22). A full bag still lets its oldest
  item go, as both emulators do.
* **Spear Boomerang reaches 3, 5, 7, 9, 11 cells and Grimtooth 3 to 7 by
  level** (2026-11-17), where mod0631 (and jAthena until //0749 of
  2004-01-27) gives 11 and 7 at every level: 817's skill_db, the iRO
  guide's Knight page (2004-04-13) and Assassin page (2004-03-27), and for
  Spear Boomerang a pRO guide of 2004-08 (research round 30). The skill
  window tells the client the learned level's reach, so it walks close
  enough before it asks (Spring Trap and Shield Boomerang too).
* **Skill reach where mod0631 and 817 part, from rAthena** (2026-11-17,
  tier later; test `bow_skill_reach`). No period source gives these reaches
  (the iRO guide and this client's text are silent); mod0631 has a fixed
  number, 817 the weapon's reach plus something (jAthena too from //0749,
  2004-01-27). rAthena's pre-renewal table (2026-10) decides: Double
  Strafe, Arrow Shower and Charge Arrow 9 cells and one more a level of
  Vulture's Eye (mod0631 13 and 8 whatever the Vulture's Eye; 817 the
  bow's reach + 2, 7 and Vulture's Eye), Mammonite 1 (mod0631 3), Pierce
  and Bowling Bash 2 (mod0631 1, 817 the spear's 3), Brandish Spear 2
  (mod0631 4, 817 1). For the 2-2 skills: Holy Cross, Investigate, Asura
  Strike, Chain Combo and Combo Finish 2, Finger Offensive 9, the Chemical
  Protections 1 and the Sage's fields 2 (the last two 817's too). Bash,
  Sonic Blow, Back Stab and Intimidate stay 1 (mod0631 and rAthena).
  rAthena also agrees with the by-level reaches above.
  **Later the same day, from RODataZone's skill pages** (iRO, captures
  2003-12-04 to 2004-05-02, round 33; tier period, so it takes over from
  rAthena): Double Strafe and Arrow Shower reach "Bow Range" and Charge
  Arrow "Bow Attack Range", so all three reach as far as the bow shoots,
  5 cells and Vulture's Eye's levels (rAthena's 9 was the tie-breaker);
  Envenom ("Melee Range") and Sand Attack ("Melee") 1, where mod0631 and
  817 both give 3 and 8. Its Mammonite "Melee Range" agrees with the 1
  above. Left at the emulators': its "9 tiles" for every spell and Stone
  Fling's 7 and Detoxify's 9 (mod0631 and 817: 8, 8, 10), one cell apart
  (THREADS.md R7).
* **Aspersio strikes Undead monsters** (2026-11-17, test
  `aspersio_strikes_the_undead`): cast on a monster of the Undead element
  it does 40 Holy damage that no MDEF lessens, then the element table as
  any spell ("Using shift-click, you can target an Undead property monster
  and inflict 40 points of Holy property damage that ignores all
  defenses": RODataZone's Priest page, iRO, capture 2003-12-16). jAthena
  added the same on 2004-05-31 (//1002-//1003, `battle.c` "damage = 40",
  skill_db Holy); mod0631 and 817 only bless the target's weapon.
* **A third off the caster's DEF while Brandish Spear, Bowling Bash or Grand
  Cross is cast** (2026-11-17): 817's `cast_def_rate` 33 (`battle.c:505`,
  players and monsters); round 24's preview of kRO's Comodo notes and a pRO
  guide of 2004-08 say "reduced to 2/3". mod0631 has no such rule.
* **The Deleters' Fire Wall and Meteor Storm change places** (2026-11-15):
  the Sky Deleter casts Meteor Storm, the Earth Deleter Fire Wall, as
  jAthena //0743 (2004-01-26) and 817 have them; mod0631 had them the wrong
  way round (`MOB_SKILL_MOVE` in `tools/import/athena_import.py`).
  Likewise mod0631's row "monster summon @ Dracula" (Familiars), filed
  under Lord of Death's id, belongs to Dracula, as 817 has it.
* **Slave summons as jAthena //0743 (2026-11-15)**: jAthena's commit of
  2004-01-26 "to make slave summons and the like resemble the real
  server" (817's rows are the same): a summon is cast for 3 s, and comes
  again 10 s later once every slave is gone, where mod0631's came at once,
  an hour apart, while two or fewer were left. Osiris, Mistress, Drake and
  Eddga gain a second summon (Eddga: five Bigfoot once an hour, five Leib
  Olmai between), the Turtle General and the Archangeling call in steps
  as their slaves thin, Gajomart, Lava Golem, Lord of Death and Dracula
  call slaves rather than free monsters, and Ghostring calls Giant
  Whispers. mod0631's last three Fake Angel rows, filed under the Fur Seal,
  go back to the Fake Angel (the Fur Seal summoned and cursed to stone).
  Each row says its source; `summon_patch_0743` in
  `tools/import/athena_import.py` reads the commit from jAthena's git.
* **Three monster skills mod0631 lists but never built** (2026-11-16),
  found by casting every monster skill row once
  (`every_monster_skill_row_runs`):
  * **Metamorphosis**: a Fabre may turn into a Pupa, a Pupa into a
    Creamy, the Peco Peco, Thief Bug and Ant eggs into a Peco Peco, a
    Thief Bug Larva and an Andre (one chance in a thousand each time it idles).
    mod0631's rows carry no monster to become and its skill table files
    the skill as an attack (nk 0); eAthena 817 fills both and builds it
    (`skill.c:2421`, `mob.c:1930`): the share of HP is kept, the new
    monster forgets whom it fought, and it respawns as the first kind.
    817 also sends 0x01b0, whose handler in this client is unread; we
    send only the vanish and appear that follow it. The rows say their
    source (`metamorphosis_817` in `tools/import/athena_import.py`).
  * **Hallucination** (Baphomet Jr., Horong, Myst Case, Stormy Knight,
    Carat, Gajomart, Perimeter, Karakasa): for
    30 s, plus 10 s a level, the hits the target takes are drawn as
    damage x 1-10 + 0-99; the HP taken is the true damage. Its status
    number is 34, which goes out as an icon: this client has its own
    picture for it, an eye in red (seen in the rig on 2026-11-16, with a
    hit of 464 drawn as 4645), so the status itself is older than the
    client. No screen effect showed in still shots. Neither 817 nor mod0631
    built it, and both give it range 0, so it could never reach anyone.
    Built as jAthena //0850 (2004-02-29, two months after this client),
    which also gives it range 1 (tier later).
  * **Keeping** (Cornutus, Stainer): below 30% HP the monster shuts
    itself in, DEF 100 (only VIT's part gets through), neither moving
    nor striking back, for 60 s plus 10 s a level. jAthena //0850, as
    above (tier later); 817 does not build it.
* **The Baphomet Card splashes** (2026-11-15): a weapon blow that lands
  also strikes every other enemy within a cell of the target, each with a
  weapon attack of its own that may miss, as this client's item text says
  ("Do splashed damages around 9 cells every time") and 817 and jAthena
  //0797 (2004-02-13) build it (`bonus bSplashRange,1`; 817 `battle.c:3396`,
  `skill.c:1563`). mod0631 gave the card only its HIT -10. The splash goes
  out as skill damage of skill 0 at level -1, as 817 sends it; at level -1
  this client shouts no skill name (`tools/import/athena_import.py`
  `EXTRA_EQUIP`, `scripts/rules/battle.lua` `rules.weapon_attack`).
* **Nine cards mod0631 left empty do what the client's text says**
  (2026-11-16; `EXTRA_EQUIP`, test `cards_mod0631_left_empty`). mod0631's
  item database gives these cards no script, so they did nothing. Each
  now follows its line in `idnum2itemdesctable.txt`, with the bonus
  names and the first jAthena commit that built it:
  * Muka (HP recovery +10%), Eggyra (SP +15%), Sidewinder (Double Attack
    level 1, 5%, with any weapon), Moonlight Flower (a quarter faster on
    foot, DEX -2): jAthena //0679 and //0695 (2004-01), 817 the same.
  * Zombie: HP recovery +15%, the client's number. jAthena and 817 have
    20, RODataZone's iRO card table (2004-02-24) 25: no two agree.
  * Hunter Fly (3%: 15% of a blow back as HP) and Dracula (3%: 5% as SP):
    jAthena //0842 and //0843 (2004-02-24/26), the same numbers as the
    client. jAthena also raised a gain of 0 to 1 whether the card fired
    or not, so every blow healed 1 HP and 1 SP. That slip is not kept.
  * Orc Lord: 25% of a close blow sent back, the client's number.
    jAthena //0843 has 30%. The client says "every single Physical
    Damage"; we keep jAthena's reading that blows from afar are not
    returned.
  * Maya: the client says "reflect Magical Damages back (Success Chance
    30%)" and RODataZone "30% chance to Reflect magic damage back on
    caster", but no period emulator builds it: jAthena has nothing before
    2005, and 817 and eAthena 0.5.2 cancel 30% of spells instead. How it
    works is **ours**: 30% of the spells aimed at the wearer (not a ground
    spell's cells) come back on the caster, worked out as if the caster had
    cast it on itself, drawn as coming from the wearer, and the kill counted
    as the wearer's.
  * The Archer Skeleton and Doppelganger Cards still do nothing. The
    client's text says "(Not implemented yet)" for both, and so does
    RODataZone; jAthena //0679 and 817 give the Doppelganger Card 30% ASPD.
* **A trap that runs out unsprung leaves its Trap** on its cell
  (2026-11-16), as 817 does (`skill.c:6143`: a player's traps, not Venom
  Dust) and an iRO beta 2 player describes ("any unused trap will be able
  to be picked up after it's lasting time has depleted", GameFAQs 20075,
  2003-02). mod0631 gives nothing back (`scripts/rules/skill_units.lua`
  `skill_unit_timer`, test `unused_trap_leaves_its_trap`).
* **Effects the client and RODataZone call not implemented are out**
  (2026-11-16; `research/nyi_items.py`, `EXTRA_EQUIP`). mod0631 (jRO)
  scripts some effects that this client's item text marks "Not
  Implemented yet" and RODataZone's iRO tables (2004-02) mark "NYI". With
  both witnesses against it (the rule of THREADS.md R6), the effect is
  taken out: the status chances of the Stun, Freeze, Flash, Curse, Poison,
  Sleep and Mute Arrows (the Freeze and Poison Arrows keep their element),
  the Sharp Arrow's critical, the Great Axe's stun and the Requiem Card's
  chaos. The Crescent Scythe's critical is +20, as both say, where mod0631
  has 30. About thirty other items are marked by the client alone
  (RODataZone lacks them, or shows the effect working); they keep
  mod0631's scripts.
* **Six items' stats follow the client and RODataZone** where both give
  the same against mod0631's script (2026-11-16, `EXTRA_EQUIP`): Muramasa
  critical +20% (mod0631 30), the Kobold Card critical +5% (4), the Poring
  Card LUK +1 and nothing else (LUK +2, Perfect Dodge +1), the Condor Card
  flee +5 (10), the Ninja Suit MDEF +3 beside its AGI +1 (none; 817 has
  it), the Nurse Cap INT +1 (LUK +1).
* **EXP is shared by the monster's max HP, and the MVP EXP comes with
  the reward item** (2026-11-15): the share as jAthena //0764 of
  2004-02-01 has it ("monster EXP handled like the real server", the same
  commit as the looter's ten items), the MVP EXP as jAthena //0814 of
  2004-02-17 ("on the real server the MVP EXP is now always given"; a
  version between had gone back), and 817 both (`map/mob.c:1730`, `:1852`). Each
  player gets the part of the max HP he took off, so a monster that
  something else (a guardian, a trap's owner gone) brought low gives the
  one who finishes it little, never less than 1 EXP; one that healed
  itself while fighting can give up to twice its EXP. mod0631 shared the
  players' own damage, so a kill alone always paid in full, and gave an
  MVP's EXP only when no reward item came. The MVP is the biggest damage
  dealer still alive.
* **Ground skills may not be stacked** (2026-11-15): a Safety Wall or a
  Pneuma fails on or against another Safety Wall or Pneuma, a trap, a
  Talkie Box or a Warp Portal fails within a cell of a trap (Venom Dust
  counts), a Fire Pillar within a cell of one not yet set off; the SP
  and the gemstone are kept. jAthena //0751 (2004-01-28) forbade it
  ("some ground skills may no longer be stacked") and //0764 (2004-02-01)
  made it the setting `player_skill_reiteration: no` ("with yes, Safety Wall, Pneuma and
  traps can be laid on top of each other"), and 817 has it
  (`skill.c:862`); mod0631 checks nothing. 817 also refuses them on
  anyone's feet, so a Safety Wall under oneself fails there; jAthena
  //0825 (2004-02-20) made that a separate `nofootset`, off, and so is
  ours (`skill_reiteration_blocked` in `scripts/rules/skill_units.lua`).
* **Lengths of 2003** (2026-11-14). Where Ragu Memo's ailment page (jRO
  players' measurements, captures of 2003-08-02 and 2003-12-12; research
  round 13, THREADS.md #21) gives a rule, it replaces 817's per-skill
  lengths: a stun lasts 3 s less (3 + VIT + LUK/3)% whatever the skill
  (817: Sonic Blow 6 to 15 s), Blind 30 s - BaseLv/10 - INT/15 (817: 10 to
  19 s), stone 17 s - MDEF/20 (817: Stone Curse's 10 to 19 s less MDEF/5),
  sleep about 30 s (817: Sandman 12 to 20 s) less INT + LUK/3 as in 817, and
  silence is shortened by VIT alone. Freezing (the same rule in both) and
  Curse (unknown to the page) keep 817's, but for Frost Diver: 3 s a level
  less MDEF (817: 6 to 15 s), the iRO guide's Mage page of 2004-04-06
  ("3sec" to "30sec"), mod0631's own length and an iRO player's "around
  20-30 seconds" at level 10 (research round 27; THREADS.md #21).
* **Poison takes HP** down to a quarter: 3% of max HP every 3 seconds
  (Ragu Memo; 817 bit every second, 3 + 1.5% for a player, 3 + 0.5% for a
  monster); in mod0631 it only shows. **Poison takes a quarter off DEF,
  freezing and stone half of DEF and add a quarter to MDEF** (817 and Ragu
  Memo agree; not in mod0631); **a Curse slows the walk** (817).
* **Blessing lifts Curse and Stone from players** too (mod0631: monsters only).
* **Storm Gust freezes on its third hit** (817) instead of 35%+3%/level a hit,
  **and passes over the frozen** (817 `skill.c:715`; jAthena //0738 of
  2004-01-24 calls both "the real server's way"); mod0631 hits them.
* **Stone** takes five seconds to set, then lasts its time and costs 1% HP
  every fifth second (817); in mod0631 it is instant and never wears off.
* **Stun, poison, blind and silence are not renewed** while they last, and
  **the undead do not freeze** (817).
* **Brandish Spear, Auto Counter, Venom Splasher, Signum Crucis, Slow
  Poison, Find Stone, Ruwach's damage, element-resisting cards against
  magic**: not in mod0631 at all.
* **Auto Counter** (2026-11-15) as 817 with its default
  `player_auto_counter_type: 1` and jAthena //0751 (2004-01-28, "matched to
  the real server"): a plain blow from the front is turned aside and
  answered ignoring DEF, with HIT +20 and twice the critical rate; skills
  are not countered (//0814, 2004-02-17: "on the real server skills seem
  not to be countered"). While the stance lasts the Knight cannot walk,
  attack or use another skill (817 `pc.c:3215`, `clif.c:5019`,
  `skill.c:3851`; monsters `mob.c:220`, `344`, `2304`). Until today ours
  was a sure critical that left the Knight free. (817's config comment
  calls type 1 "100% critical"; its code and jAthena's manual say the
  opposite.)

**Ours, where neither emulator has anything** (tier guess unless noted):

| Skill | What we do | From |
|---|---|---|
| Flasher | Blinds, at Sandman's chance | The client's skill text: "Blinds an Enemy". Chance ours |
| Sandman | Also rolls sleep on the enemies in the cells around the one who stepped on it | The iRO guide's Hunter page (2004-03-27): "an enemy and near enemies". Both emulators: that one alone. The 3x3 is ours |
| Shockwave Trap | Takes 20/35/50/65/80% of a player's SP | The client's skill text. Monsters have no SP here |
| Enchant Poison | Each hit may poison (3% to 7%) | mod0631 stores that chance on the status and never rolls it |
| Brandish Spear | The two cells either side of the knight and five in front, then rows of five, five and three, 1 to 4 areas by level; the target is pushed two cells | The drawing and the "2 cells" on the iRO guide's Knight page (2004-04-13). 817 has its own helper for the rows and pushes one cell; the per-row damage is 817's |
| Detect | Reveals the hidden, and shows hidden traps again (0x01ac), within 3 cells of the spot; the spot may be 3, 5, 7 or 9 cells away by level | The client's skill text; the reach is the iRO guide's "Detecting Range" (2004-03-27), read as how far the falcon flies; traps from iRO's Comodo patch notes ("Traps can also be seen using a detecting skill", THREADS.md #28) |
| Spring Trap | Reaches 4 to 8 cells by level | The iRO guide's Hunter page |
| Sightrasher | Under Sight only, which it ends: fire on the eight lines around the wizard, 1 to 5 cells by level, 120% to 300% MATK, pushed 5 cells back | The iRO guide's Wizard page (2004-03-27); damage 817 `battle.c:3178`; Sight and the push jAthena (2004-05). The lines and the reach by level are ours (THREADS.md #22) |
| Remove Trap | Not a trap that has gone off | The iRO guide's Hunter page |
| Venom Splasher | Only on a poisoned enemy under a third of its HP. Marks it; when the mark runs out (5.5 s to 10 s by level) the target takes the blow, the enemies on the 3x3 around it a quarter, and it may poison them. A hit before then undoes the mark | The client's skill text ("HP at least by 1/3") and the iRO guide's Assassin page (2004-03-27, "less than 1/3"); the mark, its time and the poison jAthena //1046 (2004-09-12, tier later); the quarter and the hit that undoes it anjingbalap's iRO FAQ v2.2 (2004-06-02). euRO's May-2004 text and jAthena have 2/3, a later change. The damage is 817's (THREADS.md #22) |
| Poison React | Lasts 30 s at every level | The iRO guide's Assassin page. 817's `cast_db.txt` says 30 minutes |
| Change Cart | Cart look by base level (41, 66, 81, 91) | The client's skill text for the five carts; the levels are later emulators' (tier later) |
| Provoke | Refused on the undead | The client's skill text: "Not appliable for the Undead" |
| Sonic Blow | Needs a Katar | The client's skill text: "using Katar" |
| Ice Wall | A cell someone stands on stays free. Area skills wear the wall down (Magnum Break, Arrow Shower, Napalm Beat, Fire Ball, Thunderstorm, Heaven's Drive, Sightrasher, Blitz Beat's splash, Venom Splasher's burst); it cannot be clicked | Nobody gets walled into a cell. The rest is eAthena 817 (`skill.c:847`, `3718`, `battle.c:3464`): durability 200 + 200 a level (the iRO guide's 400 to 2200; 817 gives level 1 500), no DEF, Water 1. This client turns a click on the wall into a walk (tried 2026-11-15) |
| Vending | "There is nobody to sell to." | Single player |

**Engine:**

* **Monsters on maps nobody is on** stand still. Athena's "lazy" AI let them
  wander and warp now and then.
* **Parties.** Party-wide skills (Angelus, Magnificat, Gloria, Adrenaline
  Rush...) reach the caster only: there is no party.
* **Poison React, Remove Trap, Spring Trap** are drafts from the client's
  skill text alone (tier `guess`; no emulator of the period has them).
  Poison React answers one weapon blow of the Poison element that lands
  (not a miss) with (100 + 30 x level)% and ends; Remove Trap takes up any
  Hunter trap and gives one Trap back; Spring Trap needs the falcon and
  destroys the trap. All three tried in the real client (Remove Trap and
  Spring Trap 2026-11-13, aimed at a trap from the skill window; Poison
  React 2026-11-15). The answer is of the Poison element, as every period
  skill_db has it (mod0631, 817, 0.5.2: row 139), so it misses a Poison
  monster, and a monster's plain blow is neutral: only Poison skills set
  it off.
* **The Crusader's shield skills** (behind `jobs2_2`; the first of the 2-2
  jobs' skills to be drafted, because this client can show them:
  THREADS.md #25). mod0631 has Auto Guard and Defender behind `#if 0` and
  calls Shield Charge and Shield Boomerang "no damage". From eAthena 817,
  whose numbers the iRO guide of April 2004 repeats: Guard blocks 5% to
  30% of weapon hits and holds you 0.3 s; Defender takes 20% to 80% off
  ranged weapon damage and slows attack and walk; both are switched off
  again for nothing and end when the shield comes off; Shield Charge and
  Shield Boomerang hit as Neutral whatever the weapon, the Boomerang from
  3 to 11 cells with a tenth of the shield's weight and its refine bonus
  added. Ours:
  * Shield Reflect is in neither emulator (817 plays its effect and
    nothing else). It follows the guide: 13% to 40% of the damage back for
    300 s. That only weapon hits from close by come back is our reading;
  * Guard, Defender, Shield Reflect, Shield Charge and Shield Boomerang
    refuse to start without a shield (the guide's "Requires a shield";
    817 only ends the first two when the shield comes off), and Shield
    Reflect ends with the shield too;
  * 817 also stuns for 3 s on a Shield Charge by a second roll outside its
    resistance rule (skill.c:1233); only the roll that the target's VIT
    resists is kept.
* **Providence, the Sage's endows and Dispell, the Rogue's Strips, the
  Alchemist's Chemical Protections** (behind `jobs2_2`; drafts, test
  `endows_strips_protections_dispell`). This client has no name or
  description for any 2-2 skill: they are read against the two emulators,
  the iRO guide of spring 2004 and euRO's own skill text of May 2004 (the
  MICO07 client, `research/notes/may2004-client.md`). Nor
  can it offer them: they are missing from its skill window, and only
  `@useskill` reaches them (THREADS.md #25).
  * Providence is mod0631's (5% a level against Holy and against demons,
    180 s); its 3 s of cast are 817's and the guide's.
  * The four endows last 20 minutes (817, the guide), not mod0631's 30 to
    90 s. Neither emulator lets them fail. They follow the guide: 80% to
    100% by level, and "weapon is destroyed on failure": the weapon in the
    target's hand is lost. Ours: no endow on bare hands.
  * The Strips take at (5 + 2 x level)% for 60 s (817, the guide's "Basic
    Chance"). 817's status does nothing to a monster; here Strip Shield
    takes 15% off its DEF, Strip Armor 40% off its VIT and Strip Helm 40%
    off its INT, as euRO's own skill text of May 2004 says (jAthena of
    2004-05-07 agrees on the first two and has 10% for INT). Strip Weapon
    takes a tenth off its attack, the iRO guide's number (euRO's text
    gives none). The guide's "DEX difference ... affects the success
    chance" has no number and is left out. A stripped player may put the
    piece straight back on (817 does not stop it either; the guide's
    "cannot re-equip" is not built: nothing in this world strips a player).
  * Chemical Protection stops the matching Strip for a minute a level and
    takes a Coating Wax (the guide; 817 starts the status for no time and
    never asks for it).
  * Dispell takes a Yellow and a Red Gemstone (817, the guide; mod0631 the
    yellow one) and works at 60% to 100% (the guide; neither emulator
    rolls). What it ends is 817's list: the ailments, Increase and Decrease
    AGI, Suffragium, Magnificat, Gloria, Apple of Idun, the endows and the
    Chemical Protections. The guide says "all magical effects".
* **The Sage's Magic Rod, Spell Breaker, Volcano, Deluge, Violent Gale
  and Land Protector** (behind `jobs2_2`; drafts, test
  `sage_rod_breaker_fields`). mod0631 never built them; 817 has Spell
  Breaker and lays the fields with no effect beyond ending each other. They
  follow jAthena (Magic Rod //0868 of 2004-03-06, the rest as of //0969),
  whose numbers the iRO guide's Sage page of 2004-03-27 gives too:
  * Magic Rod for 0.4 s to 1.2 s: a spell cast at the Sage does nothing and
    gives him 20% to 100% of its SP; turned on a Spell Breaker, its caster
    loses a fifth of his SP. Spell Breaker (0.7 s, 817) breaks off a spell
    being cast (jAthena's list of spells; a Phen Card does not help),
    takes its SP from a player who cast it and gives the Sage 0% to 100% of
    it. Aimed at an enemy (817, jAthena); mod0631's row says ground.
  * Volcano, Deluge and Violent Gale: 5x5 to 9x9 for 180 s, 5 s of cast and
    a Yellow Gemstone; a Fire, Water or Wind blow on whoever stands in it
    does 10% to 20% more, and a player of that armour element gains 10 to 50
    ATK, 5% to 15% max HP, 3 to 15 FLEE. jAthena reads a plain blow's
    element off the target (the attacker's here), adds Violent Gale's
    number to Volcano's ATK and makes the FLEE a percentage: slips, read as
    the guide's table. From the guide and euRO's text of May 2004, in no
    emulator: Fire Wall lasts half as long again in a Violent Gale, Ice
    Wall cannot be laid in a field, Deluge lets Water Ball and Aqua
    Benedicta be cast on dry land.
  * Land Protector: 7x7 to 11x11, a Yellow and a Blue Gemstone (the guide,
    euRO's text, jAthena; mod0631 the yellow one), 2 to 5 min (the guide's
    table; the emulators' cast_db 120 s to 240 s or 180 s). As jAthena: it
    wipes the ground skills on its cells, a one-cell skill is not laid on
    it (its catalyst is spent all the same), and nothing on the ground
    touches someone standing on it.
  * A Sage has one field at a time (jAthena); a field ends the other
    fields' status on whoever enters it (817).
  * Cast Cancel (test `sage_cast_cancel`) as jAthena //0969: asked for
    during a cast only, it stops the spell, and the Sage may act at once.
    It costs its 2 SP and 90%, 70%, 50%, 30% or 10% by level of the
    cancelled spell's SP (the guide's table; euRO's May text: "Consumption
    SP of casted magic will be reduced by Skill-level"). jAthena charges
    that share of Cast Cancel's own SP, having overwritten the spell it
    saved (`skill_use_id`): a slip, not taken.
* **Grand Cross** (behind `jobs2_2`; draft, test `grand_cross`). mod0631
  calls it "no damage" and Neutral. eAthena 817's (jAthena //0969 the
  same): 3 s of cast nothing breaks, a cross of 29 cells around the caster,
  each blow the magic and the weapon damage added and then 140% to 500%,
  Holy, the undead and demons sometimes blinded. With them: a fifth of the
  HP the caster has (euRO's skill text of May 2004, jAthena's
  skill_require_db), any weapon but a bow, no blinding a boss (euRO's
  text). Ours, from the iRO guide's Crusader page (2004-04-11): the cross
  lasts 0.9 s and strikes three times, one hit each (817 keeps it 1 s and
  mod0631's row counts 3 a hit), and strikes the caster too, at half
  ("the damage amount the caster takes is half the amount the enemy
  takes"; neither emulator has it). The guide's lower DEF while casting
  ("decreases by 2/3") is read as *to* 2/3, with 817's `cast_def_rate` 33
  and the Comodo preview's "reduced to 2/3" (see "A third off the caster's
  DEF" above).
* **The Alchemist's Acid Terror, Bomb (Demonstration) and Aid Potion
  (Potion Pitcher)** (behind `jobs2_2`; drafts, test
  `alchemist_acid_bomb_aid`). mod0631 never built them; 817's Acid Terror
  deals no damage and its Aid Potion looks for item 0 above level 1. They
  follow jAthena //0969 (2004-05-13, five months after our date), whose
  numbers the iRO guide's Alchemist page of 2004-03-27 repeats: Acid
  Terror a 140% to 300% weapon blow for an Acid Bottle, 1 s of cast; Bomb
  a 3x3 that strikes 120% to 200% each second for 40 s to 60 s, for a
  Bottle Grenade, as Fire (jAthena's skill_db and euRO's text of May 2004;
  mod0631's row says Neutral); Aid Potion throws a Red, Orange, Yellow,
  White or Blue Potion by level, worth 110% to 150%. Ours:
  * the armour (Acid Terror, 3% to 13% a throw) and weapon (Bomb, 1% to
    5% a burn) damage the guide gives falls on monsters: ArislanX's iRO
    Alchemist guide (2004-08-20, later) has a monster's broken armour
    cost it a quarter of its DEF and a broken weapon a quarter of its
    attack. Ours: it lasts as long as the monster lives (no source gives
    a time). jAthena breaks only players' gear, and nothing breaks a
    player's here: no monster casts either skill (test
    `alchemist_breaks_monster_gear`);
  * Acid Terror is Neutral whatever the weapon (ArislanX: "is neutral,
    so not subject to elemental weaknesses"; jAthena //1072, 2004-12-06);
    817's and mod0631's rows give it the weapon's element. jAthena's
    //1072 also has it ignore DEF and cards, which no player source of
    2004 says: not built;
  * the Blue Potion thrown by Aid Potion takes the guide's 10% a level
    (jAthena's 1% looks a slip);
  * Bomb's square is centred on the cell aimed at (jAthena's sits a cell
    up and to the left of it).
* **The Alchemist's Bio Cannibalize and Sphere Mine** (behind `jobs2_2`;
  drafts, test `alchemist_summons`; seen in the client on 2026-11-16).
  jAthena //0969's numbers: a Flora of 2210 + 200 x level HP for 30 s +
  10 s a level, for a Plant Bottle; a Marine Sphere of 1000 + 200 x level
  HP for 30 s, for a Mine Bottle; both go out of sight when their time is
  up. What they do is ours, on the iRO guide's Alchemist page (2004-03-27)
  and euRO's text of May 2004 ("Summon Flora on selected ground for attack
  your enemy"; the sphere uses "Suicidal Attack if it's attacked"): in
  jAthena a summon is no monster's enemy, so a Flora attacks nobody but
  another player and a sphere's blast can only hurt players. Here:
  * a summon is on its Alchemist's side (a third side beside a siege's
    two): a Flora strikes the monsters about it, they strike back, and
    what a summon takes off a monster counts as its Alchemist's damage for
    the EXP (jAthena's `mob_damage` does that part). Summons use no
    monster skills (jAthena) and leave no drops or EXP when they die;
  * the Alchemist cannot hit his own Flora; he hits his Marine Sphere to
    set it off (jAthena: every summon is a player's enemy), and it blows 5 s
    later (the guide; jAthena casts the monster's own 3 s row) on the
    monsters within 5 cells, sparing players (jAthena's blast strikes
    everything, its Alchemist too). The guide's sphere moves about and
    hurries as the fuse burns; this one stands, as the monster's row in
    every period database (mode 0);
  * on the chosen cell (the guide, euRO's text; jAthena scatters it up to 5
    cells off), and no more than three Floras at once (euRO's text; neither
    the guide nor jAthena sets a limit). Bio Cannibalize costs 15 SP (the
    guide, mod0631; jAthena 20).
* **The Bard's and Dancer's songs** (behind `jobs2_2`; drafts, test
  `bard_dancer_songs`; seen in the client on 2026-11-16). mod0631 has the
  status numbers by level and nothing that plays a song; eAthena 817 lays
  a fixed 9x9 square. They follow jAthena of 2004-05-27 to 06-11 (//0993
  to //1020, rebuilt from its "differences from the real server" thread),
  with the iRO guide's Bard and Dancer pages (2004-03-27):
  * a song is a 9x9 square around its player, the middle cell reaching it
    all, that walks with them (a throw leaves it behind, //1015; a Fly Wing
    takes it along, another map ends it); its time is the guide's and
    jAthena's (30 s to 180 s); one SP goes every 3 to 10 s by song (both);
  * while playing: a walk four times slower, no plain blow, no sitting, no
    SP recovery (HP comes back), and no skill but Adaptation, Musical
    Strike and Slinging Arrow (jAthena //0998-//1001); Adaptation only after
    the first 5 s (//0995); a weapon that is no instrument or whip, no
    SP, a blow of more than a quarter of the max HP, or being stunned,
    frozen, asleep or turned to stone ends it (jAthena, both rules there
    by //0693 of 2004-01-09; the blow also in two player guides of late
    2004, round 29). Encore plays the last song again for half its SP, plus
    its own 1 (the guide, euRO's May text; mod0631's row: 0 SP);
  * what a song gives: the guide's level column (mod0631's level numbers
    are the same) with jAthena //0820's Lessons and stat parts, which the
    guide names without numbers (`song_effect` in status_change.lua).
    Assassin Cross of Sunset is jAthena's whole formula, the guide's table
    at Lessons 10; it does nothing with Two-Hand Quicken, Adrenaline Rush,
    Spear Quicken or a bow (jAthena; euRO's May text). Fortune's Kiss drops
    jAthena's extra 10%. Poem of Bragi's delay at level 10 is the guide's
    50% (jAthena 30%). Don't Forget Me ends Increase AGI and the quickening
    skills and slows monsters' walk and blows too. Service For You costs the
    guide's 40 to 85 SP (mod0631 copied Assassin Cross's 38 to 65);
  * Dissonance strikes at once and every 3 s, 40 to 80 (the guide) plus 3
    a level of Music Lessons (jAthena, whose level part is 20 a level);
    Ugly Dance takes 10 to 50 SP a time (the guide; jAthena gives it no
    effect), from players only, monsters having none. Apple of Idun heals
    35 to 80 HP every 6 s (the guide; jAthena's healing code never runs).
    Frost Joke and Scream strike everyone hostile on the screen 3 s later
    (jAthena): 20% to 40% frozen (the guide and jAthena agree), 30% to 50%
    stunned (jAthena; the guide has no number). Musical Strike and Slinging
    Arrow take the worn arrow's element and ATK and spend no arrow, as the
    bow skills here;
  * this client draws ten of the solo songs' cells; Assassin Cross of
    Sunset's and Ugly Dance's ask for `ring_red.tga`, which its GRF lacks,
    and were sent undrawn as Volcano's until setup made the file
    (2026-11-17, "A red ring" above): all three are drawn now. Not built: the guide's
    rule that two songs overlapping make a level-1 Dissonance or Ugly Dance
    (a lone player plays one song);
  * **the ensembles** (2026-11-16, test `ensembles`), played **alone**: in
    the period game a Bard and a Dancer of one party stand side by side,
    each knowing the skill, and play it at the mean of their levels
    (jAthena //1020 skill.c:5111). Here one Bard or Dancer plays it at
    their own level, as jAthena //1003's `player_skill_partner_check: no`
    does. The rest is the solo songs' (9x9 square, 60 s, one SP every 3 to
    5 s, either an instrument or a whip). What each does: War Drum ATK +50
    to +150 and DEF +4 to +12, Ring of Nibelungen ATK +150 to +350 with a
    level-4 weapon (the guide and jAthena agree; its SP 40 to 60 is the
    guide's, mod0631 38 to 50), Siegfried Fire and Water 40% to 80% (the
    guide; jAthena 20% to 80%), Mr. Kim A Rich Man EXP +36% to +80% (the
    guide's 136% to 180%; jAthena +50% to +150%), Into the Abyss no
    gemstones or Traps, these five for the party (the player); Eternal
    Chaos DEF 0 and Loki's Veil no skills (not on bosses, the guide) for
    everyone there but the one playing; Lullaby 15% against INT (jAthena;
    the guide: "affected by INT of the caster", no number) to sleep each
    enemy there, at once and every 6 s. This client draws Mr. Kim's, War
    Drum's, Loki's Veil's and Into the Abyss's cells; Lullaby's,
    Eternal Chaos's, Nibelungen's and Siegfried's want files its GRF
    lacks and are sent undrawn (`UNDRAWN`). Played in the client on
    2026-11-16 (`@useskill`): War Drum's notes and Loki's Veil's field
    walking with the Bard, the monsters in the veil and not the Bard.
* **The Monk's skills** (behind `jobs2_2`; drafts, test `monk_skills`;
  `scripts/rules/monk.lua`). mod0631 has the damage of Occult Impaction,
  Chain Combo and Combo Finish, Steel Body, Fury, Iron Fists, Flee and
  Spiritual Cadence, and nothing that makes or spends a spirit sphere.
  The rest follows jAthena //1020 (2004-06-08; its Monk dates from //0507
  to //0544 of 2003-10/11, before our client), checked against the iRO
  guide's Monk page (2004-03-27), which agrees on almost every number:
  * spirit spheres: one a Summon Spirit Sphere, 10 minutes each, as many as
    the skill's level; 3 ATK each on every blow (jAthena). Occult
    Impaction, Blade Stop, Combo Finish and Snap cost one, Steel Body and
    Fury five, Guillotine Fist five (four straight after Combo Finish or
    in Blade Stop), Throw Spirit Sphere one a level or what is left. Not
    kept over a logout (jAthena);
  * combos: Triple Attack opens a window of 1 s less 4 ms a point of AGI
    and 2 of DEX (300 ms more if the next blow is known) in which Chain
    Combo may follow, and that one a window for Combo Finish (0.7 s), and
    that one for Guillotine Fist, which then needs no cast and reaches the
    target Combo Finish threw back. The Monk stands and does not swing
    in a window (jAthena);
  * Combo Finish throws the target back 5 cells (the guide and 817;
    mod0631 3, jAthena none). Throw Spirit Sphere does 150% to 250% a
    sphere (the guide; mod0631 has none, 817 and jAthena 150% to 350%);
    its cast is a second more for each sphere (both). Fury adds 10% to 20%
    critical rate (the guide, 817, jAthena; mod0631 raised the rate by that
    share of itself). Steel Body has the guide's 5 s cast (every emulator:
    none);
  * Guillotine Fist: ATK x (8 + SP / 10) + 250 + 150 a level, past FLEE,
    size and DEF, Neutral; the Monk dashes to the cell beyond the target,
    is left with no SP, Fury ends and no SP comes back for 5 minutes
    (jAthena //0979 and //0998; the guide);
  * Absorb Spirit Sphere gives 10 SP a sphere (the guide; jAthena 7), from
    the Monk's own spheres only (jAthena //0980 later added a chance of SP
    from a monster, "a little ahead of time": not built);
  * Blade Stop waits 0.5 s to 1.3 s; a plain blow then holds the striker
    and the Monk 20 s to 60 s, neither walking nor striking, the Monk
    allowed Throw Spirit Sphere from level 2, Occult Impaction from 3,
    Chain Combo from 4 and Guillotine Fist at 5, which let go. Not on a
    boss (the guide; jAthena catches anything). A monster held uses no
    skill either (ours: jAthena's check is for players);
  * Snap moves the Monk at once to a cell in a free straight line (the
    guide: "if there is no obstacle"); jAthena moves it anywhere.
  This client has no packet for spheres, the combo window or Blade Stop,
  and no skill window entry for any of it (THREADS.md #25): the combos
  can be reached only by `@useskill` inside a window shorter than a
  second. euRO's May-2004 client draws them all.
* **The Sage's Auto Spell and Abracadabra, the Crusader's Devotion**
  (behind `jobs2_2`; drafts, test `sage_autospell_abracadabra_devotion`;
  `scripts/rules/sage.lua`). Neither period emulator has them working
  (817 sends Auto Spell's list and casts nothing); they follow jAthena of
  2004-06-11 (Auto Spell since //0821 of 2004-02-20, Abracadabra since
  //0749 of 2004-01-27, its table since //0900 of 2004-03-24), with the iRO
  guide's Sage page (2004-03-27) and euRO's skill text of May 2004:
  * Auto Spell: 3 s cast (jAthena), 120 s and 30 s more a level (the
    guide; jAthena's cast_db has 360 s at level 8). Its levels offer
    Napalm Beat (3), the three bolts (1 to 3), Soul Strike (1 to 3), Fire
    Ball (1, 2), Frost Diver (1), as euRO's text lists them, never above the
    level learned. Each blow, hit or miss, has a 7% to 16% chance (the
    guide; jAthena 7% to 25%) to cast the spell at once, at its top level
    15% of the time, one below 35%, two below 50% (both), for two thirds
    of its SP and only if the Sage has them. **Ours:** this client has no
    window for the choice (0x01cd, THREADS.md #25), so the server picks
    the strongest spell offered and says so in the chat; `@autospell
    <spell>` picks another, for the Auto Spell on and the next ones;
  * Abracadabra: 50 SP and two Yellow Gemstones (the guide, euRO's text,
    jAthena). It draws a skill by jAthena's own table (its authors', no
    period source has one) and lends it for one use at the Abracadabra
    level learned or the skill's top level, aimed as with a scroll
    (0x0147). **Ours:** only the skills this client can name are drawn
    (jAthena's table also has the 2-2 skills), and Abracadabra's own
    thirteen skills, which this client knows nothing of, go off at once,
    on the Sage or the nearest enemy monster within 8 cells (none: nothing
    happens), with a chat line naming them. What they do is jAthena's
    except where euRO's text says otherwise: Level Up gives a whole base
    level (the text; jAthena a tenth of one), Full Recovery cures poison
    too (the text), Reverse Orcish's orc face lasts 20 minutes (rAthena's
    skill_db, tier later; jAthena sets it for good; this client draws no
    orc face for it), "?" makes the monster
    show "?" (the text). Mono Cell (a Poring) and Class Change (one of
    jAthena's fifteen bosses) are for good, as in jAthena (euRO's text: "for
    a while");
  * Devotion always fails: it wants another player of the Crusader's party
    or guild (jAthena, the guide, euRO's text). Nobody else is here.
* **Talkie Box** speaks: mod0631 lets it lie silent and 817's attempt is
  marked "not working", so it follows jAthena //0988 (2004-05-25, five
  months after our date): the first one to step on it makes it say the
  text typed when it was set, and it goes 5 s later. Its own Hunter does
  not set it off (the iRO guide's Hunter page of 2004-03-27, and jAthena
  //0991 of the next day).
  jAthena's change to a used trap is not sent: this client applies 0x00c3
  to characters only (docs/PROTOCOL.md).
* **Not built yet:** Repair Weapon
  (nothing breaks in this era's data), monsters'
  "friend" skill targets (mod0631 has none either).
* **One cast at a time for monsters.** In mod0631 a counter skill could
  start over a cast in progress.
* **Unidentified equipment cannot be worn**, and **nobody walks while
  playing dead**: mod0631 checks neither.
* **Cart Revolution needs a cart** (the client's skill text); mod0631 does
  not check.
* **"Unable to Teleport in this area"** is shown when Teleport or Warp
  Portal is refused by a map; mod0631 says nothing.
* **A monster that senses casting** reacts unless it has a target (mod0631:
  unless it is attacking), and the "casttargeted" skill condition, which
  mod0631 defines and never raises, is raised.

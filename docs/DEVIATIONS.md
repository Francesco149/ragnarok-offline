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
| Guilds, War of Emperium | Guild castles, flags, treasure, guild dungeons (WoE from open beta, Sundays) | Castle NPCs not imported. Nobody owns a castle, so each holds the monsters 817 called into ownerless castles at WoE start (`scripts/npc/castles.lua`); every server start counts as a WoE start. No Emperium, treasure or guild-dungeon access | Needs guilds and opponents. The monsters are period behaviour (kRO until 2004-02-10, 817 `mobs/gvg.txt`); the timing is ours. `castle_monsters = false` empties the castles. The rest is open (THREADS.md #17) |
| PvP | PvP rooms (Prontera/Izlude) | The imported PvP NPCs are off (`[content] pvp = false`) | Empty rooms. Turn it on to visit them |
| Trading, vending to players | Trade window, merchant shops | No other players. The client's buttons do nothing | — |
| Chat rooms | Player chat rooms; NPC "waiting rooms" (arena, PvP queues) | Not drawn; NPCs that open one log a notice | Player rooms are pointless. NPC waiting rooms are a TODO (ROADMAP) |
| Whisper, friends | — | Whispers get "not online" | — |
| Announcements | Server-wide broadcasts | Shown to you (`announce` and friends) | — |
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
* **Monster Tamers** (`scripts/npc/monster_tamers.lua`): three NPCs from
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
    forget you (mod0631 sets the quest back to its first step).
* **The Priest test on one map** (`scripts/npc/priest_test.lua`). 817 runs
  its three rounds on `priest_1-1`, `priest_2-1` and `priest_3-1`. This
  client has that map once (`job_priest`), so the first Nun's warp failed
  and nobody could become a Priest through the test. The three rounds are
  played on `job_priest` in turn; the map's flags (no teleport, no memo, no
  death penalty, logging out puts you in Prontera) are 817's.
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
  (2004-05-02). The armour is neither tested nor taken: he says to come
  wearing one, as mod0631's knight does. Not confirmed for the
  international servers (THREADS.md #7).
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
  * his list shows the swords you have heard of.
* **Alberta's doll child** (`scripts/npc/doll_child.lua`). 817's "Kid"
  wishes for a monkey doll and that is all. mod0631's takes any of nine
  dolls and gives a sweet back (a Yoyo Doll brings an Yggdrasil Seed); we
  take that, on 817's cell, with 817's first two pages and our
  translation of the rest.
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
  (`skillquests/arrowrepel.txt:115`) is dropped the same way.
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
* **"morroc"** in the Juno Kafras is a typo for `morocc`.
* **Shop names** come from 817 by cell (mod0631's are Japanese). "Shop" is
  used where 817 has none within 3 cells, or only a Japanese one.
* **Menus whose target label is missing** end the dialog. 817 jumped to the
  start of the script, which restarted the conversation on "Cancel".
* **`end;` with a message showing** gets a close button, so the window
  doesn't hang.

## Engine behaviour relative to Athena 817

* **Cell types.** Only types 1 and 5 block, as in Athena (mod0631
  `map.c:1052`, `path.c:153`). One map of this client depends on it: the
  whole floor of `c_tower1` has the type `0x444c74` (some builder's stray
  bytes). Until 2026-11-11 the server took unknown types for walls, and
  Clock Tower 1F could not be entered.

* **A Fly Wing (or Dead Branch) refused by a map flag is not used up.** In
  Athena it was spent before its script ran. The client still shows its
  "Unable to Teleport in this area" message.
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

**Slips of mod0631 that are not carried over** (the Lua marks each
"mod0631 bug"):

* **Magnum Break** did 65% damage at every level and distance: C operator
  precedence in `5*skill_lv +(wflag)?65:115`. Here: 115% + 5% a level on the
  target, 65% + 5% a level around it, as the expression was meant.
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
* **Attack speed.** Not mod0631's slip but ours, fixed now: the server
  attacked twice as fast as the status window's ASPD says. The delay between
  two attacks is twice the "aspd" value (`pc.c:2458`).

**Taken from eAthena 817 because mod0631 leaves it unfinished:**

* **Ailments** (stone, frozen, stun, sleep, poison, curse, silence, blind):
  the chance falls with the target's MDEF, VIT, INT or LUK, the length with
  the same stats, monsters keep half the chance at least, and the lengths
  come from 817's `cast_db.txt` (`data/db/skills.toml` `time`, `time2`).
  mod0631 has fixed chances, placeholder lengths ("some length or other" in
  its comments), and no Curse at all (the status is refused).
* **Poison takes HP** every second down to a quarter (817); in mod0631 it
  only shows.
* **Blessing lifts Curse and Stone from players** too (mod0631: monsters only).
* **Storm Gust freezes on its third hit** (817) instead of 35%+3%/level a hit.
* **Stone** takes five seconds to set, then lasts its time and costs 1% HP
  every fifth second (817); in mod0631 it is instant and never wears off.
* **Stun, poison, blind and silence are not renewed** while they last, and
  **the undead do not freeze** (817).
* **Brandish Spear, Auto Counter, Venom Splasher, Signum Crucis, Slow
  Poison, Find Stone, Ruwach's damage, element-resisting cards against
  magic**: not in mod0631 at all.

**Ours, where neither emulator has anything** (tier guess unless noted):

| Skill | What we do | From |
|---|---|---|
| Flasher | Blinds, at Sandman's chance | The client's skill text: "Blinds an Enemy". Chance ours |
| Shockwave Trap | Takes 20/35/50/65/80% of a player's SP | The client's skill text. Monsters have no SP here |
| Enchant Poison | Each hit may poison (3% to 7%) | mod0631 stores that chance on the status and never rolls it |
| Brandish Spear | The two cells either side of the knight and five in front, then rows of five, five and three, 1 to 4 areas by level; the target is pushed two cells | The drawing and the "2 cells" on the iRO guide's Knight page (2004-04-13). 817 has its own helper for the rows and pushes one cell; the per-row damage is 817's |
| Detect | Reveals the hidden within 3 cells of the spot; the spot may be 3, 5, 7 or 9 cells away by level | The client's skill text; the reach is the iRO guide's "Detecting Range" (2004-03-27), read as how far the falcon flies |
| Spring Trap | Reaches 4 to 8 cells by level | The iRO guide's Hunter page |
| Remove Trap | Not a trap that has gone off | The iRO guide's Hunter page |
| Poison React | Lasts 30 s at every level | The iRO guide's Assassin page. 817's `cast_db.txt` says 30 minutes |
| Change Cart | Cart look by base level (41, 66, 81, 91) | The client's skill text for the five carts; the levels are later emulators' (tier later) |
| Provoke | Refused on the undead | The client's skill text: "Not appliable for the Undead" |
| Sonic Blow | Needs a Katar | The client's skill text: "using Katar" |
| Ice Wall | A cell someone stands on stays free | Nobody gets walled into a cell. The wall cannot be attacked yet |
| Vending | "There is nobody to sell to." | Single player |

**Engine:**

* **Monsters on maps nobody is on** stand still. Athena's "lazy" AI let them
  wander and warp now and then.
* **Parties.** Party-wide skills (Angelus, Magnificat, Gloria, Adrenaline
  Rush...) reach the caster only: there is no party.
* **Poison React, Remove Trap, Spring Trap** are drafts from the client's
  skill text alone (tier `guess`; no emulator of the period has them).
  Poison React answers one weapon blow of the Poison element with
  (100 + 30 x level)% and ends; Remove Trap takes up any Hunter trap and
  gives one Trap back; Spring Trap needs the falcon and destroys the trap.
  Not tried in the real client yet (does it let you click a trap?).
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
  description for any 2-2 skill, so there is no client text to read them
  against: only the two emulators and the iRO guide of spring 2004. Nor
  can it offer them: they are missing from its skill window, and only
  `@useskill` reaches them (THREADS.md #25).
  * Providence is mod0631's (5% a level against Holy and against demons,
    180 s); its 3 s of cast are 817's and the guide's.
  * The four endows last 20 minutes (817, the guide), not mod0631's 30 to
    90 s. Neither emulator lets them fail. They follow the guide: 80% to
    100% by level, and "weapon is destroyed on failure": the weapon in the
    target's hand is lost. Ours: no endow on bare hands.
  * The Strips take at (5 + 2 x level)% for 60 s (817, the guide's "Basic
    Chance"). 817's status does nothing to a monster; here, by the guide,
    Strip Weapon takes a tenth off its attack and each of the other three a
    tenth off its DEF. The guide's "DEX difference ... affects the success
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
* **Not built yet:** the other 2-2 skills, Talkie Box's text, Repair Weapon
  (nothing breaks in this era's data), attacking an Ice Wall, monsters'
  "friend" skill targets (mod0631 has none either). Venom Splasher is 817's
  plain splash, without the delayed burst of the client's skill text.
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

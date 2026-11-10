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
* **Commands 817's scripts used but its engine lacked**, where the meaning
  is unmistakable. 817 stopped the script at each of them; we run them:
  * `releaseall` (unequip everything) comes right before `jobchange` in the
    Merchant and Thief quests, so in 817 neither job change ever happened;
  * `scriptlabel` opens every town guide (a jAthena label marker), so no
    817 guide ever spoke. It is a no-op here;
  * `save` is the later alias of `savepoint` (the Prontera inns);
  * `else(cond) goto L` (Thief quest) is read as `if`.

  817's Wizard quest is an unfinished conversion it couldn't run; we use
  Athena 659's simple Wizard guild NPC (job level 40, no test) instead
  (THREADS.md #8).
* **First job needs Basic Skill 9** (`scripts/npc/jobs.lua`). The client's
  skill description says so; 817's guild NPCs checked only the job level, so
  a Novice could carry 9 unspent points into the new job. The refusal line is
  ours (official wording: THREADS.md #2).
* **"morroc"** in the Juno Kafras is a typo for `morocc`.
* **Shop names** come from 817 by cell (mod0631's are Japanese). "Shop" is
  used where 817 has none within 3 cells, or only a Japanese one.
* **Menus whose target label is missing** end the dialog. 817 jumped to the
  start of the script, which restarted the conversation on "Cancel".
* **`end;` with a message showing** gets a close button, so the window
  doesn't hang.

## Engine behaviour relative to Athena 817

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
research should check is in research/THREADS.md #20–#23.

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
* **Attack speed.** Not mod0631's slip but ours, fixed now: the server
  attacked twice as fast as the status window's ASPD says. The delay between
  two attacks is twice the "aspd" value (`pc.c:2461`).

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
* **Brandish Spear, Auto Counter, Signum Crucis, Slow Poison, Find Stone,
  Ruwach's damage, element-resisting cards against magic**: not in mod0631
  at all.

**Ours, where neither emulator has anything** (tier guess unless noted):

| Skill | What we do | From |
|---|---|---|
| Flasher | Blinds, at Sandman's chance | The client's skill text: "Blinds an Enemy". Chance ours |
| Shockwave Trap | Takes 20/35/50/65/80% of a player's SP | The client's skill text. Monsters have no SP here |
| Enchant Poison | Each hit may poison (3% to 7%) | mod0631 stores that chance on the status and never rolls it |
| Brandish Spear | Rows of five cells from the knight toward the target, 1 to 4 rows by level | 817 has its own helper for the rows; the per-row damage is 817's |
| Detect | Reveals the hidden within 3 cells of the spot | The client's skill text |
| Change Cart | Cart look by base level (41, 66, 81, 91) | The client's skill text for the five carts; the levels are later emulators' (tier later) |
| Provoke | Refused on the undead | The client's skill text: "Not appliable for the Undead" |
| Sonic Blow | Needs a Katar | The client's skill text: "using Katar" |
| Ice Wall | A cell someone stands on stays free | Nobody gets walled into a cell. The wall cannot be attacked yet |
| Vending | "There is nobody to sell to." | Single player |

**Engine:**

* **Range.** A skill or attack needs its target within range; the emulator
  also asked for a walkable path between the two ("no shooting over
  cliffs"). Not checked yet.
* **Invulnerability after a warp** (`ghost_time`, 5 s in which monsters
  ignore you) is not there.
* **Monsters on maps nobody is on** stand still. Athena's "lazy" AI let them
  wander and warp now and then.
* **Parties.** Party-wide skills (Angelus, Magnificat, Gloria, Adrenaline
  Rush...) reach the caster only: there is no party.
* **Not built yet:** the 2-2 jobs' skills, Poison React's counter, Venom Splasher, Talkie Box's text, Remove Trap,
  Spring Trap, Repair Weapon (nothing breaks in this era's data), attacking
  an Ice Wall, monsters' "friend" skill targets (mod0631 has none either).

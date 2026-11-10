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
| Rates | Closed beta: "higher experience", factor unknown. Open beta: presumably 1x | 1x | THREADS.md #1. `base_exp_rate`/`job_exp_rate`/`drop_rate` in the config |
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
  * Izlude's "Save inside City" saved nothing in 817. It saves at izlude
    94,103, as in mod0631 and jAthena (2004-01).
  * Typos in the Kafra lines are fixed. The wording is otherwise 817's.
* **Removed outright:** 817's Kafra-points exchange in Al De Baran. Its menu
  never worked: labels starting with digits parse as numbers in 817. Kafra
  points did exist (kRO notice of 2004-02-10); what they bought is
  THREADS.md #4.
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
* **Taming, pet hatching, `sc_start` (status changes), `produce` (forging),
  item skills, equipment-look `setlook`, card removal.** Scripts that call
  these continue, and the server logs once that it isn't implemented
  (ROADMAP).

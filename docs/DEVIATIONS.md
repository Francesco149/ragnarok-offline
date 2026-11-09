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
| Guilds, War of Emperium | Guild castles, flags, treasure, guild dungeons (WoE from open beta, Sundays) | Castle NPCs not imported; the castle maps and their warps exist but nobody owns them | Needs guilds and opponents. Single-player stand-in is a research thread (THREADS.md #17) |
| PvP | PvP rooms (Prontera/Izlude) | The imported PvP NPCs are off (`[content] pvp = false`) | Empty rooms. Turn it on to visit them |
| Trading, vending to players | Trade window, merchant shops | No other players. The client's buttons do nothing | — |
| Chat rooms | Player chat rooms; NPC "waiting rooms" (arena, PvP queues) | Not drawn; NPCs that open one log a notice | Player rooms are pointless. NPC waiting rooms are a TODO (ROADMAP) |
| Whisper, friends | — | Whispers get "not online" | — |
| Announcements | Server-wide broadcasts | Shown to you (`announce` and friends) | — |
| Arena (Izlude time attack) | Solo monster rooms | Imported as is | Solo by design: no change |

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
* **Kafra** (`scripts/npc/kafra.lua` replaces all 18 town and dungeon
  Kafras):
  * Fares are the ones 817's menus *showed*. Its code charged other fares in
    places: Prontera→Geffen showed 1400 and took 1800.
  * A cart costs 800z only when you get one. 817 took the money twice and
    then jumped to a missing label, so no cart.
  * Storage needs Basic Skill 6, as the client's own Basic Skill description
    says ("[Level 6] : Enable to use a Storage"). 817 asked for Novice job
    level 6.
  * No "Use Kafra Pass" entry. kRO removed the voucher menu on 2003-05-20,
    and 817's was "Under development".
  * "Cancel" and "Other Information Check" work (817 pointed them at missing
    labels). The location tip marks the town's Kafras.
  * Izlude's "Save inside City" saved nothing in 817. It now saves at
    izlude 91,105, where every era Kafra teleport lands.
  * Typos in the Kafra lines are fixed. The wording is otherwise 817's.
* **Removed outright:** 817's Kafra-points exchange in Al De Baran. Its menu
  never worked: labels starting with digits parse as numbers in 817. Whether
  Kafra points existed in Dec 2003 is THREADS.md #4.
* **Menus whose target label is missing** end the dialog. 817 jumped to the
  start of the script, which restarted the conversation on "Cancel".
* **`end;` with a message showing** gets a close button, so the window
  doesn't hang.

## Engine behaviour relative to Athena 817

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

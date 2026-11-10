# Architecture

## Principle: confirmed in Rust, reconstructed in Lua

* **Rust** owns what the client can confirm: wire format and lengths, the
  login/char/map hand-off, map cells (from the client's `.gat`), movement
  timing, what each packet means, persistence.
* **Lua** owns what only a period *server* decided: NPCs and quests, warps,
  shops, monster spawns, @commands, and the rules (`rules.*`: a character's
  status, damage, what each skill and status change does, ground skills,
  regen, death penalty). Each value carries its source; see
  [PROVENANCE.md](PROVENANCE.md).
* **Data** (items, monsters, jobs, EXP) is TOML in `data/db/`, tagged with a
  provenance tier, generated from era emulator data by
  `tools/import/athena_import.py` and meant to be corrected by hand.

## Crates

| crate | role |
|---|---|
| `ro-grf` | GRF 0x102/0x103 (Gravity's one-round DES) and 0x200 reader, layered `Vfs` (data.grf + .gpf patches + in-memory files), 0x200 writer used by `setup` |
| `ro-proto` | the client's packet length table (embedded), stream framer, little-endian `Writer`/`Reader` with RO's packed positions, packet id names |
| `ro-data` | `.gat` cell grids, the client's `#` text tables, the TOML game database with provenance tiers |
| `ro-game` | the simulation: `Game` (sessions on login/char/map), `World` (units, maps, view areas), movement + pathfinding, NPC dialogs, shops, items, monsters/combat, regen, Lua engine, JSON save store |
| `ro-server` | the `ragnarok-offline` binary: config, TCP listeners, admin console, script watcher, `setup`/`check`/`grf` tools |

## Runtime

```
 client ──TCP──> login :6900 ┐
         ──TCP──> char  :6121 ├─ tokio tasks: frame packets ──mpsc──┐
         ──TCP──> map   :5121 ┘                                     v
 console :6123 (127.0.0.1) ── Lua lines ─────────────────>  game thread
 script watcher (mtime poll) ── reload ──────────────────>  (Game: world + Lua)
                                                              │ every 20 ms: tick()
                 per-connection writer tasks <── Outbox ──────┘
```

One thread owns the whole game (Lua state included): no locks, events
processed in arrival order, `tick()` runs walks, monster AI, attacks, casts,
status changes and ground skills, regen, floor-item expiry, respawns and
autosave. Single player needs nothing more,
and it keeps behaviour deterministic and easy to reason about.

## The three "servers"

They share one process and one `Game`, but keep the original hand-off so the
client sees exactly what it expects: login issues `(login1, login2)` tokens →
char server checks them, sends the raw account id + list → selecting a slot
issues a map token → map server checks it, sends the raw account id + 0x0073.

## World

* Units: players (unit id = account id), NPCs (ids from 110000000), monsters
  and floor items (ids from 1000). Kept in one `HashMap`, indexed per map.
* View area: 14 cells (Athena `area_size`). Each player keeps the set of unit
  ids its client has been told about; moves and spawns diff against it.
* Maps load lazily from the client's `.gat`; NPCs/warps/shops are placed when
  a map loads, monster spawns when the first player arrives, and monsters
  only think on maps with players.
* Walking: Athena-style pathfinder, a cell takes `speed` ms (×1.4 diagonal),
  the unit's cell changes at mid-step like Athena's `path_half`.

## Scripts

`ScriptEngine` loads `prelude.lua` (embedded) then `scripts/rules/**` then
every other `scripts/**/*.lua`. Files *declare* things (`npc{}`, `warp{}`,
`shop{}`, `spawn{}`, `command()`, `on()`); the game instantiates them. Calls
into Lua bind a single scoped native, `_rt(op, ...)`, to the live game
(`host.rs`); the prelude wraps it in the friendly API. NPC dialogs are
coroutines that yield until the client clicks. A reload builds a new Lua
state and swaps it in only if everything loaded.

## Battle

`battle.rs` is the layer between the rules and the world: snapshots of
players and monsters for Lua, HP and SP, the status-change store (and the
icons and option words the client draws from it), cast timers, the cells of
ground skills as map units. `combat.rs` runs the attack timing and the
monster AI (targets, chase, loot, skill lists). What a hit, a skill or a
status change *does* is Lua (`scripts/rules/`), a port of the period
emulator's battle code.

A host operation never calls a world-bound script: the `_rt` binding
belongs to one script call at a time (`ScriptEngine::busy`). What an
operation wants scripted (a monster's counter skill, a player's Auto
Berserk) is deferred and runs when the current script returns.

## Persistence

`saves/accounts/<id>.json`, `saves/chars/<id>.json`, `saves/ids.json`; atomic
writes with a `.bak` of the previous version; deleted characters are renamed
`.deleted`, never removed. Saved on logout, every `autosave_secs` (120 s) and
on shutdown (Ctrl-C or `:shutdown` in the console).

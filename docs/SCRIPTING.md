# Scripting (Lua 5.4)

Everything under `scripts/` is Lua. Files are loaded in path order, with
`scripts/rules/` first. Save a file and the running server reloads it
(`watch_scripts = true`); `@reload` in game or `:reload` in the console do the
same. If a file fails to load, the server keeps the previous scripts and logs
the error (`:notices` in the console shows it).

```
scripts/
  rules/status.lua    derived stats (HP/SP/ATK/hit/flee/ASPD/weight)
  rules/combat.lua    melee damage, death penalty, respawn HP, regen
  commands.lua        @commands
  npc/*.lua           hand-written NPCs
  import/*.lua        generated from era data: warps, monster spawns, shops
```

## Declarations

```lua
npc {
  map = "prontera", x = 152, y = 187, dir = 4,   -- dir 0 N, 2 W, 4 S, 6 E
  sprite = 117, name = "Guide",                  -- sprite = client NPC job id
  on_click = function() ... end,                 -- dialog coroutine
  touch = { 1, 1 }, on_touch = function() ... end, -- optional: step-on area
}

warp { map = "prontera", x = 156, y = 22, span = { 2, 2 }, to = { "prt_fild08", 170, 375 } }

shop { map = "prontera", x = 100, y = 100, dir = 4, sprite = 83, name = "Tool Dealer",
       items = { 501, { 502, 250 } } }          -- id = db price, {id, price}

spawn { map = "prt_fild08", mob = 1002, count = 20 }             -- anywhere on the map
spawn { map = "prt_fild08", x = 100, y = 100, xs = 10, ys = 10,  -- in an area
        mob = "PORING", count = 5, delay = 5000, delay2 = 0 }    -- respawn ms

command("hello", function(args) return "hi " .. player.name end)  -- @hello
on("login", function() notify("Welcome back!") end)              -- also "map_enter"
```

Entries on maps the client does not have are skipped (`ragnarok-offline
check` lists them).

## Dialog functions (inside `on_click` / `on_touch`)

| | |
|---|---|
| `mes(text)` | add a line to the window |
| `next()` | "next" button, waits for the click |
| `close()` | "close" button, waits for the click |
| `menu("a", "b", ...)` | choice list → 1-based index, or `nil` if cancelled (cancel ends the dialog) |
| `input_number()` / `input_string()` | ask the player |
| `cutin(name, pos)` / `cutin()` | show / remove a cut-in picture (pos 0 left, 1 center, 2 right) |

If a script ends while text is showing, the server adds the close button.
Script errors show "[script error]" in the window and the trace goes to the
server log.

## Player and world

```lua
player.name, player.zeny, player.base_level, player.job_level, player.job,
player.hp, player.max_hp, player.sp, player.max_sp, player.str .. player.luk,
player.status_point, player.skill_point, player.base_exp, player.job_exp,
player.map, player.x, player.y, player.sex, player.weight, player.max_weight,
player.hair, player.hair_color, player.clothes_color, player.gm_level
player.zeny = player.zeny - 100        -- writable: zeny hp sp levels stats points exp hair colors

var("quest_x")            var("quest_x", 2)     clear_var("quest_x")   -- saved per character
give_item(501, 5)         take_item("Red_Potion", 1)   count_item(501)  -- id or db name
heal(hp, sp)              percent_heal(100, 100)
warp("geffen", 119, 59)   save_point("prontera", 116, 73)   job_change(1)
give_exp(base, job)       emotion(2)            notify("only you see this")
announce("everyone sees this")                   open_shop("Tool Dealer")
spawn_mob(map, x, y, mob, count) -> {ids}        nearby(range) -> {{id, kind, name, x, y, info}}
walkable(map, x, y)       map_size(map)          item_name(id)   mob_name(id)
online()                  log(text)              rand(a, b)
```

## Rules (formulas)

`rules.status(c)`, `rules.attack{attacker, defender}`,
`rules.death_penalty(t)`, `rules.respawn_hp(t)` and `rules.regen(t)` are
plain functions called by the server; their inputs and outputs are documented
at the top of `scripts/rules/*.lua`. Change a number, save, and the next hit
uses it. Cite where a formula comes from in a comment (see PROVENANCE.md).

## Console (for you or an LLM)

```sh
ragnarok-offline console            # interactive
ragnarok-offline eval 'nearby(5)'   # one line; runs as the first online player
```

Console lines are Lua expressions or statements with the full API above;
`:players`, `:reload`, `:save`, `:notices`, `:shutdown`, `:help` are built in.
Typical loop: edit a script → it reloads → `eval` to inspect state → repeat.

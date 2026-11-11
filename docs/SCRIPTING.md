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
  rules/refine.lua    refine success chances
  commands.lua        @commands
  import/*.lua        generated from era data: warps, monster spawns, shops
  import/npc/**.lua   the era's NPC scripts, translated (tools/import/athena_npc.py)
  npc/*.lua           hand-written NPCs; they can replace imported ones
```

Generated files are never edited by hand: fix an imported NPC by writing a
replacement in `scripts/npc/` and listing the change in DEVIATIONS.md.

## Declarations

```lua
npc {
  map = "prontera", x = 152, y = 187, dir = 4,   -- dir 0 N, 2 W, 4 S, 6 E
  sprite = 117, name = "Guide",                  -- sprite = client NPC job id; -1 = invisible
  on_click = function() ... end,                 -- dialog coroutine
  touch = { 1, 1 }, on_touch = function() ... end, -- optional: step-on area (default: on_click)
  id = "guide_prt",                              -- optional: name for events/enable (default: name)
  events = { "OnInit" },                         -- optional: labels on_click(label) accepts
  src = "817:cities/prontera.txt:2",             -- where it comes from (imports set this)
  replaces = "817:other/kafra.txt:2474",         -- optional: drop the imported NPC with this src
}
remove_npc("817:other/kafra.txt:995")            -- drop an imported NPC outright
wrap_npc("817:jobs/archer.txt:4", function(orig)  -- run your code around an imported NPC
  return function(at) if player.job_level < 10 then mes "Not yet." close() return end return orig(at) end
end)

warp { map = "prontera", x = 156, y = 22, span = { 2, 2 }, to = { "prt_fild08", 170, 375 } }
remove_warp("job_wiz", 99, 160)                  -- drop an imported warp by its own cell

shop { map = "prontera", x = 100, y = 100, dir = 4, sprite = 83, name = "Tool Dealer",
       items = { 501, { 502, 250 } } }          -- id = db price, {id, price}

spawn { map = "prt_fild08", mob = 1002, count = 20 }             -- anywhere on the map
spawn { map = "prt_fild08", x = 100, y = 100, xs = 10, ys = 10,  -- in an area
        mob = "PORING", count = 5, delay = 5000, delay2 = 0 }    -- respawn ms

command("hello", function(args) return "hi " .. player.name end)  -- @hello
on("login", function() notify("Welcome back!") end)              -- also "map_enter"
```

Entries on maps the client does not have are skipped (`ragnarok-offline
check` lists them). Map `"-"` is a floating NPC: no unit, only events.

`content("juno")` reads the `[game.content]` switches (juno, jobs2_2, pvp,
events); an imported file for later content starts with
`if not content("juno") then return end`.

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

## Variables

```lua
v.quest = 2        -- per character, saved          (Athena: quest)
tmp.step = 1       -- per character, until logout   (Athena: @step, l0..)
g.event_on = 1     -- server-wide, saved            (Athena: $event_on)
acct.bonus = 1     -- per account, saved            (Athena: #bonus)
```

Unset variables read as 0 (or "" if the name ends in `$`); setting 0
deletes. `var("x")` (nil when unset) still works.

## Events and timers

An NPC's `on_click(at)` gets the event label when started by an event:

```lua
npc { map = "-", name = "Bells", events = { "OnInit", "OnTimer60000" },
  on_click = function(at)
    if at == "OnInit" then npc_timer_start() end   -- OnTimer<ms> labels fire in order
    if at == "OnTimer60000" then announce("The bells ring.", 0) end
  end }
do_event("Bells::OnInit")                 -- run an NPC's label (player attached)
add_timer(5000, "Bells::OnTimer60000")    -- run it in 5 s for this player
monster("prt_fild08", 0, 0, "--ja--", 1002, 3, "Bells::OnMyMobDead") -- event on each death
```

`OnInit` runs after every (re)load. Imported scripts use the same scheme:
the translator turns each Athena `On...` label into an entry point.

## Athena builtins

The translated NPCs call snake_case versions of the Athena commands, all
available to hand-written scripts too: `job_change`, `warp` (also
"SavePoint"/"Random"), `area_warp`, `map_warp`, `set_look`, `check_weight`,
`viewpoint`, `item_heal`, `read_param`, `char_info`, `equip(num)` and
`equip_name/worn/refinable/refine/weapon_level/refine_chance`,
`refine_success`, `refine_fail`, `give_skill`, `skill_level`,
`sc_start(type, val1, val2)` / `sc_end(type)` (Athena SC numbers; what
each does is `rules.status_change`), `check_option`, `set_option`,
`set_cart`, `set_falcon`, `set_riding`,
`open_storage`, `monster`, `area_monster`, `kill_monster`, `mob_count`,
`map_announce`, `area_announce`, `get_users`, `map_users`, `area_users`,
`enable_npc`, `disable_npc`, `npc_emotion`, `reset_stats`, `reset_skills`.
The mapping is the `BUILTINS` table in `tools/import/athena_npc.py`; ones
not implemented yet log a notice once (`:notices`).

## Rules (formulas, battle, skills)

`scripts/rules/` holds what a period server decided and the client cannot
confirm. Change a number, save, and the next hit uses it. Cite where a
formula comes from in a comment (see PROVENANCE.md).

| File | What |
|---|---|
| `00_const.lua` | `SC` (status change numbers), `SK` (skill ids by Aegis name), `skill_get_*`, the emulator's battle settings |
| `status.lua` | `rules.status(c)`: a character's numbers from stats, job, equipment, cards, skills and status changes |
| `battle.lua` | `battle.*`: a unit's numbers, the element table, weapon / magic / misc damage, `rules.weapon_attack` |
| `status_change.lua` | `skill_status_change_start / _end / _timer`: what each status change does and for how long |
| `skill.lua` | using a skill: `rules.skill_use` (range, cast time, delay), `skill_check_condition` (SP, items, weapon), `rules.skill_castend`, `skill_attack`, side effects of a hit |
| `skill_effects.lua` | what each skill does once cast |
| `skill_units.lua` | ground skills: cells, who they act on, how long |
| `combat.lua` | `rules.respawn` (HP and EXP lost on returning from death), `rules.regen`, `item_heal`, shop prices |
| `produce.lua` | forging and ore refining |
| `refine.lua` | `rules.refine_chance` |

The server calls `rules.weapon_attack{src, target}` when an attack comes
due, `rules.skill_use{...}` when the client asks for a skill,
`rules.skill_castend{...}` when the cast is over, and `rules.battle_tick`
every 100 ms (status changes that ran out, ground skills, delayed hits).
The rules act on the world through the battle API (`_rt` operations wrapped
in `battle.lua`): `battle.get(id)` is a snapshot of a player or monster,
`battle.damage`, `battle.heal`, `skill_status_change_start(id, type, val1,
val2)`, `skill_blown`, `skill_unitsetting`, `battle.after(ms, fn)`.

To try a skill from the console or the chat: `@allskill`, then
`@useskill MG_FIREBOLT 10` (on the nearest monster, yourself or its cell,
as the skill's target type says).

## Console (for you or an LLM)

```sh
ragnarok-offline console            # interactive
ragnarok-offline eval 'nearby(5)'   # one line; runs as the first online player
```

Console lines are Lua expressions or statements with the full API above;
`:players`, `:reload`, `:save`, `:notices`, `:shutdown`, `:help` are built in.
Typical loop: edit a script → it reloads → `eval` to inspect state → repeat.

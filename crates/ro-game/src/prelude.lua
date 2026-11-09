-- ragnarok-offline script prelude (embedded in the server binary).
-- Everything here wraps one native function, _rt(op, ...), which the server
-- binds to the live world for the duration of each script call.
-- Reference: docs/SCRIPTING.md

-- ---------------------------------------------------------------- dialogs
-- Show a line in the NPC window.
function mes(text) _rt("mes", tostring(text)) end
-- "Next" button; waits for the click.
function next() _rt("next"); coroutine.yield("next") end
-- "Close" button; waits for the click, then the window closes.
function close() _rt("close"); coroutine.yield("close") end
-- Menu; returns the chosen index (1-based) or nil if cancelled.
function menu(...)
  local opts = { ... }
  if type(opts[1]) == "table" then opts = opts[1] end
  _rt("menu", opts)
  local choice = coroutine.yield("menu")
  if choice == nil or choice == 255 or choice < 1 or choice > #opts then return nil end
  return choice
end
function input_number() _rt("input_number"); return coroutine.yield("input_number") end
function input_string() _rt("input_string"); return coroutine.yield("input_string") end
-- Cut-in picture: pos 0 left, 1 center, 2 right; cutin() removes it.
function cutin(name, pos) _rt("cutin", name or "", name and (pos or 2) or 255) end

-- ---------------------------------------------------------------- player
-- player.name, player.zeny, player.base_level, player.job, player.hp, ...
-- Assignments write through: player.zeny = player.zeny - 100
player = setmetatable({}, {
  __index = function(_, k) return _rt("player_get", k) end,
  __newindex = function(_, k, v) _rt("player_set", k, v) end,
})
-- Persistent per-character variable (quest state).
function var(name, value)
  if value == nil then return _rt("var_get", name) end
  _rt("var_set", name, value)
end
function clear_var(name) _rt("var_clear", name) end

function give_item(id, amount) return _rt("give_item", id, amount or 1) end
function take_item(id, amount) return _rt("take_item", id, amount or 1) end
function count_item(id) return _rt("count_item", id) end
function heal(hp, sp) _rt("heal", hp or 0, sp or 0) end
function percent_heal(hp, sp) _rt("percent_heal", hp or 0, sp or 0) end
function sc_end(name) _rt("sc_end", name) end
-- warp{map=,x=,y=,to={...}} declares a portal; warp("map", x, y) teleports the player.
local declare_warp = warp
function warp(map, x, y)
  if type(map) == "table" then return declare_warp(map) end
  _rt("warp", map, x or 0, y or 0)
end
function save_point(map, x, y) _rt("save_point", map, x, y) end
function job_change(job) _rt("job_change", job) end
function give_exp(base, job) _rt("give_exp", base or 0, job or 0) end
function emotion(e, on_npc) _rt("emotion", e, on_npc and true or false) end
-- Chat line only this player sees.
function notify(text) _rt("notify", tostring(text)) end
-- Yellow broadcast to everyone.
function announce(text) _rt("announce", tostring(text)) end
function open_shop(npc_name) _rt("open_shop", npc_name) end

-- ---------------------------------------------------------------- world
function spawn_mob(map, x, y, mob, count) return _rt("spawn_mob", map, x, y, mob, count or 1) end
-- Cell checks against the client's .gat (nil if the client lacks the map).
function walkable(map, x, y) return _rt("walkable", map, x, y) end
function map_size(map) return _rt("map_size", map) end
-- Units within `range` cells of the player: { {id, kind, name, x, y, info}, ... }
function nearby(range) return _rt("nearby", range or 14) end
function item_name(id) return _rt("item_name", id) end
function mob_name(id) return _rt("mob_name", id) end
function online() return _rt("online") end
function log(text) _rt("log", tostring(text)) end
function reload() return _rt("reload") end

function rand(a, b)
  if b == nil then return math.random(0, a - 1) end
  return math.random(a, b)
end

-- rules: game formulas; scripts/rules/*.lua fill this table.
rules = rules or {}

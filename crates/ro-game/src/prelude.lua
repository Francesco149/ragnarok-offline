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
-- Status changes by Athena SC number (SC_* constants, scripts/import/athena_const.lua);
-- durations/values: rules.status_change (scripts/rules/status_change.lua)
function sc_start(kind, val1, val2) return _rt("sc_start", kind, val1 or 0, val2 or 0) end
function sc_end(kind) _rt("sc_end", kind) end
-- warp{map=,x=,y=,to={...}} declares a portal; warp("map", x, y) teleports the player.
local declare_warp = warp
-- Returns false when refused (Fly Wing on a noteleport map).
function warp(map, x, y)
  if type(map) == "table" then return declare_warp(map) end
  return _rt("warp", map, x or 0, y or 0)
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

-- ---------------------------------------------------------------- content
-- content("juno") etc.: switches from ragnarok-offline.toml [content].
function content(name) return CONTENT and CONTENT[name] == true end

-- ---------------------------------------------------------------- variables
-- v.x    permanent per character (Athena `x`)
-- tmp.x  until logout (Athena `@x`, `l0`..)
-- g.x    server-wide, saved (Athena `$x`)
-- acct.x per account, saved (Athena `#x`)
-- Unset variables read as 0 (or "" when the name ends in "$"), like Athena.
local function default_of(k) return (type(k) == "string" and k:sub(-1) == "$") and "" or 0 end
local function var_table(get, set)
  return setmetatable({}, {
    __index = function(_, k) local x = _rt(get, k); if x == nil then return default_of(k) end; return x end,
    __newindex = function(_, k, val)
      if val == 0 or val == "" or val == false then val = nil end -- Athena deletes zero variables
      _rt(set, k, val)
    end,
  })
end
v = var_table("var_get", "var_set")
tmp = var_table("tvar_get", "tvar_set")
g = var_table("gvar_get", "gvar_set")
acct = var_table("avar_get", "avar_set")

-- ---------------------------------------------------------------- Athena builtins
-- Names the converter (tools/import/athena_npc.py) emits; also fine to use by hand.
local function b(x) return x and 1 or 0 end

function job_change(job) _rt("job_change", job) end
function area_warp(map, x1, y1, x2, y2, to, x, y) _rt("area_warp", map, x1, y1, x2, y2, to, x, y) end
function map_warp(from, to, x, y) _rt("map_warp", from, to, x, y) end
-- setlook types: 1 hair, 6 hair color, 7 clothes color (others: equipment look)
function set_look(kind, val)
  if kind == 1 then player.hair = val
  elseif kind == 6 then player.hair_color = val
  elseif kind == 7 then player.clothes_color = val
  else _rt("unimplemented", "setlook " .. tostring(kind)) end
end
function check_weight(item, n) return _rt("check_weight", item, n or 1) end
function viewpoint(kind, x, y, id, color) _rt("viewpoint", kind, x, y, id, color) end
function item_heal(hp, sp) heal(hp, sp) end -- rules/combat.lua replaces it (VIT, INT and skills count)
-- readparam(n): Athena SP_* numbers
local params = { [1] = "base_exp", [2] = "job_exp", [5] = "hp", [6] = "max_hp", [7] = "sp", [8] = "max_sp",
  [9] = "status_point", [11] = "base_level", [12] = "skill_point", [13] = "str", [14] = "agi", [15] = "vit",
  [16] = "int", [17] = "dex", [18] = "luk", [19] = "class", [20] = "zeny", [21] = "sex", [22] = "next_base_exp",
  [23] = "next_job_exp", [24] = "weight", [25] = "max_weight", [55] = "job_level" }
function read_param(n) local k = params[n]; return k and player[k] or 0 end
-- getcharid: 0 character, 1 party, 2 guild, 3 account
function char_id(n)
  if n == 0 then return player.char_id elseif n == 3 then return player.account_id end
  return 0
end
-- single player: no parties or guilds (docs/DEVIATIONS.md)
function party_name(id) return "" end
function guild_name(id) return "" end
function guild_master(id) return "" end
-- strcharinfo: 0 name, 1 party name, 2 guild name
function char_info(n) if n == 0 then return player.name end; return "" end
function gm_level() return player.gm_level end

-- equipment slots: 1 head top, 2 armor, 3 left hand, 4 right hand, 5 garment,
-- 6 shoes, 7 accessory 1, 8 accessory 2, 9 head mid, 10 head low
local slot_names = { "Head", "Body", "Left hand", "Right hand", "Robe", "Shoes", "Accessory 1", "Accessory 2",
  "Head 2", "Head 3" }
function equip(num) return _rt("equip_info", num) end
function equip_name(num)
  local e = equip(num)
  return (slot_names[num] or "?") .. "-[" .. (e and e.name or "Not Equipped") .. "]"
end
function equip_worn(num) return b(equip(num) ~= nil) end
function equip_refinable(num) local e = equip(num); return b(e ~= nil and e.refinable) end
function equip_identified(num) local e = equip(num); return b(e ~= nil and e.identified) end
function equip_refine(num) local e = equip(num); return e and e.refine or 0 end
function equip_weapon_level(num) local e = equip(num); return e and e.weapon_level or 0 end
-- success chance of the next refine, from rules.refine_chance (scripts/rules/refine.lua)
function equip_refine_chance(num)
  local e = equip(num)
  if not e then return 0 end
  local p = rules.refine_chance and rules.refine_chance(e.weapon_level, e.refine) or 0
  p = p + skill_level(218) -- BS_WEAPONRESEARCH (Athena pc_percentrefinery)
  return math.max(0, math.min(100, p))
end
function refine_success(num) _rt("refine_result", num, true) end
function refine_fail(num) _rt("refine_result", num, false) end

-- statusup(type): raise a stat by one as if the player clicked it (SP_STR=13 ..)
function status_up(kind) _rt("unimplemented", "statusup") end
function status_up2(kind, n) _rt("unimplemented", "statusup2") end
function bonus() end
function bonus2() end
function bonus3() end

function give_skill(id, lv, flag) _rt("give_skill", id, lv) end
function skill_level(id) return _rt("skill_level", id) end
function basic_skill_check() return 1 end -- Athena basic_skill_check: on

function check_option(n) return b((_rt("option_get") & n) ~= 0) end
function set_option(n) _rt("option_set", n) end
function set_cart() return _rt("set_cart") end
function set_falcon() return _rt("set_falcon") end
function set_riding() return _rt("set_riding") end
function open_storage() _rt("open_storage") end
function unequip_all() _rt("unequip_all") end
function item_skill(id, lv, name) _rt("unimplemented", "itemskill " .. tostring(id)) end
-- Old Blue Box (1), Old Violet Box (2), Old Card Album (3), Gift Box (4), Old Scroll (5)
function item_box(flag) return _rt("item_box", flag) end
-- Dead Branch: a random monster next to you (data/db/random.toml branch list)
function dead_branch() return _rt("dead_branch") end
function produce(kind) _rt("unimplemented", "produce") end -- rules/produce.lua replaces it

function monster(map, x, y, name, mob, n, event) return _rt("monster", map, x, y, name, mob, n, event or "") end
function area_monster(map, x1, y1, x2, y2, name, mob, n, event)
  return _rt("area_monster", map, x1, y1, x2, y2, name, mob, n, event or "")
end
function kill_monster(map, event) _rt("kill_monster", map, event) end
function mob_count(map, event) return _rt("mob_count", map, event) end
function do_event(ev) _rt("do_event", ev) end
function add_timer(ms, ev) _rt("add_timer", ms, ev) end
function del_timer(ev) _rt("del_timer", ev) end
function add_timer_count(ev, ms) _rt("add_timer_count", ev, ms) end
function npc_timer_start(npc) _rt("npc_timer", 1, npc) end
function npc_timer_stop(npc) _rt("npc_timer", 0, npc) end
function npc_timer(on, npc) _rt("npc_timer", on, npc) end
function cmd_other_npc(npc, cmd) do_event(npc .. "::OnCommand" .. cmd) end
-- announce(text, flag): flag 0 everyone, 1 map, 2 area, 3 self; +0x10 blue
function announce(text, flag) _rt("announce", tostring(text), flag or 0) end
function map_announce(map, text, flag) _rt("map_announce", map, tostring(text), flag or 0) end
function area_announce(map, x1, y1, x2, y2, text, flag) _rt("area_announce", map, x1, y1, x2, y2, tostring(text), flag or 0) end
function get_users(flag) return _rt("get_users", flag or 0) end
function map_users(map) return _rt("map_users", map) end
function area_users(map, x1, y1, x2, y2) return _rt("area_users", map, x1, y1, x2, y2) end
function enable_npc(id) _rt("enable_npc", id) end
function disable_npc(id) _rt("disable_npc", id) end
function enable_arena() _rt("unimplemented", "enablearena") end
function disable_arena() _rt("unimplemented", "disablearena") end
-- taming item: the client asks which monster (it must be `mob`)
function catch_pet(mob) _rt("catch_pet", mob) end
-- Pet Incubator: the client lists your eggs
function birth_pet() _rt("birth_pet") end
function reset_stats() _rt("reset_stats") end
function reset_skills() _rt("reset_skills") end
function change_base(class) _rt("unimplemented", "changebase") end
function change_sex() _rt("unimplemented", "changesex") end
-- chat-room bubble over an NPC (arena/PvP queues): not drawn yet
function waiting_room(title, limit, event) _rt("waiting_room", tostring(title), limit or 0, event or "") end
function warp_waiting_pc(map, x, y) _rt("unimplemented", "warpwaitingpc") end
function map_flag_nosave(map, save_map, x, y) end
function set_map_flag(map, flag) end
function remove_map_flag(map, flag) end
function pvp_on(map) end
function pvp_off(map) end
function gvg_on(map) end
function gvg_off(map) end
function npc_emotion(e) _rt("npc_emotion", e) end
-- guilds and castles: no guilds in single player (scripts/npc/guild.lua)
function map_respawn_guild(map, guild, flag) end
function agit_start() end
function agit_end() end
function castle_name(map) return "" end
function castle_data(map, idx) return 0 end
function set_castle_data(map, idx, val) end
function equip_card_count(num) return 0 end
function remove_cards(num) _rt("unimplemented", "successremovecards") end
function remove_cards_failed(num, kind) _rt("unimplemented", "failedremovecards") end
-- a command the Athena of the time didn't have either: the script stopped there
function athena_unknown(name, ...) _rt("unimplemented", "athena command " .. name); error("athena: no command " .. name) end

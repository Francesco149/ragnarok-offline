-- Ground skills: Safety Wall, Fire Wall, Sanctuary, storms, traps, portals.
-- A skill puts a *group* of cells on the map; each cell acts on who stands
-- in its range. Server-side: NOT in the client.
--
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/skill.c (archived
-- 2003-12-18): skill_unitsetting (1988), skill_unit_onplace (2335), _onout
-- (2581), _ondelete (2676), _onlimit (2720), skill_unit_timer_sub (4442),
-- skill_unit_move (4525), skill_delunit (4189). Line refs are to that file.
--
-- The server only knows a cell as something on the map that clients see
-- (crates/ro-game/src/battle.rs su_new / su_del); groups, timing and
-- effects live here. They do not survive a script reload (the server
-- removes the cells with the old scripts).

local floor, random = math.floor, math.random
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end
local get = battle.get
local sc = battle.sc

-- (the script API's next() shadows Lua's)
local function empty(t) for _ in pairs(t) do return false end return true end

local groups = {}  -- id -> group
local cells = {}   -- cell id -> cell
local next_group = 10

-- what the client draws for each skill (skill.c:204 skill_get_unit_id)
local UNIT_ID = {
  [SK.MG_SAFETYWALL] = 0x7e, [SK.MG_FIREWALL] = 0x7f, [SK.PR_BENEDICTIO] = 0x82, [SK.PR_SANCTUARY] = 0x83,
  [SK.PR_MAGNUS] = 0x84, [SK.AL_PNEUMA] = 0x85, [SK.WZ_METEOR] = 0x86, [SK.WZ_VERMILION] = 0x86,
  [SK.WZ_STORMGUST] = 0x86, [SK.WZ_ICEWALL] = 0x8d, [SK.WZ_QUAGMIRE] = 0x8e, [SK.HT_BLASTMINE] = 0x8f,
  [SK.HT_SKIDTRAP] = 0x90, [SK.HT_ANKLESNARE] = 0x91, [SK.AS_VENOMDUST] = 0x92, [SK.HT_LANDMINE] = 0x93,
  [SK.HT_SHOCKWAVE] = 0x94, [SK.HT_SANDMAN] = 0x95, [SK.HT_FLASHER] = 0x96, [SK.HT_FREEZINGTRAP] = 0x97,
  [SK.HT_CLAYMORETRAP] = 0x98,
}
UNIT_ID[-1] = nil
local function unit_id(skill, flag)
  if skill == SK.AL_WARP then return flag == 0 and 0x81 or 0x80 end        -- opening, open
  if skill == SK.WZ_FIREPILLAR then return flag == 0 and 0x87 or 0x88 end  -- waiting, burning
  if skill == SK.HT_TALKIEBOX then return flag == 0 and 0x99 or 0x8c end
  return UNIT_ID[skill] or 0
end

local map_calc_dir = battle.calc_dir

local function del_cell(cell) end -- forward

-- skill.c:4297 skill_delunitgroup
function skill_delunitgroup(g)
  if not groups[g.id] then return end
  for _, id in ipairs(g.cells) do
    local c = cells[id]
    if c and c.alive then del_cell(c, true) end
  end
  groups[g.id] = nil
end

-- skill.c:1988. Returns the group.
function skill_unitsetting(src_id, skillid, skilllv, x, y, flag)
  local src = get(src_id)
  if not src then return nil end
  local now = _rt("now")
  local count, limit, val1, val2 = 1, 10000, skilllv, 0
  local target, interval, range = BCT.ENEMY, 1000, 0
  local dir = 0
  local shelter = battle_config.defnotenemy and BCT.NOENEMY or BCT.ALL

  if skillid == SK.MG_SAFETYWALL then
    limit = 5000 * skilllv
    val2 = skilllv + 1 -- hits it stops
    target = shelter
  elseif skillid == SK.MG_FIREWALL then
    dir = map_calc_dir(src, x, y)
    count = (dir & 1) ~= 0 and 5 or 3
    val2 = 4 + skilllv -- hits each cell deals
    limit = 1000 * val2
    interval = 250
  elseif skillid == SK.AL_PNEUMA then
    target = shelter
    range = 1
  elseif skillid == SK.AL_WARP then
    target = BCT.ALL
    val1 = skillid + 6 -- people it lets through: 33 (skill.c:2018; the skill's id, not its level, is what the source adds)
    limit = flag == 0 and 2000 or 5000 * (1 + skilllv)
  elseif skillid == SK.PR_SANCTUARY then
    count = 21
    limit = 1000 * (3 * skilllv + 1)
    val1 = skilllv + 3 -- people it heals
    val2 = skilllv > 6 and 777 or skilllv * 100
    target = BCT.ALL
    range = 1
  elseif skillid == SK.PR_MAGNUS then
    count = 33
    limit = 1000 * (skilllv + 4)
    interval = 3000
  elseif skillid == SK.WZ_FIREPILLAR then
    limit = flag == 0 and 30000 or 1000
    interval = 2000
    val1 = skilllv + 2
    range = 1
  elseif skillid == SK.WZ_METEOR then
    limit, interval, range = 500, 500, 2
  elseif skillid == SK.WZ_VERMILION then
    limit, interval, range = 3500, 1500, 4
  elseif skillid == SK.WZ_ICEWALL then
    limit = 4000 * (1 + skilllv)
    count = 5
  elseif skillid == SK.WZ_STORMGUST then
    limit, interval, range = 2000 + skilllv * 300, 450, 4
  elseif skillid == SK.WZ_QUAGMIRE then
    limit, interval, count = 5000 * skilllv, 200, 25
  elseif skillid == SK.HT_SKIDTRAP then limit, range = 60000 * (6 - skilllv), 1
  elseif skillid == SK.HT_LANDMINE then limit, range = 40000 * (6 - skilllv), 1
  elseif skillid == SK.HT_ANKLESNARE then
    limit, range = 50000 * (6 - skilllv), 1
    val1 = skilllv * 5000
    interval = val1 + 5000
  elseif skillid == SK.HT_SHOCKWAVE then
    limit, range = 40000 * (6 - skilllv), 1
    val1 = skilllv * 15 + 10
  elseif skillid == SK.HT_SANDMAN or skillid == SK.HT_FLASHER or skillid == SK.HT_FREEZINGTRAP then
    limit, range = 30000 * (6 - skilllv), 1
  elseif skillid == SK.HT_BLASTMINE then
    limit, interval, range = 5000 * (6 - skilllv), 3000, 1
  elseif skillid == SK.HT_CLAYMORETRAP then
    limit, interval, range = 20000 * skilllv, 3000, 1
  elseif skillid == SK.AS_VENOMDUST then
    limit, interval, count = 5000 * skilllv, 1000, 5
  elseif skillid == SK.HT_TALKIEBOX then
    limit, range, target = 600000, 1, BCT.ALL -- 817 cast_db: 600 s
  end

  -- skill.c:4223: a player keeps 32 groups, a monster 4; the oldest makes room
  local mine = {}
  for _, og in pairs(groups) do if og.src == src_id then mine[#mine + 1] = og end end
  if #mine >= (src.kind == "player" and 32 or 4) then
    table.sort(mine, function(p, q) return p.tick < q.tick or (p.tick == q.tick and p.id < q.id) end)
    skill_delunitgroup(mine[1])
  end

  local g = {
    id = next_group, src = src_id, map = src.map, skill = skillid, lv = skilllv, unit_id = unit_id(skillid, flag & 1),
    tick = now, limit = limit, interval = interval, range = range, target = target, val1 = val1, val2 = val2,
    cells = {}, alive = 0, ticks = {}, vallist = {},
  }
  next_group = next_group + 1
  groups[g.id] = g

  for i = 0, count - 1 do
    local ux, uy, cval2, crange, alive = x, y, 0, 0, true
    if skillid == SK.MG_FIREWALL then
      if (dir & 1) ~= 0 then
        -- diagonal: a staircase of five
        local dxs = { { 1, 1, 0, 0, -1 }, { -1, -1, 0, 0, 1 } }
        local dys = { { 1, 0, 0, -1, -1 }, { 1, 0, 0, -1, -1 } }
        ux = ux + dxs[((dir >> 1) & 1) + 1][i + 1]
        uy = uy + dys[((dir >> 1) & 1) + 1][i + 1]
      elseif dir % 4 == 0 then ux = ux + i - 1 else uy = uy + i - 1 end
      cval2 = g.val2
    elseif skillid == SK.PR_SANCTUARY then
      local dx = { -1, 0, 1, -2, -1, 0, 1, 2, -2, -1, 0, 1, 2, -2, -1, 0, 1, 2, -1, 0, 1 }
      local dy = { -2, -2, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2 }
      ux, uy = ux + dx[i + 1], uy + dy[i + 1]
    elseif skillid == SK.PR_MAGNUS then
      local dx = { -1, 0, 1, -1, 0, 1, -3, -2, -1, 0, 1, 2, 3, -3, -2, -1, 0, 1, 2, 3, -3, -2, -1, 0, 1, 2, 3, -1, 0, 1, -1, 0, 1 }
      local dy = { -3, -3, -3, -2, -2, -2, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3 }
      ux, uy = ux + dx[i + 1], uy + dy[i + 1]
    elseif skillid == SK.WZ_ICEWALL then
      -- five cells across the line from the caster; each blocks the way
      local dirx = { [0] = 0, -1, -1, -1, 0, 1, 1, 1 }
      local diry = { [0] = 1, 1, 0, -1, -1, -1, 0, 1 }
      dir = map_calc_dir(src, x, y)
      ux = ux + (2 - i) * diry[dir]
      uy = uy + (i - 2) * dirx[dir]
      local c = _rt("cell", src.map, ux, uy)
      if c == 5 or c == 1 or #_rt("in_area", src.map, ux, uy, ux, uy) > 0 then
        alive = false -- a wall, or someone standing there (ours: nobody gets walled in)
      else
        _rt("set_cell", src.map, ux, uy, false)
      end
    elseif skillid == SK.WZ_QUAGMIRE then
      ux, uy = ux + (i % 5 - 2), uy + (i // 5 - 2)
    elseif skillid == SK.AS_VENOMDUST then
      local dx = { -1, 0, 0, 0, 1 }
      local dy = { 0, -1, 0, 1, 0 }
      ux, uy = ux + dx[i + 1], uy + dy[i + 1]
    end
    if alive then
      local id = _rt("su_new", src.map, ux, uy, g.unit_id, src_id, g.id)
      cells[id] = { id = id, group = g.id, map = src.map, x = ux, y = uy, val1 = skilllv, val2 = cval2,
                    limit = limit, range = crange, alive = true }
      g.cells[#g.cells + 1] = id
      g.alive = g.alive + 1
    end
  end
  if g.alive == 0 then groups[g.id] = nil end
  return g
end

-- Who a cell acts for: its group's caster (battle_check_target with a skill unit).
local function owner_of(g) return get(g.src) end

local function in_cell_range(c, g, bl)
  local range = c.range ~= 0 and c.range or g.range
  if range < 0 then return false end
  return bl.map == c.map and math.abs(bl.x - c.x) <= range and math.abs(bl.y - c.y) <= range
end

-- A Safety Wall took a hit (battle.lua calc_damage).
function skill_safetywall_hit(bl, cell_id)
  local c = cells[cell_id]
  if not c or not c.alive then return end
  local g = groups[c.group]
  if not g then return end
  g.val2 = g.val2 - 1
  if g.val2 <= 0 then del_cell(c) end
end

-- skill.c:2335 skill_unit_onplace: `bl` stands in the cell's range.
local function onplace(c, bl, now)
  local g = groups[c.group]
  if not g or not c.alive or bl.dead then return end
  local ss = owner_of(g)
  if not ss then return end
  local last = g.ticks[bl.id]
  if last and not (now - last > g.interval) then return end
  g.ticks[bl.id] = now
  local uid = g.unit_id
  local cell_src = { id = c.id, x = c.x, y = c.y, kind = "cell" }

  if uid == 0x83 then -- Sanctuary
    local race = battle.race(bl)
    local evil = race == 1 or race == 6
    if battle.hp(bl) >= battle.max_hp(bl) and not evil then return end
    -- it serves a limited number of people
    if not g.vallist[bl.id] then
      g.vallist[bl.id] = true
      g.val1 = g.val1 - 1
      if g.val1 < 0 then skill_delunitgroup(g); return end
    end
    if not evil then
      local heal = g.val2
      if battle.dcard(bl, 4128) then heal = 0 end
      _rt("clif_skill_nodamage", c.id, bl.id, SK.AL_HEAL, heal, 1)
      battle.heal(bl.id, heal, 0)
    else
      skill_attack(BF.MAGIC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
    end
  elseif uid == 0x84 then -- Magnus Exorcismus: demons and the undead
    local race = battle.race(bl)
    if race ~= 1 and race ~= 6 then return end
    skill_attack(BF.MAGIC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
  elseif uid == 0x85 or uid == 0x7e then -- Pneuma, Safety Wall
    local type = uid == 0x85 and SC.PNEUMA or SC.SAFETYWALL
    local t = sc(bl, type)
    local left = g.limit -- skill.c:3993: the group's whole time; leaving or the cell's end stops it sooner
    if not t then
      skill_status_change_start(bl.id, type, g.val1, c.id, { tick = left })
    elseif t.val2 ~= c.id then
      local other = cells[t.val2]
      local og = other and groups[other.group]
      if not og or (uid == 0x85 and g.tick > og.tick) or (uid == 0x7e and g.val1 < og.val1) then
        skill_status_change_start(bl.id, type, g.val1, c.id, { tick = left })
      end
      g.ticks[bl.id] = now - g.interval
    end
  elseif uid == 0x86 then -- Lord of Vermilion, Storm Gust, Meteor Storm
    skill_attack(BF.MAGIC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
  elseif uid == 0x7f then -- Fire Wall: each cell burns a number of times
    if c.val2 > 0 then
      c.val2 = c.val2 - 1
      skill_attack(BF.MAGIC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
    end
    if c.val2 <= 0 then del_cell(c) end
  elseif uid == 0x87 then -- Fire Pillar, stepped on: it goes off
    del_cell(c)
    skill_unitsetting(g.src, g.skill, g.lv, c.x, c.y, 1)
  elseif uid == 0x88 then
    skill_attack(BF.MAGIC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
  elseif uid == 0x90 then -- Skid Trap: slides the victim, gone 3 s later
    if g.val2 == 0 then
      -- one cell at a time, backwards from the way the victim faces
      for _ = 1, g.lv do skill_blown(cell_src, bl.id, 1, true) end
      _rt("clif_fixpos", bl.id)
      g.limit = now - g.tick + 3000
      g.val2 = bl.id
    end
  elseif uid == 0x93 then -- Land Mine
    skill_attack(BF.MISC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
    del_cell(c)
  elseif uid == 0x8f or uid == 0x97 or uid == 0x98 then -- Blast Mine, Freezing Trap, Claymore Trap
    skill_attack(uid == 0x97 and BF.WEAPON or BF.MISC, g.src, cell_src, bl.id, g.skill, g.lv, 0, g.val2 ~= 0 and { type = 5 } or nil)
    if g.val2 == 0 then
      g.limit = now - g.tick + 500
      g.val2 = bl.id
    end
  elseif uid == 0x95 or uid == 0x96 or uid == 0x94 or uid == 0x91 then -- Sandman, Flasher, Shockwave, Ankle Snare
    skill_additional_effect(g.src, bl.id, g.skill, g.lv, BF.MISC)
    if g.val2 == 0 then
      local hold = 500
      if uid == 0x91 then hold = floor(g.val1 / ((battle.mode(bl) & 0x20) ~= 0 and 5 or 1)) end
      g.limit = now - g.tick + hold
      g.val2 = bl.id
    end
  elseif uid == 0x80 then -- Warp Portal, open
    if bl.kind == "player" and g.valstr then
      -- the one stepping in after the last passage closes it (skill.c:2521)
      local left = g.val1
      g.val1 = g.val1 - 1
      if left > 0 then _rt("pc_warp", bl.id, g.valstr, g.to_x, g.to_y) else skill_delunitgroup(g) end
    end
  elseif uid == 0x8e or uid == 0x92 then -- Quagmire, Venom Dust
    local type = SkillStatusChangeTable[g.skill]
    if not sc(bl, type) then
      if uid == 0x92 then
        skill_status_change_start(bl.id, type, g.lv, 0, { tick = skill_get_time2(g.skill, g.lv) })
      else
        skill_status_change_start(bl.id, type, g.val1, c.id)
      end
    end
  end
end

-- skill.c:2581 skill_unit_onout: `bl` left the cell's range (or the cell went).
local function onout(c, bl, now)
  local g = groups[c.group]
  if not g or not c.alive then return end
  local uid = g.unit_id
  if uid == 0x83 then
    g.vallist[bl.id] = nil
  elseif uid == 0x7e or uid == 0x85 or uid == 0x8e then
    local type = uid == 0x85 and SC.PNEUMA or (uid == 0x7e and SC.SAFETYWALL or SC.QUAGMIRE)
    local t = sc(bl, type)
    -- a monster keeps the mud on it until it wears off
    if (type ~= SC.QUAGMIRE or bl.kind ~= "mob") and t and t.val2 == c.id then
      skill_status_change_end(bl.id, type)
    end
  elseif uid == 0x91 then
    if g.val2 == bl.id then skill_status_change_end(bl.id, SC.ANKLE) end
    g.limit = now - g.tick + 1000
  end
  g.ticks[bl.id] = nil
end

-- skill.c:2720 skill_unit_onlimit
local function onlimit(c, now)
  local g = groups[c.group]
  if not g then return end
  if g.unit_id == 0x81 then
    -- the portal opens where it was being cast
    local ng = skill_unitsetting(g.src, g.skill, g.lv, c.x, c.y, 1)
    if ng then ng.valstr, ng.to_x, ng.to_y = g.valstr, g.to_x, g.to_y end
  elseif g.unit_id == 0x8d then
    _rt("set_cell", c.map, c.x, c.y, true)
  end
end

-- skill.c:4189 skill_delunit
del_cell = function(c, from_group)
  if not c.alive then return end
  local g = groups[c.group]
  local now = _rt("now")
  onlimit(c, now)
  if g then
    -- those standing in it lose what it gave (skill.c:2676)
    local uid = g.unit_id
    if uid == 0x85 or uid == 0x7e or uid == 0x8e then
      local range = c.range ~= 0 and c.range or g.range
      for _, id in ipairs(_rt("in_area", c.map, c.x - range, c.y - range, c.x + range, c.y + range)) do
        local bl = get(id)
        if bl then onout(c, bl, now) end
      end
    end
  end
  c.alive = false
  cells[c.id] = nil
  _rt("su_del", c.id)
  if g and not from_group then
    g.alive = g.alive - 1
    if g.alive <= 0 then groups[g.id] = nil end
  end
end

-- skill.c:4442 / 4477: every 100 ms each cell acts on who stands in its
-- range, then goes if its time is up.
function skill_unit_timer(now)
  if empty(cells) then return end
  local list = {}
  for _, c in pairs(cells) do list[#list + 1] = c end
  table.sort(list, function(a, b) return a.id < b.id end)
  for _, c in ipairs(list) do
    local g = groups[c.group]
    if c.alive and g then
      local owner = owner_of(g)
      if not owner then
        skill_delunitgroup(g)
      else
        local range = c.range ~= 0 and c.range or g.range
        if range >= 0 then
          for _, id in ipairs(_rt("in_area", c.map, c.x - range, c.y - range, c.x + range, c.y + range)) do
            if not c.alive then break end
            local bl = get(id)
            if bl and not bl.dead and battle.check_target(owner, bl, g.target) then onplace(c, bl, now) end
          end
        end
        if c.alive and groups[c.group] and (now - g.tick >= g.limit or now - g.tick >= c.limit) then del_cell(c) end
      end
    end
  end
end

-- skill.c:4525 skill_unit_move: a unit took a step or was thrown.
function skill_unit_move(bl_id)
  if empty(cells) then return end
  local bl = get(bl_id)
  if not bl or bl.dead then return end
  local now = _rt("now")
  local near = {}
  for _, c in pairs(cells) do
    if c.map == bl.map and math.abs(c.x - bl.x) <= 5 and math.abs(c.y - bl.y) <= 5 then near[#near + 1] = c end
  end
  table.sort(near, function(a, b) return a.id < b.id end)
  for _, c in ipairs(near) do
    local g = groups[c.group]
    if c.alive and g then
      local owner = owner_of(g)
      local range = c.range ~= 0 and c.range or g.range
      if owner and range >= 0 and battle.check_target(owner, bl, g.target) then
        if in_cell_range(c, g, bl) then onplace(c, bl, now) else onout(c, bl, now) end
      end
    end
  end
  if sc(bl, SC.CLOAKING) then skill_check_cloaking(bl_id) end
end
function rules.skill_unit_move(a) skill_unit_move(a.bl) end

-- The caster died or left: its ground skills go (skill_clear_unitgroup).
function skill_unit_owner_gone(src_id)
  local list = {}
  for _, g in pairs(groups) do if g.src == src_id then list[#list + 1] = g end end
  for _, g in ipairs(list) do skill_delunitgroup(g) end
end

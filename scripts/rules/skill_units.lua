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
local checkskill = battle.pc_checkskill
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
  [SK.AM_DEMONSTRATION] = 0xb1, -- jAthena //0969 skill.c:314
  [SK.CR_GRANDCROSS] = 0x86, -- eAthena 817 skill.c:254, Lord of Vermilion's
  [SK.SA_VOLCANO] = 0x9a, [SK.SA_DELUGE] = 0x9b, [SK.SA_VIOLENTGALE] = 0x9c, [SK.SA_LANDPROTECTOR] = 0x9d, -- 817 skill.c:272
  -- the Bard's and Dancer's songs (817 skill.c:284, jAthena //1020 skill.c:347 the same)
  [SK.BA_DISSONANCE] = 0xa6, [SK.BA_WHISTLE] = 0xa7, [SK.BA_ASSASSINCROSS] = 0xa8, [SK.BA_POEMBRAGI] = 0xa9,
  [SK.BA_APPLEIDUN] = 0xaa, [SK.DC_UGLYDANCE] = 0xab, [SK.DC_HUMMING] = 0xac, [SK.DC_DONTFORGETME] = 0xad,
  [SK.DC_FORTUNEKISS] = 0xae, [SK.DC_SERVICEFORYOU] = 0xaf,
}
-- Units this client cannot draw: sent as type 0, which it ignores (docs/PROTOCOL.md 0x01c9).
-- Volcano's 0x9a asks for effect\ring_red.tga, missing from its GRF (Deluge's ring_blue and Violent
-- Gale's ring_yellow are there), and stops the game with a ResourceError box (rig, 2026-11-16)
-- Of the songs (0x9e-0xaf, each tried alone in a client that had reported nothing yet, 2026-11-16):
-- Assassin Cross of Sunset (0xa8) and Ugly Dance (0xab) ask for the same ring_red.tga; of the
-- ensembles Lullaby (0x9e) asks for effect\zz.bmp, Eternal Chaos (0xa0), Ring of Nibelungen (0xa2)
-- and Siegfried (0xa5) for effect\twirl.bmp. The other twelve are drawn (a note, an apple...)
local UNDRAWN = { [0x9a] = true, [0xa8] = true, [0xab] = true, [0x9e] = true, [0xa0] = true, [0xa2] = true, [0xa5] = true }
local FIELD_UNIT = { [0x9a] = SC.VOLCANO, [0x9b] = SC.DELUGE, [0x9c] = SC.VIOLENTGALE }
local FIELD_SKILL = {}
for _, n in ipairs({ "SA_VOLCANO", "SA_DELUGE", "SA_VIOLENTGALE", "SA_LANDPROTECTOR" }) do FIELD_SKILL[SK[n]] = true end
FIELD_SKILL[-1] = nil

-- The live cell of one of `uids` whose reach covers map x,y, if any.
local function covered_by(map, x, y, uids)
  for _, c in pairs(cells) do
    local g = c.alive and groups[c.group]
    if g and c.map == map and uids[g.unit_id] then
      local r = c.range ~= 0 and c.range or g.range
      if r >= 0 and math.abs(c.x - x) <= r and math.abs(c.y - y) <= r then return c end
    end
  end
  return nil
end
local LAND_PROTECTOR = { [0x9d] = true }

-- A Bard's or Dancer's song: a 9x9 square around the one playing it, which walks with them; the
-- middle cell reaches the square, the rest are for show (jAthena //1020 skill.c:4216). What it
-- gives those in it: SONG_UNIT's status change (status_change.lua song_effect); Dissonance hurts
-- and Ugly Dance drains SP every 3 s.
local SONG_UNIT = { [0xa7] = SC.WHISTLE, [0xa8] = SC.ASSNCROS, [0xa9] = SC.POEMBRAGI, [0xaa] = SC.APPLEIDUN,
                    [0xac] = SC.HUMMING, [0xad] = SC.DONTFORGETME, [0xae] = SC.FORTUNE, [0xaf] = SC.SERVICE4U }
DANCE_SKILL = {}
for _, n in ipairs({ "BA_DISSONANCE", "BA_WHISTLE", "BA_ASSASSINCROSS", "BA_POEMBRAGI", "BA_APPLEIDUN",
                     "DC_UGLYDANCE", "DC_HUMMING", "DC_DONTFORGETME", "DC_FORTUNEKISS", "DC_SERVICEFORYOU" }) do
  DANCE_SKILL[SK[n]] = true
end
DANCE_SKILL[-1] = nil
local function is_song_unit(uid) return uid >= 0xa6 and uid <= 0xaf end

-- The singer's part of a song, taken when it starts (jAthena //1020 skill.c:3930-4015): the
-- Lessons part (val1) and the stat parts packed a * 65536 + b (val2). status_change.lua
-- song_effect says what they add up to.
local function song_values(src, skillid)
  local function lesson(id) return src.kind == "player" and checkskill(src, id) or 0 end
  local half_ml, half_dl = floor((lesson(SK.BA_MUSICALLESSON) + 1) / 2), floor((lesson(SK.DC_DANCINGLESSON) + 1) / 2)
  local function pack(a, b) return (math.max(0, a) << 16) | math.max(0, b or 0) end
  local str, agi, vit, int, dex, luk = battle.str(src), battle.agi(src), battle.vit(src), battle.int(src),
                                       battle.dex(src), battle.luk(src)
  if skillid == SK.BA_WHISTLE then return half_ml, pack(floor(agi / 10), floor(luk / 10))
  elseif skillid == SK.BA_ASSASSINCROSS then return half_ml, pack(floor(agi / 20))
  elseif skillid == SK.BA_POEMBRAGI then return lesson(SK.BA_MUSICALLESSON), pack(floor(dex / 10), floor(int / 5))
  elseif skillid == SK.BA_APPLEIDUN then return lesson(SK.BA_MUSICALLESSON), pack(floor(vit / 10))
  elseif skillid == SK.DC_HUMMING then return half_dl, pack(floor(dex / 10))
  elseif skillid == SK.DC_DONTFORGETME then return half_dl, pack(floor(str / 20), floor(agi / 10))
  elseif skillid == SK.DC_FORTUNEKISS then return half_dl, pack(floor(luk / 10))
  elseif skillid == SK.DC_SERVICEFORYOU then return half_dl, pack(floor(int / 10))
  elseif skillid == SK.BA_DISSONANCE then return lesson(SK.BA_MUSICALLESSON), 0
  end
  return 0, 0
end
UNIT_ID[-1] = nil
local function unit_id(skill, flag)
  if skill == SK.AL_WARP then return flag == 0 and 0x81 or 0x80 end        -- opening, open
  if skill == SK.WZ_FIREPILLAR then return flag == 0 and 0x87 or 0x88 end  -- waiting, burning
  if skill == SK.HT_TALKIEBOX then return flag == 0 and 0x99 or 0x8c end
  return UNIT_ID[skill] or 0
end

local map_calc_dir = battle.calc_dir

local function del_cell(cell) end -- forward

-- The song is over: its player stops playing (jAthena //1020 skill.c:8041)
local function dance_over(g)
  local t = g.dance and sc(get(g.src), SC.DANCING)
  if t and t.val2 == g.id then skill_status_change_end(g.src, SC.DANCING) end
end

-- skill.c:4297 skill_delunitgroup
function skill_delunitgroup(g)
  if not groups[g.id] then return end
  for _, id in ipairs(g.cells) do
    local c = cells[id]
    if c and c.alive then del_cell(c, true) end
  end
  groups[g.id] = nil
  dance_over(g)
end

-- Stop playing (jAthena //1020 skill.c:7805 skill_stop_dancing): Adaptation, no SP left, a
-- weapon that is no instrument or whip, death, another map
function skill_stop_dancing(id)
  local t = sc(get(id), SC.DANCING)
  if not t then return end
  skill_status_change_end(id, SC.DANCING)
  local g = groups[t.val2]
  if g then skill_delunitgroup(g) end
end

-- How long the song `id` plays has run (ms), or nil if it plays none
function skill_dance_elapsed(id)
  local t = sc(get(id), SC.DANCING)
  local g = t and groups[t.val2]
  if not g then return nil end
  return _rt("now") - g.tick
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
    -- in a Violent Gale it lasts half as long again (the iRO guide's Sage page, 2004-03-27, and
    -- euRO's text of May 2004; no emulator of the period has it)
    if covered_by(src.map, x, y, { [0x9c] = true }) then limit = floor(limit * 3 / 2) end
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
  elseif FIELD_UNIT[unit_id(skillid, 0)] then
    -- Volcano, Deluge, Violent Gale: 5x5, 7x7, 9x9 cells for 180 s, the middle one reaching the
    -- square and the rest for show; on everyone (jAthena //0969 skill.c:3470, 3720; the guide's
    -- "Range 5, 7, 9 cells")
    limit, target = 180000, BCT.ALL
    local r = skilllv <= 2 and 2 or (skilllv <= 4 and 3 or 4)
    count, val2 = (2 * r + 1) * (2 * r + 1), r
  elseif skillid == SK.SA_LANDPROTECTOR then
    -- 7x7, 9x9, 11x11 cells (jAthena skill.c:3477, the guide); 2 min to 5 min, the iRO guide's
    -- table (2004-03-27; mod0631's and 817's cast_db 120 s to 240 s, jAthena's 180 s)
    limit, target = ({ 120000, 165000, 210000, 255000, 300000 })[math.max(1, math.min(5, skilllv))], BCT.ALL
    local r = skilllv <= 2 and 3 or (skilllv <= 4 and 4 or 5)
    count, val2 = (2 * r + 1) * (2 * r + 1), r
  elseif skillid == SK.CR_GRANDCROSS then
    -- a cross of 29 cells (817 skill.c:2962, jAthena the same), each striking who stands on it
    -- every 0.3 s. The guide's 0.9 s ("damaged 3 times"), where 817 and jAthena keep it 1 s
    count, limit, interval = 29, 900, 300
  elseif skillid == SK.AM_DEMONSTRATION then
    -- Bomb: jAthena //0969 skill.c:3589 and its cast_db, 40 s to 60 s; a blow a second
    -- ("every 1 second", the iRO guide's Alchemist page, 2004-03-27)
    count, limit, interval, range = 9, 35000 + 5000 * skilllv, 1000, 1
  elseif DANCE_SKILL[skillid] then
    -- jAthena //1020 skill.c:3930-4015: 81 cells for the song's time (its cast_db, which the iRO
    -- guide's pages repeat: Dissonance and Ugly Dance 30 s, Whistle and Humming 60 s, Assassin
    -- Cross and Fortune's Kiss 120 s, the rest 180 s); on enemies (Dissonance, Ugly Dance, Don't
    -- Forget Me) or on everyone else. Service For You and Apple of Idun are for the party there:
    -- here, with no party, everyone who is no enemy
    count, limit = 81, skill_get_time(skillid, skilllv)
    local enemy = skillid == SK.BA_DISSONANCE or skillid == SK.DC_UGLYDANCE or skillid == SK.DC_DONTFORGETME
    target = enemy and BCT.ENEMY or BCT.NOENEMY
    -- Dissonance and Ugly Dance strike at once and then every 3 s, eleven times in 30 s; Apple of
    -- Idun heals every 6 s, 31 times in 180 s (the guide's pages)
    if skillid == SK.BA_DISSONANCE or skillid == SK.DC_UGLYDANCE then interval = 3000
    elseif skillid == SK.BA_APPLEIDUN then interval = 6000 end
    val1, val2 = song_values(src, skillid)
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
  if skillid == SK.HT_TALKIEBOX and src.kind == "player" then
    -- the message typed when it was set (0x0190; jAthena //0988, 2004-05-25)
    g.valstr = _rt("pc_talkie", src_id)
  end

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
      if c == 5 or c == 1 or #_rt("in_area", src.map, ux, uy, ux, uy) > 0
        or covered_by(src.map, ux, uy, { [0x9a] = true, [0x9b] = true, [0x9c] = true }) then
        -- a wall, or someone standing there (ours: nobody gets walled in), or a Sage's field
        -- ("Ice Wall can not be casted in this area", the iRO guide's Sage page)
        alive = false
      else
        _rt("set_cell", src.map, ux, uy, false)
      end
    elseif skillid == SK.WZ_QUAGMIRE then
      ux, uy = ux + (i % 5 - 2), uy + (i // 5 - 2)
    elseif FIELD_SKILL[skillid] then
      local r, w = g.val2, 2 * g.val2 + 1
      ux, uy = ux + (i % w - r), uy + (i // w - r)
      if skillid ~= SK.SA_LANDPROTECTOR then crange = (i == (count - 1) // 2) and r or -1 end
    elseif skillid == SK.CR_GRANDCROSS then
      local dx = { 0, 0, -1, 0, 1, -2, -1, 0, 1, 2, -4, -3, -2, -1, 0, 1, 2, 3, 4, -2, -1, 0, 1, 2, -1, 0, 1, 0, 0 }
      local dy = { -4, -3, -2, -2, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 3, 4 }
      ux, uy = ux + dx[i + 1], uy + dy[i + 1]
    elseif skillid == SK.AM_DEMONSTRATION then
      -- nine cells, the middle one burning the 3x3 and the rest for show (jAthena skill.c:3759,
      -- which puts the square a cell up and left of the spot: ours centres it)
      ux, uy = ux + (i % 3 - 1), uy + (i // 3 - 1)
      if i ~= 4 then crange = -1 end
    elseif DANCE_SKILL[skillid] then
      ux, uy = ux + (i % 9 - 4), uy + (i // 9 - 4)
      crange = i == 40 and 4 or -1
    elseif skillid == SK.AS_VENOMDUST then
      local dx = { -1, 0, 0, 0, 1 }
      local dy = { 0, -1, 0, 1, 0 }
      ux, uy = ux + dx[i + 1], uy + dy[i + 1]
    end
    -- Land Protector (jAthena //0969 skill.c:3798, skill_landprotector): laid, it wipes the
    -- ground skills on its cells; a one-cell skill is not laid on it
    if skillid == SK.SA_LANDPROTECTOR then
      for _, oc in pairs(cells) do
        if oc.alive and oc.map == src.map and oc.x == ux and oc.y == uy then del_cell(oc) end
      end
    elseif alive and (crange ~= 0 and crange or range) <= 0 then
      for _, oc in pairs(cells) do
        local og = oc.alive and groups[oc.group]
        if og and og.unit_id == 0x9d and oc.map == src.map and oc.x == ux and oc.y == uy then alive = false; break end
      end
    end
    if alive then
      local id = _rt("su_new", src.map, ux, uy, UNDRAWN[g.unit_id] and 0 or g.unit_id, src_id, g.id)
      cells[id] = { id = id, group = g.id, map = src.map, x = ux, y = uy, val1 = skilllv, val2 = cval2,
                    limit = limit, range = crange, alive = true }
      if skillid == SK.WZ_ICEWALL then
        -- durability (eAthena 817 skill.c:3184 val1; the iRO guide's 400 to 2200): it loses 5
        -- every 100 ms (skill.c:6155), which is the group's time above, and what blows take
        cells[id].durability, cells[id].born = 200 + 200 * skilllv, now
      end
      g.cells[#g.cells + 1] = id
      g.alive = g.alive + 1
    end
  end
  if g.alive == 0 then groups[g.id] = nil; return g end
  if DANCE_SKILL[skillid] then
    -- the one playing it walks with it: where it stands from them (a throw moves the one and not
    -- the other, jAthena //1015 of 2004-06-09: "the effect does not move when one is knocked back")
    g.dance, g.ox, g.oy = true, x - src.x, y - src.y
    g.mapname = src.map
  end
  return g
end

-- Ground skills may not be stacked: jAthena //0751 (2004-01-28) forbade it, //0764
-- (2004-02-01) made it `player_skill_reiteration: no` and `monster_skill_reiteration: no` ("with
-- yes, Safety Wall, Pneuma and traps can be laid on top of each other";
-- skill_check_unit_range, skill_castend_pos, mobskill_castend_pos); eAthena 817
-- the same (skill.c:862, 3761; mob.c:2370). mod0631 checks nothing. jAthena
-- //0825 (2004-02-20) split off `nofootset`, refusing them on anyone's feet,
-- and left it off; 817 counts people too, so a Safety Wall under oneself
-- fails there. We follow jAthena: other cells only.
-- How far around the cell the new one reaches, by caster.
local REITERATION_RANGE = {
  [SK.MG_SAFETYWALL] = 0, [SK.WZ_FIREPILLAR] = 0, [SK.HT_TALKIEBOX] = 0, [SK.AL_PNEUMA] = 1,
}
for s = SK.HT_SKIDTRAP, SK.HT_CLAYMORETRAP do REITERATION_RANGE[s] = 0 end
local function is_trap_unit(uid) return uid >= 0x8f and uid <= 0x99 end
-- Which cells stand in the way of each skill (skill_check_unit_range_sub).
local function blocks(skillid, uid)
  if skillid == SK.MG_SAFETYWALL or skillid == SK.AL_PNEUMA then return uid == 0x7e or uid == 0x85 end
  if skillid == SK.AL_WARP or (skillid >= SK.HT_SKIDTRAP and skillid <= SK.HT_CLAYMORETRAP)
      or skillid == SK.HT_TALKIEBOX then
    return is_trap_unit(uid) -- Venom Dust (0x92) among them; another portal does not
  end
  if skillid == SK.WZ_FIREPILLAR then return uid == 0x87 end -- a pillar not yet set off
  return false
end

-- Whether `skillid` may not be laid at x,y: true when another cell is in the way.
function skill_reiteration_blocked(src, x, y, skillid)
  local range = REITERATION_RANGE[skillid]
  -- a monster's Warp Portal reaches a cell around, a player's only its own
  if skillid == SK.AL_WARP then range = src.kind == "player" and 0 or 1 end
  if not range then return false end
  for _, c in pairs(cells) do
    local g = groups[c.group]
    if c.alive and g and c.map == src.map and blocks(skillid, g.unit_id) then
      local r = c.range ~= 0 and c.range or g.range
      -- the two squares overlap (817 tests each one's corners inside the other)
      if math.abs(c.x - x) <= r + range and math.abs(c.y - y) <= r + range then return true end
    end
  end
  return false
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
  -- nothing on the ground touches someone standing on a Land Protector (jAthena skill.c:3845)
  if covered_by(bl.map, bl.x, bl.y, LAND_PROTECTOR) then return end
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
  elseif uid == 0x86 then -- Lord of Vermilion, Storm Gust, Meteor Storm, Grand Cross
    if g.skill == SK.CR_GRANDCROSS then
      -- three strikes each, whatever the clock does near the 0.9 s
      local n = (g.vallist[bl.id] or 0) + 1
      if n > 3 then return end
      g.vallist[bl.id] = n
    end
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
  elseif FIELD_UNIT[uid] then -- Volcano, Deluge, Violent Gale (jAthena skill.c:4027)
    local type = FIELD_UNIT[uid]
    local t = sc(bl, type)
    if not t or t.val2 ~= c.id then skill_status_change_start(bl.id, type, g.lv, c.id, { tick = g.limit }) end
  elseif is_song_unit(uid) then
    -- jAthena //1020 skill.c:4510: never on the one playing it; ours, for one player, on them
    -- too (battle_config.song_on_self)
    if bl.id == g.src and not battle_config.song_on_self then return end
    if uid == 0xa6 then -- Dissonance
      skill_attack(BF.MISC, g.src, cell_src, bl.id, g.skill, g.lv, 0)
    elseif uid == 0xab then -- Ugly Dance: SP, so only on a player (monsters have none)
      -- 10 to 50 SP a time: the iRO guide's Dancer page. jAthena gives the status and nothing else
      if bl.kind == "player" and bl.char.sp > 0 then _rt("bl_set", bl.id, "sp", math.max(0, bl.char.sp - 10 * g.lv)) end
    else
      local type = SONG_UNIT[uid]
      local t = sc(bl, type)
      if not t or t.val2 ~= c.id then
        skill_status_change_start(bl.id, type, g.lv, c.id, { tick = g.limit, val3 = g.val1, val4 = g.val2 })
      end
      if uid == 0xaa and bl.kind == "player" then
        -- Apple of Idun heals 35 to 80 HP every 6 s (the guide's table). jAthena's healing
        -- (pc.c:6262) sits after a return and never runs
        local got = battle.heal(bl.id, 30 + 5 * g.lv, 0)
        if (got or 0) > 0 then _rt("clif_heal", bl.id, 5, got) end
      end
    end
  elseif uid == 0xb1 then -- Bomb (jAthena //0969 skill.c:4095)
    skill_attack(BF.WEAPON, g.src, cell_src, bl.id, g.skill, g.lv, 0)
    skill_break_mob(bl.id, SC.BROKENWEAPON, g.lv) -- skill_effects.lua
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
    if uid == 0x95 and g.val2 == 0 then
      -- Sandman: "makes an enemy and near enemies fall asleep" (the iRO guide's
      -- Hunter page, 2004-03-27). Both emulators put only the one who stepped
      -- on it to sleep. How near is ours: the cells around that one.
      local ss = get(g.src)
      if ss then
        for _, t in ipairs(skill_area(ss, bl.map, bl.x, bl.y, 1, BCT.ENEMY)) do
          if t.id ~= bl.id then skill_additional_effect(g.src, t.id, g.skill, g.lv, BF.MISC) end
        end
      end
    end
    if g.val2 == 0 then
      local hold = 500
      if uid == 0x91 then hold = floor(g.val1 / ((battle.mode(bl) & 0x20) ~= 0 and 5 or 1)) end
      g.limit = now - g.tick + hold
      g.val2 = bl.id
    end
  elseif uid == 0x99 then -- Talkie Box
    -- Nothing in mod0631 or 817 (817's attempt is marked "not working").
    -- jAthena //0988 (2004-05-25, "the packets matched to the official
    -- server from Kalen's information"): the first to step on it makes it
    -- speak to those around, and it goes five seconds later. jAthena also
    -- turns it into a used trap (0x8c) with 0x00c3, which this client only
    -- applies to characters (0x51e760), so that is left out. Its own Hunter
    -- does not set it off: "The message is not played for the caster" (the
    -- iRO guide's Hunter page, 2004-03-27); jAthena //0991 (2004-05-26) adds
    -- the same test (skill.c:4378).
    if bl.id == g.src then return end
    if g.val2 == 0 then
      _rt("clif_talkiebox", c.id, g.valstr or "")
      g.limit = now - g.tick + 5000
      g.val2 = -1
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
  elseif FIELD_UNIT[uid] then
    local t = sc(bl, FIELD_UNIT[uid])
    if t and t.val2 == c.id then skill_status_change_end(bl.id, FIELD_UNIT[uid]) end
  elseif SONG_UNIT[uid] then
    local t = sc(bl, SONG_UNIT[uid])
    if t and t.val2 == c.id then skill_status_change_end(bl.id, SONG_UNIT[uid]) end
  elseif uid == 0x7e or uid == 0x85 or uid == 0x8e then
    local type = uid == 0x85 and SC.PNEUMA or (uid == 0x7e and SC.SAFETYWALL or SC.QUAGMIRE)
    local t = sc(bl, type)
    -- a monster keeps the mud on it until it wears off
    if (type ~= SC.QUAGMIRE or bl.kind ~= "mob") and t and t.val2 == c.id then
      skill_status_change_end(bl.id, type)
    end
  elseif uid == 0x91 then
    -- Only the snared one walking off ends the snare, a second later, as in
    -- jAthena //1046 (2004-09-12). mod0631 and 817 shortened the trap on any
    -- enemy that moved within 5 cells of it, caught or not, so a snare in a
    -- field went within a second, sprung or not (DEVIATIONS.md).
    if g.val2 ~= 0 and g.val2 == bl.id then
      skill_status_change_end(bl.id, SC.ANKLE)
      g.limit = now - g.tick + 1000
    end
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

-- The song walks with its player: move its cells where they now stand, and let those around
-- the middle cell walk in or out of it (jAthena //1020 skill.c:8444 skill_unit_move_unit_group,
-- pc.c:3458). Fly Wing on the same map takes it along (jAthena //1012); another map ends it.
local function dance_follow(bl)
  local t = sc(bl, SC.DANCING)
  local g = t and groups[t.val2]
  if not g then return end
  if bl.map ~= g.mapname then skill_stop_dancing(bl.id); return end
  local mid
  for _, id in ipairs(g.cells) do
    local c = cells[id]
    if c and c.alive and c.range > 0 then mid = c end
  end
  if not mid then return end
  local dx, dy = bl.x + g.ox - mid.x, bl.y + g.oy - mid.y
  if dx == 0 and dy == 0 then return end
  for _, id in ipairs(g.cells) do
    local c = cells[id]
    if c and c.alive then
      c.x, c.y = c.x + dx, c.y + dy
      _rt("su_move", c.id, c.x, c.y)
    end
  end
  local owner = owner_of(g)
  if not owner then return end
  local now = _rt("now")
  local reach = mid.range + math.max(math.abs(dx), math.abs(dy))
  for _, id in ipairs(_rt("in_area", mid.map, mid.x - reach, mid.y - reach, mid.x + reach, mid.y + reach)) do
    local o = get(id)
    if o and not o.dead and battle.check_target(owner, o, g.target) then
      if in_cell_range(mid, g, o) then onplace(mid, o, now) else onout(mid, o, now) end
    end
  end
end

-- A dancer thrown back leaves the song where it was (jAthena //1015, 2004-06-09)
function skill_dance_thrown(id, dx, dy)
  local t = sc(get(id), SC.DANCING)
  local g = t and groups[t.val2]
  if g then g.ox, g.oy = g.ox - dx, g.oy - dy end
end

-- ------------------------------------------------------------ Ice Wall as a target

-- eAthena 817 lets area skills hit an Ice Wall's cells (skill.c:847 skill_area_sub takes
-- BL_SKILL; battle.c:3464 calls the wall everyone's enemy) and takes the damage off the
-- cell's durability (skill.c:3718). This client cannot click one (tried 2026-11-15: a click
-- on the wall is a walk to the cell behind it), so area skills are the only way, as there.
local function wall_left(c, now) return c.durability - floor((now - c.born) / 100) * 5 end

-- A wall cell as a target for battle's formulas: no DEF, no flee, Water 1 (817 battle.c:716
-- gives the skill's element, the wall's being Water), formless, medium.
local WALL_DB = { level = 1, str = 0, agi = 0, vit = 0, int = 0, dex = 0, luk = 0, def = 0, mdef = 0,
                  race = 0, size = 1, element = 21, mode = 0, mvp_exp = 0, range = 0, atk1 = 0, atk2 = 0,
                  attack_motion = 0, damage_motion = 0, attack_delay = 0, drops = {} }
function icewall_bl(id)
  local c = cells[id]
  if not c or not c.alive or not c.durability then return nil end
  local left = wall_left(c, _rt("now"))
  return { id = id, kind = "mob", wall = true, map = c.map, x = c.x, y = c.y, dir = 0, dead = left <= 0,
           hp = math.max(0, left), max_hp = c.durability, def_ele = 21, sc = {}, option = 0, opt1 = 0, opt2 = 0,
           db = WALL_DB, class = 0, side = 0 }
end

-- Ice Wall cells in a rectangle, as targets.
function icewall_targets(map, x0, y0, x1, y1)
  local out = {}
  for id, c in pairs(cells) do
    if c.durability and c.alive and c.map == map and c.x >= x0 and c.x <= x1 and c.y >= y0 and c.y <= y1 then
      local bl = icewall_bl(id)
      if bl and not bl.dead then out[#out + 1] = bl end
    end
  end
  table.sort(out, function(a, b) return a.id < b.id end)
  return out
end

-- A blow on a wall cell; at nothing left it goes within 0.7 s (817 skill.c:6157).
function icewall_damage(id, damage)
  local c = cells[id]
  if not c or not c.alive or not c.durability or damage <= 0 then return end
  c.durability = c.durability - damage
  local now = _rt("now")
  if wall_left(c, now) <= 0 then
    c.limit = math.min(c.limit, now - (groups[c.group] and groups[c.group].tick or now) + 700)
  end
end

-- skill.c:4189 skill_delunit
del_cell = function(c, from_group)
  if not c.alive or c.closing then return end
  local g = groups[c.group]
  local now = _rt("now")
  -- (marked before its last act: an opening portal sets a new group, and
  -- the cap on groups must not come back to this cell)
  c.closing = true
  onlimit(c, now)
  if g then
    -- those standing in it lose what it gave (skill.c:2676)
    local uid = g.unit_id
    if uid == 0x85 or uid == 0x7e or uid == 0x8e or FIELD_UNIT[uid] or SONG_UNIT[uid] then
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
    if g.alive <= 0 then groups[g.id] = nil; dance_over(g) end
  end
end

-- Remove Trap and Spring Trap aim at a trap on the ground: the cell with
-- this unit id if it is one of the Hunter's traps, and taking it away.
local TRAPS = { [0x8f] = true, [0x90] = true, [0x91] = true, [0x93] = true, [0x94] = true, [0x95] = true,
                [0x96] = true, [0x97] = true, [0x98] = true, [0x99] = true }
function skill_trap_cell(id)
  local c = cells[id]
  local g = c and c.alive and groups[c.group]
  if not g or not TRAPS[g.unit_id] then return nil end
  return { id = c.id, map = c.map, x = c.x, y = c.y, src = g.src, skill = g.skill, sprung = g.val2 ~= 0 }
end
-- The traps standing in a rectangle, as cell ids (Detect shows them).
function skill_traps_in(map, x0, y0, x1, y1)
  local out = {}
  for id, c in pairs(cells) do
    local g = c.alive and c.map == map and groups[c.group]
    if g and TRAPS[g.unit_id] and c.x >= x0 and c.x <= x1 and c.y >= y0 and c.y <= y1 then
      out[#out + 1] = id
    end
  end
  return out
end
function skill_trap_remove(id)
  local c = cells[id]
  if c and c.alive then del_cell(c) end
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
      if g.dance and owner and (owner.dead or not sc(owner, SC.DANCING)) then owner = nil end
      if owner and g.dance then
        dance_follow(owner) -- a Fly Wing, or another map
        if not groups[g.id] or not c.alive then owner = nil end
      end
      if not owner then
        if groups[g.id] then skill_delunitgroup(g) end
      else
        local range = c.range ~= 0 and c.range or g.range
        if range >= 0 then
          for _, id in ipairs(_rt("in_area", c.map, c.x - range, c.y - range, c.x + range, c.y + range)) do
            if not c.alive then break end
            local bl = get(id)
            if bl and not bl.dead and battle.check_target(owner, bl, g.target) then onplace(c, bl, now) end
          end
        end
        if c.alive and groups[c.group] and (now - g.tick >= g.limit or now - g.tick >= c.limit) then
          -- a player's trap that ran out unsprung leaves its Trap on the cell (817 skill.c:6143,
          -- not Venom Dust; mod0631 has nothing): "any unused trap will be able to be picked up
          -- after it's lasting time has depleted" (Giga Bahamut's iRO beta 2 Hunter FAQ, GameFAQs
          -- 20075, section of 2003-02-22)
          if g.unit_id >= 0x8f and g.unit_id <= 0x98 and g.unit_id ~= 0x92 and g.val2 == 0 and owner.kind == "player" then
            _rt("drop_item", c.map, c.x, c.y, 1065, 1)
          end
          del_cell(c)
        end
      end
    end
  end
end

-- skill.c:4525 skill_unit_move: a unit took a step or was thrown.
function skill_unit_move(bl_id)
  if empty(cells) then return end
  local bl = get(bl_id)
  if not bl or bl.dead then return end
  dance_follow(bl)
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

-- A Sage has one field at a time: a new one takes his last away (jAthena skill.c:5455
-- skill_clear_element_field)
function skill_clear_element_field(src_id)
  local list = {}
  for _, g in pairs(groups) do if g.src == src_id and FIELD_SKILL[g.skill] then list[#list + 1] = g end end
  for _, g in ipairs(list) do skill_delunitgroup(g) end
end

-- The caster died or left: its ground skills go (skill_clear_unitgroup).
function skill_unit_owner_gone(src_id)
  local list = {}
  for _, g in pairs(groups) do if g.src == src_id then list[#list + 1] = g end end
  for _, g in ipairs(list) do skill_delunitgroup(g) end
end

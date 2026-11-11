-- What each skill does once its cast is over.
-- Server-side: NOT in the client.
--
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/skill.c (archived
-- 2003-12-18): skill_castend_damage_id (780), skill_castend_nodamage_id
-- (1113), skill_castend_pos2 (1795), skill_castend_map (1914). Line refs are
-- to that file. Skills mod0631 leaves out are taken from eAthena 817
-- map/skill.c (2004-02-19) and marked "817:"; the few neither has are marked
-- "ours" and listed in docs/DEVIATIONS.md.
--
-- Covered: Novice, the six first jobs, Knight, Priest, Wizard, Blacksmith,
-- Hunter, Assassin, and the monsters' skills. The 2-2 jobs (Juno, June 2004)
-- are not in the default world; their skills are not here yet.

local floor, random = math.floor, math.random
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end
local get = battle.get
local sc = battle.sc
local checkskill = battle.pc_checkskill

local function nodamage(src, bl, skill, val, ok) _rt("clif_skill_nodamage", src, bl, skill, val or 0, ok == false and 0 or 1) end

-- Hit everything hostile in a square around (x, y) except `skip`.
-- fn(target_bl) does the hit.
local function area_enemies(src, x, y, range, skip, fn)
  for _, bl in ipairs(skill_area(src, src.map, x, y, range, BCT.ENEMY)) do
    if bl.id ~= skip then fn(bl) end
  end
end

-- ------------------------------------------------------------ attack skills

local WEAPON_SINGLE = {}
for _, n in ipairs({
  "SM_BASH", "MC_MAMMONITE", "AC_DOUBLE", "AS_SONICBLOW", "KN_PIERCE", "KN_SPEARBOOMERANG", "TF_POISON",
  "TF_SPRINKLESAND", "AC_CHARGEARROW", "KN_SPEARSTAB", "RG_BACKSTAP", "RG_INTIMIDATE",
  "NPC_PIERCINGATT", "NPC_MENTALBREAKER", "NPC_RANGEATTACK", "NPC_CRITICALSLASH", "NPC_COMBOATTACK",
  "NPC_GUIDEDATTACK", "NPC_POISON", "NPC_BLINDATTACK", "NPC_SILENCEATTACK", "NPC_STUNATTACK",
  "NPC_PETRIFYATTACK", "NPC_CURSEATTACK", "NPC_SLEEPATTACK", "NPC_RANDOMATTACK",
  "NPC_WATERATTACK", "NPC_GROUNDATTACK", "NPC_FIREATTACK", "NPC_WINDATTACK", "NPC_POISONATTACK",
  "NPC_HOLYATTACK", "NPC_DARKNESSATTACK", "NPC_TELEKINESISATTACK", "NPC_LICK",
}) do WEAPON_SINGLE[SK[n]] = true end

local MAGIC_SINGLE = {}
for _, n in ipairs({
  "MG_SOULSTRIKE", "MG_COLDBOLT", "MG_FIREBOLT", "MG_LIGHTNINGBOLT", "WZ_EARTHSPIKE", "AL_HEAL", "AL_HOLYLIGHT",
  "ALL_RESURRECTION", "PR_TURNUNDEAD", "MG_FROSTDIVER", "WZ_JUPITEL", "NPC_MAGICALATTACK",
}) do MAGIC_SINGLE[SK[n]] = true end

-- skill.c:780. opt: the server's cast-end arguments, or for a secondary hit
-- {sub = true, wflag =, main = id of the main target, cx =, cy =}.
function skill_castend_damage_id(src_id, bl_id, skillid, skilllv, opt)
  opt = opt or {}
  local src, bl = get(src_id), get(bl_id)
  if not src or not bl or src.dead or bl.dead then return end
  local sd = src.kind == "player" and src or nil

  if WEAPON_SINGLE[skillid] then
    skill_attack(BF.WEAPON, src_id, src_id, bl_id, skillid, skilllv, 0)

  elseif skillid == SK.KN_BRANDISHSPEAR then
    -- 817:1237 the target, pushed back if it lives (817: one cell; two, as
    -- the iRO guide's Knight page has it); the cells around it by the rows
    -- of 817:1913 (see skill_castend_nodamage_id)
    skill_attack(BF.WEAPON, src_id, src_id, bl_id, skillid, skilllv, opt.wflag or 1)
    local after = get(bl_id)
    if after and not after.dead then
      skill_blown(src, bl_id, 2)
      _rt("clif_fixpos", bl_id)
    end

  elseif skillid == SK.AC_SHOWER or skillid == SK.SM_MAGNUM or skillid == SK.AS_GRIMTOOTH
    or skillid == SK.KN_BOWLINGBASH or skillid == SK.MC_CARTREVOLUTION or skillid == SK.NPC_SPLASHATTACK then
    -- skill.c:911 weapon skills that splash: the target, then the enemies around
    if opt.sub then
      skill_attack(BF.WEAPON, src_id, src_id, bl_id, skillid, skilllv, opt.wflag or 0, { type = 5 })
      return
    end
    local ar = 1
    local x, y = bl.x, bl.y
    if skillid == SK.SM_MAGNUM then
      x, y = src.x, src.y -- centred on the swordman
    elseif skillid == SK.AC_SHOWER then
      ar = 2
    elseif skillid == SK.NPC_SPLASHATTACK then
      ar = 3
    elseif skillid == SK.KN_BOWLINGBASH then
      -- pushed back a cell at a time, up to four, until it bumps into another enemy
      for _ = 1, 4 do
        skill_blown(src, bl_id, 1)
        local t = get(bl_id)
        if not t then return end
        if #skill_area(src, t.map, t.x, t.y, 1, BCT.ENEMY) > 1 then break end
      end
      _rt("clif_fixpos", bl_id)
      local t = get(bl_id)
      x, y = t.x, t.y
    end
    skill_attack(BF.WEAPON, src_id, src_id, bl_id, skillid, skilllv, 0)
    area_enemies(src, x, y, ar, bl_id, function(t)
      local dist = 0
      if skillid == SK.SM_MAGNUM then dist = math.max(math.abs(t.x - x), math.abs(t.y - y)) end
      skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = true, wflag = dist })
    end)

  elseif MAGIC_SINGLE[skillid] then
    skill_attack(BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, 0)

  elseif skillid == SK.WZ_WATERBALL then
    skill_attack(BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, 0)
    if skilllv > 1 then skill_status_change_start(src_id, SC.WATERBALL, skilllv, bl_id) end

  elseif skillid == SK.PR_BENEDICTIO then
    local race = battle.race(bl)
    if race == 1 or race == 6 then skill_attack(BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, 0) end

  elseif skillid == SK.MG_NAPALMBEAT or skillid == SK.MG_FIREBALL or skillid == SK.MG_THUNDERSTORM
    or skillid == SK.WZ_HEAVENDRIVE then
    -- skill.c:994 magic that splashes
    if opt.sub then
      skill_attack(BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, opt.wflag or 0, opt.ground and {} or { type = 5 })
      return
    end
    local ar = skillid == SK.MG_NAPALMBEAT and 1 or 2
    local share = 0
    if skillid == SK.MG_NAPALMBEAT then
      share = #skill_area(src, bl.map, bl.x, bl.y, 1, BCT.ENEMY) -- the damage is shared among them
    end
    skill_attack(BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, share)
    area_enemies(src, bl.x, bl.y, ar, bl_id, function(t)
      local w = share
      if skillid == SK.MG_FIREBALL then w = math.max(math.abs(t.x - bl.x), math.abs(t.y - bl.y)) end
      skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = true, wflag = w })
    end)

  elseif skillid == SK.WZ_FROSTNOVA then
    -- everyone around the caster but the caster's own cell
    if bl_id ~= opt.main and (bl.x ~= opt.cx or bl.y ~= opt.cy) then
      skill_attack(BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, 0)
    end

  elseif skillid == SK.HT_BLITZBEAT then
    -- skill.c:1043: by itself (auto) the damage is split among the enemies around
    if opt.sub then
      skill_attack(BF.MISC, src_id, src_id, bl_id, skillid, skilllv, opt.wflag or 0, { lv = opt.auto and -1 or nil })
      return
    end
    local share = 0
    if opt.auto then share = #skill_area(src, bl.map, bl.x, bl.y, 1, BCT.ENEMY) end
    skill_attack(BF.MISC, src_id, src_id, bl_id, skillid, skilllv, share, { lv = opt.auto and -1 or nil })
    area_enemies(src, bl.x, bl.y, 1, bl_id, function(t)
      skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = true, wflag = share, auto = opt.auto })
    end)

  elseif skillid == SK.TF_THROWSTONE or skillid == SK.NPC_SMOKING then
    skill_attack(BF.MISC, src_id, src_id, bl_id, skillid, skilllv, 0)

  elseif skillid == SK.NPC_SELFDESTRUCTION then
    -- skill.c:1070: its HP as damage to everyone within 5 cells, then it dies
    skill_attack(BF.MISC, src_id, src_id, bl_id, skillid, skilllv, 0)
    for _, t in ipairs(skill_area(src, src.map, bl.x, bl.y, 5, BCT.ALL)) do
      if t.id ~= bl_id and t.id ~= src_id then
        -- the snapshot keeps the HP it had before the first hit
        local dmg = battle.calc_misc_attack(src, t, skillid, skilllv, 0)
        _rt("clif_skill_damage", src_id, t.id, dmg.amotion, dmg.dmotion, dmg.damage, dmg.div, skillid, skilllv, -1)
        battle.damage(src_id, t.id, dmg.damage)
      end
    end
    _rt("mob_kill", src_id)

  elseif skillid == SK.NPC_BLOODDRAIN or skillid == SK.NPC_ENERGYDRAIN then
    local got = skill_attack(skillid == SK.NPC_BLOODDRAIN and BF.WEAPON or BF.MAGIC, src_id, src_id, bl_id, skillid, skilllv, 0)
    battle.heal(src_id, got, 0)

  elseif skillid == SK.RG_RAID then
    skill_attack(BF.WEAPON, src_id, src_id, bl_id, skillid, skilllv, 0)
  end
end

-- Remove Trap and Spring Trap, drafts: in neither mod0631 nor eAthena 817.
-- The client's skill text: "Remove a Trap Installed. After removing, Can
-- Regain 1 Trap" and "Manage a Falcon to Destroy a installed Trap". Whose
-- traps, and which kinds, it does not say: any Hunter trap here. THREADS.md #22.
-- The iRO guide's Hunter page (2004-03-27): a trap that has gone off cannot
-- be taken up again; one that is taken up gives its Trap back.
function skill_castend_trap(src, cell, skillid, skilllv)
  if skillid == SK.HT_REMOVETRAP and cell.sprung then
    nodamage(src.id, src.id, skillid, skilllv, false)
    return
  end
  nodamage(src.id, src.id, skillid, skilllv)
  skill_trap_remove(cell.id)
  if skillid == SK.HT_REMOVETRAP and src.kind == "player" then _rt("pc_item_add", src.id, 1065, 1) end
end

-- ------------------------------------------------------------ support skills

local SC_SIMPLE = {}
for _, n in ipairs({
  "AL_INCAGI", "AL_DECAGI", "AL_BLESSING", "KN_TWOHANDQUICKEN", "CR_SPEARQUICKEN", "PR_IMPOSITIO", "PR_ASPERSIO",
  "PR_KYRIE", "PR_LEXDIVINA", "PR_LEXAETERNA", "AS_ENCHANTPOISON", "AS_POISONREACT", "AC_CONCENTRATION",
  "MC_LOUD", "MG_ENERGYCOAT", "SM_PROVOKE", "SM_ENDURE", "PR_SUFFRAGIUM", "MG_SIGHT", "AL_RUWACH",
  "PR_BENEDICTIO", "PR_SLOWPOISON", "KN_AUTOCOUNTER",
}) do SC_SIMPLE[SK[n]] = true end

local SC_PARTY = {}
for _, n in ipairs({ "AL_ANGELUS", "PR_MAGNIFICAT", "PR_GLORIA", "BS_ADRENALINE", "BS_WEAPONPERFECT", "BS_OVERTHRUST" }) do
  SC_PARTY[SK[n]] = true
end

local SC_TOGGLE = {}
for _, n in ipairs({ "BS_MAXIMIZE", "NV_TRICKDEAD", "TF_HIDING", "AS_CLOAKING" }) do SC_TOGGLE[SK[n]] = true end

local ELEMENT_CHANGE = {}
for _, n in ipairs({ "NPC_ATTRICHANGE", "NPC_CHANGEWATER", "NPC_CHANGEGROUND", "NPC_CHANGEFIRE", "NPC_CHANGEWIND",
                     "NPC_CHANGEPOISON", "NPC_CHANGEHOLY", "NPC_CHANGEDARKNESS", "NPC_CHANGETELEKINESIS" }) do
  ELEMENT_CHANGE[SK[n]] = true
end

-- skill.c:4132 Cloaking only holds next to a wall
function skill_check_cloaking(bl_id)
  local bl = get(bl_id)
  if not bl or not sc(bl, SC.CLOAKING) then return end
  for dx = -1, 1 do
    for dy = -1, 1 do
      if (dx ~= 0 or dy ~= 0) then
        local c = _rt("cell", bl.map, bl.x + dx, bl.y + dy)
        if c == 1 or c == 5 then return end
      end
    end
  end
  skill_status_change_end(bl_id, SC.CLOAKING)
end

-- skill.c:1113
function skill_castend_nodamage_id(src_id, bl_id, skillid, skilllv, opt)
  opt = opt or {}
  local src, bl = get(src_id), get(bl_id)
  if not src or not bl or src.dead then return end
  if bl.dead and skillid ~= SK.ALL_RESURRECTION then return end
  local sd = src.kind == "player" and src or nil
  local md = src.kind == "mob" and src or nil

  if skillid == SK.AL_HEAL then
    local heal = battle.calc_heal(src, skilllv)
    if battle.dcard(bl, 4128) then heal = 0 end -- Golden Thief Bug Card: no healing either
    nodamage(src_id, bl_id, skillid, heal)
    battle.heal(bl_id, heal, 0)

  elseif skillid == SK.ALL_RESURRECTION then
    -- skill.c:1196: back up with 10 / 30 / 50 / 80% of max HP (an Osiris Card: all HP and SP)
    if bl.kind == "player" and bl.dead then
      nodamage(src_id, bl_id, skillid, skilllv)
      local per = ({ 10, 30, 50, 80 })[skilllv] or 10
      local hp = math.max(1, floor(bl.st.max_hp * per / 100))
      if battle.dcard(bl, 4144) then hp = bl.st.max_hp end
      _rt("bl_stand", bl_id, hp)
      if battle.dcard(bl, 4144) then _rt("bl_set", bl_id, "sp", bl.st.max_sp) end
    end

  elseif SC_SIMPLE[skillid] then
    -- Kyrie's effect shows on the one protected
    nodamage(skillid == SK.PR_KYRIE and bl_id or src_id, bl_id, skillid, skilllv)
    if skillid == SK.SM_PROVOKE then
      -- not on the undead (the client's skill text; neither emulator checks)
      if battle.elem_type(bl) == 9 or battle.race(bl) == 1 then return end
    end
    skill_status_change_start(bl_id, SkillStatusChangeTable[skillid], skilllv, 0)
    if skillid == SK.SM_PROVOKE and bl.kind == "mob" then _rt("mob_target", bl_id, src_id, skill_get_range(skillid)) end

  elseif skillid == SK.AL_CRUCIS then
    -- 817:1678 / 390: every undead or demon on screen may lose DEF
    nodamage(src_id, bl_id, skillid, skilllv)
    for _, t in ipairs(skill_area(src, src.map, src.x, src.y, 14, BCT.ENEMY)) do
      local race = battle.race(t)
      if (race == 6 or race == 1) and rnd(100) < skilllv * 2 + battle.lv(src) + 28 - battle.lv(t) then
        skill_status_change_start(t.id, SC.SIGNUMCRUCIS, skilllv, skilllv + 14) -- 817:5365 DEF down by 14% + 1% a level
        _rt("clif_emotion", t.id, 4)
      end
    end

  elseif skillid == SK.AC_MAKINGARROW then
    if sd then
      nodamage(src_id, bl_id, skillid, skilllv)
      skill_arrow_create_list(sd)
    end

  elseif skillid == SK.BS_HAMMERFALL then
    nodamage(src_id, bl_id, skillid, skilllv)
    if rnd(100) < 20 + 10 * skilllv then
      skill_status_change_start(bl_id, SC.STAN, skilllv, 0, { tick = skill_get_time2(skillid, skilllv) })
    end

  elseif skillid == SK.RG_RAID then
    nodamage(src_id, bl_id, skillid, skilllv)
    for _, t in ipairs(skill_area(src, bl.map, bl.x, bl.y, 1, BCT.ENEMY)) do
      skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = true })
    end
    skill_status_change_end(src_id, SC.HIDDING)

  elseif SC_PARTY[skillid] then
    -- skill.c:1332: the whole party; without one (a lone player, a monster), the target
    nodamage(bl_id, bl_id, skillid, skilllv)
    skill_status_change_start(bl_id, SkillStatusChangeTable[skillid], skilllv, 0)

  elseif SC_TOGGLE[skillid] then
    -- skill.c:1353: on, or off again
    if skillid ~= SK.AS_CLOAKING then nodamage(src_id, bl_id, skillid, skilllv) end
    local t = SkillStatusChangeTable[skillid]
    if not sc(bl, t) then skill_status_change_start(bl_id, t, skilllv, 0) else skill_status_change_end(bl_id, t) end
    if skillid == SK.AS_CLOAKING then skill_check_cloaking(bl_id) end

  elseif skillid == SK.TF_STEAL then
    if sd and pc_steal_item(sd, bl) then
      nodamage(src_id, bl_id, skillid, skilllv)
      _rt("mob_target", bl_id, src_id, skill_get_range(skillid))
    else
      nodamage(src_id, bl_id, skillid, skilllv, false)
    end

  elseif skillid == SK.RG_STEALCOIN then
    if sd and pc_steal_coin(sd, bl) then
      nodamage(src_id, bl_id, skillid, skilllv)
      _rt("mob_target", bl_id, src_id, skill_get_range(skillid))
    else
      nodamage(src_id, bl_id, skillid, skilllv, false)
    end

  elseif skillid == SK.MG_STONECURSE then
    nodamage(src_id, bl_id, skillid, skilllv)
    if rnd(100) < skilllv * 4 + 20 then
      skill_status_change_start(bl_id, SC.STONE, skilllv, 0, { tick = skill_get_time2(skillid, skilllv) })
    end

  elseif skillid == SK.NV_FIRSTAID then
    nodamage(src_id, bl_id, skillid, 5)
    battle.heal(bl_id, 5, 0)

  elseif skillid == SK.AL_CURE then
    nodamage(src_id, bl_id, skillid, skilllv)
    skill_status_change_end(bl_id, SC.SILENCE)
    skill_status_change_end(bl_id, SC.BLIND)
    skill_status_change_end(bl_id, SC.CONFUSION)

  elseif skillid == SK.TF_DETOXIFY then
    nodamage(src_id, bl_id, skillid, skilllv)
    skill_status_change_end(bl_id, SC.POISON)

  elseif skillid == SK.PR_STRECOVERY then
    nodamage(src_id, bl_id, skillid, skilllv)
    skill_status_change_end(bl_id, SC.FREEZE)
    skill_status_change_end(bl_id, SC.STONE)
    skill_status_change_end(bl_id, SC.SLEEP)
    skill_status_change_end(bl_id, SC.STAN)

  elseif skillid == SK.WZ_ESTIMATION then
    if sd and bl.kind == "mob" then
      nodamage(src_id, bl_id, skillid, skilllv)
      local fix = {}
      for ele = 1, 9 do fix[ele] = math.max(0, battle.attr_fix(100, ele, bl.db.element)) end
      _rt("clif_estimation", src_id, bl_id, fix)
    end

  elseif skillid == SK.MC_IDENTIFY then
    if sd then _rt("pc_identify_list", src_id) end

  elseif skillid == SK.MC_VENDING then
    -- a street shop needs customers: nothing to open with one player (docs/DEVIATIONS.md)
    if sd then _rt("clif_message", src_id, "There is nobody to sell to.") end

  elseif skillid == SK.AL_TELEPORT then
    -- skill.c:1472: level 1 anywhere on the map; level 2 offers the save point
    nodamage(src_id, bl_id, skillid, skilllv)
    if sd then
      if _rt("map_flag", sd.map, "noteleport") then
        _rt("clif_map_info", src_id, 0) -- ours: the client's "Unable to Teleport in this area" (mod0631 says nothing)
        return
      end
      if skilllv == 1 then
        _rt("pc_warp", src_id, "Random")
      else
        local save = _rt("pc_save_point", src_id)
        _rt("clif_warppoint", src_id, skillid, skilllv, { "Random", save.map .. ".gat" }, 0, 0)
      end
    elseif bl.kind == "mob" then
      _rt("mob_warp", bl_id)
    end

  elseif skillid == SK.AL_HOLYWATER then
    if sd then
      nodamage(src_id, bl_id, skillid, skilllv)
      _rt("pc_item_add", src_id, 523, 1)
    end

  elseif skillid == SK.TF_PICKSTONE then
    -- 817:2188 a Stone (7049) from the ground
    if sd then
      nodamage(src_id, bl_id, skillid, skilllv)
      _rt("pc_item_add", src_id, 7049, 1)
    end

  elseif skillid == SK.WZ_FROSTNOVA then
    for _, t in ipairs(skill_area(src, bl.map, bl.x, bl.y, 1, BCT.ENEMY)) do
      skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = true, main = bl_id, cx = bl.x, cy = bl.y })
    end

  elseif skillid == SK.KN_BRANDISHSPEAR then
    -- 817:1913 hits rows of cells in front of the knight, more rows with the
    -- level (4, 7, 10); the nearer rows hit harder (battle.lua). 817 lays the
    -- rows out with a helper of its own; ours follow the drawing on the iRO
    -- guide's Knight page (2004-04-13): area 1 is the two cells either side
    -- of the knight and the five in front, areas 2 and 3 five cells each,
    -- area 4 the middle three (docs/DEVIATIONS.md).
    local dx = bl.x - src.x; dx = dx > 0 and 1 or (dx < 0 and -1 or 0)
    local dy = bl.y - src.y; dy = dy > 0 and 1 or (dy < 0 and -1 or 0)
    if dx == 0 and dy == 0 then dy = 1 end
    local rows = 1 + (skilllv > 3 and 1 or 0) + (skilllv > 6 and 1 or 0) + (skilllv > 9 and 1 or 0)
    local hit = {}
    for row = 0, rows do
      local half = row == 4 and 1 or 2
      for w = -half, half do
        -- (-dy, dx) runs across the line of attack; row 0 is the knight's own
        local x, y = src.x + dx * row - dy * w, src.y + dy * row + dx * w
        if row > 0 or w ~= 0 then
          for _, t in ipairs(skill_area(src, src.map, x, y, 0, BCT.ENEMY)) do
            if not hit[t.id] then
              hit[t.id] = true
              skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = t.id ~= bl_id, wflag = math.max(row, 1) })
            end
          end
        end
      end
    end
    if not hit[bl_id] then skill_castend_damage_id(src_id, bl_id, skillid, skilllv, { wflag = 1 }) end

  elseif skillid == SK.AS_SPLASHER then
    -- Not in mod0631. eAthena 817 skill.c:1326 makes it a plain splash: the
    -- target and the enemies within 2 cells (its damage: battle.lua). The
    -- delayed burst of the client's skill text is in neither.
    skill_attack(BF.WEAPON, src_id, src_id, bl_id, skillid, skilllv, 0)
    for _, t in ipairs(skill_area(src, bl.map, bl.x, bl.y, 2, BCT.ENEMY)) do
      if t.id ~= bl_id then skill_attack(BF.WEAPON, src_id, src_id, t.id, skillid, skilllv, 0, { type = 5 }) end
    end

  elseif skillid == SK.RG_INTIMIDATE then
    if sd then
      if _rt("map_flag", sd.map, "noteleport") then return end
      _rt("pc_warp", src_id, "Random")
    elseif bl.kind == "mob" then
      _rt("mob_warp", bl_id)
    end

  elseif skillid == SK.TF_BACKSLIDING then
    -- skill.c:1671: five cells backwards
    _rt("stop_walking", src_id)
    skill_blown(nil, bl_id, 5, true)
    _rt("clif_fixpos", src_id)
    battle.after(200, function() nodamage(src_id, src_id, skillid, skilllv) end)

  elseif ELEMENT_CHANGE[skillid] then
    -- skill.c:1679: the monster takes another element, at a random level
    if md then
      nodamage(src_id, bl_id, skillid, skilllv)
      local ele = skill_get_pl(skillid)
      if ele == 0 then ele = rnd(9) end -- any but Undead
      _rt("bl_set", src_id, "def_ele", ele + (1 + rnd(4)) * 20)
    end

  elseif skillid == SK.NPC_SUICIDE then
    if md then
      nodamage(src_id, bl_id, skillid, skilllv)
      _rt("mob_kill", src_id)
    end

  elseif skillid == SK.NPC_SUMMONSLAVE or skillid == SK.NPC_SUMMONMONSTER then
    -- mob.c:1836 mob_summonslave: `val` of the skill line is the monster, the level how many
    if md and opt.val and opt.val > 1000 then
      for _ = 1, skilllv do
        local x, y
        for _ = 1, 50 do
          x, y = md.x + rnd(9) - 4, md.y + rnd(9) - 4
          local c = _rt("cell", md.map, x, y)
          if c ~= 1 and c ~= 5 then break end
          x, y = md.x, md.y
        end
        _rt("mob_spawn", md.map, x, y, opt.val, skillid == SK.NPC_SUMMONSLAVE and src_id or 0)
      end
    end

  elseif skillid == SK.NPC_EMOTION then
    if md then _rt("clif_emotion", src_id, opt.val or 0) end

  elseif skillid == SK.MC_CHANGECART then
    -- Neither period emulator has it. The client's skill text: "Change the
    -- appearance of cart among 5 kinds, suited by the level of character".
    -- The level steps (41, 66, 81, 91) are later emulators' (tier later).
    if sd and sd.st.cart then
      nodamage(src_id, bl_id, skillid, skilllv)
      local lv = sd.char.base_level
      local bit = 0x08
      if lv > 90 then bit = 0x400 elseif lv > 80 then bit = 0x200 elseif lv > 65 then bit = 0x100 elseif lv > 40 then bit = 0x80 end
      _rt("pc_set_option", src_id, (sd.char.option & ~0x788) | bit)
    end
  end
end

-- ------------------------------------------------------------ ground-targeted skills

local UNIT_SKILLS = {}
for _, n in ipairs({
  "MG_SAFETYWALL", "MG_FIREWALL", "AL_PNEUMA", "WZ_ICEWALL", "WZ_FIREPILLAR", "WZ_QUAGMIRE", "WZ_VERMILION",
  "WZ_STORMGUST", "PR_SANCTUARY", "PR_MAGNUS", "HT_SKIDTRAP", "HT_LANDMINE", "HT_ANKLESNARE", "HT_SHOCKWAVE",
  "HT_SANDMAN", "HT_FLASHER", "HT_FREEZINGTRAP", "HT_BLASTMINE", "HT_CLAYMORETRAP", "AS_VENOMDUST", "HT_TALKIEBOX",
}) do UNIT_SKILLS[SK[n]] = true end

-- skill.c:1795
function skill_castend_pos2(src_id, x, y, skillid, skilllv, opt)
  local src = get(src_id)
  if not src then return end
  local sd = src.kind == "player" and src or nil
  if skillid ~= SK.WZ_METEOR then _rt("clif_skill_poseffect", src_id, skillid, skilllv, x, y) end

  if skillid == SK.MG_THUNDERSTORM or skillid == SK.WZ_HEAVENDRIVE then
    for _, t in ipairs(skill_area(src, src.map, x, y, 2, BCT.ENEMY)) do
      skill_castend_damage_id(src_id, t.id, skillid, skilllv, { sub = true, ground = true })
    end

  elseif skillid == SK.PR_BENEDICTIO then
    for _, t in ipairs(skill_area(src, src.map, x, y, 1, BCT.NOENEMY)) do
      skill_castend_nodamage_id(src_id, t.id, skillid, skilllv)
    end
    for _, t in ipairs(skill_area(src, src.map, x, y, 1, BCT.ENEMY)) do
      skill_castend_damage_id(src_id, t.id, skillid, skilllv)
    end

  elseif skillid == SK.BS_HAMMERFALL then
    for _, t in ipairs(skill_area(src, src.map, x, y, 2, BCT.ENEMY)) do
      skill_castend_nodamage_id(src_id, t.id, skillid, skilllv)
    end

  elseif skillid == SK.HT_DETECTING then
    -- neither emulator has it. The client's text: the falcon detects hidden
    -- enemies from a distance. Ours: reveals those within 3 cells of the spot.
    for _, id in ipairs(_rt("in_area", src.map, x - 3, y - 3, x + 3, y + 3)) do
      skill_status_change_end(id, SC.HIDDING)
      skill_status_change_end(id, SC.CLOAKING)
    end

  elseif UNIT_SKILLS[skillid] then
    skill_unitsetting(src_id, skillid, skilllv, x, y, 0)

  elseif skillid == SK.WZ_METEOR then
    -- skill.c:1863: 2 + level/2 meteors, a second apart, on random cells within 2 of the spot
    local n = 2 + (skilllv >> 1)
    local map = src.map
    -- a meteor does not follow a caster who left the map (skill.c:671)
    local function here()
      local s = get(src_id)
      return s and not s.dead and s.map == map
    end
    local px, py
    for i = 0, n - 1 do
      local tx, ty = x + rnd(5) - 2, y + rnd(5) - 2
      if i == 0 then
        _rt("clif_skill_poseffect", src_id, skillid, skilllv, tx, ty)
      else
        local fx, fy, sx, sy = px, py, tx, ty
        battle.after(i * 1000, function()
          if not here() then return end
          skill_unitsetting(src_id, skillid, skilllv, fx, fy, 0)
          _rt("clif_skill_poseffect", src_id, skillid, skilllv, sx, sy)
        end)
      end
      px, py = tx, ty
    end
    local lx, ly = px, py
    battle.after(n * 1000, function() if here() then skill_unitsetting(src_id, skillid, skilllv, lx, ly, 0) end end)

  elseif skillid == SK.AL_WARP then
    -- skill.c:1893: where to? the save point, then one memo point per level above 1
    if not sd then return end
    if _rt("map_flag", sd.map, "noteleport") then
      _rt("clif_map_info", src_id, 0) -- ours, as for Teleport
      return
    end
    local save = _rt("pc_save_point", src_id)
    local memo = _rt("pc_memo", src_id) or {}
    local maps = { save.map .. ".gat" }
    for i = 1, 3 do
      if skilllv > i and memo[i] then maps[#maps + 1] = memo[i].map .. ".gat" end
    end
    _rt("clif_warppoint", src_id, skillid, skilllv, maps, x, y)
  end
end

-- skill.c:1914: the player picked a destination (Teleport, Warp Portal).
function rules.skill_castend_map(a)
  local sd = get(a.src)
  if not sd or sd.dead then return end
  if sd.opt1 > 0 or (sd.option & 6) ~= 0 or sc(sd, SC.DIVINA) then return end
  local map = (a.map or ""):gsub("%.gat$", "")
  _rt("stop_attack", a.src)
  if map == "cancel" or map == "" then return end
  if a.skill == SK.AL_TELEPORT then
    if map == "Random" then _rt("pc_warp", a.src, "Random") else _rt("pc_warp", a.src, "SavePoint") end
  elseif a.skill == SK.AL_WARP then
    local save = _rt("pc_save_point", a.src)
    local memo = _rt("pc_memo", a.src) or {}
    local points = { save }
    for i = 1, 3 do if a.lv > i and memo[i] then points[#points + 1] = memo[i] end end
    for _, p in ipairs(points) do
      if p.map == map then
        local g = skill_unitsetting(a.src, a.skill, a.lv, a.x, a.y, 0)
        if g then g.valstr, g.to_x, g.to_y = p.map, p.x, p.y end
        return
      end
    end
  end
end

-- ------------------------------------------------------------ Arrow Crafting

-- db/create_arrow_db.txt of mod0631: item -> up to five (arrow, amount) pairs.
-- skill.c:4708 skill_arrow_create.
skill_arrow_db = skill_arrow_db or {}

function skill_arrow_create_list(sd)
  local ids = {}
  for item in pairs(skill_arrow_db) do
    if _rt("pc_item_count", sd.id, item) > 0 then ids[#ids + 1] = item end
  end
  table.sort(ids)
  _rt("pc_arrow_list", sd.id, ids)
end

function rules.arrow_create(a)
  local sd = get(a.src)
  local recipe = skill_arrow_db[a.item]
  if not sd or sd.dead or not recipe or checkskill(sd, SK.AC_MAKINGARROW) == 0 then return end
  if not _rt("pc_item_take", a.src, a.item, 1) then return end
  for _, r in ipairs(recipe) do
    if r[1] > 0 and r[2] > 0 then _rt("pc_item_add", a.src, r[1], r[2]) end
  end
end

-- The Monk: spirit spheres, combos and Blade Stop. A draft (the 2-2 jobs, Juno, June 2004,
-- are not in the default world; docs/DEVIATIONS.md). Server-side: NOT in the client.
--
-- Source: jAthena //1020 (2004-06-08, git f7c1891; its Monk dates from //0507-//0544 of
-- 2003-10/11 and //0966 of 2004-05-07): src/map/pc.c pc_addspiritball (164) and
-- pc_delspiritball (198); skill.c skill_attack's combo windows (917), skill_castend_damage_id
-- (1590-1712), skill_castend_nodamage_id (2383-2420), skill_check_condition (5244-5286,
-- 5333), skill_use_id (5571-5578, 5692-5714); battle.c 3307-3342 (damage), 3676 (+3 a
-- sphere), 4278 and 4333-4346 (Triple Attack starts a combo), 4484 (Blade Stop catches).
-- Numbers checked against the iRO guide's Monk page (capture 2004-03-27,
-- refs/web/iro-guide-2004/20040327035012_jobmonskill.txt); where the two differ the guide
-- is taken and the place says so.
--
-- This client draws none of it: it has no packet for the spheres (0x01d0), the combo
-- window (0x01d2) or Blade Stop (0x01d1) (THREADS.md #25), and its skill window offers no
-- 2-2 skill. euRO's May-2004 client has all three (research/notes/may2004-client.md).

local floor = math.floor
local get = battle.get
local sc = battle.sc
local checkskill = battle.pc_checkskill

-- ------------------------------------------------------------ spirit spheres

-- player id -> list of the times each sphere runs out, oldest first (jAthena's
-- spirit_timer[]). Not saved: jAthena starts every login with none (pc.c:658).
spirit_balls = spirit_balls or {}

local function balls_of(id)
  local now = _rt("now")
  local list = spirit_balls[id] or {}
  local live = {}
  for _, at in ipairs(list) do if at > now then live[#live + 1] = at end end
  spirit_balls[id] = live
  return live
end

function pc_spiritball(id) return #balls_of(id) end

-- One more sphere lasting `interval` ms, at most `max`; at the most, the oldest goes
-- to make room (pc.c:178).
function pc_addspiritball(id, interval, max)
  local list = balls_of(id)
  if #list >= max then table.remove(list, 1) end
  list[#list + 1] = _rt("now") + interval
end

-- Spend `count` spheres, the oldest first.
function pc_delspiritball(id, count)
  local list = balls_of(id)
  for _ = 1, math.min(count, #list) do table.remove(list, 1) end
end

-- The spheres a skill costs (jAthena //1020 skill_require_db; the iRO guide agrees on each):
-- Throw Spirit Sphere one a level, or what is left if fewer; Guillotine Fist 5, or 4 straight
-- after Combo Finish or in Blade Stop (skill.c:5283; the guide's "Fury, 4 Spheres needed")
local SPHERES = {
  [SK.MO_INVESTIGATE] = 1, [SK.MO_STEELBODY] = 5, [SK.MO_BLADESTOP] = 1, [SK.MO_EXPLOSIONSPIRITS] = 5,
  [SK.MO_EXTREMITYFIST] = 5, [SK.MO_COMBOFINISH] = 1, [SK.MO_BODYRELOCATION] = 1,
}
SPHERES[-1] = nil
function monk_spheres_needed(sd, skill, lv)
  if skill == SK.MO_FINGEROFFENSIVE then
    local have = pc_spiritball(sd.id)
    if have > 0 and have < lv then return have end
    return lv
  end
  local n = SPHERES[skill] or 0
  if skill == SK.MO_EXTREMITYFIST and (monk_in_combo(sd, SK.MO_COMBOFINISH) or sc(sd, SC.BLADESTOP)) then n = n - 1 end
  return n
end

-- Guillotine Fist straight after Combo Finish reaches the one it threw back (skill.c:3415
-- adds Combo Finish's push to the range; 5 cells here, battle.lua)
function monk_range_bonus(sd, skill)
  if skill == SK.MO_EXTREMITYFIST and monk_in_combo(sd, SK.MO_COMBOFINISH) then return 5 end
  return 0
end

-- How many spheres the last Throw Spirit Sphere threw, by player (jAthena's spiritball_old)
finger_offensive_count = finger_offensive_count or {}

-- ------------------------------------------------------------ combos

-- The window after Triple Attack, Chain Combo and Combo Finish in which the next blow of
-- the chain may follow: 1000 ms (700 after Combo Finish) less 4 a point of AGI and 2 of
-- DEX, 300 ms more when the Monk can go on (skill.c:917; battle_config.combo_delay_rate
-- 100). The iRO guide: "the caster cannot move for a short period ... The delay time is
-- affected by DEX and AGI of the caster", longer "if 'Raging Quadruple Blow' is used in
-- conjunction".
local function combo_window(sd, base, more)
  local d = base - 4 * battle.agi(sd) - 2 * battle.dex(sd)
  if more then d = d + 300 end
  return math.max(0, d)
end

-- After a blow of the chain: the next may follow while SC.COMBO holds (val1 = the blow
-- just struck), unless the target fell; the Monk stands still and does not swing in the
-- meantime (skill.c:917, pc.c:3869).
function monk_combo_after(sd_id, skillid, target_id)
  local sd = get(sd_id)
  if not sd or sd.kind ~= "player" then return 0 end
  local balls = pc_spiritball(sd_id)
  local more, base = false, 1000
  if skillid == SK.MO_TRIPLEATTACK then
    more = checkskill(sd, SK.MO_CHAINCOMBO) > 0
  elseif skillid == SK.MO_CHAINCOMBO then
    more = checkskill(sd, SK.MO_COMBOFINISH) > 0 and balls > 0
  elseif skillid == SK.MO_COMBOFINISH then
    base = 700
    more = checkskill(sd, SK.MO_EXTREMITYFIST) > 0 and balls >= 4 and sc(sd, SC.EXPLOSIONSPIRITS) ~= nil
  end
  local delay = combo_window(sd, base, more)
  local t = get(target_id)
  if t and not t.dead then
    skill_status_change_start(sd_id, SC.COMBO, skillid, 0, { tick = delay })
  end
  _rt("bl_set", sd_id, "canmove", delay)
  return delay
end

-- The blow a combo skill follows (skill_check_condition 5263-5286).
COMBO_AFTER = {
  [SK.MO_CHAINCOMBO] = SK.MO_TRIPLEATTACK,
  [SK.MO_COMBOFINISH] = SK.MO_CHAINCOMBO,
}
COMBO_AFTER[-1] = nil

function monk_in_combo(sd, after)
  local t = sc(sd, SC.COMBO)
  return t ~= nil and t.val1 == after
end

-- ------------------------------------------------------------ Blade Stop

-- What a Monk holding a monster in Blade Stop may still use, by the level it was caught
-- at (skill.c:5571; the iRO guide's "Usable Skill" column: 2 Throw Spirit Sphere, 3
-- Occult Impaction, 4 Raging Quadruple Blow, 5 Guillotine Fist). Level 1: nothing.
local BLADE_STOP_SKILLS = {
  [SK.MO_FINGEROFFENSIVE] = 2, [SK.MO_INVESTIGATE] = 3, [SK.MO_CHAINCOMBO] = 4, [SK.MO_EXTREMITYFIST] = 5,
}
BLADE_STOP_SKILLS[-1] = nil

-- May `skill` be used under Blade Stop? (no Blade Stop: yes)
function blade_stop_allows(sd, skill)
  local t = sc(sd, SC.BLADESTOP)
  if not t then return true end
  if t.val2 == 1 then return false end -- the one caught
  local need = BLADE_STOP_SKILLS[skill]
  return need ~= nil and t.val1 >= need
end

-- The one the Monk holds (or is held by), if both still hold.
function blade_stop_partner(bl)
  local t = sc(bl, SC.BLADESTOP)
  if not t then return nil end
  local p = get(t.val3)
  if not p or p.dead or p.map ~= bl.map or not sc(p, SC.BLADESTOP) then return nil end
  return p
end

-- A plain blow lands on a Monk waiting with Blade Stop: both are held for 20 s to 60 s
-- (battle.c:4484, cast_db upkeep_time2; the iRO guide "Skill Duration"). val1 the level,
-- val2 1 for the one caught and 2 for the Monk, val3 the other one. Not a boss: the
-- guide, "This skill is not applicable for boss monsters"; jAthena catches any.
function blade_stop_catch(src, target)
  local w = sc(target, SC.BLADESTOP_WAIT)
  if not w or target.dead or src.dead then return end
  if src.kind == "mob" and battle.is_boss(src) then return end
  local lv = w.val1
  skill_status_change_end(target.id, SC.BLADESTOP_WAIT)
  local tick = skill_get_time2(SK.MO_BLADESTOP, lv)
  if tick <= 0 then tick = 10000 + 10000 * lv end
  skill_status_change_start(src.id, SC.BLADESTOP, lv, 1, { tick = tick, val3 = target.id })
  skill_status_change_start(target.id, SC.BLADESTOP, lv, 2, { tick = tick, val3 = src.id })
  _rt("stop_attack", src.id)
  _rt("stop_walking", src.id)
  _rt("stop_walking", target.id)
end

-- The Monk lets go (after Throw Spirit Sphere, Occult Impaction, Raging Quadruple Blow or
-- Guillotine Fist: skill.c:1594-1709), and so does the one held.
function blade_stop_release(id)
  local bl = get(id)
  if not bl then return end
  local p = blade_stop_partner(bl)
  skill_status_change_end(id, SC.BLADESTOP)
  if p then skill_status_change_end(p.id, SC.BLADESTOP) end
end

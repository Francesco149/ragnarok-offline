-- Death and natural recovery. Server-side: NOT in the client.
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/pc.c (archived
-- 2003-12-18) pc_damage, pc_natural_heal_hp / _sp, pc_spirit_heal and
-- conf/battle_athena.conf. Attacks and skills: battle.lua, skill.lua.

local floor = math.floor

-- pc.c:208 pc_setrestartvalue, with mod0631's battle_athena.conf
-- (death_penalty_type 0, death_penalty_base / _job 100, restart_hp_rate 0,
-- restart_sp_rate 0): what a dead character comes back with at the save
-- point, and what dying cost.
--   HP: 1; a Novice half its max HP; with an Osiris Card all HP and SP.
--   EXP: 1% of the base and job EXP *held* (type 0; not of the EXP the next
--   level needs), nothing for Novices, at the top level, or on a map
--   flagged nopenalty.
-- t: class, max_hp, max_sp, sp, base_exp, job_exp, next_base, next_job,
--    nopenalty, osiris. Returns {hp, sp, base, job} (base/job = EXP lost).
function rules.respawn(t)
  local r = { hp = 1, sp = t.sp, base = 0, job = 0 }
  if t.osiris then
    r.hp, r.sp = t.max_hp, t.max_sp
  elseif t.class == 0 then
    r.hp = floor(t.max_hp / 2)
  end
  if t.class ~= 0 and not t.nopenalty then
    if t.next_base > 0 then r.base = floor(t.base_exp * 100 / 10000) end
    if t.next_job > 0 then r.job = floor(t.job_exp * 100 / 10000) end
  end
  return r
end

-- pc.c:3806-4060. The server runs the clocks (crates/ro-game/src/combat.rs
-- tick_regen); this says how fast they run and what each tick gives.
--   hp / sp, hp_interval / sp_interval: every 4 s (HP, standing still) and
--     8 s (SP) of clock: 1 + vit/6 + max_hp/200 HP, 1 + int/6 + max_sp/100 SP
--   hp_clock / sp_clock: percent of real time the clocks run at (sitting
--     doubles both; Magnificat doubles SP; 0 = stopped)
--   inc_hp / inc_sp / inc_interval: Increase HP / SP Recovery, every 10 s
--     standing still: level*5 + max_hp*level/500, level*3 + max_sp*level/500
--   spirit_hp / spirit_sp / spirit_interval: Spiritual Cadence (Monk), sitting
-- Nothing recovers at half the weight limit or more, or while hidden
-- (pc.c:4036). t: max_hp, max_sp, vit, int, sitting, walking, weight,
-- max_weight, hiding, skills = {["4"] = level}, sc = {["20"] = true}.
function rules.regen(t)
  local skills, sc = t.skills or {}, t.sc or {}
  local function skill(id) return skills[tostring(id)] or 0 end
  local r = {
    hp = 1 + floor(t.vit / 6) + floor(t.max_hp / 200), hp_interval = 4000,
    sp = 1 + floor(t.int / 6) + floor(t.max_sp / 100), sp_interval = 8000,
    hp_clock = 100, sp_clock = 100,
  }
  local over50 = t.max_weight and t.max_weight > 0 and t.weight * 2 >= t.max_weight
  if over50 or t.hiding then
    r.hp_clock, r.sp_clock = 0, 0
  else
    if t.sitting then r.hp_clock, r.sp_clock = 200, 200 end
    if sc[tostring(SC.MAGNIFICAT)] then r.sp_clock = r.sp_clock * 2 end
    if sc[tostring(SC.EXPLOSIONSPIRITS)] then r.sp_clock = 0 end -- pc.c:3933
    if t.walking then
      -- pc.c:3846: only Moving HP Recovery heals on the move, a quarter as much
      if skill(SK.SM_MOVINGRECOVERY) > 0 then
        r.hp = math.max(1, floor(r.hp / 4))
      else
        r.hp_clock = 0
      end
    end
    local lv = skill(SK.SM_RECOVERY)
    if lv > 0 then r.inc_hp = lv * 5 + floor(t.max_hp * lv / 500) end
    lv = skill(SK.MG_SRECOVERY)
    if lv > 0 then r.inc_sp = lv * 3 + floor(t.max_sp * lv / 500) end
    r.inc_interval = 10000
  end
  -- pc.c:3976 Spiritual Cadence: sitting, every 10 s (20 s at half weight)
  local lv = skill(SK.MO_SPIRITSRECOVERY)
  if lv > 0 and t.sitting then
    r.spirit_hp = lv * 4 + floor(t.max_hp * lv / 500)
    r.spirit_sp = lv * 2 + floor(t.max_sp * lv / 500)
    r.spirit_interval = over50 and 20000 or 10000
  end
  return r
end

-- pc.c:2992 pc_itemheal: what a healing item gives grows with VIT (HP) or INT
-- (SP), 1% a point, and with Increase HP / SP Recovery, 10% a level
-- (Alchemists' Potion Research: 5% a level).
function item_heal(hp, sp)
  local sd = battle.get(player.id)
  if not sd or sd.dead then return end
  hp, sp = hp or 0, sp or 0
  local learn = 100 + battle.pc_checkskill(sd, SK.AM_LEARNINGPOTION) * 5
  if hp > 0 then
    hp = floor(hp * (sd.st.vit + 100 + battle.pc_checkskill(sd, SK.SM_RECOVERY) * 10) / 100)
    hp = floor(hp * learn / 100)
  end
  if sp > 0 then
    sp = floor(sp * (sd.st.int + 100 + battle.pc_checkskill(sd, SK.MG_SRECOVERY) * 10) / 100)
    sp = floor(sp * learn / 100)
  end
  heal(hp, sp)
end

-- pc.c:1409 pc_modifybuyvalue / pc_modifysellvalue: Discount and Overcharge
-- change shop prices by 7% at level 1, 2% more a level, 24% at level 10.
-- t: price, skills = {["37"] = level}.
local function trade_rate(lv) return 5 + lv * 2 - (lv == 10 and 1 or 0) end
function rules.buy_price(t)
  local price = t.price
  local lv = (t.skills or {})[tostring(SK.MC_DISCOUNT)] or 0
  if lv > 0 then price = price - floor(price * trade_rate(lv) / 100) end
  lv = (t.skills or {})[tostring(SK.RG_COMPULSION)] or 0
  if lv > 0 then price = price - floor(price * (5 + lv * 4) / 100) end
  return price
end
function rules.sell_price(t)
  local price = t.price
  local lv = (t.skills or {})[tostring(SK.MC_OVERCHARGE)] or 0
  if lv > 0 then price = price + floor(price * trade_rate(lv) / 100) end
  return price
end

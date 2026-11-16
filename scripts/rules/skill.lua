-- Skills: using one (range, cast time, what it costs), and the machinery
-- every skill shares (skill_attack, knock-back, side effects, timers).
-- What each skill does is in skill_effects.lua, ground skills in
-- skill_units.lua. Server-side: NOT in the client.
--
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/skill.c (archived
-- 2003-12-18): skill_use_id (3205), skill_use_pos (3347), skill_castend_id
-- (1724), skill_castend_pos (2775), skill_check_condition (2825),
-- skill_castfix (3165), skill_attack (527), skill_blown (424). Line refs are
-- to that file. Side effects of a hit (skill_additional_effect) follow
-- eAthena 817 map/skill.c (2004-02-19), marked "817:": see status_change.lua.
--
-- The server owns the packets and the clock: it calls rules.skill_use when
-- the client asks (0x0113 / 0x0116), shows the cast bar, and calls
-- rules.skill_castend when the cast time is over (crates/ro-game/src/battle.rs).

local floor, random = math.floor, math.random
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end
local get = battle.get
local sc = battle.sc
local checkskill = battle.pc_checkskill

-- skill id -> the status change it starts (skill.c:22 SkillStatusChangeTable)
SkillStatusChangeTable = {
  [SK.SM_PROVOKE] = SC.PROVOKE, [SK.SM_ENDURE] = SC.ENDURE, [SK.MG_SIGHT] = SC.SIGHT,
  [SK.MG_FROSTDIVER] = SC.FREEZE, [SK.MG_STONECURSE] = SC.STONE, [SK.AL_RUWACH] = SC.RUWACH,
  [SK.AL_INCAGI] = SC.INCREASEAGI, [SK.AL_DECAGI] = SC.DECREASEAGI, [SK.AL_CRUCIS] = SC.SIGNUMCRUCIS,
  [SK.AL_ANGELUS] = SC.ANGELUS, [SK.AL_BLESSING] = SC.BLESSING, [SK.AC_CONCENTRATION] = SC.CONCENTRATE,
  [SK.TF_HIDING] = SC.HIDDING, [SK.KN_TWOHANDQUICKEN] = SC.TWOHANDQUICKEN, [SK.KN_AUTOCOUNTER] = SC.AUTOCOUNTER,
  [SK.PR_IMPOSITIO] = SC.IMPOSITIO, [SK.PR_SUFFRAGIUM] = SC.SUFFRAGIUM, [SK.PR_ASPERSIO] = SC.ASPERSIO,
  [SK.PR_BENEDICTIO] = SC.BENEDICTIO, [SK.PR_SLOWPOISON] = SC.SLOWPOISON, [SK.PR_KYRIE] = SC.KYRIE,
  [SK.PR_MAGNIFICAT] = SC.MAGNIFICAT, [SK.PR_GLORIA] = SC.GLORIA, [SK.PR_LEXDIVINA] = SC.DIVINA,
  [SK.PR_LEXAETERNA] = SC.AETERNA, [SK.WZ_QUAGMIRE] = SC.QUAGMIRE, [SK.BS_ADRENALINE] = SC.ADRENALINE,
  [SK.BS_WEAPONPERFECT] = SC.WEAPONPERFECTION, [SK.BS_OVERTHRUST] = SC.OVERTHRUST,
  [SK.BS_MAXIMIZE] = SC.MAXIMIZEPOWER, [SK.AS_CLOAKING] = SC.CLOAKING, [SK.AS_SONICBLOW] = SC.STAN,
  [SK.AS_ENCHANTPOISON] = SC.ENCPOISON, [SK.AS_POISONREACT] = SC.POISONREACT, [SK.AS_VENOMDUST] = SC.POISON,
  [SK.NV_TRICKDEAD] = SC.TRICKDEAD, [SK.MC_LOUD] = SC.LOUD, [SK.MG_ENERGYCOAT] = SC.ENERGYCOAT,
  [SK.CR_PROVIDENCE] = SC.PROVIDENCE, [SK.CR_SPEARQUICKEN] = SC.SPEARSQUICKEN, [SK.MO_STEELBODY] = SC.STEELBODY,
  [SK.CR_AUTOGUARD] = SC.AUTOGUARD, [SK.CR_DEFENDER] = SC.DEFENDER, [SK.CR_REFLECTSHIELD] = SC.REFLECTSHIELD,
  [SK.MO_EXPLOSIONSPIRITS] = SC.EXPLOSIONSPIRITS, [SK.SA_FLAMELAUNCHER] = SC.FLAMELAUNCHER,
  [SK.SA_FROSTWEAPON] = SC.FROSTWEAPON, [SK.SA_LIGHTNINGLOADER] = SC.LIGHTNINGLOADER,
  [SK.SA_SEISMICWEAPON] = SC.SEISMICWEAPON,
  [SK.RG_STRIPWEAPON] = SC.STRIPWEAPON, [SK.RG_STRIPSHIELD] = SC.STRIPSHIELD,
  [SK.RG_STRIPARMOR] = SC.STRIPARMOR, [SK.RG_STRIPHELM] = SC.STRIPHELM,
  [SK.AM_CP_WEAPON] = SC.CP_WEAPON, [SK.AM_CP_SHIELD] = SC.CP_SHIELD,
  [SK.AM_CP_ARMOR] = SC.CP_ARMOR, [SK.AM_CP_HELM] = SC.CP_HELM,
}
SkillStatusChangeTable[-1] = nil

-- ------------------------------------------------------------ timers

-- battle.after(ms, fn): run fn on a later battle tick (Athena add_timer).
local timers = {}
function battle.after(ms, fn)
  timers[#timers + 1] = { at = _rt("now") + ms, fn = fn }
end

-- Every 100 ms (crates/ro-game/src/battle.rs tick_battle).
function rules.battle_tick(a)
  for _, d in ipairs(a.sc_due or {}) do skill_status_change_timer(d[1], d[2]) end
  if #timers > 0 then
    local due, rest = {}, {}
    for _, t in ipairs(timers) do
      if t.at <= a.now then due[#due + 1] = t else rest[#rest + 1] = t end
    end
    timers = rest
    table.sort(due, function(x, y) return x.at < y.at end)
    for _, t in ipairs(due) do t.fn() end
  end
  skill_unit_timer(a.now)
end

-- ------------------------------------------------------------ side effects of a hit

-- 817: skill_additional_effect. Chances fall with the target's stats:
-- MDEF (stone, frozen), VIT (stun, poison, silence), INT (sleep, blind),
-- LUK (curse); monsters keep at least half the chance.
function skill_additional_effect(src_id, bl_id, skillid, skilllv, attack_type)
  local src, bl = get(src_id), get(bl_id)
  if not src or not bl or bl.dead then return end
  local sd = src.kind == "player" and src or nil
  local luk = battle.luk(bl)
  local sc_def_mdef = 100 - (3 + battle.mdef(bl) + floor(luk / 3))
  local sc_def_vit = 100 - (3 + battle.vit(bl) + floor(luk / 3))
  local sc_def_int = 100 - (3 + battle.int(bl) + floor(luk / 3))
  local sc_def_luk = 100 - (3 + luk)
  local floor_ = bl.kind == "mob" and 50 or 0
  sc_def_mdef, sc_def_vit = math.max(floor_, sc_def_mdef), math.max(floor_, sc_def_vit)
  sc_def_int, sc_def_luk = math.max(floor_, sc_def_int), math.max(floor_, sc_def_luk)
  local function start(type, lv, tick) skill_status_change_start(bl_id, type, lv, 0, { tick = tick }) end
  local function time2(id) return skill_get_time2(id or skillid, skilllv) end
  local undead = battle.elem_type(bl) == 9 or battle.race(bl) == 1

  if skillid == 0 then
    -- the falcon strikes by itself: (LUK/3 + 0.1)% a hit, at job level / 10 (skill.c:296)
    if sd and sd.st.falcon and sd.st.weapon == 11 and checkskill(sd, SK.HT_BLITZBEAT) > 0
      and rnd(1000) <= floor(sd.st.luk * 10 / 3) + 1 then
      local lv = floor((sd.char.job_level + 9) / 10)
      skill_castend_damage_id(src_id, bl_id, SK.HT_BLITZBEAT, math.min(checkskill(sd, SK.HT_BLITZBEAT), lv), { auto = true })
    end
    -- Gank steals as you hit (not with a bow: 817)
    if sd and sd.st.weapon ~= 11 and checkskill(sd, SK.RG_SNATCHER) > 0 then
      if checkskill(sd, SK.RG_SNATCHER) * 15 + 55 + checkskill(sd, SK.TF_STEAL) * 10 > rnd(1000) then
        if pc_steal_item(sd, bl) then _rt("clif_skill_nodamage", src_id, bl_id, SK.TF_STEAL, checkskill(sd, SK.TF_STEAL), 1) end
      end
    end
    -- Enchant Poison: the status carries a "chance to poison" (skill.c:3802,
    -- 3% to 7%) that neither mod0631 nor 817 ever rolls. Rolled here on each hit.
    -- The poison lasts as Envenom's of the same level does (ours).
    local ep = sc(src, SC.ENCPOISON)
    if ep and not undead and rnd(100) < floor(ep.val2 * sc_def_vit / 100) then
      start(SC.POISON, ep.val1, skill_get_time2(SK.TF_POISON, ep.val1))
    end
  elseif skillid == SK.SM_BASH then
    -- Fatal Blow: Bash from level 6 stuns
    if sd and checkskill(sd, SK.SM_FATALBLOW) > 0 and rnd(100) < floor(6 * (skilllv - 5) * sc_def_vit / 100) then
      start(SC.STAN, skilllv, time2(SK.SM_FATALBLOW))
    end
  elseif skillid == SK.TF_POISON then
    if not undead and rnd(100) < floor((2 * skilllv + 10) * sc_def_vit / 100) then start(SC.POISON, skilllv, time2()) end
  elseif skillid == SK.AS_SPLASHER then
    -- the burst poisons (the client's text, "Make the enemy contagious by Poison"): Envenom's
    -- chance, 15 s to 60 s (jAthena //1046 skill.c:484 and its skill_cast_db time2)
    if not undead and rnd(100) < floor((2 * skilllv + 10) * sc_def_vit / 100) then start(SC.POISON, skilllv, 10000 + 5000 * skilllv) end
  elseif skillid == SK.AS_SONICBLOW then
    if rnd(100) < floor((2 * skilllv + 10) * sc_def_vit / 100) then start(SC.STAN, skilllv, time2()) end
  elseif skillid == SK.MG_FROSTDIVER or skillid == SK.WZ_FROSTNOVA or skillid == SK.HT_FREEZINGTRAP then
    -- Frost Diver freezes 3 s a level, less MDEF: the iRO guide's Mage page (2004-04-06, "3sec" to
    -- "30sec"), mod0631's own length (skill.c:3869), an iRO player's "around 20-30 seconds" at
    -- level 10 (GameFAQs 25510, 2003-09). 817's cast_db (6 to 15 s) for the other two
    local tick = skillid == SK.MG_FROSTDIVER and 3000 * skilllv or time2()
    if not undead and rnd(100) < floor((skilllv * 3 + 35) * sc_def_mdef / 100) then start(SC.FREEZE, skilllv, tick) end
  elseif skillid == SK.WZ_STORMGUST then
    -- 817: the third hit of a Storm Gust freezes
    local n = storm_gust_hit(bl_id)
    if not undead and n >= 3 then
      storm_gust_hit(bl_id, 0)
      start(SC.FREEZE, skilllv, time2())
    end
  elseif skillid == SK.HT_LANDMINE then
    if rnd(100) < floor((5 * skilllv + 30) * sc_def_vit / 100) then start(SC.STAN, skilllv, time2()) end
  elseif skillid == SK.HT_ANKLESNARE then
    local sec = time2()
    if (battle.mode(bl) & 0x20) ~= 0 then sec = floor(sec / 5) end
    _rt("stop_walking", bl_id)
    start(SC.ANKLE, skilllv, sec)
  elseif skillid == SK.HT_SANDMAN then
    if rnd(100) < floor((5 * skilllv + 30) * sc_def_int / 100) then start(SC.SLEEP, skilllv, time2()) end
  elseif skillid == SK.HT_FLASHER then
    -- Both emulators place this trap and give it no effect. The client's skill
    -- text: "Blinds an Enemy". The chance is ours (Sandman's); the time 817's cast_db.
    if rnd(100) < floor((5 * skilllv + 30) * sc_def_int / 100) then start(SC.BLIND, skilllv, time2()) end
  elseif skillid == SK.HT_SHOCKWAVE then
    -- No effect in either emulator. The client's skill text: "SP 20% / 35% / 50% /
    -- 65% / 80% Consume" (monsters have no SP here)
    if bl.kind == "player" then
      _rt("bl_set", bl_id, "sp", floor(bl.char.sp * (100 - (5 + 15 * skilllv)) / 100))
    end
  elseif skillid == SK.TF_SPRINKLESAND then
    if rnd(100) < floor(15 * sc_def_int / 100) then start(SC.BLIND, skilllv, time2()) end
  elseif skillid == SK.TF_THROWSTONE then
    if rnd(100) < floor(5 * sc_def_vit / 100) then start(SC.STAN, skilllv, time2()) end
  elseif skillid == SK.CR_HOLYCROSS then
    if rnd(100) < floor(3 * skilllv * sc_def_int / 100) then start(SC.BLIND, skilllv, time2()) end
  elseif skillid == SK.CR_GRANDCROSS then
    -- the undead and demons may go blind (eAthena 817 skill.c:487), "except boss monster"
    -- (euRO's skill text of May 2004); not the caster struck by his own cross
    local race = battle.race(bl)
    if bl_id ~= src_id and (undead or race == 6) and (battle.mode(bl) & 0x20) == 0 and rnd(100) < sc_def_int then
      start(SC.BLIND, skilllv, time2())
    end
  elseif skillid == SK.CR_SHIELDCHARGE then
    if rnd(100) < floor((15 + skilllv * 5) * sc_def_vit / 100) then start(SC.STAN, skilllv, time2()) end
  elseif skillid == SK.RG_RAID then
    if rnd(100) < floor((10 + 3 * skilllv) * sc_def_vit / 100) then start(SC.STAN, skilllv, time2()) end
    if rnd(100) < floor((10 + 3 * skilllv) * sc_def_int / 100) then start(SC.BLIND, skilllv, time2()) end
  elseif skillid == SK.BA_FROSTJOKE then
    -- 20% to 40%: the iRO guide's Bard page (2004-03-27), jAthena //1020 skill.c:565 the same;
    -- frozen for jAthena's cast_db time2, 10 to 14 s, less MDEF
    if not undead and rnd(100) < floor((15 + 5 * skilllv) * sc_def_mdef / 100) then start(SC.FREEZE, skilllv, time2()) end
  elseif skillid == SK.DC_SCREAM then
    -- 30% to 50% (jAthena //1020 skill.c:570; the guide's Dancer page names no number)
    if rnd(100) < floor((25 + 5 * skilllv) * sc_def_vit / 100) then start(SC.STAN, skilllv, time2()) end
  elseif skillid == SK.BD_LULLABY then
    -- 15% less the target's INT resistance, asleep for its time2, 15 s (jAthena //1020
    -- skill.c:575 and cast_db). The guide: the chance "is affected by INT of the caster", no number
    if rnd(100) < floor(15 * sc_def_int / 100) then start(SC.SLEEP, skilllv, time2()) end
  elseif skillid == SK.NPC_PETRIFYATTACK then
    if rnd(100) < sc_def_mdef then start(SC.STONE, skilllv, time2()) end
  elseif skillid == SK.NPC_POISON then
    if rnd(100) < sc_def_vit then start(SC.POISON, skilllv, time2()) end
  elseif skillid == SK.NPC_SILENCEATTACK then
    if rnd(100) < sc_def_vit then start(SC.SILENCE, skilllv, time2()) end
  elseif skillid == SK.NPC_STUNATTACK then
    if rnd(100) < sc_def_vit then start(SC.STAN, skilllv, time2()) end
  elseif skillid == SK.NPC_CURSEATTACK then
    if rnd(100) < sc_def_luk then start(SC.CURSE, skilllv, time2()) end
  elseif skillid == SK.NPC_SLEEPATTACK then
    if rnd(100) < sc_def_int then start(SC.SLEEP, skilllv, time2()) end
  elseif skillid == SK.NPC_BLINDATTACK then
    if rnd(100) < sc_def_int then start(SC.BLIND, skilllv, time2()) end
  end

  -- 817: cards that add an ailment to weapon hits (bAddEff is in 1/100 %)
  if sd and skillid ~= SK.MC_CARTREVOLUTION and ((attack_type or BF.WEAPON) & BF.WEAPON) ~= 0 then
    local card_skill = { [SC.STONE] = SK.MG_STONECURSE, [SC.FREEZE] = SK.MG_FROSTDIVER, [SC.STAN] = SK.NPC_STUNATTACK,
                         [SC.SLEEP] = SK.NPC_SLEEPATTACK, [SC.POISON] = SK.TF_POISON, [SC.CURSE] = SK.NPC_CURSEATTACK,
                         [SC.SILENCE] = SK.NPC_SILENCEATTACK, [SC.BLIND] = SK.NPC_BLINDATTACK }
    for i = SC.STONE, SC.BLIND do
      local rate = sd.st.addeff[i - SC.STONE] or 0
      if rate > 0 then
        local def = sc_def_int
        if i == SC.STONE or i == SC.FREEZE then def = sc_def_mdef
        elseif i == SC.STAN or i == SC.POISON or i == SC.SILENCE then def = sc_def_vit
        elseif i == SC.CURSE then def = sc_def_luk end
        -- mod0631's item scripts give these in percent, 817's in 1/100 %: the data here is mod0631's
        if rnd(100) < floor(rate * def / 100) then
          start(i, 7, i == SC.CONFUSION and 17000 or skill_get_time2(card_skill[i], 7))
        end
      end
    end
  end
end

-- 817 counts Storm Gust hits per target (sg_count)
local sg_count = {}
function storm_gust_hit(id, set)
  if set then sg_count[id] = set; return set end
  sg_count[id] = (sg_count[id] or 0) + 1
  return sg_count[id]
end

-- ------------------------------------------------------------ knock-back

-- skill.c:424 skill_blown: `count` cells away from `src` (or, with fixed =
-- true, opposite to the way the target faces). `from` may be a cell {x=, y=}.
function skill_blown(from, target_id, count, fixed)
  local target = get(target_id)
  if not target then return end
  local dx, dy = 0, 0
  if not fixed and from then
    dx = target.x - from.x; dx = dx > 0 and 1 or (dx < 0 and -1 or 0)
    dy = target.y - from.y; dy = dy > 0 and 1 or (dy < 0 and -1 or 0)
  end
  if dx == 0 and dy == 0 then
    local dirx = { [0] = 0, -1, -1, -1, 0, 1, 1, 1 }
    local diry = { [0] = 1, 1, 0, -1, -1, -1, 0, 1 }
    dx, dy = -dirx[target.dir & 7], -diry[target.dir & 7]
  end
  local x0, y0 = target.x, target.y
  _rt("blow", target_id, dx, dy, count)
  local after = get(target_id)
  if after and sc(after, SC.DANCING) then skill_dance_thrown(target_id, after.x - x0, after.y - y0) end
  skill_unit_move(target_id) -- ground skills notice (skill.c:495)
end

-- ------------------------------------------------------------ skill_attack

-- Poison React, a draft. Neither mod0631 nor eAthena 817 has more than the
-- status; the client's skill text: "Enable to Counter for one time when the
-- Selected player hit by a Poisonous attack". Ours (guess): a weapon blow of
-- the Poison element that lands is answered once with (100 + 30 x level)% (battle.lua)
-- if the attacker is within the weapon's reach plus one cell, as Auto
-- Counter measures it; then the status ends. THREADS.md #22.
function skill_poison_react(src_id, bl_id, ele)
  if ele ~= 5 then return end
  local bl = get(bl_id)
  local pr = bl and not bl.dead and sc(bl, SC.POISONREACT)
  if not pr then return end
  local src = get(src_id)
  if not src or src.dead or src.map ~= bl.map then return end
  local reach = bl.kind == "player" and bl.st.attackrange or bl.db.range
  if battle.distance(src, bl) > reach + 1 then return end
  skill_status_change_end(bl_id, SC.POISONREACT)
  skill_attack(BF.WEAPON, bl_id, bl_id, src_id, SK.AS_POISONREACT, pr.val1, 0)
end

-- skill.c:527. `dsrc` is what the client sees the hit come from (the caster,
-- or a ground cell {id=, x=, y=}). opt: wflag = number handed to the damage
-- formula (targets sharing, distance), type = damage packet type, lv = level
-- shown. Returns the damage done.
function skill_attack(attack_type, src_id, dsrc, bl_id, skillid, skilllv, wflag, opt)
  opt = opt or {}
  local src, bl = get(src_id), get(bl_id)
  if not src or not bl or src.dead or bl.dead then return 0 end
  local dsrc_bl = type(dsrc) == "table" and dsrc or (dsrc == src_id and src or get(dsrc))
  if not dsrc_bl then return 0 end
  -- Storm Gust passes over the frozen (817 skill.c:715; jAthena //0738, 2004-01-24, "the real
  -- server's way", with the third hit's freeze). mod0631 hits them
  if skillid == SK.WZ_STORMGUST and battle.sc(bl, SC.FREEZE) then return 0 end
  -- The Maya Card turns a spell back (the client's text: "Enable to reflect Magical Damages
  -- back( Success Chance 30%)"; no period emulator has it, 817 nullified the spell instead).
  -- Ours: a spell aimed at the wearer, not a ground spell's cell, comes back on its caster as
  -- it would have struck the wearer, drawn from the wearer, who is credited with the damage.
  local credit = src_id
  if attack_type == BF.MAGIC and bl.kind == "player" and (bl.st.magic_return or 0) > 0 and src_id ~= bl_id
    and dsrc_bl.id == src_id and (skill_get_inf(skillid) & 2) == 0 and rnd(100) < bl.st.magic_return then
    credit, dsrc_bl, bl, bl_id = bl_id, bl, src, src_id
  end
  -- Magic Rod: a spell cast at the holder (not a ground spell's cell) does nothing, and the
  -- holder takes in a share of its SP, 20% to 100% (jAthena //0868, 2004-03-06, skill.c:732;
  -- the iRO guide's Sage page; not in 817)
  local rod = attack_type == BF.MAGIC and dsrc_bl.id == src_id and battle.sc(bl, SC.MAGICROD)
  if rod then
    if bl.kind == "player" then
      local sp = math.max(1, floor(skill_get_sp(skillid, skilllv) * rod.val2 / 100))
      battle.heal(bl_id, 0, sp)
    end
    _rt("clif_skill_nodamage", bl_id, bl_id, SK.SA_MAGICROD, rod.val1, 1)
    return 0
  end
  local dmg = battle.calc_attack(attack_type, src, bl, skillid, skilllv, wflag or 0)
  if bl.wall then
    -- an Ice Wall cell: the blow comes off its durability; nothing else happens to it
    if opt.splash then dmg.damage = floor(dmg.damage * opt.splash / 100) end
    local total = dmg.damage + math.max(0, dmg.damage2)
    icewall_damage(bl_id, total)
    return total
  end
  local lv = opt.lv or skilllv
  local ptype = opt.type or -1
  if opt.splash then
    dmg.damage = floor(dmg.damage * opt.splash / 100)
    if dmg.damage2 > 0 then dmg.damage2 = floor(dmg.damage2 * opt.splash / 100) end
  end
  if dmg.damage <= 0 or dmg.damage < dmg.div then dmg.blewcount = 0 end
  if dmg.blewcount ~= 0 then
    -- 0x10000: thrown back from the way it faces, not away from the source
    skill_blown(dsrc_bl, bl_id, dmg.blewcount & 0xffff, (dmg.blewcount & 0x10000) ~= 0)
    _rt("clif_skill_damage2", dsrc_bl.id, bl_id, dmg.amotion, dmg.dmotion, dmg.damage, dmg.div, skillid, lv, ptype)
  else
    _rt("clif_skill_damage", dsrc_bl.id, bl_id, dmg.amotion, dmg.dmotion, dmg.damage, dmg.div, skillid, lv, ptype)
  end
  local total = dmg.damage + math.max(0, dmg.damage2)
  battle.damage(credit, bl_id, total)
  if attack_type == BF.WEAPON then battle.reflect(src, bl, total, dmg.flag or 0) end
  if total > 0 then skill_additional_effect(src_id, bl_id, skillid, skilllv, attack_type) end
  -- Poison React answers a blow that lands ("hit by a Poisonous attack"), not a miss
  if attack_type == BF.WEAPON and total > 0 and skillid ~= SK.AS_POISONREACT then
    local pl = skill_get_pl(skillid)
    skill_poison_react(src_id, bl_id, pl > 0 and pl or battle.attack_element(src))
  end
  -- a monster may answer a skill with one of its own (skill.c:573)
  if bl.kind == "mob" and src_id ~= bl_id then _rt("mob_event", bl_id, "skillused:" .. skillid) end
  return total
end

-- Everyone in a square around (x, y) the skill may touch, as fresh snapshots.
-- flag: BCT.ENEMY / NOENEMY / ALL / PARTY, judged from `src`.
-- `walls`: Ice Wall cells too, for searches that lead to a blow (skill_units.lua icewall_bl)
function skill_area(src, map, x, y, range, flag, walls)
  local out = {}
  for _, id in ipairs(_rt("in_area", map, x - range, y - range, x + range, y + range)) do
    local bl = get(id)
    if bl and not bl.dead and battle.check_target(src, bl, flag) then out[#out + 1] = bl end
  end
  if walls and flag == BCT.ENEMY then
    for _, w in ipairs(icewall_targets(map, x - range, y - range, x + range, y + range)) do out[#out + 1] = w end
  end
  return out
end

-- ------------------------------------------------------------ stealing

-- pc.c:1815 pc_steal_item: one try at each of the monster's drops in turn
-- (no cards, no MVPs), a monster gives only once.
local stolen = {}
function pc_steal_item(sd, bl)
  if bl.kind ~= "mob" or stolen[bl.id] or bl.db.mvp_exp > 0 then return false end
  local skill = checkskill(sd, SK.TF_STEAL) * 50
  for _, d in ipairs(bl.db.drops or {}) do
    if d.item > 0 then
      local t = _rt("item_db", d.item)
      if t and t.type ~= 6 then
        local rate = floor(d.rate * (sd.char.base_level * 4 + sd.st.dex * 3 + skill) / 1000)
        if rnd(10000) < rate then
          stolen[bl.id] = true
          _rt("pc_item_add", sd.id, d.item, 1)
          return true
        end
      end
    end
  end
  return false
end

-- pc.c:1850 pc_steal_coin
function pc_steal_coin(sd, bl)
  if bl.kind ~= "mob" or stolen["z" .. bl.id] then return false end
  local rate = checkskill(sd, SK.RG_STEALCOIN) * 10 + (sd.char.base_level - bl.db.level) * 3 + sd.st.dex * 2 + sd.st.luk * 2
  if rnd(1000) < rate then
    stolen["z" .. bl.id] = true
    _rt("bl_set", sd.id, "zeny", sd.char.zeny + bl.db.level * 10 + rnd(100))
    return true
  end
  return false
end

function rules.unit_gone(a)
  stolen[a.bl], stolen["z" .. a.bl], sg_count[a.bl] = nil, nil, nil
  skill_unit_owner_gone(a.bl)
end

-- ------------------------------------------------------------ cast time, delay

-- skill.c:3165 skill_castfix: DEX shortens the cast (none at 150), then
-- equipment (castrate) and Suffragium, which is used up.
function skill_castfix(bl, time)
  if time == 0 then return 0 end
  local castrate = bl.kind == "player" and bl.st.castrate or 100
  time = floor(time * castrate * (150 - battle.dex(bl)) / 15000)
  time = floor(time * battle_config.cast_rate / 100)
  local t = sc(bl, SC.SUFFRAGIUM)
  if t then
    time = floor(time * (100 - t.val1 * 15) / 100)
    skill_status_change_end(bl.id, SC.SUFFRAGIUM)
  end
  -- the Poem of Bragi (jAthena //1020 skill.c:5498)
  t = sc(bl, SC.POEMBRAGI)
  if t then time = floor(time * (100 - song_effect(SC.POEMBRAGI, t)) / 100) end
  return math.max(0, time)
end

-- skill.c:3189 skill_delayfix
function skill_delayfix(bl, time)
  if time <= 0 then return 0 end
  if battle_config.delay_dependon_dex then time = floor(time * (150 - battle.dex(bl)) / 150) end
  time = floor(time * battle_config.delay_rate / 100)
  -- the Poem of Bragi (jAthena //1020 skill.c:5525)
  local t = sc(bl, SC.POEMBRAGI)
  if t then
    local _, less = song_effect(SC.POEMBRAGI, t)
    time = floor(time * (100 - less) / 100)
  end
  return math.max(0, time)
end

-- Skills usable while hidden / cloaked (skill.c:3225)
local function hidden_ok(sd, skill)
  if (sd.option & 4) ~= 0 and skill == SK.AS_CLOAKING then return true end
  if (sd.option & 2) ~= 0 and (skill == SK.TF_HIDING or skill == SK.AS_GRIMTOOTH or skill == SK.RG_BACKSTAP or skill == SK.RG_RAID) then
    return true
  end
  return false
end

-- Weapon range for skills whose range is the weapon's (range < 0 in skill_db).
-- The falcon's reach grows with the level: Detect 3, 5, 7, 9 cells, Spring
-- Trap 4 to 8 (the iRO guide's Hunter page, 2004-03-27; neither emulator
-- has either skill, and their skill_db gives 8 for both).
local LEVEL_RANGE = {
  [SK.HT_DETECTING] = function(lv) return 1 + 2 * lv end,
  [SK.HT_SPRINGTRAP] = function(lv) return 3 + lv end,
  -- 3, 5, 7, 9, 11 cells: eAthena 817's skill_db and the iRO guide (2004-04-11). mod0631's: 6 at every level
  [SK.CR_SHIELDBOOMERANG] = function(lv) return 1 + 2 * lv end,
}
-- Cast times mod0631's cast_db lacks, from eAthena 817's (2004-02-19). Providence's three
-- seconds are also the iRO guide's (2004-04-11: "3 second casting delay")
local CAST_817 = {
  [SK.CR_PROVIDENCE] = 3000,
  [SK.SA_FLAMELAUNCHER] = 3000, [SK.SA_FROSTWEAPON] = 3000, [SK.SA_LIGHTNINGLOADER] = 3000, [SK.SA_SEISMICWEAPON] = 3000,
  [SK.SA_DISPELL] = 2000,
  [SK.CR_GRANDCROSS] = 3000, -- also the iRO guide's (2004-04-11): "a 3 second casting delay"
  [SK.SA_SPELLBREAKER] = 700,
  [SK.SA_VOLCANO] = 5000, [SK.SA_DELUGE] = 5000, [SK.SA_VIOLENTGALE] = 5000, [SK.SA_LANDPROTECTOR] = 5000,
  -- jAthena //0969's cast_db (2004-05-13): neither period emulator's has them
  [SK.AM_ACIDTERROR] = 1000, [SK.AM_DEMONSTRATION] = 1000,
  -- Mental Strength: "a 5 second casting time which cannot be interrupted" (the iRO guide's
  -- Monk page, 2004-03-27). mod0631, 817 and jAthena //1020 cast it at once
  [SK.MO_STEELBODY] = 5000,
}
local function skill_range(sd, skill, lv)
  if lv and LEVEL_RANGE[skill] then return LEVEL_RANGE[skill](lv) end
  local r = skill_get_range(skill)
  if r < 0 then return math.max(1, sd.st and sd.st.attackrange or 1) - (r + 1) end
  return r
end

-- The client asks to use a skill (skill_use_id / skill_use_pos). Returns
-- {cast = ms, delay = ms, cancelable =, forcecast =, target =} or nil.
function rules.skill_use(a)
  local sd = get(a.src)
  if not sd or sd.dead or sd.kind ~= "player" then return nil end
  local skill, lv = a.skill, a.lv
  local inf = skill_get_inf(skill)
  local target_id = a.target
  -- self skills arrive with the caster as target; place skills with a cell
  local place = (inf & 2) ~= 0 and (not target_id or target_id == 0)
  local bl
  if not place then
    if (inf & 4) ~= 0 then target_id = sd.id end
    -- aimed at a trap (Remove Trap, Spring Trap): a cell, not a unit that fights
    bl = inf == 32 and skill_trap_cell(target_id) or get(target_id)
    if not bl or bl.map ~= sd.map then return nil end
    if inf == 32 then bl = { x = bl.x, y = bl.y, kind = "trap" } end
  end

  -- stunned, frozen, asleep, stone; hidden; silenced
  if sd.opt1 > 0 or (sd.option & 6) ~= 0 or sc(sd, SC.DIVINA) then
    if not hidden_ok(sd, skill) or sd.opt1 > 0 then return nil end
  end
  if skill == SK.HT_BLITZBEAT and not sd.st.falcon then return nil end
  -- in Auto Counter's stance, no other skill (817 skill.c:3851)
  if sc(sd, SC.AUTOCOUNTER) and skill ~= SK.KN_AUTOCOUNTER then
    _rt("clif_skill_fail", sd.id, skill, 0, 0)
    return nil
  end
  -- in Loki's Veil, no skill at all (jAthena //1020 skill.c:5189; the guide: "all skills and
  -- magic are not allowed")
  if sc(sd, SC.ROKISWEIL) then
    _rt("clif_skill_fail", sd.id, skill, 0, 0)
    return nil
  end
  -- playing a song, only Adaptation, Musical Strike and Slinging Arrow (jAthena //1020
  -- skill.c:5592; the guide: "This skill is also usable while playing music")
  if sc(sd, SC.DANCING) and skill ~= SK.BD_ADAPTATION and skill ~= SK.BA_MUSICALSTRIKE and skill ~= SK.DC_THROWARROW then
    _rt("clif_skill_fail", sd.id, skill, 0, 0)
    return nil
  end

  -- held in Blade Stop, the Monk may use only what its level allows; the one held, nothing
  if not blade_stop_allows(sd, skill) then
    _rt("clif_skill_fail", sd.id, skill, 0, 0)
    return nil
  end
  -- the combo skills strike the one being fought, in the window the blow before left
  -- (jAthena //1020 skill.c:5695-5714; they are self skills in skill_db); Chain Combo may
  -- also strike the one Blade Stop holds, and Guillotine Fist follows Combo Finish at once
  local in_combo = false
  if COMBO_AFTER[skill] or skill == SK.MO_EXTREMITYFIST then
    local held = skill == SK.MO_CHAINCOMBO and blade_stop_partner(sd)
    if held then
      target_id, bl = held.id, held
    elseif monk_in_combo(sd, COMBO_AFTER[skill] or SK.MO_COMBOFINISH) then
      in_combo = true
      local t = sd.attacking and get(sd.attacking)
      if not t or t.map ~= sd.map then return nil end
      target_id, bl = t.id, t
    elseif COMBO_AFTER[skill] then
      return nil -- out of the window: nothing (skill_check_condition returns 0 there)
    end
  end

  -- range
  local tx, ty = a.x, a.y
  if bl then tx, ty = bl.x, bl.y end
  -- One cell more than the skill's range: the client asks from there (it did
  -- for Remove Trap at range 2 from 3 cells, 2026-11-13) and mod0631 refused
  -- it without a word (skill.c:3237, 3360). jAthena //1082 (2004-12-18) let it
  -- through: "a skill used one cell beyond its range does nothing" was its bug.
  local range = skill_range(sd, skill, a.lv) + monk_range_bonus(sd, skill)
  if not battle.check_range(sd, tx, ty, range > 0 and range + 1 or range) then return nil end

  local casttime = skill_castfix(sd, CAST_817[skill] or skill_get_cast(skill, lv))
  local delay = skill_delayfix(sd, skill_get_delay(skill, lv))
  local cancelable, forcecast = true, false
  -- Grand Cross is aimed at the caster's own cell (a ground skill in mod0631's skill_db)
  if skill == SK.CR_GRANDCROSS then cancelable = false end

  if bl then
    if skill == SK.AL_HEAL then
      if battle.elem_type(bl) == 9 then forcecast = true end -- Heal as an attack shows the cast
    elseif skill == SK.ALL_RESURRECTION then
      if battle.elem_type(bl) == 9 then
        forcecast = true
        casttime = skill_castfix(sd, skill_get_cast(SK.PR_TURNUNDEAD, lv))
      end
    elseif skill == SK.KN_BRANDISHSPEAR or skill == SK.KN_BOWLINGBASH or skill == SK.AC_CHARGEARROW
      or skill == SK.RG_STRIPWEAPON or skill == SK.RG_STRIPSHIELD or skill == SK.RG_STRIPARMOR
      or skill == SK.RG_STRIPHELM or skill == SK.CR_GRANDCROSS or skill == SK.MO_CALLSPIRITS
      or skill == SK.MO_INVESTIGATE or skill == SK.MO_STEELBODY then
      cancelable = false -- casts nothing interrupts
    end
  end
  if skill == SK.MO_FINGEROFFENSIVE then
    -- a second more for each sphere thrown (jAthena //1020 skill.c:5692; the iRO guide:
    -- "[1 second + (The numbers of spirit spheres x 1 second)]")
    casttime = casttime + casttime * math.min(lv, pc_spiritball(sd.id))
  elseif skill == SK.MO_EXTREMITYFIST then
    -- straight after Combo Finish, no cast (skill.c:5709; the guide: "can be activated
    -- without a casting delay after this skill"); a cast that shows either way
    if in_combo then casttime = 0 end
    forcecast = true
  end
  if a.item then casttime, delay = 0, 0 end -- a skill from an item: instant

  -- Cast Cancel: only while casting (jAthena //0969 skill_check_condition), and only with its
  -- own 2 SP to pay, as there the cast goes on when the condition fails. What is cancelled is
  -- kept for the SP it costs (val = skill * 100 + level)
  local val = 0
  if skill == SK.SA_CASTCANCEL then
    if not sd.casting or sd.char.sp < skill_get_sp(skill, lv) then
      _rt("clif_skill_fail", sd.id, skill, 0, 0)
      return nil
    end
    val = sd.casting * 100 + (sd.casting_lv or 1)
  end

  -- A cast-sensing monster turns on whoever casts at it (skill.c:3308;
  -- there: unless it is attacking; here: unless it has a target). mod0631
  -- defines a "casttargeted" skill condition and never raises it; raised here.
  if bl and (casttime > 0 or forcecast) and bl.kind == "mob" and (bl.db.mode & 0x10) ~= 0 and not bl.target then
    _rt("mob_target", bl.id, sd.id, 0)
    _rt("mob_event", bl.id, "casttargeted")
  end
  return { cast = casttime, delay = delay, cancelable = cancelable, forcecast = forcecast, target = place and 0 or target_id,
           val = val }
end

-- ------------------------------------------------------------ what a skill costs

local GEMS = { [715] = true, [716] = true, [717] = true }

-- the weapon each Bard and Dancer skill is played with: 13 an instrument, 14 a whip
BARD_WEAPON = {}
for _, n in ipairs({ "BA_MUSICALSTRIKE", "BA_DISSONANCE", "BA_WHISTLE", "BA_ASSASSINCROSS", "BA_POEMBRAGI", "BA_APPLEIDUN" }) do
  BARD_WEAPON[SK[n]] = 13
end
for _, n in ipairs({ "DC_THROWARROW", "DC_UGLYDANCE", "DC_HUMMING", "DC_DONTFORGETME", "DC_FORTUNEKISS", "DC_SERVICEFORYOU" }) do
  BARD_WEAPON[SK[n]] = 14
end
-- the ensembles with either (jAthena //1020 skill_require_db: 13:14)
for _, n in ipairs({ "BD_LULLABY", "BD_RICHMANKIM", "BD_ETERNALCHAOS", "BD_DRUMBATTLEFIELD", "BD_RINGNIBELUNGEN",
                     "BD_ROKISWEIL", "BD_INTOABYSS", "BD_SIEGFRIED" }) do
  BARD_WEAPON[SK[n]] = 0
end
BARD_WEAPON[-1] = nil
-- the last song each player played, for Encore (jAthena's sd->skillid_dance; not kept over a logout)
last_dance = last_dance or {}

-- skill.c:2825 skill_check_condition. Takes the SP, items and zeny when the
-- skill may be used; otherwise tells the client why not. Returns true/false.
function skill_check_condition(sd, skill, lv, a)
  local id = sd.id
  local function fail(type) _rt("clif_skill_fail", id, skill, type or 0, 0); return false end
  if sd.opt1 > 0 then return fail() end
  if a.item then return true end -- from an item: no conditions
  if sc(sd, SC.DIVINA) or sc(sd, SC.STEELBODY) then return fail() end

  local sp = skill_get_sp(skill, lv)
  if sd.st.dsprate ~= 100 then sp = floor(sp * sd.st.dsprate / 100) end
  local zeny, hpcost = 0, 0
  local items = {}
  local w = sd.st.weapon
  -- Deluge lets Water Ball and Aqua Benedicta be cast on dry land (the iRO guide's Sage page,
  -- euRO's text of May 2004: "you can use magics what available on water")
  local function water() return _rt("cell", sd.map, sd.x, sd.y) == 3 or sc(sd, SC.DELUGE) ~= nil end

  if skill == SK.MG_STONECURSE or skill == SK.AS_VENOMDUST then items[716] = 1 -- Red Gemstone
  elseif skill == SK.MG_SAFETYWALL or skill == SK.AL_WARP or skill == SK.ALL_RESURRECTION
    or skill == SK.PR_SANCTUARY or skill == SK.PR_MAGNUS or skill == SK.WZ_FIREPILLAR then items[717] = 1 -- Blue Gemstone
  elseif skill == SK.SA_VOLCANO or skill == SK.SA_DELUGE or skill == SK.SA_VIOLENTGALE then items[715] = 1
  elseif skill == SK.SA_LANDPROTECTOR then
    -- a Blue Gemstone too: the iRO guide's Sage page (2004-03-27), euRO's text of May 2004,
    -- jAthena //0969's skill_require_db (mod0631 the yellow one alone)
    items[715] = 1; items[717] = 1
  elseif skill == SK.SA_DISPELL then
    -- a Yellow and a Red Gemstone: eAthena 817's skill_require_db and the iRO guide's Sage
    -- page (2004-03-27). mod0631 takes the yellow one only (skill.c:2863)
    items[715] = 1; items[716] = 1
  elseif skill == SK.AM_CP_WEAPON or skill == SK.AM_CP_SHIELD or skill == SK.AM_CP_ARMOR or skill == SK.AM_CP_HELM then
    items[7139] = 1 -- the guide's "Glistening Coat"; 817's skill_require_db
  elseif skill == SK.CR_GRANDCROSS then
    -- a fifth of the HP the caster has: euRO's skill text of May 2004 ("Consume 20% of remained
    -- HP"), jAthena //0969's skill_require_db (hp_rate 20), with any weapon but a bow
    if w == 11 then return fail(6) end
    hpcost = floor(sd.char.hp * 20 / 100)
  elseif skill == SK.AM_ACIDTERROR then items[7136] = 1 -- Acid Bottle; the iRO guide's Alchemist page
  elseif skill == SK.AM_DEMONSTRATION then items[7135] = 1 -- Bottle Grenade (Fire Bottle), jAthena //0969's skill_db
  elseif skill == SK.AM_CANNIBALIZE then
    -- a Plant Bottle: jAthena //0969's skill_require_db, the iRO guide (2004-03-27), euRO's text of
    -- May 2004 ("MenEater Plant Bottle"); and no more than three Floras, as that text says (the
    -- emulator and the guide set no limit)
    items[7137] = 1
    local n = 0
    for _, id in ipairs(_rt("in_area", sd.map, 0, 0, 1000, 1000)) do
      local m = get(id)
      if m and m.kind == "mob" and m.summoner == sd.id and m.summon == "flora" then n = n + 1 end
    end
    if n >= 3 then return fail() end
  elseif skill == SK.AM_SPHEREMINE then items[7138] = 1 -- Mine Bottle: jAthena //0969, euRO's text of May 2004
  elseif skill == SK.AM_POTIONPITCHER then
    -- the potion by level (skill_effects.lua AID_POTION)
    items[(AID_POTION[lv] or AID_POTION[1]).item] = 1
  elseif skill == SK.SA_FLAMELAUNCHER then items[990] = 1
  elseif skill == SK.SA_FROSTWEAPON then items[991] = 1
  elseif skill == SK.SA_LIGHTNINGLOADER then items[992] = 1
  elseif skill == SK.SA_SEISMICWEAPON then items[993] = 1
  elseif skill == SK.HT_TALKIEBOX or skill == SK.HT_BLASTMINE or skill == SK.HT_SKIDTRAP or skill == SK.HT_ANKLESNARE
    or skill == SK.HT_LANDMINE or skill == SK.HT_SHOCKWAVE or skill == SK.HT_SANDMAN or skill == SK.HT_FLASHER
    or skill == SK.HT_FREEZINGTRAP or skill == SK.HT_CLAYMORETRAP then items[1065] = 1 -- Trap
  elseif skill == SK.AL_HOLYWATER then
    if not water() then return fail() end
    items[713] = 1 -- Empty Bottle
  elseif skill == SK.PR_ASPERSIO then items[523] = 1 -- Holy Water
  elseif skill == SK.WZ_WATERBALL then
    if not water() then return fail() end
  elseif skill == SK.WZ_SIGHTRASHER then
    if not sc(sd, SC.SIGHT) then return fail() end -- 817 skill.c:3990 (ST_SIGHT)
  elseif skill == SK.AS_SPLASHER then
    -- only on a poisoned enemy (the client's skill text; jAthena //1046 skill.c:5918)
    -- whose HP is down to a third: venom_splasher_ready (skill_effects.lua)
    if not venom_splasher_ready(a.target and get(a.target)) then return fail() end
  elseif skill == SK.MC_MAMMONITE then
    zeny = lv * 100
    if sd.char.zeny < zeny then return fail(5) end
  elseif skill == SK.MC_VENDING then
    if not sd.st.cart then return fail() end
  elseif skill == SK.AC_DOUBLE or skill == SK.AC_SHOWER then
    if w ~= 11 then return fail(6) end
  elseif skill == SK.KN_BRANDISHSPEAR then
    if not sd.st.riding then return fail() end
  elseif skill == SK.KN_PIERCE or skill == SK.KN_SPEARSTAB or skill == SK.KN_SPEARBOOMERANG then
    if w ~= 4 and w ~= 5 then return fail(6) end
  elseif skill == SK.KN_TWOHANDQUICKEN then
    if w ~= 3 then return fail(6) end
  elseif skill == SK.BS_ADRENALINE or skill == SK.BS_HAMMERFALL then
    if w ~= 6 and w ~= 7 and w ~= 8 then return fail(6) end
  elseif skill == SK.BS_MAXIMIZE or skill == SK.NV_TRICKDEAD or skill == SK.TF_HIDING or skill == SK.AS_CLOAKING then
    if sc(sd, SkillStatusChangeTable[skill]) then sp = 0 end -- switching it off is free
  elseif skill == SK.CR_AUTOGUARD or skill == SK.CR_DEFENDER then
    -- eAthena 817 skill.c:3893: off again for nothing. The shield: the iRO guide's Crusader
    -- page (2004-04-11, "Requires a shield"); 817 only ends them when it comes off
    if sc(sd, SkillStatusChangeTable[skill]) then sp = 0 elseif not sd.st.shield then return fail() end
  elseif skill == SK.CR_REFLECTSHIELD or skill == SK.CR_SHIELDCHARGE or skill == SK.CR_SHIELDBOOMERANG then
    if not sd.st.shield then return fail() end
  elseif skill == SK.HT_BLITZBEAT or skill == SK.HT_SPRINGTRAP then
    -- Spring Trap: the client's text, "Manage a Falcon to Destroy a installed Trap"
    if not sd.st.falcon then return fail() end
  elseif skill == SK.AS_GRIMTOOTH then
    if (sd.option & 2) == 0 then return fail() end
    if w ~= 16 then return fail(6) end
  elseif skill == SK.AS_SONICBLOW then
    if w ~= 16 then return fail(6) end -- "using Katar" (the client's skill text); mod0631 does not check
  elseif skill == SK.CR_SPEARQUICKEN then
    if w ~= 4 and w ~= 5 then return fail(6) end
  elseif skill == SK.RG_RAID then
    if (sd.option & 2) == 0 then return fail() end
  elseif skill == SK.MC_CARTREVOLUTION then
    -- ours: no cart, no Cart Revolution ("Hit a target hard using cart", the client's skill text); mod0631 does not check
    if not sd.st.cart then return fail() end
  elseif BARD_WEAPON[skill] then
    -- an instrument for the Bard's, a whip for the Dancer's (jAthena //1020 skill_require_db,
    -- euRO's text of May 2004: "only when you putting on musical instrument")
    local want = BARD_WEAPON[skill]
    if (want == 0 and w ~= 13 and w ~= 14) or (want ~= 0 and w ~= want) then return fail(6) end
    -- Encore plays the last song again for half its SP (the guide, euRO's text); here, as its
    -- own cast, it pays the song's half and its own 1 SP
    if a.encore then sp = floor(sp / 2) end
  elseif skill == SK.BD_ENCORE then
    local last = last_dance[sd.id]
    if not last or (w ~= 13 and w ~= 14) then return fail() end
  elseif skill == SK.BD_ADAPTATION then
    -- only while playing, and not in a song's first 5 s (jAthena //0995 and //0996, upkeep_time2
    -- of its cast_db)
    local played = skill_dance_elapsed(id)
    if not played or played < 5000 then return fail() end
  elseif skill == SK.MO_CALLSPIRITS then
    -- as many spheres as the level, no more (jAthena //1020 skill.c:5244)
    if pc_spiritball(id) >= lv then return fail() end
  elseif COMBO_AFTER[skill] then
    -- only in the window the blow before left (skill.c:5263), or Chain Combo in Blade Stop
    local held = skill == SK.MO_CHAINCOMBO and blade_stop_partner(sd)
    if not held and not monk_in_combo(sd, COMBO_AFTER[skill]) then return false end
  elseif skill == SK.MO_EXTREMITYFIST then
    -- in Fury (skill_require_db: explosionspirits; the guide: "During the fury status")
    if not sc(sd, SC.EXPLOSIONSPIRITS) then return fail() end
  elseif skill == SK.MO_BODYRELOCATION then
    -- not while held fast (skill_require_db: move_enable)
    if sc(sd, SC.ANKLE) or sc(sd, SC.BLADESTOP) or sc(sd, SC.AUTOCOUNTER) then return fail() end
  end
  -- spirit spheres (monk.lua)
  local balls = monk_spheres_needed(sd, skill, lv)
  if balls > 0 and pc_spiritball(id) < balls then return fail() end

  if sp > 0 and sd.char.sp < sp then return fail(1) end
  -- (mod0631 takes arrows for plain shots only: the bow skills use none. THREADS.md #22)
  -- standing in Into the Abyss ("Power Cord"), no gemstones and no Traps (jAthena //1020
  -- skill.c:5429; the guide: "without required items such as gemstones or traps")
  if sc(sd, SC.INTOABYSS) then
    for item in pairs(items) do
      if GEMS[item] or item == 1065 then items[item] = nil end
    end
  end
  for item, n in pairs(items) do
    -- a Mistress Card (4132) casts without gemstones
    if not (GEMS[item] and battle.dcard(sd, 4132)) then
      if _rt("pc_item_count", id, item) < n then
        if item == 716 or item == 717 then return fail(6 + (item - 715)) end
        return fail()
      end
    end
  end
  -- (mod0631 bug: an item in the first inventory slot was not used up, skill.c:3140)
  for item, n in pairs(items) do
    if not (GEMS[item] and battle.dcard(sd, 4132)) then _rt("pc_item_take", id, item, n) end
  end
  if sp > 0 then _rt("bl_set", id, "sp", sd.char.sp - sp) end
  if balls > 0 then
    if skill == SK.MO_FINGEROFFENSIVE then finger_offensive_count[id] = balls end
    pc_delspiritball(id, balls)
  end
  if hpcost > 0 then _rt("bl_set", id, "hp", math.max(1, sd.char.hp - hpcost)) end
  if zeny > 0 then _rt("bl_set", id, "zeny", sd.char.zeny - zeny) end
  return true
end

-- ------------------------------------------------------------ the cast is over

-- skill_castend_id / skill_castend_pos, for players and monsters
-- (mob.c:1924 mobskill_castend_id). Returns false when the use failed.
function rules.skill_castend(a)
  local src = get(a.src)
  if not src or src.dead then return false end
  local skill, lv = a.skill, a.lv
  local sd = src.kind == "player" and src or nil
  local place = (skill_get_inf(skill) & 2) ~= 0 and (not a.target or a.target == 0)
  local bl
  local trap = skill_get_inf(skill) == 32
  if not place then
    bl = trap and skill_trap_cell(a.target) or get(a.target)
    if not bl or bl.map ~= src.map then return false end
  end
  -- another cell in the way (skill_units.lua; jAthena //0751 skill_castend_pos)
  if place and skill_reiteration_blocked(src, a.x, a.y, skill) then
    if sd then _rt("clif_skill_fail", sd.id, skill, 0, 0) end
    return false
  end
  local tx, ty = a.x, a.y
  if bl then tx, ty = bl.x, bl.y end
  local range = sd and skill_range(sd, skill, lv) + battle_config.pc_skill_add_range + monk_range_bonus(sd, skill) or skill_get_range(skill)
  -- the cell beyond the range that rules.skill_use lets a player ask from counts
  -- here too (ours: jAthena //1082 widened only the request, so a skill asked
  -- for from there would spend its SP and items and fail)
  if sd and range > 0 then range = range + 1 end
  -- Hallucination: range 0 in mod0631's and 817's skill_db, so no target was ever near
  -- enough; jAthena //0850 (2004-02-29), which built the skill, gives it 1
  if skill == SK.NPC_HALLUCINATION and range < 1 then range = 1 end
  local out = range < battle.distance(src, { x = tx, y = ty })
  -- the target walked off: with skill_out_range_consume the SP is spent anyway
  if sd then
    if out and not battle_config.skill_out_range_consume then
      _rt("clif_skill_fail", sd.id, skill, 0, 0)
      return false
    end
    if not skill_check_condition(sd, skill, lv, a) then return false end
    if out then
      _rt("clif_skill_fail", sd.id, skill, 0, 0)
      return false
    end
  elseif out then
    -- mob.c:1943, 1982: the monster's target (or cell) is out of the skill's range
    return false
  end

  if place then
    skill_castend_pos2(a.src, a.x, a.y, skill, lv, a)
    return true
  end
  if trap then
    skill_castend_trap(src, bl, skill, lv)
    return true
  end
  -- an attack skill needs an enemy (skill.c:1769)
  if (skill_get_inf(skill) & 1) ~= 0 and not battle.is_enemy(src, bl) then return false end
  local nk = skill_get_nk(skill)
  -- mod0631's skill_db calls Shield Charge and Shield Boomerang "no damage" (nk 1): it never
  -- built them. eAthena 817's has 2 and 0, and hits with both (skill.c:1202)
  if skill == SK.CR_SHIELDCHARGE or skill == SK.CR_SHIELDBOOMERANG then nk = 0 end
  -- mod0631's skill_db gives the monsters' element changes (161-169) nk 0, so mob.c:1949 sends
  -- them to skill_castend_damage_id, which has no case for them, and its support-side code
  -- (skill.c:1680) never runs. jAthena //0850 (2004-02-29) made them nk 1
  if skill >= SK.NPC_ATTRICHANGE and skill <= SK.NPC_CHANGETELEKINESIS then nk = 1 end
  -- Metamorphosis: nk 0 in mod0631's skill_db (it never built the skill); 817's has 1
  if skill == SK.NPC_METAMORPHOSIS then nk = 1 end
  -- Acid Terror: nk 1 in mod0631's skill_db (it never built the skill), 0 in jAthena //0969's
  if skill == SK.AM_ACIDTERROR then nk = 0 end
  if nk == 1 then
    -- Heal and Resurrection hurt whatever is of the Undead element (friend or not: skill.c:1780)
    if (skill == SK.AL_HEAL or skill == SK.ALL_RESURRECTION) and battle.elem_type(bl) == 9 then
      skill_castend_damage_id(a.src, a.target, skill, lv, a)
    else
      skill_castend_nodamage_id(a.src, a.target, skill, lv, a)
    end
  else
    skill_castend_damage_id(a.src, a.target, skill, lv, a)
  end
  return true
end

-- A skill an item lets the player use once (clif_item_skill, 0x0147).
function item_skill(id, lv)
  _rt("item_skill", id, lv or 1, skill_get(id).name, skill_get_inf(id), skill_get_sp(id, lv or 1), math.max(1, skill_get_range(id)))
end

-- ------------------------------------------------------------ hooks from the server

-- pc.c:2822: Auto Berserk provokes its owner below a quarter of the HP
function rules.pc_damaged(a)
  local sd = get(a.bl)
  if not sd or sd.dead then return end
  -- a blow of more than a quarter of the max HP stops a song (jAthena //1020 pc.c:4494, there
  -- before //0693 of 2004-01-09; "if you lose more then 25% of your HP in one hit you lose your
  -- song", Jath's Bard FAQ of 2004-09, and Dikiwinky's idRO guide of 2004-11)
  if (a.damage or 0) * 4 > sd.st.max_hp and sc(sd, SC.DANCING) then skill_stop_dancing(sd.id) end
  if sd.char.hp < (sd.st.max_hp >> 2) and checkskill(sd, SK.SM_AUTOBERSERK) > 0 then
    local t = sc(sd, SC.PROVOKE)
    if not t or t.val2 == 0 then skill_status_change_start(sd.id, SC.PROVOKE, 10, 1) end
  end
end

-- pc.c:2340 pc_checkallowskill: status changes that need a weapon end without it
function rules.equip_changed(a)
  local sd = get(a.bl)
  if not sd then return end
  local w = sd.st.weapon
  -- a song needs the instrument or the whip it is played with (jAthena //1020 pc.c:5817)
  if sc(sd, SC.DANCING) and w ~= 13 and w ~= 14 then skill_stop_dancing(sd.id) end
  if sc(sd, SC.TWOHANDQUICKEN) and w ~= 3 then skill_status_change_end(sd.id, SC.TWOHANDQUICKEN) end
  if sc(sd, SC.SPEARSQUICKEN) and w ~= 4 and w ~= 5 then skill_status_change_end(sd.id, SC.SPEARSQUICKEN) end
  if sc(sd, SC.ADRENALINE) and w ~= 6 and w ~= 7 and w ~= 8 then skill_status_change_end(sd.id, SC.ADRENALINE) end
  -- eAthena 817 pc.c:3150: Auto Guard and Defender end with the shield; Reflect Shield by the same rule
  if not sd.st.shield then
    for _, t in ipairs({ SC.AUTOGUARD, SC.DEFENDER, SC.REFLECTSHIELD }) do
      if sc(sd, t) then skill_status_change_end(sd.id, t) end
    end
  end
end

-- /memo (pc.c:2010 pc_memo): Warp Portal level 2 remembers one cell, 3 two,
-- 4 three; not on nomemo maps. Returns how many it may keep, or {fail = 1}.
function rules.memo(a)
  local sd = get(a.src)
  if not sd then return { fail = 1 } end
  local lv = checkskill(sd, SK.AL_WARP)
  if lv < 2 or _rt("map_flag", sd.map, "nomemo") then return { fail = 1 } end
  return lv - 1
end

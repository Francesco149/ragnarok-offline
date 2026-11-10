-- Character status: everything pc_calcstatus computes. Server-side
-- formulas: NOT in the client.
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/pc.c pc_calcstatus,
-- pc_bonus, pc_bonus2 (archived 2003-12-18; research/notes/emulators.md).
-- Line refs are to that file. Tables: db/size_fix.txt, db/refine_db.txt.
--
-- rules.status(c) gets (crates/ro-game/src/status.rs char_lua):
--   c.class, c.base_level, c.job_level, c.str .. c.luk, c.option,
--   c.job = {weight, hp_coef, sp_coef, aspd_base[17], job_bonus[]} (data/db/jobs.toml),
--   c.equip.items = {{id, type, atk, def, loc, refine, wlv, view, range, script,
--                     cards = {{id, script}}, star, forged_ele}},
--   c.skills = {["5"] = level}, c.sc = {["37"] = {val1..val4}} (status_change.lua)
-- and returns the status window's fields plus what the battle rules need
-- (weapon types, elements, card modifiers). The server keeps the fields it
-- knows (crates/ro-game/src/status.rs Derived) and hands the whole table back
-- to the battle rules.
--
-- Where mod0631 has a plain slip, the intended value is used and marked
-- "mod0631 bug" (docs/DEVIATIONS.md lists them).

local floor = math.floor

-- db/size_fix.txt: weapon ATK % against small, medium, large, by weapon type
-- (bare, dagger, 1h sword, 2h sword, 1h spear, 2h spear, 1h axe, 2h axe, mace,
--  2h mace, rod, bow, knuckle, instrument, whip, book, katar)
local atkmods = {
  { 100, 100, 75, 75, 75, 75, 50, 50, 75, 100, 100, 100, 100, 100, 100, 100, 75 },
  { 100, 75, 100, 75, 75, 75, 75, 75, 100, 100, 100, 100, 100, 100, 100, 100, 100 },
  { 100, 50, 75, 100, 100, 100, 100, 100, 100, 100, 100, 75, 100, 100, 100, 75, 75 },
}

-- db/refine_db.txt: per weapon level (index 0 = armor): bonus per refine,
-- random bonus per refine above the safe limit, safe limit
local refinebonus = {
  [0] = { 1, 0, 4 }, { 2, 3, 7 }, { 3, 5, 6 }, { 5, 8, 5 }, { 7, 14, 4 },
}
rules.refinebonus = refinebonus

-- db/const.txt: bonus names -> what pc_bonus does with them
local PARAM = { bStr = "str", bAgi = "agi", bVit = "vit", bInt = "int", bDex = "dex", bLuk = "luk" }
local EFF = { Eff_Stone = 0, Eff_Freeze = 1, Eff_Stan = 2, Eff_Sleep = 3, Eff_Poison = 4, Eff_Curse = 5,
              Eff_Silence = 6, Eff_Confusion = 7, Eff_Blind = 8 }

-- "bonus bStr,1; bonus2 bAddRace,2,20; skill 28,1;" -> {{"bonus","bStr",1}, ...}
local parsed = {}
local function parse_script(script)
  local p = parsed[script]
  if p then return p end
  p = {}
  for stmt in script:gmatch("[^;]+") do
    local cmd, rest = stmt:match("^%s*(%a+%d?)%s+(.-)%s*$")
    if cmd then
      local args = {}
      for a in rest:gmatch("[^,]+") do
        a = a:match("^%s*(.-)%s*$")
        args[#args + 1] = tonumber(a) or EFF[a] or a
      end
      p[#p + 1] = { cmd, args }
    end
  end
  parsed[script] = p
  return p
end

-- pc.c:1179 pc_bonus, pc.c:1256 pc_bonus2. `left` = the weapon in the shield hand.
local function run_script(s, script, left)
  if not script or script == "" then return end
  for _, st in ipairs(parse_script(script)) do
    local cmd, a = st[1], st[2]
    if cmd == "bonus" then
      local k, v = a[1], a[2] or 0
      if PARAM[k] then s.paramb[PARAM[k]] = s.paramb[PARAM[k]] + v
      elseif k == "bAtk" then if left then s.watk_2 = s.watk_2 + v else s.watk2 = s.watk2 + v end
      elseif k == "bMatk" then -- pc.c:1196 SP_MATK1 does nothing in mod0631
      elseif k == "bDef" then s.def = s.def + v
      elseif k == "bMdef" then s.mdef = s.mdef + v
      elseif k == "bHit" then s.hit = s.hit + v
      elseif k == "bFlee" then s.flee = s.flee + v
      elseif k == "bFlee2" then s.flee2 = s.flee2 + v
      elseif k == "bCritical" then s.critical = s.critical + v
      elseif k == "bAtkEle" then if left then s.atk_ele_ = v else s.atk_ele = v end
      elseif k == "bDefEle" then s.def_ele = v
      elseif k == "bMaxHP" then s.max_hp = s.max_hp + v
      elseif k == "bMaxSP" then s.max_sp = s.max_sp + v
      elseif k == "bCastrate" then s.castrate = s.castrate + v
      elseif k == "bMaxHPrate" then s.hprate = s.hprate + v
      elseif k == "bMaxSPrate" then s.sprate = s.sprate + v
      elseif k == "bUseSPrate" then s.dsprate = s.dsprate + v
      elseif k == "bRange" then s.range_bonus = s.range_bonus + v -- not in mod0631's pc_bonus (two bows use it)
      end
    elseif cmd == "bonus2" then
      local k, t, v = a[1], a[2] or 0, a[3] or 0
      local tab = ({ bAddEle = s.addele, bAddRace = s.addrace, bAddSize = s.addsize, bSubEle = s.subele,
                     bSubRace = s.subrace, bAddEff = s.addeff, bResEff = s.reseff })[k]
      if tab then tab[t] = (tab[t] or 0) + v end
    elseif cmd == "skill" then
      -- pc.c:1293 pc_skill: a card or item lends a skill level
      local id, lv = a[1], a[2] or 1
      if (s.skills[id] or 0) < lv then
        s.skills[id] = lv
        s.card_skills[id] = lv
      end
    end
  end
end

local function zeros(n) local t = {}; for i = 0, n do t[i] = 0 end; return t end

function rules.status(c)
  local job = c.job or { weight = 20000, hp_coef = 0, sp_coef = 10, aspd_base = {}, job_bonus = {} }
  local lv = c.base_level
  local sc = c.sc or {}
  local function has(t) return sc[tostring(t)] end
  local option = c.option or 0
  local riding = (option & 0x20) ~= 0
  local cart = (option & 0x788) ~= 0

  local s = {
    paramb = { str = 0, agi = 0, vit = 0, int = 0, dex = 0, luk = 0 },
    hit = 0, flee = 0, flee2 = 0, critical = 0, watk = 0, def = 0, mdef = 0, watk2 = 0, def2 = 0, mdef2 = 0,
    max_hp = 0, max_sp = 0, attackrange = 1, range_bonus = 0, atk_ele = 0, def_ele = 0, star = 0, overrefine = 0,
    speed = 150, hprate = 100, sprate = 100, castrate = 100, dsprate = 100,
    addele = zeros(9), addrace = zeros(9), addsize = zeros(2), subele = zeros(9), subrace = zeros(9),
    addeff = zeros(8), reseff = zeros(8),
    watk_ = 0, watk_2 = 0, atk_ele_ = 0, star_ = 0, overrefine_ = 0,
    wcards = {}, dcards = {}, card_skills = {},
    weapontype1 = 0, weapontype2 = 0, shield = false, ammo = false,
    skills = {},
  }
  for k, v in pairs(c.skills or {}) do s.skills[tonumber(k)] = v end
  local function skill(id) return s.skills[id] or 0 end

  -- pc.c:757 equipment: ATK, DEF, refine bonuses, the item's script
  local items = (c.equip or {}).items or {}
  for _, it in ipairs(items) do
    s.def = s.def + (it.def or 0)
    if it.type == 4 then
      local rb = refinebonus[it.wlv] or refinebonus[1]
      local r = it.refine or 0
      if it.loc == 0x20 then
        -- second weapon of a dual wielder (pc.c:772)
        s.weapontype2 = it.view
        s.watk_ = s.watk_ + it.atk
        s.watk_2 = r * rb[1]
        if r > rb[3] then s.overrefine_ = (r - rb[3]) * rb[2] end
        if it.star > 0 or it.forged_ele > 0 then s.star_, s.atk_ele_ = it.star, it.forged_ele end
        run_script(s, it.script, true)
      else
        s.weapontype1 = it.view
        s.watk = s.watk + it.atk
        s.watk2 = s.watk2 + r * rb[1]
        if r > rb[3] then s.overrefine = s.overrefine + (r - rb[3]) * rb[2] end
        if it.star > 0 or it.forged_ele > 0 then s.star, s.atk_ele = s.star + it.star, it.forged_ele end
        s.attackrange = it.range or 1
        run_script(s, it.script, false)
      end
    elseif it.type == 5 then
      s.watk = s.watk + (it.atk or 0)
      s.def = s.def + (it.refine or 0) * refinebonus[0][1]
      if (it.loc & 0x20) ~= 0 then s.shield = true end
      run_script(s, it.script, false)
    end
  end
  -- pc.c:812 cards
  for _, it in ipairs(items) do
    for _, card in ipairs(it.cards or {}) do
      run_script(s, card.script, it.type == 4 and it.loc == 0x20)
      local list = it.type == 4 and s.wcards or s.dcards
      list[#list + 1] = card.id
    end
  end
  -- pc.c:3538 (pc_equipitem): status.weapon, the combined type of two weapons
  local w1, w2 = s.weapontype1, s.weapontype2
  local weapon = w1
  if w2 ~= 0 then
    if w1 == 0 then weapon = w2
    elseif w1 == 1 and w2 == 1 then weapon = 0x11
    elseif w1 == 2 and w2 == 2 then weapon = 0x12
    elseif w1 == 6 and w2 == 6 then weapon = 0x13
    elseif (w1 == 1 and w2 == 2) or (w1 == 2 and w2 == 1) then weapon = 0x14
    elseif (w1 == 1 and w2 == 6) or (w1 == 6 and w2 == 1) then weapon = 0x15
    elseif (w1 == 2 and w2 == 6) or (w1 == 6 and w2 == 2) then weapon = 0x16
    end
  end
  s.weapon = weapon
  -- pc.c:854 the arrow's script counts with a bow in hand (its element)
  for _, it in ipairs(items) do
    if it.type == 10 then
      s.ammo = it.id
      s.ammo_atk = it.atk or 0
      if weapon == 11 then run_script(s, it.script, false) end
    end
  end
  s.attackrange = s.attackrange + s.range_bonus
  local parame = {}
  for k, v in pairs(s.paramb) do parame[k] = v end

  -- pc.c:863 size modifiers of each hand's weapon
  s.atkmods = { [0] = atkmods[1][w1 + 1] or 100, atkmods[2][w1 + 1] or 100, atkmods[3][w1 + 1] or 100 }
  s.atkmods_ = { [0] = atkmods[1][w2 + 1] or 100, atkmods[2][w2 + 1] or 100, atkmods[3][w2 + 1] or 100 }

  -- pc.c:873 job bonus: one point per listed job level
  local names = { "str", "agi", "vit", "int", "dex", "luk" }
  for i = 1, math.min(c.job_level or 1, #(job.job_bonus or {})) do
    local b = job.job_bonus[i]
    if b and b > 0 then s.paramb[names[b]] = s.paramb[names[b]] + 1 end
  end
  -- pc.c:877 Owl's Eye
  s.paramb.dex = s.paramb.dex + skill(SK.AC_OWL)

  -- pc.c:880 status changes on the six stats and the walk speed
  local t
  t = has(SC.INCREASEAGI)
  if t then s.paramb.agi = s.paramb.agi + 2 + t.val1; s.speed = s.speed - floor(s.speed * 25 / 100) end
  if has(SC.DECREASEAGI) then s.speed = floor(s.speed * 125 / 100) end
  t = has(SC.BLESSING)
  if t then
    s.paramb.str = s.paramb.str + t.val1
    s.paramb.int = s.paramb.int + t.val1
    s.paramb.dex = s.paramb.dex + t.val1
  end
  t = has(SC.CONCENTRATE)
  if t then
    s.paramb.agi = s.paramb.agi + floor((c.agi + s.paramb.agi) * (2 + t.val1) / 100)
    s.paramb.dex = s.paramb.dex + floor((c.dex + s.paramb.dex) * (2 + t.val1) / 100)
  end
  if has(SC.GLORIA) then s.paramb.luk = s.paramb.luk + 30 end
  if has(SC.LOUD) then s.paramb.str = s.paramb.str + 4 end
  if has(SC.QUAGMIRE) then s.speed = floor(s.speed * 3 / 2) end

  local str, agi, vit = c.str + s.paramb.str, c.agi + s.paramb.agi, c.vit + s.paramb.vit
  local int, dex, luk = c.int + s.paramb.int, c.dex + s.paramb.dex, c.luk + s.paramb.luk
  s.str, s.agi, s.vit, s.int, s.dex, s.luk = str, agi, vit, int, dex, luk

  -- pc.c:914
  local i5, i7 = floor(int / 5), floor(int / 7)
  s.matk1 = int + i5 * i5
  s.matk2 = int + i7 * i7
  s.hit = s.hit + dex + lv
  s.flee = s.flee + agi + lv
  s.def2 = s.def2 + vit
  s.mdef2 = s.mdef2 + int
  s.flee2 = s.flee2 + floor(luk / 10) + 1
  s.critical = s.critical + floor(luk * 3 / 10) + 1

  -- pc.c:926 attack delay: base - (agi*4 + dex) * base / 1000; two weapons: 70% of the sum
  local function aspd_of(wt)
    local base = job.aspd_base[wt + 1] or 2000
    return base - floor((agi * 4 + dex) * base / 1000)
  end
  local aspd
  if weapon <= 16 then aspd = aspd_of(weapon)
  else aspd = floor((aspd_of(w1) + aspd_of(w2)) * 140 / 200) end
  if aspd < 100 then aspd = 100 end -- ASPD 190; the only cap (pc.c:936): what follows may go below it
  s.amotion = aspd                  -- pc.c:937: the attack motion is taken here, before riding and status changes
  s.dmotion = math.max(400, 800 - agi * 4)
  -- pc.c:942 Cavalier Mastery takes the Peco penalty back
  if riding then aspd = floor(aspd * (100 + 10 * (5 - skill(SK.KN_CAVALIERMASTERY))) / 100) end

  -- pc.c:945 passive skills
  if skill(SK.AC_VULTURE) > 0 and weapon == 11 then
    s.hit = s.hit + skill(SK.AC_VULTURE)
    s.attackrange = s.attackrange + skill(SK.AC_VULTURE)
  end
  s.flee = s.flee + skill(SK.TF_MISS) * 3
  s.flee = s.flee + ((skill(SK.MO_DODGE) * 3) >> 1)
  s.hit = s.hit + skill(SK.BS_WEAPONRESEARCH) * 2
  if (option & 2) ~= 0 and skill(SK.RG_TUNNELDRIVE) > 0 then
    s.speed = s.speed + floor(s.speed * (20 - skill(SK.RG_TUNNELDRIVE)) / 40)
  end
  -- pc.c:957 the cart slows (less with Pushcart), the Peco speeds up.
  -- mod0631 bug: it only slows with Pushcart learnt, and a cart cannot be had without it
  if cart then
    s.speed = s.speed + floor((10 - skill(SK.MC_PUSHCART)) * 0.1 * 150)
  elseif riding then
    s.speed = floor(s.speed - 0.17 * 150) -- 124.5, cut to 124 as C does
  end
  if skill(SK.CR_TRUST) > 0 then
    s.max_hp = s.max_hp + skill(SK.CR_TRUST) * 200
    s.subele[6] = s.subele[6] + skill(SK.CR_TRUST) * 5 -- mod0631 wrote addele; Faith resists Holy
  end

  -- pc.c:969 max HP = ((35 + lv*5) + (1+lv)*lv/2 * hp_coef) * (1 + vit/100) + bonuses, cap 32000
  local hp = floor(((35 + lv * 5) + floor((1 + lv) * lv / 2) * job.hp_coef) * (1 + vit / 100)) + s.max_hp
  if s.hprate ~= 100 then hp = floor(hp * s.hprate / 100) end
  s.max_hp = math.min(32000, math.max(1, hp))
  -- pc.c:979 max SP = ((sp_coef * lv / 10) + 10) * (100 + int) / 100 + bonuses + INT from equipment
  local sp = s.max_sp + floor((floor(job.sp_coef * lv / 10) + 10) * (100 + int) / 100) + parame.int
  if s.sprate ~= 100 then sp = floor(sp * s.sprate / 100) end
  s.max_sp = math.max(1, sp)

  -- pc.c:989 Dragonology
  if skill(SK.SA_DRAGONOLOGY) > 0 then
    s.addrace[9] = s.addrace[9] + skill(SK.SA_DRAGONOLOGY) * 4
    s.subrace[9] = s.subrace[9] + skill(SK.SA_DRAGONOLOGY) * 4
  end

  -- pc.c:995 status changes on the rest
  t = has(SC.ANGELUS)
  if t then s.def2 = floor(s.def2 * (110 + 5 * t.val1) / 100) end
  t = has(SC.IMPOSITIO)
  if t then s.watk = s.watk + t.val1 * 5 end
  t = has(SC.PROVOKE)
  if t then
    s.def2 = floor(s.def2 * (100 - 6 * t.val1) / 100)
    s.watk = floor(s.watk * (100 + 2 * t.val1) / 100)
  end
  t = has(SC.DRUMBATTLE)
  if t then s.watk = s.watk + t.val2; s.def = s.def + t.val3 end
  t = has(SC.NIBELUNGEN)
  if t then s.watk = s.watk + t.val2 end
  if has(SC.ETERNALCHAOS) then s.def = 0 end

  if has(SC.TWOHANDQUICKEN) then aspd = floor(aspd * 70 / 100) end
  if has(SC.ADRENALINE) then aspd = floor(aspd * 70 / 100) end
  t = has(SC.SPEARSQUICKEN)
  if t then aspd = floor(aspd * t.val2 / 100) end
  t = has(SC.ASSNCROS)
  if t then aspd = floor(aspd * t.val2 / 100) end
  t = has(SC.DONTFORGETME)
  if t then aspd = floor(aspd * t.val2 / 100); s.speed = floor(s.speed * t.val3 / 100) end
  -- pc.c:1027 speed potions, strongest first
  t = has(SC.SPEEDPOTION2) or has(SC.SPEEDPOTION1) or has(SC.SPEEDPOTION0)
  if t then aspd = floor(aspd * t.val2 / 100) end

  t = has(SC.WHISTLE)
  if t then
    s.flee = s.flee + floor(t.val2 * s.flee / 100)
    s.flee2 = s.flee2 + floor(t.val2 * s.flee2 / 100)
  end
  t = has(SC.HUMMING)
  if t then s.hit = s.hit + floor(t.val2 * s.hit / 100) end
  if has(SC.BLIND) then
    s.hit = s.hit - floor(s.hit * 25 / 100)
    s.flee = s.flee - floor(s.flee * 25 / 100)
  end
  t = has(SC.SIEGFRIED)
  if t then s.subele[1] = s.subele[1] + t.val2; s.subele[3] = s.subele[3] + t.val2 end -- mod0631 wrote addele
  t = has(SC.PROVIDENCE)
  if t then s.subele[6] = s.subele[6] + t.val2; s.subrace[6] = s.subrace[6] + t.val2 end -- mod0631 wrote add*
  t = has(SC.APPLEIDUN)
  if t then s.max_hp = math.min(32000, s.max_hp + floor(t.val2 * s.max_hp / 100)) end
  t = has(SC.SERVICE4U)
  if t then s.max_sp = s.max_sp + floor(t.val2 * s.max_sp / 100) end
  t = has(SC.FORTUNE)
  if t then s.critical = s.critical + floor(t.val1 * s.critical / 100) end
  t = has(SC.EXPLOSIONSPIRITS)
  if t then s.critical = s.critical + floor(t.val2 * s.critical / 1000) end
  if has(SC.STEELBODY) then
    s.def, s.mdef = 90, 90
    aspd = floor(aspd * 125 / 100)
    s.speed = floor(s.speed * 125 / 100)
  end
  -- pc.c:1094 a Sage walking while casting
  if c.casting and skill(SK.SA_FREECAST) > 0 then
    s.speed = floor(s.speed * (175 - skill(SK.SA_FREECAST) * 5) / 100)
  end

  s.aspd = aspd
  -- pc.c:690 max_weight = job base + str*300 (tenths; the character's own STR), +1000 per Enlarge Weight Limit
  s.max_weight = job.weight + c.str * 300 + skill(SK.MC_INCCARRY) * 1000

  -- the status window (crates/ro-game/src/status.rs). ATK: the left number
  -- is the base attack (battle.c:907; bows swap STR and DEX) plus the weapon,
  -- the right one the refine bonus. mod0631 showed STR + weapon on the left.
  local a, b = str, dex
  if weapon == 11 then a, b = dex, str end
  local a10 = floor(a / 10)
  s.base_atk = a + a10 * a10 + floor(b / 5) + floor(luk / 5)
  s.atk = s.base_atk + s.watk
  s.atk2 = s.watk2
  s.matk_max, s.matk_min = s.matk1, s.matk2
  s.crit = s.critical * 10 -- the server keeps tenths of a percent
  s.range = math.max(1, s.attackrange)
  s.bstr, s.bagi, s.bvit = s.paramb.str, s.paramb.agi, s.paramb.vit
  s.bint, s.bdex, s.bluk = s.paramb.int, s.paramb.dex, s.paramb.luk
  s.riding, s.cart, s.falcon = riding, cart, (option & 0x10) ~= 0
  s.cart_weight = c.cart_weight or 0
  return s
end

-- What the server itself keeps of a character's status (the status window,
-- walk and attack speed, weight limit, attack range).
local VIEW = { "max_hp", "max_sp", "atk", "atk2", "matk_min", "matk_max", "def", "def2", "mdef", "mdef2", "hit", "flee",
               "flee2", "crit", "max_weight", "range", "speed", "bstr", "bagi", "bvit", "bint", "bdex", "bluk" }
function rules.status_view(c)
  local s = rules.status(c)
  local v = {}
  for _, k in ipairs(VIEW) do v[k] = s[k] end
  v.amotion = s.aspd -- the window's ASPD and the attack delay are both sd->aspd (clif.c:1432, pc.c:2458)
  return v
end

-- Battle: who can hit whom, how hard, and what a hit does.
-- Server-side: NOT in the client.
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/battle.c (archived
-- 2003-12-18): battle_get_*, battle_attr_fix, battle_calc_damage,
-- battle_addmastery, battle_calc_weapon_attack, battle_calc_magic_attack,
-- battle_calc_misc_attack, battle_weapon_attack; map/skill.c skill_attack,
-- skill_blown. Line refs are to those files. db/attr_fix.txt.
--
-- A "bl" here is a snapshot of a player or monster (crates/ro-game/src/
-- battle.rs bl_json), with `st` = rules.status() for players. Snapshots go
-- stale as soon as something changes: functions take ids and load afresh.
--
-- Where mod0631 has a plain slip the intended value is used and marked
-- "mod0631 bug" (docs/DEVIATIONS.md).

local floor, random = math.floor, math.random

battle = {}

-- rand() % n
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end
battle.rnd = rnd

function battle.get(id)
  if not id or id == 0 then return nil end
  local bl = _rt("bl", id)
  if not bl then return nil end
  if bl.kind == "player" then bl.st = rules.status(bl.char) end
  return bl
end
local get = battle.get

local function sc(bl, t) return bl.sc[tostring(t)] end
battle.sc = sc
function battle.pc_checkskill(bl, id) return bl.kind == "player" and (bl.st.skills[id] or 0) or 0 end
local checkskill = battle.pc_checkskill
local function has_card(list, id) for _, c in ipairs(list) do if c == id then return true end end return false end
function battle.wcard(bl, id) return bl.kind == "player" and has_card(bl.st.wcards, id) end
function battle.dcard(bl, id) return bl.kind == "player" and has_card(bl.st.dcards, id) end
function battle.card(bl, id) return battle.wcard(bl, id) or battle.dcard(bl, id) end

-- ------------------------------------------------------------ battle_get_*

function battle.lv(bl) return bl.kind == "mob" and bl.db.level or bl.char.base_level end
function battle.hp(bl) return bl.hp end
function battle.max_hp(bl) return bl.kind == "mob" and bl.max_hp or bl.st.max_hp end
function battle.str(bl) return bl.kind == "mob" and bl.db.str or bl.st.str end
function battle.agi(bl) -- battle.c:60
  local agi
  if bl.kind == "mob" then
    agi = bl.db.agi
    local t = sc(bl, SC.INCREASEAGI)
    if t then agi = agi + 2 + t.val1 end
  else
    agi = bl.st.agi
  end
  local t = sc(bl, SC.DECREASEAGI)
  if t then agi = agi - (2 + t.val1) end
  if sc(bl, SC.QUAGMIRE) then agi = floor(agi / 2) end
  return agi
end
function battle.vit(bl) return bl.kind == "mob" and bl.db.vit or bl.st.vit end
function battle.int(bl) -- battle.c:94
  if bl.kind ~= "mob" then return bl.st.int end
  local v = bl.db.int
  local t = sc(bl, SC.BLESSING)
  if t then
    local race = bl.db.race
    if race == 1 or race == 6 then v = floor(v / 2) else v = v + t.val1 end
  end
  return v
end
function battle.dex(bl) -- battle.c:111
  local dex
  if bl.kind == "mob" then
    dex = bl.db.dex
    local t = sc(bl, SC.BLESSING)
    if t then
      local race = bl.db.race
      if race == 1 or race == 6 then dex = floor(dex / 2) else dex = dex + t.val1 end
    end
  else
    dex = bl.st.dex
  end
  if sc(bl, SC.QUAGMIRE) then dex = floor(dex / 2) end
  return dex
end
function battle.luk(bl) -- battle.c:134
  local luk
  if bl.kind == "mob" then
    luk = bl.db.luk
    if sc(bl, SC.GLORIA) then luk = luk + 30 end
  else
    luk = bl.st.luk
  end
  if sc(bl, SC.CURSE) then luk = 0 end
  return luk
end
function battle.flee(bl) -- battle.c:154
  if bl.kind ~= "mob" then return bl.st.flee end
  local flee = battle.agi(bl) + bl.db.level
  if sc(bl, SC.BLIND) then flee = flee - floor(flee * 25 / 100) end
  return flee
end
function battle.hit(bl) -- battle.c:170
  if bl.kind ~= "mob" then return bl.st.hit end
  local hit = battle.dex(bl) + bl.db.level
  if sc(bl, SC.BLIND) then hit = hit - floor(hit * 25 / 100) end
  return hit
end
function battle.flee2(bl) return bl.kind == "mob" and floor(battle.luk(bl) / 10) + 1 or bl.st.flee2 end
function battle.critical(bl) return bl.kind == "mob" and floor(battle.luk(bl) / 3) + 1 or bl.st.critical end
function battle.atk(bl) -- battle.c:207
  if bl.kind == "mob" then return bl.db.atk1 end
  local atk = bl.st.watk
  if sc(bl, SC.CURSE) then atk = atk - floor(atk * 25 / 100) end
  return atk
end
function battle.atk2(bl) -- battle.c:221
  if bl.kind ~= "mob" then return bl.st.watk2 end
  local atk2 = bl.db.atk2
  local t = sc(bl, SC.IMPOSITIO)
  if t then atk2 = atk2 + t.val1 * 5 end
  t = sc(bl, SC.PROVOKE)
  if t then atk2 = floor(atk2 * (100 + 2 * t.val1) / 100) end
  if sc(bl, SC.CURSE) then atk2 = atk2 - floor(atk2 * 25 / 100) end
  return atk2
end
function battle.def(bl)
  if bl.kind ~= "mob" then return bl.st.def end
  local def = bl.db.def
  -- Signum Crucis: eAthena 817 battle.c:502 (mod0631 never starts the status)
  local t = sc(bl, SC.SIGNUMCRUCIS)
  if t and def < 10000 then def = floor(def * (100 - t.val2) / 100) end
  return def
end
function battle.mdef(bl) return bl.kind == "mob" and bl.db.mdef or bl.st.mdef end
function battle.def2(bl) -- battle.c:258
  if bl.kind ~= "mob" then return bl.st.def2 end
  local def2 = bl.db.vit
  local t = sc(bl, SC.ANGELUS)
  if t then def2 = floor(def2 * (110 + 5 * t.val1) / 100) end
  t = sc(bl, SC.PROVOKE)
  if t then def2 = floor(def2 * (100 - 6 * t.val1) / 100) end
  t = sc(bl, SC.SIGNUMCRUCIS) -- 817 battle.c:554
  if t then def2 = floor(def2 * (100 - t.val2) / 100) end
  return def2
end
function battle.mdef2(bl) return bl.kind == "mob" and bl.db.int or bl.st.mdef2 end
function battle.amotion(bl) return bl.kind == "mob" and bl.db.attack_motion or bl.st.amotion end
function battle.dmotion(bl) -- battle.c:296
  if sc(bl, SC.PROVOKE) then return 0 end
  return bl.kind == "mob" and bl.db.damage_motion or bl.st.dmotion
end
-- defence element: type + level*20 (battle.c:311)
function battle.element(bl)
  local ret
  if bl.kind == "mob" then ret = bl.def_ele or bl.db.element else ret = 20 + bl.st.def_ele end
  if sc(bl, SC.BENEDICTIO) then ret = 26 end
  if sc(bl, SC.FREEZE) then ret = 21 end
  return ret
end
function battle.elem_type(bl) return battle.element(bl) % 10 end
local function enchant(bl, ret) -- battle.c:342
  if sc(bl, SC.FROSTWEAPON) then ret = 1 end
  if sc(bl, SC.SEISMICWEAPON) then ret = 2 end
  if sc(bl, SC.FLAMELAUNCHER) then ret = 3 end
  if sc(bl, SC.LIGHTNINGLOADER) then ret = 4 end
  if sc(bl, SC.ENCPOISON) then ret = 5 end
  if sc(bl, SC.ASPERSIO) then ret = 6 end
  return ret
end
function battle.attack_element(bl) return enchant(bl, bl.kind == "mob" and 0 or bl.st.atk_ele) end
function battle.attack_element2(bl) return enchant(bl, bl.kind == "mob" and 0 or bl.st.atk_ele_) end
function battle.race(bl) return bl.kind == "mob" and bl.db.race or 7 end
function battle.size(bl) return bl.kind == "mob" and bl.db.size or 1 end
function battle.mode(bl) return bl.kind == "mob" and bl.db.mode or 1 end
function battle.mexp(bl) return bl.kind == "mob" and bl.db.mvp_exp or 0 end
function battle.is_boss(bl) return bl.kind == "mob" and ((bl.db.mode & 0x20) ~= 0 or bl.db.mvp_exp > 0) end

-- ------------------------------------------------------------ sides

-- battle_check_target (battle.c:1730), for one player's world: a player and
-- a monster are enemies; a monster's slaves and its own kind are its friends;
-- nothing else is anyone's enemy (no PvP, no guild wars).
function battle.is_enemy(a, b)
  if not a or not b or a.id == b.id then return false end
  return a.kind ~= b.kind
end
function battle.check_target(src, target, flag)
  if flag == BCT.ALL then return true end
  if flag == BCT.ENEMY then return battle.is_enemy(src, target) end
  if flag == BCT.PARTY then
    if src.id == target.id then return true end
    if src.kind == "mob" and target.kind == "mob" then
      return src.master == target.id or target.master == src.id or (src.master ~= nil and src.master == target.master)
    end
    return false
  end
  return not battle.is_enemy(src, target) -- NOENEMY
end

function battle.distance(a, b) return math.max(math.abs(a.x - b.x), math.abs(a.y - b.y)) end

-- map.c map_calc_dir: the direction from `src` to (x, y).
-- 0 N(+y) 1 NW 2 W 3 SW 4 S 5 SE 6 E 7 NE
function battle.calc_dir(src, x, y)
  local dx, dy = x - src.x, y - src.y
  local dir
  if dx == 0 and dy == 0 then dir = 0
  elseif dx >= 0 and dy >= 0 then
    dir = 7
    if dx * 3 - 1 < dy then dir = 0 end
    if dx > dy * 3 then dir = 6 end
  elseif dx >= 0 and dy <= 0 then
    dir = 5
    if dx * 3 - 1 < -dy then dir = 4 end
    if dx > -dy * 3 then dir = 6 end
  elseif dx <= 0 and dy <= 0 then
    dir = 3
    if dx * 3 + 1 > dy then dir = 4 end
    if dx < dy * 3 then dir = 2 end
  else
    dir = 1
    if -dx * 3 - 1 < dy then dir = 0 end
    if -dx > dy * 3 then dir = 2 end
  end
  return dir
end

-- eAthena 817 map.c map_check_dir: false when `s_dir` (the way the attacker
-- looks at the target) is the way the target faces, give or take one step:
-- the attacker is behind it.
function battle.check_dir(s_dir, t_dir)
  local d = (s_dir - t_dir) % 8
  return not (d == 0 or d == 1 or d == 7)
end

-- battle_check_range (battle.c:1809): within range. (The emulator also asked
-- for a walkable path; with one client that walks into range itself, range alone decides.)
function battle.check_range(src, x, y, range)
  local d = math.max(math.abs(x - src.x), math.abs(y - src.y))
  return not (range > 0 and range < d)
end

-- ------------------------------------------------------------ element table

-- db/attr_fix.txt: damage % by defence level, attack element (row), defence element (column)
local attr_fix = {
  { -- level 1
    [0] = { [0] = 100, 100, 100, 100, 100, 100, 100, 100, 25, 100 },
    { [0] = 100, 25, 100, 150, 50, 100, 75, 100, 100, 100 },
    { [0] = 100, 100, 100, 50, 150, 100, 75, 100, 100, 100 },
    { [0] = 100, 50, 150, 25, 100, 100, 75, 100, 100, 125 },
    { [0] = 100, 175, 50, 100, 25, 100, 75, 100, 100, 100 },
    { [0] = 100, 100, 125, 125, 125, 0, 75, 50, 100, -25 },
    { [0] = 100, 100, 100, 100, 100, 100, 0, 125, 100, 150 },
    { [0] = 100, 100, 100, 100, 100, 50, 125, 0, 100, -25 },
    { [0] = 25, 100, 100, 100, 100, 100, 75, 75, 125, 100 },
    { [0] = 100, 100, 100, 100, 100, 50, 100, 0, 100, 0 },
  },
  { -- level 2
    [0] = { [0] = 100, 100, 100, 100, 100, 100, 100, 100, 25, 100 },
    { [0] = 100, 0, 100, 175, 25, 100, 50, 75, 100, 100 },
    { [0] = 100, 100, 50, 25, 175, 100, 50, 75, 100, 100 },
    { [0] = 100, 25, 175, 0, 100, 100, 50, 75, 100, 150 },
    { [0] = 100, 175, 25, 100, 0, 100, 50, 75, 100, 100 },
    { [0] = 100, 75, 125, 125, 125, 0, 50, 25, 75, -50 },
    { [0] = 100, 100, 100, 100, 100, 100, -25, 150, 100, 175 },
    { [0] = 100, 100, 100, 100, 100, 25, 150, -25, 100, -50 },
    { [0] = 0, 75, 75, 75, 75, 75, 50, 50, 150, 125 },
    { [0] = 100, 75, 75, 75, 75, 25, 125, 0, 100, 0 },
  },
  { -- level 3
    [0] = { [0] = 100, 100, 100, 100, 100, 100, 100, 100, 0, 100 },
    { [0] = 100, -25, 100, 200, 0, 100, 25, 50, 100, 125 },
    { [0] = 100, 100, 0, 0, 200, 100, 25, 50, 100, 75 },
    { [0] = 100, 0, 200, -25, 100, 100, 25, 50, 100, 175 },
    { [0] = 100, 200, 0, 100, -25, 100, 25, 50, 100, 100 },
    { [0] = 100, 50, 100, 100, 100, 0, 25, 0, 50, -75 },
    { [0] = 100, 100, 100, 100, 100, 125, -50, 175, 100, 200 },
    { [0] = 100, 100, 100, 100, 100, 0, 175, -50, 100, -75 },
    { [0] = 0, 50, 50, 50, 50, 50, 25, 25, 175, 150 },
    { [0] = 100, 50, 50, 50, 50, 0, 150, 0, 100, 0 },
  },
  { -- level 4
    [0] = { [0] = 100, 100, 100, 100, 100, 100, 100, 100, 0, 100 },
    { [0] = 100, -50, 100, 200, 0, 75, 0, 25, 100, 150 },
    { [0] = 100, 100, -25, 0, 200, 75, 0, 25, 100, 50 },
    { [0] = 100, 0, 200, -50, 100, 75, 0, 25, 100, 200 },
    { [0] = 100, 200, 0, 100, -50, 75, 0, 25, 100, 100 },
    { [0] = 100, 25, 75, 75, 75, 0, 0, -25, 25, -100 },
    { [0] = 100, 75, 75, 75, 75, 125, -100, 200, 100, 200 },
    { [0] = 100, 75, 75, 75, 75, -25, 200, -100, 100, -100 },
    { [0] = 0, 25, 25, 25, 25, 25, 0, 0, 200, 175 },
    { [0] = 100, 25, 25, 25, 25, -25, 175, 0, 100, 0 },
  },
}
battle.attr_fix_table = attr_fix

-- battle.c:601. C division truncates toward zero (a healing negative stays whole).
function battle.attr_fix(damage, atk_elem, def_elem)
  local def_type, def_lv = def_elem % 10, floor(floor(def_elem / 10) / 2)
  local t = attr_fix[def_lv]
  if not t or not t[atk_elem] then return damage end
  local v = damage * t[atk_elem][def_type]
  if v < 0 then return -floor(-v / 100) end
  return floor(v / 100)
end

-- ------------------------------------------------------------ taking damage

local function sc_end(id, t) return skill_status_change_end(id, t) end

-- battle.c:511 battle_damage / battle.c:565 battle_heal. Returns true when the target died.
function battle.damage(src_id, target_id, damage)
  if damage == 0 then return false end
  if damage < 0 then battle.heal(target_id, -damage, 0); return false end
  local t = _rt("bl", target_id)
  if not t or t.dead then return false end
  -- frozen, stone and asleep end with the first hit
  if t.sc[tostring(SC.FREEZE)] then sc_end(target_id, SC.FREEZE) end
  if t.sc[tostring(SC.STONE)] then sc_end(target_id, SC.STONE) end
  if t.sc[tostring(SC.SLEEP)] then sc_end(target_id, SC.SLEEP) end
  return _rt("bl_damage", src_id or 0, target_id, damage)
end
function battle.heal(target_id, hp, sp)
  if hp < 0 then battle.damage(0, target_id, -hp); return 0 end
  return _rt("bl_heal", target_id, hp, sp or 0)
end

-- battle.c:619 battle_calc_damage: what stands between a computed hit and the
-- target (Safety Wall, Pneuma, Lex Aeterna, Energy Coat, Kyrie Eleison).
function battle.calc_damage(bl, damage, flag)
  local t
  t = sc(bl, SC.SAFETYWALL)
  if t and damage > 0 and (flag & BF.WEAPON) ~= 0 and (flag & BF.SHORT) ~= 0 then
    skill_safetywall_hit(bl, t.val2) -- one hit off the wall
    damage = 0
  end
  if sc(bl, SC.PNEUMA) and damage > 0 and (flag & BF.WEAPON) ~= 0 and (flag & BF.LONG) ~= 0 then
    damage = 0
  end
  if sc(bl, SC.AETERNA) and damage > 0 then
    damage = damage * 2
    sc_end(bl.id, SC.AETERNA)
  end
  if sc(bl, SC.ENERGYCOAT) and damage > 0 and bl.kind == "player" then
    local sp = bl.char.sp
    if sp > 0 and (flag & BF.WEAPON) ~= 0 then
      local per = floor(sp * 5 / (bl.st.max_sp + 1))
      sp = math.max(0, sp - floor(damage * (per * 5 + 10) / 1000))
      damage = damage - floor(damage * (per * 6) / 100)
      _rt("bl_set", bl.id, "sp", sp)
    end
    if sp <= 0 then sc_end(bl.id, SC.ENERGYCOAT) end
  end
  t = sc(bl, SC.KYRIE)
  if t then
    local left = t.val2 - damage
    if (flag & BF.WEAPON) ~= 0 then
      if left >= 0 then damage = 0 else damage = -left end
    end
    local hits = t.val1 - 1
    if hits <= 0 or left <= 0 then
      sc_end(bl.id, SC.KYRIE)
    else
      _rt("sc_touch", bl.id, SC.KYRIE, hits, left, nil, t.left)
    end
  end
  if damage > 0 then
    for _, k in ipairs({ SC.STONE, SC.FREEZE, SC.SLEEP }) do
      if sc(bl, k) then sc_end(bl.id, k); break end
    end
  end
  -- battle.c:689: a hit may trigger a monster's counter skills
  if bl.kind == "mob" and bl.hp > 0 then
    if (flag & BF.SHORT) ~= 0 then _rt("mob_event", bl.id, "closedattacked") end
    if (flag & BF.LONG) ~= 0 then _rt("mob_event", bl.id, "longrangeattacked") end
  end
  return damage
end

-- battle.c:699 battle_addmastery: flat damage from weapon masteries
function battle.addmastery(sd, target, dmg)
  local race = battle.race(target)
  local damage = 0
  local s = checkskill(sd, SK.AL_DEMONBANE)
  if s > 0 and (race == 1 or race == 6) then damage = damage + s * 3 end
  s = checkskill(sd, SK.HT_BEASTBANE)
  if s > 0 and (race == 2 or race == 4) then damage = damage + s * 4 end
  local w = sd.st.weapon
  if w == 0 or w == 0x0c then damage = damage + checkskill(sd, SK.MO_IRONHAND) * 3
  elseif w == 1 or w == 2 then damage = damage + checkskill(sd, SK.SM_SWORD) * 4
  elseif w == 3 then damage = damage + checkskill(sd, SK.SM_TWOHAND) * 4
  elseif w == 4 or w == 5 then
    damage = damage + checkskill(sd, SK.KN_SPEARMASTERY) * (sd.st.riding and 5 or 4)
  elseif w == 6 or w == 7 then damage = damage + checkskill(sd, SK.AM_AXEMASTERY) * 3
  elseif w == 8 then damage = damage + checkskill(sd, SK.PR_MACEMASTERY) * 3
  elseif w == 0x0d then damage = damage + checkskill(sd, SK.BA_MUSICALLESSON) * 3
  elseif w == 0x0e then damage = damage + checkskill(sd, SK.DC_DANCINGLESSON) * 3
  elseif w == 0x0f then damage = damage + checkskill(sd, SK.SA_ADVANCEDBOOK) * 3
  elseif w == 0x10 then damage = damage + checkskill(sd, SK.AS_KATAR) * 3
  end
  return dmg + damage
end

-- ------------------------------------------------------------ weapon attacks

-- battle.c:840 battle_calc_weapon_attack. skill_num 0 = a plain attack.
-- Returns {damage, damage2, type, div, amotion, dmotion, blewcount, flag}.
-- opt.sure_crit: a certain critical (Auto Counter, 817).
function battle.calc_weapon_attack(src, target, skill_num, skill_lv, wflag, opt)
  local sd = src.kind == "player" and src or nil
  local tsd = target.kind == "player" and target or nil
  local def1, def2 = battle.def(target), battle.def2(target)
  local s_race, s_ele, s_ele_ = battle.race(src), battle.attack_element(src), battle.attack_element2(src)
  local t_race, t_size, t_mode = battle.race(target), battle.size(target), battle.mode(target)
  local flag = BF.SHORT | BF.WEAPON | BF.NORMAL
  local hitrate = battle.hit(src) - battle.flee(target) + 80
  local type, div = 0, 1
  local blewcount = 0
  local str, dex, luk = battle.str(src), battle.dex(src), battle.luk(src)
  local damage, damage2, atkmin, atkmax
  local atkmin_, atkmax_ = 0, 0
  local st = sd and sd.st

  if sd then
    -- battle.c:903 base attack; the weapon's ATK is the random part, cut by size
    local s10 = floor(str / 10)
    damage = s10 * s10 + str + floor(dex / 5) + floor(luk / 5)
    damage2 = damage
    atkmin, atkmin_ = dex, dex
    if st.weapontype1 == 11 then
      local d10 = floor(dex / 10)
      damage = d10 * d10 + dex + floor(str / 5) + floor(luk / 5)
      atkmin = (st.watk ~= 0 and dex < st.watk) and dex or st.watk -- battle.c:914 as it evaluates
      flag = (flag & ~0xf0) | BF.LONG
    end
    if target.kind == "mob" then
      if st.riding and (st.weapontype1 == 4 or st.weapontype1 == 5) and t_size == 1 then
        atkmax = st.watk -- a spear from a Peco hits medium size in full
      else
        atkmax = floor((st.watk - st.watk_) * st.atkmods[t_size] / 100)
        atkmax_ = floor(st.watk_ * st.atkmods_[t_size] / 100)
      end
    else
      atkmax = st.watk
    end
    if battle.wcard(sd, 4137) then -- Drake Card: no size penalty
      atkmax = st.watk - st.watk_
      atkmax_ = st.watk_
    end
    if sc(src, SC.WEAPONPERFECTION) then atkmax = st.watk end
  else
    -- battle.c:939 monsters: no size modifier
    damage2 = 0
    local s10 = floor(str / 10)
    damage = s10 * s10 + str
    atkmin, atkmax = battle.atk(src), battle.atk2(src)
    if src.db.range > 2 then flag = (flag & ~0xf0) | BF.LONG end
  end
  if atkmin > atkmax then atkmin = atkmax end
  if atkmin_ > atkmax_ then atkmin_ = atkmax_ end
  if sc(src, SC.MAXIMIZEPOWER) then atkmin, atkmin_ = atkmax, atkmax_ end

  -- battle.c:957 Double Attack (daggers), Raging Trifecta Blow
  local da = 0
  if sd and skill_num == 0 then
    if st.weapontype1 == 1 and checkskill(sd, SK.TF_DOUBLE) > 0 then
      da = rnd(100) < checkskill(sd, SK.TF_DOUBLE) * 5 and 1 or 0
    end
    local tri = checkskill(sd, SK.MO_TRIPLEATTACK)
    if tri > 0 then da = rnd(100) < (30 - tri) and 2 or 0 end
  end

  local cri = 0
  if da == 0 then
    -- battle.c:972 critical: 1 + luk*3 per mille, cards and Katars on top
    cri = 1 + floor(luk * 30 / 10)
    if sd then
      cri = cri + 10 * (st.critical - (floor(st.luk * 3 / 10) + 1))
      if st.weapontype1 == 16 then cri = cri * 2 end
    elseif battle_config.enemy_critical_rate ~= 100 then
      cri = math.max(1, floor(cri * battle_config.enemy_critical_rate / 100))
    end
    cri = cri - floor(battle.luk(target) * 30 / 10)
    if sc(target, SC.SLEEP) then cri = cri * 2 end
  end

  if opt and opt.sure_crit then cri = 1000 end
  if da == 0 and rnd(1000) < cri and (skill_num == 0 or skill_num == SK.KN_AUTOCOUNTER) and (sd or battle_config.enemy_critical)
    and not (tsd and battle.dcard(tsd, 4143)) then -- Orc Hero Card: never hit critically
    damage = damage + atkmax
    damage2 = damage2 + atkmax_
    type = 0x0a
  else
    local r = random(0, 0x7fff)
    if atkmax > atkmin then damage = damage + atkmin + r % (atkmax - atkmin) else damage = damage + atkmin end
    if atkmax_ > atkmin_ then damage2 = damage2 + atkmin_ + r % (atkmax_ - atkmin_) else damage2 = damage2 + atkmin_ end
    -- battle.c:1022 skills that multiply the attack
    local t = sc(src, SC.OVERTHRUST)
    if t then damage = damage + floor(damage * (5 * t.val1) / 100) end
    if skill_num > 0 then
      local pl = skill_get_pl(skill_num)
      if pl > 0 then s_ele, s_ele_ = pl, pl end
      flag = (flag & ~0xf00) | BF.SKILL
      local f = battle.weapon_skill[skill_num]
      if f then
        local r = f({ damage = damage, hitrate = hitrate, div = div, blewcount = blewcount, lv = skill_lv, wflag = wflag,
                      src = src, target = target, t_size = t_size, def1 = def1, def2 = def2, s_ele = s_ele, s_ele_ = s_ele_ })
        damage, hitrate, div, blewcount, s_ele, s_ele_ = r.damage, r.hitrate, r.div, r.blewcount, r.s_ele, r.s_ele_
      end
    end
    if skill_num ~= SK.NPC_CRITICALSLASH and skill_num ~= SK.KN_AUTOCOUNTER then
      -- battle.c:1180 defence: equipment DEF in percent, then VIT with its random part
      if skill_num ~= SK.MO_INVESTIGATE and skill_num ~= SK.MO_EXTREMITYFIST then
        local t_vit = def2
        if src.kind == "mob" and (s_race == 1 or s_race == 6) and tsd then
          t_vit = t_vit + checkskill(tsd, SK.AL_DP) * 3 -- Divine Protection
        end
        local v20 = floor(t_vit / 20)
        local vitbonusmax = v20 * v20 - 1
        damage = floor(damage * (100 - def1) / 100) - t_vit - (vitbonusmax < 1 and 0 or rnd(vitbonusmax))
        damage2 = floor(damage2 * (100 - def1) / 100) - t_vit - (vitbonusmax < 1 and 0 or rnd(vitbonusmax))
      end
    end
  end
  -- battle.c:1194 refine damage goes past DEF
  if skill_num ~= SK.MO_INVESTIGATE and skill_num ~= SK.MO_EXTREMITYFIST and sd then
    damage = damage + battle.atk2(src) - st.watk_2
    damage2 = damage2 + st.watk_2
  end
  if da == 2 then
    type, div = 0x08, 255
    damage = floor(damage * (100 + 20 * checkskill(sd, SK.MO_TRIPLEATTACK)) / 100)
  end
  -- battle.c:1206 random bonus of over-refined weapons
  if sd then
    if st.overrefine - st.overrefine_ > 0 then damage = damage + rnd(st.overrefine - st.overrefine_) + 1 end
    if st.overrefine_ > 0 then damage2 = damage2 + rnd(st.overrefine_) + 1 end
  end
  if damage < 1 then damage = 1 end
  if damage2 < 1 then damage2 = 0 end

  -- battle.c:1217 masteries (right hand)
  if skill_num ~= SK.MO_INVESTIGATE and skill_num ~= SK.MO_EXTREMITYFIST and sd then
    damage = battle.addmastery(sd, target, damage)
  end

  -- battle.c:1224 hit or miss; players always have 5%, monsters between 5% and 95%
  if hitrate < 100000 then
    if sd then hitrate = math.max(5, hitrate) else hitrate = math.max(5, math.min(95, hitrate)) end
  end
  if skill_num == SK.NPC_GUIDEDATTACK or skill_num == SK.RG_BACKSTAP or skill_num == SK.KN_AUTOCOUNTER or hitrate >= 100000
    or sc(target, SC.SLEEP) or sc(target, SC.STAN) or sc(target, SC.FREEZE) then
    hitrate = 100
  end
  if not (rnd(100) < hitrate) and type == 0 then damage, damage2 = 0, 0 end

  -- battle.c:1240 Weaponry Research, then cards
  -- (mod0631 bug: it added these, and the Star Crumbs below, to a miss as well)
  if sd and checkskill(sd, SK.BS_WEAPONRESEARCH) > 0 and damage > 0 then
    damage = damage + checkskill(sd, SK.BS_WEAPONRESEARCH) * 2
  end
  if sd then
    local cardfix = 100
    local t_ele = battle.elem_type(target)
    cardfix = floor(cardfix * (100 + st.addrace[t_race]) / 100)
    cardfix = floor(cardfix * (100 + st.addele[t_ele]) / 100)
    cardfix = floor(cardfix * (100 + st.addsize[t_size]) / 100)
    if battle.dcard(sd, 4140) and (t_mode & 0x20) ~= 0 then cardfix = floor(cardfix * 125 / 100) end -- Abysmal Knight Card
    damage = floor(damage * cardfix / 100)
  end
  if tsd then
    local cardfix = 100
    cardfix = floor(cardfix * (100 - tsd.st.subrace[s_race]) / 100)
    cardfix = floor(cardfix * (100 - tsd.st.subele[s_ele]) / 100)
    if battle.dcard(tsd, 4045) and (flag & BF.LONG) ~= 0 then cardfix = floor(cardfix * 75 / 100) end -- Horn Card
    damage = floor(damage * cardfix / 100)
  end
  -- battle.c:1261 element
  damage = battle.attr_fix(damage, s_ele, battle.element(target))
  damage2 = battle.attr_fix(damage2, s_ele_, battle.element(target))
  if tsd and battle.attack_element(src) == 3 and checkskill(tsd, SK.BS_SKINTEMPER) > 0 then
    damage = floor(damage * (100 - checkskill(tsd, SK.BS_SKINTEMPER) * 5) / 100)
  end
  -- battle.c:1271 Star Crumbs
  if sd and damage > 0 then
    damage = damage + st.star - st.star_
    damage2 = damage2 + st.star_
  end

  if sd then
    -- battle.c:1286 two weapons: each hand at its mastery's share
    if st.weapon > 16 and sd.char.class == 12 and checkskill(sd, SK.AS_LEFT) > 0 then
      damage = floor(damage * (50 + checkskill(sd, SK.AS_RIGHT) * 10) / 100)
      damage2 = floor(damage2 * (30 + checkskill(sd, SK.AS_LEFT) * 10) / 100)
      if damage2 < 1 then damage2 = 1 end
    else
      damage2 = 0
    end
    if da == 1 then
      div, type = 2, 0x08
      damage = damage * 2
    end
    if st.weapontype1 == 16 and damage > 0 then
      -- Katar: an extra hit of 1% + 2% per Double Attack level
      damage2 = floor(damage * (1 + checkskill(sd, SK.TF_DOUBLE) * 2) / 100)
      if damage2 < 1 then damage2 = 1 end
    end
  end
  if skill_num == SK.TF_POISON then damage = damage + floor(damage * 15 * skill_lv / 100) end

  -- battle.c:1336 perfect dodge
  if skill_num == 0 and tsd and rnd(100) < tsd.st.flee2 then
    damage, damage2 = 0, 0
    type = 0x0b
  end

  if damage2 < 1 then
    damage = battle.calc_damage(target, damage, flag)
  elseif damage == 0 then
    damage2 = battle.calc_damage(target, damage2, flag)
  else
    local d1, d2 = damage + damage2, damage2
    damage = battle.calc_damage(target, damage + damage2, flag)
    damage2 = floor(damage * d2 / d1)
    if damage2 <= 0 then damage2 = -1 end
    damage = damage - damage2
  end
  -- plants and the like (DEF 10000+): one point a hit
  if def1 >= 10000 then
    if damage > 0 then damage = 1 end
    if damage2 > 0 then damage2 = 1 end
  end
  return { damage = damage, damage2 = damage2, type = type, div = div, amotion = battle.amotion(src),
           dmotion = battle.dmotion(target), blewcount = blewcount, flag = flag }
end

-- battle.c:1030: what each weapon skill does to the base damage.
-- Each gets w = {damage, hitrate, div, blewcount, lv, wflag, src, target, t_size, def1, def2, s_ele, s_ele_}.
local function pct(w, p) w.damage = floor(w.damage * p / 100); return w end
battle.weapon_skill = {
  [SK.SM_BASH] = function(w) w.hitrate = w.hitrate + (w.lv < 6 and 10 or 20); return pct(w, 100 + 30 * w.lv) end,
  -- mod0631 bug: "5*lv + (wflag)?65:115" always gave 65%; meant: 115%+5%/level at the centre, 65% off it
  [SK.SM_MAGNUM] = function(w) w.blewcount = 2; return pct(w, 5 * w.lv + (w.wflag > 0 and 65 or 115)) end,
  [SK.MC_MAMMONITE] = function(w) return pct(w, 100 + 50 * w.lv) end,
  [SK.AC_DOUBLE] = function(w) w.div = 2; return pct(w, 180 + 20 * w.lv) end,
  [SK.AC_SHOWER] = function(w) w.blewcount = 2; return pct(w, 75 + 5 * w.lv) end,
  [SK.KN_PIERCE] = function(w)
    pct(w, 100 + 10 * w.lv)
    w.hitrate = floor(w.hitrate * (100 + 5 * w.lv) / 100)
    w.div = w.t_size + 1
    w.damage = w.damage * w.div
    return w
  end,
  [SK.KN_SPEARSTAB] = function(w) w.blewcount = 6; return pct(w, 100 + 15 * w.lv) end,
  [SK.KN_SPEARBOOMERANG] = function(w) return pct(w, 100 + 50 * w.lv) end,
  [SK.KN_BOWLINGBASH] = function(w) return pct(w, 100 + 50 * w.lv) end,
  -- Brandish Spear is not in mod0631's battle.c. eAthena 817 battle.c:1834:
  -- 100% + 20% per level, and more for the cells nearer the knight (wflag 1..3 = the row)
  [SK.KN_BRANDISHSPEAR] = function(w)
    pct(w, 100 + 20 * w.lv)
    local d, extra = w.damage, 0
    if w.lv > 3 and w.wflag == 1 then extra = extra + floor(d / 2) end
    if w.lv > 6 and w.wflag == 1 then extra = extra + floor(d / 4) end
    if w.lv > 9 and w.wflag == 1 then extra = extra + floor(d / 8) end
    if w.lv > 6 and w.wflag == 2 then extra = extra + floor(d / 2) end
    if w.lv > 9 and w.wflag == 2 then extra = extra + floor(d / 4) end
    if w.lv > 9 and w.wflag == 3 then extra = extra + floor(d / 2) end
    w.damage = d + extra
    return w
  end,
  -- Venom Splasher: 817 battle.c:1855 (not in mod0631)
  [SK.AS_SPLASHER] = function(w) w.div = 6; return pct(w, 200 + 20 * w.lv) end,
  [SK.AS_SONICBLOW] = function(w) w.div = 8; return pct(w, 300 + 50 * w.lv) end,
  [SK.AC_CHARGEARROW] = function(w) w.blewcount = 6; return w end,
  [SK.TF_SPRINKLESAND] = function(w) return pct(w, 125) end,
  [SK.MC_CARTREVOLUTION] = function(w)
    w.blewcount = 2
    local st = w.src.st
    if st and st.cart_weight > 0 then
      return pct(w, 150 + battle.pc_checkskill(w.src, SK.BS_WEAPONRESEARCH) + floor(st.cart_weight * 100 / 80000))
    end
    return pct(w, 150)
  end,
  [SK.NPC_COMBOATTACK] = function(w) w.div = skill_get_num(SK.NPC_COMBOATTACK, w.lv); w.damage = w.damage * w.div; return w end,
  [SK.NPC_RANDOMATTACK] = function(w) return pct(w, 50 + rnd(150)) end,
  [SK.RG_BACKSTAP] = function(w) return pct(w, 300 + 40 * w.lv) end,
  [SK.RG_RAID] = function(w) return pct(w, 100 + 40 * w.lv) end,
  [SK.RG_INTIMIDATE] = function(w) return pct(w, 100 + 30 * w.lv) end,
  [SK.CR_SHIELDCHARGE] = function(w) w.blewcount = 4 + w.lv; return pct(w, 100 + 20 * w.lv) end,
  [SK.CR_SHIELDBOOMERANG] = function(w) return pct(w, 100 + 30 * w.lv) end,
  [SK.CR_HOLYCROSS] = function(w) w.div = 2; return pct(w, 100 + 35 * w.lv) end,
  [SK.CR_GRANDCROSS] = function(w) return pct(w, 100 + 40 * w.lv) end,
  [SK.MO_INVESTIGATE] = function(w)
    w.damage = floor(floor(w.damage * (100 + 75 * w.lv) / 100) * (w.def1 + w.def2) / 100)
    w.hitrate, w.s_ele, w.s_ele_ = 100000, 0, 0
    return w
  end,
  [SK.MO_CHAINCOMBO] = function(w) w.div = 4; return pct(w, 150 + 50 * w.lv) end,
  [SK.MO_COMBOFINISH] = function(w) w.blewcount = 3; return pct(w, 240 + 60 * w.lv) end,
  [SK.BA_MUSICALSTRIKE] = function(w) return pct(w, 100 + 50 * w.lv) end,
  [SK.BA_DISSONANCE] = function(w) w.damage = w.lv * 20 + 30; return w end,
  [SK.DC_THROWARROW] = function(w) return pct(w, 100 + 50 * w.lv) end,
}
-- the monsters' element attacks: 100% + 25% per level ("for now", battle.c:1092)
for _, name in ipairs({ "NPC_WATERATTACK", "NPC_GROUNDATTACK", "NPC_FIREATTACK", "NPC_WINDATTACK", "NPC_POISONATTACK",
                        "NPC_HOLYATTACK", "NPC_DARKNESSATTACK", "NPC_TELEKINESISATTACK" }) do
  battle.weapon_skill[SK[name]] = function(w) return pct(w, 100 + 25 * w.lv) end
end
battle.weapon_skill[-1] = nil

-- ------------------------------------------------------------ magic

-- skill.h:86 skill_calc_heal
function battle.calc_heal(bl, skill_lv)
  return floor((battle.lv(bl) + battle.int(bl)) / 8) * (4 + skill_lv * 8)
end

-- battle.c:1393 battle_calc_magic_attack. `flag`: targets sharing the damage
-- (Napalm Beat) or distance from the centre (Fire Ball).
function battle.calc_magic_attack(bl, target, skill_num, skill_lv, flag)
  local int_ = battle.int(bl)
  local mdef1, mdef2 = battle.mdef(target), battle.mdef2(target)
  local i5, i7 = floor(int_ / 5), floor(int_ / 7)
  local matk1, matk2 = int_ + i5 * i5, int_ + i7 * i7
  local damage, div, blewcount = 0, 1, 0
  local normal = true
  local function fix(a, b) matk1, matk2 = floor(matk1 * a / b), floor(matk2 * a / b) end
  if bl.kind == "player" then
    matk1, matk2 = bl.st.matk1, bl.st.matk2
    if bl.st.weapontype1 == 0x0a then fix(115, 100) end -- a rod: +15%
  end
  local aflag = BF.MAGIC | BF.LONG | BF.SKILL

  if skill_num == SK.AL_HEAL or skill_num == SK.PR_BENEDICTIO then
    damage = floor(battle.calc_heal(bl, skill_lv) / 2)
    normal = false
  elseif skill_num == SK.PR_SANCTUARY then
    damage = skill_lv > 6 and 388 or skill_lv * 50
    normal = false
    blewcount = 3 | 0x10000
  elseif skill_num == SK.ALL_RESURRECTION or skill_num == SK.PR_TURNUNDEAD then
    if battle.elem_type(target) == 9 then
      local hp, mhp = battle.hp(target), battle.max_hp(target)
      local thres = skill_lv * 20 + battle.luk(bl) + battle.int(bl) + battle.lv(bl) + (200 - floor(hp * 200 / mhp))
      if rnd(1000) < thres and battle.mexp(target) == 0 then
        damage = hp
      else
        damage = battle.lv(bl) + battle.int(bl) + skill_lv * 10
      end
    end
    normal = false
  elseif skill_num == SK.MG_NAPALMBEAT then
    fix(70 + skill_lv * 10, 100)
    if flag > 0 then fix(1, flag) end
  elseif skill_num == SK.MG_FIREBALL then
    local drate = { [0] = 100, 90, 70 }
    if flag > 2 then matk1, matk2 = 0, 0 else fix((95 + skill_lv * 5) * drate[flag], 10000) end
  elseif skill_num == SK.MG_FIREWALL then
    local ele = battle.elem_type(target)
    if ele ~= 3 and ele ~= 9 then blewcount = 2 | 0x10000 end
    fix(1, 2)
  elseif skill_num == SK.MG_THUNDERSTORM then fix(80, 100)
  elseif skill_num == SK.MG_FROSTDIVER then fix(100 + skill_lv * 10, 100)
  elseif skill_num == SK.WZ_FIREPILLAR then
    mdef1, mdef2 = 0, 0
    fix(1, 5)
    matk1, matk2 = matk1 + 50, matk2 + 50
  elseif skill_num == SK.WZ_JUPITEL then blewcount = skill_lv > 6 and 6 or skill_lv
  elseif skill_num == SK.WZ_VERMILION then fix(skill_lv * 20 + 80, 100)
  elseif skill_num == SK.WZ_WATERBALL then matk1, matk2 = matk1 + skill_lv * 30, matk2 + skill_lv * 30
  elseif skill_num == SK.WZ_STORMGUST then
    fix(skill_lv * 40 + 100, 100)
    blewcount = 2 | 0x10000
  elseif skill_num == SK.AL_HOLYLIGHT then fix(125, 100)
  elseif skill_num == SK.AL_RUWACH then fix(145, 100) -- eAthena 817 battle.c:3089 (mod0631: plain MATK)
  end

  if normal then
    if matk1 > matk2 then damage = matk2 + rnd(matk1 - matk2) else damage = matk2 end
    damage = floor(damage * (100 - mdef1) / 100) - mdef2
    if damage < 1 then damage = 1 end
  end
  damage = battle.attr_fix(damage, skill_get_pl(skill_num), battle.element(target))
  div = skill_get_num(skill_num, skill_lv)
  if div > 1 and skill_num ~= SK.WZ_VERMILION then damage = damage * div end
  if battle.dcard(target, 4128) then damage = 0 end -- Golden Thief Bug Card: no magic damage
  if target.kind == "player" and damage > 0 then
    -- cards that resist an element work on magic too: eAthena 817 battle.c:3143
    -- (mod0631 applies them to weapon attacks only)
    damage = floor(damage * (100 - target.st.subele[skill_get_pl(skill_num)]) / 100)
  end
  damage = battle.calc_damage(target, damage, aflag)
  if mdef1 >= 10000 and damage > 0 then damage = 1 end
  return { damage = damage, damage2 = 0, type = 0, div = div, amotion = battle.amotion(bl),
           dmotion = battle.dmotion(target), blewcount = blewcount, flag = aflag }
end

-- ------------------------------------------------------------ everything else

-- battle.c:1561 battle_calc_misc_attack: traps, the falcon, thrown stones
function battle.calc_misc_attack(bl, target, skill_num, skill_lv, flag)
  local int_, dex = battle.int(bl), battle.dex(bl)
  local damage, div, damagefix = 0, 1, true
  local aflag = BF.MISC | BF.LONG | BF.SKILL
  if skill_num == SK.HT_LANDMINE then damage = floor(skill_lv * (dex + 75) * (100 + int_) / 100)
  elseif skill_num == SK.HT_BLASTMINE then damage = floor(skill_lv * (floor(dex / 2) + 50) * (100 + int_) / 100)
  elseif skill_num == SK.HT_CLAYMORETRAP then damage = floor(skill_lv * (floor(dex / 2) + 75) * (100 + int_) / 100)
  elseif skill_num == SK.HT_BLITZBEAT then
    damage = (floor(dex / 10) + floor(int_ / 2) + battle.pc_checkskill(bl, SK.HT_STEELCROW) * 3 + 40) * 2
    if flag > 1 then damage = floor(damage / flag) end
  elseif skill_num == SK.TF_THROWSTONE then damage, damagefix = 30, false
  elseif skill_num == SK.NPC_SELFDESTRUCTION then damage, damagefix = battle.hp(bl) - (bl.id == target.id and 1 or 0), false
  elseif skill_num == SK.NPC_SMOKING then damage, damagefix = 3, false
  end
  if damagefix then
    if damage < 1 then damage = 1 end
    damage = battle.attr_fix(damage, skill_get_pl(skill_num), battle.element(target))
  end
  div = skill_get_num(skill_num, skill_lv)
  if div > 1 then damage = damage * div end
  damage = battle.calc_damage(target, damage, aflag)
  if (damage > 0 and damage < div) or (battle.def(target) >= 10000 and battle.mdef(target) >= 10000 and damage > 0) then
    damage = div
  end
  return { damage = damage, damage2 = 0, type = 0, div = div, amotion = battle.amotion(bl),
           dmotion = battle.dmotion(target), blewcount = 0, flag = aflag }
end

function battle.calc_attack(attack_type, bl, target, skill_num, skill_lv, flag)
  if attack_type == BF.WEAPON then return battle.calc_weapon_attack(bl, target, skill_num, skill_lv, flag) end
  if attack_type == BF.MAGIC then return battle.calc_magic_attack(bl, target, skill_num, skill_lv, flag) end
  return battle.calc_misc_attack(bl, target, skill_num, skill_lv, flag)
end

-- ------------------------------------------------------------ a plain attack

-- battle.c:1669 battle_weapon_attack. Called by the server when a player's or
-- a monster's attack comes due. Returns {adelay = ms until the next one}, or
-- false to stop attacking.
function rules.weapon_attack(a)
  local src, target = get(a.src), get(a.target)
  if not src or not target or src.dead or target.dead or src.map ~= target.map then return false end
  if not battle.is_enemy(src, target) then return false end
  local adelay
  if src.kind == "player" then
    adelay = src.st.aspd * 2 -- pc.c:2461
    -- a bow needs an arrow for every shot (battle.c:1687)
    if src.st.weapontype1 == 11 then
      if not src.st.ammo then
        _rt("pc_ammo_use", src.id) -- tells the client "equip arrows first"
        return false
      end
    end
  else
    -- mob.c:190 mob_get_adelay
    adelay = src.db.attack_delay
    if sc(src, SC.ADRENALINE) then adelay = floor(adelay * 70 / 100) end
    if sc(src, SC.TWOHANDQUICKEN) then adelay = floor(adelay * 70 / 100) end
  end
  -- Auto Counter (not in mod0631; eAthena 817 battle.c:1698, 3403): a blow
  -- from the front does nothing and is answered with a certain critical;
  -- from behind it lands as a critical itself.
  local counter, opt
  local ac = sc(target, SC.AUTOCOUNTER)
  if ac then
    local dist = battle.distance(src, target)
    if dist <= 0 or battle.check_dir(battle.calc_dir(src, target.x, target.y), target.dir) then
      local reach = target.kind == "player" and target.st.attackrange or target.db.range
      local bow = target.kind == "player" and target.st.weapon == 11
      counter = not sc(src, SC.AUTOCOUNTER) and not bow and dist <= reach + 1
      _rt("clif_damage", src.id, target.id, battle.amotion(src), battle.dmotion(target), 0, 1, 0, 0)
      skill_status_change_end(target.id, SC.AUTOCOUNTER)
      if counter then
        _rt("bl_set", target.id, "dir", battle.calc_dir(target, src.x, src.y))
        local cd = battle.calc_weapon_attack(target, src, SK.KN_AUTOCOUNTER, ac.val1, 0, { sure_crit = true })
        _rt("clif_damage", target.id, src.id, cd.amotion, cd.dmotion, cd.damage, cd.div, cd.type, math.max(0, cd.damage2))
        battle.damage(target.id, src.id, cd.damage + math.max(0, cd.damage2))
      end
      return { adelay = adelay }
    end
    opt = { sure_crit = true }
  end
  local wd = battle.calc_weapon_attack(src, target, 0, 0, 0, opt)
  if src.kind == "player" and src.st.weapontype1 == 11 then _rt("pc_ammo_use", src.id) end
  local div = wd.div == 255 and 3 or wd.div
  _rt("clif_damage", src.id, target.id, wd.amotion, wd.dmotion, wd.damage, div, wd.type, math.max(0, wd.damage2))
  local total = wd.damage + math.max(0, wd.damage2)
  battle.damage(src.id, target.id, total)
  if total > 0 then skill_additional_effect(src.id, target.id, 0, 0) end
  return { adelay = adelay }
end

-- Character derived stats. Server-side formulas: NOT in the client.
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/pc.c pc_calcstatus,
-- archived 2003-12-18 (research/notes/emulators.md). Line refs are to that file.
--
-- rules.status(c) gets: c.class, c.base_level, c.job_level, c.str .. c.luk,
--   c.job = {weight, hp_coef, sp_coef, aspd_base[17]} (data/db/jobs.toml),
--   c.equip = {weight_atk, weapon_type, weapon_range, armor_def, refine_atk, items}
-- and returns the fields of the status window (see crates/ro-game/src/status.rs).

local floor = math.floor

function rules.status(c)
  local job = c.job or { weight = 20000, hp_coef = 0, sp_coef = 10, aspd_base = {} }
  local lv = c.base_level
  local e = c.equip or {}
  local s = {}

  -- pc.c:970  max_hp = ((35 + lv*5) + (1+lv)*lv/2 * hp_coef) * (1 + vit/100), cap 32000
  local hp = ((35 + lv * 5) + floor((1 + lv) * lv / 2) * job.hp_coef) * (1 + c.vit / 100)
  s.max_hp = math.min(32000, math.max(1, floor(hp)))
  -- pc.c:979  max_sp = ((sp_coef * lv / 10) + 10) * (100 + int) / 100
  s.max_sp = math.max(1, floor((floor(job.sp_coef * lv / 10) + 10) * (100 + c.int) / 100))

  -- battle.c:904 (shown as the left ATK number) base atk = str + (str/10)^2 + dex/5 + luk/5
  -- bows (weapon type 11) swap str and dex
  local str, dex, luk = c.str, c.dex, c.luk
  if e.weapon_type == 11 then str, dex = dex, str end
  local s10 = floor(str / 10)
  s.atk = str + s10 * s10 + floor(dex / 5) + floor(luk / 5)
  s.atk2 = (e.weapon_atk or 0) + (e.refine_atk or 0)

  -- pc.c:914-921
  local i5, i7 = floor(c.int / 5), floor(c.int / 7)
  s.matk_max = c.int + i5 * i5
  s.matk_min = c.int + i7 * i7
  s.hit = c.dex + lv
  s.flee = c.agi + lv
  s.def = e.armor_def or 0
  s.def2 = c.vit
  s.mdef = 0
  s.mdef2 = c.int
  s.flee2 = floor(c.luk / 10) + 1
  s.crit = floor(c.luk * 3 / 10) + 1        -- tenths of a percent (shown /10)

  -- pc.c:927  aspd = base - (agi*4 + dex) * base / 1000, floor 100 (= ASPD 190)
  local base = job.aspd_base[(e.weapon_type or 0) + 1] or 2000
  local aspd = base - floor((c.agi * 4 + c.dex) * base / 1000)
  s.amotion = math.max(100, aspd)

  -- pc.c:690  max_weight = job base + str*300 (tenths)
  s.max_weight = job.weight + c.str * 300
  s.range = e.weapon_range or 1
  return s
end

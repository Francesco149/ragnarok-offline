-- Melee auto-attack, death, regen. Server-side: NOT in the client.
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/battle.c
-- battle_calc_weapon_attack and map/pc.c natural heal, archived 2003-12-18.
-- Not yet modelled (docs/ROADMAP.md): element table (attr_fix), size
-- modifiers (atkmods), cards, double attack, status effects.

local floor = math.floor
local function rnd(a, b) if b <= a then return a end return math.random(a, b) end

-- Monster "status" in the same shape as a player's derived stats.
local function mob_stats(m)
  return {
    str = m.str, dex = m.dex, luk = m.luk, vit = m.vit,
    hit = m.level + m.dex, flee = m.level + m.agi,   -- battle.c battle_get_hit/flee (mobs)
    def = m.def, def2 = m.vit, atk1 = m.atk1, atk2 = m.atk2,
  }
end

-- attacker/defender: {kind="player", char=..., derived=...} or {kind="mob", mob=<mobs.toml row>}
function rules.attack(t)
  local a, d = t.attacker, t.defender
  local as = a.kind == "player" and a.derived or mob_stats(a.mob)
  local ds = d.kind == "player" and d.derived or mob_stats(d.mob)
  local aluk = a.kind == "player" and a.char.luk or a.mob.luk
  local dluk = d.kind == "player" and d.char.luk or d.mob.luk

  -- battle.c:904/938 base damage + random weapon part
  local dmg
  if a.kind == "player" then
    local c = a.char
    local atkmin, atkmax = c.dex, math.max(as.atk2, 0)
    if atkmin > atkmax then atkmin = atkmax end
    dmg = as.atk + rnd(atkmin, atkmax)
    -- battle.c:972 critical: (1 + luk*3) per mille, minus target luk*3; adds atkmax, skips DEF/FLEE
    local cri = 1 + aluk * 3 - dluk * 3
    if math.random(0, 999) < cri then
      return { damage = math.max(1, dmg + atkmax), kind = "crit" }
    end
  else
    local m = a.mob
    local s10 = floor(m.str / 10)
    dmg = s10 * s10 + m.str + rnd(m.atk1, math.max(m.atk1, m.atk2))
  end

  -- battle.c:895 hit chance, clamped 5..95 %
  local hitrate = math.max(5, math.min(95, as.hit - ds.flee + 80))
  if math.random(1, 100) > hitrate then
    return { damage = 0, kind = "miss" }
  end

  -- battle.c:1186 DEF: percentage (equip def) then flat VIT part with a random bonus
  local vit = ds.def2 or 0
  local v20 = floor(vit / 20)
  local vbm = v20 * v20 - 1
  dmg = floor(dmg * (100 - (ds.def or 0)) / 100) - vit - (vbm >= 1 and math.random(0, vbm - 1) or 0)
  return { damage = math.max(1, dmg), kind = "hit" }
end

-- battle_athena.conf (mod0631): death_penalty_base/job 100 = 1% of the
-- EXP needed for the next level; novices lose nothing.
function rules.death_penalty(t)
  if t.class == 0 then return { base = 0, job = 0 } end
  return { base = floor(t.next_base / 100), job = floor(t.next_job / 100) }
end

-- restart_hp_rate 0 (mod0631) -> 1 HP after "return to save point".
function rules.respawn_hp(t) return 1 end

-- pc.c pc_natural_heal_hp/sp: every hp_interval ms of standing still
-- (sitting doubles the clock) heal 1 + vit/6 + max_hp/200 HP; SP every
-- sp_interval ms: 1 + int/6 + max_sp/100 (pc.c:3940; the INT>=120 bonus of
-- later eAthena is not in this version).
function rules.regen(t)
  local sp = 1 + floor(t.int / 6) + floor(t.max_sp / 100)
  return {
    hp = 1 + floor(t.vit / 6) + floor(t.max_hp / 200), hp_interval = 4000,
    sp = sp, sp_interval = 8000,
    sit_factor = 2,
  }
end

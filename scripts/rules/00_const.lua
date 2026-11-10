-- Names the battle and skill rules share. Loaded first (file order).
--
-- SC: status-change numbers of athena-dev-2.1.1-mod0631 map/skill.h (archived
-- 2003-12-18). Below 64 the number is also the icon the client shows (0x0196),
-- so those are the client's own; 64 and up are the emulator's bookkeeping.
-- SLOWPOISON = 14 is eAthena 817's (2004-02-19); mod0631 had no Slow Poison.
-- SK: skill ids by Aegis name, from data/db/skills.toml (the server hands it
-- over as SKILL_DB). The client keys its own skill tables by the same names.

SC = {
  STONE = 64, FREEZE = 65, STAN = 66, SLEEP = 67, POISON = 68, CURSE = 69, SILENCE = 70,
  CONFUSION = 71, BLIND = 72,
  PROVOKE = 0, ENDURE = 1, TWOHANDQUICKEN = 2, CONCENTRATE = 3, HIDDING = 4, CLOAKING = 5,
  ENCPOISON = 6, POISONREACT = 7, QUAGMIRE = 8, ANGELUS = 9, BLESSING = 10, SIGNUMCRUCIS = 11,
  INCREASEAGI = 12, DECREASEAGI = 13, SLOWPOISON = 14, IMPOSITIO = 15, SUFFRAGIUM = 16, ASPERSIO = 17,
  BENEDICTIO = 18, KYRIE = 19, MAGNIFICAT = 20, GLORIA = 21, AETERNA = 22, ADRENALINE = 23,
  WEAPONPERFECTION = 24, OVERTHRUST = 25, MAXIMIZEPOWER = 26, RIDING = 27, FALCON = 28,
  TRICKDEAD = 29, LOUD = 30, ENERGYCOAT = 31, WEIGHT50 = 35, WEIGHT90 = 36,
  SPEEDPOTION0 = 37, SPEEDPOTION1 = 38, SPEEDPOTION2 = 39,
  STRIPWEAPON = 50, STRIPSHIELD = 51, STRIPARMOR = 52, STRIPHELM = 53,
  CP_WEAPON = 54, CP_SHIELD = 55, CP_ARMOR = 56, CP_HELM = 57,
  AUTOGUARD = 58, REFLECTSHIELD = 59, DEVOTION = 60, PROVIDENCE = 61, DEFENDER = 62,
  SIGHT = 75, RUWACH = 76, AUTOCOUNTER = 77,
  DRUMBATTLE = 78, SIEGFRIED = 79, WHISTLE = 80, APPLEIDUN = 81, FORTUNE = 82, SERVICE4U = 83,
  CASTCANCEL = 84, VOLCANO = 85, EXPLOSIONSPIRITS = 86, STEELBODY = 87, SPELLBREAKER = 88, DELUGE = 89,
  SAFETYWALL = 90, PNEUMA = 91, WATERBALL = 92, ANKLE = 94,
  FROSTWEAPON = 95, LIGHTNINGLOADER = 96, SEISMICWEAPON = 97, FLAMELAUNCHER = 98, SPEARSQUICKEN = 99,
  ASSNCROS = 100, POEMBRAGI = 101, NIBELUNGEN = 103, HUMMING = 104, ETERNALCHAOS = 105, DONTFORGETME = 106,
  -- ours (not in mod0631): bookkeeping for skills it left out
  SPLASHER = 110,     -- Venom Splasher counting down (817)
}
SC.DIVINA = SC.SILENCE

-- Skill database by id; SK.<NAME> = id.
skill_db = {}
SK = {}
for _, s in ipairs(SKILL_DB or {}) do
  skill_db[s.id] = s
  SK[s.name] = s.id
end
-- a missing name reads as an id nothing has, so tables keyed by it stay valid
setmetatable(SK, { __index = function(_, k) return -1 end })

local function per_level(list, lv, default)
  if type(list) ~= "table" or #list == 0 then return default end
  return list[math.max(1, math.min(lv, #list))] or default
end

function skill_get(id) return skill_db[id] or { id = id, name = "?", max = 0, inf = 0, range = 0, hit = 6, pl = 0, nk = 0 } end
function skill_get_range(id) return skill_get(id).range end
function skill_get_hit(id) return skill_get(id).hit end
function skill_get_inf(id) return skill_get(id).inf end
function skill_get_pl(id) return skill_get(id).pl end
function skill_get_nk(id) return skill_get(id).nk end
function skill_get_sp(id, lv) return per_level(skill_get(id).sp, lv, 0) end
function skill_get_num(id, lv) return per_level(skill_get(id).num, lv, 1) end
function skill_get_cast(id, lv) return per_level(skill_get(id).cast, lv, 0) end
function skill_get_delay(id, lv) return per_level(skill_get(id).delay, lv, 0) end
-- durations from eAthena 817's cast_db (2004-02-19): the skill itself, its side effect
function skill_get_time(id, lv) return per_level(skill_get(id).time, lv, 0) end
function skill_get_time2(id, lv) return per_level(skill_get(id).time2, lv, 0) end

-- attack kinds and ranges (battle.h BF_*), target filters (BCT_*)
BF = { WEAPON = 0x1, MAGIC = 0x2, MISC = 0x4, SHORT = 0x10, LONG = 0x40, SKILL = 0x100, NORMAL = 0x200 }
BCT = { NOENEMY = "noenemy", ENEMY = "enemy", ALL = "all", PARTY = "party" }

-- elements: 0 neutral 1 water 2 earth 3 fire 4 wind 5 poison 6 holy 7 dark 8 ghost 9 undead
-- races: 0 formless 1 undead 2 brute 3 plant 4 insect 5 fish 6 demon 7 demi-human 8 angel 9 dragon
-- sizes: 0 small 1 medium 2 large
-- monster mode bits: 1 can move, 2 looter, 4 aggressive, 8 assist, 0x10 cast sensor, 0x20 boss, 0x40 plant, 0x80 can attack

-- battle_athena.conf of mod0631 (the values the emulator shipped with)
battle_config = {
  enemy_critical = true,          -- monsters land criticals
  enemy_critical_rate = 100,
  cast_rate = 100,
  delay_rate = 100,
  delay_dependon_dex = false,
  pc_skill_add_range = 0,
  skill_out_range_consume = true, -- out of range at cast end: SP and items are spent anyway
  defnotenemy = true,             -- Safety Wall and Pneuma do not shelter the enemy
  skill_min_damage = false,
  sanctuary_type = 0,
  heal_exp = 0,
}

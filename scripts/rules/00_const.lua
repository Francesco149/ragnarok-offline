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
  TRICKDEAD = 29, LOUD = 30, ENERGYCOAT = 31, HALLUCINATION = 34, WEIGHT50 = 35, WEIGHT90 = 36, -- HALLUCINATION: jAthena //0850
  SPEEDPOTION0 = 37, SPEEDPOTION1 = 38, SPEEDPOTION2 = 39,
  -- a monster's armour or weapon broken by Acid Terror or Bomb: 817's skill.h numbers
  -- (commented out there: 817 and mod0631 break nothing). Monsters only, so no icon is sent
  BROKENARMOR = 32, BROKENWEAPON = 33,
  STRIPWEAPON = 50, STRIPSHIELD = 51, STRIPARMOR = 52, STRIPHELM = 53,
  CP_WEAPON = 54, CP_SHIELD = 55, CP_ARMOR = 56, CP_HELM = 57,
  AUTOGUARD = 58, REFLECTSHIELD = 59, DEVOTION = 60, PROVIDENCE = 61, DEFENDER = 62,
  SIGHT = 75, RUWACH = 76, AUTOCOUNTER = 77,
  DRUMBATTLE = 78, SIEGFRIED = 79, WHISTLE = 80, APPLEIDUN = 81, FORTUNE = 82, SERVICE4U = 83,
  CASTCANCEL = 84, VOLCANO = 85, EXPLOSIONSPIRITS = 86, STEELBODY = 87, SPELLBREAKER = 88, DELUGE = 89,
  SAFETYWALL = 90, PNEUMA = 91, WATERBALL = 92, ANKLE = 94,
  FROSTWEAPON = 95, LIGHTNINGLOADER = 96, SEISMICWEAPON = 97, FLAMELAUNCHER = 98, SPEARSQUICKEN = 99,
  ASSNCROS = 100, POEMBRAGI = 101, NIBELUNGEN = 103, HUMMING = 104, ETERNALCHAOS = 105, DONTFORGETME = 106,
  SPLASHER = 107, -- jAthena //1046's (2004-09-12): Venom Splasher's mark
  KEEPING = 145, -- jAthena //0850 (2004-02-29)
  -- ours: mod0631 has no number for them (jAthena 149, 155; 817 181, 182). Above 63 and not in
  -- battle.rs sc_icon, so no icon is sent: this client has none for them
  MAGICROD = 108, VIOLENTGALE = 109,
  -- ours too: a Bard or Dancer playing (jAthena's 144)
  DANCING = 110,
  -- ours too, the Monk's (monk.lua): jAthena //1020's 89, 156, 157, 158
  COMBO = 111, BLADESTOP_WAIT = 112, BLADESTOP = 113, EXTREMITYFIST = 114,
  -- ours too, standing in an ensemble (jAthena //1020's SC_RICHMANKIM, SC_ROKISWEIL, SC_INTOABYSS);
  -- War Drum, Ring of Nibelungen, Eternal Chaos and Siegfried keep mod0631's numbers above
  RICHMANKIM = 115, ROKISWEIL = 116, INTOABYSS = 117,
  -- ours too (sage.lua): Auto Spell (817's and jAthena's 103 is mod0631's Nibelungen), and the orc's
  -- face of Abracadabra's Reverse Orcish (battle.rs SC_ORCISH: option 0x0800)
  AUTOSPELL = 118, ORCISH = 119,
}
-- the Sage's fields and the element each strengthens
FIELD_ELEMENT = { [SC.VOLCANO] = 3, [SC.DELUGE] = 1, [SC.VIOLENTGALE] = 4 }
SC.DIVINA = SC.SILENCE

-- Skill database by id; SK.<NAME> = id.
skill_db = {}
SK = {}
for _, s in ipairs(SKILL_DB or {}) do
  skill_db[s.id] = s
  SK[s.name] = s.id
end
-- A missing name reads as an id nothing has, so tables keyed by it stay
-- valid; it is noted in SK_MISSING (a rule naming a skill the database
-- does not have is a mistake: the tests check that this stays empty).
SK_MISSING = {}
setmetatable(SK, { __index = function(_, k) SK_MISSING[k] = true; return -1 end })

local function per_level(list, lv, default)
  if type(list) ~= "table" or #list == 0 then return default end
  return list[math.max(1, math.min(lv, #list))] or default
end

function skill_get(id) return skill_db[id] or { id = id, name = "?", max = 0, inf = 0, range = 0, hit = 6, pl = 0, nk = 0 } end
function skill_get_range(id) return skill_get(id).range end
function skill_get_hit(id) return skill_get(id).hit end
-- Spell Breaker is aimed at an enemy (eAthena 817's and jAthena's skill_db); mod0631's row says ground
local INF_FIX = { [SK.SA_SPELLBREAKER] = 1 }
INF_FIX[-1] = nil
function skill_get_inf(id) return INF_FIX[id] or skill_get(id).inf end
-- Elements mod0631's skill_db leaves Neutral for skills it never built: Grand Cross is Holy
-- (eAthena 817's and jAthena's skill_db, the iRO guide's "holy attack"), Bomb Fire (jAthena's
-- skill_db, euRO's skill text of May 2004). B.S. Sacramenti is Neutral in every emulator's
-- skill_db (mod0631, 817, jAthena) but strikes as Holy in the iRO guide's Priest page
-- (2004-03-27: "damaged by holy property") and in Skygor's iRO guide (GameFAQs 28518, 5.A:
-- "BSS does holy damage, so the final result is multiplied by the appropriate elemental modifier")
-- Aspersio is Holy: RODataZone's Priest page (iRO, capture 2003-12-16: "40 points of Holy property
-- damage") and jAthena //1020's skill_db; mod0631's row says Neutral (it never let Aspersio strike)
local PL_FIX = { [SK.CR_GRANDCROSS] = 6, [SK.AM_DEMONSTRATION] = 3, [SK.PR_BENEDICTIO] = 6, [SK.PR_ASPERSIO] = 6 }
PL_FIX[-1] = nil
function skill_get_pl(id) return PL_FIX[id] or skill_get(id).pl end
function skill_get_nk(id) return skill_get(id).nk end
-- Service For You costs 40 to 85 SP: the iRO guide's Dancer page (2004-03-27) and jAthena's
-- skill_require_db (2004-05); mod0631's skill_db has Assassin Cross of Sunset's 38 to 65 on it.
-- Adaptation and Encore cost 1 SP (the guide; jAthena's skill_require_db): mod0631's row says 0.
-- War Drum (the guide's "Battle Theme") 40 to 60 SP, as the guide's table and jAthena's
-- skill_require_db; mod0631 has 38 to 50. Ring of Nibelungen the same (jAthena; the guide names none)
local SP_FIX = { [SK.DC_SERVICEFORYOU] = { 40, 45, 50, 55, 60, 65, 70, 75, 80, 85 },
                 [SK.BD_ADAPTATION] = { 1 }, [SK.BD_ENCORE] = { 1 },
                 [SK.BD_DRUMBATTLEFIELD] = { 40, 45, 50, 55, 60 }, [SK.BD_RINGNIBELUNGEN] = { 40, 45, 50, 55, 60 } }
SP_FIX[-1] = nil
function skill_get_sp(id, lv) return per_level(SP_FIX[id] or skill_get(id).sp, lv, 0) end
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
  -- Ours, for one player: a Bard's or Dancer's song touches the one playing it too. On the
  -- period servers it did not (jAthena skill.c:4528 `sg->src_id == bl->id`), so a lone Bard
  -- sang for nobody; rAthena's later Soul Link gave the singer the same (docs/DEVIATIONS.md)
  song_on_self = true,
}

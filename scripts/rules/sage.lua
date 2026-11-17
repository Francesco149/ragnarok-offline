-- The Sage's Auto Spell and Abracadabra, and the Crusader's Devotion. Drafts (the 2-2 jobs,
-- Juno, June 2004, are not in the default world; docs/DEVIATIONS.md). Server-side: NOT in
-- the client.
--
-- Source: jAthena of 2004-06-11 (git 2dd4988): src/map/skill.c skill_castend_nodamage_id
-- (SA_AUTOSPELL 3082, SA_ABRACADABRA 2156, the specials 2166-2232, CR_DEVOTION 2348),
-- skill_autospell (6184), skill_abra_dataset (6336), skill_status_change_start (7309);
-- battle.c battle_weapon_attack (4379); db/abra_db.txt (added by //0900, 2004-03-24). Auto
-- Spell came in //0821 (2004-02-20) and //0830, Abracadabra in //0749 (2004-01-27).
-- Numbers checked against the iRO guide's Sage page (capture 2004-03-27,
-- refs/web/iro-guide-2004/20040327034856_jobsagskill.txt, "Hindsight", "Hocus-pocus") and
-- euRO's skill text of May 2004 (mico07 yuno_lang/data/skilldesctable.txt:1736, 1830-1912).
--
-- This client offers none of it: its skill window has no 2-2 skill, it has no packet for Auto
-- Spell's list (0x01cd) and knows none of Abracadabra's own skills (THREADS.md #25).

local floor, random = math.floor, math.random
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end
local get = battle.get
local sc = battle.sc
local checkskill = battle.pc_checkskill

local function nodamage(src, bl, skill, val, ok) _rt("clif_skill_nodamage", src, bl, skill, val or 0, ok == false and 0 or 1) end

-- ------------------------------------------------------------ Auto Spell

-- What each level of Auto Spell offers, and up to which level of it: euRO's text of May 2004
-- (skilldesctable.txt:1741-1750), the same as jAthena's skill_autospell. The spell itself is
-- capped by the level the Sage has learned ("You cannot choose skill you didn't learn").
local function autospell_cap(skill, lv)
  if skill == SK.MG_NAPALMBEAT then return 3 end
  if skill == SK.MG_COLDBOLT or skill == SK.MG_FIREBOLT or skill == SK.MG_LIGHTNINGBOLT then
    return lv >= 2 and math.min(3, lv - 1) or 0
  end
  if skill == SK.MG_SOULSTRIKE then return lv >= 5 and math.min(3, lv - 4) or 0 end
  if skill == SK.MG_FIREBALL then return lv >= 8 and math.min(2, lv - 7) or 0 end
  if skill == SK.MG_FROSTDIVER then return lv >= 10 and 1 or 0 end
  return 0
end
AUTOSPELL_LIST = { "MG_NAPALMBEAT", "MG_COLDBOLT", "MG_FIREBOLT", "MG_LIGHTNINGBOLT", "MG_SOULSTRIKE",
                   "MG_FIREBALL", "MG_FROSTDIVER" }
-- the names this client gives them (data\skillnametable.txt)
SPELL_NAME = {
  MG_NAPALMBEAT = "Napalm Beat", MG_COLDBOLT = "Cold Bolt", MG_FIREBOLT = "Fire Bolt",
  MG_LIGHTNINGBOLT = "Lightning Bolt", MG_SOULSTRIKE = "Soul Strike", MG_FIREBALL = "Fire Ball",
  MG_FROSTDIVER = "Frost Diver",
}

-- The spells a Sage with Auto Spell at `lv` may choose: {skill, maxlv}, in the list's order.
function autospell_offer(sd, lv)
  local out = {}
  for _, n in ipairs(AUTOSPELL_LIST) do
    local cap = math.min(autospell_cap(SK[n], lv), checkskill(sd, SK[n]))
    if cap > 0 then out[#out + 1] = { skill = SK[n], name = n, maxlv = cap } end
  end
  return out
end

-- the spell each player asked for with @autospell (ours: this client has no list to pick from)
autospell_wanted = autospell_wanted or {}

-- Cast Auto Spell: jAthena sends the list (0x01cd) and starts the status when the player picks
-- (skill_autospell). This client has no such window, so the server picks: the spell asked for
-- with @autospell if the level offers it, else the last offered, the strongest (as jAthena's
-- monster Sages do, skill.c:3087), and says so in the chat (ours, DEVIATIONS.md).
function sage_autospell(src_id, skilllv)
  local sd = get(src_id)
  if not sd or sd.kind ~= "player" then return end
  local offer = autospell_offer(sd, skilllv)
  nodamage(src_id, src_id, SK.SA_AUTOSPELL, skilllv)
  if #offer == 0 then return end
  local pick = offer[#offer]
  for _, o in ipairs(offer) do
    if o.skill == autospell_wanted[src_id] then pick = o end
  end
  skill_status_change_start(src_id, SC.AUTOSPELL, skilllv, pick.skill, { val3 = pick.maxlv })
  local more = #offer > 1 and " (@autospell to choose another)" or ""
  _rt("clif_message", src_id, string.format("Auto Spell: %s, up to level %d%s", SPELL_NAME[pick.name], pick.maxlv, more))
end

-- A blow struck under Auto Spell, hit or miss (jAthena battle.c:4379): a chance of 7% to 16%
-- by level (the iRO guide's table; jAthena 5% + 2% a level, 7% to 25%) to cast the spell at
-- once, at its top level 15% of the time, one below 35%, two below 50% (the guide, jAthena),
-- never below 1, for two thirds of its SP (the guide, euRO's text), and only if the Sage has
-- them. Nothing is paid if the spell does not go off.
function sage_autospell_blow(src_id, bl_id)
  local src = get(src_id)
  local t = src and sc(src, SC.AUTOSPELL)
  if not t or rnd(100) >= 6 + t.val1 then return end
  local bl = get(bl_id)
  if not bl or bl.dead then return end
  local lv = t.val3
  local roll = rnd(100)
  if roll >= 50 then lv = lv - 2 elseif roll >= 15 then lv = lv - 1 end
  lv = math.max(1, lv)
  local spell = t.val2
  if src.kind == "player" then
    local sp = floor(skill_get_sp(spell, lv) * 2 / 3)
    if src.char.sp < sp then return end
    skill_castend_damage_id(src_id, bl_id, spell, lv, { auto = true })
    _rt("bl_set", src_id, "sp", math.max(0, get(src_id).char.sp - sp))
  else
    skill_castend_damage_id(src_id, bl_id, spell, lv, { auto = true })
  end
end

-- ------------------------------------------------------------ Abracadabra

-- jAthena's db/abra_db.txt (//0900, 2004-03-24): skill id -> {Abracadabra level it needs, chance
-- in 10000 that a draw of it stands}. Its authors' own table; no period source gives one. Kept
-- here: the skills this client can name (the first jobs, the 2-1 jobs, the quest skills) and
-- Abracadabra's own (291-303). jAthena's 2-2 rows are left out, as this client cannot lend them
-- (DEVIATIONS.md); its skills with chance 0 never come up.
ABRA_DB = {
  [1]={1,5000}, [2]={1,5000}, [3]={1,5000}, [4]={1,5000}, [5]={1,5000}, [6]={1,5000}, [7]={1,5000},
  [8]={1,5000}, [9]={1,5000}, [10]={1,5000}, [11]={1,5000}, [12]={4,5000}, [13]={2,5000}, [14]={1,5000},
  [15]={2,5000}, [16]={1,5000}, [17]={2,5000}, [18]={4,5000}, [19]={1,5000}, [20]={1,5000}, [21]={2,5000},
  [24]={1,5000}, [25]={6,5000}, [26]={2,5000}, [27]={4,5000}, [28]={1,5000}, [29]={2,5000}, [30]={4,5000},
  [31]={1,5000}, [32]={1,5000}, [33]={1,5000}, [34]={2,5000}, [35]={1,5000}, [40]={1,5000}, [41]={4,5000},
  [42]={1,5000}, [45]={4,5000}, [46]={1,5000}, [47]={2,5000}, [50]={2,5000}, [51]={2,5000}, [52]={1,5000},
  [53]={1,5000}, [54]={1,5000}, [56]={2,4000}, [57]={6,4000}, [58]={1,4000}, [59]={4,4000}, [60]={1,4000},
  [61]={2,4000}, [62]={6,4000}, [66]={1,4000}, [67]={2,4000}, [68]={2,4000}, [69]={4,4000}, [70]={2,4000},
  [71]={1,4000}, [72]={1,4000}, [73]={2,4000}, [74]={2,4000}, [75]={4,4000}, [76]={2,4000}, [77]={2,4000},
  [78]={4,4000}, [79]={6,4000}, [80]={2,4000}, [81]={2,4000}, [83]={6,4000}, [84]={2,4000}, [85]={6,4000},
  [86]={4,4000}, [87]={2,4000}, [88]={1,4000}, [89]={6,4000}, [90]={1,4000}, [91]={2,4000}, [92]={4,4000},
  [93]={1,4000}, [110]={1,4000}, [111]={2,4000}, [112]={4,4000}, [113]={4,4000}, [114]={6,4000},
  [115]={1,4000}, [116]={2,4000}, [117]={2,4000}, [118]={4,4000}, [119]={4,4000}, [120]={4,4000},
  [121]={4,4000}, [122]={4,4000}, [123]={6,4000}, [124]={1,4000}, [125]={1,4000}, [129]={4,4000},
  [130]={1,4000}, [131]={1,4000}, [135]={2,4000}, [136]={2,4000}, [137]={4,4000}, [138]={2,4000},
  [139]={2,4000}, [140]={4,4000}, [141]={4,40000}, [142]={1,5000}, [143]={1,5000}, [147]={1,5000},
  [148]={1,5000}, [149]={1,5000}, [150]={1,5000}, [151]={1,5000}, [152]={1,5000}, [153]={1,5000},
  [154]={1,5000}, [155]={1,5000}, [156]={1,5000}, [157]={1,5000},
  -- Abracadabra's own
  [291]={4,2500}, [292]={8,2500}, [293]={6,3500}, [294]={4,5000}, [295]={8,2000}, [296]={6,4000},
  [297]={6,4000}, [298]={4,5000}, [299]={4,5000}, [300]={10,500}, [301]={4,2000}, [302]={8,3000},
  [303]={4,4000},
}
-- Abracadabra's own skills, as euRO's May-2004 client names them (skillnametable.txt). The
-- skill database has no rows for them (no period emulator's skill_db reaches past 290)
ABRA_SPECIAL = {
  [291] = "Mono Cell", [292] = "Class Change", [293] = "Summon Monster", [294] = "Reverse Orcish",
  [295] = "Death", [296] = "Fortune", [297] = "Taming Monster", [298] = "?", [299] = "Gravity",
  [300] = "Level Up", [301] = "Instant Death", [302] = "Full Recovery", [303] = "Coma",
}
-- the bosses Class Change turns a monster into (jAthena skill.c:2002; euRO: "Alter target to
-- Boss monster")
local CLASS_CHANGE = { 1038, 1039, 1046, 1059, 1086, 1087, 1112, 1115, 1157, 1159, 1190, 1272, 1312, 1373, 1492 }

-- jAthena skill_abra_dataset: draw ids 0-330 until one stands (its Abracadabra level reached,
-- its chance rolled)
function abra_draw(skilllv)
  for _ = 1, 100000 do
    local id = rnd(331)
    local row = ABRA_DB[id]
    if row and row[1] <= skilllv and rnd(10000) < row[2] then return id end
  end
  return SK.MG_NAPALMBEAT
end

-- the nearest enemy monster within reach of a special (8 cells, jAthena's skill_db)
local function abra_target(sd)
  local best, bd
  for _, id in ipairs(_rt("in_area", sd.map, sd.x - 8, sd.y - 8, sd.x + 8, sd.y + 8)) do
    local m = get(id)
    if m and m.kind == "mob" and not m.dead and battle.is_enemy(sd, m) then
      local d = battle.distance(sd, m)
      if not bd or d < bd then best, bd = m, d end
    end
  end
  return best
end

-- One of Abracadabra's own skills. jAthena lends them like the others (the player aims them);
-- this client knows none of them, so they go off at once: on the Sage, or on the nearest
-- enemy monster for those euRO's text aims at "1 Enemy" (none near: nothing happens). What
-- each does is jAthena's (skill.c:2166-2232) unless the text says otherwise. Ours, DEVIATIONS.md.
function abra_special(src_id, id)
  local sd = get(src_id)
  if not sd then return end
  _rt("clif_message", src_id, "Abracadabra: " .. ABRA_SPECIAL[id])
  local on_enemy = id == 291 or id == 292 or id == 295 or id == 296 or id == 297 or id == 298
  local bl = on_enemy and abra_target(sd) or nil
  if on_enemy and not bl then
    _rt("clif_skill_fail", src_id, SK.SA_ABRACADABRA, 0, 0)
    return
  end
  local target = bl and bl.id or src_id
  nodamage(src_id, target, id, 1)
  if id == 291 then
    _rt("mob_class_change", bl.id, 1002) -- Mono Cell: a Poring
  elseif id == 292 then
    _rt("mob_class_change", bl.id, CLASS_CHANGE[rnd(#CLASS_CHANGE) + 1])
  elseif id == 293 then
    _rt("dead_branch", src_id) -- "Same as Dead Branch"
  elseif id == 294 then
    -- an orc's face: the option jAthena sets for good (0x0800); 20 minutes as rAthena's
    -- skill_db has it (tier later)
    skill_status_change_start(src_id, SC.ORCISH, 1, 0, { tick = 1200000 })
  elseif id == 295 then
    -- "Kill target instantly. You cannot get EXP and item from killed target."
    _rt("mob_kill", bl.id)
  elseif id == 296 then
    -- "Get Zeny amount target's level x 100"
    _rt("bl_set", src_id, "zeny", sd.char.zeny + battle.lv(bl) * 100)
  elseif id == 297 then
    _rt("pc_catch_try", src_id, bl.id)
  elseif id == 298 then
    _rt("clif_emotion", bl.id, 1) -- "Target'll show '?'. And nothing."
  elseif id == 299 then
    -- Gravity's mark over the head: this client has no effect for it
  elseif id == 300 then
    -- "Character level up" (euRO's text); jAthena gives a tenth of the level's EXP
    local want = _rt("pc_next_base_exp", src_id)
    if want > 0 then _rt("pc_gain_exp", src_id, want, 0) end
  elseif id == 301 then
    -- "Everything'll be same with real fainting including EXP loss"
    battle.damage(0, src_id, sd.char.hp)
  elseif id == 302 then
    -- "Recover your HP and SP, every poisoned status"
    battle.heal(src_id, sd.st.max_hp, sd.st.max_sp)
    if sc(get(src_id), SC.POISON) then skill_status_change_end(src_id, SC.POISON) end
  elseif id == 303 then
    -- "Your HP and SP will be 1"
    _rt("bl_set", src_id, "hp", 1)
    _rt("bl_set", src_id, "sp", 1)
  end
end

-- Cast Abracadabra (jAthena skill.c:2156): draw a skill and lend it for one use at the
-- Abracadabra level the Sage has learned, or the skill's top level if lower (euRO's text: "The
-- level of random skill is affected by Skill-level"); the player aims it as with a scroll.
function sage_abracadabra(src_id, skilllv)
  local sd = get(src_id)
  if not sd or sd.kind ~= "player" then return end
  nodamage(src_id, src_id, SK.SA_ABRACADABRA, skilllv)
  local id = abra_draw(skilllv)
  if ABRA_SPECIAL[id] then return abra_special(src_id, id) end
  local learned = math.max(1, checkskill(sd, SK.SA_ABRACADABRA))
  item_skill(id, math.min(learned, skill_get(id).max or 1), src_id)
end

-- ------------------------------------------------------------ Devotion

-- jAthena skill.c:2348: only on another player of the Crusader's party or guild, not a
-- Crusader, within ten levels. Alone there is no one: it always fails, after its SP is spent
-- as there. DEVIATIONS.md, with parties.
function crusader_devotion(src_id, bl_id)
  _rt("clif_skill_fail", src_id, SK.CR_DEVOTION, 0, 0)
end

-- Status changes: what starting one does, how long it lasts, what its
-- timer does when it runs out. Their effect on a character's numbers is in
-- status.lua, on a monster's in battle.lua (battle.agi, battle.def2, ...).
--
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/skill.c
-- skill_status_change_start (3706), _end (3489), _timer (3574), archived
-- 2003-12-18. Types: 00_const.lua SC. Line refs are to that file.
--
-- The ailments (stone, frozen, stunned, asleep, poison, curse, silence,
-- confusion, blind) follow eAthena 817 map/skill.c (2004-02-19) instead:
-- mod0631 gives them placeholder lengths ("some length or other" in its
-- comments), no resistance from the target's stats, no HP loss from poison,
-- and no Curse at all. 817 lines are marked "817:".
--
-- The server keeps the store (type, val1..val4, end time), shows the icon
-- and the option words, and recomputes the character (crates/ro-game/src/
-- battle.rs). When a status change's time is up it calls rules.battle_tick,
-- which lets skill_status_change_timer decide: end it, or run it on.

local floor, random = math.floor, math.random
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end

local ELEMENT_ENCHANTS = { SC.ENCPOISON, SC.ASPERSIO, SC.FLAMELAUNCHER, SC.FROSTWEAPON, SC.LIGHTNINGLOADER, SC.SEISMICWEAPON }

function skill_status_change_end(id, type)
  return _rt("sc_del", id, type)
end

-- skill.c:3683: one weapon enchantment at a time
local function enchant_end(bl, keep)
  for _, t in ipairs(ELEMENT_ENCHANTS) do
    if t ~= keep and battle.sc(bl, t) then skill_status_change_end(bl.id, t) end
  end
end

-- skill.c:3706. Returns true when the status change started.
-- opt: tick = how long (ailments: before the target's resistance), nodef = no
-- resistance (817's flag 2), val3, val4.
function skill_status_change_start(id, type, val1, val2, opt)
  local bl = battle.get(id)
  if not bl or bl.dead then return false end
  opt = opt or {}
  val1, val2 = val1 or 0, val2 or 0
  local val3, val4 = opt.val3 or 0, opt.val4 or val2
  -- 817:5540 the target's stats shorten an ailment
  local function ailment(default, stat)
    local t = (opt.tick and opt.tick > 0 and opt.tick) or (val2 > 0 and val2) or default
    if not opt.nodef then
      local def
      if stat == "mdef" then def = 100 - battle.mdef(bl)
      elseif stat == "int" then def = 100 - (battle.int(bl) + floor(battle.luk(bl) / 3))
      else def = 100 - (battle.vit(bl) + floor(battle.luk(bl) / 3)) end
      t = floor(t * def / 100)
    end
    return math.max(1000, t)
  end
  local sd = bl.kind == "player" and bl or nil
  local md = bl.kind == "mob" and bl or nil
  local function has(t) return battle.sc(bl, t) end
  local function stop(t) if has(t) then skill_status_change_end(id, t) end end

  if md then
    -- bosses cannot be turned to stone, frozen, stunned or put to sleep
    if battle.is_boss(md) and (type == SC.STONE or type == SC.FREEZE or type == SC.STAN or type == SC.SLEEP) then
      return false
    end
  elseif sd then
    -- cards that resist an ailment (skill.c:3732)
    if type >= SC.STONE and type <= SC.BLIND then
      local res = sd.st.reseff[type - SC.STONE] or 0
      if res > 0 and rnd(100) < res then return false end
    end
  end
  -- 817:5245 the undead do not freeze; 817:5251 stun, poison, blind and
  -- silence cannot be renewed while they last
  if type == SC.FREEZE and (battle.elem_type(bl) == 9 or battle.race(bl) == 1) then return false end
  if (type == SC.STAN or type == SC.POISON or type == SC.BLIND or type == SC.SILENCE) and has(type) then return false end
  if type == SC.STONE or type == SC.FREEZE or type == SC.STAN or type == SC.SLEEP then
    _rt("stop_walking", id)
  end

  local tick
  if type == SC.PROVOKE then tick = val2 == 0 and 30000 or 1000 -- val2: Auto Berserk
  elseif type == SC.ENDURE then tick = 1000 * (7 + val1 * 3)
  elseif type == SC.CONCENTRATE then tick = 1000 * (40 + val1 * 20)
  elseif type == SC.BLESSING then
    tick = 1000 * (40 + val1 * 20)
    -- (mod0631 bug: it compares the whole element word with 9, which no monster has; the Undead are meant)
    if md and battle.elem_type(bl) ~= 9 and battle.race(bl) ~= 6 then
      stop(SC.CURSE)
      stop(SC.STONE)
    end
    if sd then stop(SC.CURSE); stop(SC.STONE) end -- 817: Blessing lifts Curse and Stone from players too
  elseif type == SC.ANGELUS then tick = 1000 * (30 * val1)
  elseif type == SC.INCREASEAGI then
    tick = 1000 * (40 + val1 * 20)
    stop(SC.DECREASEAGI)
  elseif type == SC.DECREASEAGI then
    tick = 1000 * (30 + val1 * 10)
    stop(SC.INCREASEAGI)
  elseif type == SC.TWOHANDQUICKEN then tick = 1000 * (30 * val1)
  elseif type == SC.ADRENALINE then tick = 1000 * (30 * val1)
  elseif type == SC.WEAPONPERFECTION then tick = 1000 * (30 * val1)
  elseif type == SC.OVERTHRUST then tick = 1000 * (20 * val1)
  elseif type == SC.MAXIMIZEPOWER then
    tick = 1000 * val1 -- one SP every val1 seconds
    val2 = tick
  elseif type == SC.ENCPOISON then
    tick = 1000 * (15 + val1 * 15)
    val2 = floor((val1 - 1) / 2) + 3 -- chance to poison, %
    enchant_end(bl, SC.ENCPOISON)
  elseif type == SC.IMPOSITIO then tick = 60000
  elseif type == SC.SUFFRAGIUM then tick = 1000 * (40 - val1 * 10)
  elseif type == SC.ASPERSIO then
    tick = 1000 * (30 + val1 * 30)
    enchant_end(bl, SC.ASPERSIO)
  elseif type == SC.BENEDICTIO then tick = 1000 * 40 * val1
  elseif type == SC.KYRIE then
    tick = 120000
    val2 = floor(battle.max_hp(bl) * (val1 * 2 + 10) / 100) -- damage it takes
    val1 = floor(val1 / 2) + 5                              -- hits it takes
  elseif type == SC.MAGNIFICAT then tick = 1000 * 20 * val1
  elseif type == SC.GLORIA then tick = 1000 * (5 + val1 * 5)
  elseif type == SC.AETERNA then tick = 600000
  elseif type == SC.ENERGYCOAT then tick = 300000
  elseif type == SC.LOUD then tick = 300000
  elseif type == SC.TRICKDEAD then tick = 600000
  elseif type == SC.QUAGMIRE then
    tick = 5000 * val1
    for _, t in ipairs({ SC.CONCENTRATE, SC.INCREASEAGI, SC.TWOHANDQUICKEN, SC.SPEARSQUICKEN, SC.ADRENALINE, SC.LOUD }) do stop(t) end
  elseif type == SC.SIGNUMCRUCIS then tick = 5000 * 60 * 25 -- mod0631's length (the status itself is only started by 817's Signum Crucis)
  elseif type == SC.FLAMELAUNCHER or type == SC.FROSTWEAPON or type == SC.LIGHTNINGLOADER or type == SC.SEISMICWEAPON then
    -- 20 minutes: eAthena 817's cast_db and the iRO guide's Sage page (2004-03-27).
    -- mod0631 (skill.c:3861): 15 s + 15 s a level, Enchant Poison's line copied
    tick = 1200000
    enchant_end(bl, type)
  elseif type == SC.STRIPWEAPON or type == SC.STRIPSHIELD or type == SC.STRIPARMOR or type == SC.STRIPHELM then
    tick = 60000 -- eAthena 817 skill.c:5508; the guide: "for a short amount of time"
  elseif type == SC.CP_WEAPON or type == SC.CP_SHIELD or type == SC.CP_ARMOR or type == SC.CP_HELM then
    -- a minute a level: the iRO guide's Alchemist page (2004-03-27). 817's cast_db has no row for them
    tick = 60000 * val1
  elseif type == SC.PROVIDENCE then tick = 180000; val2 = val1 * 5
  elseif type == SC.DEFENDER then tick = 180000; val2 = val1 * 15 + 5
  elseif type == SC.AUTOGUARD then
    -- eAthena 817 skill.c:5655: 5, 10, 14, 18, 21, 24, 26, 28, 29, 30 %, which is the iRO
    -- guide's "Guard Chance" table (2004-04-11); 300 s (817's cast_db, the guide)
    tick = 300000
    val2 = 0
    for i = 0, val1 - 1 do
      local t = 5 - (i >> 1)
      val2 = val2 + (t < 0 and 1 or t)
    end
  elseif type == SC.REFLECTSHIELD then
    -- the iRO guide (2004-04-11): 13% to 40% of the damage returned, 300 s. Neither
    -- emulator: 817 only plays the effect and starts the status for a second or two,
    -- by chance, when the Crusader hits something (skill.c:477, :1841, :5524)
    tick = 300000
    val2 = 10 + val1 * 3
  elseif type == SC.SPEARSQUICKEN then tick = 300000; val2 = 80 - val1
  elseif type == SC.ASSNCROS then tick = 120000; val2 = 90 - val1
  elseif type == SC.WHISTLE then tick = 60000
  elseif type == SC.APPLEIDUN then tick = 180000; val2 = val1 * 2 + 5
  elseif type == SC.SERVICE4U then tick = 180000; val2 = val1 + 10
  elseif type == SC.HUMMING then tick = 60000; val2 = val1 * 2
  elseif type == SC.DONTFORGETME then tick = 180000; val2 = 100 + val1 * 3; val3 = 100 + val1 * 2
  elseif type == SC.FORTUNE then tick = 120000
  elseif type == SC.DRUMBATTLE then tick = 60000; val2 = (val1 + 1) * 25; val3 = (val1 + 1) * 2
  elseif type == SC.ETERNALCHAOS then tick = 60000
  elseif type == SC.NIBELUNGEN then tick = 60000; val2 = (val1 + 2) * 50
  elseif type == SC.SIEGFRIED then tick = 60000; val2 = (val1 + 3) * 10
  elseif type == SC.EXPLOSIONSPIRITS then tick = 180000; val2 = 75 + 25 * val1
  elseif type == SC.STEELBODY then tick = 1000 * 30 * val1
  elseif type == SC.SPEEDPOTION0 or type == SC.SPEEDPOTION1 or type == SC.SPEEDPOTION2 then
    -- Concentration 90, Awakening 85, Berserk 80 (% of the attack delay); if
    -- several are on, the strongest counts (status.lua)
    tick = 1000 * val1
    val2 = 100 - 5 * (2 + type - SC.SPEEDPOTION0)
  elseif type == SC.STONE then
    -- Ragu Memo's ailment page (jRO players' measurements, 2003-08-02 and 2003-12-12): about
    -- 17 s, "Time = 17 - (MDef/20) [s]", whatever the skill. 817 (skill.c:5541) took the
    -- skill's time from its cast_db (10 to 19 s for Stone Curse) less 200 ms a point of MDEF.
    -- Five seconds of turning to stone first (val2 = 1), then the seconds counted in val3.
    tick = (opt.tick and opt.tick > 0 and opt.tick) or (val2 > 0 and val2) or 600000
    if not opt.nodef then tick = math.max(1000, 17000 - battle.mdef(bl) * 50) end
    val3 = math.max(1, floor(tick / 1000))
    tick = 5000
    val2 = 1
    val4 = 0
  elseif type == SC.SLEEP then
    -- Ragu Memo: "about 30 s" (no formula); 817 took the skill's time (Sandman 12 to 20 s);
    -- the fall with INT and LUK is 817's
    opt.tick = 30000
    tick = ailment(30000, "int")
  elseif type == SC.FREEZE then tick = ailment(3000 * val1, "mdef")
  elseif type == SC.STAN then
    -- Ragu Memo: "Time = 3 * (1 - (3 + VIT + LUK/3)/100) [s]", about 3 s whatever the
    -- skill; 817 took the skill's time (Sonic Blow 6 to 15 s) less VIT + LUK/3 percent
    tick = opt.nodef and 3000 or math.max(1000, floor(3000 * (100 - (3 + battle.vit(bl) + floor(battle.luk(bl) / 3))) / 100))
  elseif type == SC.POISON then
    -- 817:5576 poison counts seconds in val3 and bites once a second
    tick = ailment(1000 * (10 + rnd(50)), "vit")
    val3 = math.max(1, floor(tick / 1000))
    tick = 1000
  elseif type == SC.CURSE then tick = ailment(30000, "vit")
  elseif type == SC.SILENCE then
    -- the skill's time as in 817; Ragu Memo: shortened by VIT alone, "Time = base * (100 - VIT)/100"
    tick = (opt.tick and opt.tick > 0 and opt.tick) or 1000 * (val1 > 6 and 60 or 25 + val1 * 5)
    if not opt.nodef then tick = math.max(1000, floor(tick * (100 - battle.vit(bl)) / 100)) end
  elseif type == SC.CONFUSION then tick = opt.tick or 17000 -- 817 skill.c: cards cause 10 s + 7 s of it
  elseif type == SC.BLIND then
    -- Ragu Memo: "Time = 30 - BaseLv/10 - INT/15 [s]", 15 to 30 s; 817 took the skill's time
    -- (Flasher 10 to 14 s, a monster's 10 to 19 s) less INT + LUK/3 percent
    tick = opt.nodef and 30000 or math.max(1000, 30000 - battle.lv(bl) * 100 - floor(battle.int(bl) * 1000 / 15))
  elseif type == SC.SLOWPOISON then
    -- 817:5296 only on someone poisoned; length from its cast_db
    if not has(SC.POISON) then return false end
    tick = opt.tick or skill_get_time(SK.PR_SLOWPOISON, val1)
  elseif type == SC.HIDDING then
    tick = 1000       -- checked every second
    val2 = 30 * val1  -- seconds it lasts
  elseif type == SC.CLOAKING then
    tick = 1000 * (3 + val1) -- one SP every (3 + level) seconds
    val2 = tick
  elseif type == SC.SIGHT or type == SC.RUWACH then
    tick = 10
    val2 = 40 -- 40 x 250 ms = 10 s
  elseif type == SC.SAFETYWALL or type == SC.PNEUMA then
    tick = opt.tick or 10000 -- as long as the cell that gives it (val2 = the cell)
  elseif type == SC.ANKLE then tick = opt.tick or val2
  elseif type == SC.WATERBALL then
    tick = 100
    val3 = (val1 | 1) * (val1 | 1) - 1
  elseif type == SC.AUTOCOUNTER then tick = opt.tick or skill_get_time(SK.KN_AUTOCOUNTER, val1) -- 817 cast_db: 0.4 s a level
  -- 30 s at every level: the iRO guide's Assassin page (2004-03-27). 817's cast_db says 30 minutes.
  elseif type == SC.POISONREACT then tick = opt.tick or 30000
  elseif type == SC.RIDING or type == SC.FALCON or type == SC.WEIGHT50 or type == SC.WEIGHT90 then tick = 600000
  else
    return false -- UnknownStatusChange
  end

  -- stone, frozen, stunned, asleep: one at a time, and no attacking
  if type == SC.STONE or type == SC.FREEZE or type == SC.STAN or type == SC.SLEEP then
    _rt("stop_attack", id)
    for t = SC.STONE, SC.SLEEP do if t ~= type then stop(t) end end
  elseif type == SC.HIDDING or type == SC.CLOAKING then
    _rt("stop_attack", id)
  end
  _rt("sc_set", id, type, val1, val2, val3, val4, tick)
  return true
end

-- skill.c:3458: what Sight and Ruwach do to each unit around the caster
local function reveal(src, type)
  -- AREA_SIZE: the screen (20 cells in mod0631, 14 here)
  for _, tid in ipairs(_rt("in_area", src.map, src.x - 14, src.y - 14, src.x + 14, src.y + 14)) do
    if tid ~= src.id then
      local bl = battle.get(tid)
      if bl and (bl.option & 6) ~= 0 then
        skill_status_change_end(tid, SC.HIDDING)
        skill_status_change_end(tid, SC.CLOAKING)
        if type == SC.RUWACH and battle.is_enemy(src, bl) then
          skill_attack(BF.MAGIC, src.id, src.id, tid, SK.AL_RUWACH, 1, 0)
        end
      end
    end
  end
end

-- skill.c:3574: the status change's time is up. Some run on.
function skill_status_change_timer(id, type)
  local bl = battle.get(id)
  if not bl then return end
  local t = battle.sc(bl, type)
  if not t then return end
  local sd = bl.kind == "player" and bl or nil
  local function again(ms, v1, v2, v3) _rt("sc_touch", id, type, v1, v2, v3, ms) end

  if type == SC.MAXIMIZEPOWER or type == SC.CLOAKING then
    -- lasts while there is SP: one point each interval
    if sd and sd.char.sp > 0 then
      _rt("bl_set", id, "sp", sd.char.sp - 1)
      return again(t.val2)
    end
  elseif type == SC.HIDDING then
    -- one SP every (level + 3) seconds, at most 30 s a level
    if sd and sd.char.sp > 0 and t.val2 - 1 > 0 then
      local left = t.val2 - 1
      if left % (t.val1 + 3) == 0 then _rt("bl_set", id, "sp", sd.char.sp - 1) end
      return again(1000, nil, left)
    end
  elseif type == SC.SIGHT or type == SC.RUWACH then
    reveal(bl, type)
    if t.val2 - 1 > 0 then return again(250, nil, t.val2 - 1) end
  elseif type == SC.PROVOKE then
    -- Auto Berserk checks the HP every second
    if t.val2 ~= 0 and not (sd and sd.char.hp > floor(sd.st.max_hp / 4)) then return again(1000) end
  elseif type == SC.WATERBALL then
    local target = battle.get(t.val2)
    if target and not target.dead then
      skill_attack(BF.MAGIC, id, id, t.val2, SK.WZ_WATERBALL, t.val1, 0)
      if t.val3 - 1 > 0 then return again(100, nil, nil, t.val3 - 1) end
    end
  elseif type == SC.POISON then
    -- Ragu Memo's ailment page (2003-08-02 and 2003-12-12): "every 3 seconds, 3% of
    -- MaxHP at a time, down to 25% of MaxHP", players and monsters alike. 817
    -- (skill.c:5134) bit every second, a player 3 + 1.5% of max HP, a monster
    -- 3 + 0.5%. The seconds are still counted one at a time; Slow Poison holds it.
    if battle.sc(bl, SC.SLOWPOISON) then return again(1000) end
    if t.val3 - 1 > 0 then
      local mhp = battle.max_hp(bl)
      if (t.val3 - 1) % 3 == 0 and battle.hp(bl) > (mhp >> 2) then
        local loss = math.max(1, floor(mhp * 3 / 100))
        _rt("bl_set", id, "hp", math.max(mhp >> 2, battle.hp(bl) - loss))
      end
      return again(1000, nil, nil, t.val3 - 1)
    end
  elseif type == SC.STONE then
    -- 817:5107 turned to stone after the first five seconds; then a second
    -- at a time, and every fifth one 1% of max HP while above a quarter
    if t.val2 ~= 0 then
      _rt("stop_walking", id)
      return again(1000, nil, 0, nil)
    end
    if t.val3 - 1 > 0 then
      local n = t.val4 + 1
      local mhp = battle.max_hp(bl)
      if n % 5 == 0 and battle.hp(bl) > (mhp >> 2) then
        _rt("bl_set", id, "hp", battle.hp(bl) - math.max(1, floor(mhp / 100)))
      end
      _rt("sc_touch", id, type, nil, nil, t.val3 - 1, 1000, n)
      return
    end
  elseif type == SC.AETERNA or type == SC.TRICKDEAD or type == SC.RIDING
    or type == SC.FALCON or type == SC.WEIGHT50 or type == SC.WEIGHT90 then
    return again(600000) -- no time limit
  end
  skill_status_change_end(id, type)
end

-- ------------------------------------------------------------ for scripts

-- sc_start / sc_end of item and NPC scripts act on the player the script runs for.
function sc_start(kind, val1, val2) return skill_status_change_start(player.id, kind, val1 or 0, val2 or 0) end
function sc_end(kind) skill_status_change_end(player.id, kind) end

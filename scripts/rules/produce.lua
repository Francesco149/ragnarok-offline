-- Forging and refining ores (Blacksmith): hammers and the Mini Furnace open a
-- list of what can be made; picking one uses up the materials and may fail.
-- Server-side: NOT in the client.
--
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/skill.c
-- skill_can_produce_mix (4549), skill_produce_mix (4593), map/script.c
-- buildin_produce, archived 2003-12-18. Recipes: db/produce_db.txt
-- (scripts/import/craft.lua). Line refs are to skill.c.

local floor, random = math.floor, math.random
local function rnd(n) if n <= 0 then return 0 end return random(0, n - 1) end
local get = battle.get
local checkskill = battle.pc_checkskill

local function recipe(nameid)
  for _, r in ipairs(skill_produce_db or {}) do
    if r.id == nameid then return r end
  end
end

local function is_equip(nameid)
  local t = _rt("item_db", nameid)
  return t and (t.type == 4 or t.type == 5), t and t.wlv or 0
end

-- skill.c:4549. trigger: 16 = Mini Furnace (ores only), 1..3 = a hammer of
-- that weapon level, -1 = no tool check.
local function can_produce(sd, nameid, trigger)
  local r = recipe(nameid)
  if not r then return nil end
  if trigger >= 0 then
    local equip, wlv = is_equip(nameid)
    if trigger == 16 then
      if equip then return nil end
    else
      if not equip or wlv > trigger then return nil end
    end
  end
  if r.skill > 0 and checkskill(sd, r.skill) <= 0 then return nil end
  for _, m in ipairs(r.materials) do
    if _rt("pc_item_count", sd.id, m[1]) < m[2] then return nil end
  end
  return r
end

-- The `produce` script command of the hammers and the furnace.
function produce(trigger)
  local sd = get(player.id)
  if not sd then return end
  local ids = {}
  for _, r in ipairs(skill_produce_db or {}) do
    if can_produce(sd, r.id, trigger) then ids[#ids + 1] = r.id end
  end
  _rt("pc_produce_list", sd.id, ids)
end

-- skill.c:4593: the item picked in the forge window, with up to three
-- extras: Star Crumbs (1000) and one elemental stone (994 Flame Heart,
-- 995 Mystic Frozen, 996 Rough Wind, 997 Great Nature).
function rules.produce_mix(a)
  local sd = get(a.src)
  if not sd or sd.dead then return end
  local nameid = a.item
  local r = can_produce(sd, nameid, -1)
  if not r then return end
  local sc, ele = 0, 0
  local ele_table = { [994] = 3, [995] = 1, [996] = 4, [997] = 2 }
  for _, slot in ipairs(a.slots or {}) do
    if slot > 0 and _rt("pc_item_count", sd.id, slot) > 0 then
      if slot == 1000 then
        _rt("pc_item_take", sd.id, slot, 1)
        sc = sc + 1
      elseif ele_table[slot] and ele == 0 then
        _rt("pc_item_take", sd.id, slot, 1)
        ele = ele_table[slot]
      end
    end
  end
  for _, m in ipairs(r.materials) do _rt("pc_item_take", sd.id, m[1], m[2]) end

  local lv, dex, luk = sd.char.base_level, sd.st.dex, sd.st.luk
  local skill = checkskill(sd, r.skill)
  local equip, wlv = is_equip(nameid)
  local make_per
  if not equip then
    if nameid == 998 then make_per = 2000 + lv * 30 + dex * 20 + luk * 10 + skill * 600        -- Iron
    elseif nameid == 985 then make_per = 1000 + lv * 30 + dex * 20 + luk * 10 + (skill - 1) * 500 -- Elunium
    else make_per = 1000 + lv * 30 + dex * 20 + luk * 10 + skill * 500 end
  else
    -- the best anvil carried helps; none at all hurts
    local add_per
    if _rt("pc_item_count", sd.id, 989) > 0 then add_per = 750       -- Emperium Anvil
    elseif _rt("pc_item_count", sd.id, 988) > 0 then add_per = 500   -- Golden Anvil
    elseif _rt("pc_item_count", sd.id, 987) > 0 then add_per = 250   -- Oridecon Anvil
    elseif _rt("pc_item_count", sd.id, 986) > 0 then add_per = 0     -- Anvil
    else add_per = -500 end
    if ele ~= 0 then add_per = add_per - 500 end
    add_per = add_per - sc * 500
    make_per = floor((250 + lv * 15 + dex * 10 + luk * 5 + skill * 500 + add_per) * (100 - (wlv - 1) * 20) / 100)
      + checkskill(sd, SK.BS_WEAPONRESEARCH) * 100 + (wlv >= 3 and checkskill(sd, SK.BS_ORIDEOCON) * 100 or 0)
  end
  if make_per < 1 then make_per = 1 end

  if rnd(10000) < make_per then
    local cards
    if equip then
      -- a forged weapon: 0x00ff, Star Crumbs*5 << 8 | element, the maker's id
      cards = { 0x00ff, ((sc * 5) << 8) + ele, sd.char_id & 0xffff, (sd.char_id >> 16) & 0xffff }
    end
    _rt("clif_produceeffect", sd.id, 0, nameid)
    _rt("clif_misceffect", sd.id, 3)
    _rt("pc_item_add", sd.id, nameid, 1, cards)
  else
    _rt("clif_produceeffect", sd.id, 1, nameid)
    _rt("clif_misceffect", sd.id, 2)
  end
end

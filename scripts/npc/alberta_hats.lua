-- Alberta's two hat makers, who came with Turtle Island: cut content brought
-- back (docs/DEVIATIONS.md, "Content the client ships but no source scripts").
--
-- No period emulator has them. The client does: Turtle Island's patch
-- (2003-04-30gdata_turtle_ph_1.gpf, euRO's patch entry 400, January 2004)
-- ships the sprites of exactly these eight hats, and Gravity's English
-- descriptions of them are in the base GRF. iRO's notice of the island
-- (2003-12-09) promised "brand new headgears and new quests".
--
-- What is whose (research/THREADS.md #29, round 13 T5-001..008):
-- * names, cells and recipes: RODataZone's iRO quest list (items/quests.js,
--   Wayback 2004-05-02). Ragu Memo's jRO list (trade.html, 2004-02-22) has
--   the same eight recipes, puts the merchant "near the south Kafra" and the
--   girl at 135,78, and says how she is asked: she wants a Yellow Potion
--   before she talks, "It's really hot, isn't it" gets the three hats, "Give
--   me something instead" the Helm of Solar God.
-- * sprites: rAthena's (quests_alberta.txt, "#new30"; tier later).
-- * every line of dialogue: ours. No English or Japanese text survives.
--   That she keeps the potion is our reading of Ragu Memo's note.
-- * item names in the client: data/client/item_names.tsv.
--
-- The Ice Cream Maker (events switch) stands on the merchant's cell, so with
-- events on the merchant steps one cell east (122,53 is a wall).

local function has_all(take)
  for _, t in ipairs(take) do
    if count_item(t[1]) < t[2] then return false end
  end
  return true
end

-- Each material is { id, count, name as the client spells it }.
local function list(take)
  local s = {}
  for _, t in ipairs(take) do s[#s + 1] = t[2] .. " " .. t[3] end
  return table.concat(s, ", ")
end

-- Offer one recipe: show it, ask, trade. Returns true if traded.
local function offer(who, r)
  mes(who)
  mes("^3355FF" .. r.name .. "^000000:")
  mes(list(r.take) .. ".")
  next()
  if menu("Make it for me.", "Not now.") ~= 1 then return false end
  if not has_all(r.take) then
    mes(who)
    for _, l in ipairs(r.short) do mes(l) end
    close()
    return false
  end
  for _, t in ipairs(r.take) do take_item(t[1], t[2]) end
  give_item(r.give, 1)
  mes(who)
  for _, l in ipairs(r.done) do mes(l) end
  close()
  return true
end

-- Stylish Merchant: alberta 120/53 (RODataZone), "near the south Kafra"
-- (Ragu Memo; the Kafra is at 113,60).
local MERCHANT = {
  { name = "Heart Hair Pin", give = 5041, take = { { 7013, 1200, "Coral Reef" } },
    short = { "Twelve hundred pieces of coral,", "darling. Not one less. The reefs", "off Turtle Island are full of it." },
    done = { "There! A little heart for", "your hair. Wear it for someone." } },
  { name = "First Moon Hair Pin", give = 5048, take = { { 5041, 1, "Heart Hair Pin" }, { 999, 10, "Steel" } },
    short = { "I need a Heart Hair Pin to", "rework, and ten Steel for the", "crescent. Bring both." },
    done = { "A heart becomes a new moon.", "Much more mysterious, no?" } },
  { name = "Fashionable Glasses", give = 5047, take = { { 2271, 1, "Jack a Dandy" }, { 975, 1, "Scarlet Dyestuffs" } },
    short = { "A Jack a Dandy for the frame,", "and Scarlet Dyestuffs to give", "it some colour. That's all." },
    done = { "Red frames! Now you look like", "you know where you're going." } },
  { name = "Hair Protector", give = 5042, take = { { 10007, 1, "Silk Ribbon" }, { 968, 50, "Heroic Emblem" } },
    short = { "A Silk Ribbon, and fifty", "Heroic Emblems from the Orc", "heroes. It's for warriors, love." },
    done = { "Two buns, tied tight. They say", "a woman warrior of old wore", "these. Kick hard in them." } },
}

npc {
  map = "alberta", x = content("events") and 121 or 120, y = 53, dir = 4, sprite = 51,
  name = "Stylish Merchant", id = "alberta_stylish_merchant",
  src = "ours:alberta_hats.lua (RODataZone quests.js HATS[19..22])",
  on_click = function()
    local who = "[Stylish Merchant]"
    mes(who)
    mes("Fresh off the boat from Turtle")
    mes("Island, and look at you! Plain")
    mes("as a Poring. A little something")
    mes("for the hair, perhaps?")
    next()
    local labels = {}
    for i, r in ipairs(MERCHANT) do labels[i] = r.name end
    labels[#labels + 1] = "No, thanks."
    local r = MERCHANT[menu(table.unpack(labels))]
    if not r then
      mes(who)
      mes("Your loss, sweetie.")
      return close()
    end
    offer(who, r)
  end,
}

-- Hat Store Girl: alberta 136/79 (RODataZone; Ragu Memo 135,78).
local GIRL = {
  { name = "Sunday Hat", give = 5032, take = { { 2221, 1, "Hat [1]" }, { 2227, 1, "Cap [1]" }, { 7063, 600, "Soft Feather" }, { 1059, 250, "Fabric" } },
    short = { "A Hat and a Cap, both with a", "slot, 600 Soft Feathers and", "250 Fabric. Then we talk!" },
    done = { "A wide brim for the sun.", "Just don't wear it in the rain!" } },
  { name = "Mage Hat", give = 5027, take = { { 2252, 1, "Wizard Hat" }, { 4052, 1, "Elder Willow Card" }, { 1036, 400, "Dragon Scale" }, { 7001, 50, "Mould Powder" } },
    short = { "A Wizard Hat, an Elder Willow", "Card, 400 Dragon Scales and", "50 Mould Powder. Sorry!" },
    done = { "Tall and pointy! Every mage", "wants one, even if they won't", "say so." } },
  { name = "Magician Hat", give = 5045, take = { { 2252, 1, "Wizard Hat" }, { 1054, 450, "Ancient Lips" }, { 943, 1200, "Solid Shell" } },
    short = { "A Wizard Hat, 450 Ancient Lips", "and 1200 Solid Shells to make", "it stout. Yes, that many." },
    done = { "Ta-da! No rabbit inside,", "I checked." } },
}
local HELM = { name = "Helm of Solar God", give = 5022,
  take = { { 7086, 1, "Emblem of the Sun God" }, { 969, 10, "Gold" }, { 999, 40, "Steel" }, { 1003, 50, "Coal" }, { 984, 2, "Oridecon" } },
  short = { "The Emblem of the Sun God,", "10 Gold, 40 Steel, 50 Coal and", "2 Oridecon. The emblem is the", "hard part: castle treasure." },
  done = { "Golden wings and the sun on", "your head. How you'll walk in", "it is your problem!" } }

npc {
  map = "alberta", x = 136, y = 79, dir = 4, sprite = 71,
  name = "Hat Store Girl", id = "alberta_hat_store_girl",
  src = "ours:alberta_hats.lua (RODataZone quests.js HATS[23..26])",
  on_click = function()
    local who = "[Hat Store Girl]"
    mes(who)
    mes("Haaah... this sun...")
    if count_item(503) < 1 then
      mes("I can't... talk... too hot...")
      mes("Something cool to drink...")
      mes("yellow... sweet...")
      return close()
    end
    mes("Is that a Yellow Potion?!")
    mes("Oh please, please, can I...?")
    next()
    local c = menu("It's really hot, isn't it.", "Give me something instead.", "No.")
    if c == 3 or not c then
      mes(who)
      mes("Meanie...")
      return close()
    end
    take_item(503, 1)
    mes(who)
    mes("*gulp gulp*")
    mes("Aaah, I'm alive again!")
    if c == 1 then
      mes("It IS hot. That's why everyone")
      mes("needs a good hat! I make them.")
      next()
      local r = GIRL[menu("Sunday Hat", "Mage Hat", "Magician Hat", "Maybe later.")]
      if not r then
        mes(who)
        mes("Come back when it's hot! So,")
        mes("tomorrow.")
        return close()
      end
      offer(who, r)
    else
      mes("Something in return? Hmm...")
      mes("My grandpa left me a design for")
      mes("a helm with golden wings.")
      next()
      offer(who, HELM)
    end
  end,
}

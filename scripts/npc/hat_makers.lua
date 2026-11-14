-- Four hat makers no period emulator has: Payon's Young Man, Comodo's Hair
-- Ornament Girl and Traveler, Lutie's Vending Machine. Cut content brought
-- back (docs/DEVIATIONS.md, "Content the client ships but no source scripts";
-- research/THREADS.md #31): this client carries Gravity's English description
-- and the sprite of every hat they make, and nothing in mod0631 or 817 gives
-- most of them.
--
-- What is whose:
-- * makers, cells and recipes: two period lists that agree item for item,
--   RODataZone's iRO quest list (items/quests.js, Wayback 2004-04-17,
--   HATS[27..44]) and Ragu Memo's jRO list (trade.html, Wayback 2004-02-22;
--   both saved in research/agy-threads/r13/pages/). One count differs: the
--   Chef Hat's Feathers, 300 in RODataZone, 330 in Ragu Memo and in rAthena:
--   330 here.
-- * dialogue: rAthena's (npc/quests/quests_payon.txt, quests_comodo.txt,
--   quests_lutie.txt; iRO's later official text, tier `later`), cut to the
--   client's window. Lutie's machine is rAthena's "Audi" alone: its Titicupe,
--   who tells the recipes, is left out and the machine shows them itself.
-- * sprites and facing: rAthena's.
-- * item names the client lacks: data/client/item_names.tsv.

local function has_all(take)
  for _, t in ipairs(take) do
    if count_item(t[1]) < t[2] then return false end
  end
  return true
end

local function show(take)
  for _, t in ipairs(take) do mes("^3355FF" .. t[2] .. " " .. t[3] .. "^000000") end
end

local function trade(r)
  for _, t in ipairs(r.take) do take_item(t[1], t[2]) end
  give_item(r.give, 1)
end

-- Each recipe: name, give, take = { { id, count, name as the client spells it } }.

-- Young Man: payon_in01 122/41 (RODataZone), inside the weapon and armour
-- shop (Ragu Memo). rAthena has him as a floating NPC.
local YOUNG_MAN = {
  { name = "Angel Helm", give = 5025,
    take = { { 2229, 1, "Helm [1]" }, { 2254, 1, "Angel Wing" }, { 7036, 5, "Fang of Garm" } } },
  { name = "Hat of Petite Diablo", give = 5038,
    take = { { 1038, 600, "Little Evil Horn" }, { 7048, 40, "Talon of Griffon" } } },
}

npc {
  map = "payon_in01", x = 122, y = 41, dir = 4, sprite = 89,
  name = "Young Man", id = "payon_young_man",
  src = "ours:hat_makers.lua (RODataZone quests.js HATS[27..28]; rAthena quests_payon.txt)",
  on_click = function()
    local who = "[Young Man]"
    mes(who)
    mes("...What is it?")
    next()
    if menu("Can you make me something?", "Hey.") ~= 1 then
      mes(who)
      mes("Hm.")
      mes("Take care.")
      return close()
    end
    mes(who)
    mes("Huh? So you already know what")
    mes("I specialize in, eh? No need")
    mes("for the usual secrecy, then.")
    mes("Tell me what you want.")
    next()
    local r = YOUNG_MAN[menu(YOUNG_MAN[1].name, YOUNG_MAN[2].name, "I'll come next time.")]
    if not r then
      mes(who)
      mes("Hm. Alright.")
      mes("Then we shall meet again.")
      return close()
    end
    mes(who)
    show(r.take)
    mes("That's all I require.")
    next()
    mes(who)
    mes("Cards and upgrades in what")
    mes("you hand over are lost when I")
    mes("make it. Be careful what you")
    mes("give me.")
    next()
    if menu("Give him the items.", "Don't give him the items.") ~= 1 then
      mes(who)
      mes("Do as you wish...")
      return close()
    end
    mes(who)
    if not has_all(r.take) then
      mes("...The number of items is not")
      mes("right. You better double check.")
      return close()
    end
    trade(r)
    mes(".....Hm. Here's your")
    mes(r.name .. ". Take it.")
    close()
  end,
}

-- Hair Ornament Girl: comodo 228/159 (RODataZone; Ragu Memo 228,158, "near the
-- central Kafra").
local GIRL = {
  { name = "Cross Hat", give = 5036,
    take = { { 2608, 1, "Rosary" }, { 7069, 500, "Destroyed Armor" } } },
  { name = "Bulb Band", give = 5034,
    take = { { 2233, 1, "Circlet [1]" }, { 746, 20, "Glass Bead" } } },
  { name = "Stripe Band", give = 5049,
    take = { { 1099, 1500, "Worn-out Prison Uniform" } } },
  { name = "Blue Hairband", give = 5052,
    take = { { 2211, 1, "Bandana" }, { 978, 1, "Cobaltblue Dyestuffs" }, { 7003, 300, "Anolian Skin" } } },
}

npc {
  map = "comodo", x = 228, y = 159, dir = 3, sprite = 700,
  name = "Hair Ornament Girl", id = "comodo_hair_ornament_girl",
  src = "ours:hat_makers.lua (RODataZone quests.js HATS[33..36]; rAthena quests_comodo.txt)",
  on_click = function()
    local who = "[Hair Ornament Girl]"
    mes(who)
    if player.sex == 1 then
      mes("Oh, hello! Would you like to")
      mes("order a hairband? They make")
      mes("great presents for your")
      mes("girlfriend.")
    else
      mes("Hello~! One of these would")
      mes("suit a pretty girl like you.")
      mes("Would you like to order a")
      mes("hairband?")
    end
    next()
    mes(who)
    mes("Give me a few simple items and")
    mes("I'll make you a gorgeous one.")
    next()
    local r = GIRL[menu(GIRL[1].name, GIRL[2].name, GIRL[3].name, GIRL[4].name)]
    if not r then return close() end
    if not has_all(r.take) then
      mes(who)
      mes("Oh, you've chosen the")
      mes(r.name .. "? I'll need...")
      show(r.take)
      next()
      mes(who)
      mes("Bring them all and I'll make")
      mes("it for you right away.")
      mes("See you then~")
      return close()
    end
    mes(who)
    mes("You've chosen the")
    mes(r.name .. "?")
    mes("Cards and upgrades on the items")
    mes("I use will be lost. Keep only")
    mes("what I need with you. Shall we")
    mes("make it?")
    next()
    if menu("No.", "Yes.") ~= 2 then
      mes(who)
      mes("Ah, alright. Put your other")
      mes("things in Kafra Storage first.")
      mes("See you soon~")
      return close()
    end
    mes("^3355FFSwish swish snip snip^000000")
    next()
    trade(r)
    mes(who)
    mes("Look at this! Well done~")
    mes("Please come again~")
    close()
  end,
}

-- Traveler: comodo 273/137 (RODataZone; Ragu Memo 272,137). rAthena names
-- him Isac Mari.
local TRAVELER = {
  { name = "Mine Helmet", give = 5031,
    take = { { 5009, 1, "Safety Helmet" }, { 5028, 1, "Candle" }, { 747, 1, "Crystal Mirror" }, { 999, 25, "Steel" } },
    tale = { "Miners near Geffen can't work", "without light. One of them", "fixed a light to his Safety", "Helmet to keep both hands free." } },
  { name = "Parcel Hat", give = 5023,
    take = { { 1059, 150, "Fabric" }, { 907, 100, "Resin" }, { 978, 1, "Cobaltblue Dyestuffs" } },
    tale = { "In the East they carry goods", "in a bundle on the head, not", "in carts like Alberta's", "merchants. Hard to balance!" } },
  { name = "Money Loser's Grief", give = 5021,
    take = { { 2233, 1, "Circlet [1]" }, { 969, 1, "Gold" }, { 999, 20, "Steel" }, { 949, 80, "Feather" }, { 938, 800, "Sticky Mucus" } },
    tale = { "An honest Morroc merchant", "crossed a bad guild, and lost", "all but some broken things.", "He made this hat of them." } },
  { name = "Phantom of Opera", give = 5043,
    take = { { 2281, 1, "Opera Masque" }, { 1048, 50, "Horrendous Hair" } },
    tale = { "A scarred musical genius who", "loved a singer, and vanished.", "All that was left of him was", "this mask." } },
}

npc {
  map = "comodo", x = 273, y = 137, dir = 4, sprite = 702,
  name = "Traveler", id = "comodo_traveler",
  src = "ours:hat_makers.lua (RODataZone quests.js HATS[29..32]; rAthena quests_comodo.txt)",
  on_click = function()
    local who = "[Isac Mari]"
    mes(who)
    mes("I travel the world and collect")
    mes("people's stories. Sometimes")
    mes("they teach me how to make")
    mes("special things, like hats.")
    next()
    local labels = {}
    for i, t in ipairs(TRAVELER) do labels[i] = t.name end
    labels[#labels + 1] = "Never mind."
    local r = TRAVELER[menu(table.unpack(labels))]
    if not r then
      mes(who)
      mes("Safe travels, friend.")
      return close()
    end
    if not has_all(r.take) then
      mes(who)
      for _, l in ipairs(r.tale) do mes(l) end
      next()
      mes(who)
      mes("For the " .. r.name .. ":")
      show(r.take)
      next()
      mes(who)
      mes("Bring them all and I'll make")
      mes("it right away. See you later~")
      return close()
    end
    mes(who)
    mes("You brought all I need for")
    mes("the " .. r.name .. "!")
    mes("Cards and upgrades on them will")
    mes("be lost, mind. Shall I start?")
    next()
    if menu("Give me a minute~!", "Let's make it.") ~= 2 then
      mes(who)
      mes("Alright. Better safe than")
      mes("sorry.")
      return close()
    end
    mes("^3355FF*Thook Thook*")
    mes("*Pop!*^000000")
    next()
    trade(r)
    mes(who)
    mes("Whew! This is pretty hard!")
    mes("I'm not quite sure it's right,")
    mes("but it looks about right. Ha~")
    close()
  end,
}

-- Vending Machine: xmas 114/296 (RODataZone), 115,295 (Ragu Memo, "11
-- o'clock"); here on rAthena's cell, the snowman's mouth at 115,297, with
-- rAthena's invisible sprite.
local MACHINE = {
  { name = "Raccoon Hat", give = 5033,
    take = { { 2213, 1, "Kitty Band" }, { 1036, 20, "Dragon Scale" }, { 7012, 200, "Tough Scalelike Stem" }, { 7065, 300, "Sea-Otter Fur" } } },
  { name = "Spore Hat", give = 5029,
    take = { { 7033, 850, "Poison Spore" }, { 7068, 300, "Burnt Tree" }, { 1015, 1, "Tongue" } } },
  { name = "Wonder Nutshell", give = 5050,
    take = { { 5037, 1, "Nut Shell" }, { 7064, 500, "Wing of Dragonfly" } } },
  { name = "Rainbow Eggshell", give = 5039,
    take = { { 5015, 1, "Egg Shell" }, { 978, 1, "Cobaltblue Dyestuffs" }, { 7030, 50, "Claw of Desert Wolf" } } },
  { name = "Blush", give = 5040,
    take = { { 7047, 100, "Alice's Apron" } } },
  { name = "Chef Hat", give = 5026,
    take = { { 539, 120, "Piece of Cake" }, { 982, 1, "White Dyestuffs" }, { 949, 330, "Feather" }, { 1036, 450, "Dragon Scale" } } },
  { name = "Candle", give = 5028,
    take = { { 2279, 1, "Bomb Wick" }, { 7035, 50, "Matchstick" }, { 526, 100, "Royal Jelly" } } },
  { name = "Cake Hat", give = 5024,
    take = { { 529, 10, "Candy" }, { 530, 5, "Candy Cane" }, { 538, 15, "Well-baked Cookie" }, { 539, 20, "Piece of Cake" }, { 999, 10, "Steel" } } },
}

npc {
  map = "xmas", x = 115, y = 297, dir = 4, sprite = 111,
  name = "Vending Machine", id = "lutie_vending_machine",
  src = "ours:hat_makers.lua (RODataZone quests.js HATS[37..44]; rAthena quests_lutie.txt)",
  on_click = function()
    local who = "[Audi]"
    mes(who)
    mes("^555555Exciting item vending")
    mes("machine. Invented by great")
    mes("Titicupe. Select the desired")
    mes("item and place the required")
    mes("items into the snowmouth.^000000")
    next()
    local labels = { "^FF3355Notice (Must Read)^000000" }
    for _, t in ipairs(MACHINE) do labels[#labels + 1] = t.name end
    labels[#labels + 1] = "Cancel"
    local c = menu(table.unpack(labels))
    if c == 1 then
      mes(who)
      mes("^555555This machine cannot tell")
      mes("special items from ordinary")
      mes("ones. Upgrades and cards on")
      mes("what you insert are lost.^000000")
      next()
      mes(who)
      mes("^555555Put valuables you won't use")
      mes("into Kafra Storage. No returns,")
      mes("refunds or exchanges.")
      mes("Thank you!^000000")
      return close()
    end
    local r = c and MACHINE[c - 1]
    if not r then
      mes(who)
      mes("^555555Please... Insert... Items.^000000")
      return close()
    end
    mes(who)
    mes("^555555" .. r.name .. ":^000000")
    show(r.take)
    next()
    if menu("Insert the items.", "Cancel") ~= 1 then return close() end
    if not has_all(r.take) then
      emotion(9)
      mes(who)
      mes("^555555Error Error!")
      mes("Incorrect items!^000000")
      return close()
    end
    mes("^3355FFYou gingerly place the items")
    mes("into the snowman's mouth.^000000")
    next()
    trade(r)
    emotion(0)
    mes("^3355FF*Vroooooom~~*")
    mes("*Bzzzzzt*")
    mes("*choogachooga*")
    mes("*Kapang!*^000000")
    next()
    mes(who)
    mes("^555555Thank you for coming by.")
    mes("Please come again!")
    mes("Oh, and tell a friend!^000000")
    close()
  end,
}

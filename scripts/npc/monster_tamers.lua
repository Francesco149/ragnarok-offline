-- The Monster Tamers of Alberta, Izlude and Al De Baran: taming items and pet
-- accessories made from what you bring.
--
-- Source: mod0631 conf/npc_event_mobtim.txt (lines 9, 182, 357), archived
-- 2003-12-18; it is in the list mod0631 loaded (map_athena.conf:56). Tier
-- athena2003, jRO lineage. 817 has no such NPC. The iRO pet guide confirms
-- them for the official game (capture of 2004-06-05,
-- iro.ragnarokonline.com/game/qpetsystem.asp): "Monster Tamer: Found in the
-- buildings of Alberta, Izlude, and Al De Baran, these NPCs have some of the
-- items necessary for taming and pet accessories". That page says "available
-- for purchase"; mod0631's take ingredients and no zeny, and that is what
-- is here.
--
-- Places, sprite, names and recipes are mod0631's. The English is ours: the
-- three scripts are one template in Japanese, with a line or two of patter
-- that differs per item, and one translated template serves all three here.
-- Item names are the client's. research/notes/round2-review.md.

-- { product, { ingredient, count }, ... }
local TAMERS = {
  { map = "alberta_in", x = 173, y = 76, who = "Iwado", line = 9,
    { 626, { 702, 1 }, { 512, 2 } },    -- Monster Juice: Animal Gore, 2 Apples
    { 629, { 707, 1 } },                -- Singing Flower: Singing Plant
    { 10009, { 2207, 1 }, { 705, 1 } }, -- Wild Flower: Fancy Flower, Clover
  },
  { map = "izlude_in", x = 129, y = 64, who = "Shogo", line = 182,
    { 642, { 1006, 1 }, { 958, 2 } },   -- Book of Devil: Old Magicbook, 2 Horrendous Mouths
    { 636, { 7014, 1 } },               -- No Recipient: Old Portrait
    { 635, { 1004, 1 }, { 904, 1 } },   -- Horror of Tribe: Chivalry Emblem, Scorpion Tail
  },
  { map = "aldeba_in", x = 168, y = 175, who = "Yu", line = 357,
    { 10001, { 5017, 1 } },             -- Skull Helm: Bone Helm
    { 10002, { 5004, 1 } },             -- Monster Oxygen Mask: Oxygen Mask
    { 10007, { 2208, 1 } },             -- Silk Ribbon: Ribbon
    { 10011, { 2294, 1 } },             -- Stellar Hairpin: Stellar
    { 10012, { 5015, 1 } },             -- Tiny Egg Shell: Egg Shell
    { 10014, { 912, 2 }, { 2208, 1 }, zeny = 400 }, -- Rocker Glasses: 2 Zargons, Ribbon, 400z (:614, :634)
  },
}

local function name(id) return (item_name(id):gsub("_", " ")) end

local function needs(recipe)
  local parts = {}
  for i = 2, #recipe do
    parts[#parts + 1] = recipe[i][2] .. " " .. name(recipe[i][1])
  end
  if recipe.zeny then parts[#parts + 1] = recipe.zeny .. " Zeny" end
  return table.concat(parts, " and ")
end

for _, t in ipairs(TAMERS) do
  local who = "[" .. t.who .. "]"
  npc {
    map = t.map, x = t.x, y = t.y, dir = 4, sprite = 125, name = "Monster Tamer",
    src = "m631:conf/npc_event_mobtim.txt:" .. t.line,
    on_click = function()
      mes(who)
      mes "The age of monster taming is here!"
      mes "To get through to a cute monster, don't you feel the need to collect the treats monsters love?!"
      next()
      local list = { "About ^3355FF'monster taming'^000000" }
      for i, r in ipairs(t) do list[i + 1] = "Order ^3355FF'" .. name(r[1]) .. "'^000000" end
      list[#list + 1] = "End the conversation"
      local pick = menu(list)
      if pick == 1 then
        mes(who)
        mes "All the monster treats you must have ready if you want to get on with the monster you like!"
        mes "We of the Monster Tamer guild supply them on easy terms!"
        next()
        mes(who)
        mes "A wonderful system that puts some very cool items within easy reach:"
        for _, r in ipairs(t) do mes("^3355FF" .. name(r[1]) .. "^000000: " .. needs(r) .. ".") end
        mes "Won't you try for these fabled items?!"
        return close()
      end
      local r = pick and t[pick - 1]
      if not r then
        mes(who)
        mes "It seems you haven't decided yet which monster to keep by your side."
        mes "It may become your partner for life, so don't decide lightly."
        mes "Think it over, and come and see me again when you have made up your mind."
        return close()
      end
      mes(who)
      mes "An item far beyond anything made for humans!"
      mes("Its name: ^3355FF'" .. name(r[1]) .. "'^000000!!")
      mes "A treat exactly to the taste of monsters..."
      next()
      for i = 2, #r + 1 do
        if (r[i] and count_item(r[i][1]) < r[i][2]) or (not r[i] and player.zeny < (r.zeny or 0)) then
          mes(who)
          mes("For you who are raising a monster that loves " .. name(r[1]) .. ", here is how it is made:")
          mes("^3355FF" .. needs(r) .. "!^000000")
          mes "Bring that to me and, on my pride as a Monster Tamer, I will make it for you on the spot!"
          return close()
        end
      end
      mes(who)
      mes("Oh! You have everything it takes to make " .. name(r[1]) .. ".")
      mes "Splendid!"
      mes "On my pride as a Monster Tamer, I will make it for you at once."
      next()
      mes "^3355FF- grind grind, splash splash, clatter clatter -^000000"
      next()
      mes(who)
      mes "There, it is done!"
      mes "Take good care of the cute monster you are raising."
      mes(name(r[1]) .. "!")
      for i = 2, #r do take_item(r[i][1], r[i][2]) end
      player.zeny = player.zeny - (r.zeny or 0)
      give_item(r[1], 1)
      close()
    end,
  }
end

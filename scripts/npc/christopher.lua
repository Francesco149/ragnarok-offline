-- Christopher, the smith's supplier in Geffen: 817's script (cities/geffen.txt:2),
-- mended where it could not be played. Everything else, wording included, is 817's.
--
-- * Short of zeny, every purchase jumped to a label `NoZeny` the file lacks (its
--   line is under `NoZenys`), so the script ended without a word: now it says
--   817's line.
-- * The Mini Furnace's weight test read `GetMaxWeight` and `GetWeight`, plain
--   variables that are always 0, so it always failed and the furnace could not be
--   bought at all. Now it is the weight test jAthena's script makes (//0505,
--   2003-10-30, conf/npc_town_geffen.txt: checkweight), with that script's words
--   for it, translated ("君…顔色悪いな…品物を受け取る前に、荷物を整理した方がいい。").
-- * Buying Emveretarcon went on to the Phracon purchase (200z, Phracon): now it
--   sells Emveretarcon at the 1000z its menu and jAthena's script say.
-- * The Iron Hammer was checked against 1000z and charged 100z: now 1000z, as its
--   menu and jAthena's script say.
--
-- research/THREADS.md #27, docs/DEVIATIONS.md.

local NAME = "[Christopher Guillenrow]"

local function no_zeny()
  mes(NAME)
  mes "I don't think I can let you have this... at lower price. I can't loose my money because of you."
  close()
end

local function thank_you()
  mes "Thank you for shopping at my workshop. Feel free to come anytime whenever you need."
  close()
end

local function farewell()
  mes(NAME)
  mes "Ok, Feel free to come anytime whenever you need. Farewell."
  close()
end

local function deal_cancelled()
  mes(NAME)
  mes "Deal has Canceled. Farewell."
  close()
end

-- sell `n` of `id` at `price` each, or say the zeny is short
local function sell(id, n, price)
  if player.zeny < price * n then
    no_zeny()
    return
  end
  mes(NAME)
  player.zeny = player.zeny - price * n
  give_item(id, n)
  thank_you()
end

local function ask_amount()
  mes(NAME)
  mes "So, how many do you wish to buy? If you want to quit, please note the number as '0'."
  next()
  return input_number()
end

local function anvil(price, id, lines)
  if player.zeny < price then return no_zeny() end
  mes(NAME)
  for _, l in ipairs(lines) do mes(l) end
  player.zeny = player.zeny - price
  give_item(id, 1)
  thank_you()
end

npc {
  map = "geffen_in", x = 110, y = 172, dir = 2, sprite = 63, name = "Christopher",
  replaces = "817:cities/geffen.txt:2",
  on_click = function()
    mes(NAME)
    mes "Welcome to Christopher's Workshop."
    mes "You can get every stuff to forge here."
    mes "My dear... What business brings you to me?"
    next()
    local c = menu("Purchase Anvil", "Purchase Forging Item", "Purchase Metal", "Purify Rough Ores", "Cancel")
    if c == 1 then
      mes(NAME)
      mes "Better Anvil gives you more chances to make better weapons, you know."
      mes "But it will cost you more money than common ones"
      mes "Just get off your chest and choose the best one for your convenience"
      next()
      c = menu("Anvil - 30000z.", "Oridecon Anvil - 120000z.", "Golden Anvil - 300000z.", "Better Anvil than others", "Cancel")
      if c == 1 then
        return anvil(30000, 986, { "This is the cheapest one but efficient to forge." })
      elseif c == 2 then
        return anvil(120000, 987, { "Oh, buddy you have an eye for anvil.", "This must be the proper anvil for a Blacksmith, right?" })
      elseif c == 3 then
        return anvil(300000, 988, { "This one is the best among stuffs in my workshop!", "With this, you could rule the Blacksmith world." })
      elseif c == 4 then
        mes(NAME)
        mes "Well sorry but I don't have harder one than golden anvil."
        mes "Maybe 'Ringgel' the Legendary Anvil maker would have one... But I don't think you can find him somewhere in this world"
        return close()
      end
      return farewell()
    elseif c == 2 then
      mes(NAME)
      mes "A respectable blacksmith uses fine tools."
      mes "You will become one with my Stuff."
      mes "Choose anything you want."
      next()
      c = menu("Mini-Furnace - 150z.", "Iron Hammer - 1000z.", "Golden Hammer - 3000z.", "Oridecon Hammer - 5000z.", "Cancel")
      if c == 1 then
        mes(NAME)
        mes "It's a tool qequisite for Metal refining!!"
        mes "So, how many do you wish to buy? If you want to quit, please note the number as '0'."
        next()
        local n = input_number()
        if n == 0 then return deal_cancelled() end
        if check_weight(612, n) == 0 then
          mes(NAME)
          mes "You look pale, friend... Better sort out your bag before you take the goods."
          return close()
        end
        return sell(612, n, 150)
      elseif c == 2 then
        return sell(613, 1, 1000)
      elseif c == 3 then
        return sell(614, 1, 3000)
      elseif c == 4 then
        return sell(615, 1, 5000)
      end
      return farewell()
    elseif c == 3 then
      mes(NAME)
      mes "I prepare Every Metals Only High Quality one."
      mes "Now then. Which one do you want?"
      next()
      c = menu("Phracon - 200z.", "Emveretarcon - 1000z.", "Cancel")
      if c == 1 or c == 2 then
        local n = ask_amount()
        if n == 0 then return deal_cancelled() end
        if c == 1 then return sell(1010, n, 200) end
        return sell(1011, n, 1000)
      end
      return farewell()
    elseif c == 4 then
      mes(NAME)
      mes "I can purify Oridecon and Elunium."
      mes "I make a cutting Ore out of 5 each rough ones."
      mes "Well... Which one do you want to make?"
      next()
      c = menu("Make Oridecon", "Make Elunium", "Cancel")
      if c == 1 then
        mes(NAME)
        if count_item(756) < 5 then
          mes "I told you I need 5 rough Oridecons for one Oridecon."
          return close()
        end
        take_item(756, 5)
        give_item(984, 1)
        mes "*rumble*..."
        mes "Theres your Oridecon!"
        return close()
      elseif c == 2 then
        mes(NAME)
        if count_item(757) < 5 then
          mes "I told you I need 5 rough Eluniums for one Eluniums."
          return close()
        end
        take_item(757, 5)
        give_item(985, 1)
        mes "*rumble*"
        mes "Theres your Elunium!."
        return close()
      end
      return farewell()
    end
    close()
  end,
}

-- The Trader in Aldebaran's inn (four headgears): 817's words
-- (quests/headgearquest.txt:230) with the items his own list names.
--
-- 817's code took an Apple o' Archer (2285) for the Doctor Band where it had
-- counted the Red Bandana (2275), never asked for the Cracked Diamond its list
-- names, and counted and took Iron for the Opera Masque where its list says
-- Steel. mod0631 loads the same NPC (conf/npc_event_making.txt, 2003-12-11),
-- whose code takes what the list says: so do we.
--
-- docs/DEVIATIONS.md.

local DEALS = {
  { give = 2273, take = { { 2275, 1 }, { 998, 50 }, { 733, 1 } }, zeny = 3500, -- Doctor Band
    say = { "Hmm! Have.. Any Medical License?! Alright. I've heard about an well-known unlicensed physician `Cuwaki'.. I hope you won't get caught...",
            "Yeah... It's your business whatever. Take this." } },
  { give = 5018, take = { { 2247, 1 }, { 916, 300 } }, zeny = 500, -- Feather Bonnet
    say = { "Umhahaha. You have good sense, Dude.",
            "I know you got a hard time to collect these stuffs, but this one is worth to deserve. Take it." } },
  { give = 2281, take = { { 999, 20 }, { 707, 1 } }, zeny = 5000, -- Opera Masque
    say = { "This is pretty nice one.",
            "Well little bit Scary though.. I think it has an Magnetic Attraction. What about your idea?",
            ".. You like it ..Alright.. Take it!" } },
  { give = 2280, take = { { 1019, 120 } }, zeny = 10000, -- Sakkat
    say = { "If you have a chance to visit the Uplander Village, Payon, Please go meet the Sakkat Craftsman.",
            "He've never sold Sakkat to other Dealers but to me. Because only me, can recognise its quality. Due its rarity, Sakkat becomes a very Unique and Exceptional product.",
            "OK! Take it!" } },
}

npc {
  map = "aldeba_in", x = 152, y = 166, dir = 4, sprite = 86, name = "Trader",
  replaces = "817:quests/headgearquest.txt:230",
  on_click = function()
    mes "[Trader]"
    mes "Muhahaha! I am the Very Famous one,Enigmatic Dealer!"
    mes "I am Always bustling in and out of Midgard Continent!"
    mes "Alright.. Here's my Unique and Rare Products from all over the world,You just watch and enjoy!"
    next()
    mes "(1) · ^3355FFDoctor Band^000000 : 1 Red Bandana + 50 Iron + 1 Cracked Diamond + 3500 Zeny"
    mes "(2) · ^3355FFFeather Bonnet^000000 : 1 Romantic Gent + 300 Feather of Birds + 500 Zeny"
    mes "(3) · ^3355FFPhantom of Opera^000000 : 20 Steel + 1 Singing Plant + 5000 Zeny"
    mes "(4) · ^3355FFSakkat^000000 : 120 Trunk + 10000 Zeny"
    next()
    local d = DEALS[menu("Doctor Hat", "Feather Bonnet", "Phantom of Opera", "Sakkat")]
    if not d then return close() end
    local ok = player.zeny >= d.zeny
    for _, t in ipairs(d.take) do
      if count_item(t[1]) < t[2] then ok = false end
    end
    if not ok then
      mes "[Trader]"
      mes "You,Dullard. Check this requirements again."
      mes "You Don't look like an idiot though....C'Mon,Man.Get real."
      return close()
    end
    for _, t in ipairs(d.take) do take_item(t[1], t[2]) end
    player.zeny = player.zeny - d.zeny
    mes "[Trader]"
    for _, l in ipairs(d.say) do mes(l) end
    give_item(d.give, 1)
    close()
  end,
}

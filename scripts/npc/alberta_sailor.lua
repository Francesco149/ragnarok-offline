-- The sailor on Alberta's quay: 817's, without the two voyages this client
-- has no maps for.
--
-- 817 (cities/alberta.txt:479) sells passage to the Sunken Ship (250z),
-- Izlude (500z), Turtle Island (800z), Amatsu (1000z) and Gonryun (1500z).
-- Amatsu and Gonryun are later, Japanese and Taiwanese, additions (817 took
-- them from jAthena); the December 2003 client has neither map, so the fare
-- went and the ship did not. Places, fares and wording are 817's otherwise.
-- A voyage is paid for once it has begun: 817 took the fare first.
--
-- Turtle Island is not sold here any more: 817's 800z passage set you down
-- at the mouth of the island's cave (tur_dun01 154,242) with nobody to bring
-- you back. mod0631's sailor Gotanblue takes you to the beach for 10000z
-- once the old man in the inn has sent you, and waits there for the return
-- (scripts/npc/turtle_island.lua, research/THREADS.md #24).

local TRIPS = {
  { "Sunken Ship -> 250 Zeny.", 250, "250", "alb2trea", 43, 53 },
  { "Izlude Marina -> 500 Zeny", 500, "500", "izlude", 176, 182 },
}

npc {
  map = "alberta", x = 189, y = 151, dir = 5, sprite = 100, name = "Sailor",
  replaces = "817:cities/alberta.txt:479",
  on_click = function()
    mes "[Sailor]"
    mes "Where do you wanna go?"
    next()
    local list = {}
    for i, t in ipairs(TRIPS) do list[i] = t[1] end
    list[#list + 1] = "Quit"
    local t = TRIPS[menu(list)]
    if not t then return close() end
    if player.zeny < t[2] then
      mes "[Sailor]"
      mes("I'm sorry but i told you i would need " .. t[3] .. " Zeny, and it looks like you dont have it.")
      mes "Please come back later when you have this."
      return close()
    end
    player.zeny = player.zeny - t[2]
    warp(t[4], t[5], t[6])
    close()
  end,
}

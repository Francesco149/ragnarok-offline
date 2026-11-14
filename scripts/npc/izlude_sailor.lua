-- The sailor on Izlude's pier: 817's (cities/izlude.txt:332) with mod0631's test
-- of the fare.
--
-- 817 took the fare without looking at the purse: zeny cannot go below 0, so
-- with too little the trip cost whatever one had. mod0631's sailor
-- (conf/npc_town_izlude.txt, same cell) tests it and says the fare again; his
-- line is translated. 817's third choice, "Turtle Island", answered "Under
-- development": an emulator's placeholder, not in mod0631, so it is left out
-- (the island is reached from Alberta: scripts/npc/turtle_island.lua).
--
-- docs/DEVIATIONS.md.

local TRIPS = {
  { "Byalan Island -> 150 Zeny.", 150, "izlu2dun", 107, 50 },
  { "Alberta Marina -> 500 Zeny.", 500, "alberta", 188, 169 },
}

npc {
  map = "izlude", x = 201, y = 181, dir = 2, sprite = 100, name = "Sailor",
  replaces = "817:cities/izlude.txt:332",
  on_click = function()
    mes "[Sailor]"
    mes "Hey Everybody, Attention! Come and Ride the Wind with a Fascinated Ship!!! Come on! Hurry up!"
    next()
    local t = TRIPS[menu(TRIPS[1][1], TRIPS[2][1], "Cancel")]
    if not t then return close() end
    if player.zeny < t[2] then
      mes "[Sailor]"
      mes(t[2] .. " Zeny... I said " .. t[2] .. " Zeny!")
      return close()
    end
    player.zeny = player.zeny - t[2]
    warp(t[3], t[4], t[5])
    close()
  end,
}

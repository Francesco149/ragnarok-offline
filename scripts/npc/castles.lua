-- Guild castles in a world with one player.
--
-- Nobody owns a castle here, and nobody can: guilds need other people
-- (docs/DEVIATIONS.md, research/THREADS.md #17). What a castle without an
-- owner looked like in early 2004 is period behaviour, though:
--
--   * kRO, notice of 2004-02-10: when the guild that owned a castle disbanded,
--     "monsters appeared in large numbers" in it, and guilds disbanded and
--     re-formed to level on them. That patch stopped the summoning.
--   * eAthena 817 (2004-02-19) npc/mobs/gvg.txt: at each War of Emperium
--     start, every castle without an owner gets its monsters (Raydric
--     Archers, Khalitzburgs, Abysmal Knights, an Owl Baron or two...).
--
-- scripts/import/npc/castles/monsters.lua is that file, translated. Its
-- callers wait for "OnAgitStart"; there is no siege schedule in this game, so
-- every server start counts as one: a castle that has none of its monsters
-- left gets them again. `[game.content] castle_monsters = false` leaves the
-- castles empty, as they were on kRO after 2004-02-10.
--
-- Not here, on purpose: the Emperium, ownership, treasure rooms, the way
-- into the guild dungeons. Those are a design decision, not a port.

if not content("castle_monsters") then return end

local CASTLES = {
  "aldeg_cas01", "aldeg_cas02", "aldeg_cas03", "aldeg_cas04", "aldeg_cas05",
  "gefg_cas01", "gefg_cas02", "gefg_cas03", "gefg_cas04", "gefg_cas05",
  "payg_cas01", "payg_cas02", "payg_cas03", "payg_cas04", "payg_cas05",
  "prtg_cas01", "prtg_cas02", "prtg_cas03", "prtg_cas04", "prtg_cas05",
}

npc {
  map = "-", name = "castle_garrison", events = { "OnInit" },
  on_click = function(at)
    if at ~= "OnInit" then return end
    for i, map in ipairs(CASTLES) do
      -- "All": every monster a script put on the map (a reload must not double them)
      if mob_count(map, "All") == 0 then do_event("mobcaller#" .. i .. "::OnAgitStart") end
    end
  end,
}

-- prtg_cas02's inner room is at x 184-203, y 162-182 on this client's map
-- (the CD's patch of 2003-04-30; castle_warps.lua). 817 puts that room's
-- guards on 157,174, the Emperium's cell in the later map, which is rock
-- here: they go to the far end of this client's room instead (our cell).
wrap_npc("817:mobs/gvg.txt:339", function(orig)
  return function(at)
    local monster0 = monster
    monster = function(map, x, y, ...)
      if map == "prtg_cas02" and x == 157 and y == 174 then x, y = 193, 179 end
      return monster0(map, x, y, ...)
    end
    local ok, err = pcall(orig, at)
    monster = monster0
    if not ok then error(err, 0) end
  end
end)

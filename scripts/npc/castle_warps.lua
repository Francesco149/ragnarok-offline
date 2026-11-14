-- A castle warp that led off the map.
--
-- aldeg_cas05's tile at 188,49 sends you "to 165,288" in mod0631
-- (npc_warp25.txt:195) and eAthena 817 (npc/warps/guildcastles.txt:110) alike,
-- but the map is 240x240: `check` lists the arrival as blocked (THREADS.md
-- #26). Its partner tile is 165,232 (npc_warp25.txt:194), and every pair in
-- this castle lands four cells off its partner (156,231 -> 156,227, ...), so
-- 288 is a slip for 228, a floor cell on this client's map. rAthena has 228
-- (npc/warps/guildcastles.txt:389; research round 10, T9-004).
-- docs/DEVIATIONS.md, "fixes to emulator scripts".

remove_warp("aldeg_cas05", 188, 49)
warp { map = "aldeg_cas05", x = 188, y = 49, span = { 1, 1 }, to = { "aldeg_cas05", 165, 228 } }

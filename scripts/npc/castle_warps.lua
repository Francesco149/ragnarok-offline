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

-- prtg_cas02's inner room is elsewhere on this client's map.
--
-- mod0631 (npc_warp25.txt:363-364), 817, eAthena 0.5.2, jAthena and rAthena
-- all send the door at 184,44 to 157,140 and back from 157,135: the inner
-- room of the castle as the May-2004 client draws it (x 142-173, y 132-177).
-- This client's castle comes in the CD's own patch of 2003-04-30
-- (`2003-04-30gdata_turtle_ph_2.gpf`: .gat, .gnd and .rsw), and its inner
-- room is another, smaller one at x 184-203, y 162-182, entered by a
-- dead-end passage at x 188-191, y 162-163 (`research/gat_ascii.py`; research
-- round 18, T9-216/217). The emulators' cells are solid rock there, so the
-- room could not be reached. The pair below is ours, laid out as the later
-- map's: the way back at the passage's end, the arrival a few cells in.
-- No other castle differs between the two clients.
remove_warp("prtg_cas02", 184, 44)
remove_warp("prtg_cas02", 157, 135)
warp { map = "prtg_cas02", x = 184, y = 44, span = { 1, 1 }, to = { "prtg_cas02", 189, 166 } }
warp { map = "prtg_cas02", x = 189, y = 162, span = { 1, 1 }, to = { "prtg_cas02", 184, 40 } }

-- A guild castle for one: take it, hold it, give it up.
--
-- War of Emperium needs guilds and opponents, and this game has neither
-- (research/THREADS.md #17: the owner chose to rework it for one player once
-- the original was understood). What is period here, and from where:
--
--   * an ownerless castle holds monsters (castles.lua) and its Emperium, on
--     eAthena 817's cell (npc/guild/ev_agit_*.txt, 2004-02-16). Breaking the
--     Emperium takes the castle, with 817's announcement. Only a player's
--     weapon hurts it (rules/battle.lua, 817 battle.c:1153);
--   * the steward and what he offers: jAthena's test steward (//0795,
--     2004-02-12, Kalen) in eAthena 0.5.2's English (npc/guild/*.txt,
--     2004-05-18, "Translated By AppleGirl"), with the costs of jAthena's
--     later test script (//0974, 2004-05-16, script/npc/gvg/test/): commerce
--     investment 5,000z x (1 + t(t+1)/2) x (1 + 3n), t = a fifth of the
--     level before today's, n = today's investments, two a day, up to 100;
--     a Kafra contract 10,000z. The iRO guide's War of Emperium page
--     (2004-04) lists the same services;
--   * the castle Kafra: 817's (cells, the one town she sends you to, 200z,
--     the cart), storing free as the iRO guide says ("free storage for guild
--     members"; 817 charged 40z);
--   * the master room, its daily treasure and the lever out: jAthena //0974
--     (test/tbox.txt): at each new day 2 + (C + 5) / 10 of the castle's first
--     box and 2 + C / 10 of its second, at most 12 of each, C the commerce
--     level; the room's cells are jAthena's, found on this client's maps by
--     research/castle_cells.py (payg_cas01's room is elsewhere on this
--     client's map). What a box holds: data/db/treasure.toml (Ragu Memo's jRO
--     table of 2004-02). kRO, 2003-07-29: only the master reaches the boxes;
--   * the lever into the guild dungeon, for the castle's master only, and the
--     lever out of the master room: jAthena //0974's cells and arrivals
--     (817's have slips: aldeg_cas01's two on one cell, three missing);
--   * giving the castle up empties it, and an empty castle "becomes filled
--     with monsters" (the iRO guide; kRO's notice of 2004-02-10).
--
--   * sieges on Sunday from 18:00 to 20:00 (euRO's War of Emperium of April
--     2004), with the steward's defence investment (jAthena //1046's cost)
--     and the eight guardians each castle could install (jAthena //1046's
--     cells and kinds, 10,000z each; the menu as jAthena notes the official
--     one: each slot's kind, and the HP of one that stands).
--
--   * a flag for each castle on its grounds (817 npc/guild/guild_flag.txt,
--     2004-02-14, "by kalen": cells, sprite 722, the castle each one reads),
--     with eAthena 0.5.2's text for it (npc/guild/*_flag.txt, 2004-05-18,
--     "Scripted by davidsiaw"), which names each kingdom where 817 copied
--     "Albedaran" and "Geffen" onto all twenty; 0.5.2's offer to take the
--     master home. 0.5.2 put its flags in the towns, with names (Kriemhild...)
--     this client's map names do not know: not built;
--   * the master recovers faster in the castle (the iRO guide's page; how
--     much is rAthena's, rules/combat.lua rules.regen).
--
-- Ours (docs/DEVIATIONS.md, "Guild castles"): anyone may break an Emperium
-- (817 wants a guild with Official Guild Approval); an ownerless castle can be
-- taken whenever its Emperium stands, siege or not; the castle belongs to the
-- character, and the announcements name the character where they named the
-- guild; in a siege the attackers are the castle's old garrison (817's list)
-- coming back for a castle that is held, and if they break the Emperium the
-- castle is theirs again (an ownerless castle full of monsters); what defence
-- gives is rAthena's (later); treasure is laid out when the master first comes
-- in on a new day, not at midnight; prtg_cas02's Emperium stands in this
-- client's older inner room (193,181), the emulators' cell being rock.

local C = {
  -- map, name (817 db/castle_db.txt), Kafra's town (817), Emperium (817),
  -- Kafra x y dir (817), steward name x y dir (0.5.2's name, jAthena //1046's cell),
  -- master-room arrival (0.5.2), lever out: cell -> arrival (jAthena //0974),
  -- dungeon lever: cell -> map x y (jAthena //0974), box rectangle (jAthena //0974), boxes
  { "aldeg_cas01", "Luina Guild 1", { "Al De Baran", "aldebaran", 132, 103 }, { 216, 24 },
    { 218, 170, 0 }, { "Arl Fredo", 218, 175, 0 }, { 119, 223 }, { 123, 223, 218, 176 },
    { 211, 179, "gld_dun02", 32, 122 }, { 114, 218, 123, 227 }, 1324 },
  { "aldeg_cas02", "Luina Guild 2", { "Al De Baran", "aldebaran", 132, 103 }, { 214, 24 },
    { 84, 74, 0 }, { "ChenChenLee", 78, 74, 0 }, { 135, 231 }, { 139, 234, 78, 75 },
    { 194, 136, "gld_dun02", 79, 30 }, { 130, 226, 139, 235 }, 1326 },
  { "aldeg_cas03", "Luina Guild 3", { "Al De Baran", "aldebaran", 132, 103 }, { 206, 32 },
    { 118, 76, 0 }, { "Najarf", 110, 118, 0 }, { 225, 269 }, { 229, 267, 110, 119 },
    { 200, 177, "gld_dun02", 165, 38 }, { 220, 264, 229, 273 }, 1328 },
  -- 0.5.2 copies cas05's Brimhemsen here, on cas01's cell; jAthena //1046 has ウィリオット at 67,116
  { "aldeg_cas04", "Luina Guild 4", { "Al De Baran", "aldebaran", 132, 103 }, { 36, 218 },
    { 45, 88, 0 }, { "Wiliot", 67, 116, 0 }, { 85, 12 }, { 83, 17, 67, 117 },
    { 76, 64, "gld_dun02", 160, 148 }, { 80, 8, 89, 17 }, 1330 },
  { "aldeg_cas05", "Luina Guild 5", { "Al De Baran", "aldebaran", 132, 103 }, { 28, 102 },
    { 31, 190, 0 }, { "Brimhemsen", 51, 179, 0 }, { 62, 12 }, { 64, 8, 51, 179 },
    { 22, 205, "gld_dun02", 103, 169 }, { 58, 8, 67, 17 }, 1332 },
  { "gefg_cas01", "Britoniah Guild 1", { "Geffen", "geffen", 120, 39 }, { 198, 182 },
    { 83, 47, 3 }, { "Gnahucher", 40, 48, 5 }, { 155, 112 }, { 152, 117, 40, 49 },
    { 78, 84, "gld_dun04", 39, 258 }, { 150, 108, 159, 117 }, 1334 },
  { "gefg_cas02", "Britoniah Guild 2", { "Geffen", "geffen", 120, 39 }, { 176, 178 },
    { 23, 66, 3 }, { "Esmark", 12, 66, 0 }, { 141, 115 }, { 145, 115, 12, 67 },
    { 167, 40, "gld_dun04", 125, 270 }, { 136, 112, 145, 121 }, 1336 },
  { "gefg_cas03", "Britoniah Guild 3", { "Geffen", "geffen", 120, 39 }, { 245, 167 },
    { 116, 89, 5 }, { "Jyang", 106, 23, 3 }, { 270, 290 }, { 267, 287, 106, 24 },
    { 221, 43, "gld_dun04", 268, 251 }, { 266, 286, 275, 295 }, 1338 },
  { "gefg_cas04", "Britoniah Guild 4", { "Geffen", "geffen", 120, 39 }, { 174, 178 },
    { 59, 70, 3 }, { "Kelbany", 73, 46, 3 }, { 116, 118 }, { 113, 115, 73, 47 },
    { 58, 75, "gld_dun04", 268, 108 }, { 112, 114, 121, 123 }, 1340 },
  { "gefg_cas05", "Britoniah Guild 5", { "Geffen", "geffen", 120, 39 }, { 194, 184 },
    { 61, 52, 5 }, { "Beeor", 70, 52, 3 }, { 144, 110 }, { 141, 107, 70, 53 },
    { 65, 22, "gld_dun04", 230, 35 }, { 140, 106, 149, 115 }, 1342 },
  -- this client's payg_cas01 is 300x160: the master room is at 286-295, 4-13 (jAthena: 295-304, 8-17)
  { "payg_cas01", "Chung-Rim Guild 1", { "Payon", "payon", 82, 128 }, { 139, 139 },
    { 128, 58, 3 }, { "Grunnday", 120, 58, 4 }, { 290, 7 }, { 287, 5, 120, 59 },
    { 101, 25, "gld_dun01", 186, 165 }, { 286, 4, 295, 13 }, 1344 },
  { "payg_cas02", "Chung-Rim Guild 2", { "Payon", "payon", 82, 128 }, { 39, 25 },
    { 22, 275, 5 }, { "Cherrios", 22, 260, 7 }, { 145, 143 }, { 141, 141, 22, 261 },
    { 278, 247, "gld_dun01", 54, 165 }, { 140, 140, 149, 149 }, 1346 },
  -- 0.5.2 puts Garriet beside the Emperium (268,264); the lever out leads to 50,261, jAthena's cell
  { "payg_cas03", "Chung-Rim Guild 3", { "Payon", "payon", 82, 128 }, { 269, 265 },
    { 9, 263, 5 }, { "Garriet", 50, 261, 3 }, { 158, 168 }, { 155, 165, 50, 261 },
    { 38, 42, "gld_dun01", 54, 39 }, { 154, 164, 163, 173 }, 1348 },
  { "payg_cas04", "Chung-Rim Guild 4", { "Payon", "payon", 82, 128 }, { 271, 29 },
    { 40, 235, 1 }, { "DJ", 38, 284, 3 }, { 146, 48 }, { 143, 45, 38, 285 },
    { 52, 48, "gld_dun01", 186, 39 }, { 142, 44, 151, 53 }, 1350 },
  { "payg_cas05", "Chung-Rim Guild 5", { "Payon", "payon", 82, 128 }, { 30, 30 },
    { 276, 227, 1 }, { "Najarf", 277, 249, 3 }, { 156, 131 }, { 153, 129, 277, 250 },
    { 238, 11, "gld_dun01", 223, 202 }, { 152, 128, 161, 137 }, 1352 },
  { "prtg_cas01", "Valkyrie Realms 1", { "Prontera", "prontera", 278, 211 }, { 197, 197 },
    { 96, 173, 0 }, { "Abrai", 112, 181, 0 }, { 11, 208 }, { 15, 209, 111, 181 },
    { 94, 200, "gld_dun03", 28, 251 }, { 6, 204, 15, 213 }, 1354 },
  { "prtg_cas02", "Valkyrie Realms 2", { "Prontera", "prontera", 278, 211 }, { 193, 181 },
    { 71, 36, 4 }, { "Rhay", 94, 61, 4 }, { 202, 228 }, { 206, 228, 94, 62 },
    { 84, 72, "gld_dun03", 164, 268 }, { 198, 224, 207, 233 }, 1356 },
  { "prtg_cas03", "Valkyrie Realms 3", { "Prontera", "prontera", 278, 211 }, { 17, 221 },
    { 181, 215, 4 }, { "Stick", 51, 100, 4 }, { 189, 132 }, { 193, 130, 51, 101 },
    { 5, 70, "gld_dun03", 164, 179 }, { 184, 128, 193, 137 }, 1358 },
  { "prtg_cas04", "Valkyrie Realms 4", { "Prontera", "prontera", 278, 211 }, { 292, 14 },
    { 258, 247, 4 }, { "Bandred", 259, 265, 4 }, { 271, 162 }, { 274, 161, 259, 265 },
    { 56, 283, "gld_dun03", 268, 203 }, { 266, 158, 275, 167 }, 1360 },
  { "prtg_cas05", "Valkyrie Realms 5", { "Prontera", "prontera", 278, 211 }, { 266, 266 },
    { 52, 41, 4 }, { "Reiner", 36, 37, 4 }, { 277, 178 }, { 280, 177, 36, 38 },
    { 212, 95, "gld_dun03", 199, 28 }, { 272, 174, 281, 183 }, 1362 },
}

local KAFRA_FEE = 10000 -- jAthena //0795 and //0974, 0.5.2
local TELE_FEE = 200    -- 817's castle Kafras
local CART_FEE = 800

-- The castle's state lives in server-wide variables (saved): who holds it
-- (a character id and name), the commerce level, today's investments, the
-- day they and the treasure were counted for, the Kafra contract.
local function key(c, k) return "castle_" .. c[1] .. (k and ("_" .. k) or "") end

-- The day, for investments and treasure (tests replace it).
castle_today = castle_today or function() return os.date("%Y-%m-%d") end

function castle_owner(map)
  return g["castle_" .. map]
end

local function by_map(map)
  for _, c in ipairs(C) do if c[1] == map then return c end end
end

local function owner_name(c) return g[key(c, "name$")] end
local function is_master(c) return g[key(c)] ~= 0 and g[key(c)] == player.char_id end

local function boxes_event(c) return "Agit_" .. c[1] .. "::OnBoxOpen" end
local function emp_event(c) return "Agit_" .. c[1] .. "::OnAgitBreak" end
local function guard_event(c, slot) return "Agit_" .. c[1] .. "::OnGuardian" .. slot end

-- The eight guardians a steward can install: jAthena //1046 (2004-09-12,
-- script/npc/gvg/*.txt, "@GDnum == 10" to 17), cell and kind; three cells
-- that are walls on this client's maps moved one cell (aldeg_cas04 169,53,
-- prtg_cas02 178,44, prtg_cas04 36,61: ours).
local GUARDIANS = {
  aldeg_cas01 = { { 17, 218, 1287 }, { 39, 205, 1285 }, { 38, 196, 1285 }, { 21, 194, 1286 }, { 218, 24, 1286 }, { 213, 24, 1286 }, { 73, 70, 1286 }, { 45, 228, 1285 } },
  aldeg_cas02 = { { 51, 183, 1287 }, { 27, 184, 1286 }, { 88, 43, 1286 }, { 210, 7, 1285 }, { 60, 203, 1287 }, { 21, 177, 1285 }, { 117, 46, 1286 }, { 36, 183, 1285 } },
  aldeg_cas03 = { { 110, 217, 1285 }, { 90, 112, 1286 }, { 86, 120, 1287 }, { 195, 151, 1285 }, { 116, 112, 1286 }, { 116, 76, 1287 }, { 64, 103, 1287 }, { 212, 160, 1285 } },
  aldeg_cas04 = { { 187, 100, 1285 }, { 192, 42, 1285 }, { 55, 88, 1286 }, { 145, 209, 1287 }, { 169, 52, 1287 }, { 198, 77, 1287 }, { 148, 88, 1285 }, { 48, 72, 1286 } },
  aldeg_cas05 = { { 51, 202, 1285 }, { 27, 221, 1286 }, { 145, 78, 1286 }, { 157, 192, 1287 }, { 157, 74, 1287 }, { 188, 79, 1285 }, { 156, 73, 1286 }, { 41, 112, 1286 } },
  gefg_cas01 = { { 67, 179, 1287 }, { 36, 186, 1285 }, { 50, 186, 1285 }, { 62, 41, 1287 }, { 50, 67, 1286 }, { 184, 20, 1287 }, { 189, 41, 1286 }, { 200, 167, 1286 } },
  gefg_cas02 = { { 64, 168, 1287 }, { 35, 148, 1285 }, { 20, 150, 1285 }, { 56, 41, 1287 }, { 19, 41, 1286 }, { 166, 25, 1287 }, { 166, 42, 1286 }, { 159, 188, 1286 } },
  gefg_cas03 = { { 48, 176, 1285 }, { 113, 214, 1285 }, { 48, 207, 1287 }, { 157, 62, 1287 }, { 157, 45, 1286 }, { 243, 41, 1287 }, { 234, 25, 1286 }, { 238, 160, 1286 } },
  gefg_cas04 = { { 53, 191, 1285 }, { 31, 178, 1285 }, { 49, 220, 1287 }, { 29, 46, 1287 }, { 57, 46, 1286 }, { 147, 65, 1287 }, { 160, 50, 1286 }, { 148, 189, 1286 } },
  gefg_cas05 = { { 45, 149, 1285 }, { 71, 163, 1285 }, { 72, 142, 1287 }, { 66, 47, 1287 }, { 66, 17, 1286 }, { 177, 50, 1287 }, { 177, 35, 1286 }, { 193, 166, 1286 } },
  payg_cas01 = { { 229, 92, 1285 }, { 225, 80, 1285 }, { 222, 111, 1285 }, { 99, 45, 1285 }, { 65, 31, 1285 }, { 36, 127, 1285 }, { 51, 144, 1285 }, { 138, 133, 1286 } },
  payg_cas02 = { { 237, 54, 1285 }, { 228, 72, 1285 }, { 210, 41, 1285 }, { 57, 241, 1285 }, { 42, 241, 1285 }, { 287, 257, 1285 }, { 264, 272, 1285 }, { 27, 20, 1286 } },
  payg_cas03 = { { 245, 37, 1285 }, { 269, 51, 1285 }, { 36, 39, 1285 }, { 41, 39, 1285 }, { 39, 65, 1285 }, { 19, 276, 1285 }, { 37, 277, 1285 }, { 268, 244, 1286 } },
  payg_cas04 = { { 251, 212, 1285 }, { 232, 212, 1285 }, { 231, 175, 1285 }, { 32, 287, 1285 }, { 32, 232, 1285 }, { 35, 45, 1285 }, { 36, 17, 1285 }, { 270, 41, 1286 } },
  payg_cas05 = { { 19, 279, 1285 }, { 33, 260, 1285 }, { 266, 234, 1285 }, { 266, 279, 1285 }, { 263, 37, 1285 }, { 263, 21, 1285 }, { 250, 22, 1285 }, { 36, 36, 1286 } },
  prtg_cas01 = { { 182, 68, 1287 }, { 182, 116, 1287 }, { 153, 86, 1287 }, { 59, 28, 1285 }, { 50, 36, 1285 }, { 184, 183, 1286 }, { 196, 189, 1286 }, { 107, 179, 1286 } },
  prtg_cas02 = { { 188, 175, 1286 }, { 198, 175, 1286 }, { 177, 44, 1286 }, { 71, 75, 1287 }, { 49, 28, 1287 }, { 64, 186, 1285 }, { 76, 196, 1285 }, { 75, 175, 1285 } },
  prtg_cas03 = { { 191, 190, 1286 }, { 137, 190, 1286 }, { 45, 99, 1286 }, { 50, 87, 1287 }, { 41, 87, 1287 }, { 191, 42, 1285 }, { 179, 43, 1285 }, { 191, 72, 1285 } },
  prtg_cas04 = { { 276, 14, 1286 }, { 274, 35, 1286 }, { 246, 246, 1286 }, { 38, 240, 1287 }, { 29, 240, 1287 }, { 33, 258, 1287 }, { 78, 48, 1285 }, { 35, 61, 1285 } },
  prtg_cas05 = { { 266, 262, 1286 }, { 287, 280, 1286 }, { 245, 250, 1286 }, { 236, 63, 1287 }, { 251, 63, 1287 }, { 278, 71, 1287 }, { 32, 253, 1285 }, { 44, 248, 1285 } },
}
local GUARDIAN_KIND = { [1285] = "Archer Guardian", [1286] = "Knight Guardian", [1287] = "Soldier Guardian" }
local GUARDIAN_FEE = 10000 -- jAthena //0795, //0974, //1046

-- The castles' flags on their grounds: 817 guild_flag.txt (x, y, dir, map: the map
-- last, or `check` reads the cell as a warp destination).
-- alde_gld's five stand on blocked cells beside the path, as in 817.
local FLAGS = {
  prtg_cas01 = { 160, 210, 4, "prt_gld" }, prtg_cas02 = { 153, 213, 3, "prt_gld" },
  prtg_cas03 = { 150, 220, 2, "prt_gld" }, prtg_cas04 = { 169, 220, 6, "prt_gld" },
  prtg_cas05 = { 167, 213, 5, "prt_gld" },
  payg_cas01 = { 347, 125, 2, "pay_gld" }, payg_cas02 = { 347, 116, 2, "pay_gld" },
  payg_cas03 = { 341, 106, 0, "pay_gld" }, payg_cas04 = { 330, 106, 0, "pay_gld" },
  payg_cas05 = { 318, 106, 0, "pay_gld" },
  aldeg_cas01 = { 257, 153, 7, "alde_gld" }, aldeg_cas02 = { 257, 166, 5, "alde_gld" },
  aldeg_cas03 = { 262, 153, 7, "alde_gld" }, aldeg_cas04 = { 262, 166, 5, "alde_gld" },
  aldeg_cas05 = { 267, 153, 7, "alde_gld" },
  gefg_cas01 = { 330, 199, 2, "gef_fild07" }, gefg_cas02 = { 330, 204, 2, "gef_fild07" },
  gefg_cas03 = { 330, 176, 2, "gef_fild07" }, gefg_cas04 = { 330, 171, 2, "gef_fild07" },
  gefg_cas05 = { 330, 166, 2, "gef_fild07" },
}
-- 0.5.2's kingdom a castle's flag speaks for
local KINGDOM = { aldeg = "Al De Baran", gefg = "Geffen", payg = "Payon", prtg = "Prontera" }

-- A castle's master recovers HP and SP twice as fast in it (rules.regen).
local HOME_REGEN = 200

-- What defence investment gives: rAthena pre-renewal (status.cpp, status_calc_mob,
-- tier later; the iRO guide's page says only "the castle Emperium's defense and HP
-- as well as the Guardian's HP"; jAthena //1046 cuts damage by defense/100 and
-- doubts it, "らしい?"): +1000 HP a level for the Emperium, +2000 for a guardian.
local function defended(c, id, per_level)
  local u = battle.get(id)
  if u then mob_max_hp(id, u.max_hp + per_level * g[key(c, "def")]) end
  mob_side(id, 2)
end

-- A new day: investments start again, and the master room gets that day's treasure.
local function new_day(c)
  if not map_size(c[1]) then return end
  local today = castle_today()
  if g[key(c, "day$")] == today then return end
  g[key(c, "day$")] = today
  g[key(c, "te")] = 0
  g[key(c, "td")] = 0
  kill_monster(c[1], boxes_event(c))
  if g[key(c)] == 0 then return end
  local eco, r = g[key(c, "eco")], c[10]
  local n1 = math.min(12, 2 + (eco + 5) // 10)
  local n2 = math.min(12, 2 + eco // 10)
  area_monster(c[1], r[1], r[2], r[3], r[4], "Treasure Chest", c[11], n1, boxes_event(c))
  area_monster(c[1], r[1], r[2], r[3], r[4], "Treasure Chest", c[11] + 1, n2, boxes_event(c))
end

-- An ownerless castle's Emperium: anyone may break it.
local function emperium(c)
  if not map_size(c[1]) then return end -- a client (or a test world) without the castle
  if mob_count(c[1], emp_event(c)) == 0 then
    monster(c[1], c[4][1], c[4][2], "Emperium", 1288, 1, emp_event(c))
  end
end

-- A held castle's Emperium in a siege: on its master's side, stronger by the defence.
local function emperium_defended(c)
  if not map_size(c[1]) then return end
  kill_monster(c[1], emp_event(c))
  for _, id in ipairs(monster(c[1], c[4][1], c[4][2], "Emperium", 1288, 1, emp_event(c))) do
    defended(c, id, 1000)
  end
end

local function spawn_guardian(c, slot)
  local gd = GUARDIANS[c[1]][slot]
  kill_monster(c[1], guard_event(c, slot))
  for _, id in ipairs(monster(c[1], gd[1], gd[2], "--ja--", gd[3], 1, guard_event(c, slot))) do
    defended(c, id, 2000)
  end
end

-- Every monster in the castle that is not its own (Emperium, guardians, treasure)
-- joins the attack, or (side 0) goes back to being the castle's garrison.
-- Attackers come for the Emperium along the castle's corridors (a walking-distance
-- map, the engine's siege_field), each setting out at a random moment within
-- `spread` ms; one that starts within 20 cells of the Emperium is put elsewhere
-- first (817 calls part of the garrison onto the Emperium's own cell, which
-- would break it in seconds). All of this is ours. A castle's sections are joined
-- by warp portals, which monsters do not take: attackers spawned beyond one stay
-- there and fight only the player who comes through. Every monster on the map
-- ("*"), not only the script's: an MVP's summoned slaves carry no event, and
-- those whose master a garrison reset took away (killmonster leaves slaves alive
-- in mod0631 and 817, mob.c:1474) stood by the Emperium out of the siege
-- (seen in the client, 2026-11-15).
local function set_attackers(c, side, spread)
  if side == 1 then siege_field(c[1], c[4][1], c[4][2]) end
  for _, id in ipairs(mob_ids(c[1], "*")) do
    local u = battle.get(id)
    if u and u.side ~= 2 and u.class ~= 1288 and not (u.class >= 1324 and u.class <= 1363) then
      mob_side(id, side)
      if side == 1 then
        if spread and math.max(math.abs(u.x - c[4][1]), math.abs(u.y - c[4][2])) < 20 then
          for _ = 1, 5 do -- a random cell, away from the Emperium
            _rt("mob_warp", id)
            local v = battle.get(id)
            if v and math.max(math.abs(v.x - c[4][1]), math.abs(v.y - c[4][2])) >= 20 then break end
          end
        end
        mob_march(id, rand(0, spread or 300000))
      else
        mob_march(id, -1)
      end
    end
  end
end

local function clear(c)
  for _, k in ipairs({ "", "name$", "eco", "te", "def", "td", "kafra", "day$" }) do
    g[key(c, k ~= "" and k or nil)] = (k:sub(-1) == "$") and "" or 0
  end
  for slot = 1, 8 do
    g[key(c, "gd" .. slot)] = 0
    kill_monster(c[1], guard_event(c, slot))
  end
  kill_monster(c[1], boxes_event(c))
  disable_npc("castle_kafra_" .. c[1])
end

-- Called by castles.lua once a castle has its garrison back.
function castle_unowned_setup(map)
  local c = by_map(map)
  if c and g[key(c)] == 0 then emperium(c) end
end

-- ---------------------------------------------------------------- sieges
-- War of Emperium on euRO: "Sonntag, 11 April 18Uhr" (the open beta's news,
-- research/notes/euro-history.md, W14); two hours, as 817's own schedule
-- (ev_agit_event.txt 21:00-23:00) and the Malaysian record (woe-mro-2004.md).
-- In a game of one, the attack on a held castle is the castle's old garrison
-- coming back (ours): break its Emperium and the castle is theirs again, as an
-- ownerless castle held monsters (kRO's notice of 2004-02-10). The server's
-- local clock is the player's. `@siege` starts or ends one by hand.
local SIEGES = { { wday = 1, hour = 18, len = 2 * 3600 } } -- os.date wday: 1 = Sunday
castle_now = castle_now or os.time

local function window_end(now)
  local t = os.date("*t", now)
  for _, w in ipairs(SIEGES) do
    if t.wday == w.wday then
      local from = os.time { year = t.year, month = t.month, day = t.day, hour = w.hour, min = 0, sec = 0 }
      if now >= from and now < from + w.len then return from + w.len end
    end
  end
end

local function siege_castle(i, c)
  if not map_size(c[1]) or g[key(c)] == 0 then return end
  emperium_defended(c)
  -- what is left of the old garrison joins the new one rather than doubling it
  -- (slaves too: "*", see set_attackers)
  for _, id in ipairs(mob_ids(c[1], "*")) do
    local u = battle.get(id)
    if u and u.side == 0 and u.class ~= 1288 and not (u.class >= 1324 and u.class <= 1363) then mob_vanish(id) end
  end
  if content("castle_monsters") then
    do_event("castle_garrison::OnSiege" .. i) -- the garrison, then OnSiegeMarch below
  else
    set_attackers(c, 1, 3600000)
  end
end

function castle_siege_start(until_t)
  g.siege_until = until_t
  if g.siege_on == 1 then return end
  g.siege_on = 1
  announce("The War of Emperium has begun!", 0)
  for i, c in ipairs(C) do siege_castle(i, c) end
end

function castle_siege_end()
  if g.siege_on ~= 1 then return end
  g.siege_on = 0
  g.siege_until = 0
  for _, c in ipairs(C) do
    if map_size(c[1]) and g[key(c)] ~= 0 then
      -- the castle held: its Emperium goes back, the attackers withdraw
      kill_monster(c[1], emp_event(c))
      for _, id in ipairs(mob_ids(c[1], "*")) do
        local u = battle.get(id)
        if u and u.side == 1 then mob_vanish(id) end
      end
    end
    if map_size(c[1]) then siege_field(c[1]) end
  end
  announce("The War of Emperium is over.", 0)
end

npc {
  map = "-", name = "castle_siege", events = { "OnInit", "OnTimer60000" },
  on_click = function(at)
    npc_timer_start()
    local now = castle_now()
    if g.siege_on == 1 and now >= g.siege_until then return castle_siege_end() end
    if at == "OnInit" and g.siege_on == 1 then
      -- the server was started again mid-siege: the attack goes on
      for i, c in ipairs(C) do siege_castle(i, c) end
      return
    end
    local till = window_end(now)
    if till and g.siege_on ~= 1 and g.siege_done ~= till then
      g.siege_done = till -- once a window, even if ended early by hand
      castle_siege_start(till)
    end
  end,
}

command("siege", function(args)
  if args == "end" or (args == "" and g.siege_on == 1) then
    castle_siege_end()
    return "siege over"
  end
  castle_siege_start(castle_now() + 2 * 3600)
  return "siege on for two hours"
end)

local function lose(c)
  local was = owner_name(c)
  clear(c)
  -- ours: the period announcement names the guild that took it
  announce("Guild Base [" .. c[2] .. "] has fallen to its old garrison.", 0)
  map_announce(c[1], "Emperium has been destroyed.", 17)
  set_attackers(c, 0) -- they hold it now, as an ownerless castle's monsters
  emperium(c)
end

for i, c in ipairs(C) do
  local map = c[1]
  local stew = c[6]
  local S = "[ " .. stew[1] .. " ]"
  local events = { "OnInit", "OnAgitBreak", "OnBoxOpen", "OnSiegeMarch" }
  for slot = 1, 8 do events[#events + 1] = "OnGuardian" .. slot end

  -- the castle itself: the Emperium's events, the treasure's, the guardians'
  npc {
    map = "-", name = "Agit_" .. map, events = events,
    on_click = function(at)
      if at == "OnInit" then
        if g[key(c)] == 0 then
          emperium(c)
        elseif map_size(map) then
          for slot = 1, 8 do
            if g[key(c, "gd" .. slot)] == 1 and mob_count(map, guard_event(c, slot)) == 0 then spawn_guardian(c, slot) end
          end
        end
      elseif at == "OnSiegeMarch" then
        if g[key(c)] ~= 0 and g.siege_on == 1 then set_attackers(c, 1, 3600000) end
      elseif at:match("^OnGuardian%d$") then
        g[key(c, "gd" .. at:sub(-1))] = 0 -- fallen: the steward may install it again
      elseif at == "OnAgitBreak" then
        local ok, cid = pcall(function() return player.char_id end)
        if not ok or not cid then
          -- broken by the attackers of a siege
          if g[key(c)] ~= 0 then lose(c) end
          return
        end
        g[key(c)] = player.char_id
        g[key(c, "name$")] = player.name
        tmp.regen_rate = HOME_REGEN
        g[key(c, "eco")] = 0
        g[key(c, "te")] = 0
        g[key(c, "def")] = 0
        g[key(c, "td")] = 0
        g[key(c, "kafra")] = 0
        g[key(c, "day$")] = castle_today() -- the first treasure comes with the next day, as at jAthena's midnight
        kill_monster(map, boxes_event(c))
        disable_npc("castle_kafra_" .. map)
        -- 817 ev_agit_*.txt, "[guild]" there
        announce("Guild Base [" .. c[2] .. "] has been taken by [" .. player.name .. "].", 0)
        map_announce(map, "Emperium has been destroyed.", 17)
        if g.siege_on == 1 then
          -- taken in a siege: a new Emperium to hold until it ends (817 OnAgitEliminate),
          -- and what is left of the garrison attacks it
          emperium_defended(c)
          set_attackers(c, 1)
        end
      end
    end,
  }

  -- the steward
  npc {
    map = map, x = stew[2], y = stew[3], dir = stew[4], sprite = 55, name = stew[1], id = "castle_steward_" .. map,
    on_click = function()
      if g[key(c)] == 0 then
        mes(S)
        mes "I am waiting for my master."
        mes "Brave Player! follow your destiny!..."
        close()
        return
      end
      if not is_master(c) then
        mes(S)
        mes("I am here to follow ^ff0000" .. owner_name(c) .. "^000000 's command!")
        mes "Where are you guardians! destroy these bastards!"
        close()
        return
      end
      new_day(c)
      mes(S)
      mes("Welcome Master ^ff0000" .. player.name .. "^000000 !")
      mes "I'll do anything you tell me to do sir!."
      next()
      local pick = menu("Guild Base Property", "Commerce Investment", "Defence Investment", "Guardian Installation",
        "Kafra Service Employment / Dismissal", "Enter Master's Room", "Empty Guild Base")
      if pick == 1 then
        mes(S)
        mes "Guild Base Commerce Investment Improvement Information."
        next()
        mes(S)
        mes("^0000ffCurrent Commerce Investment is " .. g[key(c, "eco")] .. ".")
        mes("- You have invested " .. g[key(c, "te")] .. " times today.^000000")
        next()
        mes(S)
        mes("^0000ffCurrent Defence is " .. g[key(c, "def")] .. ".")
        mes("- You have invested " .. g[key(c, "td")] .. " times today.^000000")
        mes "That is about it."
        close()
      elseif pick == 2 then
        mes(S)
        mes "If you improve your Commerce Investments, Guild's Productive power increases to produce goods."
        next()
        mes(S)
        mes "You can invest two times a day."
        next()
        local te, eco = g[key(c, "te")], g[key(c, "eco")]
        if te >= 2 or eco >= 100 then
          mes(S)
          mes "^ff0000Guild's Investment is currently at 100%"
          mes "you don't have to invest anymore.^000000"
          close()
          return
        end
        local t = (eco - te) // 5
        local cost = 5000 * (1 + (1 + t) * t // 2) * (1 + 3 * te)
        mes(S)
        mes("Required invest amount is " .. cost .. " Zeny.")
        mes "Would you like to invest?"
        next()
        if menu("Invest Commerce.", "Cancel.") ~= 1 then
          mes(S)
          mes "I'll follow your order Master!... you don't have to be hurry anywayz."
          close()
          return
        end
        if player.zeny < cost then
          mes(S)
          mes "Master, you do not have enough money to invest, investment has been cancelled."
          close()
          return
        end
        player.zeny = player.zeny - cost
        g[key(c, "te")] = te + 1
        g[key(c, "eco")] = eco + 1
        mes(S)
        mes "You invested successfully."
        close()
      elseif pick == 3 then
        mes(S)
        mes "If you improve investment of defence, HP of guardian/emperium increase, it helps when enemy try to attack your guild."
        next()
        mes(S)
        mes "You can invest two times a day."
        next()
        local td, def = g[key(c, "td")], g[key(c, "def")]
        if td >= 2 or def >= 100 then
          mes(S)
          mes "^ff0000Guild's Investment is currently at 100%"
          mes "you don't have to invest anymore.^000000"
          close()
          return
        end
        -- jAthena //1046 Steward_menu3 (and //0974's test script)
        local t = (def - td) // 5
        local cost = 10000 * (1 + (1 + t) * t // 2) * (1 + 3 * td)
        mes(S)
        mes("Required invest amount is " .. cost .. " Zeny.")
        mes "Would you like to invest?"
        next()
        if menu("Invest Defence.", "Cancel") ~= 1 then
          mes(S)
          mes "I'll follow your order Master!... you don't have to be hurry anywayz."
          close()
          return
        end
        if player.zeny < cost then
          mes(S)
          mes "Master, you do not have enough money to invest, investment has been cancelled."
          close()
          return
        end
        player.zeny = player.zeny - cost
        g[key(c, "td")] = td + 1
        g[key(c, "def")] = def + 1
        mes(S)
        mes "You invested successfully."
        close()
      elseif pick == 4 then
        mes(S)
        mes "Would you like to install a guardian? Guardian will protect guild base from enemies."
        mes "Please choose guardian."
        next()
        -- the official menu, as jAthena //1046 notes it ("実際のMenu"): each slot's
        -- kind, and the HP of one that stands
        local opts = {}
        for slot, gd in ipairs(GUARDIANS[map]) do
          local hp
          for _, id in ipairs(mob_ids(map, guard_event(c, slot))) do
            local u = battle.get(id)
            if u then hp = u.hp .. " / " .. u.max_hp end
          end
          opts[slot] = GUARDIAN_KIND[gd[3]] .. (hp and (" - (" .. hp .. ")") or " - Installable")
        end
        local slot = menu(table.unpack(opts))
        if not slot then return end
        if g[key(c, "gd" .. slot)] == 1 then
          mes(S)
          mes "Master...you installed that guardian already."
          close()
          return
        end
        mes(S)
        mes("Would you really like to install guardian? You need " .. GUARDIAN_FEE .. " zeny to install.")
        next()
        if menu("Install", "Cancel") ~= 1 then
          mes(S)
          mes "I'll follow your order sir, but it is worth to have a guardian soon or later."
          close()
          return
        end
        if player.zeny < GUARDIAN_FEE then
          mes(S)
          mes "Master, you do not have enough money to invest, investment has been cancelled."
          close()
          return
        end
        player.zeny = player.zeny - GUARDIAN_FEE
        g[key(c, "gd" .. slot)] = 1
        spawn_guardian(c, slot)
        mes(S)
        mes "Guardian has been installed, guardian will protect our guild base against enemies."
        close()
      elseif pick == 5 then
        if g[key(c, "kafra")] == 1 then
          mes(S)
          mes "Would you like to dismiss the current Kafra Service?"
          next()
          if menu("Dismissal", "Cancel") ~= 1 then
            mes(S)
            mes "I think you should keep the current Kafra sir, she's trying her best sir."
            close()
            return
          end
          cutin("kafra_01", 2)
          mes "[ Kafra Service ]"
          mes "What have I done wrong? if so, would you please forgive me?"
          next()
          if menu("Dismissal", "Cancel") ~= 1 then
            mes "[ Kafra Service ]"
            mes "Thank you master, I'll do my best! ^^."
            cutin("kafra_01", 255)
            close()
            return
          end
          mes "[ Kafra Service ]"
          mes "OMG!! how could you do this to meeeeh!! T_T~"
          next()
          disable_npc("castle_kafra_" .. map)
          g[key(c, "kafra")] = 0
          cutin("kafra_01", 255)
          mes(S)
          mes "...."
          mes "Kafra Service has been dismissed"
          close()
          return
        end
        mes(S)
        mes "Would you like to employ a new Kafra Service?"
        mes("^ff0000You need " .. KAFRA_FEE .. " Zeny to employ...")
        next()
        if menu("Employ Kafra.", "Cancel") ~= 1 then
          mes(S)
          mes "Yes Master! But you must get Kafra as soon as possible."
          close()
          return
        end
        if player.zeny < KAFRA_FEE then
          mes(S)
          mes "Master, you do not have enough money to employ Kafra, employment has been cancelled."
          close()
          return
        end
        player.zeny = player.zeny - KAFRA_FEE
        g[key(c, "kafra")] = 1
        enable_npc("castle_kafra_" .. map)
        mes(S)
        mes "You have been contracted with Kafra Service Company."
        next()
        cutin("kafra_01", 2)
        mes "[ Kafra Service ]"
        mes "Hello, I came here to follow your order sir!^^."
        mes "I'll do the best I can."
        close()
        cutin("kafra_01", 255)
      elseif pick == 6 then
        mes(S)
        mes "Would you like to get into our treasure room?"
        next()
        mes(S)
        mes "That place is only for guild master, which means you are the only one who has access to enter."
        next()
        if menu("Enter Master's room.", "Cancel") == 1 then
          mes(S)
          mes "Please follow me through the secret way."
          mes "You must push down the secret switch in order to get out."
          next()
          warp(map, c[7][1], c[7][2])
          return
        end
        mes(S)
        mes "Goods are producing everyday, you must get'em whenever you can."
        mes "Please don't forget to get all the goods in time."
        close()
      elseif pick == 7 then
        mes(S)
        mes "Master!!"
        mes "Are you goin to give our guild up?!"
        next()
        mes(S)
        mes "It is not worth to give up on our guild master!!!"
        mes "Please reconsider master!!"
        next()
        if menu("Empty Guild Base", "Cancel") == 1 then
          mes(S)
          mes "Master, please reconsider...T_T!!"
          mes "OMG, please!!!!!!!!"
          next()
          if menu("Cancel", "Empty Guild Base") == 2 then
            mes(S)
            mes "Master!!....."
            close()
            clear(c)
            tmp.regen_rate = nil
            announce("Stronghold [" .. c[2] .. "], [" .. player.name .. "] has surrendered the castle", 0)
            -- an empty castle fills with monsters (castles.lua), and its Emperium stands again
            if content("castle_monsters") then do_event("castle_garrison::OnCastleEmptied" .. i) end
            emperium(c)
            return
          end
        end
        mes(S)
        mes "I knew it master^^!!"
        mes "Please, don't freak me out like that again^^."
        close()
      end
    end,
  }

  -- the Kafra, there once the master has a contract with her
  local k = c[5]
  npc {
    map = map, x = k[1], y = k[2], dir = k[3], sprite = 117, name = "Kafra Service", id = "castle_kafra_" .. map,
    events = { "OnInit" },
    on_click = function(at)
      if at == "OnInit" then
        if g[key(c, "kafra")] ~= 1 then disable_npc("castle_kafra_" .. map) end
        return
      end
      cutin("kafra_01", 2)
      if not is_master(c) then
        mes "[Kafra Service]"
        mes("I am here to help only ^ff0000" .. owner_name(c) .. "^000000, please use a different Kafra Service. thank you.")
        cutin("kafra_01", 255)
        close()
        return
      end
      mes "[Kafra Service]"
      mes("Welcome, ^ff0000" .. player.name .. "^000000.")
      mes "Kafra Service is always with you."
      next()
      local pick = menu("Open Storage", "Teleport Service", "Use Cart Service", "Cancel")
      if pick == 1 then
        if skill_level(1) < 6 then
          mes "[Kafra Service]"
          mes "You need Basic Skill level 6 to use the storage."
        else
          cutin("kafra_01", 255)
          open_storage()
          return
        end
      elseif pick == 2 then
        local t = c[3]
        mes "[Kafra Service]"
        mes "Please choose a city."
        next()
        if menu(t[1] .. " -> " .. TELE_FEE .. " z", "Cancel") == 1 then
          if player.zeny < TELE_FEE then
            mes "[Kafra Service]"
            mes "Sorry, you don't have enough Zeny to use the teleport service."
          else
            player.zeny = player.zeny - TELE_FEE
            v.KF_PIT = v.KF_PIT + TELE_FEE // 10 -- 817: KF_PIT+20
            cutin("kafra_01", 255)
            warp(t[2], t[3], t[4])
            return
          end
        end
      elseif pick == 3 then
        local cl = player.class
        if cl ~= Job_Merchant and cl ~= Job_Blacksmith and cl ~= Job_Alchem then
          mes "[Kafra Service]"
          mes "Sorry, Cart Renting Service is only for Merchant/Blacksmith/Alchemist."
        else
          mes "[Kafra Service]"
          mes("Cart Rental is " .. CART_FEE .. " Zeny and the [Pushcart] skill is necessary.")
          mes "Would you like to rent one?"
          next()
          if menu("Rent Cart", "Cancel") == 1 then
            if player.zeny < CART_FEE then
              mes "[Kafra Service]"
              mes("Sorry, you must have at least " .. CART_FEE .. " zeny.")
            elseif check_option(0x0008) == 1 then
              mes "[Kafra Service]"
              mes "You already have a cart."
            elseif not set_cart() then
              mes "[Kafra Service]"
              mes "Sorry, you need the Pushcart skill to use a cart."
            else
              player.zeny = player.zeny - CART_FEE
            end
          end
        end
      else
        mes "[Kafra Service]"
        mes "Kafra Service always trying to offer best service for you."
        mes "Thank you for using our service."
      end
      cutin("kafra_01", 255)
      close()
    end,
  }

  local lo = c[8] -- the lever out of the master room: cell -> arrival beside the steward

  -- the flag on the castle's grounds: who holds it; the master may go home from it
  local f, land = FLAGS[map], KINGDOM[map:match("^(%a+)_")]
  local H = "[ Kingdom of " .. land .. " ]"
  npc {
    map = f[4], x = f[1], y = f[2], dir = f[3], sprite = 722, name = "Flag", id = "castle_flag_" .. map,
    on_click = function()
      if g[key(c)] == 0 then
        mes(H)
        mes("1. As Law of " .. land .. ", this guild base isn't taken by any guild yet.")
        next()
        mes(H)
        mes "2. In order to take this guild base, you must defeat all the guardians that protecting guild base and eliminate the emperium to take over the guild base."
        close()
      elseif not is_master(c) then
        -- 0.5.2 names the guild, then its master: here the castle is the character's
        mes(H)
        mes("1. As Law of " .. land .. ", this guild base is ^ff0000" .. owner_name(c) .. "^000000's guild base.")
        next()
        mes(H)
        mes("2. Its master is ^ff0000" .. owner_name(c) .. "^000000, and if any of you have a beef, raise your sword to honor yourself.")
        close()
      else
        mes(H)
        mes "Do you want to return to your castle?"
        next()
        if menu("Yes", "No") == 1 then
          -- 0.5.2 sends each to a cell copied between regions; ours: beside the steward
          warp(map, lo[3], lo[4])
          return
        end
        close()
      end
    end,
  }

  -- the lever out of the master room
  npc {
    map = map, x = lo[1], y = lo[2], dir = 0, sprite = 111, name = "Switch", id = "castle_lever_out_" .. map,
    on_click = function()
      mes "There is a little switch here."
      mes "Would you like to pull the switch down?"
      next()
      if menu("Yes", "No") == 1 then
        warp(map, lo[3], lo[4])
        return
      end
      close()
    end,
  }

  -- the lever into the guild dungeon
  local ld = c[9]
  npc {
    map = map, x = ld[1], y = ld[2], dir = 0, sprite = 111, name = "Switch", id = "castle_lever_dun_" .. map,
    on_click = function()
      mes "[ Vibration of Man's Voice ]"
      mes " ' Only brave man get to lead the guild base.. '"
      next()
      mes "There is a little switch here."
      mes "Would you like to pull the switch down?"
      next()
      if menu("Yes", "No") ~= 1 then close() return end
      if not is_master(c) then
        mes "[ Vibration of Man's Voice ]"
        mes " ' LOOOOOOOOSER...... ' "
        close()
        return
      end
      warp(ld[3], ld[4], ld[5])
    end,
  }
end

-- The master's treasure is laid out the first time he comes in on a new day.
on("map_enter", function()
  local c = by_map(player.map)
  local home = c and is_master(c)
  if home then new_day(c) end
  tmp.regen_rate = home and HOME_REGEN or nil
end)

-- Refining: success chance per weapon level (0 = armor) and current refine.
-- rules.refine_chance(weapon_level, refine) -> percent for refine -> refine+1.
--
-- Source: athena-dev-2.1.1-mod0631 db/refine_db.txt (archived 2003-12-18), the
-- same table as 641mod, 817 and rAthena today (research/notes/mechanics-sources.md
-- section 6). Safe limits: armor +4, weapon Lv1 +7, Lv2 +6, Lv3 +5, Lv4 +4.
-- Tier athena2003. The iRO "Refine System" page (2004-06) agrees on the safe limits.

local chance = {
  [0] = { 100, 100, 100, 100, 60, 40, 40, 20, 20, 10 }, -- armor
  [1] = { 100, 100, 100, 100, 100, 100, 100, 60, 40, 20 },
  [2] = { 100, 100, 100, 100, 100, 100, 60, 40, 20, 20 },
  [3] = { 100, 100, 100, 100, 100, 60, 50, 20, 20, 20 },
  [4] = { 100, 100, 100, 100, 60, 40, 40, 20, 20, 10 },
}

function rules.refine_chance(weapon_level, refine)
  local t = chance[weapon_level] or chance[0]
  return t[refine + 1] or 0
end

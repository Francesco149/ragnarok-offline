-- Status changes: what sc_start(type, val1, val2) does, per Athena SC type.
-- rules.status_change{type, val1, val2} -> { ms = duration, val1 =, val2 =,
--   ends = {types it replaces} } or nil (not reconstructed yet: the server
--   reports it once). Their effect on stats is in rules.status (status.lua).
--
-- Source (tier athena2003): athena-dev-2.1.1-mod0631 map/skill.c
-- skill_status_change_start (archived 2003-12-18). Types: db/const.txt SC_*.

local SPEEDPOT = { [37] = true, [38] = true, [39] = true } -- SC_SpeedPot0..2

function rules.status_change(sc)
  if SPEEDPOT[sc.type] then
    -- skill.c: tick = 1000 * val1; val2 = 100 - 5 * (2 + type - SC_SPEEDPOTION0):
    -- Concentration 90, Awakening 85, Berserk 80 (% of attack delay)
    return { ms = 1000 * sc.val1, val1 = sc.val1, val2 = 100 - 5 * (2 + sc.type - 37), ends = { 37, 38, 39 } }
  end
  return nil
end

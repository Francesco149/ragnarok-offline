-- The Peco Peco keepers of Prontera: a Knight (or Crusader) hires a mount.
--
-- Source: mod0631 conf/npc_event_rental.txt (lines 5 and 65), archived
-- 2003-12-18; it is in the list mod0631 loaded (map_athena.conf:58). Tier
-- athena2003, jRO lineage. 817 has only an emulator convenience (one
-- "Universal Rental" NPC for cart, falcon and Peco), which is not imported,
-- so a Knight had no way to a Peco Peco here. The Hunter's falcon comes from
-- 817's Falcon Breeder in the Hunter guild, at the same 2500z as mod0631's.
--
-- Places, sprite, the 2500z and the steps are mod0631's, including the trap
-- its keeper warns of: without the Riding skill the bird refuses its rider
-- and the money is gone all the same. The English is our translation of
-- mod0631's Japanese, not Gravity's text; the name "Peco Peco Breeder" is
-- ours, after 817's "Falcon Breeder" (mod0631: "Peco Peco keeper soldier").
-- The official English name is not known (research/THREADS.md #2).

local KEEPERS = {
  { x = 55, y = 350, dir = 5, line = 5, job = Job_Knight, sir = "Sir Knight", only = "Knights" },
  { x = 232, y = 318, dir = 3, line = 65, job = Job_Crusader, sir = "Crusader", only = "Crusaders", gate = "jobs2_2" },
}

for _, k in ipairs(KEEPERS) do
  if not k.gate or content(k.gate) then
    npc {
      map = "prontera", x = k.x, y = k.y, dir = k.dir, sprite = 105, name = "Peco Peco Breeder",
      src = "m631:conf/npc_event_rental.txt:" .. k.line,
      on_click = function()
        mes "[Peco Peco Breeder]"
        if player.class ~= k.job then
          mes "What can I do for you?"
          mes("Peco Pecos are for " .. k.only .. " only.")
          return close()
        end
        mes("Welcome, " .. k.sir .. ".")
        mes "Shall I get a Peco Peco ready?"
        mes "It is 2500 Zeny a bird."
        next()
        if menu("Hire one", "Never mind") ~= 1 then
          mes "[Peco Peco Breeder]"
          mes "I see. Well then..."
          return close()
        end
        mes "[Peco Peco Breeder]"
        if player.zeny < 2500 then
          mes(k.sir .. ", you don't have enough money...")
          mes "...are you broke?"
          return close()
        end
        mes "One moment! To ride one you need the 'Riding' skill."
        mes "You know that, don't you?"
        mes "Without it the bird won't let you on, and your money is gone all the same."
        mes "Well, are you ready?"
        next()
        if menu("Yes", "No") ~= 1 then return close() end
        player.zeny = player.zeny - 2500
        set_riding() -- does nothing without the skill, as in mod0631 (pc.c:3247)
        close()
      end,
    }
  end
end

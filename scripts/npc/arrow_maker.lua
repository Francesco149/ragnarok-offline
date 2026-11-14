-- The Arrow Maker of Morroc (Arrow Crafting): 817's words (skillquests/arrowcraft.txt:5),
-- mended so the skill is earned.
--
-- 817's script taught Arrow Crafting to every archer who talked to it: its
-- three answers ran on into one another with no close, so "Ok, tell me" and
-- "No, nevermind" both ended in "Congratulations" and the skill. Its list
-- spoke of 35 Powder of Butterfly, its test counted them, and its code took
-- 20 Resin; the 41 Pointed Scales were never taken (`delitem 906, to 41`).
-- Three period witnesses agree on one list, which is what the code took:
-- 20 Resin, 7 Poison Spores, 41 Pointed Scales, 13 Trunks and a Red Potion
-- (mod0631 conf/npc_event_skillget.txt, jRO, 2003-12; Godhand's Arrow
-- Crafting FAQ, GameFAQs 26381, idRO, 2003-10-23; RODataZone's Archer page,
-- iRO, capture of 2004-04-23, research/agy-threads/r8 T9-018). mod0631 and
-- RODataZone teach it from job level 30, 817 from 35: we take 30.
--
-- research/THREADS.md #7, docs/DEVIATIONS.md.

local ITEMS = { { 907, 20 }, { 7033, 7 }, { 906, 41 }, { 1019, 13 }, { 501, 1 } }

-- written out: tests/job_quests.rs reads the amounts from `count_item(id) >= n`
local function has_all()
  return count_item(907) >= 20 and count_item(7033) >= 7 and count_item(906) >= 41
    and count_item(1019) >= 13 and count_item(501) >= 1
end

npc {
  map = "morocc", x = 121, y = 109, dir = 5, sprite = 88, name = "Arrow Maker",
  replaces = "817:skillquests/arrowcraft.txt:5",
  on_click = function()
    local archer = player.class == Job_Archer or player.class == Job_Hunter
      or player.class == Job_Bard or player.class == Job_Dancer
    if player.class == Job_Archer and player.job_level < 30 then
      mes "[Arrow Maker]"
      mes "Hello friendly archer. You have to raise your level a little more before crafting arrows."
      mes "YOu need to be atleast joblevel 30."
      return close()
    end
    if not archer then
      mes "[Arrow Maker]"
      mes "I am an Arrow Maker and I can teach archers how to make arrows."
      mes "This skill can only be used by a archer."
      mes "Unfortunately you ain't a archer."
      return close()
    end
    if skill_level(147) > 0 then
      mes "[Arrow Maker]"
      mes "YOu already have this skill implemented in your skilltree."
      return close()
    end
    if has_all() then
      mes "[Arrow Maker]"
      mes "Congratulations"
      next()
      for _, it in ipairs(ITEMS) do take_item(it[1], it[2]) end
      give_skill(147, 1, 0)
      mes "[Arrow Maker]"
      mes "Hope to see you some other time."
      mes "For now good luck with your new skill."
      return close()
    end
    mes "Welcome to me and my tree. I am an Arrow Maker and I can teach archers how to make arrows."
    mes "If you want to learn this skill you will have to help me out with a task first."
    if menu("Ok, tell me", "No, nevermind") == 1 then
      mes "[Arrow Maker]"
      mes "You see since I can't leave my tree I want you to get me som ingredients for the skill."
      mes "Get me 20 ^FF0000Resin^000000, 7 ^FF0000Poison Spore^000000, 41 ^FF0000Pointed Scale^000000, 13 ^FF0000Trunk^000000, and 1 ^FF0000Red Potion^000000"
      return close()
    end
    mes "[Arrow Maker]"
    mes "Well I am sorry to hear that."
    mes "Good luck"
    close()
  end,
}

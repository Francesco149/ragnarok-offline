-- Moving HP Recovery: 817's knight in Izlude, taking what he asks for.
--
-- 817 (skillquests/movinghp.txt:5) asks for 500 Empty Bottles, a Rocker Doll
-- and a Padded Armour, tests for them, and then takes 200 Empty Bottles and a
-- Moth Wings: the two lines mod0631 has (conf/npc_event_skillget.txt:12 and
-- :95, the Japanese servers' list), left in under the international
-- servers' request. A player kept 300 bottles and the doll.
--
-- RODataZone's Swordman page (capture of 2004-05-02) gives the request as
-- "1 Rocker Doll, 500 Empty Bottle" for job level 35, so the request is
-- right and the taking is the slip. He takes the 500 bottles and the doll.
-- The armour: an iRO player's guide of autumn 2003 (vgmaster88, GameFAQs
-- 19031) has "1 Padded Armor (which you get to keep), 500 empty bottles, and
-- a rocker doll", which is 817's test: he looks for one, worn or carried,
-- and leaves it with you (research/THREADS.md #7, round 7). The wording,
-- the cell and the job test are 817's.

local EMPTY_BOTTLE, ROCKER_DOLL, PADDED_ARMOR, SKILL = 713, 752, 2312, 144

local function say(...)
  mes "[Thomas De Knight]"
  for _, line in ipairs({ ... }) do mes(line) end
end

npc {
  map = "izlude_in", x = 175, y = 130, dir = 2, sprite = 98, name = "Thomas De Knight",
  replaces = "817:skillquests/movinghp.txt:5",
  on_click = function()
    local c = player.class
    if c == Job_Swordman and player.job_level < 35 then
      say("Hello young Swordman. You have to raise your level a little more before learning the HP recovery skill.",
        "YOu need to be atleast joblevel 35.")
      return close()
    end
    if not (c == Job_Swordman or c == Job_Knight or c == Job_Knight2 or c == Job_Crusader or c == Job_Crusader2) then
      say("I am Thomas De Knight and I am a teacher in the HP recovery skill.",
        "This skill can only be used by a Swordman.", "Unfortunately you ain't a swordman.")
      return close()
    end
    if skill_level(SKILL) > 0 then
      say("YOu already have this skill implemented in your skilltree.")
      return close()
    end
    if count_item(EMPTY_BOTTLE) < 500 or count_item(ROCKER_DOLL) < 1 or count_item(PADDED_ARMOR) < 1 then
      say("Hello, I am Thomas De Knight.",
        "I would like you to do me a favour. In return I will give you the HP movement recovery skill. ",
        "Get me 500 ^FF0000Empty Bottles^000000, 1 ^FF0000Rocker Doll^000000, and 1 ^FF0000Padded Armour^000000.",
        "Come back when you have these things and your reward will be waiting for you. ")
      return close()
    end
    say("So you have succeded with your task.", "I promised you that I would give you the HP movement recovery skill.",
      "Do you want me to implement the skill right away?")
    if menu("Yes, please", "No, thanks") ~= 1 then
      say("Come back when you feel ready.", "I will be waiting for your arrival.", "Good luck.")
      return close()
    end
    say("Congratulations")
    next()
    say("I will now give you the ability to move around and gain HP.", "This is an old skill and only taught to strong men.")
    next()
    take_item(EMPTY_BOTTLE, 500)
    take_item(ROCKER_DOLL, 1)
    give_skill(SKILL, 1, 0)
    say("Now it is implemented in to your skilltree.", "The skill will be used automaticaly.", "Good luck")
    close()
  end,
}

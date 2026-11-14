-- Two quest-skill teachers where 817's script leaves out something the
-- period accounts agree on (research/THREADS.md #7, research round 11).
--
-- Charge Arrow (Deothen, payon 30,94; 817 skillquests/arrowrepel.txt:5).
-- 817 and mod0631 ask for no Crossbow. Three player accounts do, and two
-- of them say he gives it back:
--   * Godhand's Arrow Crafting FAQ (GameFAQs 26381, v1.3 of 2003-10-23,
--     idRO): "Crossbow | 1 | Archer Skeleton (Or buy it for 17000z)" and
--     "the Crossbow will be given back to you. So if your current weapon is
--     a Crossbow, then you don't need to buy another one";
--   * Battousai X's quest-skill FAQ (GameFAQs 27646, v1.3 of 2004-01-22,
--     pRO): "Items: 1 Crossbow ... He won't take your crossbow";
--   * RODataZone's Archer page (2004-04, round 8).
-- So he asks to see one (in the bag or in hand) and keeps only the rest.
-- Tier period (player accounts, idRO and pRO), not euRO.
--
-- Cart Revolution (Gershaun, alberta 232,106; replaces 817
-- skillquests/cartrevolution.txt:5). mod0631 (conf/npc_event_skillget.txt:
-- 1045-1110, archived 2003-12-18) teaches at once anyone who brings the
-- items; anyone else must hand him a Banana Juice before he says what he
-- needs, every time. Battousai X's FAQ (above, pRO): "You need a Banana
-- Juice to get him to tell you about the ingredients but if you already
-- have all the items, you won't need the juice." RODataZone (2003-10,
-- round 8) has the juice too; 817 does not.
-- His items: mod0631 picks one of three sets at random each time
-- (Iron/Mucus/Fly Wing/Tentacle 23/32/23/6, 20/30/20/5, 15/25/15/5, two
-- Grape Juices in each). The English accounts give one fixed list:
--   * RODataZone's Merchant page (iRO, capture of 2003-10-03): "20 Iron,
--     30 Sticky Mucus, 20 Fly Wing, 5 Tentacles? and 2 Grape Juice";
--   * 817: the same with 15 Iron (asked, tested and taken);
--   * Battousai X (pRO, 2004-01-22): the same with 30 Iron.
-- RODataZone's is mod0631's middle set item for item, and the other two
-- differ from it in the Iron alone, so he asks for that set: 20 Iron.
-- Job level 35 for a Merchant, as 817, RODataZone and Battousai X (mod0631
-- 30). 817's words, cut into pages that fit; its low-level line names
-- Change Cart, mended. THREADS #7.
--
-- The English of the added lines is ours. docs/DEVIATIONS.md.

-- count_item takes bare ids (Crossbow 1713, Banana Juice 532): tests/job_quests.rs reads them

local function archer_line(class)
  return class == Job_Archer or class == Job_Hunter or class == Job_Bard or class == Job_Dancer
end

wrap_npc("817:skillquests/arrowrepel.txt:5", function(orig)
  return function(at)
    local eligible = archer_line(player.class)
      and not (player.class == Job_Archer and player.job_level < 35)
      and skill_level(148) == 0
    if eligible and count_item(1713) == 0 then
      mes "[Deothen]"
      mes "Hi, I'm the master"
      mes "of the skill ^0033FFArrow Repel^000000."
      mes "I'll teach it to you if you"
      mes "help with my Grandma's soup"
      mes "and show me a crossbow."
      next()
      mes "[Deothen]"
      -- the client swallows a space right after a colour code
      mes "- 1 ^FF0000Crossbow^000000, given back"
      mes "- 2 ^FF0000Emeralds^000000"
      mes "- 3 ^FF0000Yoyo tails^000000"
      mes "- 10 ^FF0000Tentacles^000000"
      mes "- 10 ^FF0000Bill of birds^000000"
      mes "- 36 ^FF0000Banana Juices^000000"
      close()
      return
    end
    return orig(at)
  end
end)

local GRAPE_JUICE, IRON, STICKY_MUCUS, FLY_WING, TENTACLE, BANANA_JUICE = 533, 998, 938, 601, 962, 532
local CART_REVOLUTION = 153

local function gershaun(...)
  mes "[Gershaun]"
  for _, line in ipairs({ ... }) do mes(line) end
end

-- bare ids in count_item: the test reads the amounts from these lines
local function has_cart_items()
  return count_item(533) >= 2 and count_item(998) >= 20 and count_item(938) >= 30
    and count_item(601) >= 20 and count_item(962) >= 5
end

npc {
  map = "alberta", x = 232, y = 106, dir = 6, sprite = 57, name = "Gershaun",
  replaces = "817:skillquests/cartrevolution.txt:5",
  on_click = function()
    local c = player.class
    if c == Job_Merchant and player.job_level < 35 then
      gershaun("Hello friendly merchant.",
        "You have to raise your level a little more before using Cart Revolution.",
        "YOu need to be atleast joblevel 35.")
      return close()
    end
    if not (c == Job_Merchant or c == Job_Blacksmith or c == Job_Alchem) then
      gershaun("I am an merchant and I can teach other merchants how to make carts better.",
        "This skill can only be used by merchants.", "Unfortunately you ain't a merchant.")
      return close()
    end
    if skill_level(CART_REVOLUTION) > 0 then
      gershaun("YOu already have this skill implemented in your skilltree.")
      return close()
    end
    if has_cart_items() then
      gershaun("Well, well you have succeded with your task.",
        "What should I say, congratulations I guess.", "I am a man that keep a promise.")
      next()
      gershaun("Are you sure you want this skill?")
      if menu("yes, hand it over", "No, thanks") ~= 1 then
        gershaun("Well that was a pitty. Sad man.", "Good luck and don't BOTTER me again.")
        return close()
      end
      gershaun("Well then, I will now implement this skill.", "Please stand by.")
      next()
      take_item(GRAPE_JUICE, 2)
      take_item(IRON, 20)
      take_item(STICKY_MUCUS, 30)
      take_item(FLY_WING, 20)
      take_item(TENTACLE, 5)
      give_skill(CART_REVOLUTION, 1, 0)
      gershaun("Congratulations now you have it.", "Good luck.")
      return close()
    end
    if count_item(532) == 0 then
      gershaun("Phew, this sea air makes a man thirsty.",
        "Bring me a ^FF0000Banana Juice^000000, my favourite, and then we can talk business.")
      return close()
    end
    gershaun("Oh! A Banana Juice!", "My favourite! Thank you...", "Ahh, delicious.")
    take_item(BANANA_JUICE, 1)
    next()
    gershaun("Welcome to my simple place here in Alberta.", "I am Gershaun a merchant trainer.")
    next()
    gershaun("I can learn you the skill called: ^0033FFCart Revolution^000000.",
      "Would you like to know how to get this skill?")
    if menu("Yes, please", "No, thanks") ~= 1 then
      gershaun("Bad for you then. Don't botter me anymore.", "Good bye")
      return close()
    end
    gershaun("As you may understand it is not for free.",
      "That's why you need to give me some stuff in return.")
    next()
    gershaun("For this skillquest you will need:",
      "30 ^FF0000Sticky Mucus^000000", "20 ^FF0000Fly wings^000000", "20 ^FF0000Iron^000000",
      "5 ^FF0000Tentacles^000000", "2 ^FF0000Grape Juice^000000")
    next()
    gershaun("Come back later when you have all these stuff.", "Good luck with your task.")
    close()
  end,
}

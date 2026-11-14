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
-- Cart Revolution (Gershaun, alberta 232,106; 817
-- skillquests/cartrevolution.txt:5). mod0631 (conf/npc_event_skillget.txt:
-- 1045-1110, archived 2003-12-18) teaches at once anyone who brings the
-- items; anyone else must hand him a Banana Juice before he says what he
-- needs, every time. Battousai X's FAQ (above, pRO): "You need a Banana
-- Juice to get him to tell you about the ingredients but if you already
-- have all the items, you won't need the juice." RODataZone (2003-10,
-- round 8) has the juice too; 817 does not. The items stay 817's (15
-- Iron): the sources give 15, 20, 30 or one of three sets, and that is
-- still open (THREADS #7).
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

local function has_cart_items()
  return count_item(533) >= 2 and count_item(998) >= 15 and count_item(938) >= 30
    and count_item(601) >= 20 and count_item(962) >= 5
end

wrap_npc("817:skillquests/cartrevolution.txt:5", function(orig)
  return function(at)
    local cls = player.class
    local eligible = (cls == Job_Merchant or cls == Job_Blacksmith or cls == Job_Alchem)
      and not (cls == Job_Merchant and player.job_level < 35)
      and skill_level(153) == 0
    if eligible and not has_cart_items() then
      if count_item(532) == 0 then
        mes "[Gershaun]"
        mes "Phew, this sea air makes"
        mes "a man thirsty. Bring me a"
        mes "^FF0000Banana Juice^000000, my favourite,"
        mes "and then we can talk business."
        close()
        return
      end
      mes "[Gershaun]"
      mes "Oh! A Banana Juice!"
      mes "My favourite! Thank you..."
      mes "Ahh, delicious."
      take_item(532, 1)
      next()
    end
    return orig(at)
  end
end)

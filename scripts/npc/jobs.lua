-- First job changes: the requirement 817's guild NPCs left out.
--
-- The client's own Basic Skill description says "[Level 9] : Enable to
-- Transform into the 1st profession" (tier client; research/THREADS.md R1).
-- 817's job NPCs only checked the job level, so a Novice could change job
-- with Basic Skill unlearnt and carry the 9 points into the new job's skills.
-- These wrappers stop a Novice without Basic Skill 9 before the imported
-- dialogue runs; everything else is 817's (docs/DEVIATIONS.md).

local function basic_skill_9(src, who)
  wrap_npc(src, function(orig)
    return function(at)
      if not at and player.class == Job_Novice and player.job_level >= 9 and skill_level(1) < 9 then
        mes("[" .. who .. "]")
        mes("You haven't mastered the Basic Skill yet.")
        mes("Come back once you have learnt all nine levels of it.")
        close()
        return
      end
      return orig(at)
    end
  end)
end

basic_skill_9("817:jobs/swordsman.txt:118", "Sword Master")
basic_skill_9("817:jobs/magican.txt:3", "Mage Guildsman")
basic_skill_9("817:jobs/archer.txt:4", "Archer Guildsman")
basic_skill_9("817:jobs/acolyte.txt:4", "Cleric")
basic_skill_9("817:jobs/merchant.txt:3", "Merchant")
basic_skill_9("817:jobs/thief.txt:390", "Comrade")

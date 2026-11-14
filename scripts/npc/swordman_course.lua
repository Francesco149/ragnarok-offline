-- The Swordman course: four pit cells of the second stretch that 817 lacks, and
-- the Test Hall Staff's word to a non-Novice (at the end).
--
-- The pits of the second stretch lie in two strips (rows 212/210 and
-- 203/201) on either side of the path, each strip made of three runs: x 16
-- to 97, 99 to 127 and 130 to 183. 817's third run starts at 134 (its
-- pits 2_blank_7_a, 8_a, 8_e, 9_a at 155, 21 cells to each side): the four
-- two-cell pieces at x 132 that close it are missing, so on x 130 to 133
-- one could walk on the strips, outside the path, and not fall. jAthena
-- //0854 of 2004-03-02 (conf/warp/npc_warp_job.txt:131, :134, :139, :142,
-- its Swordman test of the same lineage, tier athena2003) has them:
-- 2_blank_7_0, 8_0, 8_1 and 9_0. The caves they drop into are those of
-- 817's other pits of this stretch (npc/jobs/swordsman.txt:1151-1159).
-- Three typing slips in 817's pits of the same stretch are source fixes in
-- the importer. research/course_check.py reads this file too.
-- docs/DEVIATIONS.md.

local function fall()
  local n = rand(5)
  if n == 0 then warp("job_sword1", 162, 120)
  elseif n == 1 then warp("job_sword1", 94, 120)
  elseif n == 2 then warp("job_sword1", 94, 85)
  elseif n == 3 then warp("job_sword1", 162, 85)
  else warp("job_sword1", 130, 47)
  end
end

npc {
  map = "job_sword1", x = 132, y = 212, dir = 0, sprite = 139, name = "2_blank_7_0",
  touch = { 2, 0 }, src = "jathena-0854:conf/warp/npc_warp_job.txt:131", on_click = fall,
}
npc {
  map = "job_sword1", x = 132, y = 210, dir = 0, sprite = 139, name = "2_blank_8_0",
  touch = { 2, 0 }, src = "jathena-0854:conf/warp/npc_warp_job.txt:134", on_click = fall,
}
npc {
  map = "job_sword1", x = 132, y = 203, dir = 0, sprite = 139, name = "2_blank_8_1",
  touch = { 2, 0 }, src = "jathena-0854:conf/warp/npc_warp_job.txt:139", on_click = fall,
}
npc {
  map = "job_sword1", x = 132, y = 201, dir = 0, sprite = 139, name = "2_blank_9_0",
  touch = { 2, 0 }, src = "jathena-0854:conf/warp/npc_warp_job.txt:142", on_click = fall,
}

-- The Test Hall Staff: 817 ends without a word for anyone who is not a Novice
-- (`goto L1`, a label the file lacks), leaving them in the hall. jAthena //0854
-- (conf/npc/npc_job_swordman.txt) has no such test: whoever is not on the test
-- asks how they got in and is put outside. So do we, with 817's words for it.
wrap_npc("817:jobs/swordsman.txt:14", function(orig)
  return function(at)
    if player.class ~= 0 then
      mes "[Test Hall Staff]"
      if rand(2) ~= 0 then
        mes "Who are you?! This place for the Swordman Test, You are not belong to here! Leave now!"
      else
        mes "Dear me! How did you come inside, others are not supposed to be here. Please leave now!"
      end
      next()
      warp("izlude_in", 66, 170)
      close()
      return
    end
    return orig(at)
  end
end)

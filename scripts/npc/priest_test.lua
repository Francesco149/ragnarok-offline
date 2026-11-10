-- The Priest test's three rounds on the one map this client has.
--
-- 817 (jobs/priest.txt:440-530) runs the test over three maps, priest_1-1,
-- priest_2-1 and priest_3-1: take a grail from the Mother Nun, carry it past
-- the Wraiths to the Nun it belongs to, and she sends you on to the next
-- map; the third one passes you. The client's resnametable gives all three
-- names the same map, job_priest, and the client has no other: here the
-- first Nun's warp failed and the test could not be finished. The three
-- rounds are played on job_priest one after the other. NPCs, cells, lines
-- and the Wraiths are 817's; counting the rounds is ours (docs/DEVIATIONS.md).

local ROUNDS = 3

local function nun(src, grail)
  wrap_npc(src, function()
    return function()
      if tmp.grail ~= grail then
        announce("Incorrect Grail!!!", 3)
        return close()
      end
      tmp.grail = 0
      tmp.priest_round = (tmp.priest_round or 0) + 1
      if tmp.priest_round < ROUNDS then
        announce("Correct, You Advance!!!", 3)
        warp("job_priest", 100, 17)
      else
        tmp.priest_round = 0
        v.job_priest_q = 3
        announce("Correct, " .. char_info(0) .. " Has Passed The Quest Exam!!!", 3)
        warp("prontera", 274, 354)
      end
      close()
    end
  end)
end

nun("817:jobs/priest.txt:461", 1)
nun("817:jobs/priest.txt:495", 2)

-- going in starts from the first round
wrap_npc("817:jobs/priest.txt:407", function(orig)
  return function(at)
    tmp.priest_round = 0
    return orig(at)
  end
end)

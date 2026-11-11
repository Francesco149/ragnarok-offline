-- The Merchant test's two Kafras: the parcel for Prontera and the one for
-- Byalan Island.
--
-- 817's ports of them are broken (npc/jobs/merchant.txt:846 and
-- npc/other/kafra.txt:91; mod0631 has the same two in
-- conf/npc_town_kafra.txt:9 and :1735, archived 2003-12-18, jRO lineage):
--  * Byalan's Kafra asks `frst_job_quest` whether you bring a parcel, a
--    variable nothing sets (Kay, who hands the parcels out, sets
--    job_merchant_q3), so she never takes one. One application in four is
--    sent to her, and such an applicant could only report the parcel lost,
--    fail, and pay the fee again. Past that test she reads the training
--    ground's counter for the serial number, and where mod0631 moves the
--    quest on (`set JOB_MERCHANT_Q,8`) she hands out items 3005 and 3017,
--    which do not exist.
--  * Both test the box and then jump to the very next line whatever the
--    answer, so any box passed for the right one, the wrong one too.
-- Here the two take a parcel as mod0631 does: the box with the first serial
-- number of the pair for the first errand, the second for the second,
-- nothing else. Every line is 817's (Byalan's Kafra reads the first
-- parcel's number out as "3012685" in mod0631 too; it is 3318702). A player
-- who is not on the errand gets the Kafra's ordinary service, mended too
-- (below). docs/DEVIATIONS.md.

local BOX_1, BOX_2, BOX_WRONG, LETTER = 1081, 1082, 1083, 1072

-- place: Kay's job_merchant_q3; q2: the two errands of that place.
local function delivery(k)
  return function(orig)
    return function(at)
      local q = v.job_merchant_q
      if at then return orig(at) end
      -- on the way with a parcel: 5 or 6 (half the fee paid, or all of it). Once it is
      -- delivered q3 moves on (as in 817), and she is an ordinary Kafra again.
      if v.job_merchant_q3 ~= k.place or q < 5 or q > 6 then
        if k.service then return k.service() end
        return orig(at)
      end
      cutin(k.cutin, 2)
      local function bye()
        cutin(k.cutin, 255)
        close()
      end
      mes "[Kafra]"
      for _, line in ipairs(k.hello) do mes(line) end
      if count_item(BOX_1) + count_item(BOX_2) + count_item(BOX_WRONG) == 0 then
        mes(k.no_box)
        return bye()
      end
      for _, line in ipairs(k.thanks_first or {}) do mes(line) end
      next()
      mes "[Kafra]"
      mes(k.check)
      local errand = v.job_merchant_q2 - k.q2 + 1 -- 1 or 2
      local box = errand == 1 and BOX_1 or BOX_2
      if (errand ~= 1 and errand ~= 2) or count_item(box) == 0 then
        if k.wrong_first then mes(k.wrong_first) end
        next()
        mes "[Kafra]"
        for _, line in ipairs(k.wrong) do mes(line) end
        return bye()
      end
      mes(k.right[errand])
      take_item(box, 1)
      give_item(k.receipt + errand - 1, 1)
      v.job_merchant_q = q + 2
      v.job_merchant_q3 = k.place + 4
      k.thanks(bye)
    end
  end
end

wrap_npc("817:jobs/merchant.txt:846", delivery {
  place = 1, q2 = 1, receipt = 1073, cutin = "kafra_03",
  hello = { "Delivery from the Merchant Guild? Oh, yes! Alright, Please put down here...." },
  no_box = "Excuse me.. Where's the Delivery....?",
  thanks_first = { "I really appreciate for what you've done. You must be Really tired to come here from the Distance~" },
  check = "Let me check its Serial Numbers. Hmm....",
  wrong_first = ".....Umm? This is...??",
  wrong = { "Oh, Dear. I don't think it's the Right one we ordered. It should have the serial numbers as either of 2485741 or 2328137.",
            "I am so afraid there were something worng during passage." },
  right = { "2485741.. Yes, This is the Right one we ordered~ I guess you need a Receipt. I will give it to you right away.",
            "2328137.. Yes, This is the Right one we ordered~ I guess you need a Receipt. I will give it to you right away." },
  thanks = function(bye)
    next()
    mes "[Kafra]"
    mes "I really appreciate you for what you've done so far."
    bye()
  end,
  -- Her ordinary service: to Izlude for 900z. 817 took the 900z without
  -- looking in the purse, and its "Use Kafra Pass" told everyone a pass was
  -- needed (a test that jumps to the next line; behind it, "Under
  -- development"). mod0631 (conf/npc_town_kafra.txt:25-33) checks the zeny
  -- and adds a tenth of the fare to the special reserve, as every Kafra
  -- does (scripts/npc/kafra.lua, whose wording for an empty purse this is;
  -- like there, no pass entry: kRO removed that menu on 2003-05-20).
  service = function()
    cutin("kafra_03", 2)
    mes "[Kafra]"
    mes "Welcome to Kafra Corp. As you can see, the Swordman Association has moved to the satellite, Izlude."
    mes "We operate the transportation from here, Prontera to Izlude at 900 Zeny."
    next()
    if menu("Use.", "Cancel.") ~= 1 then
      cutin("kafra_03", 255)
      close()
      return
    end
    if player.zeny < 900 then
      mes "[Kafra]"
      mes "Dear, you don't have enough money. Please check again."
      cutin("kafra_03", 255)
      close()
      return
    end
    player.zeny = player.zeny - 900
    v.KF_PIT = v.KF_PIT + 90
    cutin("kafra_03", 255)
    warp("izlude", 91, 105)
  end,
})

wrap_npc("817:other/kafra.txt:91", delivery {
  place = 4, q2 = 7, receipt = 1079, cutin = "kafra_03",
  hello = { "Delivery fro the Merchant Guild?? Ah, Right! Yes! I almost forgot.", "Okay,Please put down here...." },
  no_box = ".... I think our orders are almost one Box of Oversized and Heavy....But you seem to carry it on without difficulty.",
  check = "Let me check its Serial numbers, I have to give the receipt to you anyhow.",
  wrong = { "......? Excuse me! I think you gave me the wrong one.",
            "Our order should have the serial numbers as either of 3318702 or 3543625..." },
  right = { "Um.... 3012685. Yes! This is the right one we ordered. Here, your receipt.",
            "Um.... 3543625. Yes! This is the right one we ordered. Here, your receipt." },
  thanks = function(bye)
    -- Chief Mahnsoo's letter, if it came along (he pays for it afterwards)
    if count_item(LETTER) ~= 0 then
      next()
      menu("This is from Mr. Mahnsoo...")
      take_item(LETTER, 1)
      v.nov_3_merchant = 11
      mes "[Kapra's]"
      mes "Oh,My good Lord! Mr. Mansoo wrote me a Letter? Thank you, Thank you~"
      mes "I though he would send one one of thesedays, but I've never expected it would be so soon like this time~"
    end
    next()
    mes "[Kafra]"
    mes "I really appreciate you for what you've done so far. This is my Small Gift for you to lay my heart."
    mes "I hope you will help me next time."
    give_item(rand(2) == 0 and 513 or 512, 3)
    bye()
  end,
  -- Her ordinary service was 817's own Kafra with its slips (a cart charged
  -- twice and given without the skill, a pass entry that leads nowhere):
  -- the one service of scripts/npc/kafra.lua instead, with mod0631's save
  -- cell (817's too) and storage fee (conf/npc_town_kafra.txt:1746, :1761).
  service = function()
    return KAFRA_SERVICE { cutin = "kafra_03", save = { "izlu2dun", 87, 170 }, storage = 35 }()
  end,
})

-- The end of the training ground: the result, the subsidy, and the six job
-- guide rooms for those who take the job the test suggests.
--
-- 817's last Helper (npc/jobs/novice.txt:1324, new_1-4 60,149) is an English
-- port of the script mod0631 has in Japanese (conf/npc_job_newbie.txt:2091,
-- archived 2003-12-18, tier athena2003, jRO lineage), and the port is broken
-- in three places:
--  * "I will follow the result of the test" answers "Under development.
--    Please choose your job manualy." mod0631 sends you to one of six guide
--    rooms on the same map (npc_job_newbie.txt:2439-2459), where an
--    attendant hands over what the job's guild sent and passes you on to
--    the town (:2461-2650). The rooms are on this client's new_zone04: five
--    in the top row and one below the last, with nothing else in them.
--  * The subsidy table is scrambled: a total of 8 or less gets the most
--    (500z, 10 Red Herbs) and a total over 16 *loses* 500z. mod0631
--    (:2262-2284): up to 8, 100z and 1 Red Herb; up to 15, 150z and 3; 16,
--    300z and 5; more, 500z and 10; a Milk each time.
--  * The first course's points are never counted (817's Leo Von Frisch
--    hands out the prizes for right answers but leaves NOV_1ST_COS alone),
--    so the result always read "1st course: 0 / 6 Points". mod0631 adds a
--    point for each right answer (:531-1115).
-- Here the three follow mod0631. The English of the Helper is 817's, line
-- for line; the attendants of the guide rooms have no English anywhere, so
-- theirs is ours, translated from mod0631 and cut to the client's window
-- (the name and six lines of about 30 characters). docs/DEVIATIONS.md.

local RED_HERB, MILK, RED_POTION, LEATHER_JACKET, CLUB = 507, 519, 501, 2303, 1501

-- ---------------------------------------------------------------- 1st course
-- Leo Von Frisch gives a prize for each right answer: a Banana (513) for
-- the first and the second, three Sweet Potatoes (516), an Apple (512), a
-- piece of Meat (517), a Milk (519). Count them as mod0631 counts points.
local PRIZES = { [513] = 1, [516] = 3, [512] = 1, [517] = 1, [519] = 1 }
local function prizes()
  local n = 0
  for id, per in pairs(PRIZES) do n = n + count_item(id) // per end
  return n
end
local function credit()
  local won = prizes() - tmp.nov_quiz_base
  if won > 0 then v.nov_1st_cos = math.min(6, v.nov_1st_cos + won) end
  tmp.nov_quiz_open = 0
end

wrap_npc("817:jobs/novice.txt:248", function(orig)
  return function(at)
    if tmp.nov_quiz_open ~= 0 then credit() end -- a lesson walked away from
    tmp.nov_quiz_base = prizes()
    tmp.nov_quiz_open = 1
    orig(at)
    credit()
  end
end)

-- ------------------------------------------------------------------ the jobs
-- In the order of mod0631's @JOB_NUMBER. `said` is 817's reading of the
-- test, `town` where 817 and mod0631 send those who choose for themselves.
local JOBS = {
  { var = "nov_3_swordman", room = { "new_zone04", 20, 176 }, at = { 19, 186 }, name = "Swordman",
    save = { "izlude", 94, 104 }, town = { "izlude_in", 121, 165 },
    said = { "'You have simple-minded and always talk straight.And you have an unshaken faith so that you want to rule the world.Also you always stand by the weak.'",
             "'So We suggest you  ^696969Swordman^000000." } },
  { var = "nov_3_archer", room = { "new_zone04", 60, 176 }, at = { 59, 186 }, name = "Archer",
    save = { "payon", 87, 117 }, town = { "payon_in02", 21, 71 },
    said = { "'You are very emotional and sensative person.And you have open-minded.'",
             "'Also, You don't want to be ordinary,always eager for valuable life of your own .'",
             "'So we suggest you ^696969Archer^000000.'" } },
  { var = "nov_3_thief", room = { "new_zone04", 100, 176 }, at = { 99, 186 }, name = "Thief",
    save = { "morocc", 160, 94 }, town = { "morocc", 154, 50 },
    said = { "'You always try to enjoy your life.Life is like adventure for you.'",
             "'So we suggest you ^696969Theif^000000.'" } },
  { var = "nov_3_magician", room = { "new_zone04", 140, 176 }, at = { 139, 186 }, name = "Mage",
    save = { "geffen", 120, 100 }, town = { "geffen", 120, 60 },
    said = { "'You are self-diciplined person.Sometimes you pretend to understand whole things in the world not to be silly to the other person.",
             "'You are man of reason,an introvert,an analyst,a man of keen insight.'",
             "'So we suggest you ^696969Mage^000000." } },
  { var = "nov_3_acolyte", room = { "new_zone04", 180, 176 }, at = { 179, 186 }, name = "Acolyte",
    save = { "prontera", 273, 354 }, town = { "prontera", 233, 313 },
    said = { "'You are warm-hearted,easy-going person,taking care of people.'",
             "'But actually you pretend not to need help from the other,people think you are *an altruist.",
             "'So we suggest you ^696969Acolyte^000000.'",
             "",
             "'*The person who really enjoy helping people with self-sacrifice.'" } },
  { var = "nov_3_merchant", room = { "new_zone04", 180, 134 }, at = { 179, 146 }, name = "Merchant",
    save = { "alberta", 116, 57 }, town = { "alberta", 116, 57 },
    said = { "'You can easily catch what the public wants,and you have a telent to organize system,and you have a strong sense of responsibility.'",
             "'So we suggest you ^696969Merchant^000000.'" } },
}

-- mod0631's loop: the first job with the most points wins.
local function suggested()
  local best = 1
  for i = 2, #JOBS do
    if v[JOBS[best].var] < v[JOBS[i].var] then best = i end
  end
  return best
end

local function forget_the_course()
  for _, j in ipairs(JOBS) do v[j.var] = 0 end
  v.nov_1st_cos = 0
  v.nov_get_item01 = 0
  v.nov_get_item03 = 0
end

npc {
  map = "new_zone04", x = 60, y = 149, dir = 4, sprite = 55, name = "Helper",
  replaces = "817:jobs/novice.txt:1324",
  src = "m631:conf/npc_job_newbie.txt:2091",
  on_click = function()
    mes "[Helper]"
    mes "Congratulations~! You finised whole courses by yourself!"
    mes "You should remember what you've learned from Traning Ground. It would be useful to you."
    next()
    mes "[Helper]"
    mes "Ok, here is the result of your own."
    next()
    local first = v.nov_1st_cos
    local second = v.nov_get_item01 < 10 and 10 - v.nov_get_item01 or 0
    local job = suggested()
    mes "[Helper]"
    mes("1st course: " .. first .. " / 6 Points.")
    if second ~= 0 then
      mes("2nd course: " .. second .. " / 10 Points.")
    else
      mes "2nd course: 0 / 0 Points"
    end
    mes "And the result of personally test is....."
    next()
    mes "[Helper]"
    for _, line in ipairs(JOBS[job].said) do mes(line) end
    next()
    mes "[Helper]"
    mes ".... Report says like this. It is not my intention, I don't mean to nice to you!! To me you are just a novice!!"
    mes "Never take it seriously, it's just a game."
    next()
    mes "[Helper]"
    mes "Here are the subsides from the Training Ground. Even if they are not worthless to veteran players, not to you."
    next()
    if v.nov_get_item03 == 0 then
      v.nov_get_item03 = 1
      local total = first + second
      local herbs, zeny = 10, 500
      if total <= 8 then herbs, zeny = 1, 100
      elseif total <= 15 then herbs, zeny = 3, 150
      elseif total == 16 then herbs, zeny = 5, 300
      end
      give_item(RED_HERB, herbs)
      give_item(MILK, 1)
      player.zeny = player.zeny + zeny
    end
    mes "[Helper]"
    mes "Now I will let you go to the Job Agency. Before going there are a few things you have to know."
    next()
    mes "[Helper]"
    mes "Are you going to choose the job following the test? Or are you going to choose you job by your own will?"
    next()
    mes "[Helper]"
    mes "If you follow the test, you can get a lot of subsidy for headstart from guild and assosiation and get to start in the specific town for your job."
    mes "If you follow your will, you get to start in specific town for your job with 100 zeny before we gave you."
    next()
    mes "[Helper]"
    mes "Choose one. This is your only chance."
    next()
    if menu("I want to be what I wanted!", "I will follow the result of the test") == 2 then
      -- 817 has no line for this branch ("Under development"); mod0631:
      -- 「分かった、今すぐ該当職業案内室に送ってやるぞ。」
      mes "[Helper]"
      mes "Ok. I see. I will send you to the Job Agency of that job right away."
      next()
      forget_the_course()
      warp(table.unpack(JOBS[job].room))
      return
    end
    mes "[Helper]"
    mes "Ok. I see. Choose one what you wanted to be. I will send you to the town right away."
    next()
    local j = JOBS[menu("Swordman.", "Archer.", "Thief.", "Mage.", "Acolyte.", "Merchant")]
    if not j then return end
    mes "[Helper]"
    mes "I expect you the best."
    save_point(table.unpack(j.save))
    next()
    forget_the_course()
    warp(table.unpack(j.town))
  end,
}

-- ------------------------------------------------------------ the guide rooms
-- mod0631 conf/npc_job_newbie.txt:2461-2650. Each attendant saves where he
-- sends you, as there. `gift` runs once (mod0631's NOV_GET_ITEM05).
local ROOMS = {
  { job = 1, save = { "izlude_in", 121, 165 }, to = { "izlude_in", 121, 165 },
    last = { "A word from the Swordman", "Guild, to be read to every", "novice who comes here:", "'Sorry about this, all of you.'",
             "That is all. You get five", "Red Potions. Off you go." },
    gift = function() give_item(RED_POTION, 5) end },
  { job = 2, save = { "payon_in02", 21, 71 }, to = { "payon_in02", 21, 71 },
    last = { "So here is what was sent,", "and then I send you to town:", "three Red Potions and 50", "Zeny to start your new job." },
    gift = function() give_item(RED_POTION, 3); player.zeny = player.zeny + 50 end },
  { job = 3, save = { "morocc", 154, 50 }, to = { "morocc", 154, 50 },
    last = { "So here is what was sent,", "and then I send you to town:", "one Leather Jacket. The rest", "you are to manage by", "yourself, they say." },
    gift = function() give_item(LEATHER_JACKET, 1) end },
  { job = 4, save = { "geffen", 120, 60 }, to = { "geffen", 120, 60 },
    last = { "So here is what was sent,", "and then I send you to town:", "100 Zeny to start your new", "job." },
    gift = function() player.zeny = player.zeny + 100 end },
  { job = 5, save = { "prontera", 273, 354 }, to = { "prontera", 273, 354 },
    last = { "So here is what was sent,", "and then I send you to town:", "three Red Potions and a", "Club." },
    gift = function() give_item(RED_POTION, 3); give_item(CLUB, 1) end },
  { job = 6, save = { "alberta", 116, 57 }, to = { "alberta", 116, 57 },
    last = { "So here is what was sent,", "and then I send you to town:", "100 Zeny to start your new", "job." },
    gift = function() player.zeny = player.zeny + 100 end },
}

for _, r in ipairs(ROOMS) do
  local j = JOBS[r.job]
  local who = "[" .. j.name .. " Guide Room]"
  npc {
    map = "new_zone04", x = j.at[1], y = j.at[2], dir = 4, sprite = 105, name = "Helper",
    src = "m631:conf/npc_job_newbie.txt:" .. ({ 2461, 2496, 2527, 2558, 2587, 2618 })[r.job],
    on_click = function()
      save_point(table.unpack(r.save))
      mes(who)
      mes "A notice for those who have"
      mes("come to the " .. j.name .. " guide")
      mes "room. The guilds have their"
      mes "troubles just now, and their"
      mes "people will be late in coming"
      mes "here, we are told."
      next()
      mes(who)
      mes "Each guild has sent us the"
      mes "least a beginner needs. For"
      mes "the time being the training"
      mes "ground's own staff look after"
      mes "this room."
      next()
      mes(who)
      for _, line in ipairs(r.last) do mes(line) end
      if v.nov_get_item05 == 0 then
        v.nov_get_item05 = 1
        r.gift()
      end
      next()
      warp(table.unpack(r.to))
    end,
  }
end

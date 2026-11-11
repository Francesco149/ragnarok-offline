-- The training ground where 817's script is broken: the forest Helper, Leo
-- Von Frisch's lesson when it is cut short, the result, the subsidy, and the
-- six job guide rooms for those who take the job the test suggests.
--
-- 817's script (npc/jobs/novice.txt) is an English port of the one mod0631
-- has in Japanese (conf/npc_job_newbie.txt, archived 2003-12-18, tier
-- athena2003, jRO lineage). Parts of the port read like another version of
-- the original (two of Leo's questions differ) and are left alone; these are
-- the places where 817 contradicts itself or cannot be played, mended from
-- mod0631 (docs/DEVIATIONS.md, research/THREADS.md #6):
--  * The last Helper (new_1-4 60,149). "I will follow the result of the
--    test" answers "Under development. Please choose your job manualy."
--    mod0631 sends you to one of six guide rooms on the same map
--    (npc_job_newbie.txt:2439-2459), where an attendant hands over what the
--    job's guild sent and passes you on to the town (:2461-2650). The rooms
--    are on this client's new_zone04: five in the top row and one below the
--    last, with nothing else in them.
--  * The subsidy table is scrambled: a total of 8 or less gets the most
--    (500z, 10 Red Herbs) and a total over 16 *loses* 500z. mod0631
--    (:2262-2284): up to 8, 100z and 1 Red Herb; up to 15, 150z and 3; 16,
--    300z and 5; more, 500z and 10; a Milk each time.
--  * The first course's points are never counted (817's Leo Von Frisch
--    hands out the prizes for right answers but leaves NOV_1ST_COS alone),
--    so the result always read "1st course: 0 / 6 Points". mod0631 adds a
--    point for each right answer (:531-1115).
--  * The forest Helper (new_1-2 16,183) saves you inside the forest, so a
--    novice the Fabres knock out wakes among them, although the Helper
--    beside him says "you will lose your points and come back here", and
--    his own "knocked out by Fabre ... 2nd Course 1 th test" can never be
--    heard. mod0631 (:1687-1766) saves at new_1-2 23,188, counts the tries
--    (each costs a point of the second course) and lets you give the forest
--    up at the fifth and from the tenth.
--  * The Guide in the hall (new_1-2 100,113) gives everyone the same wrong
--    direction (below).
--  * Leo Von Frisch says nothing at all once his lesson is over and before
--    the NPC Master is heard (a jump to a label that does not exist), and a
--    lesson cut short after the last question can never be finished (the
--    script closes; the NPC Master waits for a 7 that never comes).
-- The English is 817's wherever it has a line; the attendants of the guide
-- rooms and the giving-up have no English anywhere, so theirs is ours,
-- translated from mod0631 and cut to the client's window (the name and six
-- lines of about 30 characters).

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
    local step = v.nov_1_2_cos_c
    if step == 6 then
      -- every question answered, the lesson's end not heard: 817 only closes
      mes "[Leo Von Frisch]"
      mes "Um..something interrupted my class without reason."
      next()
      give_item(RED_HERB, 4)
      v.nov_1_2_cos_c = 7
      mes "[Leo Von Frisch]"
      mes "I will reward you for the successful lesson. Take it. They will be useful when you have a hard time."
      next()
      mes "[Leo Von Frisch]"
      mes "Anyway,Let's call it a day with me,you may go to the 'NPC' master."
      mes "If you can't find him, inquire at the sly dog at the centre"
      close()
      return
    elseif step == 7 then
      -- 817 jumps to a label it does not have and says nothing
      mes "[Leo Von Frisch]"
      mes "Um? What's matter? My class is over. Your next course would be 'NPC Guide'...."
      close()
      return
    end
    if step > 7 then return orig(at) end -- a second hearing: its answers no longer count (mod0631 asks once)
    if tmp.nov_quiz_open ~= 0 then credit() end -- a lesson walked away from
    tmp.nov_quiz_base = prizes()
    tmp.nov_quiz_open = 1
    orig(at)
    credit()
  end
end)

-- ------------------------------------------------------------------ the Guide
-- The Guide in the middle of the hall tells you where to go next. 817 strung
-- his tests together without the texts between them (`if (c != 9) goto L00a;
-- if (c != 8) goto L00b; ...`), so everyone not yet done heard "finished the
-- First Course?" and the three other answers could never be heard. mod0631
-- (:215-247): 9 done, 8 hear the other one in the east room, 7 go east,
-- otherwise go to Leo. Every line is 817's.
npc {
  map = "new_zone02", x = 100, y = 113, dir = 4, sprite = 97, name = "Guide",
  replaces = "817:jobs/novice.txt:818",
  src = "m631:conf/npc_job_newbie.txt:193",
  on_click = function()
    mes "[Guide]"
    if rand(2) == 0 then
      mes "Mumble~Mumble~Life is getting Sick~What a boring Job I have~~ I was stupid enough to choose this Boring and Solitary thing!Ah~"
    else
      mes "I miss my good-for-nothing day~ Phew~ I should never accepted the Full time job.... Eh?!"
    end
    mes "Hey Buddy~ Want some Help?"
    next()
    if menu("Current Course State.", "Quit.") ~= 1 then
      mes "[Guide]"
      mes "Alright~ Bye Bye~ Don't comback unless you have no idea~"
      close()
      return
    end
    local step = v.nov_1_2_cos_c
    mes "[Guide]"
    mes "Hmm.. Let me see..."
    if step >= 9 then
      mes "Ah....? finished the First Course?"
      mes "Then meet the guy over there at the southwest,tell him 'I got the 1st lesson.'.He will lead you to the Second Hall."
      mes "Well done~ Bye Bye~ Take Care,Okay?"
    elseif step == 8 then
      mes "Err...?Didn't you hear the 'NPC Guide' Master's saying? Told you to go to the table right next him. Sigh~ Hurry up~"
    elseif step == 7 then
      mes "Did you attend to 'Sir Leo Von Frisch''s Lesson already? Umm the next is...."
      mes "Go toward next hall at the Southeast,you will see two guys over there."
      mes "Ask either of them~"
    else
      mes "I assure you are at the first place with this empty list of yours. You must go toward the Northwest."
      mes "And meet 'Leo Von Frisch',the odd veteran Knight.He is a little bit boring,but that's ok.If you work out a correct answer to his question,he will give you some Snacks. So concentrate on him,Okay?"
    end
    close()
  end,
}

-- ---------------------------------------------------------------- 2nd course
-- The Helper at the door of the Fabre forest. 817 counts the course in one
-- rising number: 10 = signed up with the Helper beside him, 11 = has been in.
local function into_the_forest()
  mes "[Helper]"
  mes "Well ..We wish you luck."
  next()
  warp("new_zone03", 96, 21)
end

local function give_up_the_forest()
  mes "[Helper]"
  mes "I see. Then the 2nd Course"
  mes "ends here. Well done."
  next()
  v.nov_2nd_tries = 11
  warp("new_zone04", 100, 12)
end

npc {
  map = "new_zone02", x = 16, y = 183, dir = 5, sprite = 84, name = "Helper",
  replaces = "817:jobs/novice.txt:61",
  src = "m631:conf/npc_job_newbie.txt:1687",
  on_click = function()
    local step = v.nov_1_2_cos_c
    if step < 10 then
      mes "[Helper]"
      mes "Heh... I can't find your name on the list."
      mes "Could you register first at the front desk?"
      close()
      return
    end
    if step == 10 then
      mes "[Helper]"
      mes "Heh... I can't find your name on the list....."
      mes("Ah! Here you are. Ok " .. player.name .. ". Are you ready?")
      next()
      if menu("Yes.", "No.") ~= 1 then
        mes "[Helper]"
        mes "Take some rest before the 2nd Course."
        close()
        return
      end
      v.nov_1_2_cos_c = 11
      v.nov_2nd_tries = 0
      save_point("new_zone02", 23, 188)
      return into_the_forest()
    end
    -- back from the forest
    local tries = v.nov_2nd_tries + 1
    v.nov_2nd_tries = tries
    mes "[Helper]"
    if tries >= 10 or tries == 5 then
      mes("2nd Course, " .. tries .. " th test...")
      if tries == 5 then
        mes "You know... you may give the"
        mes "2nd Course up, too..."
      else
        mes "...Shall we give it up?"
      end
      next()
      if menu("Yes...", "No!") == 1 then return give_up_the_forest() end
      mes "[Helper]"
      mes "...As you like."
      mes "Then are you going to start now?"
    elseif tries >= 6 then
      mes("2nd Course, " .. tries .. " th test...")
      mes "Are you going on?"
    else
      mes "You've been knocked out by Fabre.Yeah you can't fight with a bunch of Fabre anyway."
      mes("By the way,2nd Course " .. tries .. " th test.Are you going to start now?")
    end
    next()
    if menu("Yes.", "No.") == 1 then return into_the_forest() end
    mes "[Helper]"
    mes "Take some rest before the 2nd Course."
    close()
  end,
}

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
  v.nov_2nd_tries = 0
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
    local second = v.nov_2nd_tries < 10 and 10 - v.nov_2nd_tries or 0 -- a point lost for each time knocked out
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

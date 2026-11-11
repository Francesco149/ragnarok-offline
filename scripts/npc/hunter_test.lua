-- The Hunter test: an interview at the guild, things to bring, and the
-- trap room on job_hunte.
--
-- The map: job_hunte is in 2003-04-30gdata_turtle_ph_2.gpf, the second
-- Turtle Island patch file on the CD, which euRO applied in January 2004
-- (research/THREADS.md #8). Every cell below is walkable on it
-- (research/gat_cells.py on the merged GRF), the traps included.
--
-- Sources:
--   * a player's walkthrough, iRO: Forever-Zero's Archer/Hunter guide on
--     GameFAQs (faq 27633), "1.9 (02/15/04) - Added the Hunter Quest"
--     (research/notes/round5-6-review.md). It gives the shape, and it is
--     what decides where the scripts below part: the test came with Turtle
--     Island (before it the front desk changed you at once, as 817's NPC
--     still does); a personality quiz at the front desk; items from the
--     receptionist on her left; the guild master in Payon gives 200 Silver
--     Arrows and sends you to a waiting room, one candidate at a time; four
--     "Job Change Monster"s to kill among others called "Hunter Change
--     Monster" or "Job Test Monster", watery tiles that are traps; then a
--     switch in the middle with "escape", and the far side's exit; a
--     necklace from the guild master for the front desk. Its item lists
--     are the three below.
--   * eAthena 1.0 npc/jobs/2-1/hunter.txt (2004-11-04, "converted from
--     aegis script", translated by yoshiki like 817's Wizard test; in
--     data:refs/rathena, commit 706e78509): the NPCs, their cells on
--     in_hunter and payon_in02, the words, the arena's monster and trap
--     cells. Tier athena2003, nine months after the walkthrough.
--   * rAthena's npc/jobs/2-1/hunter.txt (same lineage, later; grade D):
--     only where the walkthrough confirms it against eAthena 1.0: the
--     decoys' names (eAthena calls them all "Test Monster"), the switch's
--     three buttons, and three minutes for the whole room (eAthena: 30
--     seconds after the switch).
--
-- What is ours (docs/DEVIATIONS.md):
--   * no waiting room: the Guide asks whether you are ready, or lets you
--     lounge first (as in the Wizard test);
--   * the receptionist's lists are the walkthrough's three ("all of the
--     possible item combonations I have found"), not eAthena's five;
--   * one guild master, in the Archer Guild (eAthena). The walkthrough
--     also has one "at the central palace"; this client has no payon_in03;
--   * mod0631's two warp tiles on job_hunte ("to 0,0", npc_warp25.txt:461)
--     are removed: they are the room's two ends, and the test moves you;
--   * pages cut to the dialogue window, typos mended.

remove_warp("job_hunte", 89, 139)
remove_warp("job_hunte", 89, 68)

local MAP = "job_hunte"
local ARENA = { 50, 64, 129, 143 }
local ROOM = { 176, 22 }          -- the waiting room
local GUILD_ROOM = { "payon_in02", 21, 27 }
local NECKLACE, SILVER_ARROW = 1007, 1751

-- v.hntr_q: 1 interviewed, 2 items brought, 3 passed the room
-- v.hntr_q2: at 0: the interview failed (1); at 1: the item list (1-3);
--   at 2: sent to the room (1) or through it (2)
-- v.hntr_jl: the job level on the form (40 or 50: which bow you get)

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function done_with_job_vars()
  v.hntr_q, v.hntr_q2, v.hntr_jl = 0, 0, 0
end

-- ------------------------------------------------------------ Sharon

local S = "[Hunter Sharon]"

-- each question: the text, the three answers, and which of them score
local QUIZ = {
  { { "You've been training for a while,", "and you're running out of ideas", "about where to hunt.", "What do you do?" },
    { "Walk around screaming, 'Where should I go hunt!?'", "Ask someone politely.", "Just explore." }, { 2, 3 } },
  { { "You plan to go to the Sograt", "Desert to hunt Hodes.", "But you're in Payon!", "How do you get to the desert?" },
    { "Go beg priests for warps.", "Kafra warp.", "Walk there with a friend." }, { 2, 3 } },
  { { "No priests around, your friends", "are busy, and you don't have", "enough money for a Kafra warp.", "How do you get the money?" },
    { "Beg.", "Sell useless items.", "Go hunting in a nearby field." }, { 2, 3 } },
  { { "You somehow get to the desert,", "but now you feel too weak", "to hunt Hodes.", "What do you do now?" },
    { "Go cliff hunting.", "Go rest at Morroc.", "Shoot the Hodes other people are attacking." }, { 2 } },
  { { "Back in Morroc with no HP left,", "you see a priest.", "How do you ask for a heal?" },
    { "If it's ok, can I have a heal please?", "Heal please.", "heal plz" }, { 1 } },
  { { "You find a rare item, and the", "market is full of people and", "chat rooms. What do you do", "to sell it quicker?" },
    { "Go into chat rooms and ask.", "Make your own chat room.", "See if anyone is buying." }, { 2, 3 } },
  { { "After a while, a person starts", "to beg you. What do you do?" },
    { "Give some items and money.", "Ignore that person.", "Tell them where to level and make money." }, { 3 } },
  { { "Hunting in the woods, you find", "someone who's lost.", "What do you do?" },
    { "Give him directions.", "Escort him personally.", "Ignore that person." }, { 1, 2 } },
  { { "Back to hunting: you see", "someone fighting an MVP!", "What do you do?" },
    { "Watch, and attack if asked for help.", "Start attacking.", "Return to town in panic." }, { 1 } },
  { { "On your way back to town you", "find a rare item on the street.", "What do you do?" },
    { "Finders keepers.", "Try to find the owner.", "Walk past it." }, { 2, 3 } },
}

local function interview()
  page(S, "I'm going to ask you some simple", "questions, to get a feel for who", "you are and why you want to", "become a Hunter.")
  next()
  page(S, "Relax and answer honestly", "and thoughtfully.")
  next()
  local score = 0
  for _, q in ipairs(QUIZ) do
    page(S, table.unpack(q[1]))
    next()
    local c = menu(q[2][1], q[2][2], q[2][3])
    for _, good in ipairs(q[3]) do
      if c == good then score = score + 10 end
    end
  end
  page(S, "Now we're done!!")
  next()
  if score < 90 then
    page(S, "I... don't think I can pass you.", "The way you live your life is,", "well... unacceptable for a", "Hunter candidate.")
    emotion(4)
    next()
    page(S, "Think carefully about my questions", "and your answers. Working well", "with people is just as important", "as loving nature.")
    v.hntr_q2 = 1
    return close()
  end
  if score < 100 then
    page(S, "You didn't do as well as I hoped,", "but I'll pass you... Though I", "don't know what the guild master", "will think of you...")
    next()
    page(S, "Work hard on the next test.", "Make Sharon happy, okies?")
  else
    page(S, "Great! From your answers, you", "seem to be just the type of", "person we're looking for.")
    emotion(21)
    next()
    page(S, "With your kind of attitude and", "values, you should have no", "problems becoming a Hunter.")
  end
  next()
  page(S, "Next, go and see ^5533FFthe Demon", "Hunter^000000 over there to my left.", "He'll tell you about the second", "test.")
  next()
  page(S, "Good luck!")
  v.hntr_q, v.hntr_q2 = 1, 0
  close()
end

npc {
  map = "in_hunter", x = 99, y = 99, dir = 4, sprite = 727, name = "Hunter Sharon",
  replaces = "817:jobs/hunter.txt:17", src = "eathena-1.0:npc/jobs/2-1/hunter.txt:38",
  on_click = function()
    local c = player.class
    if c == Job_Hunter then
      page(S, "Hey, " .. player.name .. "! Why did you", "come back? Falcon ran away or", "something? Tee hee.")
      emotion(18)
      next()
      page(S, "I hope you didn't think you could", "get free stuff from us just", "because you're part of our guild", "now...")
      return close()
    end
    if c ~= Job_Archer then
      if c == Job_Novice then
        page(S, "What's a novice like you doing", "here? Leave, we have nothing", "for you.")
        emotion(1)
      else
        page(S, "Good day! It's nice to meet you", "and all, but you have no", "business here.")
      end
      return close()
    end

    local q = v.hntr_q
    if q == 1 then
      page(S, "What you have to do next is go", "and see ^5533FFthe Demon Hunter^000000 over", "there to my left.")
      return close()
    end
    if q == 2 then
      page(S, "Hmmm? The guild master? Oh, he's", "out right now.")
      emotion(20)
      next()
      page(S, "If I remember right, he should be", "in Payon's Archer Village. He had", "to talk with someone in the", "^5533FFArcher Guild^000000.")
      return close()
    end
    if q == 3 then
      if player.skill_point > 0 then
        page(S, "You still have skill points left", "over. Speak with me after you've", "used them up.")
        return close()
      end
      if count_item(NECKLACE) < 1 then
        page(S, "Hmmm, I received news of your", "success... But you don't seem to", "have the ^5533FFNecklace of Wisdom^000000", "as proof.")
        emotion(20)
        next()
        page(S, "You need the Necklace of Wisdom", "to become a Hunter. Without it,", "you will have to start the test", "over.")
        next()
        if menu("Um... I've got it... somewhere...", "Heh heh... I must have misplaced it...") == 1 then
          page(S, "Well then, go get it!")
          emotion(6)
          return close()
        end
        page(S, "...That's just pathetic. Have fun", "redoing the ENTIRE TEST!")
        emotion(32)
        v.hntr_q, v.hntr_q2 = 0, 0
        return close()
      end
      page(S, "Huh? Did you pass the test?")
      next()
      page(S, "Congratulations!!")
      emotion(21)
      next()
      page(S, "Now I can change you into", "a Hunter!")
      next()
      take_item(NECKLACE, 1)
      local bow = v.hntr_jl == 50 and 1718 or 1710
      job_change(Job_Hunter)
      page(S, "There you go! You look great in", "the Hunter's outfit.")
      emotion(21)
      next()
      page(S, "For working so hard, the guild", "has a small reward for you.")
      give_item(bow, 1)
      done_with_job_vars()
      next()
      page(S, "Do all of us a favor: act", "responsibly, and love and respect", "nature! Good luck, and remember", "you are always welcome here!")
      return close()
    end

    mes(S)
    if v.hntr_q2 == 1 then
      mes "Hmm, what's this? You want to do"
      mes "the interview again? Very well."
      mes "Hopefully you'll do better."
      next()
      return interview()
    end
    mes "Hey, Archer!! Hmmm... You look like"
    mes "you've trained pretty hard."
    mes "I take it you're here to be"
    mes "a Hunter!"
    next()
    local m = menu("Of course!", "What are the requirements?", "Not right now.")
    if m == 2 then
      page(S, "The requirements?", "1. Be an Archer", "2. Have a job level of at least 40", "3. Pass all of the tests")
      next()
      page(S, "If you trained hard enough, you", "shouldn't have any problems", "becoming a Hunter.")
      return close()
    end
    if m ~= 1 then
      page(S, "Ok, I'll see you later.")
      return close()
    end
    page(S, "Heh, I knew it. First fill out", "this application form.")
    next()
    mes "(You fill out the form and hand it back.)"
    next()
    if player.job_level < 40 then
      page(S, "Hmm... It looks like you'll need", "more training. To become a Hunter,", "you need a great deal of", "experience.")
      next()
      page(S, "A ^5533FFjob level of 40^000000 is required", "of all Hunter candidates. Please", "continue your training, and good", "luck.")
      return close()
    end
    if player.job_level >= 50 then
      v.hntr_jl = 50
      page(S, "Well, well! A high level Archer...", "very nice! You must have trained", "really hard.")
      emotion(5)
      next()
    else
      v.hntr_jl = 40
      page(S, "Looks good.")
      next()
    end
    page(S, "My name is Sharon, nice to meet", "you. Before I explain how to become", "a Hunter, I'd like to do a short", "interview with you.")
    next()
    if player.job_level < 50 then
      page(S, "Do you want to do the interview", "now?")
      next()
      if menu("Yeah, let's begin.", "Nah, I'll come back in a bit.") ~= 1 then
        page(S, "Ok, I'll see you later.")
        return close()
      end
    end
    interview()
  end,
}

-- ------------------------------------------------------------ the Demon Hunter

local D = "[Demon Hunter]"

-- the walkthrough's three lists ("ITEM COMBO 1" to 3)
local LISTS = {
  { { 508, 9, "Yellow Herbs" }, { 935, 9, "Shells" }, { 955, 9, "Worm Peelings" } },
  { { 925, 3, "Bills of Birds" }, { 932, 4, "Skel-Bones" }, { 511, 5, "Green Herbs" } },
  { { 509, 5, "White Herbs" }, { 1019, 5, "Trunks" }, { 7030, 5, "Claws of Desert Wolf" } },
}

local function say_list(list)
  mes(D)
  for _, it in ipairs(list) do mes("^5533FF" .. it[2] .. " " .. it[3] .. "^000000") end
end

npc {
  map = "in_hunter", x = 55, y = 99, dir = 7, sprite = 732, name = "Demon Hunter",
  src = "eathena-1.0:npc/jobs/2-1/hunter.txt:376",
  on_click = function()
    if player.class == Job_Hunter then
      page(D, "Look at you! Nice and spiffy in", "that Hunter's outfit. How does it", "feel? Good, I bet. Well, good luck", "to you.")
      emotion(0)
      return close()
    end
    if player.class ~= Job_Archer then
      page(D, "They call me the Demon Hunter.", "What's that? You want to know why", "I'm called that?...")
      next()
      page(D, "heh... heh... heh...")
      next()
      page(D, "For your safety... it's best that", "you DON'T find out...")
      emotion(29)
      return close()
    end
    local q = v.hntr_q
    if q == 2 then
      page(D, "Huh? Can't you find the guild", "master? He should be in the", "^5533FFArcher Guild in Archer Village^000000.", "Go find him.")
      emotion(20)
      return close()
    end
    if q == 3 then
      page(D, "Ah, you passed the test! Congrats,", "go and see Sharon now.")
      return close()
    end
    if q ~= 1 then
      page(D, "You'll have to speak with", "^5533FFHunter Sharon^000000 first if you want", "to become a Hunter.")
      return close()
    end
    local list = LISTS[v.hntr_q2]
    if list then
      page(D, "Yes?")
      next()
      for _, it in ipairs(list) do
        if count_item(it[1]) < it[2] then
          page(D, "You don't have everything yet.", "Again, the items are:")
          next()
          say_list(list)
          next()
          page(D, "When you have them all, come back!")
          return close()
        end
      end
      for _, it in ipairs(list) do take_item(it[1], it[2]) end
      page(D, "Good job. You brought everything.", "Now go and find the ^5533FFGuild", "Master^000000. I think he's in the", "^5533FFArcher Guild^000000.")
      next()
      page(D, "He'll give you your final test.", "Good luck.")
      v.hntr_q, v.hntr_q2 = 2, 0
      return close()
    end
    page(D, "Hello, I'm the examiner they call", "the Demon Hunter. Is ^5566FF" .. player.name .. "^000000", "your name?")
    next()
    if menu("Yes.", "Uhhh... no.") ~= 1 then
      page(D, "Stop playing around, " .. player.name .. ".", "That's your name, right?")
      next()
      if menu("Yes.", "Uhhh... no.") ~= 1 then
        page(D, "DON'T you mess around with me!", "If you're going to fool around,", "then LEAVE!")
        emotion(32)
        close()
        return warp("pay_fild10", 133, 235)
      end
    end
    page(D, "Good. Because we make the arrows", "used in some of the tests, we also", "need the materials to make them.")
    next()
    page(D, "If you haven't noticed, our guild", "is NOT as wealthy as some of the", "others. We're in the middle of", "the forest, for pete's sake...")
    emotion(7)
    next()
    page(D, "Ahem... That is where Hunter", "wannabes like you come in handy.", "For this test, you will gather", "materials for our arrows.")
    next()
    page(D, "Let's see... the items you need", "to gather are...")
    next()
    v.hntr_q2 = rand(1, #LISTS)
    say_list(LISTS[v.hntr_q2])
    next()
    page(D, "When you have them all, return!")
    close()
  end,
}

-- ------------------------------------------------------------ the guild master

local M = "[Guild Master]"

local function briefing_menu()
  while true do
    local c = menu("What's the test about?", "Anything I should know?", "Start the test.")
    if c == 3 then return true end
    if c == 1 then
      page(M, "A test of speed, agility and", "target recognition. You will be", "sent to a room full of monsters,", "and ^5533FFkill 4^000000 of them:")
      next()
      page(M, "not just any 4: the ones called", "^5533FFJob Change Monster^000000. Kill the", "wrong one and you fail. And there", "are traps all over the room.")
      next()
      page(M, "Step on a trap and you fail too.", "Then a switch in the middle of", "the room opens the way out, at", "the far end.")
      next()
      page(M, "You have ^FF5533" .. "3 minutes^000000 for all of it.", "This will teach you what a Hunter", "needs most: moving quickly, and", "tracking.")
      next()
    elseif c == 2 then
      page(M, "Only one person can take the test", "at a time. You wait in the waiting", "room until it's your turn.")
      next()
      page(M, "Aim carefully: there are a lot of", "monsters, and you don't want to", "hit the wrong one. Watch their", "names.")
      next()
      page(M, "And ALWAYS watch your step!", "There are traps everywhere!")
      next()
    else
      return false
    end
  end
end

local function send_to_room()
  if count_item(SILVER_ARROW) <= 5 then
    page(M, "This is where your work for the", "second test pays off. Here are", "some arrows, made with the items", "you collected.")
    give_item(SILVER_ARROW, 200)
    next()
  end
  page(M, "Good luck. I'll send you to the", "test room now.")
  next()
  v.hntr_q2 = 1
  close()
  warp("job_hunte", 176, 22) -- literal, so that `check` follows it
end

npc {
  map = "payon_in02", x = 21, y = 31, dir = 1, sprite = 59, name = "Guild Master",
  src = "eathena-1.0:npc/jobs/2-1/hunter.txt:541",
  on_click = function()
    if player.class == Job_Hunter then
      page(M, "Ah, it's you again, " .. player.name .. "!", "Good to see you're well. Me? Still", "busy as usual.")
      next()
      page(M, "Well, I've got to get back to", "work. Stay safe.")
      return close()
    end
    if player.class ~= Job_Archer then
      page("[Hunter]", "Is there something you want of me?", "I have some business to take care", "of, so please be quiet.")
      emotion(20)
      return close()
    end
    local q = v.hntr_q
    if q == 1 then
      page(M, "Yes? What would an Archer want", "from me? The guild didn't send me", "word about you.")
      next()
      page(M, "Did you bring the guild the items", "they asked for? You have to do", "that first.")
      return close()
    end
    if q == 3 then
      page(M, "Hmm? What is it? Don't waste your", "time here. Go on and become a", "Hunter already...")
      return close()
    end
    if q ~= 2 then
      page(M, "You want to be a Hunter badly,", "don't you? In my day, I worked for", "over a month to become one, hehe.")
      next()
      page(M, "If there's nothing else, I suggest", "you go back to the guild!")
      return close()
    end
    if v.hntr_q2 == 2 then
      page(M, "I see you succeeded. Great job!", "Here is the proof of your success,", "the ^5533FFNecklace of Wisdom^000000.")
      emotion(21)
      next()
      page(M, "Show it to Hunter Sharon, or you", "WON'T be able to become a Hunter.")
      give_item(NECKLACE, 1)
      v.hntr_q, v.hntr_q2 = 3, 0
      next()
      page(M, "Anyway, I've got some work left", "to do here. See you later.")
      return close()
    end
    if v.hntr_q2 == 1 then
      page(M, "Ah, you're one of the Archers that", "failed. Here, let me heal your", "wounds...")
      percent_heal(100, 100)
      next()
      page(M, "You're ready, right?")
    else
      page(M, "So you've come to take the test?", "I see... Do you have any questions", "before we start?")
    end
    next()
    if briefing_menu() then return send_to_room() end
    close()
  end,
}

-- ------------------------------------------------------------ the room

-- the four to kill (eAthena 1.0 spawns six; four are enough)
local TARGETS = {
  { 67, 80, 1015 }, { 114, 78, 1015 }, { 89, 127, 1002 }, { 53, 73, 1041 }, { 125, 70, 1016 }, { 90, 92, 1015 },
}
-- eAthena 1.0's decoy cells and monsters, rAthena's names
local DECOYS = {
  { 85, 100, 1016, "Job Test Monster" }, { 72, 102, 1041, "Job Test Monster" }, { 108, 103, 1015, "Job Test Monster" },
  { 88, 127, 1002, "Job Test Monster" }, { 125, 69, 1015, "Job Test Monster" }, { 77, 112, 1016, "Job Tester Monster" },
  { 53, 106, 1015, "Job Tester Monster" }, { 53, 73, 1002, "Job Tester Monster" }, { 125, 70, 1015, "Job Tester Monster" },
  { 90, 91, 1015, "Job Tester Monster" }, { 67, 80, 1015, "Hunter Change Monster" }, { 77, 112, 1016, "Hunter Change Monster" },
  { 53, 106, 1015, "Hunter Change Monster" }, { 53, 73, 1015, "Hunter Change Monster" }, { 125, 70, 1015, "Hunter Change Monster" },
  { 90, 91, 1041, "Job Transfer Monster" }, { 85, 100, 1002, "Job Transfer Monster" }, { 72, 102, 1015, "Job Transfer Monster" },
  { 108, 103, 1015, "Job Transfer Monster" }, { 77, 112, 1015, "Job Transfer Monster" },
  { 112, 139, 1015, "Binnie" }, { 112, 139, 1015, "Darrel" }, { 112, 139, 1015, "Rex" }, { 112, 139, 1015, "Anselmo" },
  { 90, 91, 1015, "Anolian" }, { 53, 73, 1002, "Monster Sample" }, { 53, 106, 1015, "Not Me" },
  { 77, 112, 1015, "Help Me" }, { 72, 102, 1015, "Do Not Hit Me" }, { 108, 103, 1015, "Attack Speed 184" },
}
-- eAthena 1.0's traps (381 of them): cell and reach (x, y)
local TRAPS = {
  { 52, 140, 0, 1 }, { 53, 140, 0, 1 }, { 54, 141, 0, 0 }, { 55, 141, 0, 0 }, { 55, 140, 0, 0 }, { 54, 140, 0, 0 },
  { 52, 138, 0, 0 }, { 53, 138, 0, 0 }, { 62, 140, 0, 1 }, { 63, 140, 0, 1 }, { 64, 140, 0, 0 }, { 64, 141, 0, 0 },
  { 65, 140, 0, 0 }, { 65, 141, 0, 0 }, { 62, 138, 0, 0 }, { 63, 138, 0, 0 }, { 72, 140, 0, 1 }, { 73, 140, 0, 1 },
  { 72, 138, 0, 0 }, { 78, 140, 0, 0 }, { 78, 141, 0, 0 }, { 79, 140, 0, 0 }, { 79, 141, 0, 0 }, { 82, 138, 0, 0 },
  { 82, 139, 0, 0 }, { 83, 138, 0, 0 }, { 83, 139, 0, 0 }, { 99, 138, 1, 0 }, { 99, 139, 1, 0 }, { 101, 138, 0, 0 },
  { 101, 139, 0, 0 }, { 106, 140, 0, 1 }, { 107, 140, 0, 1 }, { 106, 138, 0, 0 }, { 107, 138, 0, 0 }, { 112, 140, 0, 0 },
  { 112, 141, 0, 0 }, { 113, 140, 0, 0 }, { 113, 141, 0, 0 }, { 116, 140, 0, 0 }, { 116, 141, 0, 0 }, { 117, 140, 0, 0 },
  { 117, 141, 0, 0 }, { 120, 138, 0, 0 }, { 120, 139, 0, 0 }, { 121, 138, 0, 0 }, { 121, 139, 0, 0 }, { 126, 139, 0, 2 },
  { 127, 139, 0, 2 }, { 126, 136, 0, 0 }, { 127, 136, 0, 0 }, { 52, 134, 0, 1 }, { 53, 134, 0, 1 }, { 52, 132, 0, 0 },
  { 53, 132, 0, 0 }, { 124, 130, 0, 0 }, { 124, 131, 0, 0 }, { 125, 130, 0, 0 }, { 125, 131, 0, 0 }, { 64, 128, 0, 0 },
  { 64, 129, 0, 0 }, { 65, 128, 0, 0 }, { 65, 129, 0, 0 }, { 68, 126, 0, 0 }, { 68, 127, 0, 0 }, { 69, 126, 0, 0 },
  { 69, 127, 0, 0 }, { 75, 128, 1, 0 }, { 75, 129, 1, 0 }, { 77, 128, 0, 0 }, { 77, 129, 0, 0 }, { 82, 126, 0, 0 },
  { 82, 127, 0, 0 }, { 83, 126, 0, 0 }, { 83, 127, 0, 0 }, { 96, 128, 0, 0 }, { 96, 129, 0, 0 }, { 97, 128, 0, 0 },
  { 97, 129, 0, 0 }, { 100, 126, 0, 0 }, { 100, 127, 0, 0 }, { 101, 126, 0, 0 }, { 101, 127, 0, 0 }, { 106, 128, 0, 0 },
  { 106, 129, 0, 0 }, { 107, 128, 0, 0 }, { 107, 129, 0, 0 }, { 112, 126, 0, 0 }, { 112, 127, 0, 0 }, { 113, 126, 0, 0 },
  { 113, 127, 0, 0 }, { 126, 126, 0, 0 }, { 126, 127, 0, 0 }, { 127, 126, 0, 0 }, { 127, 127, 0, 0 }, { 52, 122, 0, 1 },
  { 53, 120, 1, 0 }, { 54, 121, 0, 0 }, { 55, 121, 0, 0 }, { 55, 120, 0, 0 }, { 66, 120, 0, 0 }, { 66, 121, 0, 0 },
  { 67, 120, 0, 0 }, { 67, 121, 0, 0 }, { 114, 118, 0, 0 }, { 114, 119, 0, 0 }, { 115, 118, 0, 0 }, { 115, 119, 0, 0 },
  { 124, 120, 0, 1 }, { 125, 120, 0, 1 }, { 124, 118, 0, 0 }, { 125, 118, 0, 0 }, { 66, 116, 0, 0 }, { 66, 117, 0, 0 },
  { 67, 116, 0, 0 }, { 67, 117, 0, 0 }, { 76, 114, 0, 0 }, { 76, 115, 0, 0 }, { 77, 114, 0, 0 }, { 77, 115, 0, 0 },
  { 82, 116, 0, 0 }, { 82, 117, 0, 0 }, { 83, 116, 0, 0 }, { 83, 117, 0, 0 }, { 86, 114, 0, 0 }, { 86, 115, 0, 0 },
  { 87, 114, 0, 0 }, { 87, 115, 0, 0 }, { 92, 115, 1, 0 }, { 92, 114, 1, 0 }, { 90, 115, 0, 0 }, { 102, 116, 0, 0 },
  { 102, 117, 0, 0 }, { 103, 116, 0, 0 }, { 103, 117, 0, 0 }, { 114, 114, 0, 0 }, { 114, 115, 0, 0 }, { 115, 114, 0, 0 },
  { 115, 115, 0, 0 }, { 54, 110, 0, 1 }, { 55, 110, 0, 1 }, { 54, 108, 0, 0 }, { 55, 108, 0, 0 }, { 64, 112, 0, 0 },
  { 64, 113, 0, 0 }, { 65, 112, 0, 0 }, { 65, 113, 0, 0 }, { 80, 110, 0, 0 }, { 80, 111, 0, 0 }, { 81, 110, 0, 0 },
  { 81, 111, 0, 0 }, { 90, 108, 0, 6 }, { 91, 108, 0, 5 }, { 91, 102, 0, 0 }, { 98, 109, 0, 2 }, { 99, 109, 0, 2 },
  { 103, 109, 3, 0 }, { 103, 108, 3, 0 }, { 97, 106, 2, 0 }, { 96, 107, 1, 0 }, { 94, 108, 0, 1 }, { 95, 109, 0, 0 },
  { 95, 108, 0, 0 }, { 94, 106, 0, 0 }, { 96, 102, 0, 3 }, { 97, 102, 0, 3 }, { 95, 98, 2, 0 }, { 94, 99, 1, 0 },
  { 92, 98, 0, 0 }, { 112, 110, 0, 0 }, { 112, 111, 0, 0 }, { 113, 110, 0, 0 }, { 113, 111, 0, 0 }, { 126, 108, 0, 1 },
  { 127, 108, 0, 1 }, { 126, 106, 0, 0 }, { 53, 102, 1, 1 }, { 55, 102, 0, 1 }, { 53, 100, 1, 0 }, { 55, 100, 0, 0 },
  { 64, 106, 0, 0 }, { 64, 107, 0, 0 }, { 65, 106, 0, 0 }, { 65, 107, 0, 0 }, { 66, 100, 0, 0 }, { 66, 101, 0, 0 },
  { 67, 100, 0, 0 }, { 67, 101, 0, 0 }, { 86, 106, 0, 1 }, { 87, 106, 0, 1 }, { 82, 104, 5, 0 }, { 81, 105, 4, 0 },
  { 76, 105, 0, 0 }, { 76, 104, 0, 0 }, { 78, 101, 0, 2 }, { 79, 101, 0, 2 }, { 77, 98, 2, 0 }, { 74, 99, 0, 0 },
  { 74, 98, 0, 0 }, { 82, 100, 0, 0 }, { 82, 101, 0, 0 }, { 83, 100, 0, 0 }, { 83, 101, 0, 0 }, { 106, 104, 0, 0 },
  { 106, 105, 0, 0 }, { 107, 104, 0, 0 }, { 107, 105, 0, 0 }, { 112, 104, 0, 1 }, { 113, 104, 0, 1 }, { 112, 102, 0, 0 },
  { 113, 102, 0, 0 }, { 54, 92, 0, 0 }, { 54, 93, 0, 0 }, { 55, 92, 0, 0 }, { 55, 93, 0, 0 }, { 52, 90, 0, 0 },
  { 52, 91, 0, 0 }, { 53, 90, 0, 0 }, { 53, 91, 0, 0 }, { 64, 92, 0, 0 }, { 64, 93, 0, 0 }, { 65, 92, 0, 0 },
  { 65, 93, 0, 0 }, { 76, 94, 0, 0 }, { 76, 95, 0, 0 }, { 77, 94, 0, 0 }, { 77, 95, 0, 0 }, { 78, 92, 0, 0 },
  { 78, 93, 0, 0 }, { 79, 92, 0, 0 }, { 79, 93, 0, 0 }, { 86, 92, 0, 0 }, { 86, 93, 0, 0 }, { 87, 92, 0, 0 },
  { 87, 93, 0, 0 }, { 90, 96, 0, 0 }, { 90, 97, 0, 0 }, { 91, 96, 0, 0 }, { 91, 97, 0, 0 }, { 99, 95, 1, 0 },
  { 99, 94, 1, 0 }, { 101, 94, 0, 1 }, { 100, 93, 0, 0 }, { 100, 92, 0, 0 }, { 101, 92, 0, 0 }, { 102, 98, 0, 0 },
  { 102, 99, 0, 0 }, { 103, 98, 0, 0 }, { 103, 99, 0, 0 }, { 102, 90, 0, 0 }, { 102, 91, 0, 0 }, { 103, 90, 0, 0 },
  { 103, 91, 0, 0 }, { 114, 96, 0, 0 }, { 114, 97, 0, 0 }, { 115, 96, 0, 0 }, { 115, 97, 0, 0 }, { 112, 90, 0, 0 },
  { 112, 91, 0, 0 }, { 113, 90, 0, 0 }, { 113, 91, 0, 0 }, { 125, 97, 1, 0 }, { 125, 96, 1, 0 }, { 127, 96, 0, 0 },
  { 127, 97, 0, 0 }, { 52, 86, 0, 0 }, { 52, 87, 0, 0 }, { 53, 86, 0, 0 }, { 53, 87, 0, 0 }, { 66, 88, 0, 0 },
  { 66, 89, 0, 0 }, { 67, 88, 0, 0 }, { 67, 89, 0, 0 }, { 114, 84, 0, 0 }, { 114, 85, 0, 0 }, { 115, 84, 0, 0 },
  { 115, 85, 0, 0 }, { 126, 86, 0, 0 }, { 126, 87, 0, 0 }, { 127, 86, 0, 0 }, { 127, 87, 0, 0 }, { 54, 80, 0, 1 },
  { 55, 80, 0, 1 }, { 55, 80, 0, 0 }, { 55, 78, 0, 0 }, { 64, 82, 0, 1 }, { 65, 82, 0, 1 }, { 64, 80, 0, 0 },
  { 65, 80, 0, 0 }, { 66, 78, 0, 0 }, { 66, 79, 0, 0 }, { 67, 78, 0, 0 }, { 67, 79, 0, 0 }, { 74, 78, 0, 0 },
  { 74, 79, 0, 0 }, { 75, 78, 0, 0 }, { 75, 79, 0, 0 }, { 78, 80, 0, 0 }, { 78, 81, 0, 0 }, { 79, 80, 0, 0 },
  { 79, 81, 0, 0 }, { 82, 78, 0, 0 }, { 82, 79, 0, 0 }, { 83, 78, 0, 0 }, { 83, 79, 0, 0 }, { 94, 78, 0, 0 },
  { 94, 79, 0, 0 }, { 95, 78, 0, 0 }, { 95, 79, 0, 0 }, { 101, 80, 1, 0 }, { 101, 81, 1, 0 }, { 103, 81, 0, 0 },
  { 103, 80, 0, 0 }, { 104, 78, 0, 0 }, { 104, 79, 0, 0 }, { 105, 78, 0, 0 }, { 105, 79, 0, 0 }, { 110, 78, 0, 0 },
  { 110, 79, 0, 0 }, { 111, 78, 0, 0 }, { 111, 79, 0, 0 }, { 114, 80, 0, 0 }, { 114, 81, 0, 0 }, { 115, 80, 0, 0 },
  { 115, 81, 0, 0 }, { 124, 78, 0, 0 }, { 124, 79, 0, 0 }, { 125, 78, 0, 0 }, { 125, 79, 0, 0 }, { 126, 82, 0, 0 },
  { 126, 83, 0, 0 }, { 127, 82, 0, 0 }, { 127, 83, 0, 0 }, { 52, 68, 0, 1 }, { 53, 68, 0, 1 }, { 52, 66, 0, 0 },
  { 53, 66, 0, 0 }, { 54, 70, 0, 1 }, { 55, 70, 0, 1 }, { 54, 68, 0, 0 }, { 55, 68, 0, 0 }, { 59, 66, 1, 0 },
  { 59, 67, 1, 0 }, { 61, 67, 0, 0 }, { 61, 66, 0, 0 }, { 68, 68, 0, 1 }, { 69, 68, 0, 1 }, { 68, 66, 0, 0 },
  { 69, 66, 0, 0 }, { 76, 66, 0, 0 }, { 76, 67, 0, 0 }, { 77, 66, 0, 0 }, { 77, 67, 0, 0 }, { 82, 68, 0, 0 },
  { 82, 69, 0, 0 }, { 83, 68, 0, 0 }, { 83, 69, 0, 0 }, { 96, 66, 0, 0 }, { 96, 67, 0, 0 }, { 97, 66, 0, 0 },
  { 97, 67, 0, 0 }, { 100, 68, 0, 0 }, { 100, 69, 0, 0 }, { 101, 68, 0, 0 }, { 101, 69, 0, 0 }, { 107, 66, 1, 0 },
  { 107, 67, 1, 0 }, { 109, 67, 0, 0 }, { 109, 66, 0, 0 }, { 117, 69, 1, 0 }, { 117, 68, 1, 0 }, { 119, 69, 0, 0 },
  { 119, 68, 0, 0 }, { 124, 66, 0, 0 }, { 124, 67, 0, 0 }, { 125, 66, 0, 0 }, { 125, 67, 0, 0 }, { 126, 70, 0, 0 },
  { 126, 71, 0, 0 }, { 127, 70, 0, 0 }, { 127, 71, 0, 0 },
}

local TARGET_EV, DECOY_EV = "hunter_room::OnKilled", "hunter_room::OnWrong"
local running = false

local function reset()
  running = false
  npc_timer_stop("hunter_room")
  kill_monster(MAP, TARGET_EV)
  kill_monster(MAP, DECOY_EV)
  disable_npc("hunter_switch")
  disable_npc("hunter_exit")
end

-- out of the room, back to the waiting room, everything put back
local function fail(text)
  map_announce(MAP, text, 0)
  reset()
  warp(MAP, ROOM[1], ROOM[2])
end

local function start()
  reset()
  running = true
  tmp.hntr_kills = 0
  for _, m in ipairs(TARGETS) do monster(MAP, m[1], m[2], "Job Change Monster", m[3], 1, TARGET_EV) end
  for _, m in ipairs(DECOYS) do monster(MAP, m[1], m[2], m[4], m[3], 1, DECOY_EV) end
  npc_timer_start("hunter_room")
  warp(MAP, 90, 67)
end

-- rAthena's clock (the walkthrough and eAthena 1.0: three minutes)
local CLOCK = {
  [1000] = "The test shall now begin.",
  [3000] = "Only hunt the monsters labeled 'Job Change Monster'.",
  [5000] = "***** Be careful of the traps when hunting. *****",
  [7000] = "Once you hunt 4 'Job Change Monster's, the switch in the center will work.",
  [9000] = "When you have used the escape switch, leave through the portal at 12 o'clock.",
  [11000] = "Everything must be done within 3 minutes.",
  [14000] = " ****** 3 minutes remaining. ****** ",
  [74000] = " ****** 2 minutes remaining. ****** ",
  [134000] = " ****** 1 minute remaining. ****** ",
  [164000] = " ****** 30 seconds remaining. ****** ",
  [187000] = " Test ends in 5 seconds...",
  [188000] = " Test ends in 4 seconds...",
  [189000] = " Test ends in 3 seconds...",
  [191000] = " Test ends in 2 seconds...",
  [192000] = " Test ends in 1 second.",
  [194000] = " Time's up. Please try again.",
}
local room_events = { "OnKilled", "OnWrong", "OnTimer195000" }
for ms in pairs(CLOCK) do room_events[#room_events + 1] = "OnTimer" .. ms end

npc {
  map = "-", name = "hunter_room", sprite = -1, events = room_events,
  src = "eathena-1.0:npc/jobs/2-1/hunter.txt:704",
  on_click = function(label)
    if label == "OnKilled" then
      if not running then return end
      tmp.hntr_kills = (tmp.hntr_kills or 0) + 1
      if tmp.hntr_kills < 4 then
        return map_announce(MAP, "Okay~ You're almost there!!", 0)
      end
      -- the walkthrough: "Once you have killed all 4 Job Change
      -- Monsters, all the monsters will die"
      kill_monster(MAP, TARGET_EV)
      kill_monster(MAP, DECOY_EV)
      enable_npc("hunter_switch")
      return map_announce(MAP, "Good job... Now find the switch in the center of the map!! Watch out for the traps!!", 0)
    end
    if label == "OnWrong" then
      if not running then return end
      return fail(player.name .. "!! You made a mistake... Please try again.")
    end
    if label == "OnTimer195000" then
      area_warp(MAP, ARENA[1], ARENA[2], ARENA[3], ARENA[4], MAP, ROOM[1], ROOM[2])
      return reset()
    end
    local ms = tonumber(label:match("OnTimer(%d+)"))
    if ms and CLOCK[ms] then map_announce(MAP, CLOCK[ms], 0) end
  end,
}

-- the switch, once the four have fallen (rAthena's three buttons; the
-- walkthrough: "it will give you some options, choose 'escape'")
npc {
  map = MAP, x = 93, y = 101, dir = 1, sprite = 723, name = "Switch", id = "hunter_switch", touch = { 1, 1 },
  events = { "OnInit" }, src = "eathena-1.0:npc/jobs/2-1/hunter.txt:838",
  on_click = function(label)
    if label == "OnInit" then return disable_npc("hunter_switch") end
    mes "^3355FFThere are 3 buttons"
    mes "on the escape switch.^000000"
    next()
    local c = menu("Escape", "Cancel", "Re-test")
    if c == 1 then
      mes "^3355FFThe escape portal"
      mes "has been activated.^000000"
      close()
      map_announce(MAP, " !! Escape portal activated. !! ", 0)
      return enable_npc("hunter_exit")
    end
    if c == 2 then
      mes "^3355FFCancelled.^000000"
      return close()
    end
    mes "^3355FFYou will soon be returned"
    mes "to the waiting room.^000000"
    close()
    fail(" !! Cancellation warp activating... !! ")
  end,
}

npc {
  map = MAP, x = 89, y = 139, dir = 0, sprite = 45, name = "exit", id = "hunter_exit", touch = { 2, 2 },
  events = { "OnInit" }, src = "eathena-1.0:npc/jobs/2-1/hunter.txt:847",
  on_click = function(label)
    if label == "OnInit" then return disable_npc("hunter_exit") end
    reset()
    if player.class == Job_Archer and v.hntr_q == 2 then v.hntr_q2 = 2 end
    save_point("payon", 104, 99)
    warp(GUILD_ROOM[1], GUILD_ROOM[2], GUILD_ROOM[3])
  end,
}

local TRAP_TEXT = {
  "%s, you have fallen into a trap. You will be returned to the starting point.",
  "%s, you have blundered into a trap. I'm sorry, but for now, YOU LOSE.",
  "%s, what are you doing!? Go back and do it again!",
  "%s, come on! You can do better than this!! Try again!",
}

for i, t in ipairs(TRAPS) do
  npc {
    map = MAP, x = t[1], y = t[2], dir = 0, sprite = -1, name = "trap", id = "hunter_trap_" .. i,
    touch = { t[3], t[4] }, src = "eathena-1.0:npc/jobs/2-1/hunter.txt:859",
    on_click = function()
      if not running then return end
      fail(TRAP_TEXT[rand(1, #TRAP_TEXT)]:format(player.name))
    end,
  }
end

npc {
  map = MAP, x = 178, y = 32, dir = 1, sprite = 107, name = "Guide",
  src = "eathena-1.0:npc/jobs/2-1/hunter.txt:657",
  on_click = function()
    local G = "[Guide]"
    if player.class ~= Job_Archer or v.hntr_q ~= 2 or v.hntr_q2 ~= 1 then
      page(G, "You shouldn't be here.", "I'll send you back to Payon.")
      close()
      return warp(GUILD_ROOM[1], GUILD_ROOM[2], GUILD_ROOM[3])
    end
    if tmp.hntr_lounge ~= 1 then
      page(G, "Welcome to the Hunter testing", "site. The test begins when you", "enter the next room.")
      next()
      page(G, "Hunt 4 of the monsters named", "^3355FFJob Change Monster^000000. Then the", "switch in the centre of the room", "will work.")
      next()
      page(G, "The switch opens the exit at", "12 o'clock. Do everything within", "the time given, and get out.")
      next()
      page(G, "Faint, fall into a trap, or run", "out of time, and you fail. Then", "you must take the test again.")
      next()
      page(G, "...There is nobody in front of you", "today. Shall I let you in?")
    else
      percent_heal(100, 100)
      page(G, "Rested? Nobody is in front of you.", "Shall I let you in?")
    end
    next()
    local c = menu("I am ready.", "I'd like to lounge a bit first.", "I give up for now.")
    if c == 2 then
      tmp.hntr_lounge = 1
      page(G, "Take your time. Tell me when you", "are ready.")
      return close()
    end
    if c == 3 then
      tmp.hntr_lounge = 0
      page(G, "Very well. I'll send you to Payon.", "Hope to see you next time. Don't", "forget to save when you leave.")
      close()
      return warp(GUILD_ROOM[1], GUILD_ROOM[2], GUILD_ROOM[3])
    end
    tmp.hntr_lounge = 1
    save_point(MAP, ROOM[1], ROOM[2])
    close()
    start()
  end,
}

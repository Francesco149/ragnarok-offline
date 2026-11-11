-- The Knight test: seven knights of the Prontera Chivalry, an errand, two
-- quizzes, a three-stage fight and a test of restraint on job_knt, and an
-- interview, then the captain's verdict.
--
-- The map: job_knt is in 2003-04-30gdata_turtle_ph_2.gpf, the second Turtle
-- Island patch file on the CD, which euRO applied in January 2004 with the
-- other 2-1 test maps (research/THREADS.md #8). Every cell below is walkable
-- on it (research/gat_cells.py on the merged GRF).
--
-- Sources:
--   * jAthena //1046 script/npc/job/npc_job_07knight.txt (2004-09-12, in
--     data:refs/jathena, commit 6332410; Japanese): the flow, the NPCs and
--     their cells, the one quest variable, the items, the questions and
--     which answers count, the arena's spawns, its three minutes a stage, the
--     restraint room and its 270 seconds, the potions. Tier athena2003, the
--     nearest copy of the official script; its header calls it unfinished
--     (未完成) and it marks some lines as its author's guesses (適当).
--     Dialogue here is our English of its Japanese, cut to the window.
--   * eAthena 1.0 npc/jobs/2-1/knight.txt (2004-11-04, "converted from aegis
--     script, and my own RO screenies and play experience"; data:refs/rathena
--     commit 706e78509): English names (Sir Andrew, Sir James, Lady Amy) and
--     English wording where it asks what jAthena asks; the plant cells of the
--     restraint room (jAthena's are walls on this map).
--   * No account dates the Knight test before September 2004 (THREADS.md #8;
--     pRO in February 2004 had none, queue T1). It is here because its map
--     came with the January 2004 patch that "added 2nd job quest", as the
--     Wizard's, Priest's and Hunter's did, and 817's own Knight NPC says
--     "another requirements would be added soon". An approximation to
--     sharpen when a period account turns up.
--
-- Where jAthena is unfinished, what we did (docs/DEVIATIONS.md):
--   * one item list (jAthena names three patterns and fills one; eAthena's
--     second list is its translator's addition, by his changelog);
--   * Sir Grey always passes, as jAthena's code does (its comment: "pass
--     condition unknown");
--   * the examiners' lines while another test runs ("未調査", not researched)
--     are short pointers to the next knight;
--   * the verdict speaks of "this one", not "she" (jAthena's text is written
--     for a female candidate);
--   * no waiting room: Sir Windsor asks whether you are ready, or lets you
--     lounge first (as in the Wizard and Hunter tests);
--   * the four warp tiles mod0631 puts on job_knt ("to 0,0",
--     npc_warp25.txt:455-458) are the ends of the arena's rooms: removed;
--   * a death in the arena or the restraint room is not handled by jAthena
--     either: you wake at your save point and ask again. One who falls in the
--     restraint room does not pass when its timer runs out elsewhere
--     (jAthena's timer is the player's and would pass them).

remove_warp("job_knt", 165, 151)
remove_warp("job_knt", 122, 151)
remove_warp("job_knt", 65, 145)
remove_warp("job_knt", 22, 145)

local MAP = "job_knt"
local SRC = "jathena-1046:script/npc/job/npc_job_07knight.txt:"
local AWAKENING = 656

-- v.change_knight (jAthena's variable):
--   1 applied, 2 sent for the items, 5 items brought, 6 James failed,
--   7 James passed, 9 the arena passed, 10 Amy failed (or begun),
--   11 Amy passed, 12 restraint failed (or begun), 13 restraint passed,
--   14 Grey begun, 15 all done: the captain's verdict

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function swordman() return player.class == Job_Swordman end
local function q() return v.change_knight end

-- ------------------------------------------------------------ Captain Herman

local H = "[Captain Herman]"

local function verdict()
  if player.skill_point > 0 then
    page(H, "Use up your skill points first,", "then come back to me.")
    return close()
  end
  page(H, "So, you have been through all", "of the tests. But not everyone", "who finishes them becomes", "a knight.")
  next()
  page(H, "How earnest you were, how", "honourable, how strong; whether", "you were courteous, respectful,", "and humble...")
  next()
  page(H, "We watched you through it all.", "The seven of us decide whether", "you become a knight.")
  next()
  page(H, "Let us hear what everyone thinks.", "Shall this one become a knight?", "Shylock, your view!")
  next()
  page("[Sir Andrew]", "Even the tiresome things I asked", "for were brought, every one.", "I am in favour. This one will", "live as earnestly as a knight.")
  next()
  page(H, "A fine report. Siracuse?")
  next()
  page("[Sir James]", "Well, all right. Not quite to my", "taste yet, but this one will win", "the order some fame.")
  next()
  page(H, "Benedict, and you?")
  next()
  page("[Sir Windsor]", "...")
  next()
  page("[Sir Windsor]", "In favour...")
  next()
  page(H, "No complaint there, it seems.", "Beatrice, let us hear you.")
  next()
  page("[Lady Amy]", "Yes, good! Polite enough, and", "cute too, so a few more points...", "Oops, I shouldn't say that, right?")
  next()
  page(H, "An odd report, but a yes, no", "doubt about it. Grost, a word.")
  next()
  page("[Sir Edmond]", "Rough still, but there is good in", "this one. One day the hidden gem", "will show us its light.")
  next()
  page(H, "And last, Prospero.")
  next()
  page("[Sir Grey]", "Nothing more to say from me.", "Wishing bravely to be a knight", "is enough.")
  next()
  page(H, "All in favour. No one against.", "Then last of all, my own view.")
  next()
  page(H, "...")
  next()
  page(H, "In favour.")
  next()
  page(H, "Whether every test was cleared", "perfectly is hard to say, but", "you have the virtues of a knight.")
  next()
  job_change(Job_Knight)
  v.change_knight = 0
  page(H, "You are one of us now, a knight", "of Prontera. Protect the weak,", "and seek a knight's honour.")
  next()
  give_item(AWAKENING, 7)
  page(H, "And all of us have a gift for", "you, to mark the day.")
  next()
  page(H, "Now go! Light upon your future,", "and upon the order's!")
  close()
end

npc {
  map = "prt_in", x = 88, y = 101, dir = 4, sprite = 56, name = "Captain Herman",
  replaces = "817:jobs/knight.txt:3", src = SRC .. "21",
  on_click = function()
    if player.class == Job_Knight then
      page(H, "Ah, a knight of the order. Do you", "remember what I told you? A", "knight of ours stands tall, even", "at death's door.")
      return close()
    end
    if player.class == Job_Novice then
      page(H, "This is the Prontera Chivalry.", "What brings you here?")
      local c = menu("I want to be a Swordman.", "I want to be a Knight.", "Just looking around.")
      if c == 1 then
        page(H, "Ha ha! You have come to the", "wrong door. This is the order", "of knights, not the Swordman", "Guild. That is in Izlude.")
      elseif c == 2 then
        page(H, "It is good to dream big. But to", "be a knight, you must first be", "a Swordman. Go to the Swordman", "Guild in Izlude, and come back", "when your arm has grown strong.")
      else
        page(H, "Looking is fine, but if you have", "time to spare, go and defeat", "a monster or two.")
      end
      next()
      page(H, "Come again when you need us.")
      return close()
    end
    if not swordman() then
      page(H, "Welcome to the Prontera Chivalry.", "Take your time.")
      return close()
    end
    if q() >= 1 and q() <= 14 then
      page(H, player.name .. ", is it? The tests", "may be hard, but do your best.")
      return close()
    end
    if q() == 15 then return verdict() end
    page(H, "This is the Prontera Chivalry.", "What brings you here?")
    if menu("I want to be a Knight.", "Just looking around.") == 2 then
      page(H, "Oh, a Swordman. You look as if", "you have trained a good deal...", "Have you thought of becoming", "a knight? If you have, come", "and see me.")
      return close()
    end
    page(H, "Oho, a youth who would be", "a knight! We knights of Prontera", "will look after you.")
    next()
    page(H, "I am Herman von Ephesus, captain", "of the Prontera Chivalry. Eager", "young people still make my heart", "beat faster.")
    next()
    page(H, "We test only Swordmen of", "^5533FFJob Level 40^000000 or more. We cannot", "make knights of the green.")
    next()
    page(H, "Well then, will you apply?")
    next()
    if menu("Yes, I apply.", "Let me think it over.") == 2 then
      page(H, "Ha ha, too sudden? Think it over", "at your leisure, and come back", "when you are ready.")
      return close()
    end
    if player.job_level < 40 then
      page(H, "Ha ha, too early for you yet!", "I said Job Level 40 or more.", "I like your spirit, but no. Fight", "more monsters and come back.", "I will be waiting.")
      return close()
    end
    if player.skill_point > 0 then
      page(H, "Use up your skill points first,", "then come back to me.")
      return close()
    end
    page(H, "Good, your application is in.")
    next()
    page(H, "Go to ^5533FFSir Andrew^000000. The first", "test is his.")
    v.change_knight = 1
    close()
  end,
}

-- ------------------------------------------------------------ Sir Andrew: the errand

local A = "[Sir Andrew]"
-- jAthena's pattern 1, five of each
local ITEMS = { 1042, 950, 1032, 966, 7031, 946 }

local function say_items()
  mes(A)
  for _, id in ipairs(ITEMS) do mes("5 " .. item_name(id)) end
end

npc {
  map = "prt_in", x = 75, y = 107, dir = 4, sprite = 65, name = "Sir Andrew", src = SRC .. "293",
  on_click = function()
    if player.class == Job_Knight then
      page(A, "A knight of the order. How are", "things? Good habits keep you fed", "and equipped: pick up every last", "Jellopy from each fight.")
      next()
      page(A, "Not that we should be greedy.", "We are knights, after all.")
      return close()
    end
    if player.class == Job_Novice then
      page(A, "A Novice. Welcome to the Prontera", "Chivalry. You feel weak now, but", "work, and you will grow strong.")
      next()
      page(A, "Dream of a bright future, and", "walk your own road quietly.")
      return close()
    end
    if not swordman() or q() < 1 then
      page(A, "We knights of Prontera work for", "peace on this continent. Through", "battle after battle, we keep a", "bright future in sight.")
      return close()
    end
    if q() == 1 then
      page(A, "Very well, the first test begins.", "Gather these items for me:")
      next()
      say_items()
      next()
      page(A, "Off you go, then.")
      v.change_knight = 2
      return close()
    end
    if q() >= 2 and q() <= 4 then
      page(A, "Welcome back. Have you gathered", "the items? Let me see.")
      next()
      for _, id in ipairs(ITEMS) do
        if count_item(id) < 5 then
          page(A, "Wait a moment. These are not", "all here.")
          next()
          page(A, "Did you think you could be a", "knight without proper work? If", "you have forgotten what I asked", "for, I will say it again.")
          next()
          say_items()
          next()
          page(A, "Once more: take the test", "seriously. Now go.")
          return close()
        end
      end
      for _, id in ipairs(ITEMS) do take_item(id, 5) end
      page(A, "Perfect. Well done. These will", "do a great deal for the order's", "purse.")
      next()
      page(A, "Now go and see my fellow knight", "^5533FFSir James^000000. Take the rest of", "the tests as earnestly as you", "have lived so far.")
      v.change_knight = 5
      return close()
    end
    if q() == 5 then
      page(A, "Go and see Sir James.")
      return close()
    end
    if q() == 15 then
      page(A, "You have finished every test.", "Well done. Go and hear what the", "captain and the rest of us think.")
      return close()
    end
    page(A, "Something to tell me? If you have", "time for that, go and take the", "next test. Once more: take the", "tests seriously.")
    close()
  end,
}

-- ------------------------------------------------------------ Sir James: what a knight knows

local J = "[Sir James]"

-- jAthena's eight questions; every one must be right
local JAMES = {
  { { "Which of these weapons cannot", "be used with Two-Hand Quicken?" },
    { "Katana", "Slayer", "Broad Sword", "Flamberge" }, 4 },
  { { "Which skill is not needed to", "learn Bowling Bash?" },
    { "Two-Hand Sword Mastery Lv 5", "Magnum Break Lv 3", "Provoke Lv 10", "Bash Lv 10" }, 3 },
  { { "Which skill is not needed to", "learn Brandish Spear?" },
    { "Pierce Lv 5", "Spear Stab Lv 3", "Spear Boomerang Lv 3", "Riding Lv 1" }, 3 },
  { { "Which spear can hurt a Nightmare,", "a monster of the Ghost element?" },
    { "Zephyrus", "Lance", "Guisarme", "Crescent Scythe" }, 1 },
  { { "Riding a Peco Peco slows your", "attacks. With Cavalier Mastery", "at level 3, how fast do you", "attack, against on foot?" },
    { "70%", "80%", "90%", "100%" }, 2 },
  { { "A beginner in town asks you for", "advice. What do you do?" },
    { "Tell them of a good hunting ground.", "Take the monsters' blows for them.", "Give them plenty of zeny and items." }, 1 },
  { { "How should a knight act", "in a party?" },
    { "Go in front and protect everyone.", "Gather monsters for all to kill.", "Pick up the items, fight or not." }, 1 },
  { { "What matters most to a knight?" },
    { "Honour", "Money", "Rank" }, 1 },
}

local function james_quiz()
  for _, item in ipairs(JAMES) do
    page(J, table.unpack(item[1]))
    next()
    if menu(table.unpack(item[2])) ~= item[3] then
      page(J, "Wrong! You have a lot to learn.", "Think it over, and come back.")
      v.change_knight = 6
      return close()
    end
  end
  page(J, "Yes, a knight is honour! Live for", "it, die for it. Keep it in your", "heart always.")
  next()
  page(J, "Good, that is my test done.")
  next()
  page(J, "^5533FFSir Windsor^000000 is waiting for you.", "He is a man of few words, so", "mind yours.")
  v.change_knight = 7
  close()
end

npc {
  map = "prt_in", x = 71, y = 91, dir = 0, sprite = 65, name = "Sir James", src = SRC .. "905",
  on_click = function()
    if player.class == Job_Knight then
      page(J, "Hey, how are you! The order's", "the same as ever. Seen anyone", "dragging our honour through the", "mud? Then give them a telling-off", "for me.")
      return close()
    end
    if player.class == Job_Novice then
      page(J, "A Novice, here? Even if you like", "the look of knights, a Novice", "cannot become one. Train as a", "Swordman first.")
      return close()
    end
    if not swordman() or q() < 1 then
      page(J, "Attack and defence at once: with", "a two-handed sword you give up", "your shield. Is there no weapon", "without that weakness? Knights", "would be glad of one.")
      return close()
    end
    if q() >= 9 and q() <= 14 then
      page(J, "My test is behind you. Go on", "with the others'.")
      return close()
    end
    if q() == 15 then
      page(J, "Done with everyone's tests, eh?", "Then go and hear the captain.", "We will all be there.")
      return close()
    end
    mes(J)
    if q() == 6 then mes "You again. What is it?"
    elseif q() == 7 or q() == 8 then mes "Still something for me?"
    else mes "Hm, something for me?" end
    if menu("I want to take the Knight test.", "Nothing.") == 2 then
      page(J, "Hmph, don't be a stranger.")
      return close()
    end
    if q() >= 7 then
      page(J, "Oi, you have had my test already!", "Go to the others. I told you", "Sir Windsor is waiting.")
      return close()
    end
    if q() < 5 then
      page(J, "Ha, now now... you are in the", "middle of Sir Andrew's test,", "aren't you? Off you go!")
      return close()
    end
    if q() == 6 then
      page(J, "Right. I expect proper answers", "this time. Get one wrong again,", "and you'll be sorry!")
      next()
      page(J, "Here we go.")
      next()
      return james_quiz()
    end
    page(J, "Oh, finished the first test?", "All right, I'll give you a", "little of my time.")
    next()
    page(J, "I am James Siracuse, a knight of", "the Prontera Chivalry, naturally.")
    next()
    page(J, "My test is about how well you", "understand what a knight is, and", "what you think of a knight's", "honour.")
    next()
    page(J, "No need to be nervous. It won't", "take long: simple questions.", "Answer straight away, now.")
    next()
    james_quiz()
  end,
}

-- ------------------------------------------------------------ Sir Windsor: the arena on job_knt

local W = "[Sir Windsor]"
local WINDSOR = "knight_windsor"
local EXIT = { "prt_in", 80, 100 }
-- the three rooms: where the player goes in, and what is waiting
local STAGES = {
  { at = { 43, 147 }, area = { 22, 126, 65, 165 }, mobs = {
    { 39, 150, 1114 }, { 39, 142, 1114 }, { 47, 150, 1114 }, { 47, 142, 1114 }, { 35, 146, 1100 }, { 52, 146, 1100 },
    { 43, 137, 1160 }, { 43, 137, 1095 }, { 43, 137, 1105 }, { 43, 155, 1160 }, { 43, 155, 1095 }, { 43, 155, 1105 } } },
  { at = { 43, 52 }, area = { 22, 32, 65, 71 }, mobs = {
    { 34, 52, 1119 }, { 53, 52, 1119 }, { 29, 52, 1111 }, { 29, 52, 1111 }, { 58, 52, 1111 }, { 58, 52, 1111 },
    { 43, 42, 1106 }, { 43, 62, 1106 }, { 57, 64, 1030 }, { 57, 39, 1030 }, { 31, 64, 1030 }, { 31, 39, 1030 } } },
  { at = { 143, 152 }, area = { 122, 132, 165, 171 }, mobs = {
    { 136, 152, 1122 }, { 150, 152, 1123 }, { 143, 145, 1124 }, { 136, 158, 1280 }, { 130, 165, 1125 },
    { 156, 165, 1126 }, { 143, 167, 1258 } } },
}
local KILL_EV = { WINDSOR .. "::OnKill1", WINDSOR .. "::OnKill2", WINDSOR .. "::OnKill3" }

local function arena_reset()
  npc_timer_stop(WINDSOR)
  for _, ev in ipairs(KILL_EV) do kill_monster(MAP, ev) end
end

local function stage(n)
  local s = STAGES[n]
  tmp.knt_kills = 0
  for _, m in ipairs(s.mobs) do monster(MAP, m[1], m[2], "--ja--", m[3], 1, KILL_EV[n]) end
  npc_timer_start(WINDSOR) -- three minutes from now
end

npc {
  map = "prt_in", x = 79, y = 94, dir = 4, sprite = 733, name = "Sir Windsor", src = SRC .. "1253",
  on_click = function()
    page(W, "...")
    next()
    if player.class == Job_Knight or not swordman() or q() < 1 then
      page(W, player.class == Job_Novice and "Go and play outside..." or "Don't talk to me...")
      return close()
    end
    if q() >= 9 and q() <= 14 then
      page(W, "Want a fight, do you...")
      return close()
    end
    if q() == 15 then
      page(W, "To the captain...")
      return close()
    end
    page(W, q() == 8 and "...Again... What is it." or "What is it...")
    if menu("I want to take the Knight test.", "Nothing.") == 2 then
      page(W, "...")
      return close()
    end
    if q() ~= 7 and q() ~= 8 then
      page(W, "...")
      next()
      page(W, "Not my turn...")
      return close()
    end
    page(W, "...")
    next()
    page(W, "...Follow me.")
    close()
    warp("job_knt", 89, 101)
  end,
}

local function briefing()
  page(W, "...")
  next()
  page(W, "A fight with monsters...")
  next()
  page(W, "Defeat them all...")
  next()
  page(W, "...")
  next()
  page(W, "Finish all three stages...")
  next()
  page(W, "Three minutes a stage...")
  next()
  page(W, "...")
end

local arena_events = { "OnKill1", "OnKill2", "OnKill3", "OnTimer180000" }

npc {
  map = MAP, x = 89, y = 106, dir = 4, sprite = 733, name = "Sir Windsor", id = WINDSOR,
  events = arena_events, src = SRC .. "1650",
  on_click = function(label)
    if label == "OnTimer180000" then
      -- out of time: everyone out, everything put back (jAthena knttimer)
      for _, s in ipairs(STAGES) do
        local r = s.area
        area_warp(MAP, r[1], r[2], r[3], r[4], "prt_in", 80, 100)
      end
      return arena_reset()
    end
    local n = label and tonumber(label:match("OnKill(%d)"))
    if n then
      tmp.knt_kills = (tmp.knt_kills or 0) + 1
      if tmp.knt_kills < #STAGES[n].mobs then return end
      npc_timer_stop(WINDSOR)
      page(W, "...")
      next()
      if n == 3 then
        page(W, "The test is over...")
        next()
        page(W, "Leave here, and see", "^5533FFLady Amy^000000.")
        close()
        v.change_knight = 9
        arena_reset()
        return warp("prt_in", 80, 100)
      end
      page(W, n == 1 and "On to the next stage..." or "One stage more...")
      close()
      stage(n + 1)
      return warp(MAP, STAGES[n + 1].at[1], STAGES[n + 1].at[2])
    end
    if not swordman() or (q() ~= 7 and q() ~= 8) then
      page(W, "...")
      next()
      page(W, "You have no business here...")
      close()
      return warp("prt_in", 80, 100)
    end
    page(W, "...")
    next()
    if tmp.knt_lounge == 1 then
      percent_heal(100, 100)
      page(W, "...Ready?")
    else
      page(W, "Questions...?")
    end
    while true do
      local c = menu("I am ready.", "What kind of test is it?", "How does it go?", "I'd like to lounge a bit first.", "I want to leave.")
      if c == 1 then
        tmp.knt_lounge = 1
        page(W, "...")
        next()
        page(W, "Begin.")
        close()
        arena_reset()
        stage(1)
        return warp(MAP, 43, 147)
      elseif c == 2 then
        briefing()
        next()
      elseif c == 3 then
        page(W, "...")
        next()
        page(W, "Tell me when you are ready...", "It begins at once.")
        next()
        page(W, "One at a time... Nobody is in", "there now.")
        next()
      elseif c == 4 then
        tmp.knt_lounge = 1
        page(W, "...")
        return close()
      else
        page(W, "...")
        next()
        page(W, "...I see...")
        close()
        return warp("prt_in", 80, 100)
      end
    end
  end,
}

-- ------------------------------------------------------------ Lady Amy: courtesy

local AM = "[Lady Amy]"

-- jAthena's ten questions; each good answer is a point, eight to pass
local AMY = {
  { { "You're a knight in a Morroc street,", "looking for a party.", "How do you find one?" },
    { "Shout that you're looking.", "Open a chat room and wait.", "Look for people who want a knight." }, { 2, 3 } },
  { { "You team up with a Priest, a Wizard,", "a Hunter, an Assassin and a", "Blacksmith of your level, and the", "six of you reach the Pyramid's", "4th floor. What do you do?" },
    { "Look around and judge calmly.", "Gather monsters for the party.", "Go first, slowly." }, { 1, 3 } },
  { { "Someone drags in a crowd of", "monsters and then vanishes!", "What do you do?" },
    { "Shield the party, take the blows.", "Fight while falling back.", "Ride off fast on my Peco Peco." }, { 1, 2 } },
  { { "Everyone got away. Further on, a", "stranger lies collapsed, and asks", "very politely to be healed." },
    { "Tell the party's Priest.", "Offer to heal them for money.", "Ignore them and walk on." }, { 1 } },
  { { "You must leave for somewhere else.", "How do you share out what the", "fighting brought in?" },
    { "To whoever did the most.", "Play dumb and keep it.", "Talk it over and decide together." }, { 3 } },
  { { "You come to Prontera to sell", "something precious. The square is", "full of vendors and trade chats.", "How do you sell it?" },
    { "Walk around calling to people.", "Open a chat room and wait.", "Find someone who wants it." }, { 2, 3 } },
  { { "A beggar comes up to you." },
    { "Share my money and items.", "Ignore them and leave.", "Tell them where to hunt." }, { 3 } },
  { { "Alone in a forest maze, humming on", "your Peco Peco, you meet someone", "who is lost." },
    { "Tell them the way out.", "Lead them to the exit.", "Give them a Butterfly Wing." }, { 1, 2 } },
  { { "Surrounded, out of potions, your HP", "in the red, and a Priest passes.", "How do you ask?" },
    { "\"Could you spare me a Heal?\"", "\"A Heal, please.\"", "\"Heal.\"" }, { 1, 2 } },
  { { "On your way home you find a very", "expensive item on the ground." },
    { "Pick it up and keep it.", "Look for its owner.", "Just walk past." }, { 2 } },
}

local function amy_quiz()
  local score = 0
  for _, item in ipairs(AMY) do
    page(AM, table.unpack(item[1]))
    next()
    local c = menu(table.unpack(item[2]))
    for _, good in ipairs(item[3]) do
      if c == good then score = score + 1 end
    end
  end
  page(AM, "There, that's my test done!")
  next()
  if score < 8 then
    page(AM, "Hmm... no, it won't do! Make you a", "knight, and people would think", "badly of the whole order...")
    next()
    page(AM, "Go home and think about where you", "went wrong. Then I'll let you take", "the test again. All right?")
    return close()
  end
  page(AM, "Wonderful! With manners like that,", "you'll make a fine knight! The", "next test is... ^5533FFSir Edmond^000000's.")
  next()
  page(AM, "Listen well to what the knights", "say, and do your best with the", "rest of the tests!")
  v.change_knight = 11
  close()
end

npc {
  map = "prt_in", x = 69, y = 107, dir = 6, sprite = 728, name = "Lady Amy", src = SRC .. "438",
  on_click = function()
    if player.class == Job_Knight then
      page(AM, "Oh? What brings you to me? No", "trouble in a knight's life, I", "hope? Amy trusts you are doing", "fine.")
      next()
      page(AM, "Bye then, knight of the Prontera", "Chivalry!")
      return close()
    end
    if player.class == Job_Novice then
      page(AM, "Oh, what a cute Novice! So cute!", "Say, won't you be a knight one day?", "You'd look so cool!")
      next()
      page(AM, "I'll remember you... You have to", "become a knight! Promise!")
      return close()
    end
    if not swordman() then
      page(AM, "Welcome to the Prontera Chivalry!", "It's all knights here, but take", "your time~")
      return close()
    end
    if q() < 1 then
      page(AM, "Oh, a Swordman. Come to become", "a knight? First go and hear what", "the captain over there has to say.")
      return close()
    end
    if q() == 11 or q() == 12 then
      page(AM, "Go to Sir Edmond. Amy's test is", "over.")
      return close()
    end
    if q() == 13 or q() == 14 then
      page(AM, "Sir Grey is waiting for you.")
      return close()
    end
    if q() == 15 then
      page(AM, "Oh, it's time for everyone's", "verdict! Go and hear the captain.", "Don't be too nervous.")
      return close()
    end
    page(AM, "Hm? What do you want with Amy?")
    if menu("I want to take the Knight test.", "Nothing.") == 2 then
      page(AM, "Amy is bored...")
      return close()
    end
    if q() == 10 then
      page(AM, "So, have you studied your manners", "this time? Fail again and Amy", "will be cross! And I'll tell the", "captain not to make you a knight.")
      next()
      page(AM, "So listen well, and give a knight's", "answers! Here we go!")
      next()
      return amy_quiz()
    end
    if q() ~= 9 then
      page(AM, "Yes, you've applied! Then go and", "take the other knights' tests", "before mine! I'd love to see you", "from the start, but no!")
      return close()
    end
    page(AM, "Come closer! Now it's Amy's test.", "Tee hee.")
    next()
    page(AM, "I'm Amy Beatrice, the one rose", "among the Prontera Chivalry's", "thorns.")
    next()
    page(AM, "My test sees whether you have the", "courtesy a knight needs. Listen,", "and now and then answer me: I'll", "mark each answer.")
    next()
    page(AM, "Say what you think is right for", "a knight. Got it? Then let's", "begin.")
    next()
    v.change_knight = 10
    amy_quiz()
  end,
}

-- ------------------------------------------------------------ Sir Edmond: restraint

local E = "[Sir Edmond]"
local CALM = "knight_calm"
-- jAthena's twelve, the plants on eAthena 1.0's cells (jAthena's x 120 and 167 are walls here);
-- jAthena leaves out its four Chonchons ("they spawn elsewhere")
local CALM_MOBS = {
  { 165, 57, 1182 }, { 165, 54, 1182 }, { 122, 57, 1182 }, { 122, 54, 1182 },
  { 141, 57, 1002 }, { 145, 57, 1002 }, { 143, 55, 1002 }, { 143, 59, 1002 },
  { 141, 55, 1063 }, { 141, 59, 1063 }, { 145, 55, 1063 }, { 145, 59, 1063 },
}

local function to_calm_room()
  v.change_knight = 12
  add_timer(270000, CALM .. "::OnPass")
  kill_monster(MAP, CALM .. "::OnKill")
  for _, m in ipairs(CALM_MOBS) do monster(MAP, m[1], m[2], "--ja--", m[3], 1, CALM .. "::OnKill") end
  warp("job_knt", 143, 57)
end

npc {
  map = "-", name = CALM, sprite = -1, events = { "OnKill", "OnPass" }, src = SRC .. "1824",
  on_click = function(label)
    if label == "OnPass" then
      -- four and a half minutes without a kill. The timer is the player's: one who fell in the
      -- room and woke elsewhere has not passed (jAthena would pass them; ours, DEVIATIONS)
      kill_monster(MAP, CALM .. "::OnKill")
      if player.map ~= MAP then return end
      v.change_knight = 13
      return warp("prt_in", 80, 100)
    end
    del_timer(CALM .. "::OnPass")
    kill_monster(MAP, CALM .. "::OnKill")
    warp("prt_fild05", 353, 251)
  end,
}

npc {
  map = "prt_in", x = 70, y = 99, dir = 6, sprite = 734, name = "Sir Edmond", src = SRC .. "749",
  on_click = function()
    if player.class == Job_Knight then
      page(E, "Let your heart be as flowing", "water... Water brushes past the", "rocks and goes its way. A knight", "must flow as quietly, whatever", "comes.")
      return close()
    end
    if player.class == Job_Novice then
      page(E, "A tree with deep roots does not", "yield to the wind... Strength built", "from the ground up bears fruit one", "day. Your future is decided now...")
      return close()
    end
    if not swordman() then
      page(E, "All things in the world stand in", "harmony. Live without going", "against it.")
      return close()
    end
    if q() < 1 then
      page(E, "Who thinks dark thoughts dreams", "only dark dreams... and who dreams", "them long enough stops dreaming", "at all...")
      return close()
    end
    if q() == 13 or q() == 14 then
      page(E, "I have seen your heart. Go to", "^5533FFSir Grey^000000 for the last test...")
      return close()
    end
    if q() == 15 then
      page(E, "Hear the captain... You will hear", "what we all think then...")
      return close()
    end
    page(E, "What is it, Swordman...")
    if menu("I want to take the Knight test.", "Nothing.") == 2 then
      page(E, "What you wish for will open", "before your eyes.")
      return close()
    end
    if q() == 12 then
      page(E, "Last time you acted rashly. A", "knight protects the weak... he", "does not torment them...")
      next()
      page(E, "Where all things are in harmony,", "what if man alone goes on", "destroying... This is not only", "the test. It is the world.")
      next()
      page(E, "Once more, then. Show me a heart", "that waits in quiet.")
      close()
      return to_calm_room()
    end
    if q() ~= 11 then
      page(E, "A flower: the shoot grows, the", "stem rises, the leaves come, and", "then it blooms. Otherwise it is", "a flower unfinished...")
      next()
      page(E, "Others wait for you before me.", "You will find your way.")
      return close()
    end
    page(E, "My test now... Judge as wisely as", "in the tests before.")
    next()
    page(E, "I am Edmond Grost, of the Prontera", "Chivalry.")
    next()
    page(E, "A knight must be an example to", "others: reverent always, and fit", "to keep the order of the world.")
    next()
    page(E, "Never judge on a whim nor act on", "an impulse. Sometimes the heart", "must bend like water, and", "sometimes stand firm as rock.")
    next()
    page(E, "Kill no monster without reason;", "to enjoy the killing is unthinkable.", "Meditate. Set your heart in order.")
    next()
    page(E, "The test begins. Face it with a", "reverent heart.")
    close()
    to_calm_room()
  end,
}

-- ------------------------------------------------------------ Sir Grey: the interview

local G = "[Sir Grey]"

local function lecture()
  page(G, "Of course money and fame are fine", "things. But our knights live for", "something else.")
  next()
  page(G, "Hm, you think about things. Some", "want to be knights without a", "thought in their heads... and then", "they make trouble and stain the", "order's honour.")
  next()
  page(G, "This goes for you as well: once", "a knight, you can never go back", "to being a Swordman. Never forget", "what being a knight asks of you.")
  next()
end

local function hm() page(G, "Hm, hm.") next() end

local function grey_interview()
  page(G, "First... why did you decide to", "become a knight?")
  local c = menu("To become stronger.", "To be of use to my guild.", "I am not content as I am.")
  if c == 2 then
    page(G, "Working with a guild, or a party...", "Our king, Tristan III, said it", "too:")
    next()
    page(G, "behind the peace of every day,", "there is always a threat, so we", "must be ready for every danger.")
    next()
    page(G, "Then how will the strength you", "gain help your guild?")
    c = menu("My guild needs my strength.", "It helps fill the guild's coffers.", "I can protect my guildmates.")
    if c == 2 then lecture() else hm() end
  else
    if c == 1 then
      page(G, "To become stronger...? A knight is", "strong, true. But what is that", "strength? Something to show off,", "to become famous, or something", "else...")
    else
      page(G, "Hm, hm.")
    end
    next()
    page(G, "What is a knight's \"strength\"?")
    c = menu("Glory and fame.", "Something to protect myself.", "Something to protect others.")
    if c == 1 then lecture() else hm() end
  end
  page(G, "If you became a knight right now,", "what would you do?")
  c = menu("Go into battle at once.", "Go to the people waiting for me.", "Learn more about knights.")
  if c == 2 then
    page(G, "Who is waiting?")
    menu("My friends.", "My guildmates.", "My sweetheart.")
    page(G, "Comrades in life and death, then.", "Cherish them, and protect them.")
  else
    page(G, "Hm, and then?")
    menu("I will grow quickly.", "Try out a knight's skills.", "Go where Swordmen cannot.")
    page(G, "Don't be in such a hurry. You will", "not grow all at once. Make your", "groundwork solid.")
  end
  next()
  page(G, "I enjoyed our talk, though a", "thing or two gave me pause.")
  next()
  page(G, "Now go and hear the captain, and", "let the others judge you too.")
  next()
  page(G, "Don't worry too much. I'll put", "in a good word.")
  v.change_knight = 15
  close()
end

-- for Knights: a Claymore made to order (jAthena and eAthena 1.0 alike)
local function claymore()
  page(G, "A blade of a colour words cannot", "tell... The weapon for a knight:", "the ^3355FFClaymore^000000! Every knight", "should have one.")
  local c = menu("About the Claymore", "Buy a Claymore", "Never mind")
  if c == 3 then
    page(G, "A true knight wields a Claymore", "like a part of his own body.", "I hope you'll come back one day.")
    return close()
  end
  if c == 1 then
    page(G, "Of all the famous swords one can", "find in Prontera, the Claymore is", "said to stand out. Made for a", "knight's hand!")
    next()
    page(G, "Our order makes it for knights", "alone, for ^3355FF74,000^000000 zeny.")
    next()
    page(G, "Its tough, keen blade needs one", "precious ^3355FFSteel^000000. Bring the fee and", "the Steel, and I'll make it at", "once, on the order's honour.")
    return close()
  end
  if count_item(999) < 1 or player.zeny < 74000 then
    page(G, "I see you want one, but I can't", "make it until you're ready.", "^3355FF74,000 zeny! One Steel!^000000", "Come back when you have them.")
    return close()
  end
  if not check_weight(1163, 1) then
    page(G, "You look loaded to the brim.")
    return close()
  end
  page(G, "Oh! You've brought it all. Someone", "who knows a Claymore's worth! Right,", "I'll make it now!")
  next()
  page(G, "Heat the blade, melt in the Steel...", "^3355FF- Clang, clang, clang -^000000")
  next()
  take_item(999, 1)
  player.zeny = player.zeny - 74000
  give_item(1163, 1)
  page(G, "There, the order's pride, a", "Claymore! A trusty partner for", "your travels!")
  close()
end

npc {
  map = "prt_in", x = 87, y = 92, dir = 4, sprite = 119, name = "Sir Grey", src = SRC .. "1339",
  on_click = function()
    if player.class == Job_Knight then return claymore() end
    if player.class == Job_Novice then
      page(G, "I was a Novice once too, believe", "it or not. I never meant to be a", "knight; I only wanted to be", "strong. Before I knew it, I was", "a knight. Ha ha.")
      return close()
    end
    if not swordman() or q() < 1 then
      page(G, "Make good use of your time, young", "one. Regret comes too late once", "the time has gone.")
      return close()
    end
    if q() == 15 then
      page(G, "Go and hear the captain. The", "others will think it over well.", "Hurry along.")
      return close()
    end
    if q() == 14 then page(G, "Ah, you. What is it?") else page(G, "Something I can do for you,", "young Swordman?") end
    if menu("I want to take the Knight test.", "Nothing.") == 2 then
      page(G, "Look after yourself. Health", "comes first.")
      return close()
    end
    if q() == 14 then
      page(G, "So... you have been thinking", "things over. Then let us talk", "again.")
      next()
      return grey_interview()
    end
    if q() ~= 13 then
      page(G, "Ha, there are much younger knights", "than me here. Talk to them first.", "When they are done with you,", "I'll look after you.")
      return close()
    end
    page(G, "Oh, so you have taken all the", "others' tests.")
    next()
    page(G, "Then let's begin mine. Not that", "it's much of a test. Let's just", "talk, easy now.")
    next()
    v.change_knight = 14
    grey_interview()
  end,
}

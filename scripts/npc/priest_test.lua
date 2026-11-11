-- The Priest test: a pilgrimage, an exorcism on job_prist, and a vow.
--
-- Source: jAthena's script/npc/job/npc_job_priest.txt and npc_job_aco.txt
-- of 2004-06-08 (commit 0f45810, //1013), a transcription of jRO's test
-- with its author's notes on where the official server differed. Tier
-- later: four months after our period. It is used all the same because
--  * the test came with "added 2nd job quest" (euRO, New Comodo, January
--    2004), and its map, job_prist, is in the CD's Turtle Island patch
--    file, which euRO applied that same month (research/THREADS.md #8);
--  * the only earlier script, 817's (npc/jobs/priest.txt), is its author's
--    own invention ("Designed For Athena": ten true-or-false questions
--    and a grail carried past Wraiths on the older map job_priest). The
--    game played that until 2026-11-11; its NPCs are removed below.
-- Every cell jAthena names is walkable on this client's job_prist, and
-- mod0631's two warp tiles on the map (conf/npc_warp25.txt:452-453) are
-- the test's two gates.
--
-- What it is: Bishop Thomas (prt_church 16,41) sends an Acolyte of job
-- level 40 to the three ascetics of the Acolyte test, in order (job level
-- 50 is let off); then to Father Peter on job_prist, where thirteen
-- Zombies named for failings rise as the candidate walks a corridor and
-- must all fall before the gate opens (350 seconds), four demons tempt
-- him to give the test up (to give in is to be thrown out, somewhere
-- unpleasant), and Baphomet's parting gift waits in a last room; then
-- Sister Cecil (prt_church 27,24) takes the vow. The Bishop changes the
-- job and gives a Book.
--
-- Ours (docs/DEVIATIONS.md):
--  * The English: a retelling of jAthena's Japanese, cut to the client's
--    window (the name and six lines of about 30 characters). The ascetics'
--    and the Bishop's names are 817's.
--  * One player: a Priest wearing a Rosary could come along to help, and
--    Father Peter made a second candidate wait; both are left out.
--  * The last room. jAthena passes the candidate at the demons' gate,
--    sends him into the Mummies' room and resets the counter the Mummies
--    wait for, so none appear, and the room's exit (mod0631 and jAthena:
--    a warp tile at 97,104 "to 0,0") is commented out: one left by
--    Teleport. Here the Mummies rise (Baphomet promises "monsters I have
--    prepared for you"), and the tile leads to the church door in Prontera,
--    where the script sends everyone who leaves.
--  * A demon speaks once per attempt (jAthena: each time its cells are
--    stepped on); the clock that throws a slow candidate out does nothing
--    once he has left the map; Yosuke saves you where 817 does (jAthena's
--    cell is a rock on this map).
--  * job_prist has the flags 817 gave its own test map (no Teleport, no
--    memo, no Dead Branch; logging out puts you at the church door):
--    tools/import/athena_import.py.

local MAP = "job_prist"
local ACOLYTE, PRIEST, NOVICE = 4, 8, 0
local BOOK = 1550
local TIME_MS = 350000
local OUT = { "prontera", 234, 318 } -- the church door

-- 817's own Priest test
for _, src in ipairs { "817:jobs/priest.txt:407", "817:jobs/priest.txt:440", "817:jobs/priest.txt:447", "817:jobs/priest.txt:454",
  "817:jobs/priest.txt:461", "817:jobs/priest.txt:472", "817:jobs/priest.txt:483", "817:jobs/priest.txt:495",
  "817:jobs/priest.txt:506", "817:jobs/priest.txt:517" } do
  remove_npc(src)
end
-- the two gates are NPCs here
remove_warp(MAP, 24, 109)
remove_warp(MAP, 97, 104)

-- one page: the name and up to six lines of about 30 characters
local function say(who, ...)
  mes("[" .. who .. "]")
  for _, line in ipairs { ... } do mes(line) end
end

-- job2nd_pri (jAthena's variable): 1-3 on the pilgrimage (the next ascetic
-- is the first, second, third), 4 back from it, 5 sent to the exorcism,
-- 6 begun or failed it, 7 through it, 8 began the vow, 9 took it.

-- ------------------------------------------------------------------ Bishop
local THOMAS = "Bishop Thomas"

local function pilgrimage_route()
  say(THOMAS, "First see Father Rubalkabara,", "who trains at the ruins", "north-east of Prontera.")
  next()
  say(THOMAS, "Then Mother Mathilda. She is", "somewhere in the fields near", "Morroc, south-west of here.")
  next()
  say(THOMAS, "Last, Father Yosuke, in the", "fields north-west of", "Prontera.")
  next()
  say(THOMAS, "Take care. If you cannot find", "them, ask Sister Cecil by the", "door. When you are back I", "shall give you the next trial.")
  next()
  say(THOMAS, "God's blessing be with you...")
  close()
end

local function to_the_exorcism()
  warp("job_prist", 24, 180) -- a literal: ragnarok-offline check reads destinations so
end

npc {
  map = "prt_church", x = 16, y = 41, dir = 4, sprite = 60, name = "Bishop",
  replaces = "817:jobs/priest.txt:14",
  src = "jathena-1013:script/npc/job/npc_job_priest.txt:18",
  on_click = function()
    local class, q = player.class, v.job2nd_pri
    if class == NOVICE then
      say(THOMAS, "God's blessing be with you.", "This is Prontera Cathedral.", "What brings you here?")
      next()
      local c = menu("I want to become an Acolyte.", "I want to become a Priest.", "I am looking around.")
      if c == 1 then
        say(THOMAS, "Ho ho... an Acolyte? For that,", "go to the room on the other", "side.")
      elseif c == 2 then
        say(THOMAS, "Ho ho... become an Acolyte", "first. For that, go to the", "room on the other side.")
      else
        say(THOMAS, "Make yourself at home. No", "place in this world is safer", "than the cathedral.")
      end
      next()
      say(THOMAS, "God's blessing be with you.")
      return close()
    end
    if class == PRIEST then
      say(THOMAS, "God's blessing on a brother", "strong in faith...")
      next()
      say(THOMAS, "We Priests are here to speak", "for God. Hold out your hand", "to those in need.")
      return close()
    end
    if class ~= ACOLYTE then
      say(THOMAS, "God's blessing be with you.", "This is Prontera Cathedral.", "What brings you here?")
      next()
      if menu("Tell me about Priests.", "I am looking around.") ~= 1 then
        say(THOMAS, "Make yourself at home. No", "place in this world is safer", "than the cathedral.")
        return close()
      end
      say(THOMAS, "A Priest carries God's will", "into this world. Only an", "Acolyte who has trained long", "is given that right...")
      next()
      say(THOMAS, "An Acolyte of job level 40", "may take the trials. A Priest", "is stronger against demons", "and the undead, and has more", "skills to help others.")
      next()
      say(THOMAS, "To serve and to help is our", "calling. Ask no reward for", "it, and be thankful when you", "are the one helped.")
      return close()
    end

    if q == 9 then
      say(THOMAS, "Congratulations. You have", "come through every trial. I", "grant you the right to be a", "Priest.")
      next()
      say(THOMAS, "Lord... let Your light fall", "on this one and grant Your", "strength... One who carries", "Your will is born here", "today...")
      next()
      job_change(PRIEST)
      say(THOMAS, "You are born again as a", "Priest. Lead those in need", "with a new heart, and save", "them.")
      next()
      give_item(BOOK, 1)
      say(THOMAS, "And take this, to remember", "the day. It speaks of faith:", "you will need it on the road", "ahead.")
      next()
      say(THOMAS, "This ends the rite. You have", "done well... Walk your new", "road.")
      return close()
    elseif q >= 1 and q <= 3 then
      say(THOMAS, "Oh? Why are you here? You", "have not forgotten your", "pilgrimage, surely?")
      next()
      if menu("Forgive me, tell me the order again.", "No! Of course not!") == 1 then
        say(THOMAS, "Well, well... a start like", "this makes me worry. Listen", "well this time.")
        next()
        return pilgrimage_route()
      end
      say(THOMAS, "I see. Sister Cecil by the", "door can tell you where each", "of them is.")
      next()
      say(THOMAS, "Go, and take care. God's", "blessing be with you.")
      return close()
    elseif q == 7 then
      say(THOMAS, "You are through the exorcism.", "Well done. One who withstands", "every temptation has the", "makings of a Priest.")
      next()
      say(THOMAS, "Now go to Sister Cecil and", "take the vow of service.", "Answer the voice you hear as", "your heart tells you.")
      return close()
    elseif q == 8 then
      say(THOMAS, "Why... you did not take the", "vow to its end. Without a", "Priest's heart, evil will", "swallow you one day...")
      next()
      say(THOMAS, "Go back to Sister Cecil and", "finish it. Listen with your", "heart, not with your ears...")
      return close()
    elseif q == 6 then
      say(THOMAS, "It was hard, I see... But a", "Priest may meet harder. Do", "not lose to a trial like", "this. Will you try again?")
      next()
      if menu("I will.", "Let me prepare a little more.") ~= 1 then
        say(THOMAS, "Very well. Prepare yourself", "well. May you come through", "the trial safely.")
        return close()
      end
      say(THOMAS, "Then I send you. Ask Father", "Peter's advice as well.")
      next()
      return to_the_exorcism()
    elseif q == 5 then
      say(THOMAS, "You seem ready for the", "exorcism. Shall we begin?")
      next()
      if menu("Yes, please.", "I am not ready yet.") ~= 1 then
        say(THOMAS, "Very well. Prepare yourself", "well. May you come through", "the trial safely.")
        return close()
      end
    elseif q == 4 then
      say(THOMAS, "You have seen Father Yosuke.", "The first trial, the", "pilgrimage, is done. I", "congratulate you.")
      next()
      say(THOMAS, "Next is the exorcism: to", "drive out demons and the", "undead, and with them every", "temptation. Shall we begin?")
      next()
      if menu("Yes, please.", "Let me prepare first.") ~= 1 then
        v.job2nd_pri = 5
        say(THOMAS, "Very well. Take your time.", "May you come through the", "trial safely...")
        return close()
      end
    else
      say(THOMAS, "What brings you to me?")
      next()
      if menu("I have come to become a Priest.", "I came to greet you.") ~= 1 then
        say(THOMAS, "I am well, as you see. Do", "not forget an Acolyte's", "calling, and live rightly.")
        next()
        say(THOMAS, "God's blessing be with you...")
        return close()
      end
      say(THOMAS, "A Priest... God will rejoice", "at your resolve. I am Thomas", "Cervantes, bishop of this", "cathedral. Be at ease.")
      next()
      say(THOMAS, "There are trials to pass,", "and before them one thing:", "your job level must be 40 or", "more. Shall I write your", "application now?")
      next()
      if menu("Yes, I apply.", "Let me think it over.") ~= 1 then
        say(THOMAS, "Then think it over. Come to", "me when you have made up", "your mind.")
        return close()
      end
      if player.job_level < 40 then
        say(THOMAS, "Oh... your job level is not", "yet 40. Train some more and", "come again.")
        next()
        say(THOMAS, "God's blessing be with you...")
        return close()
      end
      v.job2nd_pri = 1
      say(THOMAS, player.name .. "...", "these are your trials. They", "are hard and will hurt.", "Come through them.")
      next()
      say(THOMAS, "First the pilgrimage: visit,", "in order, three who train in", "the fields.")
      next()
      say(THOMAS, "Then the exorcism: drive out", "demons and the undead, and", "with them temptation and", "every evil power.")
      next()
      say(THOMAS, "Last the vow of service: to", "help the weak and spread the", "faith as a Priest.")
      next()
      if player.job_level < 50 then
        say(THOMAS, "(An Acolyte who has trained", "to job level 50 need not", "make the pilgrimage: he has", "walked far enough.)")
        next()
        return pilgrimage_route()
      end
      say(THOMAS, "Your job level is 50. Well", "done: I see how hard you", "have trained. You need not", "make the pilgrimage.")
      next()
      say(THOMAS, "Go on to the exorcism. Shall", "we begin?")
      next()
      if menu("Yes, please.", "Let me prepare first.") ~= 1 then
        v.job2nd_pri = 5
        say(THOMAS, "Very well. Take your time.", "May you come through the", "trial safely...")
        return close()
      end
    end
    v.job2nd_pri = 5
    say(THOMAS, "Then I send you to the place", "of the trial. When you are", "there, hear Father Peter,", "who keeps it.")
    next()
    to_the_exorcism()
  end,
}

-- ------------------------------------------------------------ the ascetics
-- 817's three of the Acolyte test (npc/jobs/acolyte.txt), on the pilgrimage
-- of an Acolyte who is on his way to Priest; otherwise as they were.
local function ascetic(src, who, turn, save, first, come_back, done)
  wrap_npc(src, function(orig)
    return function(at)
      local q = v.job2nd_pri
      if at or player.class ~= ACOLYTE or q == 0 then return orig(at) end
      if q == turn then
        v.job2nd_pri = turn + 1
        save_point(save[1], save[2], save[3])
        for i, page in ipairs(first) do
          if i > 1 then next() end
          say(who, table.unpack(page))
        end
      elseif q < turn then
        say(who, table.unpack(come_back))
      elseif q <= 3 then
        say(who, "Go on to the next place of", "your pilgrimage. Take care", "on the road.")
      else
        say(who, table.unpack(done))
      end
      close()
    end
  end)
end

ascetic("817:jobs/acolyte.txt:175", "Father Rubalkabara", 1, { "prt_fild03", 365, 255 },
  { { "An Acolyte who would be a", "Priest... Bishop Thomas sent", "word. Your pilgrimage has", "only begun: do not give up." },
    { "Come again whenever you can.", "This is holy ground to us." },
    { "Now go on to the next place.", "Take care on the road." } },
  {}, { "Why are you here? You have", "your own training to do." })

ascetic("817:jobs/acolyte.txt:225", "Mother Mathilda", 2, { "moc_fild07", 41, 355 },
  { { "Ah, on the way to Priest? You", "have come far. We Priests", "must carry the faith to all", "the earth." },
    { "When you are a Priest, come", "to the Pyramid of Morroc.", "Many undead are there.", "Please set them free..." },
    { "Take care... God's blessing", "be with you..." } },
  { "A pilgrim? But there is one", "you must visit before me.", "Ask Bishop Thomas or Sister", "Cecil." },
  { "What brings you here? Go on", "with your training." })

ascetic("817:jobs/acolyte.txt:267", "Father Yosuke", 3, { "prt_fild00", 206, 230 },
  { { "Hm, on the way to Priest.", "You did well to find me." },
    { "Live rightly, and bring no", "shame on the name of Priest." },
    { "You may go back to the", "cathedral. This is the end", "of your pilgrimage. Take", "care on the way back." } },
  { "Hm, on the way to Priest. But", "see the others before you", "come to me. In short: you", "have the order wrong." },
  { "Hm. Your pilgrimage is over.", "The cathedral is waiting." })

-- ------------------------------------------------------------ Sister Cecil
local CECIL = "Sister Cecil"
-- the vow: question, the answer a Priest gives (1 yes, 2 no), her words on the other
local VOW = {
  { { "Can you give the rest of your", "life to God?" }, 1,
    { "Why such an answer... You are", "not yet ready in your heart", "to be a Priest. Look into", "yourself and come again." } },
  { { "Will you use your powers for", "yourself, to gain by them?" }, 2,
    { "No. Our powers are not for", "gain. Think again about what", "a Priest is: such a heart", "is only stained by greed." } },
  { { "Will you use your powers to", "help others who suffer in", "battle?" }, 1,
    { "That will not do. A Priest's", "powers are there to help,", "whoever asks. Look around", "you: you will see suffering." } },
  { { "Can you help others at a cost", "to yourself?" }, 1,
    { "...No. To help another at", "your own cost is a fine", "thing. Think about the word", "sacrifice." } },
  { { "Do you walk through town or", "the fields saying the same", "words over and over?" }, 2,
    { "No. And that goes for", "everyone. Nobody turns to", "listen to one who walks", "about shouting." } },
  { { "Do you help others by", "leading many monsters along", "behind you?" }, 2,
    { "That is very dangerous, to", "others as well as to you.", "Think again about what it", "is to help someone." } },
  { { "Can you hold to the way of", "faith, and die for it?" }, 1,
    { "With such a heart you cannot", "be a Priest. We can raise", "the dead: think what life", "and death mean to you." } },
}

npc {
  map = "prt_church", x = 27, y = 24, dir = 2, sprite = 79, name = "Nun",
  replaces = "817:cities/prontera.txt:115",
  src = "jathena-1013:script/npc/job/npc_job_priest.txt:560",
  on_click = function()
    local class, q = player.class, v.job2nd_pri
    if class ~= ACOLYTE then -- 817's Nun on this cell, to everyone else
      mes "[Nun]"
      mes "We ministers are austerely prohibited to use weapons with blades. The most important mission of us is not to annihilate monsters but to enlighten thier poor souls."
      return close()
    end
    if q >= 1 and q <= 3 then
      say(CECIL, "Your pilgrimage has begun.", "It is hard, but see it", "through. First is Father", "Rubalkabara.")
      next()
      say(CECIL, "He is at the mouth of the", "ruins north-east of Prontera.", "Many monkeys live there:", "beware of the fierce Chocos.")
      next()
      say(CECIL, "After him see Mother", "Mathilda, near Morroc, and", "then Father Yosuke, north-", "west of Prontera.")
      next()
      say(CECIL, "Go now, and do not give up. I", "shall pray for a safe road.")
      return close()
    elseif q == 4 then
      say(CECIL, "Welcome back from a long", "road. Now go to the Bishop:", "the next trial is waiting.")
      return close()
    elseif q == 5 or q == 6 then
      say(CECIL, "The exorcism? I cannot tell", "you much. Prepare well, and", "keep hold of yourself: do", "not give in to desire.")
      next()
      say(CECIL, "Father Peter, at the place", "of the trial, will tell you", "more. He and the Bishop are", "old friends.")
      return close()
    elseif q == 9 then
      say(CECIL, "Go to Bishop Thomas for the", "last rite. God's blessing be", "with you...")
      return close()
    elseif q ~= 7 and q ~= 8 then
      say(CECIL, "God's blessing be with you...", "What brings you here?")
      next()
      if menu("I want to become a Priest.", "Nothing in particular.") ~= 1 then
        say(CECIL, "Then rest a while. Come", "whenever you like, and tell", "me how the other Acolytes", "are.")
        return close()
      end
      say(CECIL, "A Priest! I am Cecil", "Margarita, a nun who helps", "Acolytes on that road.")
      next()
      say(CECIL, "There are trials to pass:", "the pilgrimage, the", "exorcism, the vow of", "service.")
      next()
      say(CECIL, "Go to Bishop Thomas, further", "in, to begin. And ask me if", "anything is hard: I shall", "help where I can.")
      return close()
    end

    if q == 7 then
      say(CECIL, "You have had a hard time! But", "you have come to the last", "step.")
      next()
      say(CECIL, "Now the rite of the vow.", "Answer my questions in", "earnest.")
    else
      say(CECIL, "......", "You have come back. This", "time, take the vow to its", "end... Answer in earnest.")
    end
    next()
    v.job2nd_pri = 8
    say(CECIL, "Let us begin.")
    next()
    for _, step in ipairs(VOW) do
      say(CECIL, table.unpack(step[1]))
      next()
      if menu("Yes.", "No.") ~= step[2] then
        say(CECIL, table.unpack(step[3]))
        return close()
      end
    end
    say(CECIL, "I have heard your vow. Last:", "do you swear to all you", "have said?")
    next()
    if menu("I swear.", "No.") ~= 1 then
      say(CECIL, "......", "Your heart is not yet ready.", "It is too soon for you to be", "a Priest.")
      next()
      say(CECIL, "Look into yourself, make up", "your mind, and come to me", "again.")
      return close()
    end
    v.job2nd_pri = 9
    say(CECIL, "This ends the rite of the", "vow. Go to Bishop Thomas for", "the last rite.")
    next()
    say(CECIL, "Never forget that we are", "with you as brothers and", "sisters... God's blessing be", "with you...")
    close()
  end,
}

-- ------------------------------------------------------------ the exorcism
-- tmp.jpri_step (jAthena's @jpri_checkwarp): 1-5 the Zombie groups still to
-- rise, 6-8 the Mummy pairs, 9 all risen.
local RISEN, MUMMIES = "jpri_undead::OnKilled", "jpri_mummy::OnKilled"
local ZOMBIE, MUMMY = 1015, 1041
-- trigger cell (16 cells to each side), then who rises where
local ZOMBIES = {
  { 51, { 24, 52, "Vice" }, { 18, 52, "Disloyalty" }, { 30, 51, "Envy" } },
  { 61, { 21, 62, "Wrath" }, { 27, 62, "Grudge" } },
  { 71, { 24, 72, "Pride" }, { 18, 72, "Lust" }, { 30, 72, "Sloth" } },
  { 81, { 21, 82, "Gluttony" }, { 27, 82, "Greed" } },
  { 91, { 24, 92, "Despair" }, { 18, 92, "Distrust" }, { 30, 92, "Fear" } },
}
local MUMMY_PAIRS = {
  { 54, { 90, 55, "Ahmose" }, { 105, 54, "Amasis" } },
  { 69, { 90, 70, "Menet" }, { 105, 70, "Ahmad" } },
  { 84, { 90, 85, "Menes" }, { 105, 85, "Nepes" } },
}

local function clear_the_place()
  kill_monster(MAP, RISEN)
  kill_monster(MAP, MUMMIES)
  del_timer("jpri_timer::OnWarp")
end

local PETER = "Father Peter"

npc {
  map = MAP, x = 24, y = 187, dir = 4, sprite = 110, name = "Tough Father",
  src = "jathena-1013:script/npc/job/npc_job_priest.txt:1018",
  on_click = function()
    if player.class ~= ACOLYTE then
      say(PETER, "God's blessing be with you.", "This trial is not yours.")
      next()
      return warp(OUT[1], OUT[2], OUT[3])
    end
    local c
    if v.job2nd_pri == 6 then
      say(PETER, "Better prepared this time?", "Come, let us get it done and", "make a Priest of you.")
      next()
      c = menu("Please.", "A moment more, please.", "I am going back to town.")
      if c == 3 then
        say(PETER, "Back to town again... You", "will never be a Priest like", "that. See a little more of", "the world, then come.")
        next()
        return warp(OUT[1], OUT[2], OUT[3])
      end
    else
      say(PETER, "God's blessing be with you.", "The first trial is behind", "you: well done. I am Peter S.", "Albert.")
      next()
      say(PETER, "How was Thomas? I ought to", "say Bishop Thomas... Not", "like me, a plain father all", "my life. Ha ha ha!")
      next()
      say(PETER, "Exorcism means what it says:", "putting down the undead and", "the demons, who harm God's", "good people.")
      next()
      say(PETER, "Until they are all put down", "no Priest has a day of rest.", "Here you try your strength", "as if you were one.")
      next()
      say(PETER, "Shall we begin?")
      next()
      c = menu("Please.", "A moment, please.", "I am going back to town.")
      if c == 3 then
        say(PETER, "What, you come this far and", "turn back? Well, it is your", "first time. Come when you", "have found your courage.")
        next()
        v.job2nd_pri = 6
        return warp(OUT[1], OUT[2], OUT[3])
      end
    end
    v.job2nd_pri = 6
    if c ~= 1 then
      say(PETER, "Hm... yes, one should be", "ready. Speak to me when you", "are.")
      return close()
    end
    say(PETER, "Then the exorcism begins. It", "is simple: put down every", "monster that appears.")
    next()
    say(PETER, "There are several stages, so", "take care. Do your best!")
    next()
    clear_the_place()
    tmp.jpri_step = 1
    tmp.jpri_tempted = 0
    add_timer(TIME_MS, "jpri_timer::OnWarp")
    warp(MAP, 24, 44)
  end,
}

-- the clock of the first stage, and what the Zombies and Mummies report to
npc {
  map = "-", name = "jpri_timer", sprite = -1, events = { "OnWarp" },
  src = "jathena-1013:script/npc/job/npc_job_priest.txt:1197",
  on_click = function(label)
    if label ~= "OnWarp" or player.map ~= MAP then return end
    clear_the_place()
    warp(OUT[1], OUT[2], OUT[3])
  end,
}
npc { map = "-", name = "jpri_undead", sprite = -1, events = { "OnKilled" }, src = "jathena-1013:script/npc/job/npc_job_priest.txt:1512", on_click = function() end }
npc { map = "-", name = "jpri_mummy", sprite = -1, events = { "OnKilled" }, src = "jathena-1013:script/npc/job/npc_job_priest.txt:1539", on_click = function() end }

local function rising(i, group, x, reach, mob, event, src_line)
  npc {
    map = MAP, x = x, y = group[1], dir = 0, sprite = -1, name = "jpri_rise_" .. i, touch = { reach, 0 },
    src = "jathena-1013:script/npc/job/npc_job_priest.txt:" .. src_line,
    on_click = function()
      if tmp.jpri_step ~= i then return end
      tmp.jpri_step = i + 1
      for k = 2, #group do monster(MAP, group[k][1], group[k][2], group[k][3], mob, 1, event) end
    end,
  }
end
for i, group in ipairs(ZOMBIES) do rising(i, group, 24, 16, ZOMBIE, RISEN, 1553 + (i - 1) * 7) end
for i, group in ipairs(MUMMY_PAIRS) do rising(5 + i, group, 98, 8, MUMMY, MUMMIES, 1589 + (i - 1) * 7) end

-- the first gate: every Zombie risen and put down
npc {
  map = MAP, x = 24, y = 109, dir = 0, sprite = -1, name = "jpri_gate_1", touch = { 2, 2 },
  src = "jathena-1013:script/npc/job/npc_job_priest.txt:999",
  on_click = function()
    if player.class == ACOLYTE then
      if tmp.jpri_step < 6 or mob_count(MAP, RISEN) > 0 then return end
      del_timer("jpri_timer::OnWarp")
    end
    warp(MAP, 168, 17)
  end,
}

-- the demons. Each: its cell, its sprite, and what it says; a page is the
-- lines, then either nothing (next), or the two answers: to give in (and
-- where that lands you) or to stand.
local STAND = "Begone, demon!"
local DEMONS = {
  { name = "Deviruchi", y = 45, sprite = 1109, line = 1216,
    { "Ho, an Acolyte! Thinking of", "turning Priest, by the look", "of you... Kya kya kya!" },
    { "Why pick so hard a road? In", "town or dungeon all they do", "is ask for your help. And", "what do they do for you?" },
    { "I mean you well. The trial is", "hard, the life after is", "harder. Find another way to", "live." },
    { give_in = "Yes... I will.", to = { "c_tower2", 168, 33 },
      lost = { "Kikiki, well chosen. Never", "come here again! A present:", "you may meet a friend of", "mine, for free! Kya kya!" } },
    { "Kikiki... do not say that.", "Give the test up now and I", "give you a thing you may see", "once in a lifetime." },
    { "Could you ever win such a", "card by your own strength?", "Think well." },
    { give_in = "Take the card.", to = { "mjolnir_05", 200, 200 },
      lost = { "Kikiki, a human after all!", "The card is yours! Not that", "I hand it over: go and get", "it yourself!" } },
    { "Kiki... a strong will. I shall", "watch whether you win one by", "yourself. You will come back", "to me sorry one day! Kya!" } },
  { name = "Doppelganger", y = 75, sprite = 1046, line = 1317,
    { "Acolyte... lend me your ear", "a while..." },
    { "Why become a Priest? I see no", "such calling in you." },
    { "If you wish it I can let you", "start your life anew: a", "Novice again, with all the", "strength you have now, and", "any job you like after." },
    { give_in = "Please do!", to = { "gef_dun02", 210, 177 },
      lost = { "Well chosen. A Novice you", "shall be, as you wished...", "Die, and be born again!" } },
    { "You do not follow me... A", "chance to turn your life", "back. Only say you will not", "be a Priest." },
    { give_in = "I will not become a Priest.", to = { "gef_dun02", 210, 177 },
      lost = { "Well chosen. A Novice you", "shall be, as you wished...", "Die, and be born again!" } },
    { "...Then I let you go, for", "now. When we next meet I", "shall give you a painful", "death." } },
  { name = "Dark Lord", y = 115, sprite = 1272, line = 1387,
    { "Halt... human. By whose leave", "do you think to pass here?" },
    { "An Acolyte who would be a", "Priest... You shall not", "pass. Go back, or I kill", "you." },
    { "Ant of a human... do not soil", "my hour of rest..." },
    { give_in = "I am sorry...", to = { "gl_church", 145, 170 }, lost = { "Never show yourself again!" } },
    { "Hmph... I could crush you", "like a dry leaf. My magic", "has no end: I shall blow you", "to dust!" },
    { give_in = "Spare my life, please!", to = { "gl_church", 145, 170 }, lost = { "Never show yourself again!" } },
    { "More spine than you look...", "The next time you stand", "before me I shall show you", "hell..." } },
  { name = "Baphomet", y = 150, sprite = 1039, line = 1448,
    { "Human..." },
    { "Shall we make a bargain...?" },
    { "I give you wealth and power:", "more gold than a life can", "spend, weapons no human can", "make..." },
    { "And you may call me whenever", "you wish. All will fear to", "stand against you, and in", "the end bow down..." },
    { "Give up the Priest's road and", "seal a pact with me. The", "world will be yours." },
    { give_in = "I will not become a Priest.", to = { "glast_01", 200, 203 },
      lost = { "Then let us seal the pact...", "You will not regret it. Come", "to where I am: there we", "seal it." } },
    { "Very well... I shall be gone,", "as you wish. But you will", "not find it easy to leave." },
    { "I have made ready some", "monsters, for you alone. Let", "me see what you are worth.", "Now go..." } },
}

for i, d in ipairs(DEMONS) do
  -- the demon itself has nothing to say when spoken to
  npc {
    map = MAP, x = 168, y = d.y, dir = 4, sprite = d.sprite, name = d.name,
    src = "jathena-1013:script/npc/job/npc_job_priest.txt:" .. d.line, on_click = function() end,
  }
  -- it speaks to whoever comes near
  npc {
    map = MAP, x = 168, y = d.y - 1, dir = 0, sprite = -1, name = "jpri_demon_" .. i, touch = { 5, 0 },
    src = "jathena-1013:script/npc/job/npc_job_priest.txt:" .. d.line,
    on_click = function()
      if player.class ~= ACOLYTE or tmp.jpri_tempted >= i then return end
      tmp.jpri_tempted = i
      for k, page in ipairs(d) do
        if page.give_in then
          if menu(page.give_in, STAND) == 1 then
            say(d.name, table.unpack(page.lost))
            next()
            clear_the_place()
            return warp(page.to[1], page.to[2], page.to[3])
          end
        else
          say(d.name, table.unpack(page))
          if k < #d then next() end
        end
      end
      close()
    end,
  }
end

-- the demons' gate: the exorcism is passed; Baphomet's monsters are next
npc {
  map = MAP, x = 168, y = 180, dir = 0, sprite = -1, name = "jpri_gate_2", touch = { 2, 2 },
  src = "jathena-1013:script/npc/job/npc_job_priest.txt:1007",
  on_click = function()
    if player.class == ACOLYTE then
      v.job2nd_pri = 7
      kill_monster(MAP, RISEN)
      kill_monster(MAP, MUMMIES)
      tmp.jpri_step = 6
    end
    warp(MAP, 98, 40)
  end,
}

-- the way out of the last room (mod0631 conf/npc_warp25.txt:452, "to 0,0")
npc {
  map = MAP, x = 97, y = 104, dir = 0, sprite = -1, name = "jpri_gate_3", touch = { 2, 2 },
  src = "m631:conf/npc_warp25.txt:452",
  on_click = function()
    clear_the_place()
    tmp.jpri_step = 0
    warp(OUT[1], OUT[2], OUT[3])
  end,
}

-- The Assassin test: the Assassin Guild under the sands near Morroc
-- (in_moc_16). An angry-looking man at the door, Kai who hides, the Nameless
-- One's quiz, Barcardi's two-part test (the targets among decoys and traps,
-- then the hide-and-sneak room), the Guild Master behind his maze, and the
-- Necklace of Oblivion back at the door.
--
-- The map: in_moc_16 in 2003-04-30gdata_turtle_ph_2.gpf (the second Turtle
-- Island patch file on the CD, which euRO applied in January 2004 with the
-- other 2-1 test maps, research/THREADS.md #8) has the Guild Master's maze of
-- walls; data.grf.original's has an open hall there. Every cell below is
-- walkable on the merged GRF's map (research/gat_cells.py), traps and
-- monsters included, except where a comment says it was moved.
--
-- Sources:
--   * eAthena 1.0 npc/jobs/2-1/assassin.txt (2004-11-04, kobra_k88, v1.1:
--     "Based of Official RO Assassin jobchange quest. There are small
--     differences due to gameplay issues", "Used some dialogue from the aegis
--     Assassin script translated By: Pgro Team", "converted the booby traps
--     from the aegis script"; data:refs/rathena commit 706e78509): everything
--     here, the NPCs and their cells, the words, the three question sets, the
--     targets and decoys, the traps, the hide room's monsters, the clock and
--     the rewards. Tier athena2003.
--   * It is the only script of the test on disk. jAthena's of 2004-09-12 is a
--     stub (script/npc/job/npc_job_12assassin.txt, "不完全", data:refs/jathena
--     commit 6332410); every English emulator up to 0.5.2 (2004-04) has
--     RoVeRT's stub; mod0631 (December 2003) has a guild member who changes
--     you at job level 40 with no test and says the requirements will come
--     with an update; 817's NPC (replaced here) does the same.
--   * No period account dates the test (research queue T1). It is here
--     because the other 2-1 tests are, with the January 2004 patch that
--     "added 2nd job quest", and because that patch's in_moc_16 brings the
--     maze. eAthena 1.0 is nine to eleven months after our period: an
--     approximation to sharpen when a period account turns up.
--
-- What is ours (docs/DEVIATIONS.md):
--   * the guild's door on moc_fild16 leads into the entrance hall, where
--     the angry-looking man stands, as in eAthena 1.0 and every later
--     script; mod0631's door (npc_warp.txt:315) led into Kai's room. The
--     hall gets eAthena's way out (18,8); Kai's room keeps mod0631's;
--   * mod0631's warp tiles between the test's rooms (npc_warp.txt:289-298)
--     are removed: the test moves you, as in eAthena (the tiles would let you
--     past Kai, into the rooms between tests, out of them mid-test, and two
--     sit on the cells eAthena sends you to);
--   * no waiting room: Barcardi asks whether you are ready, or lets you
--     lounge first (as in the Wizard, Hunter and Knight tests); nobody else
--     is ever in the test, so the Guild Master's "busy with another
--     candidate" goes;
--   * eAthena's slips mended: a decoy that spawned nowhere (mob "10631"),
--     the pass warp that came again 30 seconds later wherever you were, the
--     door out of the first room open before any test, the hide room's exit
--     that still passed you after a kill had failed you, the four guild
--     members left standing in the Guild Master's room (it disables
--     "Barkadi", "Dante" and "Gail"), the man at the door starting you over
--     when you had passed the second test, Kai and the Nameless One serving
--     anyone (the Guild Master's reward choice for whoever skipped Kai);
--   * the Nameless One's tip to a web site ("www.emperium.org", "someplace
--     called the internet") goes;
--   * three of the guild members who come to the Guild Master's room stand
--     on walls in eAthena: moved to the nearest floor;
--   * pages cut to the dialogue window, typos mended.

local MAP = "in_moc_16" -- warps below name it literally, so that `check` follows them
local SRC = "eathena-1.0:npc/jobs/2-1/assassin.txt:"
local NECKLACE = 1008
local ROOM1 = { 60, 136, 93, 177 }  -- the targets
local ROOM2 = { 64, 46, 111, 105 }  -- hide and sneak

-- eAthena's variables:
--   v.assin_q (ASSIN_Q): 1 took (or failed) the Nameless One's quiz,
--     2 passed it, 3 passed Barcardi's test, 4 has the necklace
--   v.assin_q2 (ASSIN_Q2): 1 Thomas has briefed you
--   v.assin_jl (JBLVL): the job level on Kai's form, 40 or 50 (which reward)
--   v.assin_kai (kaitrig): 1 once Kai has come out to you
--   tmp.assin_set (@temp): the Nameless One's question set, 1-3
--   tmp.assin_listen (@LISTEN): questions asked of the Nameless One
--   tmp.assin_left ($@SinMob): targets left in the first room

-- the man at the door to Kai's room, through the Nameless One's room, to
-- Barcardi's: mod0631's tiles (npc_warp.txt:293-298) and the field door
remove_warp(MAP, 20, 94)
remove_warp(MAP, 20, 140)
remove_warp(MAP, 20, 169)
remove_warp(MAP, 66, 148)
remove_warp(MAP, 88, 136)
remove_warp(MAP, 88, 105)
-- the hide room's far end, the last room, the Guild Master's (npc_warp.txt:289-292)
remove_warp(MAP, 88, 46)
remove_warp(MAP, 182, 187)
remove_warp(MAP, 182, 169)
remove_warp(MAP, 168, 116)
-- the field door into the hall (eAthena 1.0 npc/warps/other/jobquests.txt:72-73;
-- mod0631's 206,297 led to 20,74. One cell west of it, as remove_warp clears its cell;
-- the way out lands on mod0631's 206,295, as Kai's room's door does)
local FIELD = "moc_fild16" -- not literal here, so that `check` does not take the old door for a destination
remove_warp(FIELD, 206, 297)
warp { map = "moc_fild16", x = 205, y = 297, span = { 1, 1 }, to = { MAP, 18, 11 } }
warp { map = MAP, x = 18, y = 8, span = { 2, 2 }, to = { "moc_fild16", 206, 295 } }

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function thief() return player.class == Job_Thief end
local function q() return v.assin_q end

local function done_with_job_vars()
  v.assin_q, v.assin_q2, v.assin_jl, v.assin_kai = 0, 0, 0, 0
end

-- Kai's two places: in his corner, and (once he has hidden) by the wall
local KAI1, KAI2 = "assassin_kai1", "assassin_kai2"
local function kai_waits()
  enable_npc(KAI1)
  disable_npc(KAI2)
end
local function kai_hides()
  disable_npc(KAI1)
  enable_npc(KAI2)
end

-- ------------------------------------------------------------ the man at the door

local A = "[Angry looking man]"
local H = "[Assassin Huey]"

local function huey_change()
  if player.skill_point > 0 then
    page(A, "You will need to use up all your", "skill points if you want to", "become an Assassin.")
    return close()
  end
  if count_item(NECKLACE) < 1 then
    page(A, "Hm? You want me to make you an", "Assassin? Then show me your", "^5544FFNecklace of Oblivion^000000...")
    next()
    page(H, "You do have one, don't you? Only", "those the Guild Master gave the", "Necklace of Oblivion have the", "right to become Assassins.")
    next()
    if menu("It's in my other pair of Thief pants...", "Well... it kinda got stolen... heh...") == 1 then
      page(H, "I don't care where it is. If you", "want to become an Assassin, you'd", "better get it.")
      return close()
    end
    page(H, "........................")
    next()
    page(H, "!!(shouts profanities at you)!!")
    next()
    page(H, "Have fun starting ALL OVER!!")
    v.assin_q, v.assin_q2 = 0, 0
    close()
    return warp("moc_fild16", 206, 155)
  end
  page(A, "Ah... the Necklace of Oblivion.", "So the Guild Master has accepted", "you into the Assassin clan.")
  next()
  page(H, "Congratulations! After all your", "hard work, you can finally", "become an Assassin!")
  emotion(21)
  next()
  page(H, "Very well done: you are now", "officially an Assassin. You are", "free to visit us at any time,", "though...")
  set_look(7, 0)
  job_change(Job_Assassin)
  done_with_job_vars()
  next()
  page(H, "Remember that you are on your own", "now. The shadows are your new", "home, and your best friends are", "the blades in your hands...")
  close()
end

local function huey_requirements()
  page(A, "Requirements? Here they are:", "1. Be a Thief.", "2. Have a job level of 40 or more.", "3. Pass the Assassin Guild's tests.")
  next()
  page(A, "That's all there is to it. If", "you're confident in your", "abilities, the test will be", "a piece of cake.")
  next()
end

npc {
  map = MAP, x = 19, y = 33, dir = 4, sprite = 55, name = "Guildsman", id = "assassin_huey",
  replaces = "817:jobs/assassin.txt:2", src = SRC .. "29",
  on_click = function()
    if player.class == Job_Assassin then
      page(H, "Oh, it's you. You're...", player.name .. ", right?")
      next()
      page(H, "Too bad there's nothing to do", "right now. You should go train", "more. Bye.")
      return close()
    end
    local c = player.class
    if c == Job_Novice then
      page(A, "...HEY, Novice!!! GET out of here", "now!! I won't be held responsible", "if anything bad happens to you...")
      emotion(0)
      return close()
    end
    if c == Job_Swordman or c == Job_Knight or c == Job_Crusader then
      page(A, "...What are you doing here???")
      emotion(1)
      next()
      page(A, "Doing what you're ordered to do,", "eh?... You lot are nothing but dogs", "that listen to their masters...")
      emotion(32)
      return close()
    end
    if c == Job_Mage or c == Job_Wizard or c == Job_Sage then
      page(A, "...What's a mage like you doing", "here? You should be paying more", "attention to your training...")
      emotion(20)
      return close()
    end
    if c == Job_Archer or c == Job_Hunter or c == Job_Bard or c == Job_Dancer then
      page(A, "Uh... you're one of those people", "with bows, right? Sorry, there are", "no cute pets here for you to", "kill...")
      emotion(4)
      next()
      page(A, "Wait... You're not here to make", "trouble, eh!? ...LEAVE AT ONCE!!")
      emotion(0)
      return close()
    end
    if c == Job_Acolyte or c == Job_Priest or c == Job_Monk then
      page(A, "...Oh, God's little helper. What", "are you doing down in a dreadful", "place like this?")
      next()
      page(A, "Trust me, you won't find any", "salvation here... Why don't you", "just run along, okay?...")
      emotion(20)
      return close()
    end
    if c ~= Job_Thief then
      page(A, "...What the??? Do you have any", "idea where you are?... GREEDY", "SCAMMERS like you are not welcome", "here!!")
      emotion(29)
      return close()
    end
    if player.job_level < 40 then
      page(A, "Hm? What brings you here?... I", "don't think I like the way you're", "looking at me...?!")
      emotion(1)
      next()
      page(A, "...Hmm... You're not qualified", "yet. To become an Assassin, you", "have to meet our expectations.")
      next()
      page(A, "Why don't you go train some more?", "You need a job level of at least", "40 to even hope to become one", "of us...")
      return close()
    end
    if q() == 1 then
      page(A, "What's this? I can't believe you", "failed the first test.", "~Sigh~......")
      close()
      kai_hides()
      return warp("in_moc_16", 19, 76)
    end
    if q() == 2 then
      page(A, "What are you doing out here?? Go", "back in and finish the second", "test!!")
      close()
      return warp("in_moc_16", 21, 160)
    end
    if q() == 3 then
      -- eAthena starts you over here (ours: back to the last room)
      page(A, "What are you doing out here?? The", "Guild Master is waiting for you!!")
      close()
      return warp("in_moc_16", 181, 183)
    end
    if q() == 4 then return huey_change() end
    page(A, "...A Thief, huh?... And a well", "trained one at that, 'cause I", "can't seem to find my wallet!")
    emotion(18)
    next()
    page(A, "We need people like you, you know.", "So how about taking the next step", "in the world of crime, and", "becoming an Assassin?")
    next()
    while true do
      local m = menu("You bet! I've picked my last pocket.", "What are the requirements?", "Maybe later... I need to steal some things first.")
      if m == 2 then
        huey_requirements()
      elseif m == 1 then
        page(A, "It's been a long time since we've", "had an Assassin candidate...", "Anyhow, let me send you to", "^5533FFAssassin Kai^000000. He'll take care", "of the registration.")
        kai_waits()
        save_point(MAP, 19, 27)
        close()
        return warp("in_moc_16", 19, 76)
      else
        page(A, "Hmm..? Okay then... so be it...")
        return close()
      end
    end
  end,
}

-- ------------------------------------------------------------ Kai, who registers you

local K = "[Assassin Kai]"

-- what he tells a candidate who failed, for the set they failed
local TIPS = {
  { { "First of all, ^554433Grimtooth^000000 can only", "be used with a ^554433Katar^000000. It's", "useless with Daggers." },
    { "Poison is a must for all", "Assassins." },
    { "Have you used ^554433Double Attack^000000? It", "hits the enemy twice ^554433without", "using SP^000000." },
    { "^554433Red Gemstones^000000 are the only stones", "Assassins use. Blue Gemstones are", "useless to us!" },
    { "Water is stronger than Fire... so", "Water is good against Fire", "monsters." },
    { "Water is moved by Wind, so", "^554433Water is weak against Wind^000000." },
    { "You can ^554433Cloak only close to a", "wall^000000. Be more than a cell away", "from it and you'll be seen." } },
  { { "Some Katars have element powers." },
    { "What weapon is worth buying? If", "you don't know, look at your", "hands." },
    { "Katars are dropped by monsters in", "the desert. They are very useful." },
    { "The Katar is one of the strongest", "weapons an Assassin can use." },
    { "I ^554433wouldn't want to raise a Roda", "Frog as a pet^000000, and neither would", "anyone else for that matter." },
    { "^554433Fire^000000 is good against ^554433Earth", "monsters^000000." },
    { "If you want to know your weapon's", "element, look at its name." } },
  { { "The ^554433Elder Willow Card adds to", "your Intelligence^000000." },
    { "We Assassins are masters of", "dodging and attacking. Our", "defence, though, is very poor." },
    { "......" },
    { "Daggers can be used in both", "hands!" },
    { "I haven't been to Morroc for a", "long time... not since I became", "a Thief, anyway..." },
    { "Stealing mushrooms... what fond", "memories of those orange gooey", "mushrooms..." },
    { "The ^554433Baphomet Jr. Card adds 3", "Agility and 1 Critical^000000." },
    { "We Assassins have the highest", "Agility of all. We get up to", "^5533FF10 extra points of Agility^000000." } },
}

local function to_the_hall_of_tests()
  close()
  warp("in_moc_16", 19, 141)
end

local function kai_register()
  page(K, "Okay, good. Fill out this form", "first. Make sure to write down", "your name and your job level.")
  next()
  mes "(You fill out the form and hand it back.)"
  next()
  page(K, "Let's see... your name is", "^5533FF" .. player.name .. "^000000... and your", "job level is " .. player.job_level .. "...")
  next()
  if player.job_level >= 50 then
    v.assin_jl = 50
    page(K, "Wow, a job level of 50! You've", "been training hard, haven't you?", "The Guild Master will be pleased", "to see someone of your", "qualifications.")
    next()
    page(K, "Your first test is with the", "^5533FFNameless One^000000. I'll send you to", "him right away.")
    return to_the_hall_of_tests()
  end
  v.assin_jl = 40
  page(K, "Well, you barely meet the", "requirements, but meh, that's not", "important.")
  next()
  page(K, "~mumbles~ (Nobody has any guts", "anymore... bunch of pansies...)")
  next()
  page(K, "Hm? What's that? Oh, I was just", "talking to myself... It was", "nothing, really.")
  next()
  page(K, "Anyway, let me send you to the", "test hall for your first test.")
  to_the_hall_of_tests()
end

local function kai_failed()
  page(K, "Ehhh? Weren't you just here", "a minute ago?")
  emotion(1)
  next()
  page(K, "What's this?... You failed the", "first test?")
  next()
  page(K, "HAHAHAHAAHAHAHAHA!!!!")
  emotion(18)
  next()
  page(K, "Hehe... HAHA... ~cough~... Damn...", "It's been a long time since I've", "met a loser like you... Ha ha...")
  emotion(18)
  next()
  page(K, "Oh... sorry for laughing in your", "face like this... but it's just", "too funny... hahahehehe...")
  emotion(18)
  next()
  page(K, "So... do you need me to give you", "any hints?")
  next()
  local c = menu("...Yes... please.", "Stop laughing and just give me the stupid tips!", "Shut up! I don't need your stinking help!!")
  if c == 1 then
    page(K, "Hahaahhaha!!! Well, at least", "you're honest... Haha... My", "stomach... ouch, my stomach", "hurts!! You're killing me!")
    emotion(18)
    next()
    page("[Nameless One]", "...Hahaha.")
    next()
    page(K, "HAHAHAHA!! Nameless One, you think", "it's funny too?")
    emotion(18)
    next()
    page("[Nameless One]", "Hehehe... yes, very...")
    next()
    page(K, "Heh heh... Oh my... So you want", "some pointers, huh?")
    next()
    page(K, "...")
    next()
    page(K, "...............")
    next()
    page(K, "......Hmmm... Too bad. I don't", "feel like giving you any.")
    next()
    page(K, "Hahaha... I can't believe this...", "hahaha...")
    emotion(18)
    return to_the_hall_of_tests()
  end
  if c == 2 then
    page(K, "Hmm... I see. Sorry about my", "laughing... People make mistakes", "from time to time. I understand.")
    next()
    page(K, "I can't give you the answers, but", "I can tell you something about", "Assassins that may help you...")
    next()
    page(K, "For an Assassin, honour and pride", "are everything. You can't be an", "Assassin without honour. One day", "people will come to rely on you.", "Waiting for that day is an", "Assassin's destiny.")
    next()
    page(K, "Assassins live alone. Our lives", "make it hard to get close to", "anyone. Imagine what your loved", "ones would think if they saw your", "blood-stained hands...")
    next()
    page(K, "...Without a doubt they would be", "shocked, and fear you. They could", "not stay by your side, and you", "would be left alone with your", "sins.")
    next()
    page(K, "Lonesome as it is, it is still", "a worthy life. An Assassin is free", "to do as he chooses, with nobody", "to tie him down.")
    next()
    page(K, "That's all I have to say about", "Assassins... I hope it doesn't", "depress you?")
    return to_the_hall_of_tests()
  end
  if c ~= 3 then return close() end
  page(K, "...Hmmm...")
  next()
  page(K, "Good. That's the spirit! Never let", "anyone take away your confidence.", "Assassins must be strong-minded", "and determined. Sorry I laughed.")
  next()
  page(K, "Too many morons nowadays know", "nothing about their jobs...")
  next()
  page(K, "How can they forget what it took", "to become an Assassin??!! They are", "truly shameless...")
  next()
  page(K, "So please...")
  next()
  page(K, "Always be proud that you are an", "Assassin. Respect the blood that", "stains your katars and daggers!!")
  next()
  if menu("Okay, gotcha.", "Uh... I'm confused...") == 1 then
    page(K, "...Good! I'm glad you understand", "me. Here, let me give you some", "tips about the first test...")
    next()
    local tips = TIPS[tmp.assin_set] or TIPS[1]
    for _, tip in ipairs(tips) do
      page(K, table.unpack(tip))
      next()
    end
    page(K, "I've said quite a lot, and I", "actually feel a bit tired now.", "Hopefully you'll do better this", "time. I'll send you to the test", "hall again.")
    return to_the_hall_of_tests()
  end
  page(K, "...Hmph... how can you be such an", "idiot??? You couldn't understand", "what I was talking about??")
  emotion(23)
  next()
  page(K, "Is an Assassin's honour that hard", "to comprehend?!")
  next()
  page(K, "Grrrrr!!! You're probably going to", "be a moron all your life...")
  emotion(6)
  next()
  page(K, "GET OUT! Get out of here right", "now!! You're not fit to become", "an Assassin!!!")
  emotion(32)
  close()
  warp("c_tower4", 64, 76)
end

local function kai_talk()
  v.assin_kai = 1
  if q() == 1 then return kai_failed() end
  page(K, "So you're an Assassin candidate...", "~sigh~... Sorry about how I acted.", "When someone comes near me, I", "can't help but hide... It's a bad", "habit, really...")
  next()
  page(K, "Anyhow, welcome. So you want to be", "an Assassin, do you?")
  next()
  if menu("Yes.", "...No.") == 1 then return kai_register() end
  page(K, "Huh?? You don't?...")
  emotion(1)
  next()
  page(K, "Are you trying to play games with", "me??")
  next()
  page(K, "Don't you want to become an", "Assassin?...")
  next()
  if menu("Not really...", "Yes, of course I do.") == 2 then
    page(K, "...Hmph... Anyway...")
    next()
    return kai_register()
  end
  page(K, "...Well, if that's the case...", "GET OUT!!")
  close()
  warp("moc_fild16", 206, 229)
end

-- in his corner: he hides from whoever comes near
npc {
  map = MAP, x = 21, y = 91, dir = 4, sprite = 730, name = "Assassin Kai", id = KAI1, touch = { 4, 4 },
  src = SRC .. "207",
  on_click = function() end,
  on_touch = function()
    page(K, "Ummm???...")
    close()
    kai_hides()
  end,
}

npc {
  map = MAP, x = 25, y = 90, dir = 4, sprite = 730, name = "Assassin Kai", id = KAI2, touch = { 2, 1 },
  src = SRC .. "219",
  on_click = function()
    -- eAthena: anyone; ours: Thieves of job level 40, the ones the man at the door sends
    if v.assin_kai == 1 and thief() and player.job_level >= 40 then return kai_talk() end
    page(K, "Come closer. I like to see a", "person's face when I'm talking", "to them.")
    close()
  end,
  on_touch = function()
    if thief() and player.job_level >= 40 then kai_talk() end
  end,
}

-- ------------------------------------------------------------ the Nameless One's quiz

local N = "[Nameless One]"
local NAMELESS = "assassin_nameless"

-- the three sets: the question, the four answers, and which are right
local QUIZ = {
  {
    { { "1. Which of these is ^FF5533not^000000 needed", "to learn Grimtooth?" },
      { "Cloak Lv 2", "Sonic Blow Lv 5", "Hide Lv 2", "Left Hand Mastery Lv 2" }, { 4 } },
    { { "2. Enchant Poison gives your", "weapon which element?" }, { "Poison", "Earth", "Fire", "Wind" }, { 1 } },
    { { "3. What does ^5533FFLeft^000000 Hand Mastery", "do at level 4?" },
      { "Attack +80%", "Attack +70%", "Attack +90%", "Attack +108%!!" }, { 2 } },
    { { "4. What item does the skill", "Venom Dust use up?" },
      { "Green Gemstone", "Blue Gemstone", "Yellow Gemstone", "Red Gemstone" }, { 4 } },
    { { "5. Which new skill appears when", "Enchant Poison reaches level 5?" },
      { "Venom Splasher", "Grimtooth", "Sonic Blow", "Venom Dust" }, { 4 } },
    { { "6. Which skill lets you walk", "around unseen?" }, { "Hide", "Back Slide", "Cloak", "Throw Sand" }, { 3 } },
    { { "7. What does Venom Dust need?" },
      { "The enemy must be weak.", "A Red Gemstone.", "A certain amount of HP." }, { 2 } },
    { { "8. Which monster card adds to", "Intelligence?" }, { "Steel Chonchon", "Deviruchi", "Elder Willow", "Baphomet" }, { 3 } },
    { { "9. How much SP does a double hit", "with a dagger use?" }, { "15", "0", "10", "54" }, { 2 } },
    { { "10. Which sword is best in the", "Byalan Dungeon?" },
      { "Sword of Piercing Wind", "Sword of Ice", "Sword of Earth", "Sword of Fire" }, { 1 } },
  },
  {
    { { "1. Which monster drops a slotted", "Katar?" }, { "Thief Bug Male", "Peco Peco", "Desert Wolf", "Kobold" }, { 3 } },
    { { "2. Which card can go into a Jur?" }, { "Caramel", "Ghostring", "Baphomet Jr.", "Doppelganger" }, { 1 } },
    { { "3. Which class can forge weapons?" }, { "Merchant", "Blacksmith", "Thief", "Priest" }, { 2 } },
    { { "4. Which of these isn't a Katar?" }, { "Jur", "Jamadhar", "Infiltrator", "Gladius" }, { 4 } },
    { { "5. Most monsters in the Byalan", "Dungeon are of which element?" }, { "Water", "Fire", "Wind", "Earth" }, { 1 } },
    { { "6. Which monster can't be tamed", "and made a cute pet?" }, { "Poring", "Roda Frog", "Lunatic", "Poison Spore" }, { 2 } },
    { { "7. Which monster is weakest", "to Fire?" },
      { "Kobold (Sword)", "Kobold (Mace)", "Kobold (Hammer)", "Kobold (Axe)" }, { 4 } },
    { { "8. Which Katar has ^FF5533no^000000 element?" },
      { "Katar of Raging Blaze", "Katar of Dusty Thornbush", "Sharpened Legbone of Ghoul", "Infiltrator" }, { 4 } },
    { { "9. Which monster doesn't belong", "with the others?" }, { "Poring", "Ghostring", "Creamy", "Drops" }, { 3 } },
    { { "10. Which monster is ^FF3355not^000000", "Undead?" }, { "Drake", "Archer Skeleton", "Poison Spore", "Ancient Mummy" }, { 3 } },
  },
  {
    { { "1. How much Flee does Improve", "Dodge give at level 10?" }, { "20", "30", "140", "15" }, { 2 } },
    { { "2. Which monster can find someone", "hidden or cloaked?" }, { "Hode", "Whisper", "Poring", "Munak" }, { 2 } },
    { { "3. Assassins can use two weapons", "at once. Which pair ^5544FFcan^000000 an", "Assassin use?" },
      { "Jur and Mace", "Damascus and Claymore", "Damascus and Stiletto", "Axe and Stiletto" }, { 3 } },
    { { "4. In which town does one become", "a Thief?" }, { "Prontera", "Lutie", "Al De Baran", "Morroc" }, { 4 } },
    { { "5. Which card has ^FF3355nothing^000000 to do", "with ^5533FFAgility^000000?" },
      { "Baphomet Jr. Card", "Whisper Card", "Thief Bug Male Card", "Chonchon Card" }, { 2 } },
    { { "6. What makes Assassins so", "special?" },
      { "Excellent singing", "Excellent acting", "Excellent dancing", "Excellent dodging" }, { 4 } },
    { { "7. At ^5533FFjob level 50^000000, how much", "Agility does an Assassin's job", "bonus give?" }, { "7", "8", "9", "10" }, { 4 } },
    { { "8. Which of these can an Assassin", "^FF5533not^000000 use?" }, { "Sword", "Golden Helm", "Dagger", "Jur" }, { 2 } },
    -- two right answers, as in eAthena
    { { "9. Which mushrooms does a Novice", "steal to become a Thief?" },
      { "Orange Net Mushrooms", "Red Gooey Mushrooms", "Orange Gooey Mushrooms", "Hairy Orange Mushrooms" }, { 1, 3 } },
    { { "10. Which card is useless to", "an Assassin?" },
      { "Ghostring Card", "Elder Willow Card", "Skeleton Soldier Card", "Kobold Card" }, { 2 } },
  },
}

local function nameless_quiz()
  page(N, "Okay, it's time for the test. Get", "more than one question ^FF5533wrong^000000", "and you fail.")
  next()
  page(N, "You won't be told which answers", "you got right or wrong.")
  next()
  tmp.assin_set = rand(1, #QUIZ)
  page(N, "Alright, let us begin...")
  next()
  local score = 0
  for _, item in ipairs(QUIZ[tmp.assin_set]) do
    page(N, table.unpack(item[1]))
    next()
    local c = menu(table.unpack(item[2]))
    for _, good in ipairs(item[3]) do
      if c == good then score = score + 10 end
    end
  end
  page(N, "Okay, let me just tally up the", "results...")
  next()
  page(N, "Here is your score: ^FF5544" .. score .. "^000000/100...")
  if score >= 90 then
    mes "...Very well, you passed the quiz."
    next()
    page(N, "Don't get so excited yet: there's", "another test. Speak to ^5533FFBarcardi^000000", "and she'll tell you about the", "second test.")
    save_point("in_moc_16", 21, 160)
    v.assin_q = 2
    return close()
  end
  mes "You failed the test!"
  next()
  page(N, "You're underqualified... How could", "you expect to become an Assassin", "with a score like this?")
  next()
  page(N, "Since you don't seem to have an", "ounce of intelligence, a tip:", "speak with ^5533FFAssassin Kai^000000. He may", "be able to help you out.")
  close()
  kai_hides()
  warp("in_moc_16", 19, 76)
end

local SKILLS = {
  { "Yes, the skills. Skills are very", "useful to an Assassin. The basic", "ones are the masteries; I'll tell", "you about those first." },
  { "First, ^5533FFKatar Mastery^000000. It raises", "your damage with a Katar: very", "helpful for Katar users." },
  { "Assassins can wield two weapons", "at once. That makes them fierce", "warriors, but it has its", "drawbacks..." },
  { "Two weapons at once are harder to", "handle, so each hand does less", "damage." },
  { "That is why there are ^5533FFRight Hand", "Mastery and Left Hand Mastery^000000.", "Master them and both hands get", "their power back." },
  { "Left Hand Mastery can be learned", "after 2 levels of Right Hand", "Mastery. With Katar Mastery, you", "can be a deadly Assassin indeed." },
  { "^5533FFSonic Blow^000000 is an extremely fast", "attack: up to 8 hits in the blink", "of an eye." },
  { "It works only with a Katar, and", "needs Katar Mastery at level 4.", "Its damage depends on your", "Strength." },
  { "^5533FFGrimtooth^000000... what if you could", "attack without being seen? While", "cloaked, Grimtooth strikes from", "afar and splashes." },
  { "The splash makes it useful against", "a mob. The higher its level, the", "farther it reaches." },
  { "To use ^5533FFCloak^000000 you need Hide at", "level 2. Cloak makes you", "invisible, and lets you move while", "you are hidden." },
  { "You can only move close to a wall,", "though." },
  { "A technique gives your weapon the", "Poison property: ^5533FFEnchant Poison^000000." },
  { "With it, an enemy you hit may be", "poisoned. You need Envenom at", "level 1 to learn it." },
  { "^5533FFPoison React^000000 lets you counter an", "attack from a Poison monster. You", "need Enchant Poison at level 3." },
  { "^5533FFVenom Dust^000000 poisons an area of", "ground. A monster that walks in", "will likely be poisoned too." },
  { "The higher its level, the longer", "the poison stays. It uses a Red", "Gemstone, and needs Enchant", "Poison at level 5." },
  { "^5533FFVenom Splasher^000000 poisons a target", "and makes it burst, splashing", "venom everywhere." },
  { "A monster hit by it bursts only", "once its HP is down to a third." },
  { "It is the hardest Assassin skill", "to learn: you need Venom Dust and", "Poison React at level 5." },
}

local function nameless_first()
  tmp.assin_listen = 0
  page(N, "Welcome, my guest. Muhahaha!!!", "There is no use trying to find me,", "for I am perfectly hidden! To", "hide perfectly is the TRADEMARK", "of a GREAT Assassin!!")
  next()
  page(N, "Huh? Why don't I have a name?", "Muhahahahaha!!!... The better", "question is why I am hidden...")
  next()
  page(N, "After all, I may be trying to", "assassinate you!!... Does this", "scare you?")
  next()
  if menu("Eeek! Uh... I think I wet my pants...", "Bleh! You're all talk! Come on, challenge me!") ~= 2 then
    page(N, "Heh... doesn't surprise me. I could", "tell you were nothing but a wimp!", "Had we met anywhere else, I would", "have torn you limb from limb...")
    next()
    page(N, "Hmph... I don't have time for", "wussies... get lost...")
    close()
    kai_hides()
    return warp("in_moc_16", 19, 76)
  end
  page(N, "What?... So you think you're", "tough, huh?...")
  next()
  page(N, "................")
  next()
  page(N, "Hmph... Let me tell you", "something...")
  next()
  page(N, "I'm a cold-blooded killer. Wherever", "I go, a trail of blood follows my", "every footstep...")
  next()
  page(N, "A true Assassin like me needs no", "name... I am known only by the", "sharp edges of my blades.")
  next()
  page(N, "My 'handiwork' got me this far... I", "have never felt remorse over any", "job I've taken. To 'eliminate'", "targets is what I am here for.")
  next()
  page(N, "You speak with confidence, but", "understand this...")
  next()
  page(N, "Only the truly cold-blooded and", "fearless make it as Assassins. It", "takes much more than acting like", "an arrogant fool to convince me.")
  next()
  page(N, "Before the quiz, I will let you", "ask me some basic questions.")
  next()
  while true do
    local c = menu("...What are the skills?", "...What matters most to an Assassin?", "...That's enough, I want to start the test.")
    if c == 1 then
      for _, p in ipairs(SKILLS) do
        page(N, table.unpack(p))
        next()
      end
      page(N, "Those are all the Assassin skills.")
      next()
      tmp.assin_listen = tmp.assin_listen + 1
    elseif c == 2 then
      page(N, "Hmm... for Assassins, the most", "important stat is Agility. With it,", "you dodge more and attack faster.")
      next()
      page(N, "Strength is another good one: it", "raises your attack, so you kill", "faster.")
      next()
      page(N, "That is as much as I can tell you", "about stats. The rest you'll have", "to find out for yourself.")
      next()
      tmp.assin_listen = tmp.assin_listen + 1
    elseif c == 3 then
      break
    else
      return close()
    end
  end
  v.assin_q = 1
  if tmp.assin_listen == 0 then
    page(N, "Hahaha! You're very cocky, aren't", "you... Don't want to listen to what", "I have to say, eh?!")
    next()
    page(N, "Fine then... let's see whether you", "can pass this test...")
    next()
  end
  nameless_quiz()
end

local function nameless_again()
  page(N, "Hmph, you're back... So you're", "still obsessed with becoming an", "Assassin, eh?")
  next()
  page(N, "If it is your true desire, I'll", "help you out...")
  next()
  page(N, "But if you fail again... I suggest", "you give up on becoming an", "Assassin!")
  next()
  if menu("You're right, I'm not cut out for this...", "I don't care! I'm not giving up!") ~= 2 then
    page(N, "~Cough~ cough~... A good choice...", "Being an Assassin is lonely...")
    next()
    page(N, "Leave the Assassin Guild now. Go", "back to the town you came from!")
    close()
    return warp("moc_fild16", 206, 241)
  end
  page(N, "Alright then... I will try to help", "you become a lonely Assassin,", "hahahahah.")
  next()
  nameless_quiz()
end

-- perfectly hidden: he speaks to you once the step across the room has brought you back
npc {
  map = MAP, x = 19, y = 150, dir = 1, sprite = -1, name = "Nameless One", id = NAMELESS,
  events = { "OnTimer700" }, src = SRC .. "534",
  on_click = function(label)
    if label ~= "OnTimer700" or not thief() then return end
    if q() == 1 then return nameless_again() end
    if q() == 0 then return nameless_first() end
  end,
}

-- across the room: keeps candidates from walking past him before the quiz
npc {
  map = MAP, x = 19, y = 144, dir = 1, sprite = -1, name = "getbacker", id = "assassin_getbacker", touch = { 8, 0 },
  src = SRC .. "526",
  on_click = function()
    if not thief() or q() > 1 then return end
    add_timer(700, NAMELESS .. "::OnTimer700")
    warp("in_moc_16", 19, 142)
  end,
}

-- ------------------------------------------------------------ Barcardi's test

local B = "[Barcardi]"
local CLOCK = "assassin_clock"
local EXIT1, EXIT2, THOMAS = "assassin_exit1", "assassin_exit2", "assassin_thomas"
local TARGET_EV, DECOY_EV, HIDE_EV = CLOCK .. "::OnTarget", CLOCK .. "::OnDecoy", CLOCK .. "::OnHideKill"

-- 0 nothing running, 1 the targets, 2 the targets done (the way out open), 3 hide and sneak
local stage = 0
local room = ROOM1

local TARGETS = {
  { 62, 161, 1002 }, { 85, 169, 1063 }, { 88, 152, 1002 }, { 90, 143, 1113 }, { 74, 167, 1031 }, { 77, 173, 1002 },
}
local DECOYS = {
  { 62, 161, 1063, "Jabs change target" }, { 85, 169, 1031, "Warrior test target" }, { 79, 174, 1113, "Target" },
  { 85, 156, 1063, "Job quest target" }, { 74, 171, 1002, "bouncer" }, { 68, 173, 1113, "I got yours right here!" },
  { 65, 158, 1002, "Battle Target" }, { 60, 158, 1113, "Soldier Target" }, { 64, 169, 1002, "Target" },
  { 71, 173, 1063, "Job change ready" }, { 77, 172, 1002, "Don't hit me" }, { 76, 172, 1063, "Target" },
  { 75, 172, 1113, "Not me" }, { 67, 167, 1063, "Keep up the good work" }, { 86, 170, 1031, "Job target change" },
  { 86, 171, 1002, "Target" }, { 86, 170, 1113, "Target" },
  -- eAthena: mob "10631" and no count, so it never came
  { 86, 173, 1063, "Hope you can become an assassin.." },
  { 85, 170, 1031, "Battle Monster" }, { 89, 156, 1002, "Target" }, { 89, 156, 1113, "Speed182" },
  { 89, 156, 1063, "Battle Target" }, { 89, 156, 1113, "Battlefield Tears" }, { 89, 156, 1031, "Job Change Tears" },
  { 83, 169, 1002, "Evil Servant" }, { 63, 158, 1063, "Dead soul" }, { 63, 157, 1002, "Target" },
  { 64, 159, 1002, "Battle Target" }, { 63, 159, 1063, "Target" }, { 63, 159, 1002, "Archer Target" },
  { 63, 159, 1002, "Swordman Target" }, { 83, 148, 1002, "Thief Target" }, { 82, 148, 1002, "Acolyte Target" },
  { 84, 148, 1002, "Merchant Target" },
}
-- the booby traps, 2x2 squares and a few rows ("converted ... from the aegis script")
local TRAPS = {
  { 68, 158 }, { 69, 158 }, { 68, 159 }, { 69, 159 }, { 64, 162 }, { 65, 162 }, { 64, 163 }, { 65, 163 },
  { 62, 168 }, { 63, 168 }, { 62, 169 }, { 63, 169 }, { 66, 170 }, { 67, 170 }, { 66, 171 }, { 67, 171 },
  { 64, 174 }, { 64, 175 }, { 65, 174 }, { 65, 175 }, { 72, 174 }, { 72, 175 }, { 73, 174 }, { 73, 175 },
  { 72, 166 }, { 72, 167 }, { 73, 166 }, { 73, 167 }, { 72, 168 }, { 72, 169 }, { 73, 168 }, { 73, 169 },
  { 78, 168 }, { 78, 169 }, { 79, 168 }, { 79, 169 }, { 80, 172 }, { 81, 172 }, { 82, 172 }, { 83, 172 },
  { 80, 173 }, { 81, 173 }, { 82, 173 }, { 83, 173 }, { 88, 174 }, { 88, 175 }, { 89, 174 }, { 86, 166 },
  { 86, 167 }, { 87, 166 }, { 87, 167 }, { 90, 164 }, { 90, 165 }, { 91, 164 }, { 91, 165 }, { 84, 160 },
  { 85, 160 }, { 86, 160 }, { 87, 160 }, { 88, 160 }, { 89, 160 }, { 84, 161 }, { 85, 161 }, { 86, 161 },
  { 87, 161 }, { 88, 161 }, { 89, 161 }, { 86, 154 }, { 86, 155 }, { 87, 154 }, { 87, 155 }, { 84, 150 },
  { 84, 151 }, { 85, 150 }, { 85, 151 }, { 90, 150 }, { 90, 151 }, { 91, 150 }, { 91, 151 }, { 86, 146 },
  { 86, 147 }, { 87, 146 }, { 87, 147 },
}
-- the hide room: Mummies and Hydras, none to be touched
local HIDERS = {
  { 81, 77, 1041 }, { 83, 77, 1041 }, { 85, 77, 1041 }, { 88, 77, 1041 }, { 90, 77, 1041 },
  { 78, 56, 1041 }, { 80, 56, 1041 }, { 91, 56, 1041 }, { 93, 56, 1041 }, { 95, 56, 1041 }, { 97, 56, 1041 },
  { 79, 62, 1068 }, { 76, 65, 1068 }, { 96, 65, 1068 }, { 99, 65, 1068 },
}

local function reset()
  stage = 0
  npc_timer_stop(CLOCK)
  kill_monster(MAP, TARGET_EV)
  kill_monster(MAP, DECOY_EV)
  kill_monster(MAP, HIDE_EV)
  disable_npc(EXIT1)
  enable_npc(THOMAS)
end

local function say_room(text)
  area_announce(MAP, room[1], room[2], room[3], room[4], text, 0)
end

-- Barcardi::OnStart and SinTest2
local function start_targets()
  reset()
  del_timer(EXIT1 .. "::OnTimer30000")
  stage, room = 1, ROOM1
  tmp.assin_left = #TARGETS
  for _, m in ipairs(TARGETS) do monster(MAP, m[1], m[2], "Job Change Target", m[3], 1, TARGET_EV) end
  for _, m in ipairs(DECOYS) do monster(MAP, m[1], m[2], m[4], m[3], 1, DECOY_EV) end
  npc_timer_start(CLOCK)
  warp("in_moc_16", 65, 150)
end

local function start_hiding()
  reset()
  disable_npc(THOMAS)
  stage, room = 3, ROOM2
  for _, m in ipairs(HIDERS) do monster(MAP, m[1], m[2], "--ja--", m[3], 1, HIDE_EV) end
  npc_timer_start(CLOCK)
  warp("in_moc_16", 87, 102)
end

local function barcardi_explains()
  page(B, "This test has two parts. The first", "tests how well you find and", "eliminate targets. You will need", "precision and patience to pick", "them out from the rest.")
  next()
  page(B, "Look for monsters called ^5533FF'Job", "Change Target'^000000. Kill anything else", "and you fail. You have ^FF55333 minutes^000000;", "take longer and you fail too.")
  next()
  page(B, "Picking out the right target", "quickly is basic for an Assassin.", "Another basic is stealth. An", "Assassin is NEVER seen!")
  next()
  page(B, "In the second part you sneak past a", "crowd of aggressive and dangerous", "monsters. Your ^5544FFHide^000000 skill will", "come in handy. ^FF55333 minutes^000000 again.")
  next()
  page(B, "Get found by the monsters and", "killed, or run out of time, and", "you fail.")
  next()
  page(B, "You must pass BOTH parts to move on", "to the next exam. Fail one, and you", "start the whole test over again.")
  next()
end

npc {
  map = MAP, x = 21, y = 165, dir = 4, sprite = 725, name = "Barcardi", id = "assassin_barcardi",
  src = SRC .. "1030",
  on_click = function()
    if not thief() or q() ~= 2 then
      page(B, "Hm? You have no business with me.")
      return close()
    end
    page(B, "I see you passed the first test,", "^5533FF" .. player.name .. "^000000.")
    next()
    while true do
      local c = menu("What is the next test?", "I want to start the test.", "I need to go back and rest.")
      if c == 1 then
        barcardi_explains()
      elseif c == 2 then
        -- eAthena: a waiting room, one candidate at a time
        page(B, "Only one person can take the test", "at a time. ...Nobody is in the", "test rooms now. Shall I send you", "in?")
        next()
        local r = menu("I am ready.", "I'd like to lounge a bit first.")
        if r == 1 then
          page(B, "Off you go, then.")
          close()
          return start_targets()
        end
        page(B, "Take your time. Tell me when you", "are ready.")
        return close()
      elseif c == 3 then
        page(B, "...Okay, come back when you've had", "enough rest...")
        next()
        page(B, "...Oh, and don't forget to bring", "some courage with you, loser!")
        emotion(32)
        close()
        return warp("moc_fild16", 206, 241)
      else
        return close()
      end
    end
  end,
}

-- SinTest2's events and TimerSin's clock, three minutes for either part
local SAY = {
  [500] = "Okay, the test is about to start! Remember to eliminate the monsters called 'Job Change Target'.",
  [3000] = "I'll announce how much time you have left, minute by minute.",
  [5000] = "You have 3 minutes starting now!",
  [65000] = "You have 2 minutes left!",
  [125000] = "1 minute left!",
  [155000] = "30 seconds left!",
  [180000] = "Test ending in 5 secs....",
  [181000] = "4 seconds....",
  [182000] = "3 seconds...",
  [183000] = "2 seconds..",
  [184000] = "1 second.",
  [185000] = "Okay, time's up! You failed!",
}
local clock_events = { "OnTarget", "OnDecoy", "OnHideKill", "OnTimer188000", "OnInit" }
for ms in pairs(SAY) do clock_events[#clock_events + 1] = "OnTimer" .. ms end

npc {
  map = "-", name = CLOCK, sprite = -1, events = clock_events, src = SRC .. "1378",
  on_click = function(label)
    if label == "OnInit" then
      -- Kai waits in his corner; the way out of the first room and the guild
      -- members at the Guild Master's are not there yet
      kai_waits()
      disable_npc(EXIT1)
      for _, e in ipairs({ "huey", "kai", "nameless", "barcardi", "observer", "thomas" }) do
        disable_npc("assassin_end_" .. e)
      end
      return
    end
    if label == "OnTarget" then
      if stage ~= 1 then return end
      tmp.assin_left = tmp.assin_left - 1
      say_room("Good. You have " .. tmp.assin_left .. " targets left to eliminate.")
      if tmp.assin_left > 0 then return end
      npc_timer_stop(CLOCK)
      kill_monster(MAP, TARGET_EV)
      kill_monster(MAP, DECOY_EV)
      stage = 2
      enable_npc(EXIT1)
      say_room("Congratulations! You passed the test! A warp has just opened. Use it to leave the test room.")
      return add_timer(30000, EXIT1 .. "::OnTimer30000") -- moves you along after 30 seconds
    end
    if label == "OnDecoy" or label == "OnHideKill" then
      if (label == "OnDecoy" and stage ~= 1) or (label == "OnHideKill" and stage ~= 3) then return end
      stage = 0
      npc_timer_stop(CLOCK)
      say_room(label == "OnDecoy" and "You failed!" or ". . . You engaged a target without permission! You have failed the test!")
      return add_timer(3000, CLOCK .. "::OnTimer188000")
    end
    if label == "OnTimer188000" then
      area_warp(MAP, room[1], room[2], room[3], room[4], "in_moc_16", 21, 160)
      return reset()
    end
    local ms = tonumber(label:match("OnTimer(%d+)"))
    if ms == 500 and stage ~= 1 then return end
    if ms and SAY[ms] then say_room(SAY[ms]) end
  end,
}

npc {
  map = MAP, x = 62, y = 153, dir = 1, sprite = 55, name = "Observer", id = "assassin_observer",
  src = SRC .. "1172",
  on_click = function()
    page("[Observer]", "Don't mind me, I'm just here to", "see how well you do on your", "test...")
    close()
  end,
}

-- the way out of the first room, once the six have fallen; after 30 seconds it takes you anyway
npc {
  map = MAP, x = 87, y = 137, dir = 0, sprite = 45, name = "sinWarp1", id = EXIT1, touch = { 2, 1 },
  events = { "OnTimer30000" }, src = SRC .. "1180",
  on_click = function(label)
    if stage ~= 2 then return end
    if label == "OnTimer30000" then
      local x, y = player.x, player.y
      if player.map ~= MAP or x < ROOM1[1] or x > ROOM1[3] or y < ROOM1[2] or y > ROOM1[4] then return end
    else
      del_timer(EXIT1 .. "::OnTimer30000")
    end
    reset()
    warp("in_moc_16", 87, 102)
  end,
}

for i, t in ipairs(TRAPS) do
  npc {
    map = MAP, x = t[1], y = t[2], dir = 0, sprite = -1, name = "trap", id = "assassin_trap_" .. i, touch = { 0, 0 },
    src = SRC .. "1190",
    on_click = function()
      if stage ~= 1 and stage ~= 2 then return end
      del_timer(EXIT1 .. "::OnTimer30000")
      reset()
      warp("in_moc_16", 19, 161)
    end,
  }
end

-- ------------------------------------------------------------ hide and sneak

local T = "[Thomas]"

local function thomas_talk()
  if v.assin_q2 == 1 then
    page(T, "~sigh~... I told you to use your", "Hide skill!! Here, let me heal", "you...")
    next()
    page(T, "Alright, let's try this again...")
    percent_heal(100, 100)
    next()
  else
    page(T, "My name is Thomas, and I am in", "charge of this part of the test.")
    next()
    page(T, "The goal is to evade the enemy:", "reach the warp without drawing the", "monsters' attention.")
    next()
    page(T, "You must get through alive. None", "of these monsters are your", "targets, so you may not kill any", "of them. Flee, and hide.")
    next()
    page(T, "Alright... Get ready...")
    next()
    v.assin_q2 = 1
  end
  close()
  start_hiding()
end

npc {
  map = MAP, x = 89, y = 98, dir = 1, sprite = 118, name = "Thomas", id = THOMAS, touch = { 4, 2 },
  events = { "OnTimer600" }, src = SRC .. "1287",
  on_click = function(label)
    if not thief() or q() ~= 2 then
      page(T, "My name is Thomas. I am in charge", "of part of the Assassin test.")
      return close()
    end
    thomas_talk()
  end,
  on_touch = function()
    if not thief() or q() ~= 2 then return end
    add_timer(600, THOMAS .. "::OnTimer600")
    warp("in_moc_16", 87, 99)
  end,
}

npc {
  map = MAP, x = 87, y = 48, dir = 2, sprite = 45, name = "sinWarp2", id = EXIT2, touch = { 2, 2 },
  src = SRC .. "1360",
  on_click = function()
    -- eAthena passes whoever steps here, also in the three seconds after a kill has failed them
    if stage ~= 3 then return end
    reset()
    v.assin_q, v.assin_q2 = 3, 0
    save_point(MAP, 182, 180)
    page(T, "Good job! There's only one test", "left. Good luck!!")
    close()
    warp("in_moc_16", 181, 183)
  end,
}

-- ------------------------------------------------------------ the Guild Master

local GM = "[Guild Master]"
local END = { "huey", "kai", "nameless", "barcardi", "observer", "thomas" }

npc {
  map = MAP, x = 182, y = 169, dir = 1, sprite = 45, name = "sinWarp3", id = "assassin_exit3", touch = { 1, 1 },
  src = SRC .. "1437",
  on_click = function()
    -- eAthena: "busy with another Assassin Candidate" while someone is inside
    for _, e in ipairs(END) do disable_npc("assassin_end_" .. e) end
    warp("in_moc_16", 167, 116)
  end,
}

npc {
  map = MAP, x = 167, y = 114, dir = 3, sprite = -1, name = "guildmaster_Trig", id = "assassin_gm_trig", touch = { 4, 0 },
  src = SRC .. "1460",
  on_click = function()
    if not thief() or q() ~= 3 then return end
    page(GM, "Welcome, ^5533FF" .. player.name .. "^000000. This is", "my private room.")
    next()
    page(GM, "I am going to give you a simple", "test. All you have to do is come", "and meet me. Through a maze of", "invisible walls, of course...")
    next()
    page(GM, "Hahah... It's not as hard as it", "sounds. The maze is really there", "to keep intruders out of the", "guild, but it tests a candidate's", "determination as well.")
    next()
    page(GM, "Well, enough chit-chat. I look", "forward to meeting you at the end", "of the maze.")
    close()
  end,
}

local function questions1()
  page(GM, "First of all, what do you think", "matters most to an Assassin?")
  next()
  local c = menu("More power.", "An Assassin's pride.", "Endless training.")
  if c == 1 then
    page(GM, "It is true that an Assassin is", "stronger than a Thief. As an", "Assassin you can do things no", "mere Thief could dream of.")
    next()
    page(GM, "But why do you want that kind of", "power?")
    next()
    local s = menu("To take revenge on those who hurt me.", "To become wealthy.", "To explore new places.")
    if s == 1 then
      page(GM, "Vengeance, huh... It seems", "everyone has enemies these days...")
      next()
      page(GM, "But vengeance is fuelled by", "emotion, and an Assassin cannot", "give in to emotion. He must be", "cold-blooded, even heartless.")
      next()
      page(GM, "Emotions are an Assassin's biggest", "threat: they cloud the mind. An", "Assassin needs a clear head to", "succeed.")
      next()
      page(GM, "If you are to be an Assassin, find", "a way to control your emotions.", "Do not let them control you.")
    elseif s == 2 then
      page(GM, "Money matters in the world, but", "it means nothing to an Assassin.", "We carry out the dark, unpleasant", "duties no one else can.")
      next()
      page(GM, "True Assassins do not think about", "fame or fortune.")
    else
      page(GM, "An interesting idea... You want to", "learn more about the world, and", "knowing the world, you know", "yourself better.")
      next()
      page(GM, "But remember: your path will be", "lonely. Every Assassin must face", "that.")
      next()
      page(GM, "Those who succeeded as Assassins", "did not only accept it... they", "learned to embrace it.")
    end
  elseif c == 2 then
    page(GM, "Pride, you say? Did another", "Assassin tell you that?...")
    next()
    page(GM, "Pride without ability means", "nothing. Hmm... what do you hope", "to find by becoming an Assassin?")
    next()
    page(GM, "Most of those you met in this", "guild are my brethren. They have", "struggled and sacrificed a great", "deal: that is why they can be", "proud of being Assassins.")
    next()
    page(GM, "You have not yet faced what", "Assassins face. What, then, do you", "mean by 'an Assassin's pride'?")
    next()
    local s = menu("The idea of total solitude.", "The ability to make easy money.", "They just look cool.")
    if s == 1 then
      page(GM, "An Assassin's life is lonesome,", "true. In a way, to be an Assassin", "is to not exist...")
      next()
      page(GM, "We never meet our clients. The", "people we do meet... we kill.")
      next()
      page(GM, "Only Assassins know about", "Assassins... and even we hide our", "true selves from each other...")
      next()
      page(GM, "Still, comrades are better than", "nothing. Have at least one partner", "to back you up on a mission.")
    elseif s == 2 then
      page(GM, "Yes, money is important. But don't", "you think we Assassins have much", "more important things to deal", "with?")
    else
      page(GM, "...So you think Assassins look", "cool... ~sigh~ This is really sad.", "Believe me, Assassins are not", "here merely to look good...")
      next()
      page(GM, "...but to do the dark deeds no one", "else wants to do, or can.")
    end
  else
    page(GM, "Interesting... I see you have", "already trained a great deal. I", "suppose more couldn't hurt...")
    next()
    page(GM, "Is there anything in particular", "you would like to improve?")
    next()
    local s = menu("Techniques.", "More challenging targets.", "Mental toughness.")
    if s == 1 then
      page(GM, "As an Assassin you will have many", "new skills. The harder you train,", "the better you'll use them.")
      next()
      page(GM, "A great Assassin uses his skills", "masterfully.")
    elseif s == 2 then
      page(GM, "Challenging yourself is a great way", "to improve. Facing the same", "targets over and over only makes", "you weaker.")
      next()
      page(GM, "Good Assassins always find ways to", "challenge themselves.")
    else
      page(GM, "That is probably what an Assassin", "most needs to work on. Not only a", "sound body, but a sound mind.")
      next()
      page(GM, "Assassins are always under", "pressure, in situations most", "people could not imagine. They", "must be mentally tough.")
      next()
      page(GM, "That is why a true Assassin", "constantly trains his mind.")
    end
  end
end

local function questions2()
  page(GM, "So let me ask you this... If you", "became an Assassin right now, what", "would you do first?")
  next()
  local c = menu("Go into battle!", "Go back to the ones waiting for me.", "Learn more about Assassins.")
  if c == 1 then
    page(GM, "Battle... is that all?")
    next()
    local s = menu("I would level up quickly.", "I would test my new abilities.", "I would go explore new places.")
    if s == 1 then
      page(GM, "Being an Assassin shouldn't change", "how fast you train. You'd gain the", "same experience as a Thief. So", "take it slowly, okay?")
    elseif s == 2 then
      page(GM, "Testing your new skills is good. I", "understand the excitement of new", "abilities.")
      next()
      page(GM, "Just don't let it get in the way", "of your duties.")
    else
      page(GM, "Exploring is good. Find new places", "and meet new people. You never", "know when you'll meet a", "prospective client.")
    end
  elseif c == 2 then
    page(GM, "Someone's waiting for you?")
    next()
    local s = menu("Yes, the people I met in this guild.", "A friend and partner.", "The love of my life.")
    if s == 1 then
      page(GM, "Haha. I think they'd be happy to", "know you're thinking of them.", "Wherever you go, you'll always", "have a place in this guild.")
    elseif s == 2 then
      page(GM, "Good friends are hard to find, and", "partners rarer still. It's good", "you have a friend you can trust", "and work with.")
    else
      if player.sex == 1 then
        page(GM, "Oh, you have a girlfriend... then", "become her shadow, and protect", "her.")
      else
        page(GM, "Oh, you have a boyfriend... then", "become his shadow, and protect", "him.")
      end
      next()
      page(GM, "As an Assassin you'll find it very", "hard to meet anyone else, so", "treasure what you have now.")
    end
  else
    page(GM, "Okay, so what would you like to", "know?")
    next()
    local s = menu("Where's the easiest place to train?", "Where do Assassins usually go for exp?", "How does an Assassin make money?")
    if s == 1 then
      page(GM, "Wherever an Assassin is, he must", "adapt and survive. Assassins don't", "have the luxury of choosing where", "they work.")
    elseif s == 2 then
      page(GM, "Many experienced Assassins wander", "the continent of Rune-Midgard.", "Seek them out and ask for their", "knowledge.")
      next()
      page(GM, "But the best way to gain", "experience is to go out and", "experience the world yourself.")
    else
      page(GM, "Damn it, is money that important", "to you? An Assassin's life does", "not revolve around money...")
    end
  end
end

-- for job level 40, one of these at random; from job level 50, the slotted ones to choose
local REWARD_40 = { 1250, 1217, 1252 }
local REWARD_50 = { { "Jur", 1251 }, { "Katar", 1253 }, { "Main Gauche", 1208 }, { "Gladius", 1220 } }

local function interview()
  page(GM, "Ah, it's nice to meet you,", "^5533FF" .. player.name .. "^000000. I apologize for", "making you walk through that", "maze.")
  next()
  page(GM, "I saw your resume just now: it", "calls you a Thief with guts! We", "rarely find someone of your", "abilities and experience.")
  next()
  page(GM, "If you don't mind, I would like to", "ask you a few questions. Nothing", "to be nervous about. Just answer", "truthfully... if you lie, I'll kill", "you. That's all there is to it.")
  next()
  questions1()
  next()
  page(GM, "Sadly, some nitwits are eager to be", "Assassins without enough training.", "They disgrace our clan, and what", "they do often makes trouble.")
  next()
  page(GM, "You may end up like them if you are", "not careful. Remember... once you", "are an Assassin, there's no", "turning back.")
  next()
  questions2()
  next()
  page(GM, "It was nice talking with you. You", "reminded me of the good old days.")
  next()
  page(GM, "...One second...")
  next()
  page(GM, "Everyone who tested " .. player.name .. ",", "please come to my office.")
  next()
  page(GM, "They will be here soon.")
  next()
  for _, e in ipairs(END) do enable_npc("assassin_end_" .. e) end
  page(N, "I am here.")
  next()
  page(GM, "I would like to know what you think", "about ^5533FF" .. player.name .. "^000000 becoming", "an Assassin.")
  next()
  page(N, "I think ^5533FF" .. player.name .. "^000000 is a", "decent candidate.")
  next()
  page(GM, "Looks like the Nameless One more", "or less supports you. What are", "your thoughts, Huey?")
  next()
  page("[Huey]", "I think ^5533FF" .. player.name .. "^000000 will", "be fine as an Assassin.")
  next()
  page("[Huey]", "I have to get back, so I'll", "leave first.")
  next()
  page(GM, "Okay then, thank you, Huey.")
  next()
  page(GM, "So... Observer, what is your", "opinion?")
  disable_npc("assassin_end_huey")
  next()
  page("[Observer]", "...Well, not one of the better", "performances I've seen, but", "^5533FF" .. player.name .. "^000000 did pass, so I have", "no objections.")
  next()
  page(GM, "Good. It seems we all agree. I see", "no problems with ^5533FF" .. player.name .. "^000000", "myself...")
  next()
  page(GM, "^5533FF" .. player.name .. "^000000 seems to know a", "lot about Assassins, and I don't", "think the solitary life will be", "too hard either.")
  next()
  page(GM, "...I think I've said enough. Here,", "please take this...")
  next()
  -- eAthena: the choice for any form but 40 (one who skipped Kai had none)
  if v.assin_jl == 50 then
    local names = {}
    for i, r in ipairs(REWARD_50) do names[i] = r[1] end
    local c = menu(table.unpack(names)) or 1
    give_item(REWARD_50[c][2], 1)
  else
    give_item(REWARD_40[rand(1, #REWARD_40)], 1)
  end
  page(GM, "Now go and talk to ^5533FFAssassin Huey^000000", "at the guild entrance. For some", "reason he always looks ^5533FFangry^000000", "about something...")
  next()
  page(GM, "He will be the one to make you", "an Assassin.")
  next()
  give_item(NECKLACE, 1)
  v.assin_q, v.assin_q2 = 4, 0
  save_point(MAP, 18, 28)
  page(GM, player.name .. " has chosen the path of", "the Assassin. May you endure the", "hardships all Assassins must", "face.")
  next()
  page(GM, "Okay everyone, you can leave now.", "I will send you back to the", "entrance. Let's move...")
  for _, e in ipairs(END) do disable_npc("assassin_end_" .. e) end
  close()
  warp("in_moc_16", 18, 28)
end

npc {
  map = MAP, x = 149, y = 80, dir = 4, sprite = 106, name = "Guild Master", id = "assassin_gm", touch = { 1, 1 },
  src = SRC .. "1479",
  on_click = function() end,
  on_touch = function()
    if thief() and q() == 3 then interview() end
  end,
}

-- the maze's voices
local MAZE = { -- cell, eAthena line, words
  { 170, 90, 1837, { "There's no way through there. Try", "another way." } },
  { 150, 85, 1843, { "...Getting warmer... you're almost", "there..." } },
  { 153, 85, 1849, { "You can't walk there... Good,", "you're almost there, keep walking.", "You have to walk between a pole", "and a... you have to turn around", "to get in." } },
  { 160, 85, 1858, { "There's no way through there. Try", "another way." } },
  { 175, 89, 1864, { "That way leads to Botogun...", "What are you going to do there?" } },
  { 164, 88, 1871, { "Good... you're on the right track." } },
  { 149, 95, 1877, { "Good, you found the path." } },
  { 180, 101, 1883, { "...It looks like it's not that", "way..." } },
  { 186, 107, 1889, { "...Not that way either..." } },
}
for i, m in ipairs(MAZE) do
  npc {
    map = MAP, x = m[1], y = m[2], dir = 0, sprite = -1, name = "maze_trig", id = "assassin_maze_" .. i, touch = { 0, 0 },
    src = SRC .. m[3],
    on_click = function()
      page(GM, table.unpack(m[4]))
      close()
    end,
  }
end

npc {
  map = MAP, x = 186, y = 81, dir = 0, sprite = 55, name = "Guild Master", id = "assassin_gm_decoy",
  src = SRC .. "1900",
  on_click = function()
    page("[Guild Master Botogun]", "I'm sorry, but I am not the one in", "charge here. The real Guild Master", "is on the other side.")
    close()
  end,
}

-- the testers, called to the Guild Master's room. eAthena's cells for Kai,
-- Observer and Thomas (156,83, 156,77, 156,75) are walls here: the nearest floor
local MEMBERS = {
  { "huey", "Huey", 156, 85, 55, 1912 }, { "kai", "Kai", 156, 84, 730, 1913 },
  { "nameless", "Nameless One", 156, 81, 106, 1914 }, { "barcardi", "Barcardi", 156, 79, 725, 1915 },
  { "observer", "Observer", 155, 77, 55, 1916 }, { "thomas", "Thomas", 155, 75, 118, 1917 },
}
for _, m in ipairs(MEMBERS) do
  npc {
    map = MAP, x = m[3], y = m[4], dir = 1, sprite = m[5], name = m[2], id = "assassin_end_" .. m[1],
    src = SRC .. m[6], on_click = function() end,
  }
end

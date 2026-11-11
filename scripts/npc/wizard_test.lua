-- The Wizard test: things to bring, ten questions, and three rooms of
-- monsters on job_wiz.
--
-- The map: job_wiz is in 2003-04-30gdata_turtle_ph_2.gpf, one of the two
-- Turtle Island patch files on the CD, which euRO applied in January 2004,
-- in the weeks its "2nd job quests" arrived (research/THREADS.md #8). Every
-- cell below is walkable on it (research/gat_cells.py on the merged GRF).
--
-- Sources, all tier athena2003 (emulator scripts, fan translation):
--   * eAthena 817 npc/jobs/wizard.txt (2004-02-19; "Translated by yoshiki,
--     Athena Conversion by RoVeRT"): the steps, the words, the questions,
--     the monsters and their cells, the times. 817 could not run it: the
--     conversion stops halfway (`Job_Magician`, `cmdothernpc`, `enablearena`,
--     missing gotos), so 817 itself was played with 659's one-line guild
--     NPC, and so were we until now.
--   * eAthena 0.5.2 npc/jobs/wizard.txt (2004-05) and jAthena //0933
--     conf/npc/npc_job_wizard.txt (2004-04-09): the same script finished
--     for engines that had what it needs. Used to see what 817's half meant
--     (which answer is the right one where a label is misspelt, what the
--     hint costs, what happens on a retry). 0.5.2's config still calls it
--     "totaly bugged" and loads a one-line NPC instead.
--
-- What is ours (docs/DEVIATIONS.md):
--   * no waiting room. The official test took one candidate at a time: the
--     others sat in an NPC chat room and were called in turn. Alone, you
--     talk to the Helper and go in;
--   * the ten warp tiles mod0631 has on job_wiz (conf/npc_warp25.txt:440,
--     "to job_wiz 0,0": a random cell) are removed, as jAthena removed
--     them when it added this test: they are the rooms' doors, and the
--     test moves you itself;
--   * pages are cut to the dialogue window, typos mended, and one aside of
--     the translator's about Gravity's web site is left out;
--   * mended from the sources: the elemental stones are tested with "or"
--     (all three scripts end the test with an "and", so four stones of one
--     kind could do); Cartlin takes the Apple Juice she asks for before
--     giving her hint (817 and 0.5.2 jump to a label that is not there);
--     "Wind 200% on Water 3" is the right answer (817 marks 175%; 0.5.2
--     and mod0631's db/attr_fix.txt say 200%); the Fire room's guards are
--     cleared when their time runs out (0.5.2 clears the Earth room's).
--   * the Fire room's guards get two minutes as in 817; 0.5.2 and jAthena
--     give one.

for _, w in ipairs { { 99, 160 }, { 128, 169 }, { 114, 154 }, { 60, 165 }, { 48, 177 },
  { 115, 82 }, { 100, 97 }, { 116, 97 }, { 116, 113 }, { 131, 98 } } do
  remove_warp("job_wiz", w[1], w[2])
end

local RED_GEM, BLUE_GEM, YELLOW_GEM = 716, 717, 715
local RED_BLOOD, CRYSTAL_BLUE, WIND_OF_VERDURE, GREEN_LIVE = 990, 991, 992, 993
local APPLE_JUICE, BLUE_POTION = 531, 505
local MAP = "job_wiz"

-- v.job_wizard_q: 1 gemstones asked, 2 stones asked, 3 things brought,
-- 4 questions failed, 5 questions passed, 6 sent to the rooms, 7 passed

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

-- ------------------------------------------------------------ Cartlin

local C = "[Cartlin]"

local function cartlin_hint()
  page(C, "Mmmmmmmmmmmmm, apple juice.", "Slurp Slurp")
  next()
  page(C, "Thx, I'll give you the hint now.")
  next()
  page(C, "The man gives you 3 major types of questions.", "Questions about spells, monsters, and the caster.")
  next()
  page(C, "It's up to him to pick the questions.", "If he shaved and cut his hair, he might look cool >_>", "But obviously he won't do it -.-;")
  next()
  page(C, "Now, the questions about spells: they're obviously about the spells you've learned.")
  next()
  page(C, "There WILL be questions about spells you don't know much about, so go ask other wizards =P")
  next()
  page(C, "The monster questions: you COULD just go out there and fight them to learn about them.")
  next()
  page(C, "Hmm, if I remember right, there's a library in Prontera that has lots of information.", "It's easier to just go there and read about them >_>")
  next()
  page(C, "Now, the questions about wizards.....", "You're on your own for this one.", "Why? Because YOU'RE a mage.")
  next()
  page(C, "If you don't know anything about yourself, how would a stranger like me know?")
end

npc {
  map = "gef_tower", x = 111, y = 37, dir = 4, sprite = 70, name = "Wizard's Guild Worker",
  replaces = "659:jobs/npc_wizard.txt:5", src = "817:jobs/wizard.txt:5",
  on_click = function()
    local c = player.class
    if c == Job_Wizard then
      if player.sex == 1 then
        page(C, "What do you need, comrade Wizard?", "If nothing, please leave..")
        next()
        page(C, "Of course, asking me out is another thing, heh.")
      else
        page(C, "What does a wizard need of us?", "Please leave.")
        next()
        page(C, "If you get yourself a boyfriend, return though. XD.")
      end
      return close()
    end
    if c ~= Job_Mage then
      if c == Job_Novice then
        page("[Guild Clerk]", "Awwww, a little novice!", "How cute! This place is for wizards only!")
        next()
        page("[Guild Clerk]", "If you're interested in magic, the ^0000FFMagician's^000000 Guild is the place for you.")
      else
        page("[Guild Clerk]", "Hmm? What brings you to climb all the way up here?", "If you have nothing to do here, leave.",
          "This place is for wizards only =P")
      end
      next()
      page("[Guild Clerk]", "Don't fall while going down!")
      return close()
    end

    local q = v.job_wizard_q
    if q == 0 then
      page("[Guild Clerk]", "Hey! What brings you all the way up here?", "You want to be a wizard, right?")
      next()
      if menu("Of course!", "Not really") ~= 1 then
        page("[Guild Clerk]", "What? Nothing?")
        return close()
      end
      page("[Guild Clerk]", "Of course, that's always the reason.", "I'll guide you along!")
      next()
      page("[Guild Clerk]", "My name's Cartlin Medichi, I'm a noob wizard -.-;", "Relax, you can call me Cartlin.")
      next()
      page(C, "Even though many people want to be wizards, many fail because of the test.")
      next()
      page(C, "Job level 40 is required like all 2nd jobs.", "Besides, any below that and you won't have enough magic power.")
      next()
      page(C, "Well, I'll tell you about the test. After you register.")
      next()
      if menu("Sign the form", "Let me think about it.....") ~= 1 then
        page(C, "Ok then, think about it hard!", "Applications are always open!")
        return close()
      end
      if player.job_level < 40 then
        page(C, "Hey, I told you. You have to be at least job lvl 40.", "Don't you remember me telling you -.-;")
        next()
        page(C, "You're not in a hurry, go train more.", "I'll let you apply after job 40.")
        return close()
      end
      if player.skill_point ~= 0 then
        page(C, "Everything looks good, but it seems you have skill points left over.", "Reapply after spending them all.")
        return close()
      end
      page(C, "Okay, I'll take good care of this form.", "Your name is " .. player.name .. ", right?")
      next()
      page(C, "The Wizard test is divided in 3 parts.", "1st is getting a magical item.")
      next()
      page(C, "2nd is the Q&A exam,", "and 3rd is our fave, the field exam.")
      next()
      page(C, "People at job lvl 50 can skip the first test.", "They already worked hard enough.")
      next()
      if player.job_level >= 50 then
        page(C, "Wow! You're job level 50!", "You must have worked hard! I'm impressed!")
        next()
        page(C, "Okay, I'll record that you're job 50.", "You get to skip the first test, but you still got 2 left.")
        next()
        page(C, "Now, talk to that man in the corner for the test.", "It's hard, so be careful.")
        v.job_wizard_q = 3
        return close()
      end
      page(C, "Okay, you're applied. I'll tell you the items you need to bring us.", "Remember these!")
      next()
      page(C, "The first test is to bring magical items.", "Write these down or something!")
      next()
      mes(C)
      mes "The items you need to bring are............"
      if rand(2) == 0 then
        v.job_wizard_q = 1
        mes "10 Red Gemstones, 10 Blue Gemstones and 10 Yellow Gemstones."
      else
        v.job_wizard_q = 2
        mes "5 Crystal Blues, 5 Green Lives, 5 Red Bloods and 5 Winds of Verdure."
      end
      next()
      page(C, "This test isn't THAT hard, so go!")
      next()
      page(C, "So, good luck!", "I'll be waiting!")
      return close()
    end

    if q == 1 or q == 2 then
      local needs = q == 1 and { { RED_GEM, 10 }, { BLUE_GEM, 10 }, { YELLOW_GEM, 10 } }
        or { { CRYSTAL_BLUE, 5 }, { GREEN_LIVE, 5 }, { RED_BLOOD, 5 }, { WIND_OF_VERDURE, 5 } }
      page(C, "Did you get all the items?", "Let's see......")
      next()
      local all = true
      for _, it in ipairs(needs) do all = all and count_item(it[1]) >= it[2] end
      if not all then
        page(C, "You don't seem to have everything you need.")
        next()
        page(C, "It's a waste of time to come up here without the needed items >_>", "Go get the rest of the items!")
        next()
        if q == 1 then
          page(C, "10 Red Gemstones, 10 Blue Gemstones and 10 Yellow Gemstones.", "Don't forget that we need them all!")
        else
          page(C, "5 Crystal Blues, 5 Green Lives, 5 Red Bloods and 5 Winds of Verdure.", "Don't forget to bring them!")
        end
        return close()
      end
      page(C, "Great! You've brought everything!", "The guild will use these well.")
      next()
      for _, it in ipairs(needs) do take_item(it[1], it[2]) end
      v.job_wizard_q = 3
      page(C, "Congrats, you passed the first test.", "Don't just relax yet! Still 2 tests left!")
      next()
      page(C, "Now go talk to the man in the corner for the other tests.", "They're pretty hard, so be careful.")
      return close()
    end

    if q == 3 then
      page(C, "Hey, did you go talk to the man?", "No? You can't just become a wizard by getting items >_>.")
      next()
      page(C, "There's 2 more tests left.", "I'll be waiting, so go finish them.")
      return close()
    end

    if q == 4 then
      page(C, "o.O You failed the Q&A test!?", "You want to be a wizard and you can't get them >_>")
      next()
      page(C, "Heh, do you want big sis to give you a hint?", "If so, I'm thirsty, so go buy me 1 Apple Juice =P")
      next()
      if menu("Can I have a hint?", "I can pass on my own!") ~= 1 then
        page(C, "That's right! Tests are meant to be done alone!", "That's definitely the way a wizard should think!")
      elseif count_item(APPLE_JUICE) < 1 then
        page(C, "Well, if you want hints, I want apple juice!", "Otherwise, you're on your own =P.")
      else
        take_item(APPLE_JUICE, 1)
        cartlin_hint()
      end
      next()
      page(C, "Good luck!", "Mister Examiner is waiting!")
      return close()
    end

    if q == 5 then
      page(C, "Heh, I could hear you in there.", "Working so hard on the questions.")
      next()
      page(C, "Just one test left!", "Finish that and you'll be a wizard!")
      next()
      page(C, "Good luck!", "The examiner's waiting!")
      return close()
    end

    if q == 6 then
      page(C, "Why did you come out during the test?", "I never thought you were that weak >_>")
      next()
      if player.sex == 1 then
        page(C, "How could a powerful mage like you get beaten so easily!!", "Go back in and try harder -.-")
      else
        page(C, "You can't act all weak, just because you're a girl! >_>", "If you want to be a wizard, go and pass!")
        next()
        page(C, "I thought it was hard too, but it wasn't enough to make me quit!", "You have to face hardship sometimes!")
      end
      next()
      page(C, "Good luck!", "The examiner is waiting for you!")
      return close()
    end

    -- 7: passed
    if player.skill_point ~= 0 then
      page(C, "Did you pass?", "Uh oh, you have skill points left, go spend them!")
      return close()
    end
    page(C, "Great! You finished it!", "Congrats, congrats, congrats!")
    next()
    page(C, "Ack, I'm wasting time, I better change you!")
    next()
    v.job_wizard_q = 0
    job_change(Job_Wizard)
    page(C, "Now you're a wizard! Act responsibly!", "Make sure when you cast, you know what will happen!")
    next()
    page(C, "Don't do stupid things like casting in the middle of town!",
      "It lags and magic is supposed to be used on monsters anyways >_>")
    next()
    page(C, "Become mightier by partying with others!", "Oh yea, here's a present!")
    next()
    page(C, "Rustle Rustle")
    next()
    give_item(BLUE_POTION, 6)
    page(C, "Use them wisely!", "It's a present from me, so you better drink them all!")
    next()
    page(C, "Well wizard, have a good life!")
    close()
  end,
}

-- ------------------------------------------------------------ Raul: the questions

local R = "[Raul]"

-- { question, four answers, the right one }
local QUIZ = {
  { -- spells
    { "What isn't required for Fire Wall?", { "Fire Bolt Lv 4", "Fire Ball Lv 5", "Sight Lv 1", "Napalm Beat Lv 4" }, 4 },
    { "What happens to a monster's element after it's frozen by Frost Diver?",
      { "Changes to Water", "Changes to Earth", "Changes to Fire", "Changes to Wind" }, 1 },
    { "When you max out Napalm Beat, how many times matk will the dmg be?", { "1.6x", "1.7x", "2x", "20x" }, 2 },
    { "What item is required for Stone Curse?", { "Red Blood", "Blue Gem", "Yellow Gem", "Red Gem" }, 4 },
    { "What isn't required for Safety Wall?", { "Napalm Beat Lv 4", "Soul Strike Lv 5", "SP Recovery Lv 6", "Ice Wall Lv 7" }, 3 },
    { "How much SP will you regain every 10 sec with lvl 7 SP Recovery?", { "14", "21", "28", "35" }, 2 },
    { "You have 50% SP: how much dmg will Energy Coat reduce and how much SP will it use?",
      { "dmg 18% SP 1.5%", "dmg 18% SP 2%", "dmg 24% SP 1.5%", "dmg 24% SP 2%" }, 2 },
    { "Safety Wall Lv 6: SP consumed, and no. of hits absorbed?", { "40 sp, 6 hits", "35 sp, 6 hits", "40 sp, 7 hits", "35 sp, 7 hits" }, 3 },
    { "Lv 10 Thunder Storm's SP consumption?", { "84", "74", "64", "54" }, 2 },
    { "What would be a good skill to use in the Izlude Dungeon?", { "Lightning Bolt", "Fire Bolt", "Cold Bolt", "Sight" }, 1 },
  },
  { -- monsters
    { "What drops a slotted Guard?", { "Thief Bug", "Peco Peco", "Pupa", "Mace Kobold" }, 3 },
    { "Which monster is the easiest for magicians?", { "Flora", "Giearth", "Golem", "Myst" }, 1 },
    { "What monster is completely immune to Stone Curse?", { "Elder Willow", "Evil Druid", "Magnolia", "Marc" }, 2 },
    { "How much dmg do you do with a wind spell to a Water 3 monster?", { "125%", "150%", "175%", "200%" }, 4 },
    { "Baby Desert Wolf vs Familiar, which one wins?", { "Baby Desert Wolf", "Familiar", "Tie", "Don't know" }, 1 },
    { "Which monster can't be tamed as a pet?", { "Poporing", "Roda Frog", "Smokie", "Poison Spore" }, 2 },
    { "Which monster is weak against fire?", { "Knife Goblin", "Mace Goblin", "Flail Goblin", "Hammer Goblin" }, 4 },
    { "Which monster has the highest magic def?", { "Horong", "Chonchon", "Andre", "Caramel" }, 4 },
    { "Pick the monster that doesn't belong.", { "Poring", "Mastering", "Ghostring", "Spore" }, 3 },
    { "Which is not an undead?", { "Drake", "Megalodon", "Deviace", "Khalitzburg" }, 3 },
  },
  { -- the caster
    { "Which is the most important stat?", { "INT", "AGI", "DEX", "VIT" }, 1 },
    { "Which element doesn't have a bolt spell?", { "Water", "Earth", "Fire", "Wind" }, 2 },
    { "What is NOT one of the wizard's strong points?",
      { "Weak physical abilities", "Distance attacks", "Good money maker", "High magic def" }, 3 },
    { "Which town is the home of mages?", { "Prontera", "Morroc", "Alberta", "Geffen" }, 4 },
    { "Which card has nothing to do with INT?", { "Andre Larva Card", "Soldier Andre Card", "Baby Desert Wolf Card", "Elder Willow Card" }, 2 },
    { "What is superior about the mage class compared to others?", { "Close range combat", "High dodge", "Dance skills", "Magical power" }, 4 },
    { "What is the INT bonus at job lvl 40?", { "8", "7", "6", "5" }, 4 },
    { "Which item can't be equipped by mages?", { "Knife", "Cap", "Sandals", "Eye of Dullahan" }, 2 },
    { "During the mage test, which stone is one of the catalysts?", { "Yellow Gemstone", "Red Gemstone", "Blue Gemstone", "Red Blood" }, 1 },
    { "Which card has nothing to do with magic?", { "Marduk Card", "Magnolia Card", "Willow Card", "Maya Card" }, 2 },
  },
}

-- asked of those who come back from the rooms
local RETRY_QUIZ = {
  { "Which monster doesn't exist?", { "Mantis", "Cunner Tooth", "Giearth", "Caramel" }, 2 },
  { "Which monster is NOT an animal type?", { "Yoyo", "Bigfoot", "Metaller", "Zerom" }, 4 },
  { "Which monster has no cast sense?", { "Marina", "Vitata", "Scorpion", "Giearth" }, 1 },
  { "What's a good spell to use on Marine Spheres?", { "Cold Bolt", "Fire Bolt", "Lightning Bolt", "Stone Curse" }, 3 },
  { "Which monster can move?", { "Hydra", "Mandragora", "Greatest General", "Frilldora" }, 4 },
}

local function ask(list, each)
  local score = 0
  for i, q in ipairs(list) do
    page(R, i .. ". " .. q[1])
    next()
    if menu(table.unpack(q[2])) == q[3] then score = score + each end
  end
  return score
end

local function questions()
  page(R, "Let's start the test!", "If you miss one, you're screwed! HA!")
  next()
  page(R, "10 questions, get them right!", "Even if you miss, I won't tell you the correct answer.")
  next()
  page(R, "Cough, here's the questions!")
  next()
  local score = ask(QUIZ[rand(3) + 1], 10)
  page(R, "Great job, when you get the time afterwards, get something to eat heh.")
  next()
  mes(R)
  mes("Your score is = " .. score)
  if score >= 90 then
    v.job_wizard_q = 5
    mes(score == 100 and "Wonderful! You've passed the 2nd test." or "Hmm, you've missed one question, but I'll pass you.")
    next()
    page(R, "Don't relax yet! There is still a 3rd test left.", "While we prepare for the next test, sit and rest a bit.")
  else
    v.job_wizard_q = 4
    mes "Failed, go study more."
    next()
    page(R, "You're a long way away.", "With that low intelligence, you have no chance of surviving as a wizard.")
  end
  close()
end

local function briefing()
  page(R, "The test is in 3 stages.", "There are rooms in this order: Room of Water, of Earth, and of Fire.")
  next()
  page(R, "There will be monsters of the elements in their respective rooms.", "Use the right spells, and it won't be that difficult.")
  next()
  page(R, "Within the time limit, kill all the monsters,", "then kill the monsters guarding the exit.")
  next()
  page(R, "Go through the exit and the test will be over.", "Succeed and you'll become a master of magic, a wizard.")
  next()
  page(R, "You seem afraid, it's not too late to return....", "Give up now and you can return to town.")
  next()
end

npc {
  map = "gef_tower", x = 102, y = 24, dir = 2, sprite = 735, name = "Wizard",
  src = "817:jobs/wizard.txt:437",
  on_click = function()
    local c = player.class
    if c == Job_Wizard then
      page(R, "Cough Cough, what can I do for you?", "Hmmm, you're a magic user too...")
      next()
      if player.sex == 1 then
        page(R, "If you're not responsible, you might accidentally learn useless spells.",
          "Instead of that, you might want to focus on learning to use your spells now.")
        next()
        page(R, "In a fight for love, you might become a cripple hehe.")
      else
        page(R, "If you're not responsible, you might accidentally learn useless spells.",
          "You should learn it with all your heart, otherwise, just quit now.")
        next()
        page(R, "If you don't wish to study like that...............", "Heh, go be a housewife.")
      end
      return close()
    end
    if c == Job_Priest then
      page(R, "Leave, holy ones!", "Magic is considered a power not from God.......")
      next()
      page(R, "Besides, I'm not feeling too good.", "Leave now!")
      return close()
    end
    if c == Job_Novice then
      page(R, "What's a little kid like you doing here?", "Get out of here!")
      close()
      return warp("geffen", 120, 110)
    end
    if c ~= Job_Mage then
      page(R, "Ugh I don't feel good....I feel like I'm trapped in ice........")
      next()
      page(R, "Who're you?! Leave!")
      return close()
    end

    local q = v.job_wizard_q
    if q == 0 then
      page(R, "Cough, I don't have any time to talk.")
      next()
      page(R, "Sorry, but please leave.")
      return close()
    end
    if q == 1 or q == 2 then
      page(R, "Heh, a wizard trainee eh?!")
      next()
      page(R, "You COULD just stay as a mage and live a pretty decent life.....")
      next()
      page(R, "Then again, wizards ARE much more upper class.")
      return close()
    end
    if q == 3 then
      page(R, "I see you've passed the 1st test.", "Yes, I'm your test examiner. My name is Raul Expagarus.")
      next()
      page(R, "It's not too late! You can still return to a peaceful life in town if you wish.")
      next()
      page(R, "You probably have no clue how dangerous it is to use magic.")
      next()
      if menu("I want to live as a regular magician", "I want to continue the test!") ~= 2 then
        page(R, "Good choice, cough cough.", "Wizard is a difficult job, magic is not meant for humans to use.........")
        next()
        page(R, "Go down the tower and don't look back.", "Use the magic you have now and live a peaceful life.")
        return close()
      end
      page(R, "HA! Here's a little idiot.", "Hmph, I'll see how GREAT you do.")
      next()
      return questions()
    end
    if q == 4 then
      page(R, "Hmph, you want to try again?")
      next()
      page(R, "How can you hate a peaceful life so much?")
      next()
      page(R, "Be that way! If you miss one question, you fail! What do you say!?")
      next()
      if menu("I want to live in peace..........", "I want to continue.") ~= 2 then
        page(R, "Good choice.", "Wizard is a difficult job, magic was never meant for people to use.")
        next()
        page(R, "Go down and don't look back.", "Use what you have now and live in peace.")
        return close()
      end
      page(R, "Heh, another idiot!", "Let's see how great you are!")
      next()
      return questions()
    end
    if q == 5 then
      page(R, "Got some rest?", "Now, let's start the last test.")
      next()
      page(R, "I'll make a quick explanation of the test, want to hear it?")
      next()
      if menu("No thank you.", "Please go on") ~= 2 then
        page(R, "Ignorant fool, go die!", "Your corpse will be left as the monsters' food.")
        next()
        v.job_wizard_q = 6
        save_point("geffen", 120, 107)
        page(R, "Be that way, I'll send you now.", "Have fun getting slaughtered!")
        close()
        return warp(MAP, 57, 154)
      end
      page(R, "Good, a smart one, I will explain now.", "It may be hard, but do as you're asked and you'll pass.")
      next()
      briefing()
      v.job_wizard_q = 6
      if menu("I want to take the test.", "IM SCARED~ I WANNA GO HOME >_>") == 2 then
        page(R, "Good choice, it fits you better.", "Have a nice peaceful life!")
        close()
        return warp("geffen", 120, 110)
      end
      save_point("geffen", 120, 107)
      page(R, "Kid, you have courage.", "We'll start as you wish!!")
      close()
      return warp(MAP, 57, 154)
    end
    if q == 6 then
      page(R, "You want to take the test AGAIN!?", "You love magic THAT much!?")
      next()
      page(R, "You failed, so it's time for a quiz!", "If you don't like this, then pass the 3rd test.")
      next()
      page(R, "Let's start now hehe.")
      next()
      local score = ask(RETRY_QUIZ, 20)
      page(R, "Do it right so I won't have to quiz you again.")
      next()
      mes(R)
      mes("Your score is " .. score)
      if score < 80 then
        mes "Failed! Go study more!"
        next()
        page(R, "You aren't fit to be a wizard, why do you think you failed!?")
        return close()
      end
      mes(score == 100 and "If you're so smart, why can't you pass the test >_>" or "Kehe, almost perfect.")
      mes "I'll let you try again."
      next()
      if menu("Let's start!", "Explain the test again please.") == 2 then
        page(R, "Cough, I'll give you a briefing then.", "Even if it seems hard, just do as you're asked, and you'll pass.")
        next()
        briefing()
        if menu("I want to do the test.", "I'M AFRAID! SEND ME BACK HOME!! >_>") == 2 then
          page(R, "Good thought, it's much more fitting.", "Have a nice and ordinary life!")
          close()
          return warp("geffen", 120, 110)
        end
        percent_heal(100, 100)
        page(R, "Don't make me send you there again!", "Pass the test!")
      else
        page(R, "I won't help you even if you die this time.", "Don't come crying to me if you fail.........again hehe.")
        next()
        percent_heal(100, 100)
        page(R, "I'll send you to the arena now.", "Have fun getting your butt whupped!")
      end
      close()
      return warp(MAP, 57, 154)
    end
    -- 7
    page(R, "What do you want now?", "If you want, I'll let you take the test again, keke.")
    next()
    page(R, "Go, become a wizard.", "But be careful! Magic can always destroy you!")
    close()
  end,
}

-- ------------------------------------------------------------ the rooms

-- area: the room, for sending everyone in it somewhere; then the monsters
-- to clear in three minutes and the gate guards that follow them
local ROOMS = {
  {
    name = "Water", area = { 95, 150, 135, 190 }, guard_time = 60,
    mobs = { { 109, 174, 1158 }, { 118, 174, 1070 }, { 109, 165, 1066 }, { 118, 165, 1067 }, { 101, 157, 1141 }, { 126, 157, 1242 },
      { 98, 170, 1070 }, { 129, 170, 1044 } },
    guards = { { 114, 169, 1142 }, { 112, 169, 1068 }, { 116, 169, 1068 }, { 114, 171, 1068 }, { 114, 167, 1068 } },
    exit = { 116, 97 },
  },
  {
    name = "Earth", area = { 95, 75, 135, 120 }, guard_time = 60,
    mobs = { { 111, 102, 1057 }, { 120, 102, 1105 }, { 111, 102, 1103 }, { 120, 102, 1127 }, { 111, 93, 1166 }, { 120, 93, 1121 },
      { 107, 98, 1060 }, { 124, 98, 1023 }, { 104, 86, 1176 }, { 127, 86, 1139 } },
    guards = { { 116, 97, 1118 }, { 114, 95, 1020 }, { 118, 95, 1020 }, { 114, 99, 1020 }, { 118, 99, 1020 }, { 116, 94, 1020 },
      { 116, 100, 1020 } },
    exit = { 46, 99 },
  },
  {
    name = "Fire", area = { 25, 75, 65, 120 }, guard_time = 120,
    mobs = { { 58, 110, 1178 }, { 33, 110, 1123 }, { 40, 103, 1001 }, { 51, 103, 1119 }, { 40, 92, 1019 }, { 51, 92, 1033 },
      { 37, 89, 1058 }, { 54, 89, 1106 } },
    guards = { { 44, 99, 1277 }, { 43, 99, 1129 }, { 45, 99, 1129 } },
  },
}

local function ids(i) return "jobwiz_" .. i, "jobwiz_" .. i .. "g" end

-- the stage whose monsters are out, and who is being tested. Several
-- monsters can die to one spell: only the first death event that finds the
-- room empty may move the test on
local live, candidate = nil, "The candidate"

-- clear the rooms of whatever an earlier attempt left
local function reset()
  for i in ipairs(ROOMS) do
    local room, guard = ids(i)
    npc_timer_stop(room)
    npc_timer_stop(guard)
    kill_monster(MAP, room .. "::OnKilled")
    kill_monster(MAP, guard .. "::OnKilled")
  end
  npc_timer_stop("jobwiz_done")
  live = nil
end

local function call(list, id)
  for _, m in ipairs(list) do monster(MAP, m[1], m[2], "--ja--", m[3], 1, id .. "::OnKilled") end
  live = id
  npc_timer_start(id)
end

local function start_room(i)
  call(ROOMS[i].mobs, (ids(i)))
end

local function fail(room)
  live = nil
  map_announce(MAP, candidate .. " has failed the test!", 0)
  area_warp(MAP, room.area[1], room.area[2], room.area[3], room.area[4], "geffen", 120, 110)
end

-- the countdown both kinds of stage share: { ms, text } up to the limit
local function countdown(first, limit)
  local t = { { 1000, first } }
  if limit == 180 then
    t[#t + 1] = { 2000, "3 min left" }
    t[#t + 1] = { 3000, "Kill all the monsters within the time limit." }
    for _, s in ipairs { { 150, "2min 30sec left." }, { 120, "2min left" }, { 90, "1min 30sec left" }, { 60, "1min left." },
      { 30, "30sec left." }, { 10, "10 seconds!" } } do
      t[#t + 1] = { 3000 + (180 - s[1]) * 1000, s[2] }
    end
    t[#t + 1] = { 183000, "Out of Time" }
  else
    for _, s in ipairs { { 90, "1min 30sec left!" }, { 60, "1min left!" }, { 30, "30sec left!" }, { 10, "10sec left!" } } do
      if s[1] < limit then t[#t + 1] = { (limit - s[1]) * 1000, s[2] } end
    end
    t[#t + 1] = { limit * 1000, "Out of time!" }
  end
  return t
end

local function stage(i, id, texts, on_clear)
  local room = ROOMS[i]
  local at = {}
  local events = { "OnKilled" }
  for _, s in ipairs(texts) do
    at["OnTimer" .. s[1]] = s[2]
    events[#events + 1] = "OnTimer" .. s[1]
  end
  local over = texts[#texts][1]
  events[#events + 1] = "OnTimer" .. (over + 1000)
  npc {
    map = "-", name = id, sprite = -1, events = events, src = "817:jobs/wizard.txt:1070",
    on_click = function(label)
      if label == "OnKilled" then
        if live ~= id or mob_count(MAP, id .. "::OnKilled") > 0 then return end
        live = nil
        npc_timer_stop(id)
        return on_clear()
      end
      if label == "OnTimer" .. (over + 1000) then return fail(room) end
      if at[label] then map_announce(MAP, at[label], 0) end
      if label == "OnTimer" .. over then kill_monster(MAP, id .. "::OnKilled") end
    end,
  }
end

for i, room in ipairs(ROOMS) do
  local id, guard = ids(i)
  stage(i, id, countdown("Room of " .. room.name .. ", testing will begin.", 180), function()
    map_announce(MAP, player.name .. " has killed all the monsters!", 0)
    call(room.guards, guard)
  end)
  local first = room.guard_time == 60 and "The gate guards have appeared! 1 min left!" or "Gate guards have appeared! 2 min left!"
  stage(i, guard, countdown(first, room.guard_time), function()
    if room.exit then
      map_announce(MAP, player.name .. " has successfully defeated the guards!", 0)
      warp(MAP, room.exit[1], room.exit[2])
      percent_heal(100, 100)
      return start_room(i + 1)
    end
    v.job_wizard_q = 7
    map_announce(MAP, "Congratulations, " .. player.name .. ". You passed the test!", 0)
    npc_timer_start("jobwiz_done")
  end)
end

npc {
  map = "-", name = "jobwiz_done", sprite = -1, events = { "OnTimer4000", "OnTimer5000" }, src = "817:jobs/wizard.txt:1533",
  on_click = function(label)
    if label == "OnTimer4000" then return map_announce(MAP, "The test is now over.", 0) end
    local a = ROOMS[3].area
    area_warp(MAP, a[1], a[2], a[3], a[4], "gef_tower", 110, 30)
  end,
}

npc {
  map = MAP, x = 50, y = 165, dir = 4, sprite = 700, name = "Helper",
  src = "817:jobs/wizard.txt:1020",
  on_click = function()
    page("[Helper]", "Welcome to the testing grounds!", "Please go to the waiting room..")
    next()
    page("[Helper]", "Please wait if there is someone in front of you.",
      "Progress will be announced periodically, so just relax until your turn.")
    next()
    page("[Helper]", "1 person usually takes 5 - 10 min.", "It shouldn't take too long.")
    next()
    -- ours: nobody is in front of you
    page("[Helper]", "...But there is nobody in front of you today.", "Shall I let you in?")
    next()
    if menu("I am ready.", "Not yet.") ~= 1 then
      page("[Helper]", "Take your time.")
      return close()
    end
    close()
    reset()
    candidate = player.name
    warp(MAP, 114, 169)
    start_room(1)
  end,
}

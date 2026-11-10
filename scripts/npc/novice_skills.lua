-- The Novice quest skills: First Aid and Play Dead.
--
-- Source: mod0631 conf/npc_event_skillget.txt:1538 ("ナミ", a trainee nurse
-- in a Prontera inn), :1717 ("プロンテラ騎士団員", in the Chivalry) and :1890
-- ("看護将校", in Prontera Castle), archived 2003-12-18, the day this client
-- was built. Tier athena2003, jRO lineage. Places, sprites, the order of
-- the talk, the items and the variable (skill_nov, 0 to 6) are mod0631's.
-- The English is ours, translated from its Japanese; skill and item names
-- are the client's own. 817 has neither quest.
--
-- A second, independent witness for the international servers: RODataZone's
-- Novice page as captured on 2003-12-15 (tier period;
-- web.archive.org/web/20031215225409/http://www.rodatazone.com/jobs/0/novice.php).
-- It has the same three NPCs on the same cells ("Nami ... 234/133", the
-- "Prontera Chivalry Member ... 73/87", the "Nursing Instructor ... 175/146"),
-- "3 Red Herbs, 3 Clover and 1 Sterilized Bandages", and "you have 10 Minutes
-- to get a Newbie Tag". The NPC names are from there.
--
-- Three places where this is not mod0631's script as written
-- (docs/DEVIATIONS.md, research/notes/round3-review.md):
--  * First Aid from Novice job level 4. mod0631 tests `JobLevel < 3`, while
--    its own Nami says "JOBLV4以上" and RODataZone says "Novice Job 4+".
--  * The Newbie Tag is the item (7039, in this client's tables). mod0631
--    keeps it in a temporary variable; RODataZone: "he will give you a real,
--    physical Newbie Tag item". The bandages are a flag in both ("you won't
--    get them in your inventory").
--  * The ten minutes are counted. mod0631's Chivalry member says "within 10
--    minutes" and "if you are late you will have to go again", and sends you
--    round again only when the tag is missing (its temporary variable is
--    lost on logout). Here being late, logging out, or losing the tag all
--    send you round again.
--
-- The client's dialogue window shows the name and six lines of about 30
-- characters before it scrolls. The pages are cut to fit, so there are more
-- of them than in the Japanese.

local RED_HERB, CLOVER, NEWBIE_TAG = 507, 705, 7039
local FIRST_AID, PLAY_DEAD = 142, 143
local RUN_MS = 10 * 60 * 1000

local function say(who, ...)
  mes(who)
  for _, line in ipairs { ... } do mes(line) end
end

-- ---------------------------------------------------------------- First Aid

local N = "[Nami]"

local function nami_items()
  say(N, "It is first aid, so what you need is close to hand.",
    "These are the items...",
    "^3355FF3 Red Herbs^000000, ^3355FF3 Clovers^000000",
    "and ^3355FFSterilized Bandages^000000.")
end

local function nami_bandages()
  say(N, "Sterilized Bandages are Rotten Bandages from a certain monster,",
    "soaked in disinfectant until clean.")
  next()
  say(N, "No monster drops them, no shop sells them...",
    "The Nursing Instructor on the east second floor of Prontera Castle has plenty.",
    "Ask there...")
end

npc {
  map = "prt_in", x = 234, y = 133, dir = 4, sprite = 66, name = "Nami",
  src = "m631:conf/npc_event_skillget.txt:1538",
  on_click = function()
    if player.class == Job_Novice and player.job_level < 4 then
      say(N, "... z z Z", "I got it wrong! Teacher!", "...?", "Hah!?", "- looks around -")
      return close()
    end
    if skill_level(FIRST_AID) > 0 then
      say(N, "I am studying hard for my nursing instructor's licence,",
        "but the results are poor...",
        "It must be the lack of practice...")
      next()
      say(N, "I ask people with a first job or better,",
        "and Novices of ^3355FFjob level 4 or above^000000,",
        "to help me practise.", "Hee hee...")
      return close()
    end
    say(N, "Ah, I want to become a nursing instructor,",
      "and I am doing my teaching practice.",
      "Would you give me a chance to practise...?")
    next()
    if menu("Keep talking", "Sneak away") ~= 1 then
      say(N, "Hm? Where are you going!", "Now, your injection...",
        "Heh heh heh...", "Yes, patient~")
      return close()
    end

    if v.skill_nov == 1 then
      nami_items()
      next()
      nami_bandages()
      return close()
    end

    if v.skill_nov == 2 then
      if count_item(RED_HERB) < 3 or count_item(CLOVER) < 3 then
        nami_items()
        return close()
      end
      take_item(RED_HERB, 3)
      take_item(CLOVER, 3)
      say(N, "Welcome back~ That must have been hard work.",
        "The Nursing Instructor's temper seems to have worn you out most of all...",
        "Hee hee...")
      next()
      say(N, "Now, let us begin the practice.",
        "First Aid spends your SP to restore HP:",
        "about ^3355FF3 SP^000000 for about ^3355FF5 HP^000000.")
      next()
      say(N, "You make the medicine, and the way to dress the wound is...",
        "...", "one, two, and there!", "Like this. Easy, isn't it?")
      next()
      say(N, "In more detail: a tablespoon of potion,",
        "Red Herb cut to the size of a finger,",
        "this, then that, and...", "ta-daa!!")
      next()
      give_skill(FIRST_AID, 1, 0)
      v.skill_nov = 3
      say(N, "There, all done!", "You can give proper first aid now.",
        "Make good use of it.", "Goodbye~")
      return close()
    end

    say(N, "Thank you for staying with me!",
      "I will do my best and teach you with all my heart.", "Hee hee...")
    next()
    say(N, "What I teach is ^3355FFFirst Aid^000000.",
      "No special equipment or machines:",
      "simple things, a scrap of cloth, a cheap potion,")
    next()
    say(N, "to dress a wound and recover a little HP.",
      "That is what I want to teach you.",
      "I am short of practice materials. Will you help?")
    next()
    nami_items()
    next()
    nami_bandages()
    next()
    say(N, "We need them for our practice, so please be sure to bring them.")
    v.skill_nov = 1
    close()
  end,
}

-- --------------------------------------------------------------- Play Dead

local B = "[Bloomer]"

-- The run to the castle and back: tmp.nov_run is 1 while the ten minutes
-- last. It goes with the timer, and with a logout.
local function start_run()
  v.skill_nov = 4
  tmp.nov_run = 1
  del_timer("nov_playdead::OnLate")
  add_timer(RUN_MS, "nov_playdead::OnLate")
end

npc {
  map = "prt_in", x = 73, y = 87, dir = 4, sprite = 65, name = "Prontera Chivalry Member",
  id = "nov_playdead", events = { "OnLate" },
  src = "m631:conf/npc_event_skillget.txt:1717",
  on_click = function(at)
    if at == "OnLate" then
      tmp.nov_run = 0
      return
    end
    if player.class ~= Job_Novice or player.job_level < 7 or v.skill_nov < 3 or v.skill_nov >= 6 then
      say(B, "Ah, this takes me back...", "to my days as a Novice most of all.",
        "It was hard, but I have made my place now and have time to spare.")
      next()
      say(B, "So to ^3355FFNovices^000000 of ^3355FFjob level 7 or above^000000",
        "^3355FFwho have learned First Aid^000000", "I can teach a trick.")
      next()
      say(B, "If one of your Novice friends has come that far, tell them about me.",
        "It will do them good.")
      return close()
    end
    say(B, "That's right! I am Bloomer, elite member of the Prontera Chivalry!",
      "Fwahahaha!", "I only just joined, but don't I look good? Well?")
    next()

    if v.skill_nov == 4 then
      say(B, "Come on, you are running out of time~", "You take it easy, don't you.",
        "Don't you want the skill?", "Run!")
      next()
      say(B, "The Nursing Instructor is on the second floor of the east tower of Prontera Castle!")
      return close()
    end

    if v.skill_nov == 5 then
      if count_item(NEWBIE_TAG) >= 1 and tmp.nov_run == 1 then
        say(B, "There! You brought it!", "Good, you pass the test!",
          "You held out against the capsule and the Nursing Instructor's shouting...")
        next()
        say(B, "Play dead as you are now and you would pass for a corpse.",
          "When you meet an enemy you cannot beat, decide quickly and play dead.",
          "Heh, we will meet again!")
        take_item(NEWBIE_TAG, 1)
        del_timer("nov_playdead::OnLate")
        tmp.nov_run = 0
        v.skill_nov = 6
        give_skill(PLAY_DEAD, 1, 0)
        return close()
      end
      say(B, "Now, now -", "where is the Newbie Tag?", "Did you drop it somewhere?",
        "Nothing for it... You start again from the beginning!")
      next()
      say(B, "You take this too lightly.",
        "A place in the Prontera Chivalry is not won that easily.",
        "Now pull yourself together: from the beginning!")
      if count_item(NEWBIE_TAG) >= 1 then take_item(NEWBIE_TAG, 1) end
      start_run()
      return close()
    end

    say(B, "Hey, Novice!", "Young, clear eyes like yours remind me of myself long ago...",
      "...", "Ah, I sound like an old man.")
    next()
    say(B, "I am still young myself, heh...",
      "Work at it and you will get the job you hope for.",
      "Well... talent does no harm either.")
    next()
    say(B, "So! Shall I tell you about a very handy skill I learned as a Novice!")
    next()
    say(B, "...You want to know, but you cannot hide your doubts, I see...",
      "It could be a great help to you as you are now.")
    next()
    say(B, "Oh, getting interested?", "Good. I will pass on my finishing move.",
      "They call it ^3355FFPlay Dead^000000!!!",
      "Try it and you will see: you look like a real corpse.")
    next()
    say(B, "^3355FFPlay Dead^000000 is just right for a Novice.",
      "Even so it is a serious skill and takes great patience.",
      "Staying dead is harder than you think.")
    next()
    say(B, "Say you meet a strong monster, see you are no match, and play dead.",
      "If it is a clever one and tickles you to make sure, could you bear it?")
    next()
    say(B, "To look perfectly dead in every such case takes someone at the very limit of patience...",
      "That is the one requirement of Play Dead.")
    next()
    say(B, "Well, enough explaining.", "I can see in your eyes that you mean to learn.",
      "Now for the training course. Oh yes, swallow this capsule first.")
    next()
    say(B, "Within 10 minutes, see the Nursing Instructor on the second floor of the east tower of Prontera Castle",
      "and bring back a ^3355FFNewbie Tag^000000.")
    next()
    mes "^3355FF- gulp (you swallowed the medicine) -^000000"
    next()
    say(B, "Heh, that medicine makes your breathing shallow.",
      "That is an 'I was tricked!' face.", "Don't worry, it does you no harm.")
    next()
    say(B, "I thought a long run would build your patience. This too is training.",
      "Now, off you go.", "If you are late you will have to go again. Hurry, run!")
    start_run()
    close()
  end,
}

-- ------------------------------------------------------ Nursing Instructor

local D = "[Dread]"

npc {
  map = "prt_castle", x = 175, y = 146, dir = 4, sprite = 50, name = "Nursing Instructor",
  src = "m631:conf/npc_event_skillget.txt:1890",
  on_click = function()
    say(D, "...", "......", "You distract me!",
      "Stop hanging about in front of me and go somewhere else!")
    next()
    local step = v.skill_nov
    if step >= 6 or step == 3 then
      say(D, "What...? Away with you!", "I cannot stand noise!!")
    elseif step == 5 then
      say(D, "Well, what are you gaping at!", "Run, run!",
        "A beginner's days are for pressing on!")
    elseif step == 4 then
      if count_item(NEWBIE_TAG) >= 1 then
        say(D, "Well!", "Do you still want something of me?", "One Newbie Tag is enough!",
          "Will you stay a beginner all year, you idler!!")
        return close()
      end
      say(D, "Hey!!", "(...I was just dozing off)",
        "Why are you running about the sacred Castle of Prontera!",
        "(nag, nag) What do you want!")
      next()
      say(D, "...", "...What?", "You came because you want a Newbie Tag?", "- swish -")
      next()
      mes "^3355FF- you received a Newbie Tag -^000000"
      give_item(NEWBIE_TAG, 1)
      v.skill_nov = 5
    elseif step == 2 then
      say(D, "If your business is done, be off with you.",
        "I would like a little rest.", "...", "- nodding off -")
    elseif step == 1 then
      say(D, "Oh...", "You want bandages, it seems.", "Know how they are made?",
        "In the Pyramids of Morroc there is a monster wound round and round in bandages,")
      next()
      say(D, "something like a mummy.", "Its rotten bandages are disinfected thoroughly.",
        "Well? Can you believe it?", "Heh... those bandages hold a poison,")
      next()
      say(D, "and somehow it works against poisons of its own kind.",
        "Poison drives out poison... you hear it every day.",
        "...Another day I would give you more of my time,")
      next()
      say(D, "but I am a little tired now...", "Here, take them.",
        "When you can treat wounds well, come and help our nursing team.")
      next()
      say(D, "And do not go dying at some mummy's hands because you want bandages.")
      next()
      mes "^3355FF- you received Sterilized Bandages -^000000"
      v.skill_nov = 2
    else
      say(D, "This is the nursing section of the Prontera Guard!",
        "We give first aid to patients between life and death,",
        "so every member is on edge at all times.")
      next()
      say(D, "I, the Nursing Instructor, am no different: my nerves are always strung tight.",
        "When your business is done, leave promptly.")
    end
    close()
  end,
}

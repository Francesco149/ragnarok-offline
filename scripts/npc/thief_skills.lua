-- The Thief quest skills: Sand Attack, Back Slide, Find Stone, Stone Fling.
--
-- Source: mod0631 conf/npc_event_skillget.txt:2004 ("アルコスク", in the Thief
-- guild under the Pyramid) and :2443 (the pouch maker in Payon), archived
-- 2003-12-18, the day this client was built. Tier athena2003, jRO lineage.
-- Places, sprites, requirements, items and the order of the talk are
-- mod0631's. The English is ours, translated from its Japanese; skill and
-- item names are the client's own (skillnametable, idnum2itemdisplaynametable).
--
-- A second, independent witness for the international servers: RODataZone's
-- Thief page as captured on 2003-12-04 (tier period;
-- web.archive.org/web/20031204065516/http://www.rodatazone.com/jobs/1/thief.php):
-- "Talk to Alcouskou in the right-hand room of the Thief Guild (moc_prydb1,
-- 154/128)", with the same items for all four skills and "Bag Seller in
-- Payon Town (payon, 28/121)" for the bag. The NPC names are from there.
-- It differs from mod0631 in two places, where we keep mod0631: the Bag
-- Seller's cell (30,120 here) and Stone Fling's job level ("Thief Job 20+"
-- there; 15 in mod0631 and in 817).
--
-- 817 had these as four separate NPCs in Morroc and only one of them ever
-- loaded: "Sono-san" (Stone Fling, morocc 165,124), replaced here. Its Back
-- Slide and Find Stone files have spaces where the header needs a tab, and it
-- has no Sand Attack at all. The client ships the two quest items of Sand
-- Attack (7041 Fine Grit, 7042 Leather Bag of Infinity), so that quest
-- belongs to this version. docs/DEVIATIONS.md, research/THREADS.md #7.

local WHO = "[Alcouskou]"
local function say(who, ...)
  mes(who)
  for _, line in ipairs { ... } do mes(line) end
end

local function already()
  say(WHO, "You have mastered that one already.", "Try your hand at something else.")
  close()
end

-- A Thief needs the job level; an Assassin or a Rogue does not.
local function too_green(level)
  return player.class == Job_Thief and player.job_level < level
end

local function sand_attack()
  if skill_level(149) > 0 then return already() end
  if v.skill_thief_1 == 2 then
    say(WHO, "Oh!", "Yes, this is it: the ^3355FFLeather Bag of Infinity^000000, which holds any",
      "number of small things!", "Now you can use the skill I am about to teach you.")
    next()
    say(WHO, "Right, do as I do.", "Take some sand -", "swing your arm all the way back -")
    next()
    say(WHO, "Hah!", "^5533FF- zzsh -^000000")
    next()
    mes "^5533FF- fwoosh -^000000"
    next()
    say(WHO, "Heh... that looks about right.", "Good, you have got it.",
      "There is nothing more I can teach you about Sand Attack.",
      "Put it to good use in a real fight. See you.")
    take_item(7042, 1)
    give_skill(149, 1, 0)
    v.skill_thief_1 = 0
    return close()
  end
  if v.skill_thief_1 == 1 then
    say(WHO, "See Rurumuni, in the west of Payon village, and have a",
      "^3355FFLeather Bag of Infinity^000000 made.")
    next()
    say(WHO, "Go while I work on these stones.")
    return close()
  end
  if count_item(7041) < 5 or too_green(25) then
    say(WHO, "A Thief, I think, may as well have one way of protecting himself,",
      "whatever the means.", "People call it cowardly, or unfair.",
      "I don't see it that way.")
    next()
    say(WHO, "In this world the one who wins is the one who lives.",
      "Since we are in the same trade, I will teach you a trick.",
      "It is nothing more than throwing sand at someone and taking",
      "their sight for a moment.")
    next()
    say(WHO, "But hit them as hard as you can in that moment and you will",
      "do more damage than usual.", "And if it lands well, you may even stun them... heh.")
    next()
    say(WHO, "We do what it takes to stay alive. For that you need sand,",
      "and sand should be ready beforehand.",
      "Go and bring me ^3355FF5 Fine Grit^000000.")
    next()
    say(WHO, "I know you want to learn it right away, but I can't teach",
      "you before you have brought them.", "Off you go.")
    next()
    say(WHO, "Oh, one more thing.", "An Assassin is no problem, but a Thief has to be",
      "^3355FFjob level 25 or higher^000000 before I can teach this.", "Remember that.")
    return close()
  end
  say(WHO, "You gathered them well, the Fine Grit.",
    "Why did I have you bring it? Because you can never tell where",
    "a fight will find you: in a dungeon of nothing but hard rock,",
    "say, or in a forest thick with grass.")
  next()
  say(WHO, "Where there is no sand to be had, these stones give you",
    "sand you can throw at any time.", "Then you need something to keep it in.")
  next()
  say(WHO, "That much I can't do for you.",
    "Go to Rurumuni in Payon and have a",
    "^3355FFLeather Bag of Infinity^000000 made.")
  next()
  say(WHO, "Rurumuni should be in the west of Payon village.",
    "Go while I work on these stones.")
  take_item(7041, 5)
  v.skill_thief_1 = 1
  close()
end

local function back_slide()
  if skill_level(150) > 0 then return already() end
  if count_item(940) < 20 or too_green(35) then
    say(WHO, "In a fight you attack, of course. But bringing the other down",
      "while taking little harm matters too.",
      "Outwitting them on a sound judgement is as important as the attack.")
    next()
    say(WHO, "We can move faster than other people, so we should at least",
      "be able to run rings round an enemy and strike where it counts.",
      "Our reflexes let us dodge most attacks...")
    next()
    say(WHO, "...but there are dangerous moments all the same, aren't there?",
      "And when enemies swarm you all at once, it gets rough.",
      "Then the wise thing is to fall back with good grace.")
    next()
    say(WHO, "An ordinary person can only turn their back and run.",
      "We don't. We can retreat quickly, backwards,",
      "still facing the enemy.")
    next()
    say(WHO, "It is a fine technique: you gather your strength in your legs",
      "for an instant and pull back in one go.",
      "It opens a distance between you and the enemy in no time.")
    next()
    say(WHO, "It only comes with practice, over and over, so bring plenty",
      "of what the practice needs.",
      "Collect ^3355FF20 Grasshopper's Legs^000000 and I will teach you.")
    next()
    say(WHO, "Oh, one more thing: you need some grounding to make use of it.",
      "A Thief has to reach ^3355FFjob level 35^000000 before I can teach this.")
    return close()
  end
  say(WHO, "Let's practise hard!")
  next()
  say(WHO, "Ssss -")
  next()
  say(WHO, "Zzzzsh - -")
  next()
  say(WHO, "Shoom! - - -")
  next()
  say(WHO, "Good, good~", "You can do this on your own now.", "Keep practising.")
  take_item(940, 20)
  give_skill(150, 1, 0)
  close()
end

-- The first four pages are the same for Find Stone and Stone Fling.
local function stones_intro()
  say(WHO, "The best members of our guild are mostly good with their hands.",
    "They can pick out a small stone lying by the road at a glance,",
    "and choose one that is fit for throwing.")
  next()
  say(WHO, "We wondered how to put that talent to use in a real fight,",
    "and managed to turn \"throwing stones\" into a proper skill.",
    "^3355FFFind Stone^000000, as we call it, is there for the sake of ^3355FFStone Fling^000000:")
  next()
  say(WHO, "a skill for quickly picking a hard stone, good for throwing, anywhere.",
    "And Stone Fling hits a distant enemy squarely with it,",
    "which gets you some effect without spending a single coin.")
  next()
  say(WHO, "Well, you may think it is not much of a skill. What do you say?", "...")
  next()
end

local function find_stone()
  if skill_level(151) > 0 then return already() end
  if count_item(912) < 1 or count_item(948) < 1 or count_item(908) < 5 or too_green(20) then
    stones_intro()
    say(WHO, "^3355FFFind Stone^000000 is the knack of picking up a hard stone, good for",
      "throwing, wherever you are...", "There is more to it than you would think.")
    next()
    say(WHO, "It takes a fair amount of training, and it only comes with",
      "real effort and preparation.", "...Ahem...",
      "For practising, ^3355FF1 Bear's Footskin^000000 to wear on your hand.")
    next()
    say(WHO, "What you pick up with it, in place of a stone, is a ^3355FFZargon^000000!",
      "If you can pick up a Zargon, you are perfect.",
      "And ^3355FF5 Spawns^000000!!", "And your passion!", "If that's clear, off you go~")
    return close()
  end
  say(WHO, "Well, well, you really did gather them!",
    "Your keenness shows.", "Good, let's start the training, shall we.")
  next()
  say(WHO, "The point is to pick up and hold firmly a smooth stone of a good hardness.",
    "Your body learns it faster than any number of explanations.",
    "Now, give it a try.")
  next()
  mes "^3355FF- swish -^000000"
  next()
  mes "^3355FF- click! -^000000"
  next()
  mes "^3355FF- snap! -^000000"
  next()
  say(WHO, "That's good, keep it up!", "You will pick them up properly in a real fight too.",
    "From now on, judge them well with your own eyes and your own hands.")
  take_item(912, 1)
  take_item(948, 1)
  take_item(908, 5)
  give_skill(151, 1, 0)
  close()
end

local function stone_fling()
  if skill_level(152) > 0 then return already() end
  if count_item(910) < 2 or count_item(911) < 2 or too_green(15) then
    stones_intro()
    say(WHO, "^3355FFStone Fling^000000 hits the target you aim at, squarely,",
      "with a stone you can pick up anywhere.",
      "You pick the stone up from around you, but...")
    next()
    say(WHO, "...while you can't yet tell which stones are good for throwing,",
      "you can have another of the Thief classes, someone in the trade,",
      "share the stones they found and throw those.",
      "Not much fun, though...")
    next()
    say(WHO, "Learning this skill takes some practice, and the practice",
      "takes a few tools.", "You should be able to get them easily by now.")
    next()
    say(WHO, "What you need is ^3355FF2 Garlets^000000 and ^3355FF2 Scells^000000.",
      "Minerals sturdy enough to throw in place of stones, don't you think?",
      "Bring them and I will teach you the skill right away.")
    return close()
  end
  say(WHO, "You have them already?", "Then let's practise throwing!",
    "Take the Scells and the Garlets -")
  next()
  mes "^3355FF- whizz ...clack! -^000000"
  next()
  mes "^3355FF- whoosh! ...bang! -^000000"
  next()
  mes "^3355FF- swoosh!! ...crash!! -^000000"
  next()
  say(WHO, "Striiiike!", "All right!",
    "You don't need to throw Scells and Garlets any more:",
    "your aim and your power are good enough!")
  next()
  say(WHO, "You should be able to use this any time now.", "Well done!", "Take care, then~")
  take_item(910, 2)
  take_item(911, 2)
  give_skill(152, 1, 0)
  close()
end

npc {
  map = "moc_prydb1", x = 154, y = 128, dir = 4, sprite = 118, name = "Alcouskou",
  src = "m631:conf/npc_event_skillget.txt:2004",
  replaces = "817:skillquests/throwstone.txt:2",
  on_click = function()
    local c = player.class
    if c ~= Job_Thief and c ~= Job_Assassin and c ~= Job_Rogue then
      say(WHO, "We of the Thief classes have a few skills we need.",
        "They are not ones you can learn the usual way: you get them,",
        "quietly, from a fellow Thief or an Assassin.")
      next()
      say(WHO, "If you have friends of that kind, be sure to tell them about me.",
        "They can learn something new and be of help to their companions.")
      return close()
    end
    say(WHO, "There are a few skills you don't strictly need to get through",
      "life, but which turn out surprisingly useful in a pinch.",
      "Now, which one interests you?")
    next()
    local pick = menu("Sand Attack", "Back Slide", "Find Stone", "Stone Fling", "Let me think about it")
    if pick == 1 then return sand_attack() end
    if pick == 2 then return back_slide() end
    if pick == 3 then return find_stone() end
    if pick == 4 then return stone_fling() end
    if pick == 5 then
      say(WHO, "The careful type, are you.", "Come back when you have decided.")
      close()
    end
  end,
}

-- mod0631 names the NPC "ポケット販売商人" and has it speak as [ルルムニ];
-- "Bag Seller" is RODataZone's name for it (2003-12).
local R = "[Rurumuni]"
npc {
  map = "payon", x = 30, y = 120, dir = 4, sprite = 99, name = "Bag Seller",
  src = "m631:conf/npc_event_skillget.txt:2443",
  on_click = function()
    say(R, "I was born into a family of leather-bag makers here in the Payon",
      "region. We trade in hides with the hunters who hunt with bow and",
      "arrow, make bags of them, and live by that.")
    next()
    if v.skill_thief_1 >= 2 then
      say(R, "Yes, hand the bag I gave you to Alcouskou of the Thief guild",
        "and he will teach you the skill.")
      return close()
    end
    if v.skill_thief_1 ~= 1 then
      say(R, "My family and the Thief guild of Morroc have been on very",
        "good terms for a long time.",
        "Now and then we make the ^3355FFLeather Bag of Infinity^000000 they order.")
      return close()
    end
    if count_item(952) < 1 or count_item(1055) < 1 or count_item(1025) < 1 then
      say(R, "I have had the order from Alcouskou of the Thief guild.",
        "But a Leather Bag of Infinity takes quite some material.",
        "I am sorry for the trouble, but please bring me what the bag needs.")
      next()
      say(R, "Three materials:", "^3355FFEarthworm Peeling^000000", "^3355FFCobweb^000000",
        "^3355FFCactus Needle^000000", "One of each.")
      return close()
    end
    say(R, "Oh~ you have brought them!", "Very good, very good.",
      "I will make the Leather Bag of Infinity right now.",
      "Heave-ho, snip snip snip, stitch stitch stitch...", "-")
    next()
    say(R, "There, it is done~",
      "Hand this bag to Alcouskou of the Thief guild and he will",
      "teach you the skill.", "Good luck to you.")
    take_item(952, 1)
    take_item(1055, 1)
    take_item(1025, 1)
    give_item(7042, 1)
    v.skill_thief_1 = 2
    close()
  end,
}

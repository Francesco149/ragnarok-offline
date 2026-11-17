-- The Alchemist test: the guild of Al De Baran takes a Merchant's fee and an
-- errand, Gregory's sums, Darwin's first potion, Van Helmont's questions on
-- Morgenshtein's solvents, Nicholas Flamel's magic square and an errand to
-- the brothers Bain and Bajin in Juno; then the guild master's welcome.
-- Darwin takes an Illusion Flower instead of everything after Gregory.
--
-- Behind `jobs2_2` (euRO opened the 2-2 jobs on 2004-06-09).
--
-- Sources:
--   * jAthena //1046 script/npc/job/npc_job_18alchemist.txt (2004-09-12,
--     data:refs/jathena commit 6332410; Japanese, by Kalen): the flow, the
--     NPCs, their cells and sprites, the quest variable and its steps, the
--     fee and the items, every question and answer, the pass marks, the
--     magic square's eight answers, the guild dealer's stock and prices.
--     Tier athena2003 (a period emulator, months after our date). Its lines
--     marked 適当/脳内 ("made up") are its author's guesses: the fee waived
--     for an Old Magic Book and a Hammer of Blacksmith, the Job Level 50
--     skip, and what Darwin says after taking the flower.
--     Dialogue here is our English of its Japanese, cut to the window.
--   * jAthena //1046 script/npc/town/npc_town_yuno.txt: Bain and Bajin,
--     their list, their 5 Iron Ore, `npc_yunoalc`.
--   * jAthena //1046 script/warp/npc_warp_job.txt:324-346 (from //0966,
--     2004-05-07): the guild hall's eighteen warps. eAthena 1.0
--     npc/warps/other/jobquests.txt has the same with two rooms' exits
--     swapped. No period source of ours has any way into alde_alche.
--   * eAthena 1.0 npc/jobs/2-2/alchemist.txt (2004-11-04, "based off of
--     official iRO quest"; data:refs/rathena 706e78509): the English names
--     (Parmy Gianino, Raspuchin Gregory, Darwin, Van Helmont, Nicholas
--     Flamel, Vincent Carsciallo); and quests_yuno.txt there: Bain's five
--     Portable Furnaces and 2000 Zeny taken (below).
--   * "Gever Al Sharp", the dealer: eAthena npc/merchants/alchemist.txt
--     (2006, tier later); jAthena calls him アルシャープ.
--   * Morgenshtein's lessons are scripts/npc/dyes.lua (mod0631): Van
--     Helmont passes only those who have heard him (MOLGENSTAIN 3), as in
--     jAthena.
--
-- Checked: Gregory's 30 sums against our items.toml (prices with Discount
-- 24% and 20%, weights, DEF): every answer agrees (test alchemist_test).
--
-- Where we part from jAthena (docs/DEVIATIONS.md):
--   * Bain and Bajin read out 5 Portable Furnaces and 2000 Zeny, then check
--     for 3 and take no Zeny; they now take what they ask for (eAthena 1.0);
--   * Van Helmont says Morgenshtein is in Geffen (jAthena names no town);
--   * Nicholas's "do not take too long" has no timer in jAthena: none here;
--   * the brothers replace 817's Bain and Bajin (another errand, a Pair of
--     Sandals for 200,000 Zeny) while `jobs2_2` is on.

if not content("jobs2_2") then return end

local SRC = "jathena-1046:script/npc/job/npc_job_18alchemist.txt:"
local MAP = "alde_alche"

-- the guild hall (jAthena npc_warp_job.txt:324-346)
local HALL = {
  { "aldebaran", 54, 66, MAP, 41, 174 }, { MAP, 41, 171, "aldebaran", 56, 68 },
  { MAP, 129, 104, MAP, 163, 163 }, { MAP, 159, 163, MAP, 126, 104 },
  { MAP, 131, 77, MAP, 162, 107 }, { MAP, 159, 107, MAP, 128, 77 },
  { MAP, 47, 103, MAP, 155, 18 }, { MAP, 159, 18, MAP, 50, 103 },
  { MAP, 47, 77, MAP, 88, 18 }, { MAP, 92, 18, MAP, 50, 77 },
  { MAP, 14, 184, MAP, 88, 113 }, { MAP, 88, 117, MAP, 14, 180 },
  { MAP, 19, 171, "aldebaran", 68, 56 }, { "aldebaran", 66, 54, MAP, 19, 175 },
  { MAP, 18, 28, MAP, 89, 66 }, { MAP, 89, 63, MAP, 18, 24 },
  { MAP, 41, 187, MAP, 114, 178 }, { MAP, 114, 182, MAP, 41, 183 },
}
for _, w in ipairs(HALL) do
  warp { map = w[1], x = w[2], y = w[3], span = { 1, 1 }, to = { w[4], w[5], w[6] } }
end

local FURNACE, BERSERK, HAMMER, OLD_BOOK = 612, 657, 1005, 1006
local BOWL, BOTTLE, RED_HERB, YELLOW_HERB, WHITE_HERB = 7134, 713, 507, 508, 509
local ALCOHOL, DETRIMINDEXTA, KARVODAILNIROL, MIXTURE = 970, 971, 972, 974
local ILLUSION_FLOWER, ALCOHOL_BOOK = 710, 7127
local LOG, SAND, ORIDECON_STONE, ELUNIUM_STONE, COAL, IRON_ORE = 7068, 7043, 756, 757, 1003, 1002

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function merchant() return player.class == Job_Merchant end
local function alchemist() return player.class == Job_Alchem end
local function q() return v.change_alchemist end

-- v.change_alchemist (jAthena's variable and steps):
--   1 fee paid, 100 Portable Furnaces asked; 2 the same, 7 Berserk Potions;
--   3 accepted, before Gregory; 4 failed Gregory once (80 then passes);
--   5 Gregory passed, before Darwin; 6 Darwin's list given; 7 Darwin passed;
--   8 Van Helmont met; 9 Van Helmont passed; 10 Nicholas met; 11 the square
--   solved; 12 sent to Juno; 13 the brothers helped: the guild master;
--   15 the Illusion Flower given to Darwin: the guild master.

-- ------------------------------------------------------------ Parmy Gianino

local P = "[Parmy Gianino]"

local function parmy_sent_to_gregory()
  v.change_alchemist = 3
  page(P, "Now let us learn the basics", "of alchemy. First, how to put", "medicines together.")
  next()
  page(P, "Oh, before that... Gregory", "seems to want you. Whatever", "for, I wonder... Would you go", "and see him?")
  close()
end

local function parmy_bring(state)
  if state == 1 then mes("^3355FF100 " .. item_name(FURNACE) .. "^000000")
  else mes("^3355FF7 " .. item_name(BERSERK) .. "^000000") end
end

npc {
  map = MAP, x = 27, y = 185, dir = 5, sprite = 744, name = "Parmy Gianino",
  replaces = "817:jobs/alchemist.txt:5", src = SRC .. "22",
  on_click = function()
    if alchemist() then
      page(P, "Welcome, " .. player.name .. ".", "The guild is very busy today,", "as ever.")
      next()
      page(P, "Here we study putting life into", "dolls, and the engineering of", "life itself.")
      next()
      page(P, "Everyone is waiting for the", "research to succeed. It is hard", "work, so everyone is busy.")
      return close()
    end
    if not merchant() then
      page(P, "Welcome. This is the Alchemist", "Guild. We are always looking", "for fresh, gifted people.")
      next()
      page(P, "If you know a fine Merchant,", "do tell them of our guild. We", "always want those with a gift", "for alchemy.")
      return close()
    end
    if q() == 1 or q() == 2 then
      if count_item(OLD_BOOK) >= 1 and count_item(HAMMER) >= 1 then
        page(P, "Welcome back. You have brought", "an Old Magic Book and a Hammer", "of Blacksmith, I see. Then you", "need bring nothing else.")
        next()
        take_item(OLD_BOOK, 1)
        take_item(HAMMER, 1)
        return parmy_sent_to_gregory()
      end
      if (q() == 1 and count_item(FURNACE) < 100) or (q() == 2 and count_item(BERSERK) < 7) then
        page(P, "Not ready yet? To join the guild", "you must bring")
        parmy_bring(q())
        next()
        page(P, "Come again when you are ready.")
        return close()
      end
      page(P, "Welcome back. You seem to be", "ready. The guild will put what", "you brought to good use.")
      next()
      if q() == 1 then take_item(FURNACE, 100) else take_item(BERSERK, 7) end
      return parmy_sent_to_gregory()
    end
    if q() == 3 then
      page(P, "Gregory is waiting for you.", "Go and hear what he has to say.")
      return close()
    end
    if q() >= 4 then
      page(P, "Oh, forgive me, we are rather", "busy just now.")
      next()
      page(P, "Please ask one of the others.")
      return close()
    end
    page(P, "Welcome. This is the Alchemist", "Guild. What can I do for you?")
    next()
    local c = menu("What is an Alchemist?", "I want to be an Alchemist.", "Nothing in particular.")
    if c == 1 then
      page(P, "Alchemists study how to make", "new substances out of all kinds", "of matter. We begin by seeking", "out the elements and changing", "their nature.")
      next()
      page(P, "Our final aim is said to be", "making gold, but that is not", "all: simple medicines, and other", "substances, are made too.")
      next()
      page(P, "Some even study the making of", "life, a god's work... but it is", "so hard that everyone is still", "at it.")
      next()
      page(P, "An Alchemist works at research", "more than at trade, so it is", "hard work.")
      next()
      page(P, "If alchemy interests you, gain", "experience as a Merchant: handle", "many goods and you will learn", "a great deal.")
      return close()
    elseif c ~= 2 then
      page(P, "Then do come again when you", "have business with us.")
      return close()
    end
    page(P, "Oh, how lovely. I am Parmy", "Gianino, of the Alchemist Guild.")
    next()
    page(P, "Join the guild and finish its", "training, and you are known as", "an Alchemist and may take part", "in its research.")
    next()
    page(P, "But we do not take just anyone.", "You must be hard-working,", "curious, and able to lose", "yourself in research.")
    next()
    page(P, "There are a few conditions for", "joining. I will explain them", "once you apply.")
    next()
    page(P, "Well then, will you apply?")
    next()
    if menu("I apply.", "I will apply later.") ~= 1 then
      page(P, "The guild always welcomes a", "gifted Merchant. Do come again.")
      return close()
    end
    if player.job_level < 40 then
      page(P, "One moment... your level is not", "high enough. To be an Alchemist", "you need Job Level 40 or more.")
      return close()
    end
    if player.job_level == 50 then
      page(P, "Thank you for applying. You have", "mastered your job, so you are", "spared the tests.")
      next()
      return parmy_sent_to_gregory()
    end
    page(P, "Thank you for applying. To join,", "you pay ^3355FF50,000 Zeny^000000, and you", "bring us a few things.")
    next()
    page(P, "But bring an ^3355FFOld Magic Book^000000 and", "a ^3355FFHammer of Blacksmith^000000, and", "you need bring nothing else.")
    next()
    page(P, "Now sign the application, please.")
    next()
    menu(player.name)
    page(P, "Now the fee, please. Mind that", "you bring exactly what is", "asked of you.")
    next()
    if player.zeny < 50000 then
      page(P, "You seem to be short of money.")
      return close()
    end
    player.zeny = player.zeny - 50000
    v.change_alchemist = 1 + math.random(0, 1)
    page(P, "Let me see... " .. player.name .. ",", "please bring us")
    parmy_bring(q())
    next()
    page(P, "Your application is in.", "Off you go, then.")
    close()
  end,
}

-- ------------------------------------------------------------ Vincent Carsciallo

local V = "[Vincent Carsciallo]"

npc {
  map = MAP, x = 101, y = 184, dir = 4, sprite = 122, name = "Vincent Carsciallo", src = SRC .. "249",
  on_click = function()
    if player.class == Job_Novice then
      page(V, "Off you go, child. Play in here", "and you will get hurt.")
      next()
      page(V, "There are many dangerous things", "here. Go on, play outside.")
      return close()
    end
    if alchemist() then
      page(V, "Is your research going well?", "Take good care when you handle", "your medicines.")
      next()
      page(V, "Sometimes an experiment gives", "what no one expected! You must", "do a great many of them.")
      next()
      page(V, "If you ever make new life, tell", "the guild at once. Everyone is", "hard at it. Do your best too!")
      return close()
    end
    if not merchant() then
      page(V, "What business has an outsider", "in a place like this?")
      next()
      page(V, "This is no sideshow. Stop", "wandering about and go away.")
      return close()
    end
    if q() >= 13 then
      if player.skill_point > 0 then
        page(V, "You still have skill points", "unused. Use them all, then", "come back.")
        return close()
      end
      page(V, "So, you have joined and learnt", "the basics you need. Well done.")
      next()
      job_change(Job_Alchem)
      v.change_alchemist = 0
      page(V, "From now on you are an Alchemist,", "one of our guild. Work hard at", "your research and aim high.")
      next()
      give_item(ALCOHOL_BOOK, 1)
      page(V, "This is for you, specially.")
      next()
      page(V, "Until we meet again. As an", "Alchemist, never stop honing", "your craft!")
      return close()
    end
    if q() >= 1 then
      page(V, "You have applied, I see. Work", "hard at your studies, and", "always aim high.")
      next()
      page(V, "Do not forget to ask those who", "went before you. There is much to", "gain from others' knowledge.")
      return close()
    end
    page(V, "Hm, a Merchant. Interested in", "alchemy?")
    next()
    page(V, "We of the Alchemist Guild study", "all kinds of matter. Our aim is", "new results, by a way that is", "not magic.")
    next()
    page(V, "We study making new life, age and", "deathlessness, new substances out", "of common ones.")
    next()
    page(V, "Live long as a Merchant and you", "grow curious about all things. If", "you want to learn alchemy, try", "joining our guild.")
    close()
  end,
}

-- ------------------------------------------------------------ Raspuchin Gregory: the sums

local G = "[Raspuchin Gregory]"

-- { question lines, answer }: jAthena picks one of each set at random
local function priced(n, id) return n .. " " .. item_name(id) end
local SUMS = {
  { { { "12 + 23 + 34 + 45 =" }, 114 }, { { "13 + 25 + 37 + 48 =" }, 123 } },
  { { { "1000 - 36 - 227 - 348 =" }, 389 }, { { "1000 - 58 - 214 - 416 =" }, 312 } },
  { { { "14 x 17 x 3 =" }, 714 }, { { "12 x 24 x 3 =" }, 864 } },
  { { { "9765 / 3 / 5 / 7 =" }, 93 }, { { "10530 / 3 / 5 / 2 =" }, 351 } },
  { { { "(47 x 28) - (1376 / 4) =" }, 972 }, { { "(35 x 19) - (1792 / 7) =" }, 409 } },
  { { { "(2646 / 7) + (13 x 28) =" }, 742 }, { { "(2368 / 8) + (24 x 17) =" }, 704 } },
}
-- the rest name items, so they are built when asked (item_name needs the db)
local function shop_sums()
  local buy = "With Discount at 24%, what do"
  return {
    { { { buy, "you pay for " .. priced(12, 501) .. ",", priced(5, 601) .. " and", priced(1, 602) .. "?" }, 909 },
      { { buy, "you pay for " .. priced(6, 501) .. ",", priced(7, 506) .. " and", priced(8, 601) .. "?" }, 798 },
      { { buy, "you pay for " .. priced(15, 506) .. ",", priced(6, 611) .. " and", priced(4, 1065) .. "?" }, 934 },
      { { buy, "you pay for " .. priced(6, 501), "and " .. priced(7, 506) .. "?" }, 438 } },
    { { { "What do " .. priced(2, 1122) .. ",", priced(3, 2226) .. " and", priced(3, 2405) .. " weigh", "together?" }, 480 },
      { { "What do " .. priced(3, 1122) .. ",", priced(4, 2226) .. " and", priced(2, 2405) .. " weigh", "together?" }, 550 },
      { { "What do " .. priced(3, 1113) .. ",", priced(2, 2228) .. " and", priced(1, 2309) .. " weigh", "together?" }, 450 } },
    { { { "Add up the DEF of a", item_name(2107) .. ", " .. item_name(2303) .. ",", item_name(2278) .. ", " .. item_name(2321) .. ",", item_name(2206) .. ", " .. item_name(2503), "and " .. item_name(2212) .. "." }, 12 },
      { { "Add up the DEF of a", item_name(2216) .. ", " .. item_name(2307) .. ",", item_name(2281) .. ", " .. item_name(2208) .. ",", item_name(2503) .. ", " .. item_name(2405), "and " .. item_name(2283) .. "." }, 20 },
      { { "Add up the DEF of a", item_name(2103) .. ", " .. item_name(2309) .. ",", item_name(2218) .. ", " .. item_name(2244) .. ",", item_name(2250) .. ", " .. item_name(2280), "and " .. item_name(2203) .. "." }, 15 } },
    { { { "Buying " .. priced(5, 2228) .. ", how", "much more do you pay with", "Discount at 20% than at 24%?" }, 8800 },
      { { "Buying " .. priced(4, 2312) .. ", how", "much more do you pay with", "Discount at 20% than at 24%?" }, 7680 },
      { { "Buying " .. priced(3, 2330) .. ", how", "much more do you pay with", "Discount at 20% than at 24%?" }, 8520 } },
  }
end

local function gregory_quiz()
  page(G, "Now answer my questions! Let us", "see what you are made of!")
  next()
  local points = 0
  local sets = {}
  for _, s in ipairs(SUMS) do sets[#sets + 1] = s end
  for _, s in ipairs(shop_sums()) do sets[#sets + 1] = s end
  for _, set in ipairs(sets) do
    local pick = set[math.random(1, #set)]
    page(G, table.unpack(pick[1]))
    next()
    if input_number() == pick[2] then points = points + 10 end
  end
  if (q() == 3 and points >= 90) or (q() == 4 and points >= 80) then
    mes(G)
    if points == 90 then mes "Keke... one wrong on the way."
    elseif points == 80 then mes "Keke... two wrong on the way."
    else mes "Hmm, hmm..." end
    mes "Well, never mind. Gregory will"
    mes "grant you a pass, specially..."
    next()
    page(G, "Kikiki... then go. Work hard at", "your research, and make yourself", "useful to me. Kikiki...")
    next()
    page(G, "Now go! See Darwin. He will teach", "you to work in the laboratory.", "Tell him I sent you.")
    v.change_alchemist = 5
    return close()
  end
  page(G, "...Ridiculous! I was a fool to", "sit through it all! You fail!!!")
  next()
  page(G, "Someone who cannot do sums like", "these, an Alchemist? Don't make", "me laugh!")
  next()
  page(G, "You have come to the wrong door!", "Out! Out! Get out of here!!!!")
  v.change_alchemist = 4
  close()
end

npc {
  map = MAP, x = 175, y = 107, dir = 4, sprite = 749, name = "Raspuchin Gregory", src = SRC .. "331",
  on_click = function()
    if alchemist() then
      page(G, "Kikiki kekekeke... Hm, what do", "you want?!")
      next()
      page(G, "Come to steal my results, have", "you! Where did you hear of them?!")
      next()
      page(G, "Clever, are we... but you won't", "have your way. Kikiki kekekeke...")
      return close()
    end
    if not merchant() then
      page(G, "Kikiki, what do you want? Want to", "know what I do... I'll tell only", "you. Kekeke...")
      next()
      page(G, "Once this potion is finished...", "I could rule a whole country!")
      next()
      page(G, "Kikiki kekeke... Not a word to", "the people of Prontera...")
      return close()
    end
    if q() == 1 or q() == 2 then
      page(G, "Kikiki kekekeke. Hm, what are", "you?!")
      next()
      page(G, "Learn alchemy?! Do your dreaming", "in your sleep!!")
      next()
      page(G, "Kekeke!! Come back in a thousand", "years! Now get out!")
      return close()
    end
    if q() == 3 then
      if player.job_level == 50 then
        page(G, "Oh, you have mastered your job.", "Then I have nothing for you.")
        next()
        page(G, "Now go! See Darwin. He will teach", "you to work in the laboratory.", "Tell him I sent you.")
        v.change_alchemist = 5
        return close()
      end
      page(G, "Kikiki... another nobody has", "rolled into the guild!")
      next()
      page(G, "I don't like it... not one bit...", "Every one of them thinks money", "makes an Alchemist!!")
      next()
      page(G, "Kikiki... I shall interview you", "myself! No backbone, and out you", "go!!")
      next()
      page(G, "Stunned, are you... good, good.", "Thought you would sit there and be", "told things? Think again!", "Kikikiki...")
      next()
      return gregory_quiz()
    end
    if q() == 4 then
      page(G, "What, you again? I told you", "to get out!")
      next()
      page(G, "I don't like it... sneaking back", "in... what were you up to?!")
      next()
      page(G, "Hmph, very well. Gregory will", "test you once more...")
      next()
      return gregory_quiz()
    end
    if q() == 5 then
      page(G, "What are you doing? Go and study", "a little!")
      next()
    end
    if q() >= 5 then
      page(G, "Kikiki kekekeke. Don't think this", "is the end of it!")
      return close()
    end
    page(G, "Kikiki kekekeke. Hm, what are", "you?!")
    next()
    page(G, "Merchants keep stalls, don't", "they... what are you doing here?")
    next()
    page(G, "Kikikiki... get out. I have", "nothing for the likes of you...", "Kekekeke")
    close()
  end,
}

-- ------------------------------------------------------------ Darwin: the first potion

local D = "[Darwin]"

local function darwin_list()
  page(D, "^3355FF3 " .. item_name(BOWL) .. "^000000", "^3355FF3 " .. item_name(BOTTLE) .. "^000000",
    "^3355FF1 " .. item_name(RED_HERB) .. "^000000", "^3355FF1 " .. item_name(YELLOW_HERB) .. "^000000",
    "^3355FF1 " .. item_name(WHITE_HERB) .. "^000000")
  next()
  page(D, "When you are ready... come again.")
end

-- { choices, the right one, what he says to the others }
local STEPS = {
  { { "Set out the bowl.", "Put the bowl on my head.", "Kick the bowl." }, 1, { [2] = "...Hmm.", [3] = "...Hmm." } },
  { { "Put earth in the bowl.", "Put the herbs in the bowl.", "Put a harp in the bowl." }, 2, { [1] = "...Ah!", [3] = "...Ah!" } },
  { { "Crush the herbs.", "Crush the bowl.", "Crush Darwin's foot." }, 1, { [2] = "...What are you doing!", [3] = "...Ow! What is that for?!" } },
  { { "Sprinkle the water about.", "Drink the water.", "Pour in the water." }, 3, { [1] = "...What are you doing?!", [2] = "...Thirsty, are you?" } },
  { { "Crush the herbs.", "Eat the herbs.", "Sing and dance." }, 1, { [2] = "...Hungry, are you?", [3] = "...Have you gone mad?" } },
  { { "Stir-fry it with noodles.", "Pour it into the bottle.", "Drink from the bowl." }, 2, { [1] = "...Hungry, are you?", [3] = "...Feeling better?" } },
}

local function darwin_lesson()
  page(D, "...............", "...............", "...........ah...")
  next()
  page(D, "The one who wanted to learn", "alchemy... So, you are ready...")
  next()
  if count_item(BOWL) < 3 or count_item(BOTTLE) < 3 or count_item(RED_HERB) < 1
      or count_item(YELLOW_HERB) < 1 or count_item(WHITE_HERB) < 1 then
    page(D, "...Did you forget what to bring...", "I will say it once more, slowly.", "Do not forget it this time...")
    next()
    return darwin_list()
  end
  page(D, "You have everything... As I", "promised, I will show you how to", "make a medicine...")
  next()
  page(D, "First, the bowl... put the herbs", "in it... and slowly crush them...")
  next()
  page(D, "Add clean water a little at a", "time... stir gently, to a paste...", "add more herb, crush again...")
  next()
  page(D, "Like this... and when there is", "enough... pour it gently into", "a bottle...")
  next()
  take_item(BOWL, 3)
  take_item(BOTTLE, 3)
  take_item(RED_HERB, 1)
  take_item(YELLOW_HERB, 1)
  take_item(WHITE_HERB, 1)
  page(D, "There, done. Now you do just as", "I did. It is easy... you can.")
  next()
  local right = 0
  for _, step in ipairs(STEPS) do
    local c = menu(table.unpack(step[1]))
    if c == step[2] then
      right = right + 1
    else
      page(D, step[3][c] or "...")
      next()
    end
  end
  if right < #STEPS then
    page(D, "...............")
    next()
    page(D, "...The materials are all gone.", "...Get them ready again.")
    return close()
  end
  give_item(501, 1)
  give_item(503, 1)
  give_item(504, 1)
  v.change_alchemist = 7
  page(D, "...Well done. Good, for a first", "time... Take these to remember", "it by.")
  next()
  page(D, "Go and see Van Helmont in the", "room next door... He will teach", "you a little more. Ask him for", "more...")
  next()
  page(D, "Do not forget... what matters", "most to you... what you must", "keep safe...")
  close()
end

npc {
  map = MAP, x = 13, y = 15, dir = 7, sprite = 750, name = "Darwin", src = SRC .. "798",
  on_click = function()
    if alchemist() then
      page(D, "Ah... you. You came... yes.", "You came to be a fine", "Alchemist...")
      next()
      page(D, "But still no Illusion Flower...", "the lovely moonlit flower... ah...")
      next()
      page(D, "If only I could see its light...", "dear Hannah... Ahahahaha...")
      return close()
    end
    if merchant() and q() == 5 and count_item(ILLUSION_FLOWER) >= 1 then
      page(D, "That flower... will you give it", "to me? It is the one I studied...", "the moonlit one!!")
      next()
      if menu("I cannot give it to you.", "I brought it for you.") ~= 2 then
        page(D, "I see...")
        return close()
      end
      page(D, "Yes... then take my thanks... I", "will give you all I know of", "alchemy!!")
      next()
      take_item(ILLUSION_FLOWER, 1)
      mes "^3355ff- Darwin drills you in -"
      mes "- all he knows of alchemy -^000000"
      next()
      page(D, "Now... all that is left is to see", "the guild master...")
      v.change_alchemist = 15
      return close()
    end
    if merchant() and q() == 5 then
      page(D, "...............", "...............", "...........ah...")
      next()
      page(D, "A wolf? No, a person... You are", "looking for something too.")
      next()
      page(D, "If you have something dear, take", "care. To gain one thing, you may", "lose all you are...")
      next()
      page(D, "...............")
      next()
      page(D, "...And what brings you all the", "way here?")
      next()
      local c = menu("Teach me to work in the lab.", "Tell me about the flower.", "Nothing.")
      if c == 2 then
        page(D, "A flower... from my memory...", "fading... there is such a flower...")
        next()
        page(D, "For the one I loved, I tried to", "finish one piece of research", "alone...")
        next()
        page(D, "It has to do with a wolf and a", "flower, that is all I will say...", "I cannot tell you more...")
        next()
        page(D, "Yes... a single flower. One that", "gives you a glimpse of paradise...", "the ^3355FFIllusion Flower^000000...")
        next()
        page(D, "I made a homunculus. New life,", "born of a flower! No one believed", "my research.")
        next()
        page(D, "But nothing was left to me... my", "time stopped after that...", "nothing left... stopped...")
        next()
        page(D, "Ah... Hannah... you, in that", "lovely field of flowers without", "end... where have you gone...")
        return close()
      elseif c ~= 1 then
        page(D, "...............", "Do not forget what matters", "most...")
        return close()
      end
      page(D, "You came to learn alchemy... all", "I know, I worked out to reach", "my one hope...")
      next()
      page(D, "I can teach the basics... but what", "you want to do, in the end, you", "must decide yourself...")
      next()
      page(D, "I will teach you the basics... how", "to make a simple medicine... Get", "a few things ready...")
      next()
      v.change_alchemist = 6
      return darwin_list()
    end
    if merchant() and q() == 6 then return darwin_lesson() end
    if merchant() and q() == 7 then
      page(D, "Go and see... Van Helmont...")
      next()
      page(D, "Ah, Hannah, my love. Now I cannot", "even see that flower... my soul", "only waits, quietly...")
      return close()
    end
    page(D, "Nothing more to lose, or to", "gain... nothing, nothing at all...")
    next()
    page(D, "All my cursed eyes see is a", "dream I forgot long ago...", "Ahahahaha...")
    next()
    page(D, "There is no paradise... no...", "dear Hannah... ah...")
    close()
  end,
}

-- ------------------------------------------------------------ Van Helmont: the solvents

local VH = "[Van Helmont]"

local function van_helmont_go_away()
  page(VH, "Go on, off with you. I have", "nothing more to teach you.")
end

local function van_helmont_questions()
  page(VH, "Ah, the Merchant from the other", "day. Did you learn properly? Not", "off playing, I hope...")
  next()
  page(VH, "Then let me ask you a few things.", "Answer as you saw it done.")
  next()
  local right = 0
  if v.MOLGENSTAIN == 3 then
    page(VH, "Which of these is NOT needed", "to make a Counteragent?")
    next()
    if menu(item_name(KARVODAILNIROL), item_name(DETRIMINDEXTA), item_name(ALCOHOL)) == 1 then right = right + 1 end
    page(VH, "Which of these is NOT needed", "to make a Mixture?")
    next()
    if menu(item_name(KARVODAILNIROL), item_name(DETRIMINDEXTA), item_name(ALCOHOL)) == 2 then right = right + 1 end
  else
    -- jAthena: one who never heard Morgenshtein is asked the wrong way
    -- round, and no answer counts
    page(VH, "What do you need to make", "a Counteragent?")
    next()
    menu(item_name(949), item_name(938), item_name(702))
    page(VH, "What do you need to make", "a Mixture?")
    next()
    menu(item_name(528), item_name(1054), item_name(930))
  end
  if right < 2 then
    page(VH, "...Frankly, I am not pleased.", "I told you plainly to learn from", "Morgenshtein...")
    next()
    page(VH, "I have no words! Back you come,", "having learnt nothing, thought", "nothing! Out of my way!")
    return close()
  end
  page(VH, "Yes, you understand. Knowing your", "medicines matters. It is what", "keeps you safe, all through a", "life of research.")
  next()
  page(VH, "Go straight out and see Nicholas", "in the next room. He will teach", "you a good deal more.")
  next()
  van_helmont_go_away()
  v.change_alchemist = 9
  close()
end

npc {
  map = MAP, x = 79, y = 19, dir = 5, sprite = 748, name = "Van Helmont", src = SRC .. "1130",
  on_click = function()
    if alchemist() then
      page(VH, "What! I am busy now!! Out with", "your business.")
      next()
      page(VH, "Oh, I see: glad to be an", "Alchemist... but all play and", "you will never get anywhere.")
      next()
      page(VH, "So work hard, and study the", "engineering of life. And mind:", "no pride.")
      return close()
    end
    if not merchant() or q() < 7 or q() > 9 then
      page(VH, "So close... so close...", "ah, so close to finished!")
      next()
      page(VH, "Why, why did it fail!! I did", "nothing wrong! Why, why!!")
      next()
      page(VH, "Gather the materials all over", "again... but one day it will be", "done. Hehehe...")
      return close()
    end
    if q() == 9 then
      van_helmont_go_away()
      next()
      page(VH, "See Nicholas in the next room.", "I must get on with my research.")
      return close()
    end
    if q() == 7 then
      v.change_alchemist = 8
      page(VH, "Uuu... what is wrong... here it", "goes like this, there like that...", "In theory it is right... so why", "does it stop? What is wrong...")
      next()
      page(VH, "Yes, pour in the solution here and", "it should stop... it doesn't...", "Such a beginner's mistake... ah,", "let me try another way...")
      next()
      page(VH, "Yes, mend just this part. No need", "to start all over! All's well that", "ends... wait, wait, aah!!!!")
      next()
    else
      page(VH, "Yes, cut off the tentacle here,", "mix in a little sticky liquid,", "into the Jellopy solution in the", "test tube... eh? Where is my bowl...")
      next()
      page(VH, "Ah, I used the last one. Has", "Nicholas got one... a bother to go", "and borrow it...")
      next()
    end
    page(VH, "...............")
    next()
    page(VH, "...............and who are you?")
    next()
    if menu("I am applying to be an Alchemist.", "...............") == 1 then
      return van_helmont_questions()
    end
    page(VH, "Come to watch? Then keep still,", "over there.")
    next()
    page(VH, "I can't have you touching things", "for no reason. Just don't get in", "my way.")
    next()
    if menu("Please teach me something.", "...............") ~= 1 then
      page(VH, "...............")
      next()
      page(VH, "...............")
      next()
      page(VH, "...And who are you?")
      next()
      page(VH, "Whoever you are, don't hang about", "in here. Out with you! This is", "my room.")
      next()
      page(VH, "Go on, off with you! Shoo!!!")
      return close()
    end
    page(VH, "Did I not just say not to get in", "my way? Even listening takes up", "my time... mutter mutter.")
    next()
    page(VH, "Very well, an assignment. Go and", "study a little outside. Let me", "see... hmm...")
    next()
    page(VH, "Yes: see ^3355FFMorgenshtein^000000 in Geffen,", "and learn how a Counteragent and", "a Mixture are made.")
    next()
    page(VH, "No need to bring them. Just watch", "him handle them, closely.", "Understood?")
    next()
    page(VH, "Then off you go. Go on! Shoo!!!")
    close()
  end,
}

-- ------------------------------------------------------------ Nicholas Flamel: the magic square

local N = "[Nicholas Flamel]"

-- the eight 3x3 magic squares, read row by row
local SQUARES = { 618753294, 816357492, 276951438, 672159834, 492357816, 294753618, 834159672, 438951276 }

local function square_table()
  page("", "15 . 15 . 15 . 15", "15 . [] . [] . []", "15 . [] . [] . []", "15 . [] . [] . []")
end

local function nicholas_square()
  page(N, "An Alchemist needs many gifts:", "memory, focus, sums... and", "ideas.")
  next()
  page(N, "Now I shall see how much of an", "Alchemist is in you. First, look", "at this table.")
  next()
  square_table()
  next()
  page(N, "A row and a column of 15s, and", "three by three squares. Put the", "numbers 1 to 9 in the squares,", "one each, to finish the table.")
  next()
  page(N, "The rule: every row, every column", "and both diagonals must add up", "to 15. This is how you answer:")
  next()
  page(N, "1 . 2 . 3", "4 . 5 . 6", "7 . 8 . 9", "Filled in like that, you would", "type 123456789.")
  next()
  page(N, "Don't take too long, or I'll stop", "the test. Off you go: here is", "the table again.")
  next()
  square_table()
  next()
  page(N, "Your answer, then.")
  next()
  local n = input_number()
  if n <= 0 then
    page(N, "That is no answer.")
    return close()
  end
  for _, s in ipairs(SQUARES) do
    if n == s then
      page(N, "Correct. On to the next test,", "but it needs getting ready: speak", "to me again in a little while.")
      v.change_alchemist = 11
      return close()
    end
  end
  page(N, "Hmm... Wrong. Think it over", "slowly.")
  close()
end

local function nicholas_to_juno()
  page(N, "Find the brothers ^3355FFBain^000000 and", "^3355FFBajin^000000 in ^3355FFJuno^000000, the city of", "sages: they study alchemy with", "the Sages there. Go and help them", "with their research.")
end

npc {
  map = MAP, x = 145, y = 19, dir = 1, sprite = 57, name = "Nicholas Flamel", src = SRC .. "1346",
  on_click = function()
    if not merchant() or q() < 9 or q() > 13 then
      page(N, "This goes so, and that goes so...", "put that in here, and this...", "which means here is...", "...mutter mutter...")
      return close()
    end
    if q() == 9 then
      page(N, "So you are the Merchant who wants", "to be an Alchemist. But not just", "anyone can be one.")
      next()
      page(N, "No one may dabble in alchemy by", "halves. So the guild tests your", "brains.")
      next()
      page(N, "You will have seen by now: the", "formulas alchemy needs are no", "small matter.")
      next()
      page(N, "And we can't train Alchemists who", "only fail, over and over, and do", "the guild no good. Ha ha ha!")
      v.change_alchemist = 10
      next()
      return nicholas_square()
    end
    if q() == 10 then return nicholas_square() end
    if q() == 11 then
      page(N, "Next, there is a place you must", "go.")
      nicholas_to_juno()
      next()
      page(N, "Here, take these. They will help", "you make friends with them.")
      next()
      page(N, "1 " .. item_name(MIXTURE), "5 " .. item_name(LOG), "5 " .. item_name(SAND),
        "3 " .. item_name(ORIDECON_STONE), "3 " .. item_name(ELUNIUM_STONE))
      give_item(MIXTURE, 1)
      give_item(LOG, 5)
      give_item(SAND, 5)
      give_item(ORIDECON_STONE, 3)
      give_item(ELUNIUM_STONE, 3)
      v.change_alchemist = 12
      next()
      page(N, "I hope you reach Juno safely.", "Off you go.")
      return close()
    end
    if q() == 12 and v.npc_yunoalc ~= 2 then
      nicholas_to_juno()
      return close()
    end
    if q() == 12 then
      page(N, "A long journey; well done. Bain", "and Bajin wrote to say you were", "a great help.")
      next()
      page(N, "Your work succeeded, I hear. One", "who can help those two... yes,", "you are fit for it, no doubt.")
      next()
      v.change_alchemist = 13
    end
    page(N, "Go and see the guild master", "upstairs. Be glad! You are nearly", "there!")
    close()
  end,
}

-- ------------------------------------------------------------ Gever Al Sharp: the guild dealer

local AS = "[Gever Al Sharp]"
local BOOKS = { { 7144, 100000 }, { 7127, 100000 }, { 7128, 100000 }, { 7129, 100000 },
  { 7130, 100000 }, { 7131, 100000 }, { 7132, 100000 }, { 7133, 240000 } }

npc {
  map = MAP, x = 24, y = 188, dir = 2, sprite = 740, name = "Gever Al Sharp", src = SRC .. "1524",
  on_click = function()
    page(AS, "Welcome!")
    next()
    local c = menu("Buy a " .. item_name(BOWL) .. ".", "Buy a making book.", "Never mind.")
    if c == 1 then
      page(AS, "What would you like?")
      next()
      if menu(item_name(BOWL) .. " - 8z", "Stop trading.") ~= 1 then
        page(AS, "I look forward to your custom.")
        return close()
      end
      page(AS, "How many do you need? To stop,", "type 0.")
      while true do
        next()
        local n = input_number()
        if n <= 0 then
          page(AS, "Trade cancelled. Do come again", "when you need anything.")
          return close()
        end
        if n <= 100 then
          if player.zeny < n * 8 then
            page(AS, "You haven't enough money.")
            return close()
          end
          give_item(BOWL, n)
          player.zeny = player.zeny - n * 8
          break
        end
        page(AS, "100 or fewer, please.")
      end
    elseif c == 2 then
      page(AS, "Which would you like? The", item_name(7133) .. " is", "240,000 Zeny, the others", "100,000 Zeny.")
      next()
      local names = {}
      for i, b in ipairs(BOOKS) do names[i] = item_name(b[1]) end
      names[#names + 1] = "Stop trading."
      local b = BOOKS[menu(table.unpack(names))]
      if not b then
        page(AS, "I look forward to your custom.")
        return close()
      end
      if player.zeny < b[2] then
        page(AS, "You haven't enough money.")
        return close()
      end
      player.zeny = player.zeny - b[2]
      give_item(b[1], 1)
    else
      page(AS, "I look forward to your custom.")
      return close()
    end
    page(AS, "Thank you for your custom.")
    close()
  end,
}

-- ------------------------------------------------------------ Juno: Bain and Bajin

local YSRC = "jathena-1046:script/npc/town/npc_town_yuno.txt:"
-- what they ask for: eAthena 1.0's reading of jAthena's two lists (5 Furnaces, 2000 Zeny)
local WANT = { { MIXTURE, 1 }, { FURNACE, 5 }, { LOG, 5 }, { COAL, 5 }, { SAND, 5 }, { ELUNIUM_STONE, 3 }, { ORIDECON_STONE, 3 } }
local FEE = 2000

local function brothers_list(who)
  page(who, "^FF0000" .. WANT[1][2] .. " " .. item_name(WANT[1][1]) .. "^000000",
    "^FF0000" .. WANT[2][2] .. " " .. item_name(WANT[2][1]) .. "^000000",
    "^FF0000" .. WANT[3][2] .. " " .. item_name(WANT[3][1]) .. "^000000",
    "^FF0000" .. WANT[4][2] .. " " .. item_name(WANT[4][1]) .. "^000000",
    "^FF0000" .. WANT[5][2] .. " " .. item_name(WANT[5][1]) .. "^000000")
  next()
  page(who, "^FF0000" .. WANT[6][2] .. " " .. item_name(WANT[6][1]) .. "^000000",
    "^FF0000" .. WANT[7][2] .. " " .. item_name(WANT[7][1]) .. "^000000")
  next()
  page(who, "And 2,000 Zeny to mix in with", "them: small change works well.")
end

local function brothers_mix(who)
  for _, it in ipairs(WANT) do
    if count_item(it[1]) < it[2] then return false end
  end
  if player.zeny < FEE then return false end
  for _, it in ipairs(WANT) do take_item(it[1], it[2]) end
  player.zeny = player.zeny - FEE
  page(who, "Oh, you have brought everything", "we need. Thank you so much! Right,", "let us start the synthesis. Wait", "a moment.")
  next()
  mes "- Whirrr, grind grind grind -"
  next()
  mes "- Bzzzz, gong gong -"
  next()
  mes "- Shake shake, pop! -"
  next()
  page(who, "Right, it worked! For some reason,", "it made 5 ^FF0000" .. item_name(IRON_ORE) .. "^000000.")
  next()
  page(who, "Well, now we know what it does: we", "don't need these. We want to keep", "going, so come again when you", "have the materials.")
  give_item(IRON_ORE, 5)
  v.npc_yunoalc = 2
  close()
  return true
end

local function brother(def)
  npc {
    map = "yuno_in01", x = def.x, y = def.y, dir = def.dir, sprite = def.sprite, name = def.name,
    replaces = content("juno") and def.replaces or nil, src = YSRC .. def.line,
    on_click = function()
      local who = "[" .. def.name .. "]"
      if v.npc_yunoalc >= 1 then
        if brothers_mix(who) then return end
        page(who, table.unpack(def.again))
        next()
        brothers_list(who)
        next()
        page(who, "Please, then.")
        return close()
      end
      for _, p in ipairs(def.intro) do
        page(who, table.unpack(p))
        next()
      end
      if menu("I'll gather them for you.", def.no) ~= 1 then
        page(who, table.unpack(def.bye))
        return close()
      end
      page(who, "Oh! Oh! What a kind person! You", "will bring light to our research!", "Then here is what we need.")
      next()
      brothers_list(who)
      next()
      page(who, "Please, then.")
      v.npc_yunoalc = 1
      close()
    end,
  }
end

brother {
  name = "Bain", x = 103, y = 157, dir = 3, sprite = 98, line = 452, replaces = "817:cities/yuno.txt:370",
  intro = {
    { "I am an Alchemist. My brother and", "I study the homunculus. Records", "say one was made long ago, from", "natural matter and metals..." },
    { "...one with a will of its own.", "But if such a thing existed, we", "would not be having such a hard", "time of it, I think..." },
    { "Our research has stopped for want", "of materials. The Alchemists", "around here don't even know what", "to gather," },
    { "and some of it only comes from", "monsters. We are at our wits'", "end..." },
  },
  no = "Good luck with it.",
  bye = { "I envy travellers like you, who", "live as they please..." },
  again = { "You might forget again, so I'll", "read the list once more. Take", "notes as you go." },
}

brother {
  name = "Bajin", x = 100, y = 153, dir = 7, sprite = 47, line = 571, replaces = "817:cities/yuno.txt:432",
  intro = {
    { "I am Bajin, an Alchemist. I study", "with my brother. Our dream is to", "make a homunculus, but I just love", "working with my brother." },
    { "But we are short of materials, and", "the research is falling behind...", "We work so hard, and have no", "luck..." },
  },
  no = "Cheer up.",
  bye = { "I'd like to go travelling with my", "brother now and then..." },
  again = { "Oh dear, did you forget? I'll read", "it to you again: remember it", "well." },
}

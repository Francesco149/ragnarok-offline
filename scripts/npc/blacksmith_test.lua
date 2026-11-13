-- The Blacksmith test: an application at the guild in Geffen, a quiz and an
-- errand for Geshupenschte in Alberta, a delivery to one of five customers
-- across the world, and a last quiz in Morroc.
--
-- Sources:
--   * eAthena 1.0 npc/jobs/2-1/blacksmith.txt (2004-11-04; header: "EREMES
--     THE CANIVALIZER(Aegis), Translated by: yoshiki. Converted by:
--     Komurka", names changed "to the iRO names" by kobra_k88; in
--     data:refs/rathena, commit 706e78509): the flow, the NPCs and their
--     cells, the quizzes and which answers count, the five orders, the
--     rewards. The only complete script on disk. Tier athena2003.
--   * jAthena //1046 script/npc/job/npc_job_10blacksmith.txt (2004-09-12,
--     data:refs/jathena commit 6332410; Japanese, "不完全"): the same NPCs,
--     the same two sets of ten questions for Geshupenschte and Mitmayer's
--     quiz, the Tsurugi order and the Arbalest order, the Hammer of
--     Blacksmiths. Its own guild member skips the test "for lack of data"
--     and its Mitmayer is "未調査" (not researched), so it confirms the shape
--     but cannot be played. Where it parts from eAthena, see below.
--   * Before both: mod0631 (2003-12-18) and every English emulator to 0.5.2
--     (2004-04) change a Merchant of job level 40 at once, and mod0631's
--     guild member says conditions will come with an update
--     (conf/npc_job_2nd.txt:253). No player account dates the test before
--     autumn 2004 (research queue T1). It is here because the other 2-1
--     tests are: the client's patch of January 2004 "added 2nd job quest"
--     (research/THREADS.md #8). An approximation to sharpen.
--
-- Where eAthena and jAthena part (research/THREADS.md #8):
--   * pass marks. eAthena: Geshupenschte wants 100 the first time and 90 on
--     a retry, Mitmayer 80; jAthena: 90, then 80, and Mitmayer 100. We take
--     eAthena's, the translation of the official script;
--   * jAthena's eighth question asks for a level 3 weapon's safe limit and
--     counts +6; eAthena's sets ask a level 3 (+5) and a level 4 (+4)
--     weapon, which is how refining works in this game: eAthena's;
--   * the Arbalest order: jAthena asks for an Eye Patch where eAthena asks
--     for a Trunk; eAthena's (its code takes a Trunk).
--
-- What is ours (docs/DEVIATIONS.md):
--   * Tilpitz stands at jAthena's payon 151,39: eAthena's 214,79 is the
--     rebuilt Payon of later clients, outside this client's map;
--   * Geshupenschte's sixth question offers jAthena's Ring Pommel Saber
--     (環頭太刀) where eAthena has a Buster;
--   * eAthena's slips mended: Geshupenschte looked for a Knife (1210)
--     instead of the Gladius he made; three customers ran on from one
--     answer into the next ("not what I ordered... thanks again");
--   * pages cut to the dialogue window, typos mended.

local SRC = "eathena-1.0:npc/jobs/2-1/blacksmith.txt:"
local HAMMER, RECEIPT, STEEL = 1005, 1073, 999

-- v.bsmith_q: 1 applied, 2 Geshupenschte's quiz passed, 3 the order made,
--   4 the receipt brought, 5 Mitmayer's quiz passed (eAthena's BSMITH_Q)
-- v.bsmith_q2: at 1: 1 after a failed quiz; at 2 and 3: the order (1-5);
--   at 4: 1 once sent to Morroc
-- v.bsmith_q3: 1 once the order is delivered
-- v.bsmith_jl: the job level on the form (40 or 50: how much Steel you get)

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function start_over()
  v.bsmith_q, v.bsmith_q2, v.bsmith_q3 = 0, 0, 0
end

local SWORD = { [1] = true, [7] = true, [14] = true }
local MAGIC = { [2] = true, [9] = true, [16] = true }
local BOW = { [3] = true, [11] = true, [19] = true, [20] = true }
local HOLY = { [4] = true, [8] = true, [15] = true }
local THIEF = { [6] = true, [12] = true, [17] = true }

-- ------------------------------------------------------------ Altiregen

local A = "[Blacksmith Altiregen]"

local function not_a_merchant()
  local c = player.class
  page(A, "Welcome! We are the masters of", "metal, the Blacksmiths! Forging", "a weapon is almost an art.")
  next()
  page(A, "And those weapons can be sold", "for quite a profit, heh.")
  next()
  mes(A)
  if c == Job_Blacksmith then
    mes "Hey! It's been a while. Running"
    mes "an errand for Christopher? All"
    mes "this office work... my hammer is"
    mes "getting rusty."
  elseif c == Job_Novice then
    mes "You look like you want to be a"
    mes "Blacksmith! Sorry: you have to"
    mes "be a Merchant first."
  elseif SWORD[c] then
    mes "Come to order a sword? Sorry, I"
    mes "have paperwork to do. I can't"
    mes "forge anything now."
  elseif BOW[c] then
    mes "Sorry, we're not hiring anyone"
    mes "who isn't a Merchant."
  elseif MAGIC[c] then
    mes "Interested in training? Our guild"
    mes "can't help magic users with that."
  elseif HOLY[c] then
    mes "Oh! A holy one! Do me a favour"
    mes "and bless the sword I'm making!"
  elseif THIEF[c] then
    mes "Sorry, this is more of an office."
    mes "Nothing worth stealing here. We"
    mes "CAN forge you a knife, though."
  else
    mes "Only Merchants can become"
    mes "Blacksmiths."
  end
  close()
end

local function apply()
  if player.job_level < 40 then
    page(A, "You need more training as a", "Merchant. We only take job level", "40 and up.")
    next()
    page(A, "Much as I'd like to take you,", "we have regulations. Sorry. Come", "back later.")
    return close()
  end
  if player.job_level >= 50 then
    v.bsmith_jl = 50
    page(A, "Wow, I'm impressed! You've done a", "lot of training. With hard work", "come great rewards, they say.")
    emotion(1)
    next()
  else
    v.bsmith_jl = 40
    page(A, "Hmm... Your job level is", "satisfactory.")
    next()
    page(A, "Good, but we don't take just", "anybody. We'll test your skill as", "a Merchant, and your resolve.")
    next()
    page(A, "(rustle, rustle)")
    next()
  end
  page(A, "Hmm, our guildsman", "^5533FFGeshupenschte in Alberta^000000", "says he's short of hands. Go and", "help him: that's the first test.")
  next()
  page(A, "Have a nice trip!")
  v.bsmith_q, v.bsmith_q2, v.bsmith_q3 = 1, 0, 0
  close()
end

local function change()
  if count_item(HAMMER) < 1 then
    page(A, "Forgetting something? Show me the", "^5533FFHammer of Blacksmiths^000000, to prove", "you passed all the tests.")
    next()
    page(A, "You DO have it... don't you???")
    next()
    if menu("It's in storage, just a moment...", "I sold it for potions... now what?") == 1 then
      page(A, "...grrrrr...")
      emotion(7)
      return close()
    end
    page(A, "...I'll tell you what...")
    next()
    page(A, "YOU GET TO START ALL OVER!!!")
    emotion(32)
    start_over()
    return close()
  end
  if player.skill_point > 0 then
    page(A, "Use up all your skill points", "first, then I can make you", "a Blacksmith.")
    return close()
  end
  page(A, "I can see the fire in your eyes!", "You've passed! I grant you the", "power to work metal!")
  next()
  take_item(HAMMER, 1)
  job_change(Job_Blacksmith)
  page(A, "Never forget: we're the artists", "who put beauty into metal!")
  next()
  page(A, "And here's a present.", "Congratulations!")
  give_item(STEEL, v.bsmith_jl == 50 and 10 or 5)
  start_over()
  v.bsmith_jl = 0
  emotion(21)
  close()
end

npc {
  map = "geffen_in", x = 110, y = 169, dir = 4, sprite = 731, name = "Guildsman",
  replaces = "817:jobs/blacksmith.txt:3", src = SRC .. "25",
  on_click = function()
    if player.class ~= Job_Merchant then return not_a_merchant() end
    local q = v.bsmith_q
    if q == 1 then
      page(A, "You haven't left yet? Go and see", "Geshupenschte in Alberta.", "See you then.")
      emotion(1)
      return close()
    end
    if q == 2 or q == 3 then
      page(A, "How's the work Geshupenschte", "gave you? He can be a pain", "sometimes.")
      return close()
    end
    if q == 4 then
      if v.bsmith_q2 == 1 then
        page(A, "Hmm? Weren't you going to", "Morroc? You have to pass the", "test there.")
        return close()
      end
      page(A, "Congratulations! You passed", "Geshupenschte's tests!")
      next()
      page(A, "Hm? Heh, there's one more, you", "know. The guildsman for the last", "test is in ^5533FFMorroc^000000. Go there now.")
      next()
      if menu("Cruel, but I'll go.", "...I don't wanna travel T_T") == 1 then
        v.bsmith_q2 = 1
        page(A, "I know it's far, but it's not up", "to me. Besides, you know the", "region well, right?")
        next()
        page(A, "The guild won't shame itself by", "hiring fools. Off to Morroc,", "and find her!")
        return close()
      end
      page(A, "What!? You're giving up!?")
      emotion(1)
      next()
      page(A, "HOW DARE YOU INSULT OUR GUILD!", "Don't you dare even DREAM of", "becoming a Blacksmith!")
      emotion(23)
      next()
      page(A, "If you can't bear a little walk,", "how will you bear the forge!?")
      emotion(6)
      return close()
    end
    if q == 5 then return change() end

    page(A, "Hello, fellow Merchant! Are you", "here to apply? Write your name", "and your job level on this paper.")
    next()
    local m = menu("Apply", "What's required?", "Not now...")
    if m == 1 then return apply() end
    if m == 2 then
      page(A, "What's required? First, job", "level 40 at least. Then pass the", "test. That's it.")
      next()
      page(A, "The test is a bit long but", "passable: deliveries, a quiz on", "the goods of each town, and one", "on what a Blacksmith knows.")
      return close()
    end
    page(A, "All right, see you later.")
    close()
  end,
}

-- ------------------------------------------------------------ Geshupenschte

local G = "[Geshupenschte]"

-- eAthena's two sets (jAthena has the same two): the question, the four
-- answers, and which of them score. The last question scores any answer.
local QUIZ = {
  {
    { { "1. Which town and local", "product don't match?" },
      { "Morroc - Thief Clothes", "Alberta - Two-Handed Axe", "Comodo - Berserk Potion", "Alberta - Sword Mace" }, { 4 } },
    { { "2. What does the Blacksmith", "skill Hammer Fall do?" }, { "Stun", "Blind", "Poison", "Sleep" }, { 1 } },
    { { "3. What is a Merchant", "not good at?" }, { "Opening shops", "Buying cheap", "Selling high", "Running fast" }, { 4 } },
    { { "4. Where do you buy", "Blue Gemstones?" }, { "Alberta", "Morroc", "Geffen", "Prontera" }, { 3 } },
    { { "5. Where is Geffen's tool shop,", "seen from the tower?" }, { "8 o'clock", "11 o'clock", "6 o'clock", "5 o'clock" }, { 1 } },
    { { "6. Which weapon can't", "a Merchant use?" }, { "Stiletto", "Ring Pommel Saber", "Chain", "Bible" }, { 4 } },
    { { "7. Which has the highest", "defence?" }, { "Pantie", "Mink Coat", "Wooden Mail", "Silk Robe" }, { 2 } },
    { { "8. How far can you safely", "refine a level 3 weapon?" }, { "+3", "+4", "+5", "+6" }, { 3 } },
    { { "9. What can you make", "with Trunks?" }, { "Sakkat", "Bandana", "Majestic Goat", "Antler" }, { 1 } },
    { { "10. What matters most", "to a Merchant!?" }, { "Trust", "Honesty", "Money", "Talk" }, { 1, 2, 3, 4 } },
  },
  {
    { { "1. Which town and local", "product don't match?" },
      { "Al De Baran - Yggdrasil Leaf", "Alberta - Hammer", "Comodo - Berserk Potion", "Al De Baran - Hammer" }, { 4 } },
    { { "2. How much does", "a Jellopy sell for?" }, { "1 zeny", "2 zeny", "3 zeny", "4 zeny" }, { 3 } },
    { { "3. What do you need", "to open a shop?" }, { "A cart", "Goods in hand", "A weapon on", "Armour on" }, { 1 } },
    { { "4. Where is the", "Merchant Guild?" }, { "Alberta", "Morroc", "Geffen", "Prontera" }, { 1 } },
    { { "5. Where is Morroc's weapon", "shop, seen from the centre?" }, { "7 o'clock", "11 o'clock", "6 o'clock", "5 o'clock" }, { 4 } },
    { { "6. What can't a", "Merchant equip?" }, { "Main Gauche", "Claymore", "Chain", "Two-Handed Axe" }, { 2 } },
    { { "7. Which has the highest", "defence?" }, { "Pantie", "Mink Coat", "Wooden Mail", "Silk Robe" }, { 2 } },
    { { "8. How far can you safely", "refine a level 4 weapon?" }, { "+3", "+4", "+5", "+6" }, { 2 } },
    { { "9. Which monster doesn't", "drop Iron Ore?" }, { "Chonchon", "Steel Chonchon", "Zerom", "Anolian" }, { 4 } },
    { { "10. What matters most", "to a Merchant!?" }, { "Honesty", "Helpfulness", "Money", "Posture" }, { 1, 2, 3, 4 } },
  },
}

local function ask(set, points)
  local score = 0
  for _, q in ipairs(set) do
    page(G, table.unpack(q[1]))
    next()
    local c = menu(table.unpack(q[2]))
    for _, good in ipairs(q[3]) do
      if c == good then score = score + points end
    end
  end
  return score
end

local function quiz()
  local retry = v.bsmith_q2 == 1
  local score = ask(QUIZ[rand(1, #QUIZ)], 10)
  page(G, retry and "Great!" or "Good!")
  next()
  page(G, "Let's see... your score is", "^5533FF" .. score .. "^000000.")
  if score < (retry and 90 or 100) then
    mes "..."
    next()
    if retry then
      page(G, "You missed again! If you keep", "missing, I can't give you the", "job. Go and study more.")
    else
      page(G, "Sorry, friend, I can't give you", "the job. You need a bit more", "knowledge. Go and study.")
    end
    v.bsmith_q2 = 1
    return close()
  end
  mes(retry and "You passed!" or "Perfect!")
  next()
  page(G, retry and "I'll let you do the job." or "Then you can surely do the errand!", "Are you prepared?")
  v.bsmith_q, v.bsmith_q2 = 2, 0
  close()
end

-- the five orders: what to bring, the store item he works on, its customer
local ORDERS = {
  { { { STEEL, 2, "Steel" }, { 930, 1, "Rotten Bandage" }, { 717, 2, "Blue Gemstone" } }, 1610, "Arc Wand",
    "Baisulitst", "Deliver this to Baisulitst in", "Geffen, and bring back the receipt.",
    { "WHAT!? Nothing looks different?", "Look where my hand is! See that", "groove!? The SUPER GESHUPENSCHTE", "ARC WAND MK II!!!" },
    { "Feel the difference! Almost no", "inertia! Now, off to Baisulitst!" },
    { "Baisulitst should be at", "11 o'clock in Geffen." }, 3, "Steel" },
  { { { 1001, 2, "Star Dust" }, { 932, 1, "Skel-Bone" }, { 912, 1, "Zargon" } }, 1219, "Gladius",
    "Wickebine", "Now take this to Wickebine in", "Morroc, and bring back the receipt.",
    { "WHAT!? IT DOESN'T LOOK ANY", "DIFFERENT!? Look at the blade!", "Those beautiful markings! ULTRA", "GLADIUS GESHUPENSCHTE MK II!!" },
    { "Isn't it pretty? ^_^ Well, off to", "Morroc, and bring the receipt." },
    { "Take it to Wickebine, near the", "Sword Mace dealer in Morroc." }, 3, "Star Dusts" },
  { { { 1003, 2, "Coal" }, { 935, 2, "Shell" }, { 990, 2, "Red Blood" } }, 1119, "Tsurugi",
    "Gromgast", "Give this to Gromgast in Izlude,", "and bring back the receipt.",
    { "WHAT!? NOTHING LOOKS DIFFERENT!?", "Look at the blade! The beautiful", "markings! ULTRA GESHUPENSCHTE", "TSURUGI MK II!!!" },
    { "Isn't it pretty? ^_^ Anyway:", "Gromgast in Izlude! Go!" },
    { "Gromgast hangs out at", "11 o'clock in Izlude." }, 3, "Coals" },
  { { { 1002, 8, "Iron Ore" }, { 1019, 1, "Trunk" }, { 717, 2, "Blue Gemstone" } }, 1713, "Arbalest",
    "Tilpitz", "Deliver this to Tilpitz in Payon,", "and bring back the receipt.",
    { "WHAT!? NOTHING LOOKS DIFFERENT!?", "LOOK AT THE GROOVES FOR THE", "ARROWS! GESHUPENSCHTE'S MASTER", "ARCHER ARBALEST MK II!!!" },
    { "Look how well it's built! Well:", "Tilpitz in Payon, and bring the", "receipt." },
    { "Tilpitz hangs out in the south", "of Payon." }, 30, "Iron Ores" },
  { { { 1002, 8, "Iron Ore" }, { 511, 20, "Green Herb" }, { 919, 2, "Animal Skin" } }, 1513, "Morning Star",
    "Bismarck", "Take this to Bismarck in Comodo,", "and bring me the receipt.",
    { "WHAT!? NOTHING LOOKS DIFFERENT!?", "Look at the tips of the spikes!", "Green Herb antidote, mixed in!", "ANTI-POISON MORNING STAR MK II!" },
    { "Poisoned? Stab yourself with it!", "The wound... may get bigger. But", "the poison will be gone! Now, to", "Bismarck in Comodo!" },
    { "Bismarck is usually at", "12 o'clock in Comodo." }, 40, "ores" },
}

local function say_order(o)
  mes(G)
  for _, it in ipairs(o[1]) do mes("^5533FF" .. it[2] .. " " .. it[3] .. "^000000") end
  mes("and ^5533FF1 " .. o[3] .. "^000000 from a store.")
end

local function give_order()
  page(G, "Let's see... orders...")
  next()
  page(G, "Ah, here it is! The order that's", "almost due. Hmm...")
  next()
  page(G, "I need you to get me...")
  next()
  v.bsmith_q2 = rand(1, #ORDERS)
  local o = ORDERS[v.bsmith_q2]
  say_order(o)
  next()
  page(G, "Hey, we're not making you bring", "these because we're cheap!", "It's a test!")
  next()
  page(G, "I'll say it again.")
  next()
  say_order(o)
  next()
  page(G, "Good luck!")
  close()
end

local function make_order()
  local o = ORDERS[v.bsmith_q2]
  page(G, "Hmm? Did you bring", "everything?")
  next()
  for _, it in ipairs(o[1]) do
    if count_item(it[1]) < it[2] then
      page(G, "You didn't bring everything!")
      next()
      say_order(o)
      return close()
    end
  end
  if count_item(o[2]) < 1 then
    page(G, "You didn't bring everything!")
    next()
    say_order(o)
    return close()
  end
  page(G, "Didn't I ask for " .. o[10] .. " " .. o[11] .. "?")
  next()
  page(G, "Oh, no, " .. o[1][1][2] .. " is right.", "Give me a second.")
  for _, it in ipairs(o[1]) do take_item(it[1], it[2]) end
  take_item(o[2], 1)
  v.bsmith_q = 3
  next()
  page(G, "Grunt...")
  next()
  mes "~clank~ ~clonk~ ~clank~"
  next()
  page(G, "Ergh...")
  next()
  mes "~bonk~ ~bang~ ~bonk~"
  next()
  page(G, "Whew. There we go!", o[5], o[6])
  give_item(o[2], 1)
  next()
  page(G, table.unpack(o[7]))
  next()
  page(G, table.unpack(o[8]))
  close()
end

local function check_delivery()
  local o = ORDERS[v.bsmith_q2]
  if v.bsmith_q3 == 1 then
    page(G, "Did you deliver it?", "Let's see the receipt, then!")
    next()
    if menu("Here you go.", "Wait a second.") == 2 then
      page(G, "Well, tell me when you find the", "receipt. I HOPE you got one.")
      return close()
    end
    if count_item(RECEIPT) < 1 then
      page(G, "...You don't know where you left", "the receipt...")
      next()
      page(G, "WHAT!? YOU LOST IT!?", "Did you SELL it or something!?")
      emotion(23)
      next()
      page(G, "The receipt is a Merchant's soul!", "A Blacksmith's lifeline! Guess", "what? You get to... START", "ALL OVER!!")
      emotion(32)
      start_over()
      return close()
    end
    take_item(RECEIPT, 1)
    page(G, "Great! Very good! You're an honest", "Merchant indeed. Go and see", "Altiregen back in Geffen!")
    next()
    page(G, "I know you'll pass the test!!!")
    v.bsmith_q, v.bsmith_q2, v.bsmith_q3 = 4, 0, 0
    return close()
  end
  page(G, "Go and deliver it! ...You didn't", "forget... right?")
  next()
  if count_item(o[2]) < 1 then
    page(G, "WHAT!!! YOU LOST WHAT I GAVE", "YOU TO DELIVER!!!")
    emotion(23)
    next()
    page(G, "...Well then... I guess you'll", "just have to...")
    next()
    page(G, "START ALL OVER!! Now get out", "of my sight!!")
    emotion(7)
    start_over()
    return close()
  end
  page(G, table.unpack(o[9]))
  close()
end

npc {
  map = "alberta_in", x = 174, y = 22, dir = 4, sprite = 63, name = "Geshupenschte",
  src = SRC .. "230",
  on_click = function()
    if player.class ~= Job_Merchant then
      page(G, "Hello! I'm Geshupenschte, the", "Blacksmith! Nice to meet you!")
      next()
      page(G, "Blacksmith is such a great job!!", "Don't you think? Hahahahaha!")
      emotion(18)
      return close()
    end
    local q = v.bsmith_q
    if q == 1 then
      if v.bsmith_q2 == 1 then
        page(G, "Ah, you've studied more, I see.", "Don't miss any this time!", "Here we go!")
        next()
        return quiz()
      end
      page(G, "You're the one the guild sent,", "right? Whew, people order so many", "things. We can't keep up with", "the deliveries.")
      next()
      page(G, "A kid a few days ago wasted SO", "much of our money, just because", "I sent him on a slightly tricky", "errand >_>")
      next()
      page(G, "So first, a few questions!")
      next()
      if menu("All right.", "Wait, let me get prepared.") ~= 1 then
        page(G, "OK, come back when you're ready.", "I'll be waiting here.")
        return close()
      end
      return quiz()
    end
    if q == 2 then
      if v.bsmith_q2 > 0 then return make_order() end
      return give_order()
    end
    if q == 3 then return check_delivery() end
    if q == 4 then
      page(G, "Thanks for working for me!", "Now go and see Altiregen!")
      return close()
    end
    if q == 5 then
      page(G, "There's nothing for you here", "now... Thinking of Morroc,", "or Geffen?")
      return close()
    end
    page(G, "Hoho!! A Merchant! Excellent!", "I could use some help!!")
    next()
    page(G, "...Of course, I wouldn't want to", "bother someone as busy as you.", "Go on your way if you must...")
    next()
    page(G, "~Dum dee dum~ I am the best...", "I am a Blacksmith... (singing)")
    next()
    page(G, "What? Why are you still here?")
    emotion(1)
    close()
  end,
}

-- ------------------------------------------------------------ the customers

-- one customer per order: the order number, the cell, the sprite, the
-- eAthena line, the waiting talk, the thanks, and what they say after
local function customer(order, map, x, y, sprite, line, def)
  local who = "[" .. ORDERS[order][4] .. "]"
  local item = ORDERS[order][2]
  npc {
    map = map, x = x, y = y, dir = 4, sprite = sprite, name = ORDERS[order][4], src = SRC .. line,
    on_click = function()
      if player.class ~= Job_Merchant or v.bsmith_q ~= 3 then
        for i, p in ipairs(def.waiting) do
          page(who, table.unpack(p))
          if i == 1 and def.waiting_emotion then emotion(def.waiting_emotion) end
          if i < #def.waiting then next() end
        end
        return close()
      end
      if v.bsmith_q2 ~= order then
        page(who, table.unpack(def.wrong))
        emotion(4)
        return close()
      end
      if v.bsmith_q3 == 1 then
        page(who, table.unpack(def.done))
        return close()
      end
      if count_item(item) < 1 then
        page(who, table.unpack(def.no_item))
        emotion(def.no_item_emotion or 20)
        return close()
      end
      take_item(item, 1)
      v.bsmith_q3 = 1
      for _, p in ipairs(def.thanks) do
        page(who, table.unpack(p))
        next()
      end
      mes(def.scribble)
      next()
      give_item(RECEIPT, 1)
      page(who, table.unpack(def.receipt))
      close()
    end,
  }
end

customer(1, "geffen", 46, 164, 69, "923", {
  waiting = {
    { "It's been a while since I went to", "Alberta. And I'm a little upset", "about the late deliveries..." },
    { "I ordered from the Geffen", "Blacksmith Guild, and the order", "went to a smith in Alberta..." },
    { "Hmm... when will my special order", "^5555FFArc Wand^000000 come...?" },
  },
  wrong = { "...Hm? What's this? This is not", "what I ordered..." },
  done = { "Thanks again for the delivery.", "I really appreciate it!" },
  no_item = { "...Hm? You're here to deliver", "something to me, and you don't", "have it??" },
  no_item_emotion = 32,
  thanks = {
    { "Wow! Is it finally here?!", "Thank you! I waited forever!" },
    { "I ordered from the Geffen guild,", "and they gave it to the smith in", "Alberta! Thanks for coming so far." },
    { "And hit Geshupenschte once for me!", "How can anything be so late >_>", "Here's the receipt!" },
  },
  scribble = "~scribble, scribble~ ~rip~",
  receipt = { "Here you go! Goodbye, and thank", "you for the delivery!" },
})

customer(2, "morocc", 27, 112, 725, "979", {
  waiting = {
    { "...", "...When will my order be here?" },
    { "The Geffen Blacksmith Guild and", "Geshupenschte are both", "CHRONICALLY LATE!" },
    { "This is very upsetting..." },
  },
  waiting_emotion = 20,
  wrong = { "I think you're supposed to deliver", "that somewhere else..." },
  done = { "Thank you." },
  no_item = { "Ugh! You came all this way to", "deliver my order and you lost it??" },
  no_item_emotion = 6,
  thanks = {
    { "Is it finally here!?", "Give it!!!" },
    { "Tell the guild, and your teacher", "Geshupenschte: you're LATE, and I", "was WAITING." },
    { "But it does look like fine work." },
  },
  scribble = "(Writes something down.)",
  receipt = { "Here's the receipt. Thanks for", "the delivery." },
})

customer(3, "izlude", 69, 181, 734, "1038", {
  waiting = {
    { "...", "...When will that sword arrive >_>" },
    { "I need that sword for my training!", "Nooo... I mustn't get rusty!" },
    { "Noooooo T_T..." },
  },
  wrong = { "Sorry, you've got the wrong", "person." },
  done = { "Thanks for the delivery!" },
  no_item = { "Hmm... I don't understand. Where", "is the sword I ordered???" },
  thanks = {
    { "Ahhh, it's finally here.", "Let me see it." },
    { "Nice... It's better than", "I expected." },
    { "This sword is exactly what I", "need!! I like it! Here you go." },
  },
  scribble = "~scribble~ ~rip~",
  receipt = { "Here's the receipt! Thank you." },
})

-- jAthena's cell: eAthena's payon 214,79 is not on this client's map
customer(4, "payon", 151, 39, 59, "1096", {
  waiting = {
    { "When's that bow coming...", "How long ago did I order it >_>" },
    { "Nobody can make it but 'him'...", "But WHEN WILL IT COME >_>" },
    { "T_T" },
  },
  wrong = { "A delivery... for someone else?", "Then why not take it to that", "person instead???" },
  done = { "The more I look at it, the more", "beautiful it gets!!!" },
  no_item = { "Oooh, the delivery is here? What?", "You don't have anything???", "Don't play bad jokes on people!" },
  no_item_emotion = 7,
  thanks = {
    { "...Ohhh! It's finally here!!!", "Let me see it!" },
    { "Wow... Not bad... not bad at all!!", "Look at this curve!! It's", "wonderful!! I love it." },
    { "Thank you!!!" },
  },
  scribble = "~scribble, scribble~ ~rip~",
  receipt = { "Here's the receipt! Thank you", "again!!!!!" },
})

customer(5, "comodo", 158, 342, 118, "1159", {
  waiting = {
    { "...Ugh...", "...When's that delivery coming..." },
    { "...the... p... poison... >_>", "Ack..." },
    { "...Looks like... I'm... done for." },
  },
  wrong = { "I NEED AN ANTIDOTE, DAMMIT!!", "Not this stuff... ~ack~" },
  done = { "Whew, thank you." },
  no_item = { "ARE YOU TOYING WITH ME!!?", "How... ~cough~... could...", "~ugh~... you... ~ack~..." },
  no_item_emotion = 23,
  thanks = {
    { "Ugh... finally...", "~pant, pant~ Give it to me~" },
    { "Ugh... The Green Herb is in it...", "right? I... can't move... help me", "up... ~grunt~" },
    { "- STAB! -" },
    { "AAAAAAAAAACCCCCCKKKKK!!!" },
    { "...Whew, that works pretty well." },
  },
  scribble = "(~cough~)",
  receipt = { "Here's the receipt... Thank you", "for the delivery! I guess I get", "to live a little longer..." },
})

-- ------------------------------------------------------------ Mitmayer

local M = "[Mitmayer]"

-- eAthena's three sets (jAthena has the second and third); 20 points each
local LAST = {
  {
    { { "1. Which skill do you need", "for Discount?" }, { "Pushcart Lv 3", "Item Appraisal", "Mammonite Lv 10", "Enlarge Weight Limit Lv 3" }, { 4 } },
    { { "2. What does Hammer Fall do?" }, { "Stun", "Blind", "Confusion", "Poison" }, { 1 } },
    { { "3. How much zeny does Mammonite", "Lv 10 cost?" }, { "900z", "1,000z", "2,000z", "1,000,000z" }, { 2 } },
    { { "4. How much do you save with", "Discount at its highest level?" }, { "21%", "22%", "23%", "24%" }, { 4 } },
    { { "5. How much more do you earn with", "Overcharge at its highest level?" }, { "21%", "22%", "23%", "24%" }, { 3 } },
  },
  {
    { { "1. Which monster drops Steel?" }, { "Zerom", "Steel Chonchon", "Skel Worker", "Requiem" }, { 3 } },
    { { "2. What can you make", "from Red Blood?" }, { "Flame Heart", "Rough Wind", "Great Nature", "Mystic Frozen" }, { 1 } },
    { { "3. Which ore do you have the", "most of in your storage?" }, { "Wind of Verdure", "Red Blood", "Green Live", "Crystal Blue" }, { 2, 3, 4 } },
    { { "4. Which monsters are weakest", "against Wind weapons?" }, { "Fire", "Water", "Earth", "Wind" }, { 2 } },
    { { "5. How many Iron does it take", "to make Steel?" }, { "5", "4", "3", "6" }, { 1 } },
  },
  {
    { { "1. What do you do when you find", "someone in distress?" }, { "Ask what they need", "Talk for a bit", "Ignore them", "Drop an item and leave" }, { 1, 2 } },
    { { "2. Where do you learn", "Change Cart?" }, { "Al De Baran", "Alberta", "Morroc", "Izlude" }, { 2 } },
    { { "3. With Geffen Tower at the", "centre, where is the", "Blacksmith Guild?" }, { "11 o'clock", "5 o'clock", "7 o'clock", "12 o'clock" }, { 2 } },
    { { "4. Which town has the", "most Blacksmiths?" }, { "Prontera", "Morroc", "Alberta", "Geffen" }, { 4 } },
    { { "5. Which stat affects forging?" }, { "STR", "DEX", "AGI", "VIT" }, { 2 } },
  },
}

npc {
  map = "morocc", x = 95, y = 133, dir = 4, sprite = 726, name = "BS Guildsman",
  src = SRC .. "1224",
  on_click = function()
    local q = player.class == Job_Merchant and v.bsmith_q or 0
    if q == 5 then
      page(M, "I told you, you've passed. Go", "back to the guild.")
      next()
      page(M, "And don't lose the Hammer of", "Blacksmiths!! Don't forget to", "use your skill points, too!")
      return close()
    end
    if q ~= 4 then
      page(M, "Whew... the sun in Morroc is too", "strong. Well, it'll toughen my", "skin...")
      emotion(19)
      next()
      page(M, "We Blacksmiths are used to heat,", "after all: we work with fire", "every day!")
      next()
      page(M, "And a tan isn't so bad. It gives", "you a healthy, athletic look.")
      return close()
    end
    page(M, "Welcome! You want to be a smith?", "All right, but know that not", "everybody passes.")
    next()
    page(M, "It's a quiz on how well you know", "metals and the Blacksmith's", "trade. Shall we start?")
    next()
    if menu("OK.", "Not now.") ~= 1 then
      page(M, "OK, see you then.")
      return close()
    end
    page(M, "Five questions. Get too many", "wrong and you fail. I won't tell", "you which are right.")
    next()
    local score = 0
    for _, qq in ipairs(LAST[rand(1, #LAST)]) do
      page(M, table.unpack(qq[1]))
      next()
      local c = menu(table.unpack(qq[2]))
      for _, good in ipairs(qq[3]) do
        if c == good then score = score + 20 end
      end
    end
    page(M, "Great work!")
    next()
    page(M, "Let's see... your score is", "^5533FF" .. score .. "^000000 points!")
    next()
    if score < 80 then
      page(M, "Not enough. Go and study more!!", "Come back when you have.")
      return close()
    end
    page(M, "Great! Congratulations!", "You pass!")
    emotion(21)
    next()
    page(M, "Go back to the guild. Here is", "the proof that you passed: the", "^5533FFHammer of Blacksmiths^000000.")
    give_item(HAMMER, 1)
    v.bsmith_q, v.bsmith_q2 = 5, 0
    next()
    page(M, "DON'T LOSE IT!!")
    emotion(0)
    next()
    page(M, "Well, good luck!!!!")
    close()
  end,
}
